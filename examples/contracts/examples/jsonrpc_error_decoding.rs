//! This example demonstrates how to decode a custom JSON RPC error.

use alloy::{primitives::U256, rpc::json_rpc::ErrorPayload, sol};
use eyre::Result;

// Define a custom error using the sol! macro.
sol! {
    #[allow(missing_docs)]
    library Errors {
        error SomeCustomError(uint256 a);
    }
}

fn main() -> Result<()> {
    // Sample JSON error payload from an Ethereum JSON RPC response.
    let json = r#"{"code":3,"message":"execution reverted: ","data":"0x810f00230000000000000000000000000000000000000000000000000000000000000001"}"#;

    // Parse the JSON into an `ErrorPayload` struct.
    let payload: ErrorPayload = serde_json::from_str(json)?;

    // Attempt to decode the error payload as our custom error.
    let value = payload.as_decoded_error::<Errors::SomeCustomError>().unwrap();

    assert_eq!(value.a, U256::from(1));

    Ok(())
}
