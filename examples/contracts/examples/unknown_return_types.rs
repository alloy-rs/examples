//! Example demonstrating how one can handle unknown / complex return types using `DynSol`.

use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    json_abi::JsonAbi,
    network::TransactionBuilder,
    primitives::{hex, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let provider = ProviderBuilder::new().connect_anvil_with_wallet();

    // Get the first account from the wallet, Alice.
    let alice = provider.get_accounts().await?[0];

    let bytecode = hex::decode(
        // contract Colors {
        //     struct Color {
        //         uint8 r;
        //         uint8 g;
        //         uint8 b;
        //     }
        //
        //     mapping(address => Color) public colors;
        //
        //     function setColor(uint8 r, uint8 g, uint8 b) public {
        //         colors[msg.sender] = Color(r, g, b);
        //     }
        //
        //     function getColor(address user) public view returns (Color memory) {
        //         return colors[user];
        //     }
        //
        //     function getColorAsTuple(
        //         address user
        //     ) public view returns (uint8, uint8, uint8) {
        //         return (colors[user].r, colors[user].g, colors[user].b);
        //     }
        // }
        "6080604052348015600f57600080fd5b506105fb8061001f6000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c8063610c76f01461005157806384b5e5961461006d57806399efff171461009d578063befdb4f6146100cf575b600080fd5b61006b60048036038101906100669190610435565b610101565b005b610087600480360381019061008291906104e6565b6101ce565b6040516100949190610564565b60405180910390f35b6100b760048036038101906100b291906104e6565b61027d565b6040516100c69392919061058e565b60405180910390f35b6100e960048036038101906100e491906104e6565b61037c565b6040516100f89392919061058e565b60405180910390f35b60405180606001604052808460ff1681526020018360ff1681526020018260ff168152506000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008201518160000160006101000a81548160ff021916908360ff16021790555060208201518160000160016101000a81548160ff021916908360ff16021790555060408201518160000160026101000a81548160ff021916908360ff160217905550905050505050565b6101d66103cd565b6000808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000206040518060600160405290816000820160009054906101000a900460ff1660ff1660ff1681526020016000820160019054906101000a900460ff1660ff1660ff1681526020016000820160029054906101000a900460ff1660ff1660ff16815250509050919050565b60008060008060008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160009054906101000a900460ff166000808673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160019054906101000a900460ff166000808773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160029054906101000a900460ff169250925092509193909250565b60006020528060005260406000206000915090508060000160009054906101000a900460ff16908060000160019054906101000a900460ff16908060000160029054906101000a900460ff16905083565b6040518060600160405280600060ff168152602001600060ff168152602001600060ff1681525090565b600080fd5b600060ff82169050919050565b610412816103fc565b811461041d57600080fd5b50565b60008135905061042f81610409565b92915050565b60008060006060848603121561044e5761044d6103f7565b5b600061045c86828701610420565b935050602061046d86828701610420565b925050604061047e86828701610420565b9150509250925092565b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b60006104b382610488565b9050919050565b6104c3816104a8565b81146104ce57600080fd5b50565b6000813590506104e0816104ba565b92915050565b6000602082840312156104fc576104fb6103f7565b5b600061050a848285016104d1565b91505092915050565b61051c816103fc565b82525050565b6060820160008201516105386000850182610513565b50602082015161054b6020850182610513565b50604082015161055e6040850182610513565b50505050565b60006060820190506105796000830184610522565b92915050565b610588816103fc565b82525050565b60006060820190506105a3600083018661057f565b6105b0602083018561057f565b6105bd604083018461057f565b94935050505056fea2646970667358221220ce426adf2fbf80a861f23a5eb1e99a281bb07e427b9beed059e09c285f16db6c64736f6c634300081a0033"
    )?;
    let deploy_tx = TransactionRequest::default().from(alice).with_deploy_code(bytecode);

    let contract_address = provider
        .send_transaction(deploy_tx)
        .await?
        .get_receipt()
        .await?
        .contract_address
        .expect("Failed to get contract address");

    // Get the contract abi.
    let path = std::env::current_dir()?.join("examples/contracts/examples/abi/Colors.json");
    let contents = std::fs::read(path)?;
    let abi: JsonAbi = serde_json::from_slice(&contents)?;

    // Create a new `ContractInstance` of the Counter contract from the abi.
    let counter_instance =
        ContractInstance::new(contract_address, provider.clone(), Interface::new(abi));

    // Interact with the contract.
    assert_eq!(counter_instance.abi().functions().count(), 4);

    // Set color to white.
    let r = DynSolValue::Uint(U256::from(255), 8); // uint8
    let g = DynSolValue::Uint(U256::from(255), 8); // uint8
    let b = DynSolValue::Uint(U256::from(255), 8); // uint8
    let set_color_func = counter_instance.function("setColor", &[r, g, b])?;
    let set_color_receipt = set_color_func.send().await?.get_receipt().await?;
    assert!(set_color_receipt.status());

    // Get the color.
    let get_color_func = counter_instance.function("getColor", &[DynSolValue::Address(alice)])?;
    let get_color_result = get_color_func.call().await?;

    // The `r`, `g`, `b` values in the `Color` struct get converted to a `DynSolValue::Tuple`.
    assert!(get_color_result.len() == 1);
    for value in get_color_result {
        if let DynSolValue::Tuple(struct_as_tuple) = value {
            println!("{struct_as_tuple:?}");
        }
    }

    // Get the color as tuple.
    let get_color_tuple =
        counter_instance.function("getColorAsTuple", &[DynSolValue::Address(alice)])?;
    let get_color_tuple_result = get_color_tuple.call().await?;

    // The `r`, `g`, `b` are returned as a solidity tuple and hence represented as individual
    // `DynSolValue::Uint`.
    assert!(get_color_tuple_result.len() == 3);
    for value in get_color_tuple_result {
        println!("{value:?}");
    }

    Ok(())
}
