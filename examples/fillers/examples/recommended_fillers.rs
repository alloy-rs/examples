//! Example of using the `.with_recommended_fillers()` method in the provider.

use alloy::{
    consensus::Transaction,
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
    // After `alloy 0.11`, the recommended fillers are enabled by default when building the provider
    // with `ProviderBuilder::new()`.
    let provider = ProviderBuilder::new()
        // Adds the `ChainIdFiller`, `GasFiller` and the `NonceFiller` layers.
        // This is the recommended way to set up the provider.
        // One can disable the recommended fillers by calling the `disable_recommended_fillers()`
        // method or building the provider with `ProviderBuilder::default()`.
        .connect_anvil_with_wallet();

    // Build an EIP-1559 type transaction to send 100 wei to Vitalik.
    // Notice that the `nonce` field is set by the `NonceFiller`.
    // Notice that the gas related fields are set by the `GasFiller`.
    // Notice that the `chain_id` field is set by the `ChainIdFiller`.
    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx = TransactionRequest::default().with_to(vitalik).with_value(U256::from(100));

    // Send the transaction, the nonce (0) is automatically managed by the provider.
    let builder = provider.send_transaction(tx.clone()).await?;
    let node_hash = *builder.tx_hash();
    let pending_tx =
        provider.get_transaction_by_hash(node_hash).await?.expect("Pending transaction not found");
    assert_eq!(pending_tx.nonce(), 0);

    println!("Transaction sent with nonce: {}", pending_tx.nonce());

    // Send the transaction, the nonce (1) is automatically managed by the provider.
    let builder = provider.send_transaction(tx).await?;
    let node_hash = *builder.tx_hash();
    let pending_tx =
        provider.get_transaction_by_hash(node_hash).await?.expect("Pending transaction not found");
    assert_eq!(pending_tx.nonce(), 1);

    println!("Transaction sent with nonce: {}", pending_tx.nonce());

    Ok(())
}
