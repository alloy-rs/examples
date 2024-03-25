//! Example of querying deployed bytecode of a contract on Ethereum network.

use alloy::{
    network::Ethereum,
    primitives::address,
    providers::{Provider, RootProvider},
    rpc::types::eth::{BlockId, BlockNumberOrTag},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://eth.merkle.io".parse().unwrap();
    let provider = RootProvider::<Ethereum, _>::new_http(url);

    // Get bytecode of USDC-ETH Uniswap V3 pool
    let pool_address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");

    let bytecode =
        provider.get_code_at(pool_address, BlockId::Number(BlockNumberOrTag::Latest)).await?;

    println!("Bytecode: {:?}", bytecode);

    Ok(())
}
