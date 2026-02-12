pub mod models;
pub mod repository;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::path::Path;
use std::str::FromStr;
pub use repository::Repository;

#[derive(Clone)]
pub struct DbManager {
    pub pool: SqlitePool,
}

impl DbManager {
    pub async fn new(db_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let db_url = format!("sqlite:{}", db_path.to_string_lossy());
        
        let opts = SqliteConnectOptions::from_str(&db_url)?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);

        let pool = SqlitePool::connect_with(opts).await?;
        
        // Ejecutar migraciones embebidas
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;

        Ok(Self { pool })
    }

    pub fn repository(&self) -> Repository {
        Repository::new(&self.pool)
    }
}