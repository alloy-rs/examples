//! Example of using the `ManagedNonceLayer` in the provider.

use alloy::{
    network::{EthereumSigner, TransactionBuilder},
    node_bindings::Anvil,
    primitives::{address, U256},
    providers::{layers::GasEstimatorLayer, Provider, ProviderBuilder},
    rpc::{client::RpcClient, types::eth::request::TransactionRequest},
    signers::wallet::LocalWallet,
};
use eyre::Result;

/// In Ethereum, each transaction has a gas limit that represents the maximum amount of gas that
/// can be used to execute the transaction. The gas limit is used to ensure that transactions are
/// processed in a timely manner and to prevent transactions from using more gas than they are
/// supposed to.
///
/// The gas estimator in Alloy is a layer that helps you automatically populate the gas related
/// fields of a transaction request if they are not set. This can be useful if you want to ensure
/// that the gas fields are set correctly, or if you want to avoid having to manually set them
/// yourself.
#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH
    let anvil = Anvil::new().try_spawn()?;

    // Set up the wallets.
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let from = wallet.address();

    // Create a provider with the signer.
    let http = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new()
        // Add the `GasEstimatorLayer` to the provider.
        // It is generally recommended to use the `.with_recommended_layers()` method, which
        // includes the `GasEstimatorLayer`.
        .layer(GasEstimatorLayer)
        .signer(EthereumSigner::from(wallet))
        .on_client(RpcClient::new_http(http));

    // Create an EIP-1559 type transaction.
    let mut tx = TransactionRequest::default()
        .with_from(from)
        .with_to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into())
        .with_value(U256::from(100))
        // Notice that without the `ManagedNonceLayer`, you need to set the nonce field.
        .with_nonce(0)
        // It is required to set the chain_id for EIP-1559 transactions.
        .with_chain_id(anvil.chain_id());

    // Send the transaction, the nonce (0) is automatically managed by the provider.
    let builder = provider.send_transaction(tx.clone()).await?;
    let node_hash = *builder.tx_hash();
    let pending_transaction = provider.get_transaction_by_hash(node_hash).await?;
    assert_eq!(pending_transaction.nonce, 0);

    println!("Transaction sent with nonce: {}", pending_transaction.nonce);

    // Increase the nonce manually, it is generally recommended to use the `ManagedNonceLayer`
    // through the `with_recommended_layers()` method instead of manually managing the nonce.
    tx.set_nonce(1);

    // Send the transaction, the nonce (1) is automatically managed by the provider.
    let builder = provider.send_transaction(tx).await?;
    let node_hash = *builder.tx_hash();
    let pending_transaction = provider.get_transaction_by_hash(node_hash).await?;
    assert_eq!(pending_transaction.nonce, 1);

    println!("Transaction sent with nonce: {}", pending_transaction.nonce);

    Ok(())
}