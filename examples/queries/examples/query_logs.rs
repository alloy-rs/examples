//! Example of how to query logs from the Ethereum network.

use alloy_network::Ethereum;
use alloy_provider::{HttpProvider, Provider};
use alloy_rpc_client::RpcClient;
use alloy_rpc_types::Filter;
use alloy_transport_http::Http;
use eyre::Result;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = init();

    // Get logs from the latest block
    let latest_block = provider.get_block_number().await?;
    let filter = Filter::new().from_block(latest_block);
    let logs = provider.get_logs(&filter).await?;

    for log in logs {
        println!("{:?}", log);
    }
    Ok(())
}

fn init() -> HttpProvider<Ethereum> {
    let http = Http::<Client>::new("https://eth.llamarpc.com".parse().unwrap());
    HttpProvider::new(RpcClient::new(http, true))
}
