/// SQL schema for download tasks and progress tracking

/// Create download_tasks table
pub const CREATE_DOWNLOAD_TASKS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS download_tasks (
    id TEXT PRIMARY KEY NOT NULL,
    url TEXT NOT NULL,
    target_path TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
"#;

/// Create download_progress table
pub const CREATE_DOWNLOAD_PROGRESS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS download_progress (
    task_id TEXT PRIMARY KEY NOT NULL,
    downloaded_bytes INTEGER NOT NULL DEFAULT 0,
    total_bytes INTEGER,
    speed_bps INTEGER NOT NULL DEFAULT 0,
    eta_seconds INTEGER,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (task_id) REFERENCES download_tasks(id) ON DELETE CASCADE
);
"#;

/// Create indexes for better query performance
pub const CREATE_INDEXES: &str = r#"
CREATE INDEX IF NOT EXISTS idx_download_tasks_status ON download_tasks(status);
CREATE INDEX IF NOT EXISTS idx_download_tasks_created_at ON download_tasks(created_at);
CREATE INDEX IF NOT EXISTS idx_download_tasks_updated_at ON download_tasks(updated_at);
"#;

/// Initialize all tables
pub async fn initialize_schema(db: &burncloud_database_core::Database) -> crate::Result<()> {
    db.execute_query(CREATE_DOWNLOAD_TASKS_TABLE).await?;
    db.execute_query(CREATE_DOWNLOAD_PROGRESS_TABLE).await?;
    db.execute_query(CREATE_INDEXES).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_syntax() {
        // Basic syntax validation - these should not panic when parsing
        assert!(CREATE_DOWNLOAD_TASKS_TABLE.contains("CREATE TABLE"));
        assert!(CREATE_DOWNLOAD_PROGRESS_TABLE.contains("CREATE TABLE"));
        assert!(CREATE_INDEXES.contains("CREATE INDEX"));
    }

    #[tokio::test]
    async fn test_schema_initialization() {
        use burncloud_database_core::create_in_memory_database;

        let db = create_in_memory_database().await.unwrap();
        let result = initialize_schema(&db).await;
        assert!(result.is_ok());
    }
}