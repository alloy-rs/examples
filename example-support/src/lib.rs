//! Shared configuration helpers for the examples workspace.

use eyre::{bail, Result, WrapErr};

/// Returns the JSON-RPC endpoint configured through `RPC_URL`.
pub fn rpc_url() -> Result<String> {
    let url = std::env::var("RPC_URL")
        .wrap_err("RPC_URL must be set to a JSON-RPC endpoint for this example")?;

    if url.trim().is_empty() {
        bail!("RPC_URL must not be empty");
    }

    Ok(url)
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
