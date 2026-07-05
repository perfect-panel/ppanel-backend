use crate::config::Config;

pub async fn setting_telegram_bot(config: &Config) -> Result<(), anyhow::Error> {
    let token = &config.telegram.bot_token;

    if token.is_empty() {
        tracing::info!("[setting_telegram_bot] bot_token not configured, skipping");
        return Ok(());
    }

    let webhook_domain = &config.telegram.web_hook_domain;

    if webhook_domain.is_empty() {
        tracing::info!("[setting_telegram_bot] no webhook domain configured, skipping setWebhook");
        return Ok(());
    }

    let secret = format!("{:x}", md5::compute(token.as_bytes()));
    let webhook_url = format!(
        "{}/v1/telegram/webhook?secret={}",
        webhook_domain.trim_end_matches('/'),
        secret
    );

    let client = reqwest::Client::new();
    let api_url = format!("https://api.telegram.org/bot{}/setWebhook", token);

    let resp = client
        .post(&api_url)
        .json(&serde_json::json!({ "url": webhook_url }))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("setWebhook request failed: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!(
            "setWebhook returned {status}: {body}"
        ));
    }

    tracing::info!("[setting_telegram_bot] webhook registered: {webhook_url}");
    Ok(())
}
