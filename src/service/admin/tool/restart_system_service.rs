pub async fn restart_system() -> anyhow::Result<()> {
    // In Go this calls svcCtx.Restart() which sends SIGTERM to self.
    // In Rust we log the request; actual restart is an ops concern.
    tracing::info!("[restart_system] restart requested by admin");
    Ok(())
}
