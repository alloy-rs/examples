//! Demonstrates how to implement a custom transport layer that delays dispatching the requests.

use eyre::Result;
use std::{
    task::{Context, Poll},
    time::Duration,
};

use alloy::{
    network::TransactionBuilder,
    node_bindings::Anvil,
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::{client::ClientBuilder, types::TransactionRequest},
    signers::local::PrivateKeySigner,
    transports::BoxFuture,
};
use tokio::time::sleep;
use tower::{Layer, Service};

/// A [`tower::Service`] that delays the dispatch of requests by a specified duration.
#[derive(Debug, Clone)]
pub struct DelayService<S> {
    service: S,
    delay: Duration,
}

/// A [`tower::Layer`] that returns a new [`DelayService`] with the specified delay.
#[derive(Debug, Clone)]
pub struct DelayLayer {
    delay: Duration,
}

impl DelayLayer {
    /// Creates a new [`DelayLayer`] with the specified delay.
    pub const fn new(delay: Duration) -> Self {
        Self { delay }
    }
}

impl<S> Layer<S> for DelayLayer {
    type Service = DelayService<S>;

    fn layer(&self, service: S) -> Self::Service {
        DelayService { service, delay: self.delay }
    }
}

/// Implement the [`tower::Service`] trait for the [`DelayService`].
impl<S, Request> Service<Request> for DelayService<S>
where
    S: Service<Request> + Send,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let delay = self.delay;
        let future = self.service.call(req);

        Box::pin(async move {
            println!("Delaying for {} seconds...", delay.as_secs());
            sleep(delay).await;
            println!("Dispatching request...");
            future.await
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().try_spawn()?;
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();

    // Build a RPC client with the `DelayLayer`.
    let client = ClientBuilder::default()
        .layer(DelayLayer::new(Duration::from_secs(1)))
        .http(anvil.endpoint().parse()?);

    // Instatiate a provider with the RPC-client that uses the `DelayLayer`.
    let provider = ProviderBuilder::new().wallet(signer).connect_client(client);

    let bob = Address::from([0x42; 20]);
    let tx = TransactionRequest::default().with_to(bob).with_value(U256::from(1));

    let bob_balance_before = provider.get_balance(bob).await?;
    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;
    assert!(receipt.status(), "Transaction failed");
    let bob_balance_after = provider.get_balance(bob).await?;
    println!("Balance before: {bob_balance_before}\nBalance after: {bob_balance_after}");

    Ok(())
}
