//! Example of signing and sending a transaction using a Trezor device.

use alloy::{
    network::{EthereumSigner, TransactionBuilder},
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::request::TransactionRequest,
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
        .with_recommended_fillers()
        .signer(EthereumSigner::from(signer))
        .on_http(rpc_url)?;

    // Build a transaction.
    let tx = TransactionRequest::default()
        .with_value(U256::from(100))
        .with_to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into());

    // Send the transaction and wait for the receipt.
    let receipt =
        provider.send_transaction(tx).await?.with_required_confirmations(3).get_receipt().await?;

    println!("Send transaction: {:?}", receipt.transaction_hash);

    Ok(())
}
