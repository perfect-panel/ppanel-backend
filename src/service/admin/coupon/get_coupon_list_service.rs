use crate::model::dto::{Coupon, GetCouponListRequest, GetCouponListResponse};
use crate::repository::coupon::CouponRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_coupon_list(
    repo: &dyn CouponRepo,
    req: GetCouponListRequest,
) -> Result<GetCouponListResponse, anyhow::Error> {
    let (total, items) = repo
        .query_list_by_page(
            req.page,
            req.size,
            req.subscribe,
            req.search.as_deref(),
        )
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?;

    let list: Vec<Coupon> = items
        .into_iter()
        .map(|e| {
            // subscribe is stored as comma-separated string of i64s
            let subscribe_vec: Vec<i64> = if e.subscribe.is_empty() {
                vec![]
            } else {
                e.subscribe
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .filter_map(|s| s.trim().parse::<i64>().ok())
                    .collect()
            };
            Coupon {
                id: e.id,
                name: e.name,
                code: e.code,
                count: e.count,
                type_: e.type_ as u8,
                discount: e.discount,
                start_time: e.start_time,
                expire_time: e.expire_time,
                user_limit: e.user_limit,
                subscribe: subscribe_vec,
                used_count: e.used_count,
                enable: e.enable.unwrap_or(false),
                created_at: e.created_at,
                updated_at: e.updated_at,
            }
        })
        .collect();

    Ok(GetCouponListResponse { total, list })
}
