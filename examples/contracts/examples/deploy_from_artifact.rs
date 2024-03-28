//! Example of deploying a contract from an artifact to Anvil and interacting with it.

use alloy::{
    network::EthereumSigner,
    node_bindings::Anvil,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::client::RpcClient,
    signers::wallet::LocalWallet,
    sol,
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
        .signer(EthereumSigner::from(signer))
        .on_client(RpcClient::new_http(rpc_url));

    println!("Anvil running at `{}`", anvil.endpoint());

    // Get the base fee for the block.
    let base_fee = provider.get_gas_price().await?;

    // Deploy the contract.
    let contract_builder = Counter::deploy_builder(&provider);
    let estimate = contract_builder.estimate_gas().await?;
    let contract_address =
        contract_builder.gas(estimate).gas_price(base_fee).nonce(0).deploy().await?;

    println!("Deployed contract at address: {contract_address:?}");

    let contract = Counter::new(contract_address, &provider);

    let estimate = contract.setNumber(U256::from(42)).estimate_gas().await?;
    let builder = contract.setNumber(U256::from(42)).nonce(1).gas(estimate).gas_price(base_fee);
    let receipt = builder.send().await?.get_receipt().await?;

    println!("Set number to 42: {:?}", receipt.transaction_hash);

    // Increment the number to 43.
    let estimate = contract.increment().estimate_gas().await?;
    let builder = contract.increment().nonce(2).gas(estimate).gas_price(base_fee);
    let receipt = builder.send().await?.get_receipt().await?;

    println!("Incremented number: {:?}", receipt.transaction_hash);

    // Retrieve the number, which should be 43.
    let Counter::numberReturn { _0 } = contract.number().call().await?;

    println!("Retrieved number: {:?}", _0.to_string());

    Ok(())
}
