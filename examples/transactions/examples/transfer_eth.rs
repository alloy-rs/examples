//! Example of how to transfer ETH from one account to another.

use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    node_bindings::Anvil,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().try_spawn()?;

    // Get the RPC URL.
    let rpc_url = anvil.endpoint().parse()?;

    // Set up signer from the first default Anvil account (Alice).
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::from(signer);

    // Create a provider with the wallet.
    let provider =
        ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(rpc_url);

    // Build a transaction to send 100 wei from Alice to Bob.
    // The `from` field is automatically filled to the first signer's address (Alice).
    let bob = anvil.addresses()[1];
    let tx = TransactionRequest::default().with_to(bob).with_value(U256::from(100));

    // Send the transaction and listen for the transaction to be included.
    let tx_hash = provider.send_transaction(tx).await?.watch().await?;

    println!("Sent transaction: {tx_hash}");

    Ok(())
}
