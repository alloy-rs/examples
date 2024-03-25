//! Example of using the HTTP provider to get the latest block number.

use alloy::{
    network::Ethereum,
    providers::{HttpProvider, Provider},
    rpc::client::RpcClient,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup the HTTP transport which is consumed by the RPC client
    let rpc_url = "https://eth.llamarpc.com".parse().unwrap();

    // Create the RPC client
    let rpc_client = RpcClient::new_http(rpc_url);

    // Provider can then be instantiated using the RPC client, HttpProvider is an alias
    // RootProvider. RootProvider requires two generics N: Network and T: Transport
    let provider = HttpProvider::<Ethereum>::new(rpc_client);

    // Get latest block number
    let latest_block = provider.get_block_number().await?;

    println!("Latest block number: {}", latest_block);

    Ok(())
}
