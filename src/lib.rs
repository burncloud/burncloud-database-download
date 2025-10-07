//! burncloud-database-download
//!
//! 下载任务数据库持久化层,负责将下载任务数据存储到 SQLite 数据库中。

pub mod error;
pub mod models;
pub mod repository;
pub mod schema;

pub use error::{DownloadDbError, Result};
pub use models::{DownloadTaskRecord, DownloadProgressRecord};
pub use repository::DownloadRepository;

// Re-export core dependencies for convenience
pub use burncloud_database::{Database, DatabaseConnection, Result as DbCoreResult};
pub use burncloud_download_types::{TaskId, DownloadStatus, DownloadProgress, DownloadTask};