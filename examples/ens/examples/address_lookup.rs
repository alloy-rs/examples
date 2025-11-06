//! Example of looking up ENS names from Ethereum addresses.
use alloy::{ens::ProviderEnsExt, primitives::address, providers::ProviderBuilder};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a provider.
    let rpc_url = "https://reth-ethereum.ithaca.xyz/rpc".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // Vitalik's Ethereum address.
    let vitalik_address = address!("0xd8da6bf26964af9d7eed9e03e53415d37aa96045");

    // Perform reverse ENS lookup to get the ENS name for the address.
    let ens_name = provider.lookup_address(&vitalik_address).await?;

    println!("Address {vitalik_address} resolves to: {ens_name:?}");

    Ok(())
}
