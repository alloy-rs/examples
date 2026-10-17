//! Example of deploying a contract at a deterministic CREATE2 address using generated bindings.
//!
//! CREATE2 is executed by a factory contract, so address prediction uses the factory's address,
//! not the transaction sender. Ordinary Contract::deploy uses CREATE instead.

use alloy::{
    primitives::{B256, U256},
    providers::{Provider, ProviderBuilder},
    sol,
};
use eyre::{ensure, Result};

// solc 0.8.26 --via-ir --optimize --evm-version paris --bin ExampleCreate2Factory.sol.
sol! {
    #[allow(missing_docs)]
    #[sol(rpc, bytecode = "60808060405234601557610183908161001b8239f35b600080fdfe6080604052600436101561001257600080fd5b60003560e01c63cdcb760a1461002757600080fd5b346100e05760403660031901126100e05760243567ffffffffffffffff81116100e057366023820112156100e05780600401359067ffffffffffffffff82116100e557604051601f8301601f19908116603f0116810167ffffffffffffffff8111828210176100e55760405282815236602484840101116100e05760006020846100dc9560246100c2960183860137830101526004356100fb565b6040516001600160a01b0390911681529081906020820190565b0390f35b600080fd5b634e487b7160e01b600052604160045260246000fd5b906020815191016000f5906001600160a01b0382161561011757565b60405162461bcd60e51b815260206004820152600e60248201526d10d4915055114c8819985a5b195960921b6044820152606490fdfea2646970667358221220e191bd6e40d676e35762eb36d863cf96eb796cb64710cb1c3e9a5fdaaca1e5c864736f6c634300081a0033")]
    contract Create2Factory {
        function deploy(bytes32 salt, bytes memory initCode) public returns (address deployed) {
            assembly {
                deployed := create2(0, add(initCode, 32), mload(initCode), salt)
            }
            require(deployed != address(0), "CREATE2 failed");
        }
    }
}

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Counter,
    "examples/artifacts/Counter.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    let provider = ProviderBuilder::new().connect_anvil_with_wallet();
    let factory = Create2Factory::deploy(&provider).await?;

    // deploy_builder supplies the full creation code, including ABI-encoded constructor
    // arguments when the contract has them. Hashing runtime code would predict the wrong address.
    let init_code = Counter::deploy_builder(&provider).calldata().clone();
    let salt = B256::from(U256::from(42).to_be_bytes::<32>());
    let predicted = factory.address().create2_from_code(salt, &init_code);
    assert!(provider.get_code_at(predicted).await?.is_empty());

    let receipt = factory.deploy_call(salt, init_code.clone()).send().await?.get_receipt().await?;
    ensure!(receipt.status(), "CREATE2 deployment reverted");
    assert!(!provider.get_code_at(predicted).await?.is_empty());

    // Bind to the predicted address and verify the newly deployed contract works.
    let counter = Counter::new(predicted, &provider);
    counter.setNumber(U256::from(42)).send().await?.watch().await?;
    assert_eq!(counter.number().call().await?, U256::from(42));

    // Reusing the same salt and creation code at the same factory cannot redeploy this address.
    assert!(factory.deploy_call(salt, init_code).call().await.is_err());
    println!("CREATE2 deployed Counter at {predicted}");
    Ok(())
}
