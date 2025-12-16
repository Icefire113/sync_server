pub async fn version() -> String {
    format!(
        "[config_share] sync_server version: v{}",
        env!("CARGO_PKG_VERSION")
    )
}
