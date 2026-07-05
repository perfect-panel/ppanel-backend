use axum::extract::Query;

use crate::model::dto::*;
use crate::service::admin::tool::query_ip_location_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_ip_location(
    Query(req): Query<QueryIPLocationRequest>,
) -> HttpResult {
    match query_ip_location_service::query_ip_location(req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
