//! Example of how to transfer ETH from one account to another.

use alloy::{
    network::TransactionBuilder,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let provider = ProviderBuilder::new().connect_anvil_with_wallet();

    // Create two users, Alice and Bob.
    let accounts = provider.get_accounts().await?;
    let alice = accounts[0];
    let bob = accounts[1];

    // Build a transaction to send 100 wei from Alice to Bob.
    // The `from` field is automatically filled to the first signer's address (Alice).
    let tx =
        TransactionRequest::default().with_from(alice).with_to(bob).with_value(U256::from(100));

    // Send the transaction and listen for the transaction to be included.
    let tx_hash = provider.send_transaction(tx).await?.watch().await?;

    println!("Sent transaction: {tx_hash}");

    Ok(())
}
