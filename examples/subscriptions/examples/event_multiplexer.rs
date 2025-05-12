//! Example of multiplexing the watching of event logs.

use std::str::FromStr;

use alloy::{
    node_bindings::Anvil,
    primitives::I256,
    providers::{ProviderBuilder, WsConnect},
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolEvent,
};
use eyre::Result;
use futures_util::StreamExt;

// Codegen from embedded Solidity code and precompiled bytecode.
// solc v0.8.26; solc EventMultiplexer.sol --via-ir --optimize --bin
sol!(
    #[allow(missing_docs)]
    #[sol(rpc, bytecode = "60808060405234601557610207908161001b8239f35b600080fdfe6080604052600436101561001257600080fd5b60003560e01c80634350913814610156578063a5f3c23b14610108578063adefc37b146100ba5763bbe93d911461004857600080fd5b346100b557610056366101bb565b818102919060008212600160ff1b82141661009f57818305149015171561009f57337fd7a123d4c8e44db3186e04b9c96c102287276929c930f2e8abcaa555ef5dcacc600080a3005b634e487b7160e01b600052601160045260246000fd5b600080fd5b346100b5576100c8366101bb565b906000828203921281831281169183139015161761009f57337f32e913bf2ad35da1e845597618bb9f3f80642a68dd39f30a093a7838aa61fb27600080a3005b346100b557610116366101bb565b906000828201928312911290801582169115161761009f57337f6da406ea462447ed7804b4a4dc69c67b53d3d45a50381ae3e9cf878c9d7c23df600080a3005b346100b557610164366101bb565b9081156101a557600160ff1b811460001983141661009f5705337f1c1e8bbe327890ea8d3f5b22370a56c3fcef7ff82f306161f64647fe5d285881600080a3005b634e487b7160e01b600052601260045260246000fd5b60409060031901126100b557600435906024359056fea2646970667358221220d876fbacf1e90fc174532f3525420c446351b467f788f9d7a726a7d55045909664736f6c634300081a0033")]
    contract EventMultiplexer {
        event Add(address indexed sender, int256 indexed value);
        event Sub(address indexed sender, int256 indexed value);
        event Mul(address indexed sender, int256 indexed value);
        event Div(address indexed sender, int256 indexed value);

        function add(int256 a, int256 b) public {
            emit Add(msg.sender, a + b);
        }

        function sub(int256 a, int256 b) public {
            emit Sub(msg.sender, a - b);
        }

        function mul(int256 a, int256 b) public {
            emit Mul(msg.sender, a * b);
        }

        function div(int256 a, int256 b) public {
            emit Div(msg.sender, a / b);
        }
    }
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    let pk: PrivateKeySigner = anvil.keys()[0].clone().into();

    // Create a provider.
    let ws = WsConnect::new(anvil.ws_endpoint());
    let provider = ProviderBuilder::new().wallet(pk).connect_ws(ws).await?;

    // Deploy the `EventExample` contract.
    let contract = EventMultiplexer::deploy(provider).await?;

    println!("Deployed contract at: {}", contract.address());

    // Create filters for each event.
    let add_filter = contract.Add_filter().watch().await?;
    let sub_filter = contract.Sub_filter().watch().await?;
    let mul_filter = contract.Mul_filter().watch().await?;
    let div_filter = contract.Div_filter().watch().await?;

    let a = I256::from_str("1")?;
    let b = I256::from_str("1")?;

    // Build the transaction calls.
    let add_call = contract.add(a, b);
    let sub_call = contract.sub(a, b);
    let mul_call = contract.mul(a, b);
    let div_call = contract.div(a, b);

    // Send the transaction calls.
    let _ = add_call.send().await?;
    let _ = sub_call.send().await?;
    let _ = mul_call.send().await?;
    let _ = div_call.send().await?;

    // Convert the filters into streams.
    let mut add_stream = add_filter.into_stream();
    let mut sub_stream = sub_filter.into_stream();
    let mut mul_stream = mul_filter.into_stream();
    let mut div_stream = div_filter.into_stream();

    let add_log = &EventMultiplexer::Add::SIGNATURE_HASH;
    let sub_log = &EventMultiplexer::Sub::SIGNATURE_HASH;
    let mul_log = &EventMultiplexer::Mul::SIGNATURE_HASH;
    let div_log = &EventMultiplexer::Div::SIGNATURE_HASH;

    // Use tokio::select! to multiplex the streams and capture the log
    // tokio::select! will return the first event that arrives from any of the streams
    // The for loop helps capture all the logs.
    for _ in 0..4 {
        let log = tokio::select! {
            Some(log) = add_stream.next() => {
                log?.1
            }
            Some(log) = sub_stream.next() => {
                log?.1
            }
            Some(log) = mul_stream.next() => {
                log?.1
            }
            Some(log) = div_stream.next() => {
                log?.1
            }
        };

        let topic = &log.topics()[0];

        if topic == add_log {
            println!("Received Add: {log:?}");
        } else if topic == sub_log {
            println!("Received Sub: {log:?}");
        } else if topic == mul_log {
            println!("Received Mul: {log:?}");
        } else if topic == div_log {
            println!("Received Div: {log:?}");
        }
    }

    Ok(())
}
