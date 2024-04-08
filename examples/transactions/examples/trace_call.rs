//! Example of how to trace a transaction using `trace_call`.

use alloy::{
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{
        eth::{BlockId, BlockNumberOrTag, TransactionRequest},
        trace::parity::TraceType,
    },
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a provider.
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_reqwest_http(rpc_url)?;

    // Create two users, Alice and Bob.
    let alice = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let bob = address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");

    // Create a transaction to send 100 wei from Alice to Bob.
    let tx = TransactionRequest {
        from: Some(alice),
        to: Some(bob),
        value: Some(U256::from(100)),
        ..Default::default()
    };

    // Trace the transaction on top of the latest block.
    let trace_type = [TraceType::Trace];
    let result = provider
        .trace_call(&tx, &trace_type, Some(BlockId::Number(BlockNumberOrTag::Latest)))
        .await?;

    println!("{:?}", result.trace);

    Ok(())
}
