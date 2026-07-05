use crate::model::dto::ticket::UpdateTicketStatusRequest;
use crate::repository::ticket::TicketRepo;
use anyhow::Context;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_ticket_status(
    repo: &dyn TicketRepo,
    req: UpdateTicketStatusRequest,
) -> anyhow::Result<()> {
    let Some(status) = req.status else {
        // No status transition requested — treat as a no-op (matches Go
        // behaviour where a nil/0 status short-circuits the update).
        return Ok(());
    };

    // Verify the ticket still exists so the caller gets a meaningful error
    // instead of a silent "0 rows affected" response.
    repo.find_one(req.id).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::USER_NOT_EXIST,
            &format!("ticket {} not found: {}", req.id, e),
        ))
    })?;

    repo.update_ticket_status(req.id, 0, status as i16)
        .await
        .context("update ticket status")?;
    Ok(())
}
