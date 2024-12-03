//! Example of how to trace a transaction using `trace_call_many`.

use alloy::{
    network::TransactionBuilder,
    primitives::{address, U256},
    providers::{ext::TraceApi, ProviderBuilder},
    rpc::types::{trace::parity::TraceType, TransactionRequest},
};

use eyre::{Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // create a provider
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let alice = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    let bob = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

    let jon = address!("f29Fd6e51aad88F6F4ce6aB8827279cffFb92377");
    let vitalik = address!("f9dA6BF26964aF9D7eEd9e03E53415D37aA96033");

    // Define transactions
    let tx1 =
        TransactionRequest::default().with_from(alice).with_to(bob).with_value(U256::from(150));
    let tx2 =
        TransactionRequest::default().with_from(jon).with_to(vitalik).with_value(U256::from(250));

    // Define the trace for the trace_list
    let trace_type: &[TraceType] = &[TraceType::Trace];

    // Trace the transaction on top of the latest block.
    let trace_call_list = &[(tx1, trace_type), (tx2, trace_type)];

    let result = provider.trace_call_many(trace_call_list).await?;

    // Print the trace results.
    for (index, trace_result) in result.iter().enumerate() {
        println!("Trace result for transaction {}: {:?}", index, trace_result);
    }
    Ok(())
}
