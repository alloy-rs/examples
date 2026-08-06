//! Example of resuming a canonical log stream from a known block number.

use alloy::{
    node_bindings::Anvil,
    providers::{CanonicalEvent, Provider, ProviderBuilder},
    rpc::types::Filter,
};
use eyre::{eyre, Result, WrapErr};
use futures_util::StreamExt;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<()> {
    // Run against a local node so this example is deterministic and needs no endpoint.
    // Ensure `anvil` is available in PATH.
    let anvil = Anvil::new().try_spawn()?;
    let provider = ProviderBuilder::new().connect(&anvil.endpoint()).await?;

    let filter = Filter::new().event("Transfer(address,address,uint256)");

    // Persist this value in a real indexer, then reuse it after a restart. The stream first
    // backfills from this block and subsequently polls for new canonical blocks.
    let resume_from = 0;
    let mut logs = provider
        .watch_canonical_logs_from(resume_from, &filter)
        .poll_interval(Duration::from_millis(100))
        .max_reorg_depth(64)
        .into_stream();

    let event = timeout(Duration::from_secs(5), logs.next())
        .await
        .wrap_err("timed out waiting for the canonical log stream")?
        .ok_or_else(|| eyre!("canonical log stream ended unexpectedly"))??;

    match event {
        CanonicalEvent::Added(block_logs) => {
            println!(
                "Added canonical block {} with {} matching logs",
                block_logs.block.header.number,
                block_logs.logs.len()
            );
            assert_eq!(block_logs.block.header.number, resume_from);
        }
        CanonicalEvent::Removed(block_logs) => {
            println!(
                "Removed reorged block {} with {} matching logs",
                block_logs.block.header.number,
                block_logs.logs.len()
            );
        }
    }

    Ok(())
}
