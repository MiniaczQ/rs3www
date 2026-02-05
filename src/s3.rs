use std::{convert::Infallible, pin::Pin, sync::Arc, task::Poll, time::Duration};

use aws_config::{AppName, BehaviorVersion, Region};
use aws_sdk_s3::{
    Client,
    config::{Credentials, SharedCredentialsProvider},
};
use axum::{
    body::Body,
    extract::Request,
    response::{IntoResponse, Response},
};
use hyper::{
    Method, StatusCode,
    header::{self},
};
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use tower_service::Service;
use tracing::info;

use crate::{
    adapter::StreamAdapter,
    config::AppConfig,
    content_type::ContentTypeConfig,
    directory::{DirectoryConfig, explorer::Directory},
    error::AppError,
};

#[derive(Clone)]
pub struct S3Service {
    client: S3ServiceClient,
    directory: Arc<DirectoryConfig>,
    index_cache: Cache<String, String>,
    explorer_cache: Cache<String, Directory>,
}

#[derive(Clone)]
struct S3ServiceClient {
    client: Client,
    bucket: Arc<String>,
    content_type: Arc<ContentTypeConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct S3Config {
    pub access_key: String,
    pub secret_key: String,
    pub url: String,
    pub region: String,
    pub bucket: String,
}

impl S3ServiceClient {
    async fn get_object(self, key: impl AsRef<str>) -> Result<Response<Body>, AppError> {
        let response = self
            .client
            .get_object()
            .bucket(self.bucket.as_str())
            .key(key.as_ref())
            .send()
            .await?;
        let content_type = self.content_type.derive(key, response.content_type);
        let stream = StreamAdapter::new(response.body.into_async_read());
        let body = Body::from_stream(stream);
        let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, content_type)
            .body(body)
            .unwrap();
        Ok(response)
    }

    async fn list_directory(self, key: impl AsRef<str>) -> Result<Directory, AppError> {
        let response = self
            .client
            .list_objects_v2()
            .bucket(self.bucket.as_str())
            .prefix(key.as_ref())
            .delimiter('/')
            .send()
            .await?;

        let files: Vec<_> = response
            .contents()
            .iter()
            .map(|o| {
                o.key()
                    .unwrap()
                    .strip_prefix(key.as_ref())
                    .unwrap()
                    .to_owned()
            })
            .collect();

        let directories: Vec<_> = response
            .common_prefixes()
            .iter()
            .map(|o| {
                o.prefix()
                    .unwrap()
                    .strip_prefix(key.as_ref())
                    .unwrap()
                    .to_owned()
            })
            .collect();

        if files.is_empty() && directories.is_empty() {
            return Err(AppError::NotFound);
        }

        Ok(Directory::new(files, directories))
    }
}

impl S3Service {
    pub fn new(config: &AppConfig) -> Self {
        let creds = Credentials::builder()
            .provider_name("custom")
            .access_key_id(&config.s3.access_key)
            .secret_access_key(&config.s3.secret_key)
            .build();
        let conf = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::v2025_08_07())
            .credentials_provider(SharedCredentialsProvider::new(creds))
            .endpoint_url(&config.s3.url)
            .region(Region::new(config.s3.region.clone()))
            .app_name(AppName::new("rs3www").unwrap())
            .force_path_style(true)
            .build();
        let client = aws_sdk_s3::Client::from_conf(conf);

        let index_cache = Cache::builder()
            .time_to_live(Duration::from_secs(config.directory.index.cache_seconds))
            .time_to_idle(Duration::from_secs(config.directory.index.cache_seconds) / 8)
            .max_capacity(1000)
            .build();

        let explorer_cache = Cache::builder()
            .time_to_live(Duration::from_secs(config.directory.explorer.cache_seconds))
            .time_to_idle(Duration::from_secs(config.directory.explorer.cache_seconds) / 8)
            .max_capacity(1000)
            .build();

        Self {
            client: S3ServiceClient {
                client,
                bucket: Arc::new(config.s3.bucket.clone()),
                content_type: Arc::new(config.content_type.clone()),
            },
            directory: Arc::new(config.directory.clone()),
            index_cache,
            explorer_cache,
        }
    }

    async fn call_fallible(self, req: Request) -> Result<Response<Body>, AppError> {
        // Refuse non-GET methods
        match *req.method() {
            Method::GET => {}
            _ => {
                return Err(AppError::MethodNotAllowed);
            }
        }

        let mut key = req.uri().path()[1..].to_owned();

        info!("access to: `{}`", key);

        let is_dir = key.ends_with('/');

        if !is_dir && !key.is_empty() {
            let result = self.client.clone().get_object(&key).await;
            match result {
                Err(AppError::NotFound) => {
                    key += "/";
                }
                _ => return result,
            }
        }

        info!("looking for index file");
        // Cached index file
        if let Some(index_file) = self.index_cache.get(&key).await {
            let index_file_key = key.clone() + &index_file;
            return self.client.get_object(index_file_key).await;
        }
        // Search for valid index file
        for index_file in &self.directory.index.files {
            let index_file_key = key.clone() + index_file;
            let result = self.client.clone().get_object(&index_file_key).await;
            if result.is_ok() {
                self.index_cache
                    .insert(index_file_key, index_file.clone())
                    .await;
                return result;
            }
        }

        info!("exploring the directory");
        // Cached directory explorer
        if let Some(directory) = self.explorer_cache.get(&key).await {
            return Ok(directory.into_response());
        }
        // Explore directory
        let directory = self.client.list_directory(&key).await?;
        self.explorer_cache
            .insert(key.clone(), directory.clone())
            .await;
        return Ok(directory.into_response());
    }
}

impl Service<Request> for S3Service {
    type Response = Response;

    type Error = Infallible;

    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let this = self.clone();
        Box::pin(async move {
            Ok(this
                .call_fallible(req)
                .await
                .unwrap_or_else(IntoResponse::into_response))
        })
    }
}
