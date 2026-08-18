#[cfg(feature = "log-to-file")]
use std::{fs::File, sync::Mutex};

use std::fmt::Display;

use anyhow::Context;
use rand::{RngExt, SeedableRng, rng, rngs::StdRng};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, layer::SubscriberExt};

/// The number of bytes to generate for secure random strings (api keys, admin tokens, etc)
const SECURE_RANDOM_SIZE: usize = 32;

/// Gets a random string of 64 hex digits (32 bytes) generated from a cryptographically secure random number generator ([StdRng])
pub fn get_random_string_s() -> String {
    let mut buff: [u8; SECURE_RANDOM_SIZE] = [0u8; _];
    StdRng::from_rng(&mut rng()).fill(&mut buff);

    format!("{}", LowerCaseHexSlice(&buff))
}

#[allow(unused)]
struct LowerCaseHexSlice<'a>(&'a [u8]);
impl Display for LowerCaseHexSlice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

#[allow(unused)]
struct UpperCaseHexSlice<'a>(&'a [u8]);
impl Display for UpperCaseHexSlice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.0 {
            write!(f, "{:02X}", b)?;
        }
        Ok(())
    }
}

pub fn init_logging() -> anyhow::Result<()> {
    #[cfg(feature = "log-to-file")]
    let log_file = File::create("sync_server.log").context("Creating log files")?;

    #[cfg(debug_assertions)]
    let default_filter = format!(
        "error,{}=trace,tower_http=debug,axum::rejection=trace",
        env!("CARGO_CRATE_NAME")
    );
    #[cfg(not(debug_assertions))]
    let default_filter = format!(
        "error,{}=info,tower_http=debug,axum::rejection=trace",
        env!("CARGO_CRATE_NAME")
    );

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));

    let registry = tracing_subscriber::registry().with(env_filter);

    #[cfg(feature = "log-to-file")]
    let file_layer = fmt::layer()
        .with_file(true)
        .with_target(false)
        .with_line_number(true)
        .with_ansi(false)
        .with_writer(Mutex::new(log_file));

    let stderr_layer = fmt::layer()
        .with_file(true)
        .with_ansi(true)
        .with_line_number(true)
        .with_target(false)
        .with_writer(std::io::stderr);

    #[cfg(feature = "log-to-file")]
    let registry = registry.with(file_layer);

    let registry = registry.with(stderr_layer);
    registry.try_init().context("init tracing")?;

    Ok(())
}
