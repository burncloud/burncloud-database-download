use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use burncloud_download_types::{DownloadTask, DownloadStatus, DownloadProgress, TaskId};

/// 下载任务数据库记录
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DownloadTaskRecord {
    /// 任务ID (UUID字符串)
    pub id: String,
    /// 下载URL
    pub url: String,
    /// 目标文件路径
    pub target_path: String,
    /// 任务状态 (JSON序列化)
    pub status: String,
    /// 创建时间 (Unix timestamp)
    pub created_at: i64,
    /// 更新时间 (Unix timestamp)
    pub updated_at: i64,
}

impl DownloadTaskRecord {
    /// 从 DownloadTask 创建数据库记录
    pub fn from_task(task: &DownloadTask) -> serde_json::Result<Self> {
        let now = chrono::Utc::now().timestamp();

        Ok(Self {
            id: task.id.to_string(),
            url: task.url.clone(),
            target_path: task.target_path.to_string_lossy().to_string(),
            status: serde_json::to_string(&task.status)?,
            created_at: now,
            updated_at: now,
        })
    }

    /// 转换为 DownloadTask
    pub fn to_task(&self) -> Result<DownloadTask, String> {
        let status: DownloadStatus = serde_json::from_str(&self.status)
            .map_err(|e| format!("Failed to parse status: {}", e))?;

        let task_id = TaskId::from_string(&self.id)
            .map_err(|e| format!("Invalid task ID: {}", e))?;

        use std::time::UNIX_EPOCH;
        let created = UNIX_EPOCH + std::time::Duration::from_secs(self.created_at as u64);
        let updated = UNIX_EPOCH + std::time::Duration::from_secs(self.updated_at as u64);

        Ok(DownloadTask {
            id: task_id,
            url: self.url.clone(),
            target_path: self.target_path.clone().into(),
            status,
            created_at: created,
            updated_at: updated,
        })
    }
}

/// 下载进度数据库记录
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DownloadProgressRecord {
    /// 任务ID (UUID字符串)
    pub task_id: String,
    /// 已下载字节数
    pub downloaded_bytes: i64,
    /// 总字节数 (可选)
    pub total_bytes: Option<i64>,
    /// 下载速度 (bytes/second)
    pub speed_bps: i64,
    /// 预计剩余时间 (秒, 可选)
    pub eta_seconds: Option<i64>,
    /// 更新时间 (Unix timestamp)
    pub updated_at: i64,
}

impl DownloadProgressRecord {
    /// 从 DownloadProgress 创建数据库记录
    pub fn from_progress(task_id: &TaskId, progress: &DownloadProgress) -> Self {
        let now = chrono::Utc::now().timestamp();

        Self {
            task_id: task_id.to_string(),
            downloaded_bytes: progress.downloaded_bytes as i64,
            total_bytes: progress.total_bytes.map(|b| b as i64),
            speed_bps: progress.speed_bps as i64,
            eta_seconds: progress.eta_seconds.map(|s| s as i64),
            updated_at: now,
        }
    }

    /// 转换为 DownloadProgress
    pub fn to_progress(&self) -> DownloadProgress {
        DownloadProgress {
            downloaded_bytes: self.downloaded_bytes as u64,
            total_bytes: self.total_bytes.map(|b| b as u64),
            speed_bps: self.speed_bps as u64,
            eta_seconds: self.eta_seconds.map(|s| s as u64),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_download_task_record_roundtrip() {
        let task = DownloadTask::new(
            "https://example.com/file.zip".to_string(),
            PathBuf::from("/downloads/file.zip")
        );

        let record = DownloadTaskRecord::from_task(&task).unwrap();
        assert_eq!(record.id, task.id.to_string());
        assert_eq!(record.url, task.url);

        let restored_task = record.to_task().unwrap();
        assert_eq!(restored_task.id, task.id);
        assert_eq!(restored_task.url, task.url);
    }

    #[test]
    fn test_download_progress_record_roundtrip() {
        let task_id = TaskId::new();
        let progress = DownloadProgress {
            downloaded_bytes: 1024,
            total_bytes: Some(10240),
            speed_bps: 512,
            eta_seconds: Some(18),
        };

        let record = DownloadProgressRecord::from_progress(&task_id, &progress);
        assert_eq!(record.task_id, task_id.to_string());
        assert_eq!(record.downloaded_bytes, 1024);

        let restored_progress = record.to_progress();
        assert_eq!(restored_progress.downloaded_bytes, progress.downloaded_bytes);
        assert_eq!(restored_progress.total_bytes, progress.total_bytes);
    }
}