//! Example of how to transfer ETH from one account to another.

use alloy::{
    network::TransactionBuilder,
    node_bindings::Anvil,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::TransactionRequest,
};
use eyre::Result;

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

    // Build a transaction to send 100 wei from Alice to Bob.
    let tx =
        TransactionRequest::default().with_from(alice).with_to(bob).with_value(U256::from(100));

    // Send the transaction and wait for the receipt.
    let pending_tx = provider.send_transaction(tx).await?;

    println!("Pending transaction...{:?}", pending_tx.tx_hash());

    // Wait for the transaction to be included.
    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block: {:?}",
        receipt.block_number.expect("Failed to get block number").to_string()
    );

    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));

    Ok(())
}
