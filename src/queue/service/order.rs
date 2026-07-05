use std::sync::Arc;

use anyhow::{anyhow, Context};
use chrono::{Datelike, Months, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::Config;
use crate::model::entity::log::{BALANCE_TYPE_RECHARGE, COMMISSION_TYPE_PURCHASE, COMMISSION_TYPE_RENEWAL, RESET_SUBSCRIBE_TYPE_PAID};
use crate::model::entity::order::Order;
use crate::model::entity::subscribe::Subscribe;
use crate::model::entity::user::{User, UserSubscribe};
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;

const ORDER_TYPE_SUBSCRIBE: i16 = 1;
const ORDER_TYPE_RENEWAL: i16 = 2;
const ORDER_TYPE_RESET_TRAFFIC: i16 = 3;
const ORDER_TYPE_RECHARGE: i16 = 4;

const ORDER_STATUS_UNPAID: i16 = 1;
const ORDER_STATUS_PAID: i16 = 2;
const ORDER_STATUS_CANCELLED: i16 = 3;

const USER_SUBSCRIBE_STATUS_ACTIVE: i16 = 1;

#[derive(Debug, Clone, Deserialize)]
pub struct OrderTaskPayload {
    pub order_id: i64,
}

pub struct ActivateOrderLogic {
    repos: Arc<Repositories>,
    config: Arc<Config>,
}

impl ActivateOrderLogic {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>) -> Self {
        Self { repos, config }
    }

    pub async fn execute(&self, payload: OrderTaskPayload) -> anyhow::Result<()> {
        let mut order = self.find_order(payload.order_id).await?;
        if order.status != ORDER_STATUS_UNPAID {
            return Ok(());
        }

        match order.type_ {
            ORDER_TYPE_SUBSCRIBE => self.activate_new_subscription(&order).await?,
            ORDER_TYPE_RENEWAL => self.activate_renewal(&order).await?,
            ORDER_TYPE_RESET_TRAFFIC => self.activate_traffic_reset(&order).await?,
            ORDER_TYPE_RECHARGE => self.activate_balance_recharge(&order).await?,
            other => return Err(anyhow!("invalid order type: {other}")),
        }

        order.status = ORDER_STATUS_PAID;
        order.updated_at = now_ms();
        self.repos.order.update(&order).await?;
        Ok(())
    }

    async fn find_order(&self, order_id: i64) -> anyhow::Result<Order> {
        self.repos
            .order
            .find_one(order_id)
            .await
            .with_context(|| format!("find order {order_id}"))
    }

    async fn activate_new_subscription(&self, order: &Order) -> anyhow::Result<()> {
        let user = self.repos.user.find_one_user(order.user_id).await?;
        let plan = self.repos.subscribe.find_one(order.subscribe_id).await?;
        let user_subscribe = self.create_user_subscribe(order, &plan).await?;

        Telemetry::subscribe_access(&self.repos, user_subscribe.id, &user_subscribe.token, "", "").await;
        self.apply_commission(&user, order, COMMISSION_TYPE_PURCHASE).await?;
        Ok(())
    }

    async fn activate_renewal(&self, order: &Order) -> anyhow::Result<()> {
        let user = self.repos.user.find_one_user(order.user_id).await?;
        let plan = self.repos.subscribe.find_one(order.subscribe_id).await?;
        let token = order
            .subscribe_token
            .as_deref()
            .context("renewal order missing subscribe_token")?;
        let mut user_subscribe = self.repos.user.find_one_subscribe_by_token(token).await?;
        let now = Utc::now();
        let base_time = if user_subscribe.expire_time < now.timestamp_millis() {
            now
        } else {
            datetime_from_ms(user_subscribe.expire_time)
        };

        if plan.renewal_reset || should_reset_for_renewal(user_subscribe.expire_time, now) {
            user_subscribe.download = 0;
            user_subscribe.upload = 0;
        }
        user_subscribe.expire_time = add_time(&plan.unit_time, order.quantity, base_time).timestamp_millis();
        user_subscribe.traffic = plan.traffic;
        user_subscribe.finished_at = None;
        user_subscribe.status = USER_SUBSCRIBE_STATUS_ACTIVE;
        user_subscribe.updated_at = now.timestamp_millis();

        let updated = self.repos.user.update_subscribe(&user_subscribe).await?;
        Telemetry::subscribe_access(&self.repos, updated.id, &updated.token, "", "").await;
        self.apply_commission(&user, order, COMMISSION_TYPE_RENEWAL).await?;
        Ok(())
    }

    async fn activate_traffic_reset(&self, order: &Order) -> anyhow::Result<()> {
        let token = order
            .subscribe_token
            .as_deref()
            .context("traffic reset order missing subscribe_token")?;
        let mut user_subscribe = self.repos.user.find_one_subscribe_by_token(token).await?;
        user_subscribe.download = 0;
        user_subscribe.upload = 0;
        user_subscribe.status = USER_SUBSCRIBE_STATUS_ACTIVE;
        user_subscribe.updated_at = now_ms();
        self.repos.user.update_subscribe(&user_subscribe).await?;

        Telemetry::reset_subscribe(
            &self.repos,
            order.user_id,
            RESET_SUBSCRIBE_TYPE_PAID,
            Some(order.order_no.clone()),
        )
        .await;
        Ok(())
    }

    async fn activate_balance_recharge(&self, order: &Order) -> anyhow::Result<()> {
        let mut user = self.repos.user.find_one_user(order.user_id).await?;
        user.balance += order.amount;
        user.updated_at = now_ms();
        let updated = self.repos.user.update_user(&user).await?;
        Telemetry::balance(
            &self.repos,
            updated.id,
            BALANCE_TYPE_RECHARGE,
            order.amount,
            Some(order.order_no.clone()),
            updated.balance,
        )
        .await;
        Ok(())
    }

    async fn create_user_subscribe(&self, order: &Order, plan: &Subscribe) -> anyhow::Result<UserSubscribe> {
        if plan.quota > 0 {
            let current = self
                .repos
                .user
                .count_user_subscribes_by_user_and_subscribe(order.user_id, order.subscribe_id)
                .await?;
            if current >= plan.quota {
                return Err(anyhow!("subscribe quota limit exceeded"));
            }
        }

        let now = Utc::now();
        let user_subscribe = UserSubscribe {
            id: 0,
            user_id: order.user_id,
            order_id: order.id,
            subscribe_id: order.subscribe_id,
            start_time: now.timestamp_millis(),
            expire_time: add_time(&plan.unit_time, order.quantity, now).timestamp_millis(),
            finished_at: None,
            traffic: plan.traffic,
            download: 0,
            upload: 0,
            token: format!("Order-{}-{}", order.order_no, Uuid::new_v4()),
            uuid: Uuid::new_v4().to_string(),
            status: USER_SUBSCRIBE_STATUS_ACTIVE,
            note: String::new(),
            created_at: now.timestamp_millis(),
            updated_at: now.timestamp_millis(),
        };

        self.repos.user.insert_subscribe(&user_subscribe).await.map_err(Into::into)
    }

    async fn apply_commission(&self, user: &User, order: &Order, commission_type: i32) -> anyhow::Result<()> {
        if user.referer_id == 0 {
            return Ok(());
        }

        let mut referer = self.repos.user.find_one_user(user.referer_id).await?;
        let referral_percentage = if referer.referral_percentage > 0 {
            i64::from(referer.referral_percentage)
        } else {
            self.config.invite.referral_percentage
        };
        if referral_percentage == 0 {
            return Ok(());
        }

        let only_first_purchase = if referer.referral_percentage > 0 {
            referer.only_first_purchase
        } else {
            self.config.invite.only_first_purchase
        };
        if only_first_purchase && !order.is_new {
            return Ok(());
        }

        let commission_base = order.amount - order.fee_amount;
        let commission = commission_base * referral_percentage / 100;
        referer.commission += commission;
        referer.updated_at = now_ms();
        self.repos.user.update_user(&referer).await?;
        Telemetry::commission(&self.repos, referer.id, commission_type, commission, &order.order_no).await;
        Ok(())
    }
}

pub struct DeferCloseOrderLogic {
    repos: Arc<Repositories>,
}

impl DeferCloseOrderLogic {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn execute(&self, payload: OrderTaskPayload) -> anyhow::Result<()> {
        let order = self.repos.order.find_one(payload.order_id).await;
        let mut order = match order {
            Ok(order) => order,
            Err(sqlx::Error::RowNotFound) => return Ok(()),
            Err(err) => return Err(err.into()),
        };

        if order.status == ORDER_STATUS_UNPAID {
            order.status = ORDER_STATUS_CANCELLED;
            order.updated_at = now_ms();
            self.repos.order.update(&order).await?;
        }

        Ok(())
    }
}

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn datetime_from_ms(timestamp_ms: i64) -> chrono::DateTime<Utc> {
    match chrono::DateTime::<Utc>::from_timestamp_millis(timestamp_ms) {
        Some(datetime) => datetime,
        None => Utc::now(),
    }
}

fn add_time(unit: &str, amount: i64, from: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    match unit {
        "hour" => from + chrono::Duration::hours(amount),
        "day" => from + chrono::Duration::days(amount),
        "week" => from + chrono::Duration::weeks(amount),
        "month" => add_months(from, amount),
        "year" => add_months(from, amount.saturating_mul(12)),
        _ => from + chrono::Duration::days(amount),
    }
}

fn add_months(from: chrono::DateTime<Utc>, amount: i64) -> chrono::DateTime<Utc> {
    if amount <= 0 {
        return from;
    }

    let months = match u32::try_from(amount) {
        Ok(value) => value,
        Err(_) => u32::MAX,
    };
    match from.checked_add_months(Months::new(months)) {
        Some(datetime) => datetime,
        None => from,
    }
}

fn should_reset_for_renewal(expire_time_ms: i64, now: chrono::DateTime<Utc>) -> bool {
    datetime_from_ms(expire_time_ms).day() == now.day()
}
