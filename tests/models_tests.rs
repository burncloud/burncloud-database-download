use burncloud_database_download::models::*;
use burncloud_download_types::{DownloadTask, DownloadProgress, TaskId};
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