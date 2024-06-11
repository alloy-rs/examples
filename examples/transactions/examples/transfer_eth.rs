//! Example of how to transfer ETH from one account to another.

use alloy::{
    network::{EthereumSigner, TransactionBuilder},
    node_bindings::Anvil,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::wallet::LocalWallet,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().try_spawn()?;

    // Get the RPC URL.
    let rpc_url = anvil.endpoint().parse()?;

    // Set up wallet from the first default Anvil account (Alice).
    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    // Create two users, Alice and Bob.
    let alice = wallet.address();
    let bob = anvil.addresses()[1];

    // Create a provider with a signer and the network.
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(wallet))
        .on_http(rpc_url);

    // Build a transaction to send 100 wei from Alice to Bob.
    let tx =
        TransactionRequest::default().with_from(alice).with_to(bob).with_value(U256::from(100));

    // Send the transaction and listen for the transaction to be included.
    let tx_hash = provider.send_transaction(tx).await?.watch().await?;

    println!("Send transaction... {}", tx_hash);

    Ok(())
}
