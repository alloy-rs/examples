//! Example of spinning up a local Reth node instance and connecting it with a provider.

use alloy::{
    node_bindings::Reth,
    providers::{Provider, ProviderBuilder},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Reth node.
    // Ensure `reth` is available in $PATH.
    let reth = Reth::new().dev().disable_discovery().instance(1).spawn();
    let provider = ProviderBuilder::new().connect_http(reth.endpoint().parse()?);

    let chain_id = provider.get_chain_id().await?;

    println!("Reth running at: {} with chain id: {chain_id}", reth.endpoint());

    assert_eq!(chain_id, 1337);
    assert_eq!(reth.http_port(), 8545);
    assert_eq!(reth.ws_port(), 8546);
    assert_eq!(reth.auth_port(), Some(8551));
    assert_eq!(reth.p2p_port(), None);

    Ok(())
}
