//! Example of deploying an ERC-1167 minimal proxy and calling its implementation through bindings.
//!
//! The proxy delegates calls to an implementation while retaining its own storage. Constructors
//! run only on the implementation; a clone requiring initialization must be initialized separately.

use alloy::{
    network::TransactionBuilder,
    primitives::{hex, Address, FixedBytes, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    sol,
};
use eyre::{ensure, OptionExt, Result};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Counter,
    "examples/artifacts/Counter.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    let provider = ProviderBuilder::new().connect_anvil_with_wallet();
    let implementation = Counter::deploy(&provider).await?;
    implementation.setNumber(U256::from(7)).send().await?.watch().await?;

    // The 55-byte creation code installs the standard 45-byte ERC-1167 runtime code.
    let init_code = minimal_proxy_bytecode(*implementation.address());
    let tx = TransactionRequest::default().with_deploy_code(init_code);
    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;
    ensure!(receipt.status(), "Proxy deployment reverted");
    let proxy_address = receipt.contract_address.ok_or_eyre("Missing proxy address")?;
    assert_eq!(provider.get_code_at(proxy_address).await?.as_ref(), &init_code[10..]);

    // Use the implementation ABI at the proxy address. Storage belongs to the proxy, so the
    // implementation's existing value is not copied and proxy calls do not change that value.
    let proxy = Counter::new(proxy_address, &provider);
    assert_eq!(proxy.number().call().await?, U256::ZERO);
    proxy.setNumber(U256::from(42)).send().await?.watch().await?;
    proxy.increment().send().await?.watch().await?;
    assert_eq!(proxy.number().call().await?, U256::from(43));
    assert_eq!(implementation.number().call().await?, U256::from(7));

    println!("ERC-1167 proxy {proxy_address} delegates to {}", implementation.address());
    Ok(())
}

// ERC-1167: https://eips.ethereum.org/EIPS/eip-1167
const fn minimal_proxy_bytecode(implementation: Address) -> [u8; 55] {
    let prefix = FixedBytes::new(hex!("3d602d80600a3d3981f3363d3d373d3d3d363d73"));
    let suffix = FixedBytes::new(hex!("5af43d82803e903d91602b57fd5bf3"));
    prefix.concat_const::<20, 40>(implementation.0).concat_const::<15, 55>(suffix).0
}
