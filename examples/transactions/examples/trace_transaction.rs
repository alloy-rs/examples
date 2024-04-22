//! Example of how to trace a transaction using `trace_transaction`.

use alloy::{
    node_bindings::Anvil,
    primitives::b256,
    providers::{debug::DebugApi, ProviderBuilder},
    rpc::types::trace::geth::{
        GethDebugBuiltInTracerType, GethDebugTracerType, GethDebugTracingOptions,
        GethDefaultTracingOptions,
    },
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().fork("https://eth.merkle.io").try_spawn()?;

    // Create a provider.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url)?;

    // Hash of the tx we want to trace
    let hash = b256!("97a02abf405d36939e5b232a5d4ef5206980c5a6661845436058f30600c52df7");

    // Default tracing
    let default_options = GethDebugTracingOptions::default();
    let result = provider.debug_trace_transaction(hash, default_options).await?;

    println!("DEFAULT_TRACE: {result:?}");

    // Trace with built-in call tracer.
    let call_options = GethDebugTracingOptions {
        config: GethDefaultTracingOptions {
            disable_storage: Some(true),
            enable_memory: Some(false),
            ..Default::default()
        },
        tracer: Some(GethDebugTracerType::BuiltInTracer(GethDebugBuiltInTracerType::CallTracer)),
        ..Default::default()
    };
    let result = provider.debug_trace_transaction(hash, call_options).await?;

    println!("CALL_TRACE: {result:?}");

    // Trace using a custom JavaScript tracer.
    let js_options = GethDebugTracingOptions {
        tracer: Some(GethDebugTracerType::JsTracer("{data: [], fault: function(log) {}, step: function(log) { if(log.op.toString() == \"DELEGATECALL\") this.data.push(log.stack.peek(0)); }, result: function() { return this.data; }}".into())),
        ..Default::default()
    };

    let result = provider.debug_trace_transaction(hash, js_options).await?;

    println!("JS_TRACER: {result:?}");

    Ok(())
}
