//! Example of using the `SignerLayer` in the provider.

use alloy_network::EthereumSigner;
use alloy_node_bindings::Anvil;
use alloy_primitives::{address, b256, U256, U64};
use alloy_provider::{Provider, ProviderBuilder, RootProvider};
use alloy_rpc_client::RpcClient;
use alloy_rpc_types::request::TransactionRequest;
use alloy_signer_wallet::LocalWallet;
use alloy_transport_http::Http;
use eyre::Result;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH
    let anvil = Anvil::new().try_spawn()?;

    // Set up the wallets.
    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    // Create a provider with a signer and the network.
    let http = Http::<Client>::new(anvil.endpoint().parse()?);
    let provider = ProviderBuilder::new()
        // Add the `SignerLayer` to the provider
        .signer(EthereumSigner::from(wallet))
        .provider(RootProvider::new(RpcClient::new(http, true)));

    let tx = TransactionRequest {
        nonce: Some(U64::from(0)),
        value: Some(U256::from(100)),
        to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into(),
        gas_price: Some(U256::from(20e9)),
        gas: Some(U256::from(21000)),
        ..Default::default()
    };

    let builder = provider.send_transaction(tx).await?;
    let node_hash = *builder.tx_hash();

    println!(
        "Node hash matches expected hash: {}",
        node_hash == b256!("eb56033eab0279c6e9b685a5ec55ea0ff8d06056b62b7f36974898d4fbb57e64")
    );

    let pending = builder.register().await?;
    let pending_transaction_hash = *pending.tx_hash();

    println!(
        "Pending transaction hash matches node hash: {}",
        pending_transaction_hash == node_hash
    );

    let transaction_hash = pending.await?;
    assert_eq!(transaction_hash, node_hash);

    println!("Transaction hash matches node hash: {}", transaction_hash == node_hash);

    let receipt =
        provider.get_transaction_receipt(transaction_hash).await.unwrap().expect("no receipt");
    let receipt_hash = receipt.transaction_hash.expect("no receipt hash");

    println!("Receipt hash matches node hash: {}", receipt_hash == node_hash);

    Ok(())
}