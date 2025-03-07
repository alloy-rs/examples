//! Example of using the `on_builtin` method in the provider.

use alloy::{
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
};
use eyre::Result;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    // Instantiate a HTTP transport provider by passing the HTTP endpoint url.
    let http_rpc_url = anvil.endpoint();
    let http_provider = ProviderBuilder::new().connect(&http_rpc_url).await?;

    // Get latest block number.
    let block_number = http_provider.get_block_number().await?;

    println!("Latest block number: {block_number:?}");

    // This requires the `pubsub` and `ws` features to be enabled.
    let ws_rpc_url = anvil.ws_endpoint();
    let ws_provider = ProviderBuilder::new().connect(&ws_rpc_url).await?;

    let sub = ws_provider.subscribe_blocks().await?;

    let mut stream = sub.into_stream().take(2);

    println!("Awaiting block headers...");

    let handle = tokio::spawn(async move {
        while let Some(header) = stream.next().await {
            println!("{}", header.number);
        }
    });

    handle.await?;

    // This requires the `pubsub` and `ipc` features to be enabled.
    // This would throw a runtime error if the ipc does not exist.
    let ipc_path = "/tmp/reth.ipc";
    let ipc_provider = ProviderBuilder::new().connect(ipc_path).await?;

    let _block_number = ipc_provider.get_block_number().await?;

    Ok(())
}
