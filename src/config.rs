use std::{
    fs::File,
    io::{self, Read, Write},
};

use thiserror::Error;
use tracing::info;

const CONFIG_FILE_PATH: &str = "./config.json";

#[derive(Error, Debug)]
pub enum ConfigLoadError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub log_http_requests: bool,
    pub admin_token: Option<String>,
}

impl Config {
    pub fn create_if_not_exists() -> Result<Self, ConfigLoadError> {
        match File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(CONFIG_FILE_PATH)
        {
            // config did not exist and was created
            Ok(mut file) => {
                info!("Config file not found, creating");
                let cfg = Self::default();
                let str = serde_json::to_string(&cfg)?;
                file.write(str.as_bytes())?;
                Ok(cfg)
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Self::load(),
            Err(e) => Err(io::Error::from(e).into()),
        }
    }

    pub fn load() -> Result<Self, ConfigLoadError> {
        info!("Loading saved config");
        let mut file: File = File::options().read(true).open(CONFIG_FILE_PATH)?;
        let mut buf: String = String::new();
        file.read_to_string(&mut buf)?;
        Ok(serde_json::from_str(&buf)?)
    }
}
