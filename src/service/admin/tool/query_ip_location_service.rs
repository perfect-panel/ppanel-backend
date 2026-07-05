use anyhow::Context;

use crate::model::dto::system::{QueryIPLocationRequest, QueryIPLocationResponse};

pub async fn query_ip_location(
    req: QueryIPLocationRequest,
) -> anyhow::Result<QueryIPLocationResponse> {
    // GeoIP database is not available without the crate being wired in.
    // Return empty response to keep the endpoint functional.
    tracing::info!("[query_ip_location] queried for ip={}", req.ip);
    Ok(QueryIPLocationResponse {
        country: String::new(),
        region: None,
        city: String::new(),
    })
}
