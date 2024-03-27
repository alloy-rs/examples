//! Example of using the `ManagedNonceLayer` in the provider.

use alloy::{
    network::{EthereumSigner, TransactionBuilder},
    node_bindings::Anvil,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::{client::RpcClient, types::eth::request::TransactionRequest},
    signers::wallet::LocalWallet,
};
use eyre::Result;

/// The `.with_recommended_layers()` method includes the `GasEstimatorLayer` and the
/// `ManagedNonceLayer`.
///
/// Alternatively, you can add the layers individually using the builder methods:
///
/// ```rust
/// .with_gas_estimation()
/// .with_nonce_management()
/// ```
///
/// or by using the `.layer()` method:
///
/// ```rust
/// .layer(GasEstimatorLayer)
/// .layer(ManagedNonceLayer)
/// ```
///
/// In Ethereum, each transaction has a gas limit that represents the maximum amount of gas that
/// can be used to execute the transaction. The gas limit is used to ensure that transactions are
/// processed in a timely manner and to prevent transactions from using more gas than they are
/// supposed to.
///
/// The gas estimator in Alloy is a layer that helps you automatically populate the gas related
/// fields of a transaction request if they are not set. This can be useful if you want to ensure
/// that the gas fields are set correctly, or if you want to avoid having to manually set them
/// yourself.
///
/// The nonce of a transaction is a number that represents the number of transactions
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
    // Ensure `anvil` is available in $PATH
    let anvil = Anvil::new().try_spawn()?;

    // Set up the wallets.
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let from = wallet.address();

    // Create a provider with the signer.
    let http = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new()
        // Adds the `GasEstimatorLayer` and the `ManagedNonceLayer` layers.
        .with_recommended_layers()
        // Alternatively, you can add the layers individually:
        // .with_gas_estimation()
        // .with_nonce_management()
        .signer(EthereumSigner::from(wallet))
        .on_client(RpcClient::new_http(http));

    // Create an EIP-1559 type transaction.
    // Notice that the `nonce` field is set by the `ManagedNonceLayer`.
    // Notice that without the `GasEstimatorLayer`, you need to set the gas related fields.
    let tx = TransactionRequest::default()
        .with_from(from)
        .with_to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into())
        .with_value(U256::from(100))
        .with_chain_id(anvil.chain_id());

    // Send the transaction, the nonce (0) is automatically managed by the provider.
    let builder = provider.send_transaction(tx.clone()).await?;
    let node_hash = *builder.tx_hash();
    let pending_transaction = provider.get_transaction_by_hash(node_hash).await?;
    assert_eq!(pending_transaction.nonce, 0);

    println!("Transaction sent with nonce: {}", pending_transaction.nonce);

    // Send the transaction, the nonce (1) is automatically managed by the provider.
    let builder = provider.send_transaction(tx).await?;
    let node_hash = *builder.tx_hash();
    let pending_transaction = provider.get_transaction_by_hash(node_hash).await?;
    assert_eq!(pending_transaction.nonce, 1);

    println!("Transaction sent with nonce: {}", pending_transaction.nonce);

    Ok(())
}
