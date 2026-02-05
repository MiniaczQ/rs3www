use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DirectoryExplorerConfig {
    /// How long is directory cached for.
    pub cache_seconds: u64,
    /// Whether to return a list of directories and files when no index file is available.
    pub enabled: bool,
}

#[derive(Template, Serialize, Debug, Clone)]
#[template(path = "explorer.html")]
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
        let html = self.render().unwrap();
        Html(html).into_response()
    }
}
