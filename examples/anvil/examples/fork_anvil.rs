//! Example of spinning up a forked Anvil node.

use alloy_node_bindings::Anvil;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH
    let anvil = Anvil::new().fork("https://eth.llamarpc.com").spawn();

    println!("Anvil running at `{}`", anvil.endpoint());

    Ok(())
}
