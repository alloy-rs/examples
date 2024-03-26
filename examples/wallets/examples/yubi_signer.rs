//! Example of signing and sending a transaction using a Yubi device.

use alloy::{
    network::EthereumSigner,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::{client::RpcClient, types::eth::request::TransactionRequest},
    signers::wallet::{
        yubihsm::{Connector, Credentials, UsbConfig},
        YubiWallet,
    },
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // We use USB for the example, but you can connect over HTTP as well. Refer
    // to the [YubiHSM](https://docs.rs/yubihsm/0.34.0/yubihsm/) docs for more information.
    let connector = Connector::usb(&UsbConfig::default());

    // Instantiate the connection to the YubiKey. Alternatively, use the
    // `from_key` method to upload a key you already have, or the `new` method
    // to generate a new keypair.
    let signer = YubiWallet::connect(connector, Credentials::default(), 0);

    // Create a provider with the signer.
    let http = "http://localhost:8545".parse()?;
    let provider = ProviderBuilder::new()
        .signer(EthereumSigner::from(signer))
        .on_client(RpcClient::new_http(http));

    // Create a transaction.
    let tx = TransactionRequest {
        value: Some(U256::from(100)),
        to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into(),
        gas_price: Some(U256::from(20e9)),
        gas: Some(U256::from(21000)),
        ..Default::default()
    };

    // Broadcast the transaction and wait for the receipt.
    let receipt =
        provider.send_transaction(tx).await?.with_required_confirmations(3).get_receipt().await?;

    println!("Send transaction: {:?}", receipt.transaction_hash);

    Ok(())
}
