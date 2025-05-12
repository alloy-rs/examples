//! Example of signing, encoding and sending a raw transaction using a wallet.

use alloy::{
    network::TransactionBuilder,
    primitives::U256,
    providers::{Provider, ProviderBuilder, WalletProvider},
    rpc::types::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let provider = ProviderBuilder::new().connect_anvil_with_wallet();

    // Create two users, Alice and Bob.
    let accounts = provider.get_accounts().await?;
    let alice = accounts[0];
    let bob = accounts[1];

    // Build a transaction to send 100 wei from Alice to Bob.
    // The `from` field is automatically filled to the first signer's address (Alice).
    let tx = TransactionRequest::default()
        .with_to(bob)
        .with_nonce(0)
        .with_chain_id(provider.get_chain_id().await?)
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000);

    // Build and sign the transaction using the `EthereumWallet` with the provided wallet.
    let tx_envelope = tx.build(&provider.wallet()).await?;

    // Send the raw transaction and retrieve the transaction receipt.
    // [Provider::send_tx_envelope] is a convenience method that encodes the transaction using
    // EIP-2718 encoding and broadcasts it to the network using [Provider::send_raw_transaction].
    let receipt = provider.send_tx_envelope(tx_envelope).await?.get_receipt().await?;

    println!("Sent transaction: {}", receipt.transaction_hash);

    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));

    Ok(())
}
