//! Example of subscribing to blocks and watching block headers by polling.

use alloy::node_bindings::Anvil;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_client::RpcClient;
use eyre::Result;
use futures_util::{stream, StreamExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    // Create a provider.
    let ws = alloy_rpc_client::WsConnect::new(anvil.ws_endpoint());
    let provider = ProviderBuilder::new().on_client(RpcClient::connect_pubsub(ws).await?);

    // Subscribe to blocks.
    let subscription = provider.subscribe_blocks().await?;
    let mut stream = subscription.into_stream().take(2);

    while let Some(block) = stream.next().await {
        println!("Received block number: {:?}", block.header.number.unwrap().to_string());
    }

    // Poll for block headers.
    let poller = provider.watch_blocks().await?;
    let mut stream = poller.into_stream().flat_map(stream::iter).take(2);

    while let Some(block_hash) = stream.next().await {
        println!("Polled for block header: {block_hash:?}");
    }

    Ok(())
}
