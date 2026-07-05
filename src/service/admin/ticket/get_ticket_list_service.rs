use crate::model::dto::ticket::{
    CreateTicketFollowRequest, GetTicketListRequest, GetTicketListResponse,
    GetTicketRequest, Ticket, UpdateTicketStatusRequest,
};
use crate::repository::ticket::TicketRepo;
use anyhow::Context;

pub async fn get_ticket_list(
    repo: &dyn TicketRepo,
    req: GetTicketListRequest,
) -> anyhow::Result<GetTicketListResponse> {
    let (total, list) = repo
        .query_ticket_list(req.page, req.size, 0, req.status.map(|s| s as i16), req.search.as_deref())
        .await
        .context("query ticket list")?;
    let tickets = list.into_iter().map(|t| Ticket {
        id: t.id,
        title: t.title,
        description: t.description.unwrap_or_default(),
        user_id: t.user_id,
        follow: None,
        status: t.status as u8,
        created_at: t.created_at,
        updated_at: t.updated_at,
    }).collect();
    Ok(GetTicketListResponse { total, list: tickets })
}

pub async fn get_ticket(
    repo: &dyn TicketRepo,
    req: GetTicketRequest,
) -> anyhow::Result<Ticket> {
    let t = repo.find_one(req.id).await.context("get ticket")?;
    Ok(Ticket {
        id: t.id,
        title: t.title,
        description: t.description.unwrap_or_default(),
        user_id: t.user_id,
        follow: None,
        status: t.status as u8,
        created_at: t.created_at,
        updated_at: t.updated_at,
    })
}

pub async fn create_ticket_follow(
    repo: &dyn TicketRepo,
    _user_id: i64,
    req: CreateTicketFollowRequest,
) -> anyhow::Result<()> {
    use crate::model::entity::ticket::Follow;
    use chrono::Utc;
    let now = Utc::now().timestamp_millis();
    let follow = Follow {
        id: 0,
        ticket_id: req.ticket_id,
        from: req.from,
        type_: req.type_ as i16,
        content: Some(req.content),
        created_at: now,
    };
    repo.insert_follow(&follow).await.context("create ticket follow")?;
    Ok(())
}

pub async fn update_ticket_status(
    repo: &dyn TicketRepo,
    req: UpdateTicketStatusRequest,
) -> anyhow::Result<()> {
    if let Some(status) = req.status {
        repo.update_ticket_status(req.id, 0, status as i16)
            .await
            .context("update ticket status")?;
    }
    Ok(())
}
