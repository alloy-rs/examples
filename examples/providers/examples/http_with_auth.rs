//! Example of using the reqwest HTTP client with an `Authorization` header to get the latest block
//! number.

use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::client::RpcClient,
    transports::http::{
        reqwest::{
            header::{HeaderMap, HeaderValue, AUTHORIZATION},
            Client,
        },
        Http,
    },
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Set the Authorization header.
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_static("deadbeef"));

    // Create the reqwest::Client with the AUTHORIZATION header.
    let client_with_auth = Client::builder().default_headers(headers).build()?;

    // Create the HTTP transport.
    let rpc_url = "https://reth-ethereum.ithaca.xyz/rpc".parse()?;
    let http = Http::with_client(client_with_auth, rpc_url);
    let rpc_client = RpcClient::new(http, false);

    // Create a provider with the HTTP transport.
    let provider = ProviderBuilder::new().connect_client(rpc_client);

    // Get latest block number.
    let latest_block = provider.get_block_number().await?;

    println!("Latest block number: {latest_block}");

    Ok(())
}
