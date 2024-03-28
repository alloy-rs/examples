//! Example of querying deployed bytecode of a contract on the Ethereum network.

use alloy::{
    network::Ethereum,
    primitives::address,
    providers::{Provider, RootProvider},
    rpc::types::eth::{BlockId, BlockNumberOrTag},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a provider.
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = RootProvider::<Ethereum, _>::new_http(rpc_url);

    // Get the bytecode of the Uniswap V3 USDC-ETH pool on Ethereum mainnet.
    let pool_address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");
    let bytecode =
        provider.get_code_at(pool_address, BlockId::Number(BlockNumberOrTag::Latest)).await?;

    println!("Bytecode: {bytecode:?}");

    Ok(())
}
