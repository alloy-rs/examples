//! Example of using the `GasFiller` in the provider.

use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    node_bindings::Anvil,
    primitives::{address, U256},
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
    let wallet = EthereumWallet::from(signer);

    // Create a provider with the wallet.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new()
        // Add the `GasFiller` to the provider.
        // It is generally recommended to use the `.with_recommended_fillers()` method, which
        // includes the `GasFiller`.
        .with_gas_estimation()
        .wallet(wallet)
        .on_http(rpc_url);

    // Build an EIP-1559 type transaction to send 100 wei to Vitalik.
    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx = TransactionRequest::default()
        .with_to(vitalik)
        .with_value(U256::from(100))
        // Notice that without the `NonceFiller`, you need to set `nonce` field.
        .with_nonce(0)
        // Notice that without the `ChainIdFiller`, you need to set the `chain_id` field.
        .with_chain_id(anvil.chain_id());

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
