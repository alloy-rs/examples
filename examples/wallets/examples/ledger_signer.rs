//! Example of signing and sending a transaction using a Ledger device.

use alloy::{
    network::{EthereumSigner, TransactionBuilder},
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::request::TransactionRequest,
    signers::{
        ledger::{HDPath, LedgerSigner},
        Signer,
    },
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Instantiate the application by acquiring a lock on the Ledger device.
    let signer = LedgerSigner::new(HDPath::LedgerLive(0), Some(1)).await?;
    let from = signer.address();

    // Create a provider with the signer.
    let rpc_url = "http://localhost:8545".parse()?;
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(signer))
        .on_http(rpc_url)?;

    // Build a transaction to send 100 wei to Vitalik.
    let tx = TransactionRequest::default()
        .with_from(from)
        .with_to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into())
        .with_value(U256::from(100));

    // Send the transaction and wait for the receipt.
    let receipt =
        provider.send_transaction(tx).await?.with_required_confirmations(3).get_receipt().await?;

    println!("Send transaction: {:?}", receipt.transaction_hash);

    Ok(())
}
