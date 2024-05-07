//! Example of watching and polling for contract events by WebSocket subscription.

use alloy::{node_bindings::Anvil, providers::ProviderBuilder, rpc::client::WsConnect, sol};
use eyre::Result;
use futures_util::StreamExt;

// Codegen from embedded Solidity code and precompiled bytecode.
// solc v0.8.24; solc a.sol --via-ir --optimize --bin
sol!(
    #[allow(missing_docs)]
    #[sol(rpc, bytecode = "0x60808060405234610019575f8055610143908161001e8239f35b5f80fdfe60806040526004361015610011575f80fd5b5f3560e01c80632baeceb7146100c357806361bc221a146100a75763d09de08a1461003a575f80fd5b346100a3575f3660031901126100a3575f5460018101905f60018312911290801582169115161761008f57805f55337ff6d1d8d205b41f9fb9549900a8dba5d669d68117a3a2b88c1ebc61163e8117ba5f80a3005b634e487b7160e01b5f52601160045260245ffd5b5f80fd5b346100a3575f3660031901126100a35760205f54604051908152f35b346100a3575f3660031901126100a3575f545f19810190811360011661008f57805f55337fdc69c403b972fc566a14058b3b18e1513da476de6ac475716e489fae0cbe4a265f80a300fea2646970667358221220c045c027059726f9175a4abd427eb3f7a3fe8e27108bc19e4ae46055e7c1842c64736f6c63430008180033")]
    contract Counter {
        int256 public counter = 0;

        event Increment(address indexed by, int256 indexed value);
        event Decrement(address indexed by, int256 indexed value);

        function increment() public {
            counter += 1;
            emit Increment(msg.sender, counter);
        }

        function decrement() public {
            counter -= 1;
            emit Decrement(msg.sender, counter);
        }
    }
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    // Create a WebSocket provider.
    let ws = WsConnect::new(anvil.ws_endpoint());
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Deploy the `Counter` contract.
    let contract = Counter::deploy(provider.clone()).await?;

    println!("Deployed contract at: {}", contract.address());

    // Create filters for each event.
    let increment_filter = contract.Increment_filter().watch().await?;
    let decrement_filter = contract.Decrement_filter().watch().await?;

    // Build a call to increment the counter.
    let increment_call = contract.increment();

    // Build a call to decrement the counter.
    let decrement_call = contract.decrement();

    // Send the transaction call twice for each event.
    for _ in 0..2 {
        let _ = increment_call.send().await?;
        let _ = decrement_call.send().await?;
    }

    // Poll for logs.
    increment_filter
        .into_stream()
        .take(2)
        .for_each(|log| async {
            match log {
                Ok((_event, log)) => {
                    println!("Received Increment: {log:?}");
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
        })
        .await;

    decrement_filter
        .into_stream()
        .take(2)
        .for_each(|log| async {
            match log {
                Ok((_event, log)) => {
                    println!("Received Decrement: {log:?}");
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
        })
        .await;

    Ok(())
}
