use alloy::{
    network::Ethereum,
    primitives::{address, fixed_bytes},
    providers::{Provider, RootProvider},
    rpc::types::eth::Filter,
};
use eyre::Result;
#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://eth.merkle.io".parse().unwrap();
    let provider = RootProvider::<Ethereum, _>::new_http(url);

    // Get logs from the latest block
    let latest_block = provider.get_block_number().await?;

    // Get all logs from the latest block
    let filter = Filter::new().from_block(latest_block);

    let logs = provider.get_logs(&filter).await?;

    for log in logs {
        println!("{:?}", log);
    }

    let tranfer_event_signature =
        fixed_bytes!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");

    // Get all logs from the latest block that match the transfer event signature/topic
    let filter = Filter::new().event_signature(tranfer_event_signature).from_block(latest_block);
    // You could also use the event name instead of the event signature like so:
    // .event("Transfer(address,address,uint256)")

    let logs = provider.get_logs(&filter).await?;

    for log in logs {
        println!("Transfer event: {:?}", log);
    }

    let uni_address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");

    // Get all from the latest block emitted by the UNI token address
    let filter = Filter::new().address(uni_address).from_block(latest_block);

    let logs = provider.get_logs(&filter).await?;

    for log in logs {
        println!("UNI token logs: {:?}", log);
    }

    Ok(())
}
