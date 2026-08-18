use std::fmt::Debug;

pub mod local;
pub mod s3;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Storage backend internal error: {0}")]
    Internal(String),
}

impl StorageError {
    pub fn internal<E: std::fmt::Display>(msg: &str, err: E) -> Self {
        Self::Internal(format!("{msg}: {err}"))
    }
}

/// Describes how files are stored and retrieved.
#[async_trait::async_trait]
pub trait StorageProvider: Send + Sync + Debug {
    async fn put(&self, key: &str, bytes: &[u8]) -> Result<(), StorageError>;
    async fn get(&self, key: &str) -> Result<Vec<u8>, StorageError>;
    async fn delete(&self, key: &str) -> Result<(), StorageError>;
    async fn exists(&self, key: &str) -> Result<bool, StorageError>;
}
