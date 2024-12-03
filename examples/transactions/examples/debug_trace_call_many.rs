//! Example of how to trace a transaction using `debug_trace_call_many`.

use alloy::{
    network::TransactionBuilder,
    primitives::{address, U256},
    providers::{ext::DebugApi, ProviderBuilder},
    rpc::types::{
        trace::geth::GethDebugTracingCallOptions, Bundle, StateContext, TransactionRequest,
    },
};
use eyre::Result;

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

    // Create the bundle of transactions.
    let bundles = vec![Bundle { transactions: vec![tx1, tx2], block_override: None }];

    // Define the State context and trace option
    let state_context = StateContext::default();
    let trace_options = GethDebugTracingCallOptions::default();

    //Call `debug_trace_call_many` on the provider.
    let result = provider.debug_trace_call_many(bundles, state_context, trace_options).await;

    match result {
        Ok(traces) => {
            println!("Traces:\n{:?}", traces);
        }
        Err(err) => {
            println!("Error tracing transactions: {:?}", err);
        }
    }

    Ok(())
}
