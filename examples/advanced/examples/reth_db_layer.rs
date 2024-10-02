#![allow(unreachable_pub)]
use std::path::PathBuf;

/// We use the tower-like layering functionality that has been baked into the alloy-provider to
/// intercept the requests and redirect to the `RethDbProvider`.
pub struct RethDBLayer {
    db_path: PathBuf,
}

/// Initialize the `RethDBLayer` with the path to the reth datadir.
impl RethDBLayer {
    pub const fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    pub const fn db_path(&self) -> &PathBuf {
        &self.db_path
    }
}
