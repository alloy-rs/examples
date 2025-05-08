//! Example of sending a private transaction using Flashbots Protect.

use alloy::{
    network::{eip2718::Encodable2718, EthereumWallet, TransactionBuilder},
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up the HTTP transport which is consumed by the RPC client.
    //
    // By default, Flashbots Protect transactions are only shared with the Flashbots Builder, which
    // builds only a subset of all Ethereum blocks. In `fast` mode, transactions are shared with
    // all registered builders no less than one block after they are received to increase the
    // number of blocks the user's transaction can be included in.
    //
    // Fast mode has 2 key differences from the default Protect experience:
    // - Shared with all builders: By default, Flashbots Protect transactions are only shared with
    //   the Flashbots Builder, which builds only a subset of all Ethereum blocks. In fast mode,
    //   transactions are shared with all registered builders no less than one block after they are
    //   received to increase the number of blocks the user's transaction can be included in.
    // - Larger refund paid to validator: By default, only 10% of MEV-Share refunds are paid to
    //   validators. In fast mode, validators receive 50% of refunds which makes it more likely that
    //   the user’s transactions will be chosen in a given block.
    //
    // For more information, see the [Flashbots documentation](https://docs.flashbots.net/flashbots-protect/overview).
    //
    // To use `fast` mode change the URL to `https://rpc.flashbots.net/fast`.
    let flashbots_url = "https://rpc.flashbots.net".parse()?;

    // Create a provider.
    let provider = ProviderBuilder::new().connect_http(flashbots_url);

    // Create a signer from a random private key.
    let signer = PrivateKeySigner::random();
    let wallet = EthereumWallet::from(signer);

    // Build a transaction to send 100 wei from Alice to Bob.
    // The `from` field is automatically filled to the first signer's address (Alice).
    let bob = PrivateKeySigner::random().address();
    let tx = TransactionRequest::default()
        .with_to(bob)
        .with_nonce(0)
        .with_chain_id(1)
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000);

    // Build the transaction with the provided wallet. Flashbots Protect requires the transaction to
    // be signed locally and send using `eth_sendRawTransaction`.
    let tx_envelope = tx.build(&wallet).await?;

    // Encode the transaction using EIP-2718 encoding.
    let tx_encoded = tx_envelope.encoded_2718();

    // Send the raw transaction. The transaction is sent to the Flashbots relay and, if valid, will
    // be included in a block by a Flashbots builder. Note that the transaction request, as defined,
    // is invalid and will not be included in the blockchain.
    let pending = provider.send_raw_transaction(&tx_encoded).await?.register().await?;

    println!("Sent transaction: {}", pending.tx_hash());

    Ok(())
}
