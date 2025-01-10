//! Example of using `trace_call` on pending transactions.
use alloy::{
    providers::{ext::TraceApi, Provider, ProviderBuilder, WsConnect},
    rpc::types::trace::parity::TraceType,
};

use eyre::Result;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "wss://eth-mainnet.g.alchemy.com/v2/your-api-key";

    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    let sub = provider.subscribe_full_pending_transactions().await?;

    let mut stream = sub.into_stream().take(1);

    println!("Awaiting pending transactions...");

    let handle = tokio::spawn(async move {
        while let Some(tx) = stream.next().await {
            let trace_type = [TraceType::Trace];
            let result = provider.trace_call(&tx.into(), &trace_type).await;
            println!("{:?}", result.unwrap().trace);
        }
    });

    handle.await?;
    Ok(())
}
