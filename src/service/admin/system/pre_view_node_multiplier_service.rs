use crate::model::dto::PreViewNodeMultiplierResponse;

/// Return a preview of the current node-multiplier ratio.
///
/// The Go reference uses a `NodeMultiplierManager` scheduler; in this Rust
/// rewrite we don't yet have that scheduler, so this returns a constant 1.0
/// ratio. Wiring the live multiplier is tracked separately.
pub async fn pre_view_node_multiplier(
    _repos: &crate::repository::Repositories,
) -> Result<PreViewNodeMultiplierResponse, anyhow::Error> {
    let now = chrono::Utc::now();
    Ok(PreViewNodeMultiplierResponse {
        current_time: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        ratio: 1.0,
    })
}
