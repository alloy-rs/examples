//! Example of using the IPC provider to get the latest block number.

use alloy::{
    network::Ethereum,
    providers::{Provider, RootProvider},
    transports::ipc::IpcConnect,
};
use alloy_rpc_client::RpcClient;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup the IPC transport which is consumed by the RPC client
    let ipc_path = "/tmp/reth.ipc";

    // IPC transport
    let ipc = IpcConnect::new(ipc_path.to_string());

    // RPC client using IPC transport
    let ipc_client = RpcClient::connect_pubsub(ipc).await?;

    let provider = RootProvider::<Ethereum, _>::new(ipc_client);

    let latest_block = provider.get_block_number().await?;

    println!("Latest block: {}", latest_block);

    Ok(())
}
