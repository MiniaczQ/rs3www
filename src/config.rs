use std::net::IpAddr;

use config::{ConfigBuilder, Environment, File, FileFormat, builder::DefaultState};
use serde::{Deserialize, Serialize};

use crate::{content_type::ContentTypeConfig, directory::DirectoryConfig, s3::S3Config};

pub fn config() -> AppConfig {
    let config = ConfigBuilder::<DefaultState>::default()
        .add_source(File::new("config.toml", FileFormat::Toml).required(false))
        .add_source(Environment::default().separator("__"));
    config.build().unwrap().try_deserialize().unwrap()
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub service: ServiceConfig,
    pub s3: S3Config,
    pub content_type: ContentTypeConfig,
    pub directory: DirectoryConfig,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceConfig {
    pub ip: IpAddr,
    pub port: u16,
}
