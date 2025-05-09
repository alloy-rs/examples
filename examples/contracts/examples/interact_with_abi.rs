//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.

use alloy::{primitives::address, providers::ProviderBuilder, sol};
use eyre::Result;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IWETH9,
    "examples/abi/IWETH9.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let rpc_url = "https://reth-ethereum.ithaca.xyz/rpc";
    let provider =
        ProviderBuilder::new().connect_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url))?;

    // Create a contract instance.
    let contract = IWETH9::new(address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"), provider);

    // Call the contract, retrieve the total supply.
    let total_supply = contract.totalSupply().call().await?;

    println!("WETH total supply is {total_supply}");

    Ok(())
}
