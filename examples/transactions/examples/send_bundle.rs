//! Example of sending a signed transaction bundle with `eth_sendBundle` and Flashbots
//! authentication.
//!
//! Set `RPC_URL` to a regular Ethereum JSON-RPC endpoint and `RELAY_URL` to a compatible
//! bundle relay on the same network. `PRIVATE_KEY` must belong to a funded account.
//! `FLASHBOTS_AUTH_KEY` is a separate, persistent identity key that does not need funds.
//! This example sends a zero-value transaction to the sender itself, spending gas if included.
//! Relay acceptance does not guarantee inclusion; bundles target a specific block.

use alloy::{
    network::{eip2718::Encodable2718, TransactionBuilder},
    providers::{ext::MevApi, Provider, ProviderBuilder},
    rpc::types::{mev::EthSendBundle, TransactionRequest},
    signers::local::PrivateKeySigner,
};
use example_support::{required_env, rpc_url};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let signer = required_env("PRIVATE_KEY")?.parse::<PrivateKeySigner>()?;
    let sender = signer.address();
    let flashbots_auth = required_env("FLASHBOTS_AUTH_KEY")?.parse::<PrivateKeySigner>()?;
    let relay_url = required_env("RELAY_URL")?.parse()?;

    // Use a regular node for chain ID, pending nonce, gas estimation, and fee estimation.
    // A bundle relay generally does not provide these Ethereum RPC methods.
    let provider = ProviderBuilder::new().wallet(signer).connect_http(rpc_url()?.parse()?);
    let relay = ProviderBuilder::new().connect_http(relay_url);
    let tx = TransactionRequest::default().with_from(sender).with_to(sender);

    // Fill and sign locally without broadcasting to the public mempool.
    let envelope = provider.fill(tx).await?.try_into_envelope()?;
    let target_block = provider.get_block_number().await? + 1;
    let bundle = EthSendBundle {
        txs: vec![envelope.encoded_2718().into()],
        block_number: target_block,
        ..Default::default()
    };

    // The builder signs the request body and attaches the X-Flashbots-Signature header.
    // Propagate relay errors so a rejected submission is not reported as success.
    let response = relay.send_bundle(bundle).with_auth(flashbots_auth).await?;
    println!("Bundle accepted for block {target_block}: {response:?}");

    Ok(())
}
