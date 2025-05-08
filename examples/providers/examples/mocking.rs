//! This example shows to how to use the `MockTransport` to mock the provider responses for testing
//! purposes.
//!
//! This aids in testing parts of your code that relies on provider without having to connect to a
//! network.
use alloy::{
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::json_rpc::ErrorPayload,
    transports::mock::Asserter,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Asserter is used to push responses to the provided whether success or failure as you'll see
    // in the next steps. Note that the `Asserter` wraps a FIFO queue and responses are returned
    // in the order they are pushed.
    let asserter = Asserter::new();

    // Initialize the provider with the `MockTransport` that intercepts incoming requests and uses
    // the `Asserter` to return the next response.
    // `Asserter` is cheaply cloneable as the underlying queue is wrapped in an `Arc`.
    let provider = ProviderBuilder::new().connect_mocked_client(asserter.clone());

    // Mock the response for a basic `get_block_number` request.
    let expected_bn = 1000;
    // The `.push_success` accepts any type that implements `serde::Serialize`
    asserter.push_success(&expected_bn);
    // Push an error response.
    asserter.push_failure_msg("SOME ERROR");
    // One can also push a custom `ErrorPayload` response that you're expecting the RPC server to
    // return.
    asserter.push_failure(ErrorPayload::invalid_request());

    let actual_bn = provider.get_block_number().await?;
    assert_eq!(actual_bn, expected_bn);

    // Mock an error response.
    let err = provider.get_block_number().await.unwrap_err();
    assert!(err.as_error_resp().unwrap().to_string().contains("SOME ERROR"));

    // Mock a custom error response.
    let err = provider.get_block_number().await.unwrap_err();
    assert_eq!(err.as_error_resp().unwrap().code, -32600);
    assert_eq!(err.as_error_resp().unwrap().message, "Invalid Request");

    // Mocking a certain response and expecting a different response will lead to a deserialization
    // error.
    let expected_balance = U256::MAX;
    asserter.push_success(&expected_balance);

    // Try to get block_number instead of balance.
    // This will fail because the next response is going to be U256::MAX, but `get_block_number`
    // tries to deserialize the response to `u64` which is not possible.
    let err = provider.get_block_number().await.unwrap_err();
    assert!(err.is_deser_error());

    // Since we're using the `MockTransport`, we can assert raw JSON-RPC requests as well.
    let expected_balance = U256::from(1000);
    asserter.push_success(&expected_balance);

    // Raw request
    let addr = Address::with_last_byte(1);
    let balance: U256 = provider.raw_request("eth_getBalance".into(), (addr,)).await?;

    assert_eq!(balance, expected_balance);

    Ok(())
}
