//! Example of signing, encoding and sending a raw transaction using a wallet.

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
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().try_spawn()?;

    // Create a provider.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Create a signer from the first default Anvil account (Alice).
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let signer: EthereumSigner = wallet.into();

    // Create two users, Alice and Bob.
    let alice = anvil.addresses()[0];
    let bob = anvil.addresses()[1];

    // Build a transaction to send 100 wei from Alice to Bob.
    let tx = TransactionRequest::default()
        .with_from(alice)
        .with_to(bob)
        .with_nonce(0)
        .with_chain_id(anvil.chain_id())
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000);

    // Build the transaction using the EthereumSigner with the provided signer.
    let tx_envelope = tx.build(&signer).await?;

    // Encode the transaction using EIP-2718 encoding.
    let tx_encoded = tx_envelope.encoded_2718();

    // Send the raw transaction and retrieve the transaction receipt.
    let receipt = provider.send_raw_transaction(&tx_encoded).await?.get_receipt().await?;

    println!("Send transaction: {}", receipt.transaction_hash);

    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));

    Ok(())
}
