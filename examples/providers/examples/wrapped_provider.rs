//! Example demonstrating how to wrap a provider in a custom type.

use alloy::{
    providers::{Provider, ProviderBuilder},
    transports::TransportResult,
};
use eyre::Result;

/// Simple free function to get the latest block number.
async fn get_block_number<P: Provider>(provider: &P) -> TransportResult<u64> {
    provider.get_block_number().await
}

/// Wrapped provider type.
struct MyProvider<P: Provider> {
    provider: P,
}

impl<P: Provider> MyProvider<P> {
    /// Create a new instance of `MyProvider`.
    fn new(provider: P) -> Self {
        Self { provider }
    }

    async fn get_block_number(&self) -> TransportResult<u64> {
        get_block_number(&self.provider).await
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let provider = ProviderBuilder::new().on_anvil();

    let my_provider = MyProvider::new(provider);

    let latest_block = my_provider.get_block_number().await?;

    println!("Latest block number: {latest_block}");

    Ok(())
}
