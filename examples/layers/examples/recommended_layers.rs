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
