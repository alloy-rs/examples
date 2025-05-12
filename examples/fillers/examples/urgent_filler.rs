//! Rolls a custom filler that fetches priority fee information from an external endpoint and fills
//! the EIP-1559 gas fields for urgent inclusion.

use eyre::Result;

use alloy::{
    consensus::Transaction,
    network::{Network, TransactionBuilder},
    primitives::{Address, U256},
    providers::{
        fillers::{FillerControlFlow, TxFiller},
        Provider, ProviderBuilder, SendableTx,
    },
    rpc::types::TransactionRequest,
    transports::{RpcError, TransportErrorKind, TransportResult},
};
use reqwest::Client;

/// The custom filler that fetches gas prices from an external API
/// and fills the EIP-1559 gas fields for urgent inclusion.
#[derive(Clone, Debug, Default)]
pub struct UrgentQueue {
    client: Client,
}

impl UrgentQueue {
    /// Instantiate a new [`UrgentQueue`] filler.
    pub fn new() -> Self {
        Self { client: Client::new() }
    }
}

/// The fillable type for the [`UrgentQueue`] filler.
#[derive(Debug)]
pub struct GasPriceFillable {
    max_fee_per_gas: u128,
    max_priority_fee_per_gas: u128,
}

impl<N: Network> TxFiller<N> for UrgentQueue {
    type Fillable = GasPriceFillable;

    // Implements the status check for the filler.
    // indicating whether the filler is ready to  fill in the transaction request, or if it is
    // missing required properties.
    fn status(&self, tx: &<N as Network>::TransactionRequest) -> FillerControlFlow {
        if tx.max_fee_per_gas().is_some() && tx.max_priority_fee_per_gas().is_some() {
            FillerControlFlow::Finished
        } else {
            FillerControlFlow::Ready
        }
    }
    fn fill_sync(&self, _tx: &mut SendableTx<N>) {}

    // Fills in the transaction request with properties from GasFillable
    async fn fill(
        &self,
        fillable: Self::Fillable,
        mut tx: SendableTx<N>,
    ) -> TransportResult<SendableTx<N>> {
        if let Some(builder) = tx.as_mut_builder() {
            println!("Filling transaction with gas prices from Blocknative");
            builder.set_max_fee_per_gas(fillable.max_fee_per_gas);
            builder.set_max_priority_fee_per_gas(fillable.max_priority_fee_per_gas);
        } else {
            panic!("Expected a builder");
        }

        Ok(tx)
    }

    // Prepares the gas fees by fetching the blocknative API.
    async fn prepare<P>(
        &self,
        _provider: &P,
        _tx: &<N as Network>::TransactionRequest,
    ) -> TransportResult<Self::Fillable>
    where
        P: Provider<N>,
    {
        println!("Fetching gas prices from Blocknative");
        let data =
            match self.client.get("https://api.blocknative.com/gasprices/blockprices").send().await
            {
                Ok(res) => res,
                Err(e) => {
                    return Err(RpcError::Transport(TransportErrorKind::Custom(Box::new(
                        std::io::Error::other(format!("Failed to fetch gas price, {e}")),
                    ))));
                }
            };
        let body = data.text().await.unwrap();
        let json = serde_json::from_str::<serde_json::Value>(&body).unwrap();
        let prices = &json["blockPrices"][0]["estimatedPrices"][0];
        let max_fee_per_gas = (prices["maxFeePerGas"].as_f64().unwrap() * 1e9) as u128;
        let max_priority_fee_per_gas =
            (prices["maxPriorityFeePerGas"].as_f64().unwrap() * 1e9) as u128;

        let fillable = GasPriceFillable { max_fee_per_gas, max_priority_fee_per_gas };
        Ok(fillable)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Instantiate the provider with the UrgentQueue filler
    let provider =
        ProviderBuilder::new().filler(UrgentQueue::default()).connect_anvil_with_wallet();
    let bob = Address::from([0x42; 20]);
    let tx = TransactionRequest::default().with_to(bob).with_value(U256::from(1));

    let bob_balance_before = provider.get_balance(bob).await?;
    let res = provider.send_transaction(tx).await?.get_receipt().await?;
    let bob_balance_after = provider.get_balance(bob).await?;
    println!("Balance before: {bob_balance_before}\nBalance after: {bob_balance_after}");

    let tx = provider.get_transaction_by_hash(res.transaction_hash).await?.unwrap();
    println!("Max fee per gas: {:?}", tx.max_fee_per_gas());
    println!("Max priority fee per gas: {:?}", tx.max_priority_fee_per_gas());

    Ok(())
}
