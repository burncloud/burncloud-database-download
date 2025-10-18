use burncloud_database::{Database, Result};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Download {
    pub gid: String,
    pub status: String,
    pub uris: String,
    pub total_length: i64,
    pub completed_length: i64,
    pub download_speed: i64,
    pub download_dir: Option<String>,
    pub filename: Option<String>,
    pub connections: i32,
    pub split: i32,
    pub created_at: String,
    pub updated_at: String,
}

pub struct DownloadDB {
    db: Database,
}

impl DownloadDB {
    pub async fn new() -> Result<Self> {
        let db = Database::new().await?;
        let instance = Self { db };
        instance.init_tables().await?;
        Ok(instance)
    }

    async fn init_tables(&self) -> Result<()> {
        self.db.execute_query("
            CREATE TABLE IF NOT EXISTS downloads (
                gid TEXT PRIMARY KEY,
                status TEXT NOT NULL DEFAULT 'waiting',
                uris TEXT NOT NULL,
                total_length INTEGER DEFAULT 0,
                completed_length INTEGER DEFAULT 0,
                download_speed INTEGER DEFAULT 0,
                download_dir TEXT,
                filename TEXT,
                connections INTEGER DEFAULT 16,
                split INTEGER DEFAULT 5,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            CREATE INDEX IF NOT EXISTS idx_downloads_status ON downloads(status);
        ").await?;
        Ok(())
    }

    pub async fn add(&self, gid: &str, uris: Vec<String>, download_dir: Option<&str>, filename: Option<&str>) -> Result<()> {
        if self.get(gid).await?.is_some() {
            return Ok(());
        }

        let uris_json = serde_json::to_string(&uris).unwrap();

        self.db.execute_query_with_params(
            "INSERT INTO downloads (gid, uris, download_dir, filename) VALUES (?, ?, ?, ?)",
            vec![
                gid.to_string(),
                uris_json,
                download_dir.unwrap_or("").to_string(),
                filename.unwrap_or("").to_string()
            ]
        ).await?;
        Ok(())
    }

    pub async fn update_progress(&self, gid: &str, completed: i64, speed: i64) -> Result<()> {
        self.db.execute_query_with_params(
            "UPDATE downloads SET completed_length = ?, download_speed = ?, updated_at = CURRENT_TIMESTAMP WHERE gid = ?",
            vec![completed.to_string(), speed.to_string(), gid.to_string()]
        ).await?;
        Ok(())
    }

    pub async fn update_status(&self, gid: &str, status: &str) -> Result<()> {
        self.db.execute_query_with_params(
            "UPDATE downloads SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE gid = ?",
            vec![status.to_string(), gid.to_string()]
        ).await?;
        Ok(())
    }

    pub async fn get(&self, gid: &str) -> Result<Option<Download>> {
        self.db.fetch_optional::<Download>(
            &format!("SELECT * FROM downloads WHERE gid = '{}'", gid)
        ).await
    }

    pub async fn list(&self, status: Option<&str>) -> Result<Vec<Download>> {
        let query = match status {
            Some(s) => format!("SELECT * FROM downloads WHERE status = '{}' ORDER BY created_at DESC", s),
            None => "SELECT * FROM downloads ORDER BY created_at DESC".to_string(),
        };
        self.db.fetch_all::<Download>(&query).await
    }

    pub async fn delete(&self, gid: &str) -> Result<()> {
        self.db.execute_query_with_params(
            "DELETE FROM downloads WHERE gid = ?",
            vec![gid.to_string()]
        ).await?;
        Ok(())
    }
}