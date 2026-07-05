use std::sync::Arc;
use crate::cache::Cache;
use crate::config::Config;
use crate::model::dto::server::OnlineUsersRequest;
use crate::repository::Repositories;

pub async fn push_online_users(
    _repos: Arc<Repositories>,
    _config: Arc<Config>,
    cache: Arc<Cache>,
    req: OnlineUsersRequest,
) -> anyhow::Result<()> {
    let key = format!("node:online:{}", req.common.server_id);
    let user_ids: Vec<i64> = req.users.iter().map(|u| u.sid).collect();
    let value = serde_json::to_string(&user_ids)?;
    cache.set_ex(&key, &value, 120).await?;
    Ok(())
}
