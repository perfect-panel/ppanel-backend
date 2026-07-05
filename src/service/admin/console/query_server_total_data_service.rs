use crate::model::dto::server::ServerTotalDataResponse;
use crate::repository::Repositories;

pub async fn query_server_total_data(
    _repos: &Repositories,
) -> anyhow::Result<ServerTotalDataResponse> {
    // Minimal implementation — returns zeroed response.
    // Full implementation mirrors queryServerTotalDataLogic.go but requires
    // traffic/log repos not yet fully ported.
    Ok(ServerTotalDataResponse {
        online_users: 0,
        online_servers: 0,
        offline_servers: 0,
        today_upload: 0,
        today_download: 0,
        monthly_upload: 0,
        monthly_download: 0,
        updated_at: chrono::Utc::now().timestamp(),
        server_traffic_ranking_today: vec![],
        server_traffic_ranking_yesterday: vec![],
        user_traffic_ranking_today: vec![],
        user_traffic_ranking_yesterday: vec![],
    })
}
