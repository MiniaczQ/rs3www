use serde::{Deserialize, Serialize};

/// Default file for a directory.
#[derive(Serialize, Deserialize, Clone)]
pub struct DirectoryIndexConfig {
    /// How long to wait before retrying a missing file.
    pub cache_seconds: u64,
    /// Default file to show when in a directory.
    /// Returns the first file found.
    pub files: Vec<String>,
}
