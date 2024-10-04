//! Example of comparing new heads from multiple providers.

use alloy::{
    network::AnyNetwork,
    providers::{Provider, ProviderBuilder},
};
use chrono::Utc;
use clap::Parser;
use eyre::Result;
use futures_util::StreamExt;
use tokio::sync::mpsc;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short = 'r', help = "rpcs to connect to, usage: -r <name>:<url> -r <name>:<url> ...")]
    rpcs: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let rpcs: Vec<(&str, &str)> = cli.rpcs.iter().filter_map(|s| s.split_once(':')).collect();

    let (sx, mut rx) = mpsc::unbounded_channel();
    for (name, url) in rpcs {
        let sx = sx.clone();
        let name = name.to_string();
        let url = url.to_string();
        let provider = match ProviderBuilder::new().network::<AnyNetwork>().on_builtin(&url).await {
            Ok(provider) => provider,
            Err(e) => {
                eprintln!("skipping {} at {} because of error: {}", name, url, e);
                continue;
            }
        };

        tokio::spawn(async move {
            let mut stream = match provider.subscribe_blocks().await {
                Ok(stream) => stream.into_stream(),
                Err(e) => {
                    eprintln!("skipping {} at {} because of error: {}", name, url, e);
                    return;
                }
            };
            while let Some(block) = stream.next().await {
                if let Err(e) =
                    sx.send((name.clone(), block.header.number, Utc::now().timestamp_millis()))
                {
                    eprintln!("sending to channel failed: {}", e);
                }
            }
        });
    }

    let mut last_block_number = 0;
    let mut last_timestamp = 0;
    while let Some((name, number, timestamp)) = rx.recv().await {
        if number > last_block_number {
            println!("{} block #{} at {}", name, number, timestamp);
            last_block_number = number;
            last_timestamp = timestamp;
            continue;
        }
        if number == last_block_number {
            println!(
                "{} block #{} at {} +{}ms",
                name,
                number,
                timestamp,
                timestamp - last_timestamp
            );
            last_timestamp = timestamp;
        }
    }

    Ok(())
}
