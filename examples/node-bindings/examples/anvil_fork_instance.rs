//! Example of spinning up a forked Anvil instance and connecting it with a provider.

use alloy::{
    node_bindings::Anvil,
    providers::{ext::AnvilApi, ProviderBuilder},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let rpc_url = "https://reth-ethereum.ithaca.xyz/rpc";
    let anvil = Anvil::new().fork(rpc_url).try_spawn()?;
    let provider = ProviderBuilder::new().connect_http(anvil.endpoint_url());

    // Get node info using the Anvil API.
    let info = provider.anvil_node_info().await?;

    println!("Node info: {info:#?}");

    assert_eq!(info.environment.chain_id, 1);
    assert_eq!(info.fork_config.fork_url, Some(rpc_url.to_string()));

    Ok(())
}
