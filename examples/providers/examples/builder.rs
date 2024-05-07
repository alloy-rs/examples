//! Example of using the `ProviderBuilder` to create a provider with a signer and network.

use alloy::{
    network::{EthereumSigner, TransactionBuilder},
    node_bindings::Anvil,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::TransactionRequest,
    signers::wallet::LocalWallet,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    // Set up signer from the first default Anvil account (Alice).
    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    // Create two users, Alice and Bob.
    let alice = wallet.address();
    let bob = anvil.addresses()[1];

    // Set up the HTTP provider with the `reqwest` crate.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(wallet))
        .on_http(rpc_url);

    // Create a transaction.
    let tx =
        TransactionRequest::default().with_from(alice).with_to(bob).with_value(U256::from(100));

    // Send the transaction and wait for the receipt.
    let pending_tx = provider.send_transaction(tx).await?;

    println!("Pending transaction... {}", pending_tx.tx_hash());

    // Wait for the transaction to be included.
    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));

    Ok(())
}
