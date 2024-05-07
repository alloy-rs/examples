//! Example of using the WS provider with auth to subscribe to new blocks.

use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::client::WsConnect,
    transports::Authorization,
};
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
    let ws_basic = WsConnect::with_auth(rpc_url, Some(auth));
    let ws_bearer = WsConnect::with_auth(rpc_url, Some(auth_bearer));

    // Create the provider.
    let provider_basic = ProviderBuilder::new().on_ws(ws_basic).await?;
    let provider_bearer = ProviderBuilder::new().on_ws(ws_bearer).await?;

    // Subscribe to new blocks.
    let sub_basic = provider_basic.subscribe_blocks();
    let sub_bearer = provider_bearer.subscribe_blocks();

    // Wait and take the next 4 blocks.
    let mut stream_basic = sub_basic.await?.into_stream().take(4);
    let mut stream_bearer = sub_bearer.await?.into_stream().take(4);

    println!("Awaiting blocks...");

    // Take the basic stream and print the block number upon receiving a new block.
    let basic_handle = tokio::spawn(async move {
        while let Some(block) = stream_basic.next().await {
            println!("Latest block number (basic): {}", block.header.number.unwrap());
        }
    });

    // Take the bearer stream and print the block number upon receiving a new block.
    let bearer_handle = tokio::spawn(async move {
        while let Some(block) = stream_bearer.next().await {
            println!("Latest block number (bearer): {}", block.header.number.unwrap());
        }
    });

    // Wait for both tasks to complete.
    let _ = tokio::try_join!(basic_handle, bearer_handle)?;

    Ok(())
}
