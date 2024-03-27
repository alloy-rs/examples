//! Example of multiplexing watching event logs.

use alloy::{network::Ethereum, node_bindings::Anvil, primitives::I256, sol, sol_types::SolEvent};
use alloy_provider::RootProvider;
use alloy_rpc_client::RpcClient;
use eyre::Result;
use futures_util::StreamExt;
use std::str::FromStr;

sol!(
    #[derive(Debug)]
    #[sol(rpc, bytecode = "0x608060405234801561001057600080fd5b50610485806100206000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c80634350913814610051578063a5f3c23b1461006d578063adefc37b14610089578063bbe93d91146100a5575b600080fd5b61006b60048036038101906100669190610248565b6100c1565b005b61008760048036038101906100829190610248565b610114565b005b6100a3600480360381019061009e9190610248565b610167565b005b6100bf60048036038101906100ba9190610248565b6101ba565b005b80826100cd91906102e6565b3373ffffffffffffffffffffffffffffffffffffffff167f1c1e8bbe327890ea8d3f5b22370a56c3fcef7ff82f306161f64647fe5d28588160405160405180910390a35050565b80826101209190610350565b3373ffffffffffffffffffffffffffffffffffffffff167f6da406ea462447ed7804b4a4dc69c67b53d3d45a50381ae3e9cf878c9d7c23df60405160405180910390a35050565b80826101739190610394565b3373ffffffffffffffffffffffffffffffffffffffff167f32e913bf2ad35da1e845597618bb9f3f80642a68dd39f30a093a7838aa61fb2760405160405180910390a35050565b80826101c691906103d7565b3373ffffffffffffffffffffffffffffffffffffffff167fd7a123d4c8e44db3186e04b9c96c102287276929c930f2e8abcaa555ef5dcacc60405160405180910390a35050565b600080fd5b6000819050919050565b61022581610212565b811461023057600080fd5b50565b6000813590506102428161021c565b92915050565b6000806040838503121561025f5761025e61020d565b5b600061026d85828601610233565b925050602061027e85828601610233565b9150509250929050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60006102f182610212565b91506102fc83610212565b92508261030c5761030b610288565b5b600160000383147f800000000000000000000000000000000000000000000000000000000000000083141615610345576103446102b7565b5b828205905092915050565b600061035b82610212565b915061036683610212565b92508282019050828112156000831216838212600084121516171561038e5761038d6102b7565b5b92915050565b600061039f82610212565b91506103aa83610212565b92508282039050818112600084121682821360008512151617156103d1576103d06102b7565b5b92915050565b60006103e282610212565b91506103ed83610212565b92508282026103fb81610212565b91507f80000000000000000000000000000000000000000000000000000000000000008414600084121615610433576104326102b7565b5b8282058414831517610448576104476102b7565b5b509291505056fea2646970667358221220386c6c77ebc5f1bae50f37d123c5a510f2f678b30900c2d5ebf09f68c9353f4b64736f6c63430008180033")]
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
    // Ensure `anvil` is available in $PATH
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    let ws = alloy_rpc_client::WsConnect::new(anvil.ws_endpoint());
    let provider = RootProvider::<Ethereum, _>::new(RpcClient::connect_pubsub(ws).await?);

    let deployed_contract = EventMultiplexer::deploy(provider.clone()).await?;

    println!("Deployed contract at: {:?}", deployed_contract.address());

    let add_filter = deployed_contract.Add_filter().watch().await?;
    let sub_filter = deployed_contract.Sub_filter().watch().await?;
    let mul_filter = deployed_contract.Mul_filter().watch().await?;
    let div_filter = deployed_contract.Div_filter().watch().await?;

    let a = I256::from_str("1").unwrap();
    let b = I256::from_str("1").unwrap();
    // Build calls
    let add_call = deployed_contract.add(a, b);
    let sub_call = deployed_contract.sub(a, b);
    let mul_call = deployed_contract.mul(a, b);
    let div_call = deployed_contract.div(a, b);

    // Send calls
    let _ = add_call.send().await?;
    let _ = sub_call.send().await?;
    let _ = mul_call.send().await?;
    let _ = div_call.send().await?;

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
    // The for loop helps capture all the logs
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

        let topic = &log.topics[0];

        if topic == add_log {
            println!("Received Add: {:?}", log);
        } else if topic == sub_log {
            println!("Received Sub: {:?}", log);
        } else if topic == mul_log {
            println!("Received Mul: {:?}", log);
        } else if topic == div_log {
            println!("Received Div: {:?}", log);
        }
    }

    Ok(())
}
