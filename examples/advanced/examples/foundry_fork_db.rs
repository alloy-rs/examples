//! This example demonstrates how to use `foundry_fork_db` to build a minimal fork with a db that
//! caches responses from the RPC provider.
//!
//! `foundry_fork_db` is designed out-of-the-box to smartly cache and deduplicate requests to the
//! rpc provider, while fetching data that is missing from it's db instance.
//!
//! `foundry_fork_db` serves as the backend for Foundry's forking functionality in Anvil and Forge.

use std::sync::Arc;

use alloy::{
    consensus::BlockHeader,
    eips::BlockId,
    network::{AnyNetwork, AnyRpcBlock, TransactionBuilder},
    node_bindings::Anvil,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};
use alloy_evm::{eth::EthEvmContext, EthEvm, Evm};
use eyre::Result;
use foundry_fork_db::{cache::BlockchainDbMeta, BlockchainDb, SharedBackend};
use revm::{
    context::{BlockEnv, Evm as RevmEvm, TxEnv},
    context_interface::block::BlobExcessGasAndPrice,
    database::WrapDatabaseRef,
    handler::{instructions::EthInstructions, EthPrecompiles},
    inspector::NoOpInspector,
    primitives::hardfork::SpecId,
    DatabaseRef,
};

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().spawn();
    let provider =
        ProviderBuilder::new().network::<AnyNetwork>().connect_http(anvil.endpoint_url());

    let block = provider.get_block(BlockId::latest()).await?.unwrap();

    // The `BlockchainDbMeta` is used a identifier when the db is flushed to the disk.
    // This aids in cases where the disk contains data from multiple forks.
    let meta = BlockchainDbMeta::default().with_block(&block.inner).with_url(&anvil.endpoint());

    let db = BlockchainDb::new(meta, None);

    // Spawn the backend with the db instance.
    // `SharedBackend` is used to send request to the `BackendHandler` which is responsible for
    // filling missing data in the db, and also deduplicate requests that are being sent to the
    // RPC provider.
    //
    // For example, if we send two requests to get_full_block(0) simultaneously, the
    // `BackendHandler` is smart enough to only send one request to the RPC provider, and queue the
    // other request until the response is received. Once the response from RPC provider is
    // received it relays the response to both the requests     // over their respective channels.
    //
    // The `SharedBackend` and `BackendHandler` communicate over an unbounded channel.
    let shared = SharedBackend::spawn_backend(Arc::new(provider.clone()), db, None).await;

    let start_t = std::time::Instant::now();
    let block_rpc = shared.get_full_block(0).unwrap();
    let time_rpc = start_t.elapsed();

    // `SharedBackend` is cloneable and holds the channel to the same `BackendHandler`.
    #[allow(clippy::redundant_clone)]
    let cloned_backend = shared.clone();

    // Block gets cached in the db
    let start_t = std::time::Instant::now();
    let block_cache = cloned_backend.get_full_block(0).unwrap();
    let time_cache = start_t.elapsed();

    assert_eq!(block_rpc, block_cache);

    println!("-------get_full_block--------");
    // The backend handle falls back to the RPC provider if the block is not in the cache.
    println!("1st request     (via rpc): {time_rpc:?}");
    // The block is cached due to the previous request and can be fetched from db.
    println!("2nd request (via fork db): {time_cache:?}\n");

    let alice = anvil.addresses()[0];
    let bob = anvil.addresses()[1];

    let basefee = block.header.base_fee_per_gas.unwrap();

    let tx_req = TransactionRequest::default()
        .with_from(alice)
        .with_to(bob)
        .with_value(U256::from(100))
        .with_max_fee_per_gas(basefee as u128)
        .with_max_priority_fee_per_gas(basefee as u128 + 1)
        .with_gas_limit(21000)
        .with_nonce(0);

    let mut evm = configure_evm(block, shared.clone());

    // Fetches accounts from the RPC
    let start_t = std::time::Instant::now();
    let alice_bal = shared.basic_ref(alice)?.unwrap().balance;
    let bob_bal = shared.basic_ref(bob)?.unwrap().balance;
    let time_rpc = start_t.elapsed();

    let res = evm.transact(configure_tx_env(tx_req)).unwrap();

    let total_spent = U256::from(res.result.gas_used()) * U256::from(basefee) + U256::from(100);

    shared.data().do_commit(res.state);

    // Fetches accounts from the cache
    let start_t = std::time::Instant::now();
    let alice_bal_after = shared.basic_ref(alice)?.unwrap().balance;
    let bob_bal_after = shared.basic_ref(bob)?.unwrap().balance;
    let time_cache = start_t.elapsed();

    println!("-------get_account--------");
    println!("1st request     (via rpc): {time_rpc:?}");
    println!("2nd request (via fork db): {time_cache:?}\n");

    assert_eq!(alice_bal_after, alice_bal - total_spent);
    assert_eq!(bob_bal_after, bob_bal + U256::from(100));

    Ok(())
}

fn configure_evm(
    block: AnyRpcBlock,
    shared: SharedBackend,
) -> EthEvm<WrapDatabaseRef<SharedBackend>, NoOpInspector> {
    let block_env = BlockEnv {
        number: U256::from(block.header.number()),
        beneficiary: block.header.beneficiary(),
        timestamp: U256::from(block.header.timestamp()),
        gas_limit: block.header.gas_limit(),
        basefee: block.header.base_fee_per_gas().unwrap_or(0),
        prevrandao: block.header.mix_hash(),
        difficulty: block.header.difficulty(),
        blob_excess_gas_and_price: Some(BlobExcessGasAndPrice::new_with_spec(
            block.header.excess_blob_gas().unwrap_or_default(),
            SpecId::PRAGUE,
        )),
    };

    let context = EthEvmContext::new(WrapDatabaseRef(shared), SpecId::PRAGUE).with_block(block_env);

    let evm = RevmEvm::new(context, EthInstructions::default(), EthPrecompiles::default())
        .with_inspector(NoOpInspector);

    EthEvm::new(evm, false)
}

fn configure_tx_env(tx_req: TransactionRequest) -> TxEnv {
    TxEnv {
        caller: tx_req.from.unwrap(),
        kind: tx_req.to.unwrap(),
        value: tx_req.value.unwrap(),
        gas_price: tx_req.max_fee_per_gas.unwrap(),
        gas_limit: tx_req.gas.unwrap_or_default(),
        ..Default::default()
    }
}
