use std::sync::Arc;
use anyhow::anyhow;

use crate::cache::Cache;
use crate::config::Config;
use crate::model::dto::server::{GetServerUserListResponse, ServerUser};
use crate::model::entity::subscribe::Subscribe;
use crate::repository::node::NodeFilter;
use crate::repository::subscribe::FilterParams;
use crate::repository::Repositories;
use crate::service::server::constant::{SERVER_CACHE_TTL_SECS, SERVER_USER_LIST_CACHE_KEY};
use crate::service::server::meta::{generate_etag, RequestMeta, ResponseMeta};
use result::code_error::CodeError;
use result::error_code;

pub async fn get_server_user_list(
    repos: Arc<Repositories>,
    config: Arc<Config>,
    cache: Arc<Cache>,
    server_id: i64,
    protocol: &str,
    meta: RequestMeta,
) -> Result<(GetServerUserListResponse, ResponseMeta), anyhow::Error> {
    let mut resp_meta = ResponseMeta::new();
    let cache_key = format!("{}{server_id}:{protocol}", SERVER_USER_LIST_CACHE_KEY);

    if let Ok(Some(cached)) = cache.get(&cache_key).await {
        if !cached.is_empty() {
            let etag = generate_etag(cached.as_bytes());
            if meta.if_none_match == etag {
                return Err(anyhow::anyhow!("304 Not Modified"));
            }
            resp_meta.set_header("ETag", &etag);
            let resp: GetServerUserListResponse = serde_json::from_str(&cached)
                .map_err(|e| anyhow!("json decode cache: {e}"))?;
            return Ok((resp, resp_meta));
        }
    }

    let server = repos
        .node
        .find_one_server(server_id)
        .await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

    let (_, nodes) = repos
        .node
        .filter_node_list(
            &NodeFilter {
                page: 1,
                size: 1000,
                server_ids: vec![server.id],
                protocol: Some(protocol.to_string()),
                ..Default::default()
            },
            false,
        )
        .await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

    let mut node_ids: Vec<i64> = Vec::new();
    let mut node_tags: Vec<String> = Vec::new();
    for n in &nodes {
        node_ids.push(n.id);
        if !n.tags.is_empty() {
            for tag in n.tags.split(',') {
                let t = tag.trim().to_string();
                if !t.is_empty() && !node_tags.contains(&t) {
                    node_tags.push(t);
                }
            }
        }
    }

    let subs = query_matched_subscribes(&repos, &node_ids, &node_tags).await?;

    if subs.is_empty() {
        let placeholder = placeholder_user(server_id, protocol, &config.node.node_secret);
        return Ok((GetServerUserListResponse { users: vec![placeholder] }, resp_meta));
    }

    let mut users: Vec<ServerUser> = Vec::new();
    for sub in &subs {
        let _ = repos.user.activate_pending_subscribes_by_subscribe_id(sub.id).await;
        let user_subs = repos
            .user
            .find_users_subscribe_by_subscribe_id(sub.id)
            .await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;
        for us in user_subs {
            users.push(ServerUser {
                id: us.user_id,
                uuid: us.uuid,
                speed_limit: sub.speed_limit,
                device_limit: sub.device_limit,
            });
        }
    }

    if users.is_empty() {
        users.push(placeholder_user(server_id, protocol, &config.node.node_secret));
    }

    let resp = GetServerUserListResponse { users };
    let encoded = serde_json::to_string(&resp).map_err(|e| anyhow!("json encode: {e}"))?;
    let etag = generate_etag(encoded.as_bytes());
    resp_meta.set_header("ETag", &etag);
    let _ = cache.set_ex(&cache_key, &encoded, SERVER_CACHE_TTL_SECS).await;

    if meta.if_none_match == etag {
        return Err(anyhow::anyhow!("304 Not Modified"));
    }

    Ok((resp, resp_meta))
}

async fn query_matched_subscribes(
    repos: &Repositories,
    node_ids: &[i64],
    node_tags: &[String],
) -> Result<Vec<Subscribe>, anyhow::Error> {
    let mut seen: std::collections::HashSet<i64> = std::collections::HashSet::new();
    let mut result: Vec<Subscribe> = Vec::new();

    if !node_ids.is_empty() {
        let mut params = FilterParams {
            page: 1,
            size: 9999,
            nodes: node_ids.to_vec(),
            ..Default::default()
        };
        let (_, subs) = repos.subscribe.filter_list(&mut params).await
            .map_err(|e| anyhow!("subscribe filter by nodes: {e}"))?;
        for s in subs {
            if seen.insert(s.id) { result.push(s); }
        }
    }

    if !node_tags.is_empty() {
        let mut params = FilterParams {
            page: 1,
            size: 9999,
            tags: node_tags.to_vec(),
            ..Default::default()
        };
        let (_, subs) = repos.subscribe.filter_list(&mut params).await
            .map_err(|e| anyhow!("subscribe filter by tags: {e}"))?;
        for s in subs {
            if seen.insert(s.id) { result.push(s); }
        }
    }

    Ok(result)
}

fn placeholder_user(server_id: i64, protocol: &str, secret: &str) -> ServerUser {
    let name = format!("ppanel:server-user-placeholder:{server_id}:{}:{secret}", protocol.trim());
    let uuid = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, name.as_bytes());
    ServerUser { id: 1, uuid: uuid.to_string(), speed_limit: 0, device_limit: 0 }
}
