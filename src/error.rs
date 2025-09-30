use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadDbError {
    #[error("Database error: {0}")]
    Database(#[from] burncloud_database_core::DatabaseError),

    #[error("SQLx error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Task not found: {0}")]
    TaskNotFound(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid status: {0}")]
    InvalidStatus(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, DownloadDbError>;