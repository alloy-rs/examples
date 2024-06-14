use alloy::{
    contract::{ContractInstance, Interface},
    network::Ethereum,
    providers::ProviderBuilder,
    sol,
    transports::http::{Client, Http},
};
use eyre::Result;

// Codegen from embedded Solidity code and precompiled bytecode.
sol! {
    #[allow(missing_docs)]
    // solc v0.8.26; solc a.sol --via-ir --optimize --bin
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

#[tokio::main]
async fn main() -> Result<()> {
    let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet();

    // Deploy the Counter contract
    let counter = Counter::deploy(provider.clone()).await?;

    // Get the contract abi
    let path = std::env::current_dir()?.join("examples/contracts/examples/artifacts/Counter.json");

    // Read the artifact which contains `abi`, `bytecode`, `deployedBytecode`,and `metadata`
    let artifact = std::fs::read(path).unwrap();
    let json: serde_json::Value = serde_json::from_slice(&artifact)?;

    // Get `abi` from the artifact
    let abi_val = json.get("abi").unwrap();
    let abi = serde_json::from_str(&abi_val.to_string())?;

    // Create a new `ContractInstance` of the Counter contract from the abi
    let counter_instance: ContractInstance<Http<Client>, _, Ethereum> =
        ContractInstance::new(*counter.address(), provider.clone(), Interface::new(abi));

    // Interact with the contract

    // Read
    let init_val = counter_instance.function("number", &[])?.call().await?;

    // Decode the value
    println!("init val {:#?}", init_val);

    Ok(())
}
