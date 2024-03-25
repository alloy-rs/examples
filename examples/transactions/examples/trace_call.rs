//! Example of how to trace a transaction using `trace_call`.

use alloy::{
    network::Ethereum,
    node_bindings::Anvil,
    primitives::U256,
    providers::{HttpProvider, Provider},
    rpc::types::{
        eth::{BlockId, BlockNumberOrTag, TransactionRequest},
        trace::parity::TraceType,
    },
};
use eyre::Result;
use reqwest::Url;

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().fork("https://eth.merkle.io").spawn();
    let provider =
        HttpProvider::<Ethereum>::new_http("https://eth.merkle.io".parse::<Url>().unwrap());

    let from = anvil.addresses()[0];
    let to = anvil.addresses()[1];

    let tx_req = TransactionRequest {
        from: Some(from),
        to: Some(to),
        value: Some(U256::from(100)),
        ..Default::default()
    };
    let trace_type = [TraceType::Trace];
    let res = provider
        .trace_call(&tx_req, &trace_type, Some(BlockId::Number(BlockNumberOrTag::Latest)))
        .await?;

    println!("{:?}", res.trace);

    Ok(())
}
