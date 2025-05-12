//! Example of using the WS provider to subscribe to new blocks.

use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use eyre::Result;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    // Create the provider.
    let rpc_url = "wss://eth-mainnet.g.alchemy.com/v2/your-api-key";
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    // Subscribe to new blocks.
    let sub = provider.subscribe_blocks().await?;

    // Wait and take the next 4 blocks.
    let mut stream = sub.into_stream().take(4);

    println!("Awaiting block headers...");

    // Take the stream and print the block number upon receiving a new block.
    let handle = tokio::spawn(async move {
        while let Some(header) = stream.next().await {
            println!("Latest block number: {}", header.number);
        }
    });

    handle.await?;

    Ok(())
}
