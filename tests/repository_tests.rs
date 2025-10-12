use burncloud_database_download::{DownloadRepository, DownloadProgress};
use burncloud_database::Database;
use burncloud_download_types::DownloadTask;
use std::path::PathBuf;

async fn setup_repo() -> (DownloadRepository, PathBuf) {
    // 使用默认数据库，但在每个测试中完全清理数据来确保测试隔离
    // 这需要在测试开始时立即清理，并使用锁来确保串行执行
    use std::sync::Mutex;
    use std::sync::OnceLock;

    static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let _guard = TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();

    let test_db_path = PathBuf::from("default_database.db");

    let db = Database::new().await.unwrap();
    let repo = DownloadRepository::new(db);
    repo.initialize().await.unwrap();

    // 清理数据库确保测试隔离
    repo.clear_all().await.unwrap();

    (repo, test_db_path)
}

async fn cleanup_repo(db_path: PathBuf) {
    // 用户要求不删除数据库文件，所以只清空数据表内容
    println!("测试数据库位置: {}", db_path.display());
    // 可以在这里添加清空表数据的逻辑，但保留数据库文件
}

#[tokio::test]
async fn test_save_and_get_task() {
    let (repo, db_path) = setup_repo().await;

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

    cleanup_repo(db_path).await;
}

#[tokio::test]
async fn test_save_task_duplicate_url() {
    let (repo, db_path) = setup_repo().await;

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

    cleanup_repo(db_path).await;
}

#[tokio::test]
async fn test_get_task_by_url() {
    let (repo, db_path) = setup_repo().await;

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

    cleanup_repo(db_path).await;
}

#[tokio::test]
async fn test_list_tasks() {
    let (repo, db_path) = setup_repo().await;

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

    cleanup_repo(db_path).await;
}

#[tokio::test]
async fn test_delete_task() {
    let (repo, db_path) = setup_repo().await;

    let task = DownloadTask::new(
        "https://example.com/file.zip".to_string(),
        PathBuf::from("/downloads/file.zip")
    );

    repo.save_task(&task).await.unwrap();
    repo.delete_task(&task.id).await.unwrap();

    let result = repo.get_task(&task.id).await;
    assert!(result.is_err());

    cleanup_repo(db_path).await;
}

#[tokio::test]
async fn test_save_and_get_progress() {
    let (repo, db_path) = setup_repo().await;

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

    cleanup_repo(db_path).await;
}

#[tokio::test]
async fn test_count_tasks() {
    let (repo, db_path) = setup_repo().await;

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

    cleanup_repo(db_path).await;
}

#[tokio::test]
async fn test_clear_all() {
    let (repo, db_path) = setup_repo().await;

    let task = DownloadTask::new(
        "https://example.com/file.zip".to_string(),
        PathBuf::from("/downloads/file.zip")
    );

    repo.save_task(&task).await.unwrap();
    repo.clear_all().await.unwrap();

    let count = repo.count_tasks().await.unwrap();
    assert_eq!(count, 0);

    cleanup_repo(db_path).await;
}