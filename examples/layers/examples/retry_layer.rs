//! This example demonstrates how to use the [`RetryBackoffLayer`] in the provider.

use alloy::{
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
    rpc::client::RpcClient,
    transports::layers::RetryBackoffLayer,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let anvil = Anvil::new().spawn();

    // The maximum number of retries for rate limit errors
    let max_retry = 10;

    //  The initial backoff in milliseconds
    let backoff = 1000;

    //  The number of compute units per second for this provider
    let cups = 100;

    // Instantiate the RetryBackoffLayer with the configuration
    let retry_layer = RetryBackoffLayer::new(max_retry, backoff, cups);

    // Add the layer to the transport client.
    // The layer will retry all requests that return a rate limit error (eg. 429) until max_retries
    // have been reached.
    let client = RpcClient::builder().layer(retry_layer).http(anvil.endpoint_url());

    let provider = ProviderBuilder::new().connect_client(client);

    let latest_block = provider.get_block_number().await?;

    assert_eq!(latest_block, 0);

    Ok(())
}
