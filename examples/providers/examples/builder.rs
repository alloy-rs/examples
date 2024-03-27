//! Example of using the `ProviderBuilder` to create a provider with a signer and network.

use alloy::{
    network::{EthereumSigner, TransactionBuilder},
    node_bindings::Anvil,
    primitives::U256,
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::{client::RpcClient, types::eth::TransactionRequest},
    signers::wallet::LocalWallet,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    // Set up signer from the first default Anvil account (Alice).
    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    // Create two users, Alice and Bob.
    let alice = wallet.address();
    let bob = anvil.addresses()[1];

    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_client = RpcClient::new_http(anvil.endpoint().parse()?);
    let provider_with_signer = ProviderBuilder::new()
        .signer(EthereumSigner::from(wallet))
        .provider(RootProvider::new(rpc_client));

    // Create a transaction.
    let mut tx = TransactionRequest::default()
        .to(Some(bob))
        .value(U256::from(100))
        .nonce(0)
        .gas_limit(U256::from(21000));

    tx.set_gas_price(U256::from(20e9));

    // Send the transaction and wait for the receipt.
    let pending_tx = provider_with_signer.send_transaction(tx).await?;

    println!("Pending transaction...{:?}", pending_tx.tx_hash());

    // Wait for the transaction to be included.
    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block: {:?}",
        receipt.block_number.expect("Failed to get block number").to_string()
    );

    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));

    Ok(())
}
