//! Example of subscribing and listening for all contract events by `WebSocket` subscription.

use alloy::{
    primitives::address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    sol,
    sol_types::SolEvent,
};
use eyre::Result;
use futures_util::stream::StreamExt;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IWETH9,
    "examples/abi/IWETH9.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Create the provider.
    let rpc_url = "wss://eth-mainnet.g.alchemy.com/v2/your-api-key";
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    // Create a filter to watch for all WETH9 events.
    let weth9_token_address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    let filter = Filter::new()
        // By NOT specifying an `event` or `event_signature` we listen to ALL events of the
        // contract.
        .address(weth9_token_address)
        .from_block(BlockNumberOrTag::Latest);

    // Subscribe to logs.
    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        // Match on topic 0, the hash of the signature of the event.
        match log.topic0() {
            // Match the `Approval(address,address,uint256)` event.
            Some(&IWETH9::Approval::SIGNATURE_HASH) => {
                let IWETH9::Approval { src, guy, wad } = log.log_decode()?.inner.data;
                println!("Approval from {src} to {guy} of value {wad}");
            }
            // Match the `Transfer(address,address,uint256)` event.
            Some(&IWETH9::Transfer::SIGNATURE_HASH) => {
                let IWETH9::Transfer { src, dst, wad } = log.log_decode()?.inner.data;
                println!("Transfer from {src} to {dst} of value {wad}");
            }
            // WETH9's `Deposit(address,uint256)` and `Withdrawal(address,uint256)` events are not
            // handled here.
            _ => (),
        }
    }

    Ok(())
}
