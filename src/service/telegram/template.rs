//! Telegram bot message templates.

use chrono::{DateTime, TimeZone, Utc};

fn fmt_ts(ts: i64) -> String {
    match Utc.timestamp_opt(ts, 0).single() {
        Some(dt) => dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        None => "N/A".into(),
    }
}

fn fmt_bytes(bytes: i64) -> String {
    const GB: f64 = 1_073_741_824.0;
    const MB: f64 = 1_048_576.0;
    let b = bytes as f64;
    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.2} MB", b / MB)
    } else {
        format!("{} B", bytes)
    }
}

/// `/start` welcome message.
pub fn welcome(bot_name: &str) -> String {
    format!(
        "👋 Welcome to *{bot_name}*!\n\nAvailable commands:\n\
         /bind <token> — Bind your account\n\
         /traffic — Check traffic usage"
    )
}

/// Successful bind message.
pub fn bind_success(email: &str) -> String {
    format!("✅ Account *{email}* bound successfully.")
}

/// Bind failure message.
pub fn bind_failed(reason: &str) -> String {
    format!("❌ Failed to bind account: {reason}")
}

/// Traffic usage report.
pub fn traffic_info(
    email: &str,
    upload: i64,
    download: i64,
    total: i64,
    expire_time: i64,
) -> String {
    let used = upload + download;
    let remaining = if total > 0 { total - used } else { 0 };
    format!(
        "📊 *Traffic Report*\n\
         Account: {email}\n\
         Upload:    {up}\n\
         Download:  {down}\n\
         Used:      {used_s}\n\
         Remaining: {rem}\n\
         Total:     {tot}\n\
         Expires:   {exp}",
        up = fmt_bytes(upload),
        down = fmt_bytes(download),
        used_s = fmt_bytes(used),
        rem = fmt_bytes(remaining),
        tot = if total > 0 { fmt_bytes(total) } else { "Unlimited".into() },
        exp = if expire_time > 0 { fmt_ts(expire_time) } else { "Never".into() },
    )
}

/// No active subscription message.
pub fn no_subscription() -> String {
    "⚠️ No active subscription found for your account.".into()
}

/// Not bound message.
pub fn not_bound() -> String {
    "⚠️ Your Telegram account is not bound. Use /bind <token> to bind.".into()
}

/// Generic error message.
pub fn error_msg(msg: &str) -> String {
    format!("❌ Error: {msg}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_bytes_gb() {
        assert!(fmt_bytes(2_000_000_000).contains("GB"));
    }

    #[test]
    fn test_welcome_contains_bind() {
        assert!(welcome("TestBot").contains("/bind"));
    }
}
