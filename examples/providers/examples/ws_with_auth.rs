//! Example of using the WS provider with auth to subscribe to new blocks.

use alloy::{
    providers::{Provider, ProviderBuilder, WsConnect},
    transports::Authorization,
};
use eyre::Result;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    // Create authorization methods.
    let auth = Authorization::basic("username", "password");
    let auth_bearer = Authorization::bearer("bearer-token");

    // Create the WS connection object with authentication.
    let rpc_url = "wss://your-ws-endpoint.com/";
    let ws_basic = WsConnect::new(rpc_url).with_auth(auth);
    let ws_bearer = WsConnect::new(rpc_url).with_auth(auth_bearer);

    // Create the provider.
    let provider_basic = ProviderBuilder::new().connect_ws(ws_basic).await?;
    let provider_bearer = ProviderBuilder::new().connect_ws(ws_bearer).await?;

    // Subscribe to new block headers.
    let sub_basic = provider_basic.subscribe_blocks();
    let sub_bearer = provider_bearer.subscribe_blocks();

    // Wait and take the next 4 block headers
    let mut stream_basic = sub_basic.await?.into_stream().take(4);
    let mut stream_bearer = sub_bearer.await?.into_stream().take(4);

    println!("Awaiting block headers...");

    // Take the basic stream and print the block number upon receiving a new block header.
    let basic_handle = tokio::spawn(async move {
        while let Some(header) = stream_basic.next().await {
            println!("Latest block number (basic): {}", header.number);
        }
    });

    // Take the bearer stream and print the block number upon receiving a new block header.
    let bearer_handle = tokio::spawn(async move {
        while let Some(header) = stream_bearer.next().await {
            println!("Latest block number (bearer): {}", header.number);
        }
    });

    // Wait for both tasks to complete.
    let _ = tokio::try_join!(basic_handle, bearer_handle)?;

    Ok(())
}
