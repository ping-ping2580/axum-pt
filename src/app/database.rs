use super::config;
// use anyhow::anyhow;
// use sea_orm::prelude::*;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};
use std::cmp::max;
use std::time::Duration;

pub async fn init() -> anyhow::Result<DatabaseConnection>
{
    let database_config = config::get().database();
    let mut options = ConnectOptions::new(
        format!("postgres://{}:{}@{}:{}/{}",
                database_config.user(),
                database_config.password(),
                database_config.host(),
                database_config.port(),
                database_config.database(),
                // database_config.schema(),
        )
    );

    let cpus = num_cpus::get() as u32;
    options.min_connections(max(cpus * 4, 10))
        .max_connections(max(cpus * 4, 20))
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(3600 * 24))
        .sqlx_logging(false)
        .set_schema_search_path(database_config.schema());

    let db = Database::connect(options).await?;
    db.ping().await?;
    tracing::info!("database connection established");

    log_database_version(&db).await?;

    Ok(db)
}

async fn log_database_version(db: &DatabaseConnection) -> anyhow::Result<()>
{
    let version_result = db
        .query_one(Statement::from_string(
            DbBackend::Postgres,
            String::from("SELECT version()"),
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("Database version not found"))?;

    tracing::info!(
        "database version: {}",
        version_result.try_get_by_index::<String>(0)?
    );

    Ok(())
}