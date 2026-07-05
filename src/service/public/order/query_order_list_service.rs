//! `QueryOrderList` — paginated order list for the current user.
//!
//! Port of `server/internal/logic/public/order/queryOrderListLogic.go`.
//! Filters by user and strips the internal `commission` field to avoid
//! leaking referer economics to the user-facing API.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::misc::StringInt64Slice;
use crate::model::dto::order::{OrderDetail, QueryOrderListRequest, QueryOrderListResponse};
use crate::model::dto::payment::PaymentMethod;
use crate::model::dto::subscribe::{Subscribe, SubscribeDiscount};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct QueryOrderListService {
    repos: Arc<Repositories>,
}

impl QueryOrderListService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn query(
        &self,
        user_id: i64,
        req: QueryOrderListRequest,
    ) -> Result<QueryOrderListResponse, anyhow::Error> {
        let page = i64::from(req.page.max(1));
        let size = i64::from(req.size.max(1));

        // Go passes `status=0` to mean "all" and `subscribe_id=0` /
        // `search=""` to mean unfiltered. The repo signature takes
        // `Option<&str>` for the search term.
        let (total, data) = self
            .repos
            .order
            .query_list_by_page(page, size, 0, user_id, 0, None)
            .await
            .map_err(|_| {
                anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR))
            })?;

        let mut list = Vec::with_capacity(data.len());
        for item in data {
            list.push(build_detail(&item));
        }

        Ok(QueryOrderListResponse { total, list })
    }
}

/// Project the repository row into the public-facing `OrderDetail` DTO.
///
/// Mirrors Go's `tool.DeepCopy(&orderInfo, item)` followed by
/// `orderInfo.Commission = 0`. The Rust port hides the commission by
/// setting it to `None`, and populates the joined `payment` / `subscribe`
/// names from the read-side join the repository exposes.
fn build_detail(item: &crate::repository::order::OrderDetails) -> OrderDetail {
    OrderDetail {
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
        // Commission is hidden — Go zeroes it explicitly.
        commission: None,
        // Joined payment name is the only public field needed for the
        // list view; richer data is fetched on the detail endpoint.
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
    }
}
