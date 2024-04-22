//! Example of how to create and sign a transaction with EthereumSigner.

use alloy::{
    network::{eip2718::Encodable2718, EthereumSigner, TransactionBuilder},
    node_bindings::Anvil,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::TransactionRequest,
    signers::wallet::LocalWallet,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().try_spawn()?;

    // Set up signer from the first default Anvil account (Alice).
    let signer: LocalWallet = anvil.keys()[0].clone().into();

    // Create two users, Alice and Bob.
    let alice = anvil.addresses()[0];
    let bob = anvil.addresses()[1];

    let rpc_url = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url)?;

    // Build a transaction to send 100 wei from Alice to Bob.
    let tx = TransactionRequest::default()
        .with_from(alice)
        .with_to(bob.into())
        .with_nonce(0)
        .with_chain_id(anvil.chain_id())
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000);

    // Build the transaction using the EthereumSigner with the provided signer.
    let to_send = tx.build(&EthereumSigner::from(signer)).await?;

    // Encode the transaction using EIP-2718 encoding.
    let out = to_send.encoded_2718();

    // Send the raw transaction and retrieve the transaction receipt.
    let receipt = provider.send_raw_transaction(&out).await?.get_receipt().await?;

    println!("Send transaction receipt: {receipt:?}");

    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));

    Ok(())
}
