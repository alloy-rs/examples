//! Example of signing and sending a transaction using a Yubi device.

use alloy::{
    network::{EthereumSigner, TransactionBuilder},
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::wallet::{
        yubihsm::{Connector, Credentials, UsbConfig},
        YubiWallet,
    },
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = "https://eth.merkle.io".parse()?;

    // We use USB for the example, but you can connect over HTTP as well. Refer
    // to the [YubiHSM](https://docs.rs/yubihsm/0.34.0/yubihsm/) docs for more information.
    let connector = Connector::usb(&UsbConfig::default());

    // Instantiate the connection to the YubiKey. Alternatively, use the
    // `from_key` method to upload a key you already have, or the `new` method
    // to generate a new keypair.
    let signer = YubiWallet::connect(connector, Credentials::default(), 0);

    // Create a provider with the signer.
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(signer))
        .on_http(rpc_url);

    // Build a transaction to send 100 wei from Alice to Vitalik.
    // The `from` field is automatically filled to the first signer's address (Alice).
    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx = TransactionRequest::default().with_to(vitalik).with_value(U256::from(100));

    // Send the transaction and wait for inclusion with 3 confirmations.
    let tx_hash =
        provider.send_transaction(tx).await?.with_required_confirmations(3).watch().await?;

    println!("Sent transaction: {tx_hash}");

    Ok(())
}
