use crate::model::entity::user::{
    AuthMethods, Device, DeviceOnlineRecord, User, UserSubscribe, Withdrawal,
};

#[derive(Debug, Default)]
pub struct UserFilter {
    pub search: Option<String>,
    pub user_id: Option<i64>,
    pub subscribe_id: Option<i64>,
    pub user_subscribe_id: Option<i64>,
    pub order: Option<String>,
    pub unscoped: bool,
}

#[derive(Debug, Default)]
pub struct SubscribeFilter {
    pub subscribers: Vec<i64>,
    pub is_active: Option<bool>,
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(Debug, Default)]
pub struct EmailRecipientFilter {
    pub scope: i16,
    pub register_start_time: i64,
    pub register_end_time: i64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserStatisticsWithDate {
    pub date: String,
    pub register: i64,
    pub new_order_users: i64,
    pub renewal_order_users: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct SubscribeDetails {
    pub id: i64,
    pub user_id: i64,
    pub order_id: i64,
    pub subscribe_id: i64,
    pub subscribe_name: Option<String>,
    pub start_time: i64,
    pub expire_time: i64,
    pub finished_at: Option<i64>,
    pub traffic: i64,
    pub download: i64,
    pub upload: i64,
    pub token: String,
    pub uuid: String,
    pub status: i16,
    pub note: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[async_trait::async_trait]
pub trait UserRepo: Send + Sync {
    async fn insert_user(&self, data: &User) -> Result<User, sqlx::Error>;
    async fn find_one_user(&self, id: i64) -> Result<User, sqlx::Error>;
    async fn update_user(&self, data: &User) -> Result<User, sqlx::Error>;
    async fn delete_user(&self, id: i64) -> Result<u64, sqlx::Error>;
    
    // NOTE: 与 Go 版本的差异 - 关联数据加载
    // Go: QueryPageList 会自动 Preload("UserDevices").Preload("AuthMethods")
    // Rust: 当前实现不包含关联数据，只返回 User 主表数据
    // 如需关联数据，请在 handler 层手动查询，或使用专门的查询方法
    async fn query_page_list(
        &self,
        page: i64,
        size: i64,
        filter: &UserFilter,
    ) -> Result<(i64, Vec<User>), sqlx::Error>;

    async fn insert_subscribe(&self, data: &UserSubscribe) -> Result<UserSubscribe, sqlx::Error>;
    async fn find_one_subscribe(&self, id: i64) -> Result<UserSubscribe, sqlx::Error>;
    async fn find_one_subscribe_by_token(
        &self,
        token: &str,
    ) -> Result<UserSubscribe, sqlx::Error>;
    async fn find_one_subscribe_by_order_id(
        &self,
        order_id: i64,
    ) -> Result<UserSubscribe, sqlx::Error>;
    async fn find_one_subscribe_details_by_id(
        &self,
        id: i64,
    ) -> Result<SubscribeDetails, sqlx::Error>;
    async fn find_one_user_subscribe(&self, id: i64) -> Result<SubscribeDetails, sqlx::Error>;
    async fn update_subscribe(&self, data: &UserSubscribe) -> Result<UserSubscribe, sqlx::Error>;
    async fn delete_subscribe(&self, token: &str) -> Result<u64, sqlx::Error>;
    async fn delete_subscribe_by_id(&self, id: i64) -> Result<u64, sqlx::Error>;

    async fn insert_auth_method(&self, data: &AuthMethods) -> Result<AuthMethods, sqlx::Error>;
    async fn find_user_auth_methods(
        &self,
        user_id: i64,
    ) -> Result<Vec<AuthMethods>, sqlx::Error>;
    async fn update_auth_method(&self, data: &AuthMethods) -> Result<AuthMethods, sqlx::Error>;
    async fn delete_user_auth_methods(
        &self,
        user_id: i64,
        platform: &str,
    ) -> Result<u64, sqlx::Error>;
    async fn delete_user_auth_method_by_identifier(
        &self,
        auth_type: &str,
        identifier: &str,
    ) -> Result<u64, sqlx::Error>;
    async fn find_auth_method_by_open_id(
        &self,
        method: &str,
        open_id: &str,
    ) -> Result<Option<AuthMethods>, sqlx::Error>;
    async fn find_auth_method_by_user_id(
        &self,
        method: &str,
        user_id: i64,
    ) -> Result<Option<AuthMethods>, sqlx::Error>;
    async fn find_auth_method_by_platform(
        &self,
        user_id: i64,
        platform: &str,
    ) -> Result<Option<AuthMethods>, sqlx::Error>;
    async fn update_user_auth_method_owner(
        &self,
        auth_type: &str,
        identifier: &str,
        user_id: i64,
    ) -> Result<u64, sqlx::Error>;
    async fn upsert_user_auth_method(
        &self,
        data: &AuthMethods,
    ) -> Result<AuthMethods, sqlx::Error>;

    async fn insert_device(&self, data: &Device) -> Result<Device, sqlx::Error>;
    async fn find_one_device(&self, id: i64) -> Result<Device, sqlx::Error>;
    async fn find_one_device_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<Device>, sqlx::Error>;
    async fn update_device(&self, data: &Device) -> Result<Device, sqlx::Error>;
    async fn delete_device(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn query_device_list(
        &self,
        user_id: i64,
    ) -> Result<(Vec<Device>, i64), sqlx::Error>;
    async fn query_device_page_list(
        &self,
        user_id: i64,
        _subscribe_id: i64,
        page: i64,
        size: i64,
    ) -> Result<(Vec<Device>, i64), sqlx::Error>;

    async fn insert_device_online_record(
        &self,
        data: &DeviceOnlineRecord,
    ) -> Result<DeviceOnlineRecord, sqlx::Error>;
    async fn find_device_online_record(
        &self,
        user_id: i64,
        start_time: &str,
        end_time: &str,
    ) -> Result<Option<DeviceOnlineRecord>, sqlx::Error>;

    async fn insert_withdrawal(&self, data: &Withdrawal) -> Result<Withdrawal, sqlx::Error>;

    async fn find_users_by_ids(&self, ids: &[i64]) -> Result<Vec<User>, sqlx::Error>;
    async fn find_one_by_refer_code(&self, refer_code: &str) -> Result<Option<User>, sqlx::Error>;
    async fn find_one_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
    async fn batch_delete_users(&self, ids: &[i64]) -> Result<u64, sqlx::Error>;
    async fn count_affiliates(&self, referer_id: i64) -> Result<i64, sqlx::Error>;
    async fn query_affiliate_list(
        &self,
        referer_id: i64,
        page: i64,
        size: i64,
    ) -> Result<(i64, Vec<User>), sqlx::Error>;

    async fn find_subscribes_by_ids(
        &self,
        ids: &[i64],
    ) -> Result<Vec<UserSubscribe>, sqlx::Error>;
    async fn query_monthly_reset_subscribe_ids(
        &self,
        subscribe_ids: &[i64],
        now: i64,
    ) -> Result<Vec<i64>, sqlx::Error>;
    async fn query_first_reset_subscribe_ids(
        &self,
        subscribe_ids: &[i64],
        _now: i64,
    ) -> Result<Vec<i64>, sqlx::Error>;
    async fn query_yearly_reset_subscribe_ids(
        &self,
        subscribe_ids: &[i64],
        _now: i64,
    ) -> Result<Vec<i64>, sqlx::Error>;
    async fn reset_subscribe_traffic_by_ids(&self, ids: &[i64]) -> Result<u64, sqlx::Error>;
    async fn find_traffic_exceeded_subscribes(
        &self,
    ) -> Result<Vec<UserSubscribe>, sqlx::Error>;
    async fn find_expired_subscribes(&self, now: i64) -> Result<Vec<UserSubscribe>, sqlx::Error>;
    async fn mark_subscribes_finished(
        &self,
        ids: &[i64],
        status: i16,
        finished_at: i64,
    ) -> Result<u64, sqlx::Error>;
    async fn query_user_subscribe(
        &self,
        user_id: i64,
        statuses: &[i64],
    ) -> Result<Vec<SubscribeDetails>, sqlx::Error>;
    async fn find_users_subscribe_by_subscribe_id(
        &self,
        subscribe_id: i64,
    ) -> Result<Vec<UserSubscribe>, sqlx::Error>;
    async fn find_user_subscribes_by_status(
        &self,
        statuses: &[i64],
    ) -> Result<Vec<UserSubscribe>, sqlx::Error>;
    async fn activate_pending_subscribes_by_subscribe_id(
        &self,
        subscribe_id: i64,
    ) -> Result<u64, sqlx::Error>;
    async fn count_user_subscribes_by_user_and_subscribe(
        &self,
        user_id: i64,
        subscribe_id: i64,
    ) -> Result<i64, sqlx::Error>;
    async fn count_user_subscribes_by_subscribe_id_and_status(
        &self,
        subscribe_id: i64,
        statuses: &[i64],
    ) -> Result<i64, sqlx::Error>;
    async fn update_user_subscribe_with_traffic(
        &self,
        id: i64,
        download: i64,
        upload: i64,
    ) -> Result<u64, sqlx::Error>;

    async fn query_register_user_total_by_date(&self, date: i64) -> Result<i64, sqlx::Error>;
    async fn query_register_user_total_by_monthly(&self, date: i64) -> Result<i64, sqlx::Error>;
    async fn query_register_user_total(&self) -> Result<i64, sqlx::Error>;
    async fn count_enabled_users(&self) -> Result<i64, sqlx::Error>;
    async fn query_admin_users(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn query_active_subscriptions(
        &self,
        subscribe_ids: &[i64],
    ) -> Result<Vec<(i64, i64)>, sqlx::Error>;
    async fn query_email_recipients(
        &self,
        filter: &EmailRecipientFilter,
    ) -> Result<Vec<String>, sqlx::Error>;
    async fn count_email_recipients(&self, filter: &EmailRecipientFilter) -> Result<i64, sqlx::Error>;

    async fn query_subscribe_ids_by_filter(
        &self,
        filter: &SubscribeFilter,
    ) -> Result<Vec<i64>, sqlx::Error>;
    async fn count_subscribes_by_filter(&self, filter: &SubscribeFilter) -> Result<i64, sqlx::Error>;

    async fn query_daily_user_statistics_list(
        &self,
        now: i64,
    ) -> Result<Vec<UserStatisticsWithDate>, sqlx::Error>;
    async fn query_monthly_user_statistics_list(
        &self,
        now: i64,
    ) -> Result<Vec<UserStatisticsWithDate>, sqlx::Error>;
}

pub mod pg;
pub mod mysql;
