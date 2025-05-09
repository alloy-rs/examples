//! Example of subscribing and listening for specific contract events by `WebSocket` subscription.

use alloy::{
    primitives::address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
};
use eyre::Result;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    // Create the provider.
    let rpc_url = "wss://eth-mainnet.g.alchemy.com/v2/your-api-key";
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    // Create a filter to watch for UNI token transfers.
    let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
    let filter = Filter::new()
        .address(uniswap_token_address)
        // By specifying an `event` or `event_signature` we listen for a specific event of the
        // contract. In this case the `Transfer(address,address,uint256)` event.
        .event("Transfer(address,address,uint256)")
        .from_block(BlockNumberOrTag::Latest);

    // Subscribe to logs.
    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        println!("Uniswap token logs: {log:?}");
    }

    Ok(())
}
