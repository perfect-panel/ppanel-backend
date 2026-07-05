//! Subscribe content generation.
//!
//! Port of `server/internal/logic/subscribe/subscribeLogic.go`.

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::adapter::{Adapter, Client, ClientConfig, User as AdapterUser};
use crate::config::Config;
use crate::repository::node::NodeFilter;
use crate::repository::Repositories;
use crate::service::subscribe::user_agent::detect_client;

use result::code_error::CodeError;
use result::error_code;

/// Output of a successful subscribe generation.
pub struct SubscribeOutput {
    pub content: String,
    pub content_type: String,
    pub userinfo: String,
    pub disposition: String,
}

pub struct SubscribeService {
    pub repos: Arc<Repositories>,
    pub config: Arc<Config>,
}

impl SubscribeService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>) -> Self {
        Self { repos, config }
    }

    /// Generate subscription content.
    ///
    /// `ua`    — User-Agent header
    /// `token` — subscribe token from URL path
    /// `host`  — Host header (for subscribe_url)
    /// `params`— query string params passed to the template
    pub async fn handle_subscribe(
        &self,
        ua: &str,
        token: &str,
        host: &str,
        params: HashMap<String, String>,
    ) -> Result<SubscribeOutput, anyhow::Error> {
        // 1. Load all subscribe applications (client templates).
        let clients = self
            .repos
            .client
            .list()
            .await
            .map_err(|e| anyhow!("load clients: {e}"))?;

        // 2. Detect client by UA.
        let app = detect_client(ua, &clients);

        // 3. Validate token → user subscribe.
        let user_subscribe = self
            .repos
            .user
            .find_one_subscribe_by_token(token)
            .await
            .map_err(|e| {
                if matches!(e, sqlx::Error::RowNotFound) {
                    anyhow!(CodeError::new_err_code(error_code::USER_NOT_EXIST))
                } else {
                    anyhow!("find subscribe by token: {e}")
                }
            })?;

        // 4. Load user.
        let user = self
            .repos
            .user
            .find_one_user(user_subscribe.user_id)
            .await
            .map_err(|e| anyhow!("find user: {e}"))?;

        // 5. Load subscribe plan.
        let subscribe_plan = self
            .repos
            .subscribe
            .find_one(user_subscribe.subscribe_id)
            .await
            .map_err(|e| anyhow!("find subscribe plan: {e}"))?;

        let now = Utc::now().timestamp();

        // 6. Check expiry / traffic.
        let is_expired = user_subscribe.expire_time > 0 && user_subscribe.expire_time < now;
        let used = user_subscribe.upload + user_subscribe.download;
        let is_traffic_exceeded = subscribe_plan.traffic > 0 && used >= subscribe_plan.traffic;

        // 7. Collect node ids.
        let node_ids: Vec<i64> = if is_expired || is_traffic_exceeded {
            vec![]
        } else {
            serde_json::from_str::<Vec<i64>>(&subscribe_plan.nodes).unwrap_or_default()
        };

        // 8. Fetch (Node, Server) pairs.
        let pairs = if node_ids.is_empty() {
            vec![]
        } else {
            let filter = NodeFilter {
                node_ids: node_ids.clone(),
                enabled: Some(true),
                page: 1,
                size: 10000,
                ..Default::default()
            };
            let (_, nodes) = self
                .repos
                .node
                .filter_node_list(&filter, true)
                .await
                .map_err(|e| anyhow!("filter nodes: {e}"))?;

            let mut result = Vec::with_capacity(nodes.len());
            for node in nodes {
                match self.repos.node.find_one_server(node.server_id).await {
                    Ok(server) => result.push((node, server)),
                    Err(e) => {
                        tracing::warn!(node_id = node.id, "server load failed: {e}");
                    }
                }
            }
            result
        };

        // 9. Build proxy list.
        let proxies = Adapter::proxies(&pairs);

        // 10. Template + output format.
        let (template, output_format) = match app {
            Some(a) => (
                a.subscribe_template.clone().unwrap_or_default(),
                a.output_format.clone(),
            ),
            None => (String::new(), "base64".into()),
        };

        // 11. Subscribe URL for AdapterUser.
        let scheme = if self.config.tls.enable { "https" } else { "http" };
        let subscribe_url = format!(
            "{scheme}://{host}{path}/{token}",
            path = self.config.subscribe.subscribe_path
        );

        let adapter_user = AdapterUser {
            password: user.password.clone(),
            expired_at: user_subscribe.expire_time,
            download: user_subscribe.download,
            upload: user_subscribe.upload,
            traffic: subscribe_plan.traffic,
            subscribe_url,
        };

        // 12. Render.
        let renderer = Client {
            config: ClientConfig {
                site_name: self.config.site.site_name.clone(),
                subscribe_name: subscribe_plan.name.clone(),
                output_format: output_format.clone(),
                params,
            },
        };
        let content = renderer
            .build(&template, &proxies, &adapter_user)
            .map_err(|e| anyhow!("template render: {e}"))?;

        // 13. Subscription-Userinfo header.
        let userinfo = format!(
            "upload={upload}; download={download}; total={total}; expire={expire}",
            upload = user_subscribe.upload,
            download = user_subscribe.download,
            total = subscribe_plan.traffic,
            expire = user_subscribe.expire_time,
        );

        let content_type = match output_format.as_str() {
            "json" => "application/json; charset=utf-8".into(),
            _ => "text/plain; charset=utf-8".into(),
        };

        let safe_name = subscribe_plan.name.replace('"', "");
        let disposition = format!(r#"attachment; filename="{safe_name}.yaml""#);

        Ok(SubscribeOutput { content, content_type, userinfo, disposition })
    }
}
