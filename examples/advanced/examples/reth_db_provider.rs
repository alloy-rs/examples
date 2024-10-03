//! In this example, we demonstrate how we wrap the `Provider` trait over reth-db by
//! leveraging `ProviderCall`.
//!
//! `ProviderCall` enables the alloy-provider to fetch results of a rpc request from arbitrary
//! sources. These arbitray sources could be a RPC call over the network, a local database, or even
//! a synchronous function call.
//!
//! `ProviderCall` is the final future in the flow of an rpc request and is used by the
//! `RpcWithBlock` and `EthCall` types under the hood to give flexibility to the user to use
//! their own implementation of the `Provider` trait and fetch results from any source.
//!
//! Learn more about `ProviderCall` [here](https://github.com/alloy-rs/alloy/pull/788).
use alloy::{
    eips::BlockId,
    node_bindings::{utils::run_with_tempdir, Reth},
    primitives::address,
    providers::{Provider, ProviderBuilder},
};
use eyre::Result;
use reth_alloy::RethDbLayer;

#[tokio::main]
async fn main() -> Result<()> {
    run_with_tempdir("provider-call-reth-db", |data_dir| async move {
        // Initializing reth with a tmp data directory.
        // We use a tmp directory for the purposes of this example.
        // This would actually use an existing reth datadir specified by `--datadir` when starting
        // your reth node.
        let reth = Reth::new()
            .dev()
            .disable_discovery()
            .block_time("1s")
            .data_dir(data_dir.clone())
            .spawn();

        let db_path = data_dir.join("db");

        // Initialize the provider with the reth-db layer. The reth-db layer intercepts the rpc
        // requests and returns the results from the reth-db database.
        // Any RPC method that is not implemented in the RethDbProvider gracefully falls back to the
        // RPC provider specified in the `on_http` method.
        let provider =
            ProviderBuilder::new().layer(RethDbLayer::new(db_path)).on_http(reth.endpoint_url());

        // Initialize the RPC provider to compare the time taken to fetch the results.
        let rpc_provider = ProviderBuilder::new().on_http(reth.endpoint_url());

        println!("--------get_block_number---------");

        let start_t = std::time::Instant::now();
        let latest_block_db = provider.get_block_number().await.unwrap();
        println!("via reth-db: {:?}", start_t.elapsed());

        let start_t = std::time::Instant::now();
        let latest_block_rpc = rpc_provider.get_block_number().await.unwrap();
        println!("via rpc:     {:?}\n", start_t.elapsed());

        assert_eq!(latest_block_db, latest_block_rpc);

        println!("------get_transaction_count------");

        let alice = address!("14dC79964da2C08b23698B3D3cc7Ca32193d9955");

        let start_t = std::time::Instant::now();
        let nonce_db =
            provider.get_transaction_count(alice).block_id(BlockId::latest()).await.unwrap();
        println!("via reth-db: {:?}", start_t.elapsed());

        let start_t = std::time::Instant::now();
        let nonce_rpc =
            rpc_provider.get_transaction_count(alice).block_id(BlockId::latest()).await.unwrap();
        println!("via rpc:     {:?}\n", start_t.elapsed());

        assert_eq!(nonce_db, nonce_rpc);
    })
    .await;

    Ok(())
}
