//! Example of how to trace a transaction using `debug_trace_call_many`.

use alloy::{
    network::TransactionBuilder,
    node_bindings::Reth,
    primitives::{address, U256},
    providers::{ext::DebugApi, ProviderBuilder},
    rpc::types::{
        trace::geth::GethDebugTracingCallOptions, Bundle, StateContext, TransactionRequest,
    },
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

    // Create the bundle of transactions.
    let bundles = vec![Bundle { transactions: vec![tx1, tx2], block_override: None }];

    // Define the state context and trace option.
    let state_context = StateContext::default();
    let trace_options = GethDebugTracingCallOptions::default();

    // Call `debug_trace_call_many` on the provider.
    let result = provider.debug_trace_call_many(bundles, state_context, trace_options).await;

    // Print the trace results.
    match result {
        Ok(traces) => {
            println!("Traces:\n{traces:?}");
        }
        Err(err) => {
            println!("Error tracing transactions: {err:?}");
        }
    }

    Ok(())
}
