use crate::model::dto::ticket::CreateTicketFollowRequest;
use crate::model::entity::ticket::Follow;
use crate::repository::ticket::TicketRepo;
use anyhow::Context;
use chrono::Utc;
use result::code_error::CodeError;
use result::error_code;

pub async fn create_ticket_follow(
    repo: &dyn TicketRepo,
    _user_id: i64,
    req: CreateTicketFollowRequest,
) -> anyhow::Result<()> {
    // Ensure the parent ticket exists so we don't insert an orphan follow.
    repo.find_one(req.ticket_id)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::USER_NOT_EXIST,
                &format!("ticket {} not found: {}", req.ticket_id, e),
            ))
        })?;

    let now = Utc::now().timestamp_millis();
    let follow = Follow {
        id: 0,
        ticket_id: req.ticket_id,
        from: req.from,
        type_: req.type_ as i16,
        content: Some(req.content),
        created_at: now,
    };
    repo.insert_follow(&follow)
        .await
        .context("create ticket follow")?;
    Ok(())
}
