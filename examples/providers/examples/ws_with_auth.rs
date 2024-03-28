//! Example of using the WS provider with auth to subscribe to new blocks.

use alloy::{network::Ethereum, transports::Authorization};
// Temp Fix
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_client::{RpcClient, WsConnect};
//
use eyre::Result;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up the WS transport which is consumed by the RPC client.
    let rpc_url = "wss://your-ws-endpoint.com/";

    // Create authorization methods.
    let auth = Authorization::basic("username", "password");
    let auth_bearer = Authorization::bearer("bearer-token");

    // Create the WS connection object with authentication.
    let ws_transport_basic = WsConnect::with_auth(rpc_url, Some(auth));
    let ws_transport_bearer = WsConnect::with_auth(rpc_url, Some(auth_bearer));

    // Connect to the WS client.
    let rpc_client_basic = RpcClient::connect_pubsub(ws_transport_basic).await?;
    let rpc_client_bearer = RpcClient::connect_pubsub(ws_transport_bearer).await?;

    // Create the provider.
    let provider_basic = RootProvider::<Ethereum, _>::new(rpc_client_basic);
    let provider_bearer = RootProvider::<Ethereum, _>::new(rpc_client_bearer);

    // Subscribe to new blocks.
    let sub_basic = provider_basic.subscribe_blocks();
    let sub_bearer = provider_bearer.subscribe_blocks();

    // Wait and take the next 4 blocks.
    let mut stream_basic = sub_basic.await?.into_stream().take(4);
    let mut stream_bearer = sub_bearer.await?.into_stream().take(4);

    println!("Awaiting blocks...");

    // Spawning the block processing for basic auth as a new task.
    let basic_handle = tokio::spawn(async move {
        while let Some(block) = stream_basic.next().await {
            println!("From basic {:?}", block.header.number);
        }
    });

    // Similarly for bearer auth.
    let bearer_handle = tokio::spawn(async move {
        while let Some(block) = stream_bearer.next().await {
            println!("From bearer {:?}", block.header.number);
        }
    });

    // Wait for both tasks to complete.
    let _ = tokio::try_join!(basic_handle, bearer_handle)?;

    Ok(())
}
