//! Example of resolving ENS names to Ethereum addresses.

use alloy::{ens::ProviderEnsExt, providers::ProviderBuilder};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a provider.
    let rpc_url = "https://reth-ethereum.ithaca.xyz/rpc".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // Resolve the ENS name "vitalik.eth" to its Ethereum address.
    let address = provider.resolve_name("vitalik.eth").await?;

    println!("vitalik.eth resolves to: {address:?}");

    Ok(())
}
