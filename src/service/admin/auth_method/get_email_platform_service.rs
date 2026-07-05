use email::get_supported_platforms;

pub async fn get_email_platform() -> anyhow::Result<Vec<String>> {
    let platforms = get_supported_platforms();
    Ok(platforms.iter().map(|p| p.platform.clone()).collect())
}
