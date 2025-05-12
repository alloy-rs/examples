//! Example of watching and polling for contract events by `WebSocket` subscription.

use alloy::{
    node_bindings::Anvil,
    providers::{ProviderBuilder, WsConnect},
    signers::local::PrivateKeySigner,
    sol,
};
use eyre::Result;
use futures_util::StreamExt;

// Codegen from embedded Solidity code and precompiled bytecode.
// solc v0.8.26; solc Counter.sol --via-ir --optimize --bin
sol!(
    #[allow(missing_docs)]
    #[sol(rpc, bytecode = "6080806040523460195760008055610155908161001f8239f35b600080fdfe6080604052600436101561001257600080fd5b60003560e01c80632baeceb7146100d057806361bc221a146100b25763d09de08a1461003d57600080fd5b346100ad5760003660031901126100ad57600054600181019060006001831291129080158216911516176100975780600055337ff6d1d8d205b41f9fb9549900a8dba5d669d68117a3a2b88c1ebc61163e8117ba600080a3005b634e487b7160e01b600052601160045260246000fd5b600080fd5b346100ad5760003660031901126100ad576020600054604051908152f35b346100ad5760003660031901126100ad5760005460001981019081136001166100975780600055337fdc69c403b972fc566a14058b3b18e1513da476de6ac475716e489fae0cbe4a26600080a300fea26469706673582212200d333e08e1230b0b9919825888e587a45c68e2aa2f7f58752712491e2201da9c64736f6c634300081a0033")]
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
    let pk: PrivateKeySigner = anvil.keys()[0].clone().into();

    // Create a WebSocket provider.
    let ws = WsConnect::new(anvil.ws_endpoint());
    let provider = ProviderBuilder::new().wallet(pk).connect_ws(ws).await?;

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
