//! A custom transport layer that implements retry and backoff functionality into the client
use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::{
        client::ClientBuilder,
        json_rpc::{RequestPacket, ResponsePacket},
    },
    transports::TransportError,
};
use eyre::Result;
use std::{
    future::{self, Future, IntoFuture},
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context, Poll},
};
use tower::{retry::Policy, Layer, Service};

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://mainnet.infura.io/v3/your-api-key".parse()?; // Infura reqs/s set to 1, for testing.
    let client = ClientBuilder::default().layer(RetryLayer::new(RetryPolicy::new(3))).http(url);

    let provider = ProviderBuilder::new().on_client(client);

    for _ in 0..10 {
        let block_number = provider.get_block_number().into_future().await?;
        println!("block_number: {:?}", block_number);
    }

    Ok(())
}

/// RetryPolicy
#[derive(Debug)]
struct RetryPolicy {
    max_retries: AtomicU64,
}

impl RetryPolicy {
    const fn new(max_retries: u64) -> Self {
        Self { max_retries: AtomicU64::new(max_retries) }
    }
}

impl Clone for RetryPolicy {
    fn clone(&self) -> Self {
        let max_retries = self.max_retries.load(Ordering::Relaxed);
        Self { max_retries: AtomicU64::new(max_retries) }
    }
}

impl Policy<RequestPacket, ResponsePacket, TransportError> for RetryPolicy {
    type Future = Pin<Box<dyn Future<Output = Self> + Send + 'static>>;

    fn retry(
        &self,
        req: &RequestPacket,
        result: Result<&ResponsePacket, &TransportError>,
    ) -> Option<Self::Future> {
        // Retry on any error for testing.
        // TODO: Use rate-limit specific errors/codes and retry accordingly.
        if result.is_err() {
            println!("error: {:#?}", result.unwrap_err());
            // Retry: Return Some()
            let max_retries = self.max_retries.load(Ordering::Relaxed);
            if max_retries > 0 {
                self.max_retries.store(max_retries - 1, Ordering::Relaxed);
                if let RequestPacket::Single(req) = req {
                    println!("retrying request {:?}", req.meta().method);
                }
                Some(Box::pin(future::ready(self.clone())))
            } else {
                println!("max_retries exhausted, giving up retrying request: {:#?}", req);
                // TODO: Add backoff interval here.
                None
            }
        } else {
            println!("success, no retry");
            None
        }
    }

    fn clone_request(&self, req: &RequestPacket) -> Option<RequestPacket> {
        Some(req.clone())
    }
}

/// RetryLayer
struct RetryLayer {
    policy: RetryPolicy,
}

impl RetryLayer {
    const fn new(policy: RetryPolicy) -> Self {
        Self { policy }
    }
}

impl<S> Layer<S> for RetryLayer {
    type Service = RetryService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RetryService { inner, policy: self.policy.clone() }
    }
}

/// RetryService
#[derive(Debug, Clone)]
struct RetryService<S> {
    inner: S,
    policy: RetryPolicy,
}
impl<S> Service<RequestPacket> for RetryService<S>
where
    S: Service<RequestPacket, Response = ResponsePacket, Error = TransportError>
        + Send
        + 'static
        + Clone,
    S::Future: Send + 'static,
{
    type Response = ResponsePacket;
    type Error = TransportError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: RequestPacket) -> Self::Future {
        // TODO: Implement retry logic

        let inner = self.inner.clone();
        let policy = self.policy.clone();

        let mut inner = std::mem::replace(&mut self.inner, inner);
        Box::pin(async move {
            let mut retries = 0;

            let mut res = inner.call(req.clone()).await;

            while let Some(_policy) = policy.retry(&req, res.as_ref()) {
                retries += 1;
                println!("Retry attempt: {}", retries);
                res = inner.call(req.clone()).await;
            }
            res
        })
    }
}
