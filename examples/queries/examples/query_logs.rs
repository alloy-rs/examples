//! Example of querying logs from the Ethereum network.

use alloy::{
    primitives::{address, b256},
    providers::{Provider, ProviderBuilder},
    rpc::types::Filter,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a provider.
    let rpc_url = "https://reth-ethereum.ithaca.xyz/rpc".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // Get logs from the latest block
    let latest_block = provider.get_block_number().await?;

    // Create a filter to get all logs from the latest block.
    let filter = Filter::new().from_block(latest_block);

    // Get all logs from the latest block that match the filter.
    let logs = provider.get_logs(&filter).await?;

    for log in logs {
        println!("{log:?}");
    }

    // Get all logs from the latest block that match the transfer event signature/topic.
    let transfer_event_signature =
        b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");
    let filter = Filter::new().event_signature(transfer_event_signature).from_block(latest_block);
    // You could also use the event name instead of the event signature like so:
    // .event("Transfer(address,address,uint256)")

    // Get all logs from the latest block that match the filter.
    let logs = provider.get_logs(&filter).await?;

    for log in logs {
        println!("Transfer event: {log:?}");
    }

    // Get all logs from the latest block emitted by the UNI token address.
    let uniswap_token_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
    let filter = Filter::new().address(uniswap_token_address).from_block(latest_block);

    // Get all logs from the latest block that match the filter.
    let logs = provider.get_logs(&filter).await?;

    for log in logs {
        println!("Uniswap token logs: {log:?}");
    }

    Ok(())
}
