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
use std::{marker::PhantomData, path::PathBuf, sync::Arc};

use alloy::{
    eips::{BlockId, BlockNumberOrTag},
    node_bindings::{utils::run_with_tempdir, Reth},
    primitives::{address, Address, U64},
    providers::{
        ParamsWithBlock, Provider, ProviderBuilder, ProviderCall, ProviderLayer, RootProvider,
        RpcWithBlock,
    },
    rpc::client::NoParams,
    transports::{Transport, TransportErrorKind},
};
use eyre::Result;

use reth_chainspec::{ChainSpec, ChainSpecBuilder};
use reth_db::{
    mdbx::{tx::Tx, RO},
    open_db_read_only, DatabaseEnv,
};
use reth_node_ethereum::EthereumNode;
use reth_node_types::NodeTypesWithDBAdapter;
use reth_provider::{
    providers::StaticFileProvider, BlockNumReader, DatabaseProvider, ProviderError,
    ProviderFactory, StateProvider,
};

#[tokio::main]
async fn main() -> Result<()> {
    run_with_tempdir("provider-call-reth-db", |data_dir| async move {
        // Initialize reth node with a specific data directory
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
            ProviderBuilder::new().layer(RethDBLayer::new(db_path)).on_http(reth.endpoint_url());

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

mod reth_db_layer;
use reth_db_layer::RethDBLayer;

/// Implement the `ProviderLayer` trait for the `RethDBLayer` struct.
impl<P, T> ProviderLayer<P, T> for RethDBLayer
where
    P: Provider<T>,
    T: Transport + Clone,
{
    type Provider = RethDbProvider<P, T>;

    fn layer(&self, inner: P) -> Self::Provider {
        RethDbProvider::new(inner, self.db_path().clone())
    }
}

/// A provider that overrides the vanilla `Provider` trait to get results from the reth-db.
///
/// It holds the `reth_provider::ProviderFactory` that enables read-only access to the database
/// tables and static files.
#[derive(Clone, Debug)]
pub struct RethDbProvider<P, T> {
    inner: P,
    db_path: PathBuf,
    provider_factory: WrapProviderFactory,
    _pd: PhantomData<T>,
}

impl<P, T> RethDbProvider<P, T> {
    /// Create a new `RethDbProvider` instance.
    pub fn new(inner: P, db_path: PathBuf) -> Self {
        let db = open_db_read_only(&db_path, Default::default()).unwrap();
        let chain_spec = ChainSpecBuilder::mainnet().build();
        let static_file_provider =
            StaticFileProvider::read_only(db_path.join("static_files"), false).unwrap();

        let provider_factory =
            ProviderFactory::new(db.into(), chain_spec.into(), static_file_provider);

        Self {
            inner,
            db_path,
            provider_factory: WrapProviderFactory::new(provider_factory),
            _pd: PhantomData,
        }
    }

    const fn factory(&self) -> &WrapProviderFactory {
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
impl<P, T> Provider<T> for RethDbProvider<P, T>
where
    P: Provider<T>,
    T: Transport + Clone,
{
    fn root(&self) -> &RootProvider<T> {
        self.inner.root()
    }

    /// Override the `get_block_number` method to fetch the latest block number from the reth-db.
    fn get_block_number(&self) -> ProviderCall<T, NoParams, U64, u64> {
        let provider = self.factory().provider().map_err(TransportErrorKind::custom).unwrap();

        let best = provider.best_block_number().map_err(TransportErrorKind::custom);

        ProviderCall::ready(best)
    }

    /// Override the `get_transaction_count` method to fetch the transaction count of an address.
    ///
    /// `RpcWithBlock` uses `ProviderCall` under the hood.
    fn get_transaction_count(&self, address: Address) -> RpcWithBlock<T, Address, U64, u64> {
        let this = self.factory().clone();
        RpcWithBlock::new_provider(move |block_id| {
            let provider = this.provider_at(block_id).map_err(TransportErrorKind::custom).unwrap();

            let maybe_acc =
                provider.basic_account(address).map_err(TransportErrorKind::custom).unwrap();

            let nonce = maybe_acc.map(|acc| acc.nonce).unwrap_or_default();

            ProviderCall::ready(Ok(nonce))
        })
    }
}

/// A helper type to get the appropriate DB provider from `reth_provider::ProviderFactory`.
#[derive(Clone, Debug)]
struct WrapProviderFactory {
    inner: Arc<ProviderFactory<NodeTypesWithDBAdapter<EthereumNode, Arc<DatabaseEnv>>>>,
}

impl WrapProviderFactory {
    const fn new(
        inner: ProviderFactory<NodeTypesWithDBAdapter<EthereumNode, Arc<DatabaseEnv>>>,
    ) -> Self {
        Self { inner: Arc::new(inner) }
    }

    /// Get a read-only `DatabaseProvider`
    fn provider(&self) -> Result<DatabaseProvider<Tx<RO>, ChainSpec>, ProviderError> {
        self.inner.provider()
    }

    /// Get a read-only `DatabaseProvider` at a specific block
    fn provider_at(
        &self,
        block_id: BlockId,
    ) -> Result<Box<(dyn StateProvider + 'static)>, ProviderError> {
        match block_id {
            BlockId::Hash(hash) => self.inner.history_by_block_hash(hash.block_hash),
            BlockId::Number(BlockNumberOrTag::Number(num)) => {
                self.inner.history_by_block_number(num)
            }
            _ => self.inner.latest(),
        }
    }
}
