//! Demonstrate the Multicall Batch Layer.
//! Provider layer that aggregates contract calls (`eth_call`) over a time period into a single
//! Multicall3 contract call. This is useful for reducing the number of requests made to the RPC.

use std::time::Duration;

use alloy::{
    primitives::address,
    providers::{layers::CallBatchLayer, Provider, ProviderBuilder},
    sol,
};
use eyre::Result;
use IWETH9::{balanceOfCall, totalSupplyCall, IWETH9Instance};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    IWETH9,
    "examples/abi/IWETH9.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Instantiate a provider with the `CallBatchLayer` enabled.
    let provider = ProviderBuilder::new()
        // Enables `eth_call` batching by leveraging the Multicall3 contract.
        // The `CallBatchLayer` will wait for a certain amount of time before sending a request. See: <https://docs.rs/alloy-provider/latest/alloy_provider/layers/struct.CallBatchLayer.html#method.wait>.
        // This delay is added to aggregate any incoming `eth_calls` that can be together.
        // In this case, we set the delay to 10ms.
        .layer(CallBatchLayer::new().wait(Duration::from_millis(10)))
        // Can also use the shorthand `with_call_batching` on the build which set the delay to 1ms.
        // .with_call_batching()
        .connect_anvil_with_wallet_and_config(|a| a.fork("https://reth-ethereum.ithaca.xyz/rpc"))?;

    // Create a new instance of the IWETH9 contract.
    let weth =
        IWETH9Instance::new(address!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"), &provider);

    let alice = address!("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let bob = address!("0xc7bBeC68d12a0d1830360F8Ec58fA599bA1b0e9b");

    // Calls that will be batched.
    let alice_weth = weth.balanceOf(alice).into_transaction_request();
    let bob_weth = weth.balanceOf(bob).into_transaction_request();
    let total_supply = weth.totalSupply().into_transaction_request();

    // Requests need to be parallelized to be batched.
    let (alice_weth, bob_weth, total_supply, block_number, alice_eth) = tokio::try_join!(
        // Batch `eth_call` requests.
        provider.call(alice_weth).decode_resp::<balanceOfCall>(),
        provider.call(bob_weth).decode_resp::<balanceOfCall>(),
        provider.call(total_supply).decode_resp::<totalSupplyCall>(),
        // Get block number and get balance calls can also be batched.
        provider.get_block_number(),
        provider.get_balance(alice)
    )?;

    // Resolve `Ok`.
    let alice_weth = alice_weth?;
    let bob_weth = bob_weth?;
    let total_supply = total_supply?;

    println!("Block Number: {block_number}");
    println!(
        "Alice's WETH balance: {alice_weth}\nBob's WETH balance: {bob_weth}\nTotal WETH supply: {total_supply}\nAlice's ETH
    balance: {alice_eth}"
    );

    Ok(())
}
