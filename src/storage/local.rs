use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use crate::{
    config,
    storage::{StorageError, StorageProvider},
};

#[derive(Debug)]
pub struct LocalStorage {
    root: PathBuf,
}

impl LocalStorage {
    pub fn new(conf: &config::LocalStorageConfig) -> Self {
        Self {
            root: PathBuf::from(conf.path.clone()),
        }
    }
}

#[async_trait::async_trait]
impl StorageProvider for LocalStorage {
    async fn put(&self, key: &str, bytes: Vec<u8>) -> Result<(), StorageError> {
        let object_path = self.root.join(key);

        if let Some(parent) = object_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| StorageError::internal("Failed to create parent directories", e))?;
        }

        match File::options()
            .truncate(true)
            .write(true)
            .read(true)
            .create(true)
            .open(object_path)
        {
            Ok(mut file) => file
                .write_all(&bytes)
                .map_err(|e| StorageError::internal("Failed to write to file", e)),
            Err(e) => Err(StorageError::internal("Failed to open file", e)),
        }
    }

    async fn get(&self, key: &str) -> Result<Vec<u8>, StorageError> {
        let object_path = self.root.join(key);
        match File::options().read(true).open(object_path) {
            Ok(mut file) => {
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)
                    .map_err(|e| StorageError::internal("Failed to read file", e))?;
                Ok(bytes)
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(StorageError::NotFound),
            Err(e) => Err(StorageError::internal("Failed to open file", e)),
        }
    }

    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        let object_path = self.root.join(key);
        match fs::remove_file(object_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(StorageError::internal("Failed to delete file", e)),
        }
    }

    async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        let object_path = self.root.join(key);
        fs::exists(object_path)
            .map_err(|e| StorageError::internal("Failed to check if file exist", e))
    }
}
