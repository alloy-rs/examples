//! Example of sending a MEV bundle with `eth_sendBundle` via Flashbots authentication.
//!
//! Bundle submission is time-sensitive. This example shows how to:
//! 1. Build and sign a transaction locally
//! 2. Wrap it in an [`EthSendBundle`]
//! 3. Submit it to a relay with the `X-Flashbots-Signature` header via [`MevApi::send_bundle`]
//!
//! See the [Flashbots auction docs](https://docs.flashbots.net/flashbots-auction/searchers/advanced/rpc-endpoint)
//! and Alloy's `provider-mev-api` feature.

use alloy::{
    network::{eip2718::Encodable2718, EthereumWallet, NetworkTransactionBuilder, TransactionBuilder},
    primitives::U256,
    providers::{ext::MevApi, ProviderBuilder},
    rpc::types::{mev::EthSendBundle, TransactionRequest},
    signers::local::PrivateKeySigner,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Flashbots relay endpoint. Other builders expose compatible `eth_sendBundle` APIs.
    let relay_url = "https://relay.flashbots.net".parse()?;

    // Signer used only for the Flashbots `X-Flashbots-Signature` header (identity), not the tx.
    let flashbots_auth = PrivateKeySigner::random();

    // Create a provider pointed at the MEV relay.
    let provider = ProviderBuilder::new().connect_http(relay_url);

    // Build a sample transaction with a separate wallet. This tx is intentionally underfunded so
    // the relay will reject it — the goal is to demonstrate the API, not land a real bundle.
    let signer = PrivateKeySigner::random();
    let wallet = EthereumWallet::from(signer);
    let recipient = PrivateKeySigner::random().address();

    let tx = TransactionRequest::default()
        .with_to(recipient)
        .with_nonce(0)
        .with_chain_id(1)
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_max_fee_per_gas(20_000_000_000);

    let tx_envelope = tx.build(&wallet).await?;
    let raw_tx = tx_envelope.encoded_2718().into();

    // Target the next block. In production, query a regular RPC for the latest block number.
    let target_block = 1u64;

    let bundle = EthSendBundle {
        txs: vec![raw_tx],
        block_number: target_block,
        ..Default::default()
    };

    // `send_bundle` returns a builder; call `.with_auth(signer)` to attach Flashbots signature
    // headers, then await the request.
    match provider.send_bundle(bundle).with_auth(flashbots_auth).await {
        Ok(bundle_hash) => println!("Bundle accepted: {bundle_hash:?}"),
        Err(err) => {
            // Expected for this underfunded demo transaction.
            println!("Bundle submission failed (expected for demo tx): {err}");
        }
    }

    Ok(())
}
