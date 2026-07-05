//! List nodes available for a user's active subscription.

use std::sync::Arc;

use anyhow::anyhow;

use chrono::Utc;

use crate::model::entity::node::Node;
use crate::repository::node::NodeFilter;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct QueryUserSubscribeNodeListService {
    pub repos: Arc<Repositories>,
}

impl QueryUserSubscribeNodeListService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    /// Return the enabled nodes associated with the user's active subscribe plan.
    pub async fn query_nodes(&self, user_id: i64) -> Result<Vec<Node>, anyhow::Error> {
        // Active statuses: 1=active, 2=pending
        let subscribes = self
            .repos
            .user
            .query_user_subscribe(user_id, &[1, 2])
            .await
            .map_err(|e| anyhow!("query user subscribe: {e}"))?;

        let sub = subscribes
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!(CodeError::new_err_code(error_code::USER_NOT_EXIST)))?;

        // Check expiry.
        let now = Utc::now().timestamp();
        if sub.expire_time > 0 && sub.expire_time < now {
            return Ok(vec![]);
        }

        // Load plan.
        let plan = self
            .repos
            .subscribe
            .find_one(sub.subscribe_id)
            .await
            .map_err(|e| anyhow!("find subscribe plan: {e}"))?;

        let node_ids: Vec<i64> = serde_json::from_str(&plan.nodes).unwrap_or_default();
        if node_ids.is_empty() {
            return Ok(vec![]);
        }

        let filter = NodeFilter {
            node_ids,
            enabled: Some(true),
            page: 1,
            size: 10000,
            ..Default::default()
        };

        let (_, nodes) = self
            .repos
            .node
            .filter_node_list(&filter, false)
            .await
            .map_err(|e| anyhow!("filter nodes: {e}"))?;

        Ok(nodes)
    }
}
