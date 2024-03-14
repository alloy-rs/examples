use alloy_network::{Ethereum, EthereumSigner};
use alloy_node_bindings::Anvil;
use alloy_primitives::{U256, U64};
use alloy_provider::{Provider, ProviderBuilder, RootProvider};
use alloy_rpc_client::RpcClient;
use alloy_rpc_types::TransactionRequest;
use alloy_signer::Wallet;
use alloy_transport_http::Http;
use eyre::Result;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup the HTTP transport which is consumed by the RPC client
    let anvil = Anvil::new().spawn();

    let pk = &anvil.keys()[0];
    let from = anvil.addresses()[0];
    let signer = Wallet::from(pk.to_owned());

    let rpc_client = RpcClient::new(Http::<Client>::new(anvil.endpoint().parse().unwrap()), false);
    let provider_with_signer = ProviderBuilder::<_, Ethereum>::new()
        .signer(EthereumSigner::from(signer))
        .network::<Ethereum>()
        .provider(RootProvider::new(rpc_client));

    let to = anvil.addresses()[1];
    let tx_req = TransactionRequest {
        to: Some(to),
        value: Some(U256::from(100)),
        nonce: Some(U64::from(0)),
        gas_price: Some(U256::from(20e9)),
        gas: Some(U256::from(21000)),
        ..Default::default()
    };

    let pending_tx = provider_with_signer.send_transaction(tx_req).await?;

    println!("Pending transaction...{:?}", pending_tx.tx_hash());
    let receipt = pending_tx.get_receipt().await?;

    assert_eq!(receipt.from, from);

    Ok(())
}
