pub async fn get_sms_platform() -> anyhow::Result<Vec<String>> {
    // SMS crate is not yet created (per AGENTS.md). Return known platforms.
    Ok(vec![
        "aliyun".to_string(),
        "tencent".to_string(),
        "twilio".to_string(),
        "abosend".to_string(),
        "smsbao".to_string(),
    ])
}
