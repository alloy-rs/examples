//! Example of using the WS provider to subscribe to new blocks.

// Temp Fix
use alloy_network::Ethereum;
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_client::{RpcClient, WsConnect};
//
use eyre::Result;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up the WS transport which is consumed by the RPC client.
    let rpc_url = "wss://eth-mainnet.g.alchemy.com/v2/your-api-key";

    // Create the WS connection object.
    let ws_transport = WsConnect::new(rpc_url);

    // Connect to the WS client.
    let rpc_client = RpcClient::connect_pubsub(ws_transport).await?;

    // Create the provider.
    let provider = RootProvider::<Ethereum, _>::new(rpc_client);

    // Subscribe to new blocks.
    let sub = provider.subscribe_blocks().await?;

    // Wait and take the next 4 blocks.
    let mut stream = sub.into_stream().take(4);

    println!("Awaiting blocks...");

    while let Some(block) = stream.next().await {
        println!("{:?}", block.header.number);
    }

    let handle = tokio::spawn(async move {
        while let Some(block) = stream.next().await {
            println!("{:?}", block.header.number);
        }
    });

    handle.await?;

    Ok(())
}
