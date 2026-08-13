use aws_config::Region;

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
        todo!()
    }

    async fn get(&self, key: &str) -> Result<Vec<u8>, StorageError> {
        todo!()
    }

    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        todo!()
    }

    async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        todo!()
    }
}
