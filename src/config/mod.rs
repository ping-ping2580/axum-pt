mod server;
mod database;

use std::sync::LazyLock;
use anyhow::Context;
use config::{Config, FileFormat};
use serde::Deserialize;
pub use server::ServerConfig;
pub use database::DatabaseConfig;


static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::load().expect("Failed to load config"));
#[derive(Debug, Deserialize)]
pub struct AppConfig
{
    server: ServerConfig,
    database: DatabaseConfig,
}

impl AppConfig
{
    pub fn load() -> anyhow::Result<Self>
    {
        Config::builder()
            .add_source(
                config::File::with_name("application")
                    .format(FileFormat::Yaml)
                    .required(true)
            )
            .add_source(
                config::Environment::with_prefix("App")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(",")
            )
            .build()
            .with_context(|| anyhow::anyhow!("Couldn't load config"))?
            .try_deserialize()
            .with_context(|| anyhow::anyhow!("Couldn't deserialize config"))
    }

    pub fn server(&self) -> &ServerConfig
    {
        &self.server
    }

    pub fn database(&self) -> &DatabaseConfig
    {
        &self.database
    }
}

pub fn get() -> &'static AppConfig
{
    &CONFIG
}
