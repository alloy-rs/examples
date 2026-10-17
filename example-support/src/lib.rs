//! Shared configuration helpers for the examples workspace.

use eyre::{bail, Result, WrapErr};

/// Returns a non-empty environment variable or an actionable error.
pub fn required_env(name: &str) -> Result<String> {
    let value =
        std::env::var(name).wrap_err_with(|| format!("{name} must be set for this example"))?;

    if value.trim().is_empty() {
        bail!("{name} must not be empty");
    }

    Ok(value)
}

/// Returns the JSON-RPC endpoint configured through `RPC_URL`.
pub fn rpc_url() -> Result<String> {
    required_env("RPC_URL").wrap_err("RPC_URL must contain a JSON-RPC endpoint")
}

/// Returns the WebSocket endpoint configured through `WS_URL`.
pub fn ws_url() -> Result<String> {
    required_env("WS_URL").wrap_err("WS_URL must contain a WebSocket JSON-RPC endpoint")
}

/// Returns the IPC socket path configured through `IPC_PATH`.
pub fn ipc_path() -> Result<String> {
    required_env("IPC_PATH").wrap_err("IPC_PATH must contain a JSON-RPC IPC socket path")
}

/// Returns the comma-separated JSON-RPC endpoints configured through `RPC_URLS`.
pub fn rpc_urls() -> Result<Vec<String>> {
    let value = std::env::var("RPC_URLS")
        .wrap_err("RPC_URLS must be set to comma-separated JSON-RPC endpoints for this example")?;
    let urls: Vec<_> = value
        .split(',')
        .map(str::trim)
        .filter(|url| !url.is_empty())
        .map(ToOwned::to_owned)
        .collect();

    if urls.len() < 2 {
        bail!("RPC_URLS must contain at least two JSON-RPC endpoints");
    }

    Ok(urls)
}
