//! This example demonstrates how to decode a custom JSON RPC error.

use alloy::{primitives::U256, rpc::json_rpc::ErrorPayload, sol};

fn main() -> eyre::Result<()> {
    // Define a custom error using the sol! macro
    sol! {
        library Errors {
            error SomeCustomError(uint256 a);
        }
    }

    // Sample JSON error payload from an Ethereum RPC response
    let json = r#"{"code":3,"message":"execution reverted: ","data":"0x810f00230000000000000000000000000000000000000000000000000000000000000001"}"#;

    // Parse the JSON into an ErrorPayload struct
    let payload: ErrorPayload = serde_json::from_str(json)?;

    // Attempt to decode the error payload as our custom error
    let Errors::ErrorsErrors::SomeCustomError(value) =
        payload.as_decoded_error::<Errors::ErrorsErrors>(false).unwrap();

    assert_eq!(value.a, U256::from(1));

    Ok(())
}
