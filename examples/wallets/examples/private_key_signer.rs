//! Example of using a local wallet to sign and send a transaction.

use alloy::{
    network::TransactionBuilder,
    node_bindings::Anvil,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    // Set up signer from the first default Anvil account (Alice).
    // [RISK WARNING! Writing a private key in the code file is insecure behavior.]
    // The following code is for testing only. Set up signer from private key, be aware of danger.
    // let signer: PrivateKeySigner = "<PRIVATE_KEY>".parse().expect("should parse private key");
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();

    // Create a provider with the wallet.
    let rpc_url = anvil.endpoint_url();
    let provider = ProviderBuilder::new().wallet(signer).connect_http(rpc_url);

    // Build a transaction to send 100 wei from Alice to Vitalik.
    // The `from` field is automatically filled to the first signer's address (Alice).
    let tx = TransactionRequest::default()
        .with_to(address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"))
        .with_value(U256::from(100));

    // Send the transaction and wait for inclusion.
    let tx_hash = provider.send_transaction(tx).await?.watch().await?;

    println!("Sent transaction: {tx_hash}");

    Ok(())
}
