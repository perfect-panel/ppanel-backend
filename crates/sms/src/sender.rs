#[async_trait::async_trait]
pub trait Sender: Send + Sync {
    async fn send(&self, area: &str, phone: &str, code: &str, expire: u32) -> anyhow::Result<()>;
}
