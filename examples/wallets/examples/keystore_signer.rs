//! Example of using a keystore wallet to sign and send a transaction.

use std::path::PathBuf;

use alloy::{
    network::TransactionBuilder,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::LocalSigner,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Password to decrypt the keystore file with.
    let password = "test";

    // Set up signer using Alice's keystore file.
    // The private key belongs to Alice, the first default Anvil account.
    let keystore_file_path =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("examples/keystore/alice.json");
    let signer = LocalSigner::decrypt_keystore(keystore_file_path, password)?;

    // Create a provider with the wallet.
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect_anvil_with_config(|anvil| anvil.block_time(1));

    // Build a transaction to send 100 wei from Alice to Vitalik.
    // The `from` field is automatically filled to the first signer's address (Alice).
    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx = TransactionRequest::default().with_to(vitalik).with_value(U256::from(100));

    // Send the transaction and wait for inclusion.
    let tx_hash = provider.send_transaction(tx).await?.watch().await?;

    println!("Sent transaction: {tx_hash}");

    Ok(())
}
