//! Demonstrates how to leverage `ProviderCall` to wrap the `Provider` trait over reth-db.
use std::{env::temp_dir, marker::PhantomData, path::PathBuf, str::FromStr, sync::Arc};

use alloy::{
    node_bindings::{utils::run_with_tempdir, Reth},
    primitives::{Address, U64},
    providers::{
        Provider, ProviderBuilder, ProviderCall, ProviderLayer, RootProvider, RpcWithBlock,
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
    providers::StaticFileProvider, BlockNumReader, DatabaseProvider, ProviderError, ProviderFactory,
};

/// A `ProviderLayer` that wraps the `Provider` trait over reth-db.
struct RethDBLayer {
    db_path: PathBuf,
}

/// Implement the `ProviderLayer` trait for `RethDBLayer`.
impl RethDBLayer {
    const fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }
}

impl<P, T> ProviderLayer<P, T> for RethDBLayer
where
    P: Provider<T>,
    T: Transport + Clone,
{
    type Provider = RethDBProvider<P, T>;

    fn layer(&self, inner: P) -> Self::Provider {
        RethDBProvider::new(inner, self.db_path.clone())
    }
}

/// A provider that wraps the `Provider` trait over reth-db.
#[derive(Clone, Debug)]
pub struct RethDBProvider<P, T> {
    inner: P,
    db_path: PathBuf,
    provider_factory: ProviderFactory<NodeTypesWithDBAdapter<EthereumNode, Arc<DatabaseEnv>>>,
    _pd: PhantomData<T>,
}

impl<P, T> RethDBProvider<P, T> {
    /// Create a new `RethDBProvider` instance.
    pub fn new(inner: P, db_path: PathBuf) -> Self {
        let db = open_db_read_only(&db_path, Default::default()).unwrap();
        let chain_spec = ChainSpecBuilder::mainnet().build();
        let static_file_provider =
            StaticFileProvider::read_only(db_path.join("static_files"), false).unwrap();

        let provider_factory =
            ProviderFactory::new(db.into(), chain_spec.into(), static_file_provider);

        Self { inner, db_path, provider_factory, _pd: PhantomData }
    }

    /// Get the DB provider.
    fn provider(&self) -> Result<DatabaseProvider<Tx<RO>, ChainSpec>, ProviderError> {
        self.provider_factory.provider()
    }

    /// Get the DB Path
    pub fn db_path(&self) -> PathBuf {
        self.db_path.clone()
    }
}

impl<P, T> Provider<T> for RethDBProvider<P, T>
where
    P: Provider<T>,
    T: Transport + Clone,
{
    fn root(&self) -> &RootProvider<T> {
        self.inner.root()
    }

    fn get_block_number(&self) -> ProviderCall<T, NoParams, U64, u64> {
        let provider = self.provider().map_err(TransportErrorKind::custom).unwrap();

        let best = provider.best_block_number().map_err(TransportErrorKind::custom);

        drop(provider);

        ProviderCall::<T, NoParams, U64, u64>::ready(best)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    run_with_tempdir("provider-call-reth-db", |data_dir| async move {
        let reth = Reth::new()
            .dev()
            .disable_discovery()
            .block_time("1s")
            .data_dir(data_dir.clone())
            .spawn();

        let db_path = data_dir.join("db");

        let provider =
            ProviderBuilder::new().layer(RethDBLayer::new(db_path)).on_http(reth.endpoint_url());

        let rpc_provider = ProviderBuilder::new().on_http(reth.endpoint_url());

        let start_t = std::time::Instant::now();
        let latest_block = provider.get_block_number().await.unwrap();
        println!("Latest block from DB={latest_block} | Time Taken: {:?}", start_t.elapsed());

        let start_t = std::time::Instant::now();
        let latest_block = rpc_provider.get_block_number().await.unwrap();
        println!("Latest block from RPC={latest_block} | Time Taken: {:?}", start_t.elapsed());
    })
    .await;

    Ok(())
}
