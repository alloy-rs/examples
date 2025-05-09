//! Example of using the `NonceFiller` in the provider.

use alloy::{
    consensus::Transaction,
    network::TransactionBuilder,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::request::TransactionRequest,
};
use eyre::Result;

/// In Ethereum, the nonce of a transaction is a number that represents the number of transactions
/// that have been sent from a particular account. The nonce is used to ensure that transactions are
/// processed in the order they are intended, and to prevent the same transaction from being
/// processed multiple times.
///
/// The nonce manager in Alloy is a layer that helps you manage the nonce
/// of transactions by keeping track of the current nonce for a given account and automatically
/// incrementing it as needed. This can be useful if you want to ensure that transactions are sent
/// in the correct order, or if you want to avoid having to manually manage the nonce yourself.
#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    // ProviderBuilder::new() enables the recommended fillers by default (ChainIdFiller, GasFiller
    // and NonceFiller).
    let provider = ProviderBuilder::new()
        // You can disable the recommended fillers by calling the `disable_recommended_fillers()`
        // and pick the fillers of your choice.
        .disable_recommended_fillers()
        // Add the `NonceFiller` to the provider.
        // It is generally recommended to use the recommended fillers which includes the
        // NonceFiller, enabled by building the provider using ProviderBuilder::new().
        //
        // The `NonceFiller` has two types: `Cached` and `Simple`.
        // Unlike `Cached`, `Simple` does not store the transaction count locally,
        // which results in more frequent calls to the provider, but it is more resilient to chain
        // reorganizations.
        .with_cached_nonce_management()
        // .with_simple_nonce_management()
        .connect_anvil_with_wallet();

    // Build an EIP-1559 type transaction to send 100 wei to Vitalik.
    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx = TransactionRequest::default()
        .with_to(vitalik)
        .with_value(U256::from(100))
        // Notice that without the `GasFiller`, you need to set the gas related fields.
        .with_gas_limit(21_000)
        .with_max_fee_per_gas(20_000_000_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        // Notice that without the `ChainIdFiller`, you need to set the `chain_id` field.
        .with_chain_id(provider.get_chain_id().await?);

    // Send the transaction, the nonce (0) is automatically managed by the provider.
    let builder = provider.send_transaction(tx.clone()).await?;
    let node_hash = *builder.tx_hash();
    let pending_tx =
        provider.get_transaction_by_hash(node_hash).await?.expect("Transaction not found");
    assert_eq!(pending_tx.nonce(), 0);

    println!("Transaction sent with nonce: {}", pending_tx.nonce());

    // Send the transaction, the nonce (1) is automatically managed by the provider.
    let builder = provider.send_transaction(tx).await?;
    let node_hash = *builder.tx_hash();
    let pending_tx =
        provider.get_transaction_by_hash(node_hash).await?.expect("Transaction not found");
    assert_eq!(pending_tx.nonce(), 1);

    println!("Transaction sent with nonce: {}", pending_tx.nonce());

    Ok(())
}
