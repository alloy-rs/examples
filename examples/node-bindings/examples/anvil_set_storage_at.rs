//! Example of mocking WETH balance of a target account using [`AnvilApi::anvil_set_storage_at`].

use alloy::{
    primitives::{address, keccak256, utils::parse_units, Address, U256},
    providers::{ext::AnvilApi, ProviderBuilder},
    sol,
    sol_types::SolValue,
};
use eyre::Result;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract IERC20 {
        function balanceOf(address target) returns (uint256);
    }
);

static WETH_ADDR: Address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let rpc_url = "https://reth-ethereum.ithaca.xyz/rpc";
    let provider = ProviderBuilder::new().connect_anvil_with_config(|anvil| anvil.fork(rpc_url));

    // Create an instance of the WETH contract.
    let iweth = IERC20::new(WETH_ADDR, provider.clone());

    // Random empty account.
    let account = address!("F605F9d1cB055E87E30bcAEe4CB9389a35aBe8Ff");

    // Get the WETH balance of the target account before mocking.
    let balance_before = iweth.balanceOf(account).call().await?;
    println!("WETH balance before: {balance_before}");
    assert_eq!(balance_before, U256::ZERO);

    // Mock WETH balance using the Anvil API.
    let hashed_slot = keccak256((account, U256::from(3)).abi_encode());
    let mocked_balance: U256 = parse_units("1.0", "ether")?.into();
    provider.anvil_set_storage_at(WETH_ADDR, hashed_slot.into(), mocked_balance.into()).await?;

    // Get the WETH balance of the target account after mocking.
    let balance_after = iweth.balanceOf(account).call().await?;
    println!("WETH balance after: {balance_after}");
    assert_eq!(balance_after, mocked_balance);

    Ok(())
}
