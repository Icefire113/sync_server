use std::{
    fs::File,
    io::{Read, Write},
};

use anyhow::{Context, anyhow};
use serde::Deserialize;
use tracing::info;

const CONFIG_FILE_PATH: &str = "./config.json";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct LocalStorageConfig {
    /// The root directory of the local storage
    pub path: String,
}

impl Default for LocalStorageConfig {
    fn default() -> Self {
        Self {
            path: "./storage".to_string(),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct S3StorageConfig {
    pub bucket: String,
    pub region: Option<String>,
    pub endpoint_url: Option<String>,
}

impl Default for S3StorageConfig {
    fn default() -> Self {
        Self {
            bucket: "sync-server".to_string(),
            region: Default::default(),
            endpoint_url: Default::default(),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", content = "config")]
pub enum StorageBackend {
    Local(LocalStorageConfig),
    S3(S3StorageConfig),
}

impl Default for StorageBackend {
    fn default() -> Self {
        Self::Local(Default::default())
    }
}

/// A byte size that can be deserialized from a number (bytes) or a
/// human-readable string such as `"1k"`, `"1M"`, `"1024k"`, `"1g"` (case-insensitive
/// `b`/`k`/`m`/`g` suffixes; `k`=KiB, `m`=MiB, `g`=GiB).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileSize(pub u64);

impl FileSize {
    fn parse(s: &str) -> anyhow::Result<Self> {
        let s = s.trim();
        if s.is_empty() {
            return Err(anyhow!("File size string is empty"));
        }
        let (num_part, mult) = match s.chars().last() {
            Some(c) if c.is_ascii_alphabetic() => {
                let (num, suf) = s.split_at(s.len() - 1);
                let mult = match suf.to_ascii_lowercase().as_str() {
                    "b" => 1u64,
                    "k" => 1 << 10,
                    "m" => 1 << 20,
                    "g" => 1 << 30,
                    _ => return Err(anyhow!("Invalid file size suffix: {:?}", suf)),
                };
                (num.trim(), mult)
            }
            _ => (s, 1u64),
        };
        let num: u64 = num_part
            .parse()
            .map_err(|_| anyhow!("Invalid file size number: {:?}", num_part))?;
        let bytes = num
            .checked_mul(mult)
            .ok_or_else(|| anyhow!("File size overflows u64: {:?}", s))?;
        Ok(Self(bytes))
    }

    fn to_human(&self) -> String {
        const G: u64 = 1 << 30;
        const M: u64 = 1 << 20;
        const K: u64 = 1 << 10;
        let v = self.0;
        if v % G == 0 && v / G > 0 {
            format!("{}g", v / G)
        } else if v % M == 0 && v / M > 0 {
            format!("{}m", v / M)
        } else if v % K == 0 && v / K > 0 {
            format!("{}k", v / K)
        } else {
            v.to_string()
        }
    }
}

impl Default for FileSize {
    /// Defaults to 512KiB
    fn default() -> Self {
        Self(524288)
    }
}

impl std::fmt::Display for FileSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_human())
    }
}

impl<'de> serde::Deserialize<'de> for FileSize {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <String as Deserialize>::deserialize(deserializer)?;
        Self::parse(&s).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for FileSize {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_human())
    }
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub log_http_requests: bool,
    pub max_file_size: FileSize,
    pub storage_backend: StorageBackend,
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
                file.write_all(str.as_bytes())?;
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
        let cfg: Self =
            serde_json::from_str(&buf).context(anyhow!("Failed to parse config file"))?;
        Ok(cfg)
    }
}
