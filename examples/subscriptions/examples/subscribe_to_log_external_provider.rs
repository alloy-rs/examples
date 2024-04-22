//! Example of subscribing to logs from the Ethereum network using an external provider.

use alloy::{
    primitives::{address, b256},
    providers::{Provider, ProviderBuilder},
    rpc::{
        client::WsConnect,
        types::eth::{BlockNumberOrTag, Filter},
    },
};
use eyre::Result;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
    let tranfer_event_signature =
        b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");
    let filter = Filter::new()
        .address(uniswap_token_address)
        .event_signature(tranfer_event_signature)
        .from_block(BlockNumberOrTag::Latest);

    let rpc_url = "wss://eth.merkle.io"; // DON'T use wss://eth.merkle.io _> this filters wrongly, tested alchemy.io's to be working fine

    // Create the provider.
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

    let sub = provider.subscribe_logs(&filter).await.expect("Failed to subscribe to logs");
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        println!("Uniswap token logs: {log:?}");
    }

    Ok(())
}
