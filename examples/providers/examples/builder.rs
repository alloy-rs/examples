//! Example of using the `ProviderBuilder` to create a provider with a signer and network.

use alloy::{
    network::TransactionBuilder,
    node_bindings::Anvil,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    // Set up signer from the first default Anvil account (Alice).
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();

    // Create two users, Alice and Bob.
    let alice = signer.address();
    let bob = anvil.addresses()[1];

    // Set up the HTTP provider with the `reqwest` crate.
    let rpc_url = anvil.endpoint_url();
    let provider = ProviderBuilder::new().wallet(signer).connect_http(rpc_url);

    // Create a transaction.
    let tx = TransactionRequest::default().with_to(bob).with_value(U256::from(100));

    // Send the transaction and wait for the broadcast.
    let pending_tx = provider.send_transaction(tx).await?;

    println!("Pending transaction... {}", pending_tx.tx_hash());

    // Wait for the transaction to be included and get the receipt.
    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));

    Ok(())
}
