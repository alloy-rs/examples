//! Example of how to transfer ERC20 tokens from one account to another.

use alloy::{
    network::Ethereum,
    node_bindings::Anvil,
    primitives::{Address, Bytes, U256},
    providers::{Provider, ReqwestProvider},
    rpc::types::eth::TransactionRequest,
    sol,
    sol_types::SolCall,
};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    ERC20Example,
    "examples/contracts/ERC20Example.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().fork("https://eth.merkle.io").try_spawn()?;

    // Create a provider.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = ReqwestProvider::<Ethereum>::new_http(rpc_url);

    // Create two users, Alice and Bob.
    let alice = anvil.addresses()[0];
    let bob = anvil.addresses()[1];

    // Deploy the `ERC20Example` contract.
    let contract_address = deploy_token_contract(&provider, alice).await?;

    // Create the transaction input to transfer 100 tokens from Alice to Bob.
    let input = ERC20Example::transferCall { to: bob, amount: U256::from(100) }.abi_encode();
    let input = Bytes::from(input);

    // Create a transaction with the input.
    let tx = TransactionRequest {
        from: Some(alice),
        to: Some(contract_address),
        input: Some(input).into(),
        ..Default::default()
    };

    // Send the transaction and wait for the receipt.
    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    println!("Send transaction: {:?}", receipt.transaction_hash);

    // Check the balances of Alice and Bob after the transfer.
    let alice_balance = balance_of(&provider, alice, contract_address).await?;
    let bob_balance = balance_of(&provider, bob, contract_address).await?;

    assert_eq!(alice_balance, U256::from(999999999999999999900_i128));
    assert_eq!(bob_balance, U256::from(100));

    Ok(())
}

async fn deploy_token_contract(
    provider: &ReqwestProvider<Ethereum>,
    from: Address,
) -> Result<Address> {
    // Compile the contract.
    let bytecode = ERC20Example::BYTECODE.to_owned();

    // Create a transaction.
    let tx = TransactionRequest {
        from: Some(from),
        input: Some(bytecode).into(),
        to: None,
        ..Default::default()
    };

    // Send the transaction and wait for the receipt.
    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    // Get the contract address.
    let contract_address = receipt.contract_address.expect("Contract address not found");

    println!("Deployed contract at: {contract_address}");

    Ok(contract_address)
}

async fn balance_of(
    provider: &ReqwestProvider<Ethereum>,
    account: Address,
    contract_address: Address,
) -> Result<U256> {
    // Encode the call.
    let call = ERC20Example::balanceOfCall { account }.abi_encode();
    let input = Bytes::from(call);

    // Create a transaction.
    let tx = TransactionRequest {
        to: Some(contract_address),
        input: Some(input).into(),
        ..Default::default()
    };

    // Call the contract.
    let result = provider.call(&tx, None).await?;

    // Decode the result.
    let result = ERC20Example::balanceOfCall::abi_decode_returns(&result, true)?._0;

    Ok(result)
}
