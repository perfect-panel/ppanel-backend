use crate::model::dto::ticket::{Follow, GetTicketRequest, Ticket};
use crate::repository::ticket::TicketRepo;
use anyhow::Context;

pub async fn get_ticket(
    repo: &dyn TicketRepo,
    req: GetTicketRequest,
) -> anyhow::Result<Ticket> {
    let t = repo.find_one(req.id).await.context("get ticket")?;
    let follows = repo
        .find_follows_by_ticket(t.id)
        .await
        .context("load ticket follows")?;
    let follow_dtos: Vec<Follow> = follows
        .into_iter()
        .map(|f| Follow {
            id: f.id,
            ticket_id: f.ticket_id,
            from: f.from,
            type_: f.type_ as u8,
            content: f.content.unwrap_or_default(),
            created_at: f.created_at,
        })
        .collect();
    Ok(Ticket {
        id: t.id,
        title: t.title,
        description: t.description.unwrap_or_default(),
        user_id: t.user_id,
        follow: Some(follow_dtos),
        status: t.status as u8,
        created_at: t.created_at,
        updated_at: t.updated_at,
    })
}
