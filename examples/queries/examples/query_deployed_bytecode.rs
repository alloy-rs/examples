//! Example of querying deployed bytecode of a contract on the Ethereum network.

use alloy::{
    primitives::address,
    providers::{Provider, ProviderBuilder},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a provider.
    let rpc_url = "https://reth-ethereum.ithaca.xyz/rpc".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // Get the bytecode of the Uniswap V3 USDC-ETH pool on Ethereum mainnet.
    let pool_address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");
    let bytecode = provider.get_code_at(pool_address).await?;

    println!("Bytecode: {bytecode:?}");

    Ok(())
}
