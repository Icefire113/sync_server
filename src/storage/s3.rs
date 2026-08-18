use aws_config::Region;
use aws_sdk_s3::primitives::ByteStream;
use tracing::error;

use crate::{
    config,
    storage::{StorageError, StorageProvider},
};

#[derive(Debug)]
pub struct S3Storage {
    bucket: String,
    client: aws_sdk_s3::Client,
}

impl S3Storage {
    pub async fn new(conf: &config::S3StorageConfig) -> Self {
        // unwraps here are ok as we
        let sdk_config: aws_config::SdkConfig = aws_config::from_env()
            .endpoint_url(conf.endpoint_url.clone().unwrap())
            .region(Region::new(conf.region.clone().unwrap()))
            .load()
            .await;

        let s3_config = aws_sdk_s3::config::Builder::from(&sdk_config)
            .force_path_style(true)
            .build();

        let s3_client = aws_sdk_s3::Client::from_conf(s3_config);
        Self {
            bucket: conf.bucket.clone(),
            client: s3_client,
        }
    }
}

#[async_trait::async_trait]
impl StorageProvider for S3Storage {
    async fn put(&self, key: &str, bytes: &[u8]) -> Result<(), StorageError> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(bytes.to_vec()))
            .send()
            .await
            .map_err(|e| {
                error!("Error putting object into s3 {:?}", e);
                StorageError::internal("Failed to put object into s3", e)
            })?;
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Vec<u8>, StorageError> {
        let r = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                error!("Error getting object from s3 {:?}", e);
                StorageError::internal("Failed to get object from s3", e)
            })?;

        let bytes = r
            .body
            .collect()
            .await
            .map_err(|e| StorageError::internal("Failed to read object body from s3", e))?
            .into_bytes();
        Ok(bytes.to_vec())
    }

    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                error!("Error deleting object from s3 {:?}", e);
                StorageError::internal("Failed to delete object from s3", e)
            })?;
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        match self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => match e.as_service_error() {
                Some(se) if se.is_not_found() => Ok(false),
                _ => {
                    error!("Error checking object in s3 {:?}", e);
                    Err(StorageError::internal("Failed to check object in s3", e))
                }
            },
        }
    }
}
