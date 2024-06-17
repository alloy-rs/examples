//! Example showing how to send an [EIP-4844](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-4844.md) transaction.

use alloy::{
    consensus::{SidecarBuilder, SimpleCoder},
    eips::eip4844::DATA_GAS_PER_BLOB,
    network::TransactionBuilder,
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node with the Cancun hardfork enabled.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().args(["--hardfork", "cancun"]).try_spawn()?;

    // Create a provider.
    let provider = ProviderBuilder::new().on_builtin(&anvil.endpoint()).await?;

    // Create two users, Alice and Bob.
    let alice = anvil.addresses()[0];
    let bob = anvil.addresses()[1];

    // Create a sidecar with some data.
    let sidecar: SidecarBuilder<SimpleCoder> =
        SidecarBuilder::from_slice("Blobs are fun!".as_bytes());
    let sidecar = sidecar.build()?;

    // Build a transaction to send the sidecar from Alice to Bob.
    // The `from` field is automatically filled to the first signer's address (Alice).
    let gas_price = provider.get_gas_price().await?;
    let eip1559_est = provider.estimate_eip1559_fees(None).await?;
    let tx = TransactionRequest::default()
        .with_to(bob)
        .with_nonce(0)
        .with_max_fee_per_blob_gas(gas_price)
        .with_max_fee_per_gas(eip1559_est.max_fee_per_gas)
        .with_max_priority_fee_per_gas(eip1559_est.max_priority_fee_per_gas)
        .with_blob_sidecar(sidecar);

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
    assert_eq!(
        receipt.blob_gas_used.expect("Expected to be EIP-4844 transaction"),
        DATA_GAS_PER_BLOB as u128
    );

    Ok(())
}
