use anyhow::Context;
use chrono::Utc;

use crate::model::dto::order::{OrdersStatistics, RevenueStatisticsResponse};
use crate::repository::order::OrderRepo;

pub async fn query_revenue_statistics(
    order_repo: &dyn OrderRepo,
) -> anyhow::Result<RevenueStatisticsResponse> {
    let now = Utc::now().timestamp_millis();

    let today_data = order_repo
        .query_date_orders(now)
        .await
        .context("query today orders")?;
    let today = OrdersStatistics {
        date: None,
        amount_total: today_data.amount_total,
        new_order_amount: today_data.new_order_amount,
        renewal_order_amount: today_data.renewal_order_amount,
        list: None,
    };

    let monthly_data = order_repo
        .query_monthly_orders(now)
        .await
        .context("query monthly orders")?;
    let monthly_list_raw = order_repo
        .query_daily_orders_list(now)
        .await
        .unwrap_or_default();
    let monthly_list: Vec<OrdersStatistics> = monthly_list_raw
        .into_iter()
        .map(|d| OrdersStatistics {
            date: Some(d.date),
            amount_total: d.amount_total,
            new_order_amount: d.new_order_amount,
            renewal_order_amount: d.renewal_order_amount,
            list: None,
        })
        .collect();
    let monthly = OrdersStatistics {
        date: None,
        amount_total: monthly_data.amount_total,
        new_order_amount: monthly_data.new_order_amount,
        renewal_order_amount: monthly_data.renewal_order_amount,
        list: Some(monthly_list),
    };

    let all_data = order_repo
        .query_total_orders()
        .await
        .context("query total orders")?;
    let all_list_raw = order_repo
        .query_monthly_orders_list(now)
        .await
        .unwrap_or_default();
    let all_list: Vec<OrdersStatistics> = all_list_raw
        .into_iter()
        .map(|d| OrdersStatistics {
            date: Some(d.date),
            amount_total: d.amount_total,
            new_order_amount: d.new_order_amount,
            renewal_order_amount: d.renewal_order_amount,
            list: None,
        })
        .collect();
    let all = OrdersStatistics {
        date: None,
        amount_total: all_data.amount_total,
        new_order_amount: all_data.new_order_amount,
        renewal_order_amount: all_data.renewal_order_amount,
        list: Some(all_list),
    };

    Ok(RevenueStatisticsResponse { today, monthly, all })
}
