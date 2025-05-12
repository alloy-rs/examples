//! This example demonstrates how to write a custom layer for the [`hyper`] HTTP client that can
//! modify the underlying HTTP request before it is sent.

use alloy::{
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
    rpc::client::RpcClient,
    transports::http::{
        hyper,
        hyper_util::{
            client::legacy::{Client, Error},
            rt::TokioExecutor,
        },
        Http, HyperClient, HyperResponse, HyperResponseFut,
    },
};
use eyre::Result;
use http_body_util::Full;
use tower::{Layer, Service};

#[tokio::main]
async fn main() -> Result<()> {
    // Start an Anvil node.
    let anvil = Anvil::new().spawn();

    // Create a new Hyper client.
    let hyper_client =
        Client::builder(TokioExecutor::new()).build_http::<Full<hyper::body::Bytes>>();

    // Use tower::ServiceBuilder to stack layers on top of the Hyper client.
    let service = tower::ServiceBuilder::new().layer(RequestModifyingLayer).service(hyper_client);

    // Instantiate the HyperClient with the stacked layers.
    let layer_transport = HyperClient::<Full<hyper::body::Bytes>, _>::with_service(service);
    let http = Http::with_client(layer_transport, anvil.endpoint_url());

    // Create a new RPC client with the Hyper transport.
    let rpc_client = RpcClient::new(http, true);

    let provider = ProviderBuilder::new().connect_client(rpc_client);

    let num = provider.get_block_number().await.unwrap();

    assert_eq!(num, 0);

    Ok(())
}

// Layer that will be stacked on top of the Hyper client.
struct RequestModifyingLayer;

// Implement the `Layer` trait for the custom layer.
impl<S> Layer<S> for RequestModifyingLayer {
    type Service = RequestModifyingService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestModifyingService { inner }
    }
}

// Service that will modify the request before it is sent.
#[derive(Clone)] // Service must be Cloneable.
struct RequestModifyingService<S> {
    inner: S,
}

impl<S, B> Service<hyper::Request<B>> for RequestModifyingService<S>
where
    S: Service<hyper::Request<B>, Response = HyperResponse, Error = Error>
        + Clone
        + Send
        + Sync
        + 'static,
    S::Future: Send,
    S::Error: std::error::Error + Send + Sync + 'static,
    B: From<Vec<u8>> + Send + 'static + Clone + Sync + std::fmt::Debug,
{
    type Error = Error;
    type Future = HyperResponseFut;
    type Response = HyperResponse;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: hyper::Request<B>) -> Self::Future {
        // Modify the request here.

        // Example: Add a custom header to the request.
        let header = req.headers_mut();
        header.insert("x-alloy", "hyper".parse().unwrap());

        println!("Request: {req:?}");

        let fut = self.inner.call(req);

        Box::pin(fut)
    }
}
