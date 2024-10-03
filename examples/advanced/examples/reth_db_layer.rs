//! `RethDbLayer` implementation to be used with `RethDbProvider` to wrap the Provider trait over
//! reth-db.
#![allow(dead_code)]
use std::path::PathBuf;

/// We use the tower-like layering functionality that has been baked into the alloy-provider to
/// intercept the requests and redirect to the `RethDbProvider`.
pub(crate) struct RethDbLayer {
    db_path: PathBuf,
}

/// Initialize the `RethDBLayer` with the path to the reth datadir.
impl RethDbLayer {
    pub(crate) const fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    pub(crate) const fn db_path(&self) -> &PathBuf {
        &self.db_path
    }
}

const fn main() {}
