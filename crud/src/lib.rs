pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../migrations");

mod error;
mod query;
mod update_data;

use std::{str::FromStr, time::Duration};

use anyhow::{Context, Ok};
pub use error::CrudError;
pub use query::query_dictionary;
use sqlx::{
    migrate::MigrateDatabase,
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
pub use update_data::update_dictionary;

mod test_data;
use sqlx::ConnectOptions;
pub use test_data::insert_test_data;

pub async fn establish_conn(log_info: bool) -> anyhow::Result<PgPool> {
    let db_url: String = dotenvy::var("DATABASE_URL").context("DATABASE_URL must be set")?;

    if !sqlx::Postgres::database_exists(&db_url).await? {
        sqlx::Postgres::create_database(&db_url).await?
    }

    let db_opt = PgConnectOptions::from_str(&db_url)?
        .log_statements(if log_info {
            log::LevelFilter::Info
        } else {
            log::LevelFilter::Off
        })
        .log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(3));

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect_with(db_opt)
        .await
        .context(format!("failed to connect to DATABASE_URL: {}", db_url))?;
    MIGRATOR.run(&db).await?;
    Ok(db)
}
