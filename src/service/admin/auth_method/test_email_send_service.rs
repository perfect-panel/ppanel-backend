use anyhow::Context;

use crate::config::Config;
use crate::model::dto::auth::TestEmailSendRequest;

pub async fn test_email_send(
    cfg: &Config,
    req: TestEmailSendRequest,
) -> anyhow::Result<()> {
    if !cfg.email.enable {
        anyhow::bail!("email is not enabled");
    }
    let sender = email::new_sender(
        &cfg.email.platform,
        &cfg.email.platform_config,
        &cfg.site.site_name,
    )
    .context("create email sender")?;
    sender
        .send(
            &[req.email],
            "Test Email Send",
            "This is a test email sent by ppanel.",
        )
        .await
        .context("send test email")?;
    Ok(())
}
