//! Demonstrates how to leverage `ProviderCall` to wrap the `Provider` trait over reth-db.
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

        let alice = address!("14dC79964da2C08b23698B3D3cc7Ca32193d9955");

        let start_t = std::time::Instant::now();
        let nonce = provider.get_transaction_count(alice).await.unwrap();
        println!("Nonce from DB={nonce} | Time Taken: {:?}", start_t.elapsed());

        let start_t = std::time::Instant::now();
        let nonce = rpc_provider.get_transaction_count(alice).await.unwrap();
        println!("Nonce from RPC={nonce} | Time Taken: {:?}", start_t.elapsed());

        let nonce_at_block = provider
            .get_transaction_count(alice)
            .block_id(BlockId::Number(BlockNumberOrTag::Number(1)))
            .await
            .unwrap();
        println!("Nonce from DB at block 1={nonce_at_block}");
    })
    .await;

    Ok(())
}

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
    provider_factory: WrapProviderFactory,
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

        Self {
            inner,
            db_path,
            provider_factory: WrapProviderFactory::new(Arc::new(provider_factory)),
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

impl<P, T> Provider<T> for RethDBProvider<P, T>
where
    P: Provider<T>,
    T: Transport + Clone,
{
    fn root(&self) -> &RootProvider<T> {
        self.inner.root()
    }

    fn get_block_number(&self) -> ProviderCall<T, NoParams, U64, u64> {
        let provider = self.factory().provider().map_err(TransportErrorKind::custom).unwrap();

        let best = provider.best_block_number().map_err(TransportErrorKind::custom);

        drop(provider);

        ProviderCall::<T, NoParams, U64, u64>::ready(best)
    }

    fn get_transaction_count(&self, address: Address) -> RpcWithBlock<T, Address, U64, u64> {
        let this = self.factory().clone();
        RpcWithBlock::new_provider(move |block_id| {
            let provider = this.provider_at(block_id).map_err(TransportErrorKind::custom).unwrap();

            let maybe_acc =
                provider.basic_account(address).map_err(TransportErrorKind::custom).unwrap();

            let nonce = maybe_acc.map(|acc| acc.nonce).unwrap_or_default();

            drop(provider);

            ProviderCall::<T, ParamsWithBlock<Address>, U64, u64>::ready(Ok(nonce))
        })
    }
}

#[derive(Clone, Debug)]
struct WrapProviderFactory {
    inner: Arc<ProviderFactory<NodeTypesWithDBAdapter<EthereumNode, Arc<DatabaseEnv>>>>,
}

impl WrapProviderFactory {
    const fn new(
        inner: Arc<ProviderFactory<NodeTypesWithDBAdapter<EthereumNode, Arc<DatabaseEnv>>>>,
    ) -> Self {
        Self { inner }
    }

    /// Get the DB provider.
    fn provider(&self) -> Result<DatabaseProvider<Tx<RO>, ChainSpec>, ProviderError> {
        self.inner.provider()
    }

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
