//! Foundry Fork DB
use std::sync::Arc;

use eyre::Result;

use alloy::{
    eips::BlockId,
    network::AnyNetwork,
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
    rpc::types::BlockTransactionsKind,
};
use foundry_fork_db::{cache::BlockchainDbMeta, BlockchainDb, SharedBackend};

// TODO: Add docs and explanation of the workflow.
// TODO: Depict how the backend handler can smartly manages duplicate requests.

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().spawn();
    let provider = ProviderBuilder::new().network::<AnyNetwork>().on_http(anvil.endpoint_url());

    let block =
        provider.get_block(BlockId::latest(), BlockTransactionsKind::Hashes).await?.unwrap();

    let pin_block = BlockId::number(block.header.number);

    let meta = BlockchainDbMeta::default()
        .with_chain_id(31337)
        .with_block(&block.inner)
        .with_url(&anvil.endpoint());

    let db = BlockchainDb::new(meta, None);

    let shared = SharedBackend::spawn_backend(Arc::new(provider), db, Some(pin_block)).await;

    let start_t = std::time::Instant::now();
    let block_rpc = shared.get_full_block(0).unwrap();
    let time_rpc = start_t.elapsed();

    // `SharedBackend` are clonable and have the same underlying cache.
    let cloned_backend = shared.clone();

    // Block gets cached in the db
    let start_t = std::time::Instant::now();
    let block_cache = cloned_backend.get_full_block(0).unwrap();
    let time_cache = start_t.elapsed();

    assert_eq!(block_rpc, block_cache);

    assert!(time_cache < time_rpc);

    println!("Time taken for 1st request  (via RPC): {:?}", time_rpc);
    println!("Time taken for 2nd requst (via Cache): {:?}", time_cache);

    Ok(())
}
