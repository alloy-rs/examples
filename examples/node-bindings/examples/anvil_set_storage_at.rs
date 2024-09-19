//! Example of mocking WETH balance of a target account using `anvil_set_storage_at`.

use alloy::{
    node_bindings::Anvil,
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
    // The RPC URL of the node to fork.
    let fork_url = "https://eth.merkle.io";

    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().fork(fork_url).try_spawn()?;
    let provider =
        ProviderBuilder::new().with_recommended_fillers().on_http(anvil.endpoint().parse()?);

    let iweth = IERC20::new(WETH_ADDR, provider.clone());

    // Random empty account.
    let account = address!("F605F9d1cB055E87E30bcAEe4CB9389a35aBe8Ff");

    let balance_before = iweth.balanceOf(account).call().await?._0;
    println!("WETH balance before: {}", balance_before);
    assert_eq!(balance_before, U256::ZERO);

    // Mock WETH balance using the Anvil API.
    let hashed_slot = keccak256((account, U256::from(3)).abi_encode());
    let one_ether: U256 = parse_units("1.0", "ether")?.into();
    provider.anvil_set_storage_at(WETH_ADDR, hashed_slot.into(), one_ether.into()).await?;

    let balance_after = iweth.balanceOf(account).call().await?._0;
    println!("WETH balance after: {}", balance_after);
    assert_eq!(balance_after, one_ether);

    Ok(())
}
