use anyhow::Context;
use chrono::Utc;

use crate::model::dto::payment::UpdatePaymentMethodRequest;
use crate::repository::payment::PaymentRepo;

pub async fn update_payment_method(
    repo: &dyn PaymentRepo,
    req: UpdatePaymentMethodRequest,
) -> anyhow::Result<()> {
    let existing = repo.find_one(req.id).await.context("find payment method")?;
    let now = Utc::now().timestamp_millis();
    let entity = crate::model::entity::payment::Payment {
        id: existing.id,
        name: req.name,
        platform: req.platform,
        description: req.description,
        icon: req.icon.unwrap_or(existing.icon),
        domain: req.domain.unwrap_or(existing.domain),
        config: req.config.to_string(),
        fee_mode: req.fee_mode as i64,
        fee_percent: req.fee_percent.unwrap_or(existing.fee_percent),
        fee_amount: req.fee_amount.unwrap_or(existing.fee_amount),
        sort: req.sort.unwrap_or(existing.sort),
        enable: req.enable.or(existing.enable),
        token: existing.token,
        created_at: existing.created_at,
        updated_at: now,
    };
    repo.update(&entity).await.context("update payment method")?;
    Ok(())
}
