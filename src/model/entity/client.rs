use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SubscribeApplication {
    pub id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub scheme: String,
    pub user_agent: String,
    pub is_default: bool,
    pub subscribe_template: Option<String>,
    pub output_format: String,
    pub download_link: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DownloadLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ios: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub android: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linux: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub harmony: Option<String>,
}
