//! This example demonstrates how to use the [`MulticallBuilder`] to make multicalls using the
//! [`IMulticall3`] contract.

use alloy::{
    primitives::{address, U256},
    providers::{CallItemBuilder, Failure, Provider, ProviderBuilder},
    sol,
};
use IWETH9::IWETH9Instance;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    IWETH9,
    "examples/abi/IWETH9.json"
);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Create a new provider
    let provider = ProviderBuilder::new()
        .on_anvil_with_wallet_and_config(|a| a.fork("https://eth.merkle.io"))?;
    // Deploy the Multicall3 contract
    // let multicall3 = deploy_multicall3(&provider).await;
    // Deploy the WETH contract
    // let weth = deploy_weth(&provider).await;
    let weth =
        IWETH9Instance::new(address!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"), &provider);

    let alice = address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");
    let bob = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");

    let multicall = provider
        .multicall()
        // Set the address of the Multicall3 contract. If unset it uses the default address from <https://github.com/mds1/multicall>: 0xcA11bde05977b3631167028862bE2a173976CA11
        // .address(multicall3)
        // Get the total supply of WETH on our anvil fork.
        .add(weth.totalSupply())
        // Get Alice's WETH balance.
        .add(weth.balanceOf(alice))
        // Also fetch Alice's ETH balance.
        .get_eth_balance(alice);

    let (init_total_supply, alice_weth, alice_eth_bal) = multicall.aggregate().await?;

    println!(
        "Initial total supply: {}, Alice's WETH balance: {}, Alice's ETH balance: {}",
        init_total_supply._0, alice_weth._0, alice_eth_bal.balance
    );

    // Simulate a transfer of WETH from Alice to Bob.
    let wad = U256::from(20);

    // This would fail as Alice doesn't have any WETH.
    let tx = CallItemBuilder::new(weth.transfer(bob, U256::from(10))).allow_failure(true);
    let deposit = CallItemBuilder::new(weth.deposit()).value(wad); // Set the amount of eth that should be deposited into the contract.
    let multicall = provider
        .multicall()
        // Bob's intial WETH balance.
        .add(weth.balanceOf(bob))
        // Attempted WETH transfer from Alice to Bob which would fail.
        .add_call(tx.clone())
        // Alices deposits ETH and mints WETH.
        .add_call(deposit)
        // Attempt transfer again. Succeeds!
        .add_call(tx)
        // Alice's WETH balance after the transfer.
        .add(weth.balanceOf(alice))
        // Bob's final balance.
        .add(weth.balanceOf(bob));

    assert_eq!(multicall.len(), 6);

    // It is important to use `aggregate3_value` as we're trying to simulate calls to payable
    // functions that should be sent a value, using any other multicall3 method would result in an
    // error.
    let (init_bob, failed_transfer, deposit, succ_transfer, alice_weth, bob_weth) =
        multicall.aggregate3_value().await?;

    // Since, `aggregate3_value` allows for calls to fail without reverting, it returns a tuple of
    // results which contain the decoded return value in Ok(_) variant and the `Failure` type in the
    // Err(_) variant.
    assert!(matches!(failed_transfer.unwrap_err(), Failure { idx: 1, return_data: _ }));

    let init_bob = init_bob?;
    assert_eq!(init_bob._0, U256::ZERO);

    assert!(deposit.is_ok());
    assert!(succ_transfer.is_ok());

    let alice_weth = alice_weth?;
    let bob_weth = bob_weth?;

    println!("Alice's WETH balance: {}, Bob's WETH balance: {}", alice_weth._0, bob_weth._0);

    Ok(())
}
