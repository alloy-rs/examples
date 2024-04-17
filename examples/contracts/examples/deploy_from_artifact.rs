//! Example of deploying a contract from an artifact to Anvil and interacting with it.

use alloy::{
    network::EthereumSigner, node_bindings::Anvil, primitives::U256, providers::ProviderBuilder,
    signers::wallet::LocalWallet, sol,
};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Counter,
    "examples/artifacts/Counter.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().try_spawn()?;

    // Set up signer from the first default Anvil account (Alice).
    let signer: LocalWallet = anvil.keys()[0].clone().into();

    // Create a provider with a signer.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(signer))
        .on_http(rpc_url)?;

    println!("Anvil running at `{}`", anvil.endpoint());

    // Deploy the contract.
    let contract = Counter::deploy(&provider).await?;

    println!("Deployed contract at address: {:?}", contract.address());

    let builder = contract.setNumber(U256::from(42));
    let receipt = builder.send().await?.get_receipt().await?;

    println!("Set number to 42: {:?}", receipt.transaction_hash);

    // Increment the number to 43.
    let builder = contract.increment();
    let receipt = builder.send().await?.get_receipt().await?;

    println!("Incremented number: {:?}", receipt.transaction_hash);

    // Retrieve the number, which should be 43.
    let Counter::numberReturn { _0 } = contract.number().call().await?;

    println!("Retrieved number: {:?}", _0.to_string());

    Ok(())
}
