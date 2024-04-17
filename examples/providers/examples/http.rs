//! Example of using the HTTP provider with the `reqwest` crate to get the latest block number.

use alloy::providers::{Provider, ProviderBuilder};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = "https://eth.merkle.io".parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url)?;

    // Get latest block number.
    let latest_block = provider.get_block_number().await?;

    println!("Latest block number: {latest_block}");

    Ok(())
}
