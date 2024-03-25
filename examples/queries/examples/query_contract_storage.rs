//! Example of querying contract storage from the Ethereum network.

use alloy::{
    network::Ethereum,
    primitives::{address, U256},
    providers::{Provider, RootProvider},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://eth.merkle.io".parse().unwrap();
    let provider = RootProvider::<Ethereum, _>::new_http(url);

    // Get slot0 from USDC-ETH Uniswap V3 pool
    let pool_address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");

    let storage_slot = U256::from(0);

    let storage = provider.get_storage_at(pool_address, storage_slot, None).await?;

    println!("Slot 0: {:?}", storage);

    Ok(())
}
