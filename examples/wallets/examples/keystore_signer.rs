//! Example of using a keystore wallet to sign and send a transaction.

use alloy::{
    network::{EthereumSigner, TransactionBuilder},
    node_bindings::Anvil,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::request::TransactionRequest,
    signers::wallet::Wallet,
};
use eyre::Result;
use std::{env, path::PathBuf};

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up an Anvil node.
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    // Password to decrypt the keystore file with.
    let password = "test";

    // Set up signer using Alice's keystore file.
    // The private key belongs to Alice, the first default Anvil account.
    let keystore_file_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("examples/keystore/alice.json");
    let signer = Wallet::decrypt_keystore(keystore_file_path, password)?;
    let alice = signer.address();

    // Create a provider with the signer.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(signer))
        .on_http(rpc_url)?;

    // Build a transaction to send 100 wei to Vitalik from Alice.
    let tx = TransactionRequest::default()
        .with_from(alice)
        .with_to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"))
        .with_value(U256::from(100));

    // Send the transaction and wait for the receipt.
    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    println!("Send transaction: {:?}", receipt.transaction_hash);

    Ok(())
}
