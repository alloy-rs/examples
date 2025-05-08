//! Example of spinning up a local Anvil instance and connecting it with a provider.

use alloy::{
    node_bindings::Anvil,
    providers::{ext::AnvilApi, ProviderBuilder},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().block_time(1).chain_id(1337).try_spawn()?;
    let provider = ProviderBuilder::new().connect_http(anvil.endpoint_url());

    // Get node info using the Anvil API.
    let info = provider.anvil_node_info().await?;

    println!("Node info: {info:#?}");

    assert_eq!(info.environment.chain_id, 1337);
    assert_eq!(info.fork_config.fork_url, None);

    Ok(())
}
