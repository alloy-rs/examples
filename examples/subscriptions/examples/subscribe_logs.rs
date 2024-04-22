//! Example of subscribing and listening for contract events by WebSocket subscription.

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
    // Set up the WS transport which is consumed by the RPC client.
    let rpc_url = "wss://eth-mainnet.g.alchemy.com/v2/your-api-key";

    // Create the provider.
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Create a filter to watch for Uniswap token transfers.
    let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
    let tranfer_event_signature =
        b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");
    let filter = Filter::new()
        .address(uniswap_token_address)
        .event_signature(tranfer_event_signature)
        .from_block(BlockNumberOrTag::Latest);

    // Subscribe to logs.
    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        println!("Uniswap token logs: {log:?}");
    }

    Ok(())
}
