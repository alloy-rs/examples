//! Demonstrates how to leverage `ProviderCall` to wrap the `Provider` trait over reth-db.
use std::{marker::PhantomData, path::PathBuf, str::FromStr, sync::Arc};

use alloy::{
    primitives::U64,
    providers::{Provider, ProviderBuilder, ProviderCall, ProviderLayer, RootProvider},
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
        println!("Getting best block number from db");

        let provider = self.provider().unwrap();

        let best = provider.best_block_number().map_err(TransportErrorKind::custom);

        ProviderCall::<T, NoParams, U64, u64>::ready(best)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let db_path = PathBuf::from_str("DB_PATH")?;
    let provider = ProviderBuilder::new()
        .layer(RethDBLayer::new(db_path))
        .on_http("http://127.0.0.1:8545".parse().unwrap());

    let rpc_provider = ProviderBuilder::new().on_http("http://127.0.0.1:8545".parse().unwrap());

    let start_t = std::time::Instant::now();
    let latest_block = provider.get_block_number().await?;
    println!("Latest block from DB {latest_block} | Time Taken: {:?}", start_t.elapsed());

    let start_t = std::time::Instant::now();
    let latest_block = rpc_provider.get_block_number().await?;
    println!("Latest block from RPC {latest_block} | Time Taken: {:?}", start_t.elapsed());

    Ok(())
}
