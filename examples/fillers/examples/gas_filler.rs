//! Example of using the `GasFiller` in the provider.

use alloy::{
    network::TransactionBuilder,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::request::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let provider = ProviderBuilder::new()
        // Add the `GasFiller` to the provider.
        // It is generally recommended to use the `.with_recommended_fillers()` method, which
        // includes the `GasFiller`.
        .with_gas_estimation()
        .on_anvil_with_wallet();

    // Build an EIP-1559 type transaction to send 100 wei to Vitalik.
    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx = TransactionRequest::default()
        .with_to(vitalik)
        .with_value(U256::from(100))
        // Notice that without the `NonceFiller`, you need to set `nonce` field.
        .with_nonce(0)
        // Notice that without the `ChainIdFiller`, you need to set the `chain_id` field.
        .with_chain_id(provider.get_chain_id().await?);

    // Send the transaction, the nonce (0) is automatically managed by the provider.
    let builder = provider.send_transaction(tx.clone()).await?;
    let node_hash = *builder.tx_hash();
    let pending_tx =
        provider.get_transaction_by_hash(node_hash).await?.expect("Pending transaction not found");
    assert_eq!(pending_tx.nonce, 0);

    println!("Transaction sent with nonce: {}", pending_tx.nonce);

    // Update the nonce and send the transaction again.
    let tx = tx.with_nonce(1);

    // Send the transaction, the nonce (1) is automatically managed by the provider.
    let builder = provider.send_transaction(tx).await?;
    let node_hash = *builder.tx_hash();
    let pending_tx =
        provider.get_transaction_by_hash(node_hash).await?.expect("Pending transaction not found");
    assert_eq!(pending_tx.nonce, 1);

    println!("Transaction sent with nonce: {}", pending_tx.nonce);

    Ok(())
}
