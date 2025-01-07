//! Example demonstrating how to wrap the [`Provider`] in a struct and pass it through free
//! functions.

use alloy::{
    network::EthereumWallet,
    node_bindings::Anvil,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
    transports::TransportResult,
};
use eyre::Result;
use Counter::CounterInstance;

/// Creates and returns an implementation of the [`Provider`] trait.
async fn get_provider(url: &str) -> TransportResult<impl Provider> {
    ProviderBuilder::new().with_recommended_fillers().on_builtin(url).await
}

/// Simple free function to get the latest block number.
async fn get_block_number<P: Provider>(provider: &P) -> TransportResult<u64> {
    provider.get_block_number().await
}

/// Deployer that ingests a [`Provider`] and [`EthereumWallet`] and deploys [`Counter`]
struct Deployer<P: Provider> {
    provider: P,
    wallet: EthereumWallet,
}

impl<P: Provider> Deployer<P> {
    /// Create a new instance of `MyProvider`.
    fn new(provider: P, private_key: PrivateKeySigner) -> Self {
        let wallet = EthereumWallet::new(private_key);
        Self { provider, wallet }
    }

    /// Deploys [`Counter`] using the given [`EthereumWallet`] and returns the address it was
    /// deployed at.
    async fn deploy(&self) -> Result<Address> {
        CounterInstance::deploy_builder(&self.provider)
            .from(self.wallet.default_signer().address())
            .deploy()
            .await
            .map_err(Into::into)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().spawn();

    let provider = get_provider(&anvil.endpoint()).await?;
    let latest_block = get_block_number(&provider).await?;

    println!("Latest block number: {latest_block}");

    let signer_pk = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
    let counter_address = Deployer::new(&provider, signer_pk).deploy().await?;

    println!("Deployed `Counter` at {counter_address}");

    let counter = CounterInstance::new(counter_address, provider);
    let num = counter.number().call().await?.number.to::<u64>();

    println!("Current Number {num}");

    Ok(())
}

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
