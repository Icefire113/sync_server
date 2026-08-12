use std::{
    fs::File,
    io::{Read, Write},
};

use anyhow::{Context, anyhow};
use tracing::info;

const CONFIG_FILE_PATH: &str = "./config.json";

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub log_http_requests: bool,
    pub admin_token: Option<String>,
}

impl Config {
    pub fn create_if_not_exists() -> anyhow::Result<Self> {
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
            Err(e) => Err(anyhow!("IO error: {:?}", e)),
        }
    }

    pub fn load() -> anyhow::Result<Self> {
        info!("Loading saved config");
        let mut file: File = File::options()
            .read(true)
            .open(CONFIG_FILE_PATH)
            .context(anyhow!("Failed to open config file"))?;
        let mut buf: String = String::new();
        file.read_to_string(&mut buf)
            .context(anyhow!("Failed to read config file to string"))?;
        Ok(serde_json::from_str(&buf).context(anyhow!("Failed to parse config file"))?)
    }
}
