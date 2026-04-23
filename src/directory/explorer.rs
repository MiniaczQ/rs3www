use axum::response::{IntoResponse, Response};
use maud::{DOCTYPE, html};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DirectoryExplorerConfig {
    /// How long is directory cached for.
    pub cache_seconds: u64,
    /// Whether to return a list of directories and files when no index file is available.
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct Directory {
    files: Vec<String>,
    directories: Vec<String>,
}

impl Directory {
    pub fn new(files: Vec<String>, directories: Vec<String>) -> Self {
        Self { files, directories }
    }
}

impl IntoResponse for Directory {
    fn into_response(self) -> Response {
        html! {
            (DOCTYPE)
            body {
                ul {
                    li { a href=".." { ".." } }
                    @for dir in &self.directories {
                        li { a href=(dir) { (dir) } }
                    }
                    @for file in &self.files {
                        li { a href=(file) { (file) } }
                    }
                }
            }
        }
        .into_response()
    }
}
