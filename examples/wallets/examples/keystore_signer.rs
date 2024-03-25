//! Example of using a keystore wallet to sign and broadcast a transaction on a local Anvil node.

use alloy::{
    network::EthereumSigner,
    node_bindings::Anvil,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::{client::RpcClient, types::eth::request::TransactionRequest},
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

    // Set up a wallet using Alice's keystore file.
    // The private key belongs to Alice, the first default Anvil account.
    let keystore_file_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("examples/keystore/alice.json");
    let wallet = Wallet::decrypt_keystore(keystore_file_path, password)?;

    // Create a provider with the signer.
    let http = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new()
        .signer(EthereumSigner::from(wallet))
        .on_client(RpcClient::new_http(http));

    // Create a transaction.
    let tx = TransactionRequest {
        value: Some(U256::from(100)),
        to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into(),
        nonce: Some(0),
        gas_price: Some(U256::from(20e9)),
        gas: Some(U256::from(21000)),
        ..Default::default()
    };

    // Broadcast the transaction and wait for the receipt.
    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    println!("Send transaction: {:?}", receipt.transaction_hash);

    Ok(())
}
