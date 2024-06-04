//! Example of how to transfer ERC20 tokens from one account to another.

use alloy::{node_bindings::Anvil, primitives::U256, providers::ProviderBuilder, sol};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20Example,
    "examples/artifacts/ERC20Example.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().fork("https://eth.merkle.io").try_spawn()?;

    // Create a provider.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Create two users, Alice and Bob.
    let alice = anvil.addresses()[0];
    let bob = anvil.addresses()[1];

    // Deploy the `ERC20Example` contract.
    let contract = ERC20Example::deploy(provider).await?;

    // Register the balances of Alice and Bob before the transfer.
    let alice_before_balance = contract.balanceOf(alice).call().await?._0;
    let bob_before_balance = contract.balanceOf(bob).call().await?._0;

    // Transfer and wait for the receipt.
    let amount = U256::from(100);
    let receipt = contract.transfer(bob, amount).send().await?.get_receipt().await?;

    println!("Send transaction: {}", receipt.transaction_hash);

    // Register the balances of Alice and Bob after the transfer.
    let alice_after_balance = contract.balanceOf(alice).call().await?._0;
    let bob_after_balance = contract.balanceOf(bob).call().await?._0;

    // Check the balances of Alice and Bob after the transfer.
    assert_eq!(alice_before_balance - alice_after_balance, amount);
    assert_eq!(bob_after_balance - bob_before_balance, amount);

    Ok(())
}
