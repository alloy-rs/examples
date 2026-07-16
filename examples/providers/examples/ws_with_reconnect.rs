//! Example of configuring WebSocket connection and reconnection retries.

use alloy::providers::{ConnectionConfig, Provider, ProviderBuilder};
use example_support::ws_url;
use eyre::Result;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = ws_url()?;
    let config =
        ConnectionConfig::new().with_max_retries(10).with_retry_interval(Duration::from_secs(1));

    // The same retry policy is used for the initial connection and later WebSocket reconnects.
    let provider = ProviderBuilder::new().connect_with_config(&endpoint, config).await?;

    println!("Connected to chain {}", provider.get_chain_id().await?);
    Ok(())
}
