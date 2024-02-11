pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../migrations");

mod error;
mod query;
mod update_data;

use anyhow::{Context, Ok};
pub use error::CrudError;
pub use query::query_dictionary;
use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions, PgPool};
pub use update_data::update_dictionary;

mod test_data;
pub use test_data::insert_test_data;

pub async fn establish_conn() -> anyhow::Result<PgPool> {
    let db_url: String = if cfg!(debug_assertions) {
        dotenvy::var("DATABASE_URL").context("DATABASE_URL must be set")?
    } else {
        let db_user = std::env::var("DATABASE_USER")
            .expect("Unable to find DATABASE_USER in environment variables");
        let db_password = std::env::var("DATABASE_PASSWORD")
            .expect("Unable to find DATABASE_PASSWORD in environment variables");
        const DB_NAME: &str = "genshin_dictionary";
        format!("postgresql://{db_user}:{db_password}@db:5432/{DB_NAME}")
    };

    if !sqlx::Postgres::database_exists(&db_url).await? {
        sqlx::Postgres::create_database(&db_url).await?
    }
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .context(format!("failed to connect to DATABASE_URL: {}", db_url))?;
    MIGRATOR.run(&db).await?;
    Ok(db)
}
