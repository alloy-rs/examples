//! Example of how to trace a transaction using `trace_call_many`.

use alloy::{
    network::TransactionBuilder,
    node_bindings::Reth,
    primitives::{address, U256},
    providers::{ext::TraceApi, ProviderBuilder},
    rpc::types::{trace::parity::TraceType, TransactionRequest},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Reth node.
    // Ensure `reth` is available in $PATH.
    let reth = Reth::new().dev().disable_discovery().instance(1).spawn();
    let provider = ProviderBuilder::new().connect_http(reth.endpoint().parse()?);

    // Get users, these have allocated balances in the dev genesis block.
    let alice = address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");
    let bob = address!("3C44CdDdB6a900fa2b585dd299e03d12FA4293BC");
    let charlie = address!("90F79bf6EB2c4f870365E785982E1f101E93b906");
    let dan = address!("15d34AAf54267DB7D7c367839AAf71A00a2C6A65");

    // Define transactions.
    let tx1 =
        TransactionRequest::default().with_from(alice).with_to(bob).with_value(U256::from(150));
    let tx2 =
        TransactionRequest::default().with_from(charlie).with_to(dan).with_value(U256::from(250));

    // Define the trace type for the trace call list.
    let trace_type: &[TraceType] = &[TraceType::Trace];

    // Trace the transaction on top of the latest block.
    let trace_call_list = &[(tx1, trace_type), (tx2, trace_type)];

    let result = provider.trace_call_many(trace_call_list).await?;

    // Print the trace results.
    for (index, trace_result) in result.iter().enumerate() {
        println!("Trace result for transaction {index}: {trace_result:?}");
    }
    Ok(())
}
