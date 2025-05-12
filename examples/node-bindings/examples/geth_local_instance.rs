//! Example of spinning up a local Geth node instance and connecting it with a provider.

use alloy::{
    node_bindings::Geth,
    providers::{Provider, ProviderBuilder},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Geth node.
    // Ensure `geth` is available in $PATH.
    let geth = Geth::new().chain_id(1337).port(8545_u16).authrpc_port(8551).spawn();
    let provider = ProviderBuilder::new().connect_http(geth.endpoint().parse()?);

    let chain_id = provider.get_chain_id().await?;

    println!("Geth running at: {} with chain id: {chain_id}", geth.endpoint());

    assert_eq!(chain_id, 1337);
    assert_eq!(geth.port(), 8545);
    assert_eq!(geth.auth_port(), Some(8551));
    assert_eq!(geth.p2p_port(), None);

    Ok(())
}
