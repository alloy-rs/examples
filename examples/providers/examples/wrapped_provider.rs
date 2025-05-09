//! Example demonstrating how to wrap the [`Provider`] in a struct and pass it through free
//! functions.

use alloy::{
    network::EthereumWallet,
    node_bindings::Anvil,
    primitives::address,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionReceipt,
    signers::local::PrivateKeySigner,
    sol,
    transports::{TransportErrorKind, TransportResult},
};
use eyre::Result;
use Counter::CounterInstance;

// Codegen from embedded Solidity code and precompiled bytecode.
sol! {
    #[allow(missing_docs)]
    // solc v0.8.26; solc Counter.sol --via-ir --optimize --bin
    #[sol(rpc, bytecode="6080806040523460135760df908160198239f35b600080fdfe6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033")]
    contract Counter {
        uint256 public number;

        function setNumber(uint256 newNumber) public {
            number = newNumber;
        }

        function increment() public {
            number++;
        }
    }
}

/// Deployer that ingests a [`Provider`] and [`EthereumWallet`] and deploys [`Counter`]
struct Deployer<P: Provider> {
    provider: P,
    wallet: EthereumWallet,
}

impl<P: Provider> Deployer<P> {
    /// Create a new instance of [`Deployer`].
    fn new(provider: P, private_key: PrivateKeySigner) -> Self {
        let wallet = EthereumWallet::new(private_key);
        Self { provider, wallet }
    }

    /// Deploys [`Counter`] using the given [`EthereumWallet`] and returns [`CounterInstance`]
    async fn deploy(&self) -> Result<CounterInstance<&P>> {
        let addr = CounterInstance::deploy_builder(&self.provider)
            .from(self.wallet.default_signer().address())
            .deploy()
            .await?;

        Ok(CounterInstance::new(addr, &self.provider))
    }
}

struct CounterContract<P: Provider> {
    provider: P,
    counter: CounterInstance<P>,
}

impl<P: Provider> CounterContract<P> {
    /// Create a new instance of [`CounterContract`].
    const fn new(provider: P, counter: CounterInstance<P>) -> Self {
        Self { provider, counter }
    }

    /// Returns the current number stored in the [`Counter`].
    async fn number(&self) -> TransportResult<u64> {
        let number = self.counter.number().call().await.map_err(TransportErrorKind::custom)?;
        Ok(number.to::<u64>())
    }

    /// Increments the number stored in the [`Counter`].
    async fn increment(&self) -> TransportResult<TransactionReceipt> {
        self.counter
            .increment()
            .from(address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266")) // Default anvil signer
            .send()
            .await
            .map_err(TransportErrorKind::custom)?
            .get_receipt()
            .await
            .map_err(TransportErrorKind::custom)
    }

    /// Returns the inner provider.
    fn provider(&self) -> &impl Provider {
        &self.provider
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().spawn();

    let provider = ProviderBuilder::new().connect(anvil.endpoint().as_str()).await?;

    let signer_pk = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
    let deployer = Deployer::new(provider.clone(), signer_pk);
    let counter_instance = deployer.deploy().await?;

    println!("Deployed `Counter` at {}", counter_instance.address());

    let counter = CounterContract::new(&provider, counter_instance);
    let num = counter.number().await?;

    println!("Current number: {num}");

    counter.increment().await?;

    let num = counter.number().await?;

    println!("Incremented number: {num}");

    let block_num = counter.provider().get_block_number().await?;

    println!("Current block number: {block_num}");

    Ok(())
}
