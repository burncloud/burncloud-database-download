use crate::{
    error::{DownloadDbError, Result},
    models::{DownloadTaskRecord, DownloadProgressRecord},
    schema::initialize_schema,
};
use burncloud_database::Database;
use burncloud_download_types::{DownloadTask, DownloadProgress, TaskId, DownloadStatus};
use sqlx::Row;

/// 下载任务数据库仓库
pub struct DownloadRepository {
    db: Database,
}

impl DownloadRepository {
    /// 创建新的仓库实例
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// 初始化数据库schema
    pub async fn initialize(&self) -> Result<()> {
        initialize_schema(&self.db).await
    }

    /// 保存下载任务，如果相同URL已存在则返回已存在的任务
    pub async fn save_task(&self, task: &DownloadTask) -> Result<DownloadTask> {
        // 首先检查是否已存在相同URL的任务
        if let Ok(existing_task) = self.get_task_by_url(&task.url).await {
            return Ok(existing_task);
        }

        // 如果不存在，创建新任务
        let record = DownloadTaskRecord::from_task(task)?;

        sqlx::query(
            r#"
            INSERT INTO download_tasks (id, url, target_path, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                url = excluded.url,
                target_path = excluded.target_path,
                status = excluded.status,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(&record.id)
        .bind(&record.url)
        .bind(&record.target_path)
        .bind(&record.status)
        .bind(record.created_at)
        .bind(record.updated_at)
        .execute(self.db.connection()?.pool())
        .await?;

        Ok(task.clone())
    }

    /// 根据URL获取任务
    pub async fn get_task_by_url(&self, url: &str) -> Result<DownloadTask> {
        let record: DownloadTaskRecord = sqlx::query_as(
            "SELECT id, url, target_path, status, created_at, updated_at FROM download_tasks WHERE url = ? LIMIT 1"
        )
        .bind(url)
        .fetch_one(self.db.connection()?.pool())
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DownloadDbError::TaskNotFound(url.to_string()),
            e => DownloadDbError::Sqlx(e),
        })?;

        Ok(record.to_task().map_err(|e| DownloadDbError::Other(e))?)
    }

    /// 根据ID获取任务
    pub async fn get_task(&self, task_id: &TaskId) -> Result<DownloadTask> {
        let id_str = task_id.to_string();

        let record: DownloadTaskRecord = sqlx::query_as(
            "SELECT id, url, target_path, status, created_at, updated_at FROM download_tasks WHERE id = ?"
        )
        .bind(&id_str)
        .fetch_one(self.db.connection()?.pool())
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DownloadDbError::TaskNotFound(id_str),
            e => DownloadDbError::Sqlx(e),
        })?;

        Ok(record.to_task().map_err(|e| DownloadDbError::Other(e))?)
    }

    /// 获取所有任务
    pub async fn list_tasks(&self) -> Result<Vec<DownloadTask>> {
        let records: Vec<DownloadTaskRecord> = sqlx::query_as(
            "SELECT id, url, target_path, status, created_at, updated_at FROM download_tasks ORDER BY created_at DESC"
        )
        .fetch_all(self.db.connection()?.pool())
        .await?;

        records.into_iter()
            .map(|r| r.to_task().map_err(|e| DownloadDbError::Other(e)))
            .collect()
    }

    /// 根据状态筛选任务
    pub async fn list_tasks_by_status(&self, status: &DownloadStatus) -> Result<Vec<DownloadTask>> {
        let status_json = serde_json::to_string(status)?;

        let records: Vec<DownloadTaskRecord> = sqlx::query_as(
            "SELECT id, url, target_path, status, created_at, updated_at FROM download_tasks WHERE status = ? ORDER BY created_at DESC"
        )
        .bind(&status_json)
        .fetch_all(self.db.connection()?.pool())
        .await?;

        records.into_iter()
            .map(|r| r.to_task().map_err(|e| DownloadDbError::Other(e)))
            .collect()
    }

    /// 删除任务
    pub async fn delete_task(&self, task_id: &TaskId) -> Result<()> {
        let id_str = task_id.to_string();

        sqlx::query("DELETE FROM download_tasks WHERE id = ?")
            .bind(&id_str)
            .execute(self.db.connection()?.pool())
            .await?;

        Ok(())
    }

    /// 保存下载进度
    pub async fn save_progress(&self, task_id: &TaskId, progress: &DownloadProgress) -> Result<()> {
        let record = DownloadProgressRecord::from_progress(task_id, progress);

        sqlx::query(
            r#"
            INSERT INTO download_progress (task_id, downloaded_bytes, total_bytes, speed_bps, eta_seconds, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(task_id) DO UPDATE SET
                downloaded_bytes = excluded.downloaded_bytes,
                total_bytes = excluded.total_bytes,
                speed_bps = excluded.speed_bps,
                eta_seconds = excluded.eta_seconds,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(&record.task_id)
        .bind(record.downloaded_bytes)
        .bind(record.total_bytes)
        .bind(record.speed_bps)
        .bind(record.eta_seconds)
        .bind(record.updated_at)
        .execute(self.db.connection()?.pool())
        .await?;

        Ok(())
    }

    /// 获取任务进度
    pub async fn get_progress(&self, task_id: &TaskId) -> Result<DownloadProgress> {
        let id_str = task_id.to_string();

        let record: DownloadProgressRecord = sqlx::query_as(
            "SELECT task_id, downloaded_bytes, total_bytes, speed_bps, eta_seconds, updated_at FROM download_progress WHERE task_id = ?"
        )
        .bind(&id_str)
        .fetch_one(self.db.connection()?.pool())
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DownloadDbError::TaskNotFound(id_str),
            e => DownloadDbError::Sqlx(e),
        })?;

        Ok(record.to_progress())
    }

    /// 删除任务进度
    pub async fn delete_progress(&self, task_id: &TaskId) -> Result<()> {
        let id_str = task_id.to_string();

        sqlx::query("DELETE FROM download_progress WHERE task_id = ?")
            .bind(&id_str)
            .execute(self.db.connection()?.pool())
            .await?;

        Ok(())
    }

    /// 获取任务数量统计
    pub async fn count_tasks(&self) -> Result<i64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM download_tasks")
            .fetch_one(self.db.connection()?.pool())
            .await?;

        Ok(row.get("count"))
    }

    /// 获取按状态分组的任务数量
    pub async fn count_tasks_by_status(&self) -> Result<Vec<(String, i64)>> {
        let rows = sqlx::query("SELECT status, COUNT(*) as count FROM download_tasks GROUP BY status")
            .fetch_all(self.db.connection()?.pool())
            .await?;

        Ok(rows.into_iter()
            .map(|row| (row.get("status"), row.get("count")))
            .collect())
    }

    /// 清空所有数据
    pub async fn clear_all(&self) -> Result<()> {
        sqlx::query("DELETE FROM download_progress")
            .execute(self.db.connection()?.pool())
            .await?;

        sqlx::query("DELETE FROM download_tasks")
            .execute(self.db.connection()?.pool())
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use burncloud_database::create_in_memory_database;
    use std::path::PathBuf;

    async fn setup_repo() -> DownloadRepository {
        let db = create_in_memory_database().await.unwrap();
        let repo = DownloadRepository::new(db);
        repo.initialize().await.unwrap();
        repo
    }

    #[tokio::test]
    async fn test_save_and_get_task() {
        let repo = setup_repo().await;

        let task = DownloadTask::new(
            "https://example.com/file.zip".to_string(),
            PathBuf::from("/downloads/file.zip")
        );

        let saved_task = repo.save_task(&task).await.unwrap();
        assert_eq!(saved_task.id, task.id);
        assert_eq!(saved_task.url, task.url);

        let retrieved = repo.get_task(&task.id).await.unwrap();
        assert_eq!(retrieved.id, task.id);
        assert_eq!(retrieved.url, task.url);
    }

    #[tokio::test]
    async fn test_save_task_duplicate_url() {
        let repo = setup_repo().await;

        let task1 = DownloadTask::new(
            "https://example.com/file.zip".to_string(),
            PathBuf::from("/downloads/file1.zip")
        );
        let task2 = DownloadTask::new(
            "https://example.com/file.zip".to_string(), // 相同的URL
            PathBuf::from("/downloads/file2.zip")       // 不同的路径
        );

        // 保存第一个任务
        let saved_task1 = repo.save_task(&task1).await.unwrap();
        assert_eq!(saved_task1.id, task1.id);

        // 保存第二个任务（相同URL），应该返回第一个任务
        let saved_task2 = repo.save_task(&task2).await.unwrap();
        assert_eq!(saved_task2.id, task1.id); // 应该返回第一个任务的ID
        assert_eq!(saved_task2.url, task1.url);

        // 验证数据库中只有一条记录
        let count = repo.count_tasks().await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_get_task_by_url() {
        let repo = setup_repo().await;

        let task = DownloadTask::new(
            "https://example.com/file.zip".to_string(),
            PathBuf::from("/downloads/file.zip")
        );

        repo.save_task(&task).await.unwrap();

        let retrieved = repo.get_task_by_url(&task.url).await.unwrap();
        assert_eq!(retrieved.id, task.id);
        assert_eq!(retrieved.url, task.url);

        // 测试不存在的URL
        let result = repo.get_task_by_url("https://nonexistent.com/file.zip").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_tasks() {
        let repo = setup_repo().await;

        let task1 = DownloadTask::new(
            "https://example.com/file1.zip".to_string(),
            PathBuf::from("/downloads/file1.zip")
        );
        let task2 = DownloadTask::new(
            "https://example.com/file2.zip".to_string(),
            PathBuf::from("/downloads/file2.zip")
        );

        repo.save_task(&task1).await.unwrap();
        repo.save_task(&task2).await.unwrap();

        let tasks = repo.list_tasks().await.unwrap();
        assert_eq!(tasks.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_task() {
        let repo = setup_repo().await;

        let task = DownloadTask::new(
            "https://example.com/file.zip".to_string(),
            PathBuf::from("/downloads/file.zip")
        );

        repo.save_task(&task).await.unwrap();
        repo.delete_task(&task.id).await.unwrap();

        let result = repo.get_task(&task.id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_save_and_get_progress() {
        let repo = setup_repo().await;

        let task = DownloadTask::new(
            "https://example.com/file.zip".to_string(),
            PathBuf::from("/downloads/file.zip")
        );

        repo.save_task(&task).await.unwrap();

        let progress = DownloadProgress {
            downloaded_bytes: 1024,
            total_bytes: Some(10240),
            speed_bps: 512,
            eta_seconds: Some(18),
        };

        repo.save_progress(&task.id, &progress).await.unwrap();

        let retrieved = repo.get_progress(&task.id).await.unwrap();
        assert_eq!(retrieved.downloaded_bytes, progress.downloaded_bytes);
        assert_eq!(retrieved.total_bytes, progress.total_bytes);
    }

    #[tokio::test]
    async fn test_count_tasks() {
        let repo = setup_repo().await;

        let task1 = DownloadTask::new(
            "https://example.com/file1.zip".to_string(),
            PathBuf::from("/downloads/file1.zip")
        );
        let task2 = DownloadTask::new(
            "https://example.com/file2.zip".to_string(),
            PathBuf::from("/downloads/file2.zip")
        );

        repo.save_task(&task1).await.unwrap();
        repo.save_task(&task2).await.unwrap();

        let count = repo.count_tasks().await.unwrap();
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_clear_all() {
        let repo = setup_repo().await;

        let task = DownloadTask::new(
            "https://example.com/file.zip".to_string(),
            PathBuf::from("/downloads/file.zip")
        );

        repo.save_task(&task).await.unwrap();
        repo.clear_all().await.unwrap();

        let count = repo.count_tasks().await.unwrap();
        assert_eq!(count, 0);
    }
}