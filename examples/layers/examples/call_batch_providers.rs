// ---------- External Imports & Macro Definitions ----------
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};

#[cfg(not(target_arch = "wasm32"))]
use tokio::time::sleep;
#[cfg(target_arch = "wasm32")]
use wasmtimer::tokio::sleep;

// For demonstration purposes we assume these types are defined or imported from your code base:
pub mod alloy_primitives {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Address(pub [u8; 20]);
    
    // Stub implementation of Bytes. In a real code base, use a proper Bytes type.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Bytes(pub Vec<u8>);
    
    // Stub U256 (usually a big integer type).
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct U256(pub [u8; 32]);
}

use alloy_primitives::{Address, Bytes, U256};

// A simple helper macro to convert a hex literal into an Address.
// In your real code base, use your preferred implementation.
#[macro_export]
macro_rules! address {
    ($addr:expr) => {{
        let s = $addr.strip_prefix("0x").unwrap_or($addr);
        let bytes = hex::decode(s).expect("Invalid hex in address");
        let mut arr = [0u8; 20];
        arr.copy_from_slice(&bytes[..20]);
        Address(arr)
    }};
}

// ---------- Constants ----------
/// The default Multicall3 address.
/// Adjust the value as needed.
pub const MULTICALL3_ADDRESS: Address = address!("0xcA11bde05977b3631167028862bE2a173976CA11");

// ---------- Solidity Bindings via sol! Macro ----------


sol! {
    /// [`Multicall3`](https://github.com/mds1/multicall) bindings.
    #[sol(bytecode = "608060405234801561001057600080fd5b50610ee0806100206000396000f3fe6080604052600436106100f35760003560e01c80634d2301cc1161008a578063a8b0574e11610059578063a8b0574e1461025a578063bce38bd714610275578063c3077fa914610288578063ee82ac5e1461029b57600080fd5b80634d2301cc146101ec57806372425d9d1461022157806382ad56cb1461023457806386d516e81461024757600080fd5b80633408e470116100c65780633408e47014610191578063399542e9146101a45780633e64a696146101c657806342cbb15c146101d957600080fd5b80630f28c97d146100f8578063174dea711461011a578063252dba421461013a57806327e86d6e1461015b575b600080fd5b34801561010457600080fd5b50425b6040519081526020015b60405180910390f35b61012d610128366004610a85565b6102ba565b6040516101119190610bbe565b61014d610148366004610a85565b6104ef565b604051610111929190610bd8565b34801561016757600080fd5b50437fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0140610107565b34801561019d57600080fd5b5046610107565b6101b76101b2366004610c60565b610690565b60405161011193929190610cba565b3480156101d257600080fd5b5048610107565b3480156101e557600080fd5b5043610107565b3480156101f858600080fd5b50610107610207366004610ce2565b73ffffffffffffffffffffffffffffffffffffffff163190565b34801561022d57600080fd5b5044610107565b61012d610242366004610a85565b6106ab565b34801561025357600080fd5b5045610107565b34801561026657600080fd5b50604051418152602001610111565b61012d610283366004610c60565b61085a565b6101b7610296366004610a85565b610a1a565b3480156102a757600080fd5b506101076102b6366004610d18565b4090565b60606000828067ffffffffffffffff8111156102d8576102d8610d31565b60405190808252806020026020018201604052801561031e57816020015b6040805180820190915260008152606060208201528152602001906001900390816102f65790505b5092503660005b8281101561047757600085828151811061034157610341610d60565b6020026020010151905087878381811061035d5761035d610d60565b905060200281019061036f9190610d8f565b6040810135958601959093506103886020850185610ce2565b73ffffffffffffffffffffffffffffffffffffffff16816103ac6060870187610dcd565b6040516103ba929190610e32565b60006040518083038185875af1925050503d80600081146103f7576040519150601f19603f3d011682016040523d82523d6000602084013e6103fc565b606091505b50602080850191909152901515808452908501351761046d577f08c379a000000000000000000000000000000000000000000000000000000000600052602060045260176024527f4d756c746963616c6c333a2063616c6c206661696c656400000000000000000060445260846000fd5b5050600101610325565b508234146104e6576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601a60248201527f4d756c746963616c6c333a2076616c7565206d69736d6174636800000000000060448201526064015b60405180910390fd5b50505092915050565b436060828067ffffffffffffffff81111561050c5761050c610d31565b60405190808252806020026020018201604052801561053f57816020015b606081526020019060019003908161052a5790505b5091503660005b8281101561068657600087878381811061056257610562610d60565b90506020028101906105749190610e42565b92506105836020840184610ce2565b73ffffffffffffffffffffffffffffffffffffffff166105a66020850185610dcd565b6040516105b4929190610e32565b6000604051808303816000865af19150503d80600081146105f1576040519150601f19603f3d011682016040523d82523d6000602084013e6105f6565b606091505b5086848151811061060957610609610d60565b602090810291909101015290508061067d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f4d756c746963616c6c333a2063616c6c206661696c656400000000000000000060448201526064016104dd565b50600101610546565b5050509250929050565b43804060606106a086868661085a565b905093509350939050565b6060818067ffffffffffffffff8111156106c7576106c7610d31565b60405190808252806020026020018201604052801561070d57816020015b6040805180820190915260008152606060208201528152602001906001900390816106e55790505b5091503660005b828110156104e657600084828151811061073057610730610d60565b6020026020010151905086868381811061074c5761074c610d60565b905060200281019061075e9190610e76565b925061076d6020840184610ce2565b73ffffffffffffffffffffffffffffffffffffffff166107906040850185610dcd565b60405161079e929190610e32565b6000604051808303816000865af19150503d80600081146107db576040519150601f19603f3d011682016040523d82523d6000602084013e6107e0565b606091505b506020808401919091529015158083529084013517610851577f08c379a000000000000000000000000000000000000000000000000000000000600052602060045260176024527f4d756c746963616c6c333a2063616c6c206661696c656400000000000000000060445260646000fd5b50600101610714565b6060818067ffffffffffffffff81111561087657610876610d31565b6040519080825280602002602001820160405280156108bc57816020015b6040805180820190915260008152606060208201528152602001906001900390816108945790505b5091503660005b82811015610a105760008482815181106108df576108df610d60565b602002602001015190508686838181106108fb576108fb610d60565b905060200281019061090d9190610e42565b925061091c6020840184610ce2565b73ffffffffffffffffffffffffffffffffffffffff1661093f6020850185610dcd565b60405161094d929190610e32565b6000604051808303816000865af19150503d806000811461098a576040519150601f19603f3d011682016040523d82523d6000602084013e61098f565b606091505b506020830152151581528715610a07578051610a07576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f4d756c746963616c6c333a2063616c6c206661696c656400000000000000000060448201526064016104dd565b506001016108c3565b5050509392505050565b602081526000610bd16020830184610b32565b9392505050565b600060408284031215610cf457600080fd5b813573ffffffffffffffffffffffffffffffffffffffff81168114610bd157600080fd5b600060408284031215610d2a57600080fd5b5035919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b600082357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81833603018112610dc357600080fd5b9190910192915050565b60008083357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe1843603018112610e0257600080fd5b83018035915067ffffffffffffffff821115610e1d57600080fd5b602001915036819003821315610a7e57600080fd5b8183823760009101908152919050565b600082357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc1833603018112610dc357600080fd5b600082357fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa1833603018112610dc357600080fdfea2646970667358221220bb2b5c71a328032f97c676ae39a1ec2148d3e5d6f73d95e9b17910152d61f16264736f6c634300080c0033")]
    #[derive(Debug, PartialEq, Eq)]
    interface IMulticall3 {
        struct Call {
            address target;
            bytes callData;
        }

        struct Call3 {
            address target;
            bool allowFailure;
            bytes callData;
        }

        struct Call3Value {
            address target;
            bool allowFailure;
            uint256 value;
            bytes callData;
        }

        struct Result {
            bool success;
            bytes returnData;
        }

        function aggregate(Call[] calldata calls) external payable returns (uint256 blockNumber, bytes[] memory returnData);

        function aggregate3(Call3[] calldata calls) external payable returns (Result[] memory returnData);

        function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory returnData);

        function blockAndAggregate(
            Call[] calldata calls
        ) external payable returns (uint256 blockNumber, bytes32 blockHash, Result[] memory returnData);

        function getBasefee() external view returns (uint256 basefee);

        function getBlockHash(uint256 blockNumber) external view returns (bytes32 blockHash);

        function getBlockNumber() external view returns (uint256 blockNumber);

        function getChainId() external view returns (uint256 chainid);

        function getCurrentBlockCoinbase() external view returns (address coinbase);

        function getCurrentBlockDifficulty() external view returns (uint256 difficulty);

        function getCurrentBlockGasLimit() external view returns (uint256 gaslimit);

        function getCurrentBlockTimestamp() external view returns (uint256 timestamp);

        function getEthBalance(address addr) external view returns (uint256 balance);

        function getLastBlockHash() external view returns (bytes32 blockHash);

        function tryAggregate(
            bool requireSuccess,
            Call[] calldata calls
        ) external payable returns (Result[] memory returnData);

        function tryBlockAndAggregate(
            bool requireSuccess,
            Call[] calldata calls
        ) external payable returns (uint256 blockNumber, bytes32 blockHash, Result[] memory returnData);
    }
}

// -------------------- Call Batching Layer Definitions --------------------
/// Default wait duration before sending a batch.
const DEFAULT_WAIT: Duration = Duration::from_millis(1);

#[derive(Debug)]
pub struct CallBatchLayer {
    m3a: Address,
    wait: Duration,
}

impl Default for CallBatchLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl CallBatchLayer {
    /// Create a new `CallBatchLayer` with a default wait of 1ms.
    pub fn new() -> Self {
        Self { m3a: MULTICALL3_ADDRESS, wait: DEFAULT_WAIT }
    }

    /// Set the amount of time to wait before sending the batch.
    ///
    /// This is the amount of time to wait after the first request is received before sending all
    /// the requests received in that time period.
    ///
    /// This means that every request has a maximum delay of `wait` before being sent.
    ///
    /// The default is 1ms.
    pub fn wait(mut self, wait: Duration) -> Self {
        self.wait = wait;
        self
    }

    /// Set the multicall3 address.
    ///
    /// The default is [`MULTICALL3_ADDRESS`].
    pub fn multicall3_address(mut self, m3a: Address) -> Self {
        self.m3a = m3a;
        self
    }
}

impl<P, N> ProviderLayer<P, N> for CallBatchLayer
where
    P: Provider<N> + 'static,
    N: Network,
{
    type Provider = CallBatchProvider<P, N>;

    fn layer(&self, inner: P) -> Self::Provider {
        CallBatchProvider::new(inner, self)
    }
}

type CallBatchMsgTx = TransportResult<IMulticall3::Result>;

struct CallBatchMsg {
    call: IMulticall3::Call3,
    tx: oneshot::Sender<CallBatchMsgTx>,
}

impl fmt::Debug for CallBatchMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BatchProviderMessage(")?;
        self.call.fmt(f)?;
        f.write_str(")")
    }
}

#[derive(Debug)]
enum CallBatchMsgKind<N: Network = Ethereum> {
    Call(N::TransactionRequest),
    BlockNumber,
    ChainId,
    Balance(Address),
}

impl CallBatchMsg {
    fn new<N: Network>(
        kind: CallBatchMsgKind<N>,
        m3a: Address,
    ) -> (Self, oneshot::Receiver<CallBatchMsgTx>) {
        let (tx, rx) = oneshot::channel();
        (Self { call: kind.into_call3(m3a), tx }, rx)
    }
}

impl<N: Network> CallBatchMsgKind<N> {
    fn into_call3(self, m3a: Address) -> IMulticall3::Call3 {
        // Helper closure to build a call for the Multicall3 contract.
        let m3a_call = |data: Vec<u8>| IMulticall3::Call3 {
            target: m3a,
            allowFailure: true,
            callData: data.into(),
        };
        match self {
            Self::Call(tx) => IMulticall3::Call3 {
                // Extract the target address and calldata from the transaction.
                target: tx.to().unwrap_or_default(),
                allowFailure: true,
                callData: tx.input().cloned().unwrap_or_default(),
            },
            Self::BlockNumber => m3a_call(IMulticall3::getBlockNumberCall {}.abi_encode()),
            Self::ChainId => m3a_call(IMulticall3::getChainIdCall {}.abi_encode()),
            Self::Balance(addr) => m3a_call(IMulticall3::getEthBalanceCall { addr }.abi_encode()),
        }
    }
}

pub struct CallBatchProvider<P, N: Network = Ethereum> {
    provider: Arc<P>,
    inner: CallBatchProviderInner,
    _pd: PhantomData<N>,
}

impl<P, N: Network> Clone for CallBatchProvider<P, N> {
    fn clone(&self) -> Self {
        Self {
            provider: self.provider.clone(),
            inner: self.inner.clone(),
            _pd: PhantomData,
        }
    }
}

impl<P: fmt::Debug, N: Network> fmt::Debug for CallBatchProvider<P, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BatchProvider(")?;
        self.provider.fmt(f)?;
        f.write_str(")")
    }
}

impl<P: Provider<N> + 'static, N: Network> CallBatchProvider<P, N> {
    fn new(inner: P, layer: &CallBatchLayer) -> Self {
        let inner = Arc::new(inner);
        let tx = CallBatchBackend::spawn(inner.clone(), layer);
        Self {
            provider: inner,
            inner: CallBatchProviderInner { tx, m3a: layer.m3a },
            _pd: PhantomData,
        }
    }
}

#[derive(Clone)]
struct CallBatchProviderInner {
    tx: mpsc::UnboundedSender<CallBatchMsg>,
    m3a: Address,
}

impl CallBatchProviderInner {
    /// We only want to perform a scheduled multicall if:
    /// - The request has no block ID or state overrides,
    /// - The request has a target address,
    /// - The request has no other properties (`nonce`, `gas`, etc cannot be sent with a multicall).
    ///
    /// Ref: <https://github.com/wevm/viem/blob/ba8319f71503af8033fd3c77cfb64c7eb235c6a9/src/actions/public/call.ts#L295>
    fn should_batch_call<N: Network>(&self, params: &crate::EthCallParams<N>) -> bool {
        // TODO: block ID is not yet implemented
        if params.block().is_some_and(|block| block != BlockId::latest()) {
            return false;
        }
        if params.overrides.as_ref().is_some_and(|overrides| !overrides.is_empty()) {
            return false;
        }
        let tx = params.data();
        if tx.to().is_none() {
            return false;
        }
        if let Ok(serde_json::Value::Object(obj)) = serde_json::to_value(tx) {
            if obj.keys().any(|k| !matches!(k.as_str(), "to" | "data" | "input")) {
                return false;
            }
        }
        true
    }

    async fn schedule<N: Network>(self, msg: CallBatchMsgKind<N>) -> TransportResult<Bytes> {
        let (msg, rx) = CallBatchMsg::new(msg, self.m3a);
        self.tx.send(msg).map_err(|_| TransportErrorKind::backend_gone())?;

        let IMulticall3::Result { success, returnData: data } =
            rx.await.map_err(|_| TransportErrorKind::backend_gone())??;
        if !success {
            let revert_data = if data.is_empty() { "" } else { &format!(" with data: {data}") };
            return Err(TransportErrorKind::custom_str(&format!(
                "multicall batched call reverted{revert_data}"
            )));
        }
        Ok(data)
    }

    async fn schedule_and_decode<N: Network, T>(
        self,
        msg: CallBatchMsgKind<N>,
    ) -> TransportResult<T>
    where
        T: SolValue + From<<T::SolType as SolType>::RustType>,
    {
        let data = self.schedule(msg).await?;
        T::abi_decode(&data, false).map_err(TransportErrorKind::custom)
    }
}


struct CallBatchBackend<P, N: Network = Ethereum> {
    inner: Arc<P>,
    m3a: Address,
    wait: Duration,
    rx: mpsc::UnboundedReceiver<CallBatchMsg>,
    pending: Vec<CallBatchMsg>,
    _pd: PhantomData<N>,
}

impl<P: Provider<N> + 'static, N: Network> CallBatchBackend<P, N> {
    /// Spawn the backend task that collects individual calls into a batch.
    /// Returns the sending side of the channel to enqueue new batched calls.
    fn spawn(inner: Arc<P>, layer: &CallBatchLayer) -> mpsc::UnboundedSender<CallBatchMsg> {
        let CallBatchLayer { m3a, wait } = *layer;
        let (tx, rx) = mpsc::unbounded_channel();
        let this = Self {
            inner,
            m3a,
            wait,
            rx,
            pending: Vec::new(),
            _pd: PhantomData,
        };
        // Spawn the backend task using the provided spawn_task utility.
        this.run().spawn_task();
        tx
    }

    /// The core loop that gathers incoming calls and batches them.
    async fn run(mut self) {
        'outer: loop {
            // Wait for the first message.
            debug_assert!(self.pending.is_empty());
            match self.rx.recv().await {
                Some(msg) => self.process_msg(msg),
                None => break, // Channel closed; exit the loop.
            }

            // Wait for additional messages for a short time.
            debug_assert!(!self.pending.is_empty());
            sleep(self.wait).await;
            'inner: loop {
                match self.rx.try_recv() {
                    Ok(msg) => self.process_msg(msg),
                    Err(mpsc::error::TryRecvError::Empty) => break 'inner,
                    Err(mpsc::error::TryRecvError::Disconnected) => break 'outer,
                }
            }
            // No more messages have arrived within the wait window; send the batch.
            self.send_batch().await;
        }
    }

    /// Add a new call to the pending batch.
    fn process_msg(&mut self, msg: CallBatchMsg) {
        self.pending.push(msg);
    }

    /// Send the pending batch to the network.
    async fn send_batch(&mut self) {
        let result = self.send_batch_inner().await;
        // Take (clear) the pending messages.
        let pending = mem::take(&mut self.pending);
        match result {
            Ok(results) => {
                debug_assert_eq!(results.len(), pending.len());
                // For each message, send the corresponding result.
                for (result, msg) in results.into_iter().zip(pending) {
                    let _ = msg.tx.send(Ok(result));
                }
            }
            Err(e) => {
                // On error, propagate the error to all pending messages.
                for msg in pending {
                    let _ = msg.tx.send(Err(TransportErrorKind::custom_str(&e.to_string())));
                }
            }
        }
    }

    /// Build and send a single aggregated call to Multicall3.
    async fn send_batch_inner(&mut self) -> TransportResult<Vec<IMulticall3::Result>> {
        debug_assert!(!self.pending.is_empty());
        debug!(len = self.pending.len(), "sending multicall");
        // Build a TransactionRequest using the Multicall3 address and our payload.
        let tx = N::TransactionRequest::default()
            .with_to(self.m3a)
            .with_input(self.make_payload());
        // Perform the call via the underlying provider.
        let bytes = self.inner.call(tx).await?;
        if bytes.is_empty() {
            return Err(TransportErrorKind::custom_str(&format!(
                "Multicall3 not deployed at {}",
                self.m3a
            )));
        }
        // Decode the returned bytes into a vector of call results.
        let ret = IMulticall3::aggregate3Call::abi_decode_returns(&bytes, false)
            .map_err(TransportErrorKind::custom)?;
        Ok(ret.returnData)
    }

    /// Create the ABI-encoded payload for the aggregate call.
    fn make_payload(&self) -> Vec<u8> {
        IMulticall3::aggregate3Call {
            calls: self.pending.iter().map(|msg| msg.call.clone()).collect(),
        }
        .abi_encode()
    }
}

// ---------- Provider Implementation for CallBatchProvider ----------

impl<P: Provider<N> + 'static, N: Network> Provider<N> for CallBatchProvider<P, N> {
    fn root(&self) -> &RootProvider<N> {
        self.provider.root()
    }

    fn call(&self, tx: <N as Network>::TransactionRequest) -> crate::EthCall<N, Bytes> {
        // Use CallBatchCaller to decide whether to batch or fallback.
        crate::EthCall::call(CallBatchCaller::new(self), tx)
    }

    fn get_block_number(
        &self,
    ) -> crate::ProviderCall<
        alloy_rpc_client::NoParams,
        alloy_primitives::U64,
        alloy_primitives::BlockNumber,
    > {
        crate::ProviderCall::BoxedFuture(Box::pin(
            self.inner.clone().schedule_and_decode::<N, u64>(CallBatchMsgKind::BlockNumber),
        ))
    }

    fn get_chain_id(
        &self,
    ) -> crate::ProviderCall<
        alloy_rpc_client::NoParams,
        alloy_primitives::U64,
        alloy_primitives::ChainId,
    > {
        crate::ProviderCall::BoxedFuture(Box::pin(
            self.inner.clone().schedule_and_decode::<N, u64>(CallBatchMsgKind::ChainId),
        ))
    }

    fn get_balance(&self, address: Address) -> crate::RpcWithBlock<Address, U256, U256> {
        let this = self.clone();
        crate::RpcWithBlock::new_provider(move |block| {
            if block != BlockId::latest() {
                // Fallback to a direct call if not using the latest block.
                this.provider.get_balance(address).block_id(block).into_future()
            } else {
                ProviderCall::BoxedFuture(Box::pin(
                    this.inner
                        .clone()
                        .schedule_and_decode::<N, U256>(CallBatchMsgKind::Balance(address)),
                ))
            }
        })
    }
}

// ---------- CallBatchCaller Implementation ----------

struct CallBatchCaller {
    inner: CallBatchProviderInner,
    weak: WeakClient,
}

impl CallBatchCaller {
    fn new<P: Provider<N>, N: Network>(provider: &CallBatchProvider<P, N>) -> Self {
        Self {
            inner: provider.inner.clone(),
            weak: provider.provider.weak_client(),
        }
    }
}

impl<N: Network> Caller<N, Bytes> for CallBatchCaller {
    fn call(
        &self,
        params: crate::EthCallParams<N>,
    ) -> TransportResult<crate::ProviderCall<crate::EthCallParams<N>, Bytes>> {
        if !self.inner.should_batch_call(&params) {
            // If the call doesn't qualify for batching, use the underlying provider.
            return Caller::<N, Bytes>::call(&self.weak, params);
        }
        // Otherwise, schedule the call as part of a batch.
        Ok(crate::ProviderCall::BoxedFuture(Box::pin(
            self.inner.clone().schedule::<N>(CallBatchMsgKind::Call(params.into_data())),
        )))
    }

    fn estimate_gas(
        &self,
        params: crate::EthCallParams<N>,
    ) -> TransportResult<crate::ProviderCall<crate::EthCallParams<N>, Bytes>> {
        // Fallback to the weak client's implementation.
        Caller::<N, Bytes>::estimate_gas(&self.weak, params)
    }

    fn call_many(
        &self,
        params: crate::EthCallManyParams<'_>,
    ) -> TransportResult<crate::ProviderCall<crate::EthCallManyParams<'static>, Bytes>> {
        // Fallback to the weak client's implementation.
        Caller::<N, Bytes>::call_many(&self.weak, params)
    }
}
