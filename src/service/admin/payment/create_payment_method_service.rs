use anyhow::Context;
use chrono::Utc;

use crate::model::dto::payment::CreatePaymentMethodRequest;
use crate::model::entity::payment::Payment;
use crate::repository::payment::PaymentRepo;

pub async fn create_payment_method(
    repo: &dyn PaymentRepo,
    req: CreatePaymentMethodRequest,
) -> anyhow::Result<()> {
    let now = Utc::now().timestamp_millis();
    let entity = Payment {
        id: 0,
        name: req.name,
        platform: req.platform,
        description: req.description,
        icon: req.icon.unwrap_or_default(),
        domain: req.domain.unwrap_or_default(),
        config: req.config.to_string(),
        fee_mode: req.fee_mode as i64,
        fee_percent: req.fee_percent.unwrap_or(0),
        fee_amount: req.fee_amount.unwrap_or(0),
        sort: req.sort.unwrap_or(0),
        enable: req.enable,
        token: String::new(),
        created_at: now,
        updated_at: now,
    };
    repo.insert(&entity).await.context("create payment method")?;
    Ok(())
}
