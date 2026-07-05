use crate::model::dto::payment::{
    CreatePaymentMethodRequest, DeletePaymentMethodRequest,
    GetPaymentMethodListRequest, GetPaymentMethodListResponse,
    UpdatePaymentMethodRequest,
};
use crate::model::entity::payment::Payment;
use crate::repository::payment::PaymentRepo;
use anyhow::Context;
use chrono::Utc;

pub async fn get_payment_method_list(
    repo: &dyn PaymentRepo,
    req: GetPaymentMethodListRequest,
) -> anyhow::Result<GetPaymentMethodListResponse> {
    let (total, _list) = repo
        .find_list_by_page(req.page as i64, req.size as i64, None)
        .await
        .context("query payment methods")?;
    Ok(GetPaymentMethodListResponse { total, list: vec![] })
}

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

pub async fn update_payment_method(
    repo: &dyn PaymentRepo,
    req: UpdatePaymentMethodRequest,
) -> anyhow::Result<()> {
    let existing = repo.find_one(req.id).await.context("find payment method")?;
    let now = Utc::now().timestamp_millis();
    let entity = Payment {
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

pub async fn delete_payment_method(
    repo: &dyn PaymentRepo,
    req: DeletePaymentMethodRequest,
) -> anyhow::Result<()> {
    repo.delete(req.id).await.context("delete payment method")?;
    Ok(())
}

pub async fn get_payment_platform() -> anyhow::Result<Vec<String>> {
    Ok(vec!["alipay".into(), "stripe".into(), "epay".into()])
}
