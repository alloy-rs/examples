//! Example of using `trace_block` to examine transactions of the latest block.
use alloy::{
    providers::{ext::TraceApi, Provider, ProviderBuilder, WsConnect},
    rpc::types::{trace::parity::Action, BlockId, BlockNumberOrTag},
};

use eyre::Result;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    //let ws_url = "wss://eth-mainnet.g.alchemy.com/v2/your-api-key";
    let ws_url = "ws://10.0.0.145:8546";
    let ws = WsConnect::new(ws_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    let subscription = provider.subscribe_blocks().await?;
    let mut stream = subscription.into_stream();

    while let Some(block) = stream.next().await {
        println!("Received block number: {}", block.inner.number);

        let traces = provider
            .trace_block(BlockId::Number(BlockNumberOrTag::Number(block.inner.number)))
            .await?;

        for trace in traces {
            if let Action::Call(tx) = trace.trace.action {
                if tx.input.0.len() < 4 {
                    continue;
                }
                println!("{:?}", tx);
            }
        }
    }
    Ok(())
}
