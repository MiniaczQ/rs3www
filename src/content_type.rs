use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Content type derivation.
/// Precendence:
/// - custom mappings,
/// - s3 response,
/// - fallback
#[derive(Serialize, Deserialize, Clone)]
pub struct ContentTypeConfig {
    /// Custom mapping based on dot delimited suffix.
    /// E.g.
    /// - `http` -> `text/http`
    /// - `png`  -> `image/png`
    /// - `pdf`  -> `application/pdf`
    extension_mapping: HashMap<String, String>,
    /// Whether to use MIME type provided by the S3 response.
    forward_s3_type: bool,
    /// Default MIME header value.
    fallback: String,
}

impl ContentTypeConfig {
    pub fn derive(&self, s3_path: impl AsRef<str>, s3_mime: Option<impl AsRef<str>>) -> String {
        if let Some(extension) = s3_path.as_ref().split('.').last() {
            if let Some(mime_type) = self.extension_mapping.get(extension) {
                return mime_type.clone();
            }
        }

        if self.forward_s3_type {
            if let Some(s3_mime) = s3_mime {
                return s3_mime.as_ref().to_owned();
            }
        }

        self.fallback.clone()
    }
}
