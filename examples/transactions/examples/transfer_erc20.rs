//! Example of how to transfer ERC20 tokens from one account to another.

use alloy::{
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    sol,
};
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
    let rpc_url = "https://reth-ethereum.ithaca.xyz/rpc";
    let provider =
        ProviderBuilder::new().connect_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url))?;

    // Create two users, Alice and Bob.
    let accounts = provider.get_accounts().await?;
    let alice = accounts[0];
    let bob = accounts[1];

    // Deploy the `ERC20Example` contract.
    let contract = ERC20Example::deploy(provider).await?;

    // Register the balances of Alice and Bob before the transfer.
    let alice_before_balance = contract.balanceOf(alice).call().await?;
    let bob_before_balance = contract.balanceOf(bob).call().await?;

    // Transfer and wait for inclusion.
    let amount = U256::from(100);
    let tx_hash = contract.transfer(bob, amount).send().await?.watch().await?;

    println!("Sent transaction: {tx_hash}");

    // Register the balances of Alice and Bob after the transfer.
    let alice_after_balance = contract.balanceOf(alice).call().await?;
    let bob_after_balance = contract.balanceOf(bob).call().await?;

    // Check the balances of Alice and Bob after the transfer.
    assert_eq!(alice_before_balance - alice_after_balance, amount);
    assert_eq!(bob_after_balance - bob_before_balance, amount);

    Ok(())
}
