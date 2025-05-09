//! Example showing how to decode events and errors from a contract using the `sol!` macro.

use alloy::{providers::ProviderBuilder, sol};
use eyre::Result;
use futures_util::StreamExt;

// Generate a contract instance from Solidity.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc, bytecode = "608060405260008055348015601357600080fd5b506103e9806100236000396000f3fe608060405234801561001057600080fd5b50600436106100575760003560e01c80632baeceb71461005c5780632ccbdbca1461006657806361bc221a14610070578063c3e8b5ca1461008e578063d09de08a14610098575b600080fd5b6100646100a2565b005b61006e610103565b005b61007861013e565b60405161008591906101f9565b60405180910390f35b610096610144565b005b6100a061017f565b005b60016000808282546100b49190610243565b925050819055506000543373ffffffffffffffffffffffffffffffffffffffff167fdc69c403b972fc566a14058b3b18e1513da476de6ac475716e489fae0cbe4a2660405160405180910390a3565b6040517f23b0db14000000000000000000000000000000000000000000000000000000008152600401610135906102e3565b60405180910390fd5b60005481565b6040517fa5f9ec670000000000000000000000000000000000000000000000000000000081526004016101769061034f565b60405180910390fd5b6001600080828254610191919061036f565b925050819055506000543373ffffffffffffffffffffffffffffffffffffffff167ff6d1d8d205b41f9fb9549900a8dba5d669d68117a3a2b88c1ebc61163e8117ba60405160405180910390a3565b6000819050919050565b6101f3816101e0565b82525050565b600060208201905061020e60008301846101ea565b92915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b600061024e826101e0565b9150610259836101e0565b92508282039050818112600084121682821360008512151617156102805761027f610214565b5b92915050565b600082825260208201905092915050565b7f4572726f72204100000000000000000000000000000000000000000000000000600082015250565b60006102cd600783610286565b91506102d882610297565b602082019050919050565b600060208201905081810360008301526102fc816102c0565b9050919050565b7f4572726f72204200000000000000000000000000000000000000000000000000600082015250565b6000610339600783610286565b915061034482610303565b602082019050919050565b600060208201905081810360008301526103688161032c565b9050919050565b600061037a826101e0565b9150610385836101e0565b9250828201905082811215600083121683821260008412151617156103ad576103ac610214565b5b9291505056fea2646970667358221220a878a3c1da1a1170e4496cdbc63bd5ed1587374bcd6cf6d4f1d5b88fa981795d64736f6c63430008190033")]
    contract CounterWithError {
        int256 public counter = 0;

        // Events - using `Debug` to print the events
        #[derive(Debug)]
        event Increment(address indexed by, int256 indexed value);
        #[derive(Debug)]
        event Decrement(address indexed by, int256 indexed value);

        // Custom Error
        error ErrorA(string message);
        error ErrorB(string message);

        // Functions
        function increment() public {
            counter += 1;
            emit Increment(msg.sender, counter);
        }

        function decrement() public {
            counter -= 1;
            emit Decrement(msg.sender, counter);
        }

        function revertA() public pure {
            revert ErrorA("Error A");
        }

        function revertB() public pure {
            revert ErrorB("Error B");
        }
    }
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let provider = ProviderBuilder::new().connect_anvil_with_wallet();

    // Deploy the `Counter` contract.
    let contract = CounterWithError::deploy(provider.clone()).await?;

    // Setup a filter for the Increment and Decrement events.
    let increment_filter = contract.Increment_filter().watch().await?;
    let decrement_filter = contract.Decrement_filter().watch().await?;

    // Convert to streams.
    let mut increment_stream = increment_filter.into_stream();
    let mut decrement_stream = decrement_filter.into_stream();

    // Call the increment and decrement functions.
    let increment_call = contract.increment();
    let decrement_call = contract.decrement();

    // Wait for the calls to be included.
    let _increment_res = increment_call.send().await?;
    let _decrement_res = decrement_call.send().await?;

    // Catch the events.
    for _ in 0..2 {
        let log = tokio::select! {
            Some(Ok((incr, log))) = increment_stream.next() => {
                println!("Increment: {incr:#?}");
                // Return raw log
                log
            }
            Some(Ok((decr, log))) = decrement_stream.next() => {
                println!("Decrement: {decr:#?}");
                // Return raw log
                log
            }
        };
        println!("Log: {log:#?}");
    }

    // Call the `revertA` function.
    let err_call = contract.revertA();
    let err_result = err_call.send().await;

    if let Err(err) = err_result {
        println!("Error A: {err:#?}");
    }

    // Call the `revertB` function.
    let err_call = contract.revertB();
    let err_result = err_call.send().await;

    if let Err(err) = err_result {
        println!("Error B: {err:#?}");
    }

    Ok(())
}
