//! Example of signing and sending a transaction using a Trezor device.

use alloy::{
    network::EthereumSigner,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::{client::RpcClient, types::eth::request::TransactionRequest},
    signers::trezor::{HDPath, TrezorSigner},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Instantiate the application by acquiring a lock on the Trezor device.
    let signer = TrezorSigner::new(HDPath::TrezorLive(0), Some(1)).await?;

    // Create a provider with the signer.
    let rpc_url = "http://localhost:8545".parse()?;
    let provider = ProviderBuilder::new()
        .signer(EthereumSigner::from(signer))
        .on_client(RpcClient::new_http(rpc_url));

    // Create a transaction.
    let tx = TransactionRequest {
        value: Some(U256::from(100)),
        to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into(),
        gas_price: Some(U256::from(20e9)),
        gas: Some(U256::from(21000)),
        ..Default::default()
    };

    // Send the transaction and wait for the receipt.
    let receipt =
        provider.send_transaction(tx).await?.with_required_confirmations(3).get_receipt().await?;

    println!("Send transaction: {:?}", receipt.transaction_hash);

    Ok(())
}
