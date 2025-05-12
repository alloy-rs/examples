//! Example of spinning up a local Anvil node using the [`ProviderBuilder`].

use alloy::providers::{ext::AnvilApi, ProviderBuilder};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let provider = ProviderBuilder::new()
        .connect_anvil_with_config(|anvil| anvil.block_time(1).chain_id(1337));

    // Get node info using the Anvil API.
    let info = provider.anvil_node_info().await?;

    println!("Node info: {info:#?}");

    assert_eq!(info.environment.chain_id, 1337);
    assert_eq!(info.fork_config.fork_url, None);

    Ok(())
}
