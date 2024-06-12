//! Example of how to trace a transaction using `trace_call`.

use alloy::{
    network::TransactionBuilder,
    primitives::{address, U256},
    providers::{ext::TraceApi, ProviderBuilder},
    rpc::types::{trace::parity::TraceType, TransactionRequest},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a provider.
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Build a transaction to send 100 wei from Alice to Vitalik.
    let alice = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx =
        TransactionRequest::default().with_from(alice).with_to(vitalik).with_value(U256::from(100));

    // Trace the transaction on top of the latest block.
    let trace_type = [TraceType::Trace];
    let result = provider.trace_call(&tx, &trace_type).await?;

    println!("{:?}", result.trace);

    Ok(())
}
