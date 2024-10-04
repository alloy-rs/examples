//! Example of comparing pending transactions from multiple providers.

use clap::Parser;
use eyre::Result;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short = 'r', help = "rpcs to connect to, usage: -r <name>:<url> -r <name>:<url> ...")]
    rpcs: Vec<String>,

    #[clap(short = 'c', default_value_t = 50, help = "number of pending transactions to compare")]
    count: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let tmp: Vec<(&str, &str)> = cli.rpcs.iter().filter_map(|s| s.split_once(':')).collect();
    let mut rpcs = vec![];
    for (name, url) in tmp {
        if url.starts_with("http") {
            eprintln!("skipping {} at {} because it is not a websocket/ipc endpoint", name, url);
            continue;
        }
        rpcs.push((name, url));
    }

    Ok(())
}
