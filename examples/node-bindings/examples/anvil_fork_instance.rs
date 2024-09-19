//! Example of spinning up a forked Anvil instance and connecting it with a provider.

use alloy::{
    node_bindings::Anvil,
    providers::{ext::AnvilApi, ProviderBuilder},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // The RPC URL of the node to fork.
    let fork_url = "https://eth.merkle.io";

    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().fork(fork_url).try_spawn()?;
    let provider =
        ProviderBuilder::new().with_recommended_fillers().on_http(anvil.endpoint().parse()?);

    // Get node info using the Anvil API.
    let info = provider.anvil_node_info().await?;

    println!("Node info: {:#?}", info);

    assert_eq!(info.environment.chain_id, 1);
    assert_eq!(info.fork_config.fork_url, Some(fork_url.to_string()));

    Ok(())
}
