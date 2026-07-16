//! Example of resuming a canonical block stream from a known block number.

use alloy::{
    node_bindings::Anvil,
    providers::{CanonicalEvent, Provider, ProviderBuilder},
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

    // Persist this value in a real indexer, then reuse it after a restart. The stream first
    // backfills from this block and subsequently polls for new canonical blocks.
    let resume_from = 0;
    let mut blocks = provider
        .watch_canonical_blocks_from(resume_from)
        .poll_interval(Duration::from_millis(100))
        .full()
        .max_reorg_depth(64)
        .into_stream();

    let event = timeout(Duration::from_secs(5), blocks.next())
        .await
        .wrap_err("timed out waiting for the canonical block stream")?
        .ok_or_else(|| eyre!("canonical block stream ended unexpectedly"))??;

    match event {
        CanonicalEvent::Added(block) => {
            println!("Added canonical block {}", block.header.number);
            assert_eq!(block.header.number, resume_from);
        }
        CanonicalEvent::Removed(block) => {
            println!("Removed reorged block {}", block.header.number);
        }
    }

    Ok(())
}
