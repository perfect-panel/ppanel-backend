use serde_json;

use crate::model::dto::NodeConfig;
use crate::repository::Repositories;
use crate::service::admin::system::update_config::persist_config;

const CATEGORY: &str = "server";

/// Persist node/server configuration to the `system` table.
///
/// `dns`, `block`, and `outbound` are serialised to JSON for storage, matching
/// the Go reference which uses `tool.ConvertValueToString` for slice/struct
/// fields.
pub async fn update_node_config(
    repos: &Repositories,
    req: NodeConfig,
) -> Result<(), anyhow::Error> {
    persist_config(repos, CATEGORY, "node_secret", &req.node_secret).await?;
    persist_config(
        repos,
        CATEGORY,
        "node_pull_interval",
        &req.node_pull_interval.to_string(),
    )
    .await?;
    persist_config(
        repos,
        CATEGORY,
        "node_push_interval",
        &req.node_push_interval.to_string(),
    )
    .await?;
    persist_config(
        repos,
        CATEGORY,
        "traffic_report_threshold",
        &req.traffic_report_threshold.to_string(),
    )
    .await?;
    persist_config(repos, CATEGORY, "ip_strategy", &req.ip_strategy).await?;
    persist_config(
        repos,
        CATEGORY,
        "dns",
        &serde_json::to_string(&req.dns).unwrap_or_else(|_| "[]".to_string()),
    )
    .await?;
    persist_config(
        repos,
        CATEGORY,
        "block",
        &serde_json::to_string(&req.block).unwrap_or_else(|_| "[]".to_string()),
    )
    .await?;
    persist_config(
        repos,
        CATEGORY,
        "outbound",
        &serde_json::to_string(&req.outbound).unwrap_or_else(|_| "[]".to_string()),
    )
    .await?;
    Ok(())
}
