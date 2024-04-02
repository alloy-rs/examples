//! Example of using the IPC provider to get the latest block number.

use alloy::{
    network::Ethereum,
    providers::{Provider, RootProvider},
    rpc::client::RpcClient,
    transports::ipc::IpcConnect,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up the IPC transport which is consumed by the RPC client.
    let ipc_path = "/tmp/reth.ipc";

    // Create the IPC connection object.
    let ipc = IpcConnect::new(ipc_path.to_string());

    // Connect to the IPC client.
    let ipc_client = RpcClient::connect_pubsub(ipc).await?;

    // Create the provider.
    let provider = RootProvider::<_, Ethereum>::new(ipc_client);

    let latest_block = provider.get_block_number().await?;

    println!("Latest block: {latest_block}");

    Ok(())
}
