use result::code_error::CodeError;
use result::error_code;

use crate::model::dto::{NodeConfig, NodeDNS, NodeOutbound};
use crate::model::entity::system::System;
use crate::repository::Repositories;

/// Read node/server configuration from the `system` table (category = "server").
pub async fn get_node_config(
    repos: &Repositories,
) -> Result<NodeConfig, anyhow::Error> {
    let rows = repos
        .system
        .get_node_config()
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })?;
    let mut resp = NodeConfig {
        node_secret: String::new(),
        node_pull_interval: 60,
        node_push_interval: 60,
        traffic_report_threshold: 0,
        ip_strategy: String::new(),
        dns: Vec::new(),
        block: Vec::new(),
        outbound: Vec::new(),
    };
    for row in &rows {
        apply_node_row(&mut resp, row);
    }
    Ok(resp)
}

fn apply_node_row(c: &mut NodeConfig, row: &System) {
    match row.key.as_str() {
        "node_secret" => c.node_secret = row.value.clone(),
        "node_pull_interval" => {
            if let Ok(v) = row.value.parse() {
                c.node_pull_interval = v;
            }
        }
        "node_push_interval" => {
            if let Ok(v) = row.value.parse() {
                c.node_push_interval = v;
            }
        }
        "traffic_report_threshold" => {
            if let Ok(v) = row.value.parse() {
                c.traffic_report_threshold = v;
            }
        }
        "ip_strategy" => c.ip_strategy = row.value.clone(),
        "dns" => {
            if let Ok(v) = serde_json::from_str::<Vec<NodeDNS>>(&row.value) {
                c.dns = v;
            }
        }
        "block" => {
            if let Ok(v) = serde_json::from_str::<Vec<String>>(&row.value) {
                c.block = v;
            }
        }
        "outbound" => {
            if let Ok(v) = serde_json::from_str::<Vec<NodeOutbound>>(&row.value) {
                c.outbound = v;
            }
        }
        _ => {}
    }
}
