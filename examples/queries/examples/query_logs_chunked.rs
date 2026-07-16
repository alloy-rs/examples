//! Example of querying event logs over a large block range using `chunked()`.
//!
//! Some JSON-RPC providers limit the block range for `eth_getLogs` requests.
//! This first tries the full range in a single request.
//! If that fails, it splits the range into chunks and queries them concurrently.
//! If a chunk still fails, it falls back to querying each block individually.

use alloy::{
    contract::Event,
    primitives::address,
    providers::{Provider, ProviderBuilder},
    rpc::types::Filter,
    sol,
    sol_types::SolEvent,
};
use example_support::rpc_url;
use eyre::Result;

sol! {
    #[allow(missing_docs)]
    event Transfer(address indexed from, address indexed to, uint256 value);
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create a provider.
    let rpc_url = rpc_url()?.parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let latest_block = provider.get_block_number().await?;

    let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
    let filter = Filter::new()
        .address(uniswap_token_address)
        .event_signature(Transfer::SIGNATURE_HASH)
        .from_block(latest_block - 50_000)
        .to_block(latest_block);
    // You could also use the event name instead of the event signature like so:
    // .event("Transfer(address,address,uint256)")

    let event: Event<_, Transfer, _> = Event::new(&provider, filter);
    // Query the last 50,000 blocks for UNI Transfer events, splitting the range into
    // 250-block chunks queried with 4 concurrent requests.
    let transfers = event.chunked().chunk_size(250).concurrent(4).query().await?;

    for (Transfer { from, to, value }, _) in &transfers {
        println!("Transfer from {from} to {to} of value {value}");
    }

    println!("Fetched {} UNI Transfer logs across the last 50,000 blocks", transfers.len());

    Ok(())
}
