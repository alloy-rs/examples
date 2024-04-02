//! Example of using the IPC provider to get the latest block number.

use alloy::providers::{Provider, ProviderBuilder};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up the IPC transport which is consumed by the RPC client.
    let ipc_path = "/tmp/reth.ipc";

    // Create the provider.
    let provider = ProviderBuilder::new().on_builtin(ipc_path).await?;

    let latest_block = provider.get_block_number().await?;

    println!("Latest block: {latest_block}");

    Ok(())
}
