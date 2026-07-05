use crate::model::dto::order::{GetOrderListRequest, GetOrderListResponse, Order};
use crate::model::dto::payment::PaymentMethod;
use crate::repository::order::OrderRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_order_list(
    repo: &dyn OrderRepo,
    req: GetOrderListRequest,
) -> Result<GetOrderListResponse, anyhow::Error> {
    let status = req.status.map(|s| s as i16).unwrap_or(0);
    let user_id = req.user_id.unwrap_or(0);
    let subscribe_id = req.subscribe_id.unwrap_or(0);

    let (total, items) = repo
        .query_list_by_page(
            req.page,
            req.size,
            status,
            user_id,
            subscribe_id,
            req.search.as_deref(),
        )
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?;

    let list = items
        .into_iter()
        .map(|d| Order {
            id: d.id,
            user_id: d.user_id,
            order_no: d.order_no,
            type_: d.type_ as u8,
            quantity: d.quantity,
            price: d.price,
            amount: d.amount,
            gift_amount: d.gift_amount,
            discount: d.discount,
            coupon: d.coupon.unwrap_or_default(),
            coupon_discount: d.coupon_discount,
            commission: Some(d.commission),
            payment: PaymentMethod {
                id: d.payment_id,
                name: d.payment_name.unwrap_or_default(),
                platform: d.method,
                description: String::new(),
                icon: String::new(),
                fee_mode: 0,
                fee_percent: 0,
                fee_amount: d.fee_amount,
                sort: 0,
            },
            fee_amount: d.fee_amount,
            trade_no: d.trade_no.unwrap_or_default(),
            status: d.status as u8,
            subscribe_id: d.subscribe_id,
            created_at: d.created_at,
            updated_at: d.updated_at,
        })
        .collect();

    Ok(GetOrderListResponse { total, list })
}
