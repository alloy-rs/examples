//! Example of signing and sending a transaction using a Ledger device.

use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::ledger::{HDPath, LedgerSigner},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Instantiate the application by acquiring a lock on the Ledger device.
    let signer = LedgerSigner::new(HDPath::LedgerLive(0), Some(1)).await?;
    let wallet = EthereumWallet::from(signer);

    // Create a provider with the wallet.
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider =
        ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(rpc_url);

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
