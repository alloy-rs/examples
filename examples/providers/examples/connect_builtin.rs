//! Example of using the `RootProvider<N, T: BoxTransport>::connect_builtin` to create a provider
//! from a connection string. The connection string can be a HTTP, WS or IPC endpoint.

use alloy::node_bindings::Anvil;
use alloy_network::Ethereum;
use alloy_provider::{Provider, RootProvider};
use alloy_transport::BoxTransport;
use eyre::Result;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().block_time(1).try_spawn()?;
    let http = anvil.endpoint();
    let ws = anvil.ws_endpoint();

    // Instantiate a HTTP transport provider by passing the http endpoint url
    let http_provider =
        RootProvider::<Ethereum, BoxTransport>::connect_builtin(http.as_str()).await?;

    // Get latest block number
    let block_number = http_provider.get_block_number().await?;

    println!("Latest block number: {:?}", block_number);

    // This requires the `pubsub` and `ws` features to be enabled on alloy-provider
    let ws_provider = RootProvider::<Ethereum, BoxTransport>::connect_builtin(ws.as_str()).await?;

    let sub = ws_provider.subscribe_blocks().await?;

    let mut stream = sub.into_stream().take(2);

    println!("Awaiting blocks...");

    let handle = tokio::spawn(async move {
        while let Some(block) = stream.next().await {
            println!("{:?}", block.header.number);
        }
    });

    handle.await?;

    let ipc_path = "/tmp/reth.ipc";

    // This requires the `pubsub` and `ipc` features to be enabled on alloy-provider
    // This would throw a runtime error if the ipc does not exist
    let ipc_provider = RootProvider::<Ethereum, BoxTransport>::connect_builtin(ipc_path).await?;

    let _block_number = ipc_provider.get_block_number().await?;

    Ok(())
}
