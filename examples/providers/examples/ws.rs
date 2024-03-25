//! Example of using the WS provider to subscribe to new blocks.

use alloy::network::Ethereum;
// Temp Fix
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_client::{RpcClient, WsConnect};
//
use eyre::Result;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "wss://eth-mainnet.g.alchemy.com/v2/your-api-key";

    let ws_transport = WsConnect::new(rpc_url);

    let rpc_client = RpcClient::connect_pubsub(ws_transport).await?;

    let provider = RootProvider::<Ethereum, _>::new(rpc_client);

    let sub = provider.subscribe_blocks().await?;

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
