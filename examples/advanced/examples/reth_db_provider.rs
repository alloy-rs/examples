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

use std::{path::PathBuf, sync::Arc};

use alloy::{
    eips::{BlockId, BlockNumberOrTag},
    node_bindings::{utils::run_with_tempdir, Reth},
    primitives::{address, Address, U64},
    providers::{
        Provider, ProviderBuilder, ProviderCall, ProviderLayer, RootProvider, RpcWithBlock,
    },
    rpc::client::NoParams,
    transports::TransportErrorKind,
};
use eyre::Result;

use reth_chainspec::ChainSpecBuilder;
use reth_db::{open_db_read_only, DatabaseEnv};
use reth_node_ethereum::EthereumNode;
use reth_node_types::NodeTypesWithDBAdapter;
use reth_provider::{
    providers::StaticFileProvider, BlockNumReader, DatabaseProviderFactory, ProviderError,
    ProviderFactory, StateProvider, TryIntoHistoricalStateProvider,
};
mod reth_db_layer;
use reth_db_layer::RethDbLayer;

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
        // RPC provider specified in the `connect_http` method.
        let provider = ProviderBuilder::new()
            .layer(RethDbLayer::new(db_path))
            .connect_http(reth.endpoint_url());

        // Initialize the RPC provider to compare the time taken to fetch the results.
        let rpc_provider = ProviderBuilder::new().connect_http(reth.endpoint_url());

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

/// Implement the `ProviderLayer` trait for the `RethDBLayer` struct.
impl<P> ProviderLayer<P> for RethDbLayer
where
    P: Provider,
{
    type Provider = RethDbProvider<P>;

    fn layer(&self, inner: P) -> Self::Provider {
        RethDbProvider::new(inner, self.db_path().clone())
    }
}

/// A provider that overrides the vanilla `Provider` trait to get results from the reth-db.
///
/// It holds the `reth_provider::ProviderFactory` that enables read-only access to the database
/// tables and static files.
#[derive(Clone, Debug)]
pub struct RethDbProvider<P> {
    inner: P,
    db_path: PathBuf,
    provider_factory: DbAccessor,
}

impl<P> RethDbProvider<P> {
    /// Create a new `RethDbProvider` instance.
    pub fn new(inner: P, db_path: PathBuf) -> Self {
        let db = open_db_read_only(&db_path, Default::default()).unwrap();
        let chain_spec = ChainSpecBuilder::mainnet().build();
        let static_file_provider =
            StaticFileProvider::read_only(db_path.join("static_files"), false).unwrap();

        let provider_factory =
            ProviderFactory::new(db.into(), chain_spec.into(), static_file_provider);

        let db_accessor: DbAccessor<
            ProviderFactory<NodeTypesWithDBAdapter<EthereumNode, Arc<DatabaseEnv>>>,
        > = DbAccessor::new(provider_factory);
        Self { inner, db_path, provider_factory: db_accessor }
    }

    const fn factory(&self) -> &DbAccessor {
        &self.provider_factory
    }

    /// Get the DB Path
    pub fn db_path(&self) -> PathBuf {
        self.db_path.clone()
    }
}

/// Implement the `Provider` trait for the `RethDbProvider` struct.
///
/// This is where we override specific RPC methods to fetch from the reth-db.
impl<P> Provider for RethDbProvider<P>
where
    P: Provider,
{
    fn root(&self) -> &RootProvider {
        self.inner.root()
    }

    /// Override the `get_block_number` method to fetch the latest block number from the reth-db.
    fn get_block_number(&self) -> ProviderCall<NoParams, U64, u64> {
        let provider = self.factory().provider().map_err(TransportErrorKind::custom).unwrap();

        let best = provider.best_block_number().map_err(TransportErrorKind::custom);

        ProviderCall::ready(best)
    }

    /// Override the `get_transaction_count` method to fetch the transaction count of an address.
    ///
    /// `RpcWithBlock` uses `ProviderCall` under the hood.
    fn get_transaction_count(&self, address: Address) -> RpcWithBlock<Address, U64, u64> {
        let this = self.factory().clone();
        RpcWithBlock::new_provider(move |block_id| {
            let provider = this.provider_at(block_id).map_err(TransportErrorKind::custom).unwrap();

            let maybe_acc =
                provider.basic_account(&address).map_err(TransportErrorKind::custom).unwrap();

            let nonce = maybe_acc.map(|acc| acc.nonce).unwrap_or_default();

            ProviderCall::ready(Ok(nonce))
        })
    }
}

/// A helper type to get the appropriate DB provider.
#[derive(Debug, Clone)]
struct DbAccessor<DB = ProviderFactory<NodeTypesWithDBAdapter<EthereumNode, Arc<DatabaseEnv>>>>
where
    DB: DatabaseProviderFactory<Provider: TryIntoHistoricalStateProvider + BlockNumReader>,
{
    inner: DB,
}

impl<DB> DbAccessor<DB>
where
    DB: DatabaseProviderFactory<Provider: TryIntoHistoricalStateProvider + BlockNumReader>,
{
    const fn new(inner: DB) -> Self {
        Self { inner }
    }

    fn provider(&self) -> Result<DB::Provider, ProviderError> {
        self.inner.database_provider_ro()
    }

    fn provider_at(&self, block_id: BlockId) -> Result<Box<dyn StateProvider>, ProviderError> {
        let provider = self.inner.database_provider_ro()?;

        let block_number = match block_id {
            BlockId::Hash(hash) => {
                if let Some(num) = provider.block_number(hash.into())? {
                    num
                } else {
                    return Err(ProviderError::BlockHashNotFound(hash.into()));
                }
            }
            BlockId::Number(BlockNumberOrTag::Number(num)) => num,
            _ => provider.best_block_number()?,
        };

        provider.try_into_history_at_block(block_number)
    }
}
