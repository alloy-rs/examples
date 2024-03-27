//! Example of using the `ProviderBuilder` to create a provider with a signer and network.

use alloy::{
    network::{EthereumSigner, TransactionBuilder},
    node_bindings::Anvil,
    primitives::U256,
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::{client::RpcClient, types::eth::TransactionRequest},
    signers::wallet::Wallet,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    let pk = &anvil.keys()[0];
    let from = anvil.addresses()[0];
    let signer = Wallet::from(pk.to_owned());

    // Setup the HTTP transport which is consumed by the RPC client
    let rpc_client = RpcClient::new_http(anvil.endpoint().parse()?);
    let provider_with_signer = ProviderBuilder::new()
        .signer(EthereumSigner::from(signer))
        .provider(RootProvider::new(rpc_client));

    let to = anvil.addresses()[1];

    let mut tx_req = TransactionRequest::default()
        .to(Some(to))
        .value(U256::from(100))
        .nonce(0)
        .gas_limit(U256::from(21000));

    tx_req.set_gas_price(U256::from(20e9));

    let pending_tx = provider_with_signer.send_transaction(tx_req).await?;

    println!("Pending transaction...{:?}", pending_tx.tx_hash());

    let receipt = pending_tx.get_receipt().await?;

    assert_eq!(receipt.from, from);

    Ok(())
}
