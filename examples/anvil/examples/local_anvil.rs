//! Example of spinning up a local Anvil node.

use alloy_node_bindings::Anvil;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH
    let anvil = Anvil::new().block_time(1).chain_id(1337).try_spawn()?;

    println!("Anvil running at `{}`", anvil.endpoint());

    Ok(())
}
