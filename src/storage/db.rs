use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::path::PathBuf;
use anyhow::{Result, Context};
use tokio::fs;

pub struct StorageManager {
    pub pool: Pool<Sqlite>,
}

impl StorageManager {
    pub async fn new() -> Result<Self> {
        let db_path = Self::get_db_path().await?;
        let options = SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .context("Failed to connect to SQLite database")?;

        let manager = Self { pool };
        manager.init().await?;
        Ok(manager)
    }

    async fn get_db_path() -> Result<PathBuf> {
        let home = std::env::var("HOME").context("HOME env var not set")?;
        let base_dir = PathBuf::from(home).join(".local/share/machine");
        
        if !base_dir.exists() {
            fs::create_dir_all(&base_dir).await?;
        }
        
        Ok(base_dir.join("machine.db"))
    }

    async fn init(&self) -> Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS snapshots (
                id TEXT PRIMARY KEY,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                data TEXT NOT NULL
            )"
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS investigations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                data TEXT NOT NULL
            )"
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS observations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                metric TEXT NOT NULL,
                value REAL NOT NULL
            )"
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn save_snapshot(&self, id: &str, data: &str) -> Result<()> {
        sqlx::query("INSERT INTO snapshots (id, data) VALUES (?, ?)")
            .bind(id)
            .bind(data)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_snapshot(&self, id: &str) -> Result<Option<String>> {
        let row: Option<(String,)> = sqlx::query_as("SELECT data FROM snapshots WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.0))
    }

    pub async fn list_snapshots(&self, limit: i64) -> Result<Vec<(String, String)>> {
        let rows: Vec<(String, String)> = sqlx::query_as("SELECT id, timestamp FROM snapshots ORDER BY timestamp DESC LIMIT ?")
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows)
    }
}
