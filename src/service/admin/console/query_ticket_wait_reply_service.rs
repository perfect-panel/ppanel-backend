use anyhow::Context;

use crate::model::dto::ticket::TicketWaitRelpyResponse;
use crate::repository::ticket::TicketRepo;

pub async fn query_ticket_wait_reply(
    repo: &dyn TicketRepo,
) -> anyhow::Result<TicketWaitRelpyResponse> {
    let count = repo
        .query_wait_reply_total()
        .await
        .context("count waiting tickets")?;
    Ok(TicketWaitRelpyResponse { count })
}
