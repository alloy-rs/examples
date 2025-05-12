//! Test the fallback layer provider.

use std::{num::NonZeroUsize, time::Duration};

use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::client::RpcClient,
    transports::{
        http::{reqwest::Url, Http},
        layers::FallbackLayer,
    },
};
use eyre::Result;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();

    // Configure the fallback layer
    let fallback_layer =
        FallbackLayer::default().with_active_transport_count(NonZeroUsize::new(3).unwrap());

    // Define your list of transports to use
    let transports = vec![
        Http::new(Url::parse("https://reth-ethereum.ithaca.xyz/rpc")?),
        Http::new(Url::parse("https://eth.llamarpc.com")?),
        Http::new(Url::parse("https://ethereum-rpc.publicnode.com")?),
    ];

    // Apply the FallbackLayer to the transports
    let transport = ServiceBuilder::new().layer(fallback_layer).service(transports);
    let client = RpcClient::builder().transport(transport, false);
    let provider = ProviderBuilder::new().connect_client(client);

    // Get the latest block number using the provider with ranked transports.
    // This will also print the rankings of the transports to the console.
    let max = 10;
    let mut count = 0;
    loop {
        let latest_block = provider.get_block_number().await?;
        println!("Latest block number: {latest_block}");
        tokio::time::sleep(Duration::from_secs(1)).await;

        count += 1;
        if count >= max {
            break;
        }
    }

    Ok(())
}
