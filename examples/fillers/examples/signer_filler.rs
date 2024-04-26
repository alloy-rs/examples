//! Example of using the `SignerFiller` in the provider.

use alloy::{
    network::{EthereumSigner, TransactionBuilder},
    node_bindings::Anvil,
    primitives::{address, b256, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::request::TransactionRequest,
    signers::wallet::LocalWallet,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().try_spawn()?;

    // Set up signer from the first default Anvil account (Alice).
    let signer: LocalWallet = anvil.keys()[0].clone().into();

    // Create two users, Alice and Vitalik.
    let alice = signer.address();
    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

    // Create a provider with the signer.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new()
        // Add the `SignerFiller` to the provider
        .signer(EthereumSigner::from(signer))
        .on_http(rpc_url);

    // Build a legacy type transaction to send 100 wei to Vitalik.
    let tx = TransactionRequest::default()
        .with_from(alice)
        // Notice that without the `NonceFiller`, you need to manually set the nonce field.
        .with_nonce(0)
        .with_to(vitalik)
        .with_value(U256::from(100))
        // Notice that without the `GasFiller`, you need to set the gas related fields.
        .with_gas_price(20_000_000_000)
        .with_gas_limit(21_000);

    let builder = provider.send_transaction(tx).await?;
    let node_hash = *builder.tx_hash();

    println!(
        "Node hash matches expected hash: {}",
        node_hash == b256!("eb56033eab0279c6e9b685a5ec55ea0ff8d06056b62b7f36974898d4fbb57e64")
    );

    let pending_tx = builder.register().await?;
    let pending_tx_hash = *pending_tx.tx_hash();

    println!("Pending transaction hash matches node hash: {}", pending_tx_hash == node_hash);

    let tx_hash = pending_tx.await?;
    assert_eq!(tx_hash, node_hash);

    println!("Transaction hash matches node hash: {}", tx_hash == node_hash);

    let receipt = provider.get_transaction_receipt(tx_hash).await?.expect("Receipt not found");
    let receipt_hash = receipt.transaction_hash;
    assert_eq!(receipt_hash, node_hash);

    println!("Transaction receipt hash matches node hash: {}", receipt_hash == node_hash);

    Ok(())
}
