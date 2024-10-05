//! Example of comparing new heads from multiple providers.

use alloy::{
    network::AnyNetwork,
    providers::{Provider, ProviderBuilder},
};
use chrono::Utc;
use clap::Parser;
use eyre::Result;
use futures_util::StreamExt;
use std::sync::atomic::{AtomicU32, Ordering};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Barrier};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short = 'r', help = "rpcs to connect to, usage: -r <name>:<url> -r <name>:<url> ...")]
    rpcs: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let rpcs: Vec<(&str, &str)> = cli.rpcs.iter().filter_map(|s| s.split_once(':')).collect();

    let barrier = Arc::new(Barrier::new(rpcs.len() + 1));
    let total_streams = Arc::new(AtomicU32::new(0));
    let (sx, mut rx) = mpsc::unbounded_channel();
    for (name, url) in rpcs {
        let sx = sx.clone();
        let name = Arc::new(name.to_string());
        let url = url.to_string();
        let total_streams = total_streams.clone();
        let barrier = barrier.clone();

        tokio::spawn(async move {
            let provider =
                match ProviderBuilder::new().network::<AnyNetwork>().on_builtin(&url).await {
                    Ok(provider) => provider,
                    Err(e) => {
                        eprintln!("skipping {} at {} because of error: {}", name, url, e);
                        barrier.wait().await;
                        return;
                    }
                };

            let mut stream = match provider.subscribe_blocks().await {
                Ok(stream) => stream.into_stream(),
                Err(e) => {
                    eprintln!("skipping {} at {} because of error: {}", name, url, e);
                    barrier.wait().await;
                    return;
                }
            };
            total_streams.fetch_add(1, Ordering::SeqCst);
            barrier.wait().await;

            while let Some(block) = stream.next().await {
                if let Err(e) = sx.send((name.clone(), block, Utc::now())) {
                    eprintln!("sending to channel failed: {}", e);
                }
            }
        });
    }

    barrier.wait().await;

    #[derive(Debug)]
    struct TxTrack {
        first_seen: chrono::DateTime<Utc>,
        seen_by: Vec<(Arc<String>, chrono::DateTime<Utc>)>,
    }

    let total_streams = total_streams.load(Ordering::SeqCst) as usize;
    let mut tracker = HashMap::new();
    while let Some((name, block, timestamp)) = rx.recv().await {
        let block_number = block.header.number;
        let track = tracker
            .entry(block_number)
            .and_modify(|t: &mut TxTrack| {
                t.seen_by.push((name.clone(), timestamp));
            })
            .or_insert(TxTrack { first_seen: timestamp, seen_by: vec![(name.clone(), timestamp)] });

        if track.seen_by.len() == total_streams {
            let mut msg = String::new();
            for (name, timestamp) in track.seen_by.iter() {
                msg.push_str(&format!(
                    "{} +{}ms ",
                    name,
                    (*timestamp - track.first_seen).num_milliseconds()
                ));
            }
            println!(
                "block #{} at {} - {}",
                block_number,
                track.first_seen.timestamp_millis(),
                msg
            );
            tracker.remove(&block_number);
        }
    }

    Ok(())
}
