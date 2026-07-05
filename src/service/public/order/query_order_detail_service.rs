//! `QueryOrderDetail` — fetch a single order by `order_no`.
//!
//! Port of `server/internal/logic/public/order/queryOrderDetailLogic.go`.
//! Hides the internal `commission` field from the public-facing response,
//! matching Go's `resp.Commission = 0` step.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::misc::StringInt64Slice;
use crate::model::dto::order::{OrderDetail, QueryOrderDetailRequest};
use crate::model::dto::payment::PaymentMethod;
use crate::model::dto::subscribe::{Subscribe, SubscribeDiscount};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct QueryOrderDetailService {
    repos: Arc<Repositories>,
}

impl QueryOrderDetailService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn query(
        &self,
        _user_id: i64,
        req: QueryOrderDetailRequest,
    ) -> Result<OrderDetail, anyhow::Error> {
        let item = self
            .repos
            .order
            .find_one_details_by_order_no(&req.order_no)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    anyhow!(CodeError::new_err_code(error_code::ORDER_NOT_EXIST))
                }
                _ => anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)),
            })?;

        // Go's behaviour is identical: copy the row, zero the commission.
        Ok(OrderDetail {
            id: item.id,
            user_id: item.user_id,
            order_no: item.order_no.clone(),
            type_: item.type_ as u8,
            quantity: item.quantity,
            price: item.price,
            amount: item.amount,
            gift_amount: item.gift_amount,
            discount: item.discount,
            coupon: item.coupon.clone().unwrap_or_default(),
            coupon_discount: item.coupon_discount,
            commission: None, // hidden
            payment: PaymentMethod {
                id: item.payment_id,
                name: item.payment_name.clone().unwrap_or_default(),
                platform: item.method.clone(),
                description: String::new(),
                icon: String::new(),
                fee_mode: 0,
                fee_percent: 0,
                fee_amount: 0,
                sort: 0,
            },
            method: item.method.clone(),
            fee_amount: item.fee_amount,
            trade_no: item.trade_no.clone().unwrap_or_default(),
            status: item.status as u8,
            subscribe_id: item.subscribe_id,
            subscribe: Subscribe {
                id: item.subscribe_id,
                name: item.subscribe_name.clone().unwrap_or_default(),
                language: Some(String::new()),
                description: None,
                unit_price: item.price,
                unit_time: String::new(),
                discount: Vec::<SubscribeDiscount>::new(),
                replacement: 0,
                inventory: 0,
                traffic: 0,
                speed_limit: 0,
                device_limit: 0,
                quota: 0,
                nodes: StringInt64Slice::default(),
                node_tags: Vec::new(),
                show: false,
                sell: false,
                sort: 0,
                deduction_ratio: 0,
                allow_deduction: false,
                reset_cycle: 0,
                renewal_reset: false,
                show_original_price: false,
                created_at: 0,
                updated_at: 0,
            },
            created_at: item.created_at,
            updated_at: item.updated_at,
        })
    }
}
