//! Example showing how to send an [EIP-7594](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-7594.md) transaction.

use alloy::{
    consensus::{
        EthereumTxEnvelope, SidecarBuilder, Signed, SimpleCoder, TxEip4844Variant,
        TxEip4844WithSidecar,
    },
    eips::Encodable2718,
    network::{TransactionBuilder, TransactionBuilder4844},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node with the Cancun hardfork enabled.
    // Ensure `anvil` is available in $PATH.
    let provider = ProviderBuilder::new()
        .connect_anvil_with_wallet_and_config(|anvil| anvil.args(["--hardfork", "cancun"]))?;

    // Create two users, Alice and Bob.
    let accounts = provider.get_accounts().await?;
    let alice = accounts[0];
    let bob = accounts[1];

    // Create a sidecar with some data.
    let sidecar: SidecarBuilder<SimpleCoder> = SidecarBuilder::from_slice(b"Blobs are fun!");
    let sidecar = sidecar.build()?;

    // Build a transaction to send the sidecar from Alice to Bob.
    // The `from` field is automatically filled to the first signer's address (Alice).
    let tx = TransactionRequest::default().with_to(bob).with_blob_sidecar(sidecar);

    // Fill the transaction (e.g., nonce, gas, etc.) using the provider and convert it to an envelope.
    let envelope = provider.fill(tx).await?.try_into_envelope()?;

    // Create an EIP-4844 transaction with sidecar, using the data from the envelope(extracting the tx and sidecar).
    let tx_with_sidecar = TxEip4844WithSidecar::from_tx_and_sidecar(
        envelope.as_eip4844().unwrap().tx().tx().clone(),
        envelope.as_eip4844().unwrap().tx().sidecar().unwrap().clone(),
    );

    // Here we now convert the sidecar of the transaction into the EIP-7594 variant.
    // I want to use the conversion helpers added here: https://github.com/alloy-rs/alloy/pull/3040

    // Wrap the transaction in the EIP-4844 variant enum.
    let tx = TxEip4844Variant::TxEip4844WithSidecar(tx_with_sidecar);

    // Now we encode the final transaction with the signature from the envelope.
    let encoded_tx =
        EthereumTxEnvelope::Eip4844(Signed::new_unhashed(tx, *envelope.signature())).encoded_2718();

    // Send the raw transaction to the network.
    let pending_tx = provider.send_raw_transaction(&encoded_tx).await?;

    println!("Pending transaction... {}", pending_tx.tx_hash());

    // // Wait for the transaction to be included and get the receipt.
    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));

    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));

    Ok(())
}
