use anyhow::Context;
use chrono::Utc;

use crate::model::dto::user::{UserStatistics, UserStatisticsResponse};
use crate::repository::order::OrderRepo;
use crate::repository::user::UserRepo;

pub async fn query_user_statistics(
    user_repo: &dyn UserRepo,
    order_repo: &dyn OrderRepo,
) -> anyhow::Result<UserStatisticsResponse> {
    let now = Utc::now().timestamp_millis();

    // today
    let today_register = user_repo
        .query_register_user_total_by_date(now)
        .await
        .unwrap_or(0);
    let (today_new, today_renewal) = order_repo
        .query_date_user_counts(now)
        .await
        .unwrap_or((0, 0));

    // monthly
    let monthly_register = user_repo
        .query_register_user_total_by_monthly(now)
        .await
        .unwrap_or(0);
    let (monthly_new, monthly_renewal) = order_repo
        .query_monthly_user_counts(now)
        .await
        .unwrap_or((0, 0));
    let monthly_list_raw = user_repo
        .query_daily_user_statistics_list(now)
        .await
        .unwrap_or_default();
    let monthly_list: Vec<UserStatistics> = monthly_list_raw
        .into_iter()
        .map(|d| UserStatistics {
            date: Some(d.date),
            register: d.register,
            new_order_users: d.new_order_users,
            renewal_order_users: d.renewal_order_users,
            list: None,
        })
        .collect();

    // all-time
    let all_register = user_repo
        .query_register_user_total()
        .await
        .unwrap_or(0);
    let (all_new, all_renewal) = order_repo
        .query_total_user_counts()
        .await
        .unwrap_or((0, 0));
    let all_list_raw = user_repo
        .query_monthly_user_statistics_list(now)
        .await
        .unwrap_or_default();
    let all_list: Vec<UserStatistics> = all_list_raw
        .into_iter()
        .map(|d| UserStatistics {
            date: Some(d.date),
            register: d.register,
            new_order_users: d.new_order_users,
            renewal_order_users: d.renewal_order_users,
            list: None,
        })
        .collect();

    Ok(UserStatisticsResponse {
        today: UserStatistics {
            date: None,
            register: today_register,
            new_order_users: today_new,
            renewal_order_users: today_renewal,
            list: None,
        },
        monthly: UserStatistics {
            date: None,
            register: monthly_register,
            new_order_users: monthly_new,
            renewal_order_users: monthly_renewal,
            list: Some(monthly_list),
        },
        all: UserStatistics {
            date: None,
            register: all_register,
            new_order_users: all_new,
            renewal_order_users: all_renewal,
            list: Some(all_list),
        },
    })
}
