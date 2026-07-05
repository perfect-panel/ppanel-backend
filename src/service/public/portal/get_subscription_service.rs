//! `GetSubscription` — user's active subscription info (portal-facing).
//!
//! Returns the list of available subscribe plans filtered by show=true and
//! sell=true, suitable for the portal purchase screen.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::subscribe::{GetSubscriptionResponse, Subscribe, SubscribeDiscount};
use crate::repository::subscribe::FilterParams;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct GetSubscriptionService {
    repos: Arc<Repositories>,
}

impl GetSubscriptionService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn get(
        &self,
        language: Option<String>,
    ) -> Result<GetSubscriptionResponse, anyhow::Error> {
        let mut params = FilterParams {
            page: 1,
            size: 100,
            show: true,
            sell: true,
            language,
            ..Default::default()
        };

        let (_total, rows) = self
            .repos
            .subscribe
            .filter_list(&mut params)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;

        let list = rows
            .into_iter()
            .map(|s| {
                let discount: Vec<SubscribeDiscount> = if s.discount.is_empty() {
                    vec![]
                } else {
                    serde_json::from_str(&s.discount).unwrap_or_default()
                };
                let nodes: Vec<i64> = if s.nodes.is_empty() {
                    vec![]
                } else {
                    serde_json::from_str(&s.nodes).unwrap_or_default()
                };
                let node_tags: Vec<String> = if s.node_tags.is_empty() {
                    vec![]
                } else {
                    serde_json::from_str(&s.node_tags).unwrap_or_default()
                };
                Subscribe {
                    id: s.id,
                    name: s.name,
                    language: Some(s.language),
                    description: s.description,
                    unit_price: s.unit_price,
                    unit_time: s.unit_time,
                    discount,
                    replacement: s.replacement,
                    inventory: s.inventory,
                    traffic: s.traffic,
                    speed_limit: s.speed_limit,
                    device_limit: s.device_limit,
                    quota: s.quota,
                    nodes: crate::model::dto::misc::StringInt64Slice(nodes),
                    node_tags,
                    show: s.show,
                    sell: s.sell,
                    sort: s.sort,
                    deduction_ratio: s.deduction_ratio,
                    allow_deduction: s.allow_deduction,
                    reset_cycle: s.reset_cycle,
                    renewal_reset: s.renewal_reset,
                    show_original_price: s.show_original_price,
                    created_at: s.created_at,
                    updated_at: s.updated_at,
                }
            })
            .collect();

        Ok(GetSubscriptionResponse { list })
    }
}
