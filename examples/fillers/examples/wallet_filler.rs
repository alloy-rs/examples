//! Example of using the `WalletFiller` in the provider.

use alloy::{
    network::TransactionBuilder,
    node_bindings::Anvil,
    primitives::{address, b256, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::request::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().try_spawn()?;

    // Set up signer from the first default Anvil account (Alice).
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();

    // Create a provider with the wallet.
    let rpc_url = anvil.endpoint_url();
    let provider = ProviderBuilder::new()
        // Add the `WalletFiller` to the provider
        .wallet(signer)
        .connect_http(rpc_url);

    // Build an EIP-1559 type transaction to send 100 wei to Vitalik.
    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx = TransactionRequest::default()
        .with_to(vitalik)
        .with_value(U256::from(100))
        // Notice that without the `ChainIdFiller`, you need to set the `chain_id` field.
        .with_chain_id(provider.get_chain_id().await?)
        // Notice that without the `NonceFiller`, you need to manually set the nonce field.
        .with_nonce(0)
        // Notice that without the `GasFiller`, you need to set the gas related fields.
        .max_fee_per_gas(20_000_000_000)
        .max_priority_fee_per_gas(1_000_000_000)
        .with_gas_limit(21_000);

    let builder = provider.send_transaction(tx).await?;
    let node_hash = *builder.tx_hash();

    println!(
        "Node hash matches expected hash: {}",
        node_hash == b256!("eb56033eab0279c6e9b685a5ec55ea0ff8d06056b62b7f36974898d4fbb57e64")
    );

    // Send the transaction and wait for the broadcast.
    let pending_tx = builder.register().await?;

    println!("Pending transaction hash matches node hash: {}", *pending_tx.tx_hash() == node_hash);

    let tx_hash = pending_tx.await?;
    assert_eq!(tx_hash, node_hash);

    println!("Transaction hash matches node hash: {}", tx_hash == node_hash);

    // Wait for the transaction to be included and get the receipt.
    let receipt =
        provider.get_transaction_receipt(tx_hash).await?.expect("Transaction receipt not found");
    let receipt_hash = receipt.transaction_hash;
    assert_eq!(receipt_hash, node_hash);

    println!("Transaction receipt hash matches node hash: {}", receipt_hash == node_hash);

    Ok(())
}
