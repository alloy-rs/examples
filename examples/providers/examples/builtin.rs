//! Example of using the `on_builtin` method in the provider.

use alloy::{
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
};
use eyre::Result;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    // Instantiate a HTTP transport provider by passing the HTTP endpoint url
    let http_rpc_url = anvil.endpoint();
    let http_provider = ProviderBuilder::new().on_builtin(&http_rpc_url).await?;

    // Get latest block number
    let block_number = http_provider.get_block_number().await?;

    println!("Latest block number: {block_number:?}");

    // This requires the `pubsub` and `ws` features to be enabled on alloy-provider
    let ws_rpc_url = anvil.ws_endpoint();
    let ws_provider = ProviderBuilder::new().on_builtin(&ws_rpc_url).await?;

    let sub = ws_provider.subscribe_blocks().await?;

    let mut stream = sub.into_stream().take(2);

    println!("Awaiting blocks...");

    let handle = tokio::spawn(async move {
        while let Some(block) = stream.next().await {
            println!("{}", block.header.number.expect("Failed to get block number"));
        }
    });

    handle.await?;

    // This requires the `pubsub` and `ipc` features to be enabled on alloy-provider
    // This would throw a runtime error if the ipc does not exist
    let ipc_path = "/tmp/reth.ipc";
    let ipc_provider = ProviderBuilder::new().on_builtin(ipc_path).await?;

    let _block_number = ipc_provider.get_block_number().await?;

    Ok(())
}
