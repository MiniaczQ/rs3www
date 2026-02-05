use serde::{Deserialize, Serialize};

use crate::directory::{explorer::DirectoryExplorerConfig, index::DirectoryIndexConfig};

pub mod explorer;
pub mod index;

#[derive(Serialize, Deserialize, Clone)]
pub struct DirectoryConfig {
    pub index: DirectoryIndexConfig,
    pub explorer: DirectoryExplorerConfig,
}
