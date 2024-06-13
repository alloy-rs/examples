//! Example of using a keystore wallet to sign and send a transaction.

use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    node_bindings::Anvil,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::LocalSigner,
};
use eyre::Result;
use std::{env, path::PathBuf};

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    // Password to decrypt the keystore file with.
    let password = "test";

    // Set up signer using Alice's keystore file.
    // The private key belongs to Alice, the first default Anvil account.
    let keystore_file_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("examples/keystore/alice.json");
    let signer = LocalSigner::decrypt_keystore(keystore_file_path, password)?;
    let wallet = EthereumWallet::from(signer);

    // Create a provider with the wallet.
    let rpc_url = anvil.endpoint().parse()?;
    let provider =
        ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(rpc_url);

    // Build a transaction to send 100 wei from Alice to Vitalik.
    // The `from` field is automatically filled to the first signer's address (Alice).
    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx = TransactionRequest::default().with_to(vitalik).with_value(U256::from(100));

    // Send the transaction and wait for inclusion.
    let tx_hash = provider.send_transaction(tx).await?.watch().await?;

    println!("Sent transaction: {tx_hash}");

    Ok(())
}
