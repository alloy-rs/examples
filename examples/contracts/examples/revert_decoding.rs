//! This example demonstrates how to decode revert data into a custom error.

use alloy::{primitives::U256, providers::ProviderBuilder, sol};
use eyre::Result;
use Errors::{ErrorsErrors, SomeCustomError};

// Define a custom error using the sol! macro.
sol! {
    // solc: 0.8.25; solc DecodingRevert.sol --optimize --bin
    #[allow(missing_docs)]
    #[derive(Debug, PartialEq, Eq)]
    library Errors {
        error SomeCustomError(uint256 a);
        error AnotherError(uint64 b);
    }

    #[derive(Debug)]
    #[sol(rpc, bytecode = "6080604052348015600e575f80fd5b5060a780601a5f395ff3fe6080604052348015600e575f80fd5b50600436106026575f3560e01c8063b48fb6cf14602a575b5f80fd5b60396035366004605b565b603b565b005b60405163810f002360e01b81526004810182905260240160405180910390fd5b5f60208284031215606a575f80fd5b503591905056fea26469706673582212200898a6b7d5b1bcc62a40abf2470704fe9c6cd850c77b0654134fc0ecbf0d5e6f64736f6c63430008190033")]
    contract ThrowsError {
        function error(uint256 a) external {
           revert Errors.SomeCustomError(a);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup an Anvil provider with a wallet.
    // Make sure `anvil` is in your $PATH.
    let provider = ProviderBuilder::new().connect_anvil_with_wallet();

    // Deploy the contract.
    let contract = ThrowsError::deploy(&provider).await?;

    // Call the `error` function which will revert with a custom error.
    let err = contract.error(U256::from(1)).call().await.unwrap_err();

    // Get the raw bytes of the revert data.
    let revert_data = err.as_revert_data().unwrap();

    println!("Decoding revert data: {revert_data:?}");

    // Decode the revert data as a custom error.
    let decoded_err = err.as_decoded_error::<SomeCustomError>().unwrap();

    println!("Decoded as: {decoded_err:?}");

    assert_eq!(decoded_err, SomeCustomError { a: U256::from(1) });

    // At times you may not be sure which error is being returned by the function.
    // In such cases, we can try to decode the revert over all the possible errors provieded to the
    // the sol! macro.
    let decoded_err = err.as_decoded_interface_error::<Errors::ErrorsErrors>().unwrap();

    // The above returns an enum with the errors as its variants.
    match decoded_err {
        ErrorsErrors::SomeCustomError(err) => {
            println!("Decoded as: {err:?}");
            assert_eq!(err.a, U256::from(1));
        }
        ErrorsErrors::AnotherError(err) => {
            println!("Decoded as: {err:?}");
            assert_eq!(err.b, 0);
        }
    }

    Ok(())
}
