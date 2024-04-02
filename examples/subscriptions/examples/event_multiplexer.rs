//! Example of multiplexing the watching of event logs.

use alloy::{
    node_bindings::Anvil,
    primitives::I256,
    providers::ProviderBuilder,
    rpc::client::{RpcClient, WsConnect},
    sol,
    sol_types::SolEvent,
};
use eyre::Result;
use futures_util::StreamExt;
use std::str::FromStr;

// Codegen from embedded Solidity code and precompiled bytecode.
// solc v0.8.24; solc a.sol --via-ir --optimize --bin
sol!(
    #[allow(missing_docs)]
    #[sol(rpc, bytecode = "0x6080806040523461001657610213908161001b8239f35b5f80fdfe6080604052600480361015610012575f80fd5b5f3560e01c80634350913814610165578063a5f3c23b14610116578063adefc37b146100c75763bbe93d9114610046575f80fd5b346100c357610054366101c7565b8181029291905f8212600160ff1b8214166100b057818405149015171561009d5750337fd7a123d4c8e44db3186e04b9c96c102287276929c930f2e8abcaa555ef5dcacc5f80a3005b601190634e487b7160e01b5f525260245ffd5b601183634e487b7160e01b5f525260245ffd5b5f80fd5b50346100c3576100d6366101c7565b91905f838203931281841281169184139015161761009d5750337f32e913bf2ad35da1e845597618bb9f3f80642a68dd39f30a093a7838aa61fb275f80a3005b50346100c357610125366101c7565b91905f838201938412911290801582169115161761009d5750337f6da406ea462447ed7804b4a4dc69c67b53d3d45a50381ae3e9cf878c9d7c23df5f80a3005b50346100c357610174366101c7565b9182156101b457600160ff1b82145f1984141661009d575005337f1c1e8bbe327890ea8d3f5b22370a56c3fcef7ff82f306161f64647fe5d2858815f80a3005b601290634e487b7160e01b5f525260245ffd5b60409060031901126100c357600435906024359056fea26469706673582212208fc27b878cb877b8c4e5dee739e0a35a64f82759549ed8f6d4ddab5ded9717bb64736f6c63430008180033")]
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

    // Create a provider.
    let ws = WsConnect::new(anvil.ws_endpoint());
    let provider = ProviderBuilder::new().on_client(RpcClient::connect_pubsub(ws).await?);

    // Deploy the `EventExample` contract.
    let contract = EventMultiplexer::deploy(provider).await?;

    println!("Deployed contract at: {:?}", contract.address());

    // Create filters for each event.
    let add_filter = contract.Add_filter().watch().await?;
    let sub_filter = contract.Sub_filter().watch().await?;
    let mul_filter = contract.Mul_filter().watch().await?;
    let div_filter = contract.Div_filter().watch().await?;

    let a = I256::from_str("1").unwrap();
    let b = I256::from_str("1").unwrap();

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
                log.unwrap().1
            }
            Some(log) = sub_stream.next() => {
                log.unwrap().1
            }
            Some(log) = mul_stream.next() => {
                log.unwrap().1
            }
            Some(log) = div_stream.next() => {
                log.unwrap().1
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
