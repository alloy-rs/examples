//! Example depicting how to decode logs present in a transaction receipt.

use alloy::{providers::ProviderBuilder, sol};

sol! {
    #[sol(rpc, bytecode = "6080604052348015600e575f80fd5b506101858061001c5f395ff3fe608060405234801561000f575f80fd5b506004361061003f575f3560e01c806306661abd146100435780632baeceb71461005d578063d09de08a14610067575b5f80fd5b61004b5f5481565b60405190815260200160405180910390f35b61006561006f565b005b6100656100c7565b5f545f0361007957565b60015f8082825461008a9190610123565b90915550505f546040519081527f32814a5bdfd1b8c3d76c49c54e043d6e8aa93d197a09e16599b567135503f748906020015b60405180910390a1565b60015f808282546100d8919061013c565b90915550505f546040519081527f51af157c2eee40f68107a47a49c32fbbeb0a3c9e5cd37aa56e88e6be92368a81906020016100bd565b634e487b7160e01b5f52601160045260245ffd5b818103818111156101365761013661010f565b92915050565b808201808211156101365761013661010f56fea2646970667358221220d955d2934ffc8ca69c62bc3b116ad64fca0144d8bd6cf5a635ae1954025d810c64736f6c63430008190033")]
    contract Counter {
        uint256 public count;

        event Increment(uint256 value);
        event Decrement(uint256 value);

        function increment() public {
            count += 1;
            emit Increment(count);
        }

        function decrement() public {
            if (count == 0) return;
            count -= 1;
            emit Decrement(count);
        }
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Create an AnvilProvider
    // Ensure `anvil` is available in $PATH.
    let provider = ProviderBuilder::new().connect_anvil_with_wallet();

    // Deploy the `Counter` contract.
    let counter = Counter::deploy(&provider).await?;

    // Send a transaction to increment the counter.
    let increment = counter.increment().send().await?;

    // Get the receipt of the transaction.
    let receipt = increment.get_receipt().await?;

    // Decode the `Increment` event log.
    // Returns None, if there were no logs emitted or if the emitted logs cannot be decoded as
    // `Increment`.
    let maybe_log = receipt.decoded_log::<Counter::Increment>();

    let Some(increment_log) = maybe_log else { eyre::bail!("Increment not emitted") };

    assert_eq!(increment_log.address, *counter.address());
    let Counter::Increment { value } = increment_log.data;
    println!("Incremented value: {value}");

    // Attempt to decode as a `Decrement` event log.
    let decrement_log = receipt.decoded_log::<Counter::Decrement>();

    // None, as there is no `Decrement` event emitted.
    assert!(decrement_log.is_none());

    Ok(())
}
