use crate::model::entity::user::{
    AuthMethods, Device, DeviceOnlineRecord, User, UserSubscribe, Withdrawal,
};
use crate::repository::audit;
use crate::repository::normalize_page;
use crate::repository::user::{
    EmailRecipientFilter, SubscribeDetails, SubscribeFilter, UserFilter, UserRepo,
    UserStatisticsWithDate,
};
use chrono::Datelike;

pub struct MySqlUserRepo {
    pool: sqlx::MySqlPool,
}

impl MySqlUserRepo {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

fn day_range(ts: i64) -> (i64, i64) {
    let secs = ts / 1000;
    let s = secs - secs % 86400;
    (s * 1000, (s + 86400) * 1000)
}

fn month_start(ts: i64) -> i64 {
    let secs = ts / 1000;
    let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(secs, 0).unwrap_or_default();
    let year = dt.year();
    let month = dt.month();
    chrono::NaiveDate::from_ymd_opt(year, month, 1)
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .unwrap()
        .and_utc()
        .timestamp_millis()
}

fn month_range(ts: i64) -> (i64, i64) {
    let secs = ts / 1000;
    let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(secs, 0).unwrap_or_default();
    let year = dt.year();
    let month = dt.month();
    let start = chrono::NaiveDate::from_ymd_opt(year, month, 1)
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .unwrap()
        .and_utc();
    let (ny, nm) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let end = chrono::NaiveDate::from_ymd_opt(ny, nm, 1)
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .unwrap()
        .and_utc();
    (start.timestamp_millis(), end.timestamp_millis())
}

fn six_months_ago(ts: i64) -> i64 {
    let secs = ts / 1000;
    let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(secs, 0).unwrap_or_default();
    let ago = dt.checked_sub_months(chrono::Months::new(5)).unwrap_or(dt);
    ago.timestamp_millis()
}

fn mysql_placeholders(n: usize) -> String {
    std::iter::repeat("?").take(n).collect::<Vec<_>>().join(", ")
}

#[async_trait::async_trait]
impl UserRepo for MySqlUserRepo {
    async fn insert_user(&self, data: &User) -> Result<User, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO `user` (password, algo, salt, avatar, balance, refer_code, referer_id,
               commission, referral_percentage, only_first_purchase, gift_amount, enable, is_admin,
               enable_balance_notify, enable_login_notify, enable_subscribe_notify, enable_trade_notify,
               rules, created_at, updated_at, deleted_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&data.password)
        .bind(&data.algo)
        .bind(&data.salt)
        .bind(&data.avatar)
        .bind(data.balance)
        .bind(&data.refer_code)
        .bind(data.referer_id)
        .bind(data.commission)
        .bind(data.referral_percentage)
        .bind(data.only_first_purchase)
        .bind(data.gift_amount)
        .bind(data.enable)
        .bind(data.is_admin)
        .bind(data.enable_balance_notify)
        .bind(data.enable_login_notify)
        .bind(data.enable_subscribe_notify)
        .bind(data.enable_trade_notify)
        .bind(&data.rules)
        .bind(data.created_at)
        .bind(data.updated_at)
        .bind(data.deleted_at)
        .execute(&self.pool)
        .await?;
        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, User>("SELECT * FROM `user` WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_user(&self, id: i64) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM `user` WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update_user(&self, data: &User) -> Result<User, sqlx::Error> {
        sqlx::query(
            "UPDATE `user` SET password = ?, algo = ?, salt = ?, avatar = ?, balance = ?,
               refer_code = ?, referer_id = ?, commission = ?, referral_percentage = ?,
               only_first_purchase = ?, gift_amount = ?, enable = ?, is_admin = ?,
               enable_balance_notify = ?, enable_login_notify = ?, enable_subscribe_notify = ?,
               enable_trade_notify = ?, rules = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&data.password)
        .bind(&data.algo)
        .bind(&data.salt)
        .bind(&data.avatar)
        .bind(data.balance)
        .bind(&data.refer_code)
        .bind(data.referer_id)
        .bind(data.commission)
        .bind(data.referral_percentage)
        .bind(data.only_first_purchase)
        .bind(data.gift_amount)
        .bind(data.enable)
        .bind(data.is_admin)
        .bind(data.enable_balance_notify)
        .bind(data.enable_login_notify)
        .bind(data.enable_subscribe_notify)
        .bind(data.enable_trade_notify)
        .bind(&data.rules)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;
        sqlx::query_as::<_, User>("SELECT * FROM `user` WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete_user(&self, id: i64) -> Result<u64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp_millis();
        let res = sqlx::query("UPDATE `user` SET deleted_at = ? WHERE id = ?")
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn insert_subscribe(&self, data: &UserSubscribe) -> Result<UserSubscribe, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO user_subscribe (user_id, order_id, subscribe_id, start_time, expire_time,
               finished_at, traffic, download, upload, token, uuid, status, note, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(data.user_id)
        .bind(data.order_id)
        .bind(data.subscribe_id)
        .bind(data.start_time)
        .bind(data.expire_time)
        .bind(data.finished_at)
        .bind(data.traffic)
        .bind(data.download)
        .bind(data.upload)
        .bind(&data.token)
        .bind(&data.uuid)
        .bind(data.status)
        .bind(&data.note)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;
        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, UserSubscribe>("SELECT * FROM user_subscribe WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_subscribe(&self, id: i64) -> Result<UserSubscribe, sqlx::Error> {
        sqlx::query_as::<_, UserSubscribe>("SELECT * FROM user_subscribe WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_subscribe_by_token(
        &self,
        token: &str,
    ) -> Result<UserSubscribe, sqlx::Error> {
        sqlx::query_as::<_, UserSubscribe>("SELECT * FROM user_subscribe WHERE token = ?")
            .bind(token)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_subscribe_by_order_id(
        &self,
        order_id: i64,
    ) -> Result<UserSubscribe, sqlx::Error> {
        sqlx::query_as::<_, UserSubscribe>("SELECT * FROM user_subscribe WHERE order_id = ?")
            .bind(order_id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_subscribe_details_by_id(
        &self,
        id: i64,
    ) -> Result<SubscribeDetails, sqlx::Error> {
        sqlx::query_as::<_, SubscribeDetails>(
            "SELECT us.*, s.name AS subscribe_name FROM user_subscribe us \
             LEFT JOIN subscribe s ON s.id = us.subscribe_id WHERE us.id = ?",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one_user_subscribe(&self, id: i64) -> Result<SubscribeDetails, sqlx::Error> {
        sqlx::query_as::<_, SubscribeDetails>(
            "SELECT us.*, s.name AS subscribe_name FROM user_subscribe us \
             LEFT JOIN subscribe s ON s.id = us.subscribe_id WHERE us.id = ?",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_subscribe(&self, data: &UserSubscribe) -> Result<UserSubscribe, sqlx::Error> {
        sqlx::query(
            "UPDATE user_subscribe SET user_id = ?, order_id = ?, subscribe_id = ?, start_time = ?,
               expire_time = ?, finished_at = ?, traffic = ?, download = ?, upload = ?, token = ?,
               uuid = ?, status = ?, note = ?, updated_at = ? WHERE id = ?",
        )
        .bind(data.user_id)
        .bind(data.order_id)
        .bind(data.subscribe_id)
        .bind(data.start_time)
        .bind(data.expire_time)
        .bind(data.finished_at)
        .bind(data.traffic)
        .bind(data.download)
        .bind(data.upload)
        .bind(&data.token)
        .bind(&data.uuid)
        .bind(data.status)
        .bind(&data.note)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;
        sqlx::query_as::<_, UserSubscribe>("SELECT * FROM user_subscribe WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete_subscribe(&self, token: &str) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM user_subscribe WHERE token = ?")
            .bind(token)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn delete_subscribe_by_id(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM user_subscribe WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn insert_auth_method(&self, data: &AuthMethods) -> Result<AuthMethods, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO user_auth_methods (user_id, auth_type, auth_identifier, verified, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(data.user_id)
        .bind(&data.auth_type)
        .bind(&data.auth_identifier)
        .bind(data.verified)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;
        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, AuthMethods>("SELECT * FROM user_auth_methods WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_user_auth_methods(
        &self,
        user_id: i64,
    ) -> Result<Vec<AuthMethods>, sqlx::Error> {
        sqlx::query_as::<_, AuthMethods>(
            "SELECT * FROM user_auth_methods WHERE user_id = ? ORDER BY id ASC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    async fn update_auth_method(&self, data: &AuthMethods) -> Result<AuthMethods, sqlx::Error> {
        sqlx::query(
            "UPDATE user_auth_methods SET user_id = ?, auth_type = ?, auth_identifier = ?,
             verified = ?, updated_at = ? WHERE id = ?",
        )
        .bind(data.user_id)
        .bind(&data.auth_type)
        .bind(&data.auth_identifier)
        .bind(data.verified)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;
        sqlx::query_as::<_, AuthMethods>("SELECT * FROM user_auth_methods WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete_user_auth_methods(
        &self,
        user_id: i64,
        platform: &str,
    ) -> Result<u64, sqlx::Error> {
        let res =
            sqlx::query("DELETE FROM user_auth_methods WHERE user_id = ? AND auth_type = ?")
                .bind(user_id)
                .bind(platform)
                .execute(&self.pool)
                .await?;
        Ok(res.rows_affected())
    }

    async fn delete_user_auth_method_by_identifier(
        &self,
        auth_type: &str,
        identifier: &str,
    ) -> Result<u64, sqlx::Error> {
        let res = sqlx::query(
            "DELETE FROM user_auth_methods WHERE auth_type = ? AND auth_identifier = ?",
        )
        .bind(auth_type)
        .bind(identifier)
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected())
    }

    async fn find_auth_method_by_open_id(
        &self,
        method: &str,
        open_id: &str,
    ) -> Result<Option<AuthMethods>, sqlx::Error> {
        sqlx::query_as::<_, AuthMethods>(
            "SELECT * FROM user_auth_methods WHERE auth_type = ? AND auth_identifier = ?",
        )
        .bind(method)
        .bind(open_id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_auth_method_by_user_id(
        &self,
        method: &str,
        user_id: i64,
    ) -> Result<Option<AuthMethods>, sqlx::Error> {
        sqlx::query_as::<_, AuthMethods>(
            "SELECT * FROM user_auth_methods WHERE auth_type = ? AND user_id = ?",
        )
        .bind(method)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_auth_method_by_platform(
        &self,
        user_id: i64,
        platform: &str,
    ) -> Result<Option<AuthMethods>, sqlx::Error> {
        sqlx::query_as::<_, AuthMethods>(
            "SELECT * FROM user_auth_methods WHERE user_id = ? AND auth_type = ?",
        )
        .bind(user_id)
        .bind(platform)
        .fetch_optional(&self.pool)
        .await
    }

    async fn update_user_auth_method_owner(
        &self,
        auth_type: &str,
        identifier: &str,
        user_id: i64,
    ) -> Result<u64, sqlx::Error> {
        let res = sqlx::query(
            "UPDATE user_auth_methods SET user_id = ? WHERE auth_type = ? AND auth_identifier = ?",
        )
        .bind(user_id)
        .bind(auth_type)
        .bind(identifier)
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected())
    }

    async fn upsert_user_auth_method(
        &self,
        data: &AuthMethods,
    ) -> Result<AuthMethods, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO user_auth_methods (user_id, auth_type, auth_identifier, verified, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)
             ON DUPLICATE KEY UPDATE user_id = VALUES(user_id), verified = VALUES(verified), updated_at = VALUES(updated_at)",
        )
        .bind(data.user_id)
        .bind(&data.auth_type)
        .bind(&data.auth_identifier)
        .bind(data.verified)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;
        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, AuthMethods>("SELECT * FROM user_auth_methods WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn insert_device(&self, data: &Device) -> Result<Device, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO user_device (ip, user_id, user_agent, identifier, online, enabled, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&data.ip)
        .bind(data.user_id)
        .bind(&data.user_agent)
        .bind(&data.identifier)
        .bind(data.online)
        .bind(data.enabled)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;
        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Device>("SELECT * FROM user_device WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_device(&self, id: i64) -> Result<Device, sqlx::Error> {
        sqlx::query_as::<_, Device>("SELECT * FROM user_device WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_device_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<Device>, sqlx::Error> {
        sqlx::query_as::<_, Device>("SELECT * FROM user_device WHERE identifier = ?")
            .bind(identifier)
            .fetch_optional(&self.pool)
            .await
    }

    async fn update_device(&self, data: &Device) -> Result<Device, sqlx::Error> {
        sqlx::query(
            "UPDATE user_device SET ip = ?, user_id = ?, user_agent = ?, identifier = ?,
             online = ?, enabled = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&data.ip)
        .bind(data.user_id)
        .bind(&data.user_agent)
        .bind(&data.identifier)
        .bind(data.online)
        .bind(data.enabled)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;
        sqlx::query_as::<_, Device>("SELECT * FROM user_device WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete_device(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM user_device WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn query_device_list(
        &self,
        user_id: i64,
    ) -> Result<(Vec<Device>, i64), sqlx::Error> {
        let items = sqlx::query_as::<_, Device>(
            "SELECT * FROM user_device WHERE user_id = ? ORDER BY updated_at DESC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        let total = items.len() as i64;
        Ok((items, total))
    }

    async fn query_device_page_list(
        &self,
        user_id: i64,
        _subscribe_id: i64,
        page: i64,
        size: i64,
    ) -> Result<(Vec<Device>, i64), sqlx::Error> {
        let mut page = page;
        let mut size = size;
        normalize_page(&mut page, &mut size);
        let offset = (page - 1) * size;
        let (total,) =
            sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM user_device WHERE user_id = ?")
                .bind(user_id)
                .fetch_one(&self.pool)
                .await?;
        let items = sqlx::query_as::<_, Device>(
            "SELECT * FROM user_device WHERE user_id = ? ORDER BY updated_at DESC LIMIT ? OFFSET ?",
        )
        .bind(user_id)
        .bind(size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok((items, total))
    }

    async fn insert_device_online_record(
        &self,
        data: &DeviceOnlineRecord,
    ) -> Result<DeviceOnlineRecord, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO user_device_online_record (user_id, identifier, online_time, offline_time,
             online_seconds, duration_days, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(data.user_id)
        .bind(&data.identifier)
        .bind(data.online_time)
        .bind(data.offline_time)
        .bind(data.online_seconds)
        .bind(data.duration_days)
        .bind(data.created_at)
        .execute(&self.pool)
        .await?;
        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, DeviceOnlineRecord>(
            "SELECT * FROM user_device_online_record WHERE id = ?",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_device_online_record(
        &self,
        user_id: i64,
        start_time: &str,
        end_time: &str,
    ) -> Result<Option<DeviceOnlineRecord>, sqlx::Error> {
        sqlx::query_as::<_, DeviceOnlineRecord>(
            "SELECT * FROM user_device_online_record \
             WHERE user_id = ? AND online_time >= ? AND online_time < ? LIMIT 1",
        )
        .bind(user_id)
        .bind(start_time)
        .bind(end_time)
        .fetch_optional(&self.pool)
        .await
    }

    async fn insert_withdrawal(&self, data: &Withdrawal) -> Result<Withdrawal, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO user_withdrawal (user_id, amount, content, status, reason, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(data.user_id)
        .bind(data.amount)
        .bind(&data.content)
        .bind(data.status)
        .bind(&data.reason)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;
        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Withdrawal>("SELECT * FROM user_withdrawal WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn query_page_list(
        &self,
        page: i64,
        size: i64,
        filter: &UserFilter,
    ) -> Result<(i64, Vec<User>), sqlx::Error> {
        let offset = (page - 1) * size;

        let mut clauses = Vec::new();
        if !filter.unscoped {
            clauses.push("u.deleted_at IS NULL".to_string());
        }
        if filter.user_id.is_some() {
            clauses.push("u.id = ?".to_string());
        }
        if filter.subscribe_id.is_some() {
            clauses.push(
                "EXISTS (SELECT 1 FROM user_subscribe WHERE user_id = u.id AND subscribe_id = ? AND status IN (0,1))"
                    .to_string(),
            );
        }
        if filter.user_subscribe_id.is_some() {
            clauses.push(
                "EXISTS (SELECT 1 FROM user_subscribe WHERE user_id = u.id AND id = ? AND status IN (0,1))"
                    .to_string(),
            );
        }
        if let Some(ref search) = filter.search {
            if !search.is_empty() {
                clauses.push(
                    "(LOWER(u.refer_code) LIKE LOWER(?) OR EXISTS (SELECT 1 FROM user_auth_methods WHERE user_id = u.id AND LOWER(auth_identifier) LIKE LOWER(?)))"
                        .to_string(),
                );
            }
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };
        let dir = match filter.order.as_deref() {
            Some("ASC") => "ASC",
            _ => "DESC",
        };

        let pattern = filter.search.as_ref()
            .filter(|s| !s.is_empty())
            .map(|search| format!("{}%", search));

        let count_sql = format!("SELECT COUNT(*) FROM `user` u {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if let Some(uid) = filter.user_id {
            count_q = count_q.bind(uid);
        }
        if let Some(sid) = filter.subscribe_id {
            count_q = count_q.bind(sid);
        }
        if let Some(usid) = filter.user_subscribe_id {
            count_q = count_q.bind(usid);
        }
        if let Some(ref p) = pattern {
            count_q = count_q.bind(p).bind(p);
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM `user` u {} ORDER BY u.id {} LIMIT ? OFFSET ?",
            where_str, dir,
        );
        let mut list_q = sqlx::query_as::<_, User>(audit(&list_sql));
        if let Some(uid) = filter.user_id {
            list_q = list_q.bind(uid);
        }
        if let Some(sid) = filter.subscribe_id {
            list_q = list_q.bind(sid);
        }
        if let Some(usid) = filter.user_subscribe_id {
            list_q = list_q.bind(usid);
        }
        if let Some(ref p) = pattern {
            list_q = list_q.bind(p).bind(p);
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }

    async fn find_users_by_ids(&self, ids: &[i64]) -> Result<Vec<User>, sqlx::Error> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let placeholders = mysql_placeholders(ids.len());
        let sql = format!("SELECT * FROM `user` WHERE id IN ({})", placeholders);
        let mut q = sqlx::query_as::<_, User>(audit(&sql));
        for id in ids {
            q = q.bind(id);
        }
        q.fetch_all(&self.pool).await
    }

    async fn find_one_by_refer_code(&self, refer_code: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM `user` WHERE refer_code = ?")
            .bind(refer_code)
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_one_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT u.* FROM `user` u INNER JOIN user_auth_methods a ON a.user_id = u.id
             WHERE a.auth_type = 'email' AND a.auth_identifier = ?",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
    }

    async fn batch_delete_users(&self, ids: &[i64]) -> Result<u64, sqlx::Error> {
        if ids.is_empty() {
            return Ok(0);
        }
        let now = chrono::Utc::now().timestamp_millis();
        let placeholders = mysql_placeholders(ids.len());
        let sql = format!("UPDATE `user` SET deleted_at = ? WHERE id IN ({})", placeholders);
        let mut q = sqlx::query(audit(&sql)).bind(now);
        for id in ids {
            q = q.bind(id);
        }
        let res = q.execute(&self.pool).await?;
        Ok(res.rows_affected())
    }

    async fn count_affiliates(&self, referer_id: i64) -> Result<i64, sqlx::Error> {
        let (total,) =
            sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM `user` WHERE referer_id = ?")
                .bind(referer_id)
                .fetch_one(&self.pool)
                .await?;
        Ok(total)
    }

    async fn query_affiliate_list(
        &self,
        referer_id: i64,
        page: i64,
        size: i64,
    ) -> Result<(i64, Vec<User>), sqlx::Error> {
        let offset = (page - 1) * size;
        let (total,) =
            sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM `user` WHERE referer_id = ?")
                .bind(referer_id)
                .fetch_one(&self.pool)
                .await?;
        let items = sqlx::query_as::<_, User>(
            "SELECT * FROM `user` WHERE referer_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(referer_id)
        .bind(size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok((total, items))
    }

    async fn find_subscribes_by_ids(
        &self,
        ids: &[i64],
    ) -> Result<Vec<UserSubscribe>, sqlx::Error> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let placeholders = mysql_placeholders(ids.len());
        let sql = format!("SELECT * FROM user_subscribe WHERE id IN ({})", placeholders);
        let mut q = sqlx::query_as::<_, UserSubscribe>(audit(&sql));
        for id in ids {
            q = q.bind(id);
        }
        q.fetch_all(&self.pool).await
    }

    async fn query_monthly_reset_subscribe_ids(
        &self,
        subscribe_ids: &[i64],
        now: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        if subscribe_ids.is_empty() {
            return Ok(vec![]);
        }
        let in_ph = mysql_placeholders(subscribe_ids.len());
        let sql = format!(
            "SELECT id FROM user_subscribe WHERE subscribe_id IN ({}) AND status IN (0,1) \
             AND (DAY(FROM_UNIXTIME(start_time / 1000)) = 1 OR start_time >= ?) \
             AND expire_time > ? ORDER BY id",
            in_ph,
        );
        let mut q = sqlx::query_as::<_, (i64,)>(audit(&sql));
        for id in subscribe_ids {
            q = q.bind(id);
        }
        q = q.bind(now - 86400000).bind(now);
        let rows = q.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|(i,)| i).collect())
    }

    async fn query_first_reset_subscribe_ids(
        &self,
        subscribe_ids: &[i64],
        _now: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        if subscribe_ids.is_empty() {
            return Ok(vec![]);
        }
        let placeholders = mysql_placeholders(subscribe_ids.len());
        let sql = format!(
            "SELECT id FROM user_subscribe WHERE subscribe_id IN ({}) AND status IN (0,1) ORDER BY id",
            placeholders,
        );
        let mut q = sqlx::query_as::<_, (i64,)>(audit(&sql));
        for id in subscribe_ids {
            q = q.bind(id);
        }
        let rows = q.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|(i,)| i).collect())
    }

    async fn query_yearly_reset_subscribe_ids(
        &self,
        subscribe_ids: &[i64],
        _now: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        if subscribe_ids.is_empty() {
            return Ok(vec![]);
        }
        let placeholders = mysql_placeholders(subscribe_ids.len());
        let sql = format!(
            "SELECT id FROM user_subscribe WHERE subscribe_id IN ({}) AND status IN (0,1) ORDER BY id",
            placeholders,
        );
        let mut q = sqlx::query_as::<_, (i64,)>(audit(&sql));
        for id in subscribe_ids {
            q = q.bind(id);
        }
        let rows = q.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|(i,)| i).collect())
    }

    async fn reset_subscribe_traffic_by_ids(&self, ids: &[i64]) -> Result<u64, sqlx::Error> {
        if ids.is_empty() {
            return Ok(0);
        }
        let placeholders = mysql_placeholders(ids.len());
        let sql = format!(
            "UPDATE user_subscribe SET download = 0, upload = 0 WHERE id IN ({})",
            placeholders,
        );
        let mut q = sqlx::query(audit(&sql));
        for id in ids {
            q = q.bind(id);
        }
        let res = q.execute(&self.pool).await?;
        Ok(res.rows_affected())
    }

    async fn find_traffic_exceeded_subscribes(
        &self,
    ) -> Result<Vec<UserSubscribe>, sqlx::Error> {
        sqlx::query_as::<_, UserSubscribe>(
            "SELECT * FROM user_subscribe WHERE status = 1 AND traffic > 0 AND (download + upload) >= traffic",
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_expired_subscribes(&self, now: i64) -> Result<Vec<UserSubscribe>, sqlx::Error> {
        sqlx::query_as::<_, UserSubscribe>(
            "SELECT * FROM user_subscribe WHERE status = 1 AND expire_time <= ?",
        )
        .bind(now)
        .fetch_all(&self.pool)
        .await
    }

    async fn mark_subscribes_finished(
        &self,
        ids: &[i64],
        status: i16,
        finished_at: i64,
    ) -> Result<u64, sqlx::Error> {
        if ids.is_empty() {
            return Ok(0);
        }
        let placeholders = mysql_placeholders(ids.len());
        let sql = format!(
            "UPDATE user_subscribe SET status = ?, finished_at = ? WHERE id IN ({})",
            placeholders,
        );
        let mut q = sqlx::query(audit(&sql))
            .bind(status)
            .bind(finished_at);
        for id in ids {
            q = q.bind(id);
        }
        let res = q.execute(&self.pool).await?;
        Ok(res.rows_affected())
    }

    async fn query_user_subscribe(
        &self,
        user_id: i64,
        statuses: &[i64],
    ) -> Result<Vec<SubscribeDetails>, sqlx::Error> {
        if statuses.is_empty() {
            return sqlx::query_as::<_, SubscribeDetails>(
                "SELECT us.*, s.name AS subscribe_name FROM user_subscribe us \
                 LEFT JOIN subscribe s ON s.id = us.subscribe_id WHERE us.user_id = ? ORDER BY us.id DESC",
            )
            .bind(user_id)
            .fetch_all(&self.pool)
            .await;
        }
        let placeholders = mysql_placeholders(statuses.len());
        let sql = format!(
            "SELECT us.*, s.name AS subscribe_name FROM user_subscribe us \
             LEFT JOIN subscribe s ON s.id = us.subscribe_id WHERE us.user_id = ? AND us.status IN ({}) ORDER BY us.id DESC",
            placeholders,
        );
        let mut q = sqlx::query_as::<_, SubscribeDetails>(audit(&sql)).bind(user_id);
        for s in statuses {
            q = q.bind(s);
        }
        q.fetch_all(&self.pool).await
    }

    async fn find_users_subscribe_by_subscribe_id(
        &self,
        subscribe_id: i64,
    ) -> Result<Vec<UserSubscribe>, sqlx::Error> {
        sqlx::query_as::<_, UserSubscribe>(
            "SELECT * FROM user_subscribe WHERE subscribe_id = ? AND status IN (0,1)",
        )
        .bind(subscribe_id)
        .fetch_all(&self.pool)
        .await
    }

    async fn find_user_subscribes_by_status(
        &self,
        statuses: &[i64],
    ) -> Result<Vec<UserSubscribe>, sqlx::Error> {
        if statuses.is_empty() {
            return sqlx::query_as::<_, UserSubscribe>("SELECT * FROM user_subscribe")
                .fetch_all(&self.pool)
                .await;
        }
        let placeholders = mysql_placeholders(statuses.len());
        let sql = format!("SELECT * FROM user_subscribe WHERE status IN ({})", placeholders);
        let mut q = sqlx::query_as::<_, UserSubscribe>(audit(&sql));
        for s in statuses {
            q = q.bind(s);
        }
        q.fetch_all(&self.pool).await
    }

    async fn activate_pending_subscribes_by_subscribe_id(
        &self,
        subscribe_id: i64,
    ) -> Result<u64, sqlx::Error> {
        let res = sqlx::query(
            "UPDATE user_subscribe SET status = 1 WHERE subscribe_id = ? AND status = 0",
        )
        .bind(subscribe_id)
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected())
    }

    async fn count_user_subscribes_by_user_and_subscribe(
        &self,
        user_id: i64,
        subscribe_id: i64,
    ) -> Result<i64, sqlx::Error> {
        let (total,) = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM user_subscribe WHERE user_id = ? AND subscribe_id = ?",
        )
        .bind(user_id)
        .bind(subscribe_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(total)
    }

    async fn count_user_subscribes_by_subscribe_id_and_status(
        &self,
        subscribe_id: i64,
        statuses: &[i64],
    ) -> Result<i64, sqlx::Error> {
        if statuses.is_empty() {
            let (total,) = sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM user_subscribe WHERE subscribe_id = ?",
            )
            .bind(subscribe_id)
            .fetch_one(&self.pool)
            .await?;
            return Ok(total);
        }
        let placeholders = mysql_placeholders(statuses.len());
        let sql = format!(
            "SELECT COUNT(*) FROM user_subscribe WHERE subscribe_id = ? AND status IN ({})",
            placeholders,
        );
        let mut q = sqlx::query_as::<_, (i64,)>(audit(&sql)).bind(subscribe_id);
        for s in statuses {
            q = q.bind(s);
        }
        let (total,) = q.fetch_one(&self.pool).await?;
        Ok(total)
    }

    async fn update_user_subscribe_with_traffic(
        &self,
        id: i64,
        download: i64,
        upload: i64,
    ) -> Result<u64, sqlx::Error> {
        let res = sqlx::query(
            "UPDATE user_subscribe SET download = download + ?, upload = upload + ? WHERE id = ?",
        )
        .bind(download)
        .bind(upload)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected())
    }

    async fn query_register_user_total_by_date(&self, date: i64) -> Result<i64, sqlx::Error> {
        let (start, end) = day_range(date);
        let (total,) = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM `user` WHERE created_at >= ? AND created_at < ?",
        )
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await?;
        Ok(total)
    }

    async fn query_register_user_total_by_monthly(&self, date: i64) -> Result<i64, sqlx::Error> {
        let (start, end) = month_range(date);
        let (total,) = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM `user` WHERE created_at >= ? AND created_at < ?",
        )
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await?;
        Ok(total)
    }

    async fn query_register_user_total(&self) -> Result<i64, sqlx::Error> {
        let (total,) = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM `user`")
            .fetch_one(&self.pool)
            .await?;
        Ok(total)
    }

    async fn count_enabled_users(&self) -> Result<i64, sqlx::Error> {
        let (total,) =
            sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM `user` WHERE enable = ?")
                .bind(true)
                .fetch_one(&self.pool)
                .await?;
        Ok(total)
    }

    async fn query_admin_users(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM `user` WHERE is_admin = ?")
            .bind(true)
            .fetch_all(&self.pool)
            .await
    }

    async fn query_active_subscriptions(
        &self,
        subscribe_ids: &[i64],
    ) -> Result<Vec<(i64, i64)>, sqlx::Error> {
        if subscribe_ids.is_empty() {
            return Ok(vec![]);
        }
        let placeholders = mysql_placeholders(subscribe_ids.len());
        let sql = format!(
            "SELECT subscribe_id, COUNT(*) FROM user_subscribe WHERE subscribe_id IN ({}) AND status IN (0,1) GROUP BY subscribe_id",
            placeholders,
        );
        let mut q = sqlx::query_as::<_, (i64, i64)>(audit(&sql));
        for id in subscribe_ids {
            q = q.bind(id);
        }
        q.fetch_all(&self.pool).await
    }

    async fn query_email_recipients(
        &self,
        filter: &EmailRecipientFilter,
    ) -> Result<Vec<String>, sqlx::Error> {
        if filter.scope == 5 {
            return Ok(vec![]);
        }
        let (sql, binds) = email_recipient_sql_mysql(filter, "a.auth_identifier");
        let mut q = sqlx::query_as::<_, (String,)>(audit(&sql));
        for b in &binds {
            q = q.bind(b);
        }
        let rows = q.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|(s,)| s).collect())
    }

    async fn count_email_recipients(&self, filter: &EmailRecipientFilter) -> Result<i64, sqlx::Error> {
        if filter.scope == 5 {
            return Ok(0);
        }
        let (sql, binds) = email_recipient_sql_mysql(filter, "COUNT(*)");
        let mut q = sqlx::query_as::<_, (i64,)>(audit(&sql));
        for b in &binds {
            q = q.bind(b);
        }
        let (total,) = q.fetch_one(&self.pool).await?;
        Ok(total)
    }

    async fn query_subscribe_ids_by_filter(
        &self,
        filter: &SubscribeFilter,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let (sql, binds) = subscribe_filter_sql_mysql(filter, "SELECT id FROM user_subscribe");
        let mut q = sqlx::query_as::<_, (i64,)>(audit(&sql));
        for b in &binds {
            q = q.bind(b);
        }
        let rows = q.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|(i,)| i).collect())
    }

    async fn count_subscribes_by_filter(&self, filter: &SubscribeFilter) -> Result<i64, sqlx::Error> {
        let (sql, binds) = subscribe_filter_sql_mysql(filter, "SELECT COUNT(*) FROM user_subscribe");
        let mut q = sqlx::query_as::<_, (i64,)>(audit(&sql));
        for b in &binds {
            q = q.bind(b);
        }
        let (total,) = q.fetch_one(&self.pool).await?;
        Ok(total)
    }

    async fn query_daily_user_statistics_list(
        &self,
        now: i64,
    ) -> Result<Vec<UserStatisticsWithDate>, sqlx::Error> {
        let start = month_start(now);
        sqlx::query_as::<_, UserStatisticsWithDate>(
            "SELECT DATE_FORMAT(FROM_UNIXTIME(u.created_at / 1000), '%Y-%m-%d') AS date,
                    COUNT(*) AS register,
                    COALESCE(MAX(n.new_order_users), 0) AS new_order_users,
                    COALESCE(MAX(r.renewal_order_users), 0) AS renewal_order_users
             FROM `user` u
             LEFT JOIN (
                 SELECT DATE_FORMAT(FROM_UNIXTIME(created_at / 1000), '%Y-%m-%d') AS date,
                        COUNT(DISTINCT user_id) AS new_order_users
                 FROM `order`
                 WHERE is_new = true AND created_at >= ? AND created_at <= ? AND status IN (2,5)
                 GROUP BY DATE_FORMAT(FROM_UNIXTIME(created_at / 1000), '%Y-%m-%d')
             ) n ON DATE_FORMAT(FROM_UNIXTIME(u.created_at / 1000), '%Y-%m-%d') = n.date
             LEFT JOIN (
                 SELECT DATE_FORMAT(FROM_UNIXTIME(created_at / 1000), '%Y-%m-%d') AS date,
                        COUNT(DISTINCT user_id) AS renewal_order_users
                 FROM `order`
                 WHERE is_new = false AND created_at >= ? AND created_at <= ? AND status IN (2,5)
                 GROUP BY DATE_FORMAT(FROM_UNIXTIME(created_at / 1000), '%Y-%m-%d')
             ) r ON DATE_FORMAT(FROM_UNIXTIME(u.created_at / 1000), '%Y-%m-%d') = r.date
             WHERE u.created_at >= ? AND u.created_at <= ?
             GROUP BY DATE_FORMAT(FROM_UNIXTIME(u.created_at / 1000), '%Y-%m-%d')
             ORDER BY date ASC",
        )
        .bind(start)
        .bind(now)
        .bind(start)
        .bind(now)
        .bind(start)
        .bind(now)
        .fetch_all(&self.pool)
        .await
    }

    async fn query_monthly_user_statistics_list(
        &self,
        now: i64,
    ) -> Result<Vec<UserStatisticsWithDate>, sqlx::Error> {
        let start = six_months_ago(now);
        sqlx::query_as::<_, UserStatisticsWithDate>(
            "SELECT DATE_FORMAT(FROM_UNIXTIME(u.created_at / 1000), '%Y-%m') AS date,
                    COUNT(*) AS register,
                    COALESCE(MAX(n.new_order_users), 0) AS new_order_users,
                    COALESCE(MAX(r.renewal_order_users), 0) AS renewal_order_users
             FROM `user` u
             LEFT JOIN (
                 SELECT DATE_FORMAT(FROM_UNIXTIME(created_at / 1000), '%Y-%m') AS date,
                        COUNT(DISTINCT user_id) AS new_order_users
                 FROM `order`
                 WHERE is_new = true AND created_at >= ? AND status IN (2,5)
                 GROUP BY DATE_FORMAT(FROM_UNIXTIME(created_at / 1000), '%Y-%m')
             ) n ON DATE_FORMAT(FROM_UNIXTIME(u.created_at / 1000), '%Y-%m') = n.date
             LEFT JOIN (
                 SELECT DATE_FORMAT(FROM_UNIXTIME(created_at / 1000), '%Y-%m') AS date,
                        COUNT(DISTINCT user_id) AS renewal_order_users
                 FROM `order`
                 WHERE is_new = false AND created_at >= ? AND status IN (2,5)
                 GROUP BY DATE_FORMAT(FROM_UNIXTIME(created_at / 1000), '%Y-%m')
             ) r ON DATE_FORMAT(FROM_UNIXTIME(u.created_at / 1000), '%Y-%m') = r.date
             WHERE u.created_at >= ?
             GROUP BY DATE_FORMAT(FROM_UNIXTIME(u.created_at / 1000), '%Y-%m')
             ORDER BY date ASC",
        )
        .bind(start)
        .bind(start)
        .bind(start)
        .fetch_all(&self.pool)
        .await
    }
}

fn subscribe_filter_sql_mysql(filter: &SubscribeFilter, head: &str) -> (String, Vec<i64>) {
    let mut clauses = Vec::new();
    let mut binds: Vec<i64> = Vec::new();
    if !filter.subscribers.is_empty() {
        let ph = mysql_placeholders(filter.subscribers.len());
        clauses.push(format!("subscribe_id IN ({})", ph));
        for s in &filter.subscribers {
            binds.push(*s);
        }
    }
    if filter.is_active == Some(true) {
        clauses.push("status IN (0,1,2)".to_string());
    }
    if filter.start_time != 0 {
        clauses.push("start_time <= ?".to_string());
        binds.push(filter.start_time);
    }
    if filter.end_time != 0 {
        clauses.push("expire_time >= ?".to_string());
        binds.push(filter.end_time);
    }
    let where_str = if clauses.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", clauses.join(" AND "))
    };
    (format!("{} {}", head, where_str), binds)
}

fn email_recipient_sql_mysql(filter: &EmailRecipientFilter, select_expr: &str) -> (String, Vec<i64>) {
    let mut clauses = vec!["a.auth_type = 'email'".to_string()];
    let mut binds: Vec<i64> = Vec::new();
    if filter.register_start_time != 0 {
        clauses.push("u.created_at >= ?".to_string());
        binds.push(filter.register_start_time);
    }
    if filter.register_end_time != 0 {
        clauses.push("u.created_at <= ?".to_string());
        binds.push(filter.register_end_time);
    }
    match filter.scope {
        2 => clauses.push(
            "EXISTS (SELECT 1 FROM user_subscribe us WHERE us.user_id = u.id AND us.status IN (1,2))"
                .to_string(),
        ),
        3 => clauses.push(
            "EXISTS (SELECT 1 FROM user_subscribe us WHERE us.user_id = u.id AND us.status = 3)"
                .to_string(),
        ),
        4 => clauses.push(
            "NOT EXISTS (SELECT 1 FROM user_subscribe us WHERE us.user_id = u.id)".to_string(),
        ),
        _ => {}
    }
    let where_str = format!("WHERE {}", clauses.join(" AND "));
    let sql = format!(
        "SELECT {} FROM user_auth_methods a INNER JOIN `user` u ON u.id = a.user_id {}",
        select_expr, where_str,
    );
    (sql, binds)
}
