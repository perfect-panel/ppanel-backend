use anyhow::Context;

use crate::model::dto::payment::DeletePaymentMethodRequest;
use crate::repository::payment::PaymentRepo;

pub async fn delete_payment_method(
    repo: &dyn PaymentRepo,
    req: DeletePaymentMethodRequest,
) -> anyhow::Result<()> {
    repo.delete(req.id).await.context("delete payment method")?;
    Ok(())
}
