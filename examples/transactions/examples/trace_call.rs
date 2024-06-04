//! Example of how to trace a transaction using `trace_call`.

use alloy::{
    network::TransactionBuilder,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{eth::TransactionRequest, trace::parity::TraceType},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a provider.
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Create two users, Alice and Bob.
    let alice = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let bob = address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");

    // Build a transaction to send 100 wei from Alice to Bob.
    let tx =
        TransactionRequest::default().with_from(alice).with_to(bob).with_value(U256::from(100));

    // Trace the transaction on top of the latest block.
    let trace_type = [TraceType::Trace];
    let result = provider.trace_call(&tx, &trace_type).await?;

    println!("{:?}", result.trace);

    Ok(())
}
