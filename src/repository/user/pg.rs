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

pub struct PgUserRepo {
    pool: sqlx::PgPool,
}

impl PgUserRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
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

fn pg_placeholders(start: usize, n: usize) -> String {
    (0..n)
        .map(|i| format!("${}", start + i))
        .collect::<Vec<_>>()
        .join(", ")
}

#[async_trait::async_trait]
impl UserRepo for PgUserRepo {
    async fn insert_user(&self, data: &User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"INSERT INTO "user" (password, algo, salt, avatar, balance, refer_code, referer_id,
               commission, referral_percentage, only_first_purchase, gift_amount, enable, is_admin,
               enable_balance_notify, enable_login_notify, enable_subscribe_notify, enable_trade_notify,
               rules, created_at, updated_at, deleted_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
               RETURNING *"#,
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
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one_user(&self, id: i64) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(r#"SELECT * FROM "user" WHERE id = $1"#)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update_user(&self, data: &User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"UPDATE "user" SET password = $1, algo = $2, salt = $3, avatar = $4, balance = $5,
               refer_code = $6, referer_id = $7, commission = $8, referral_percentage = $9,
               only_first_purchase = $10, gift_amount = $11, enable = $12, is_admin = $13,
               enable_balance_notify = $14, enable_login_notify = $15, enable_subscribe_notify = $16,
               enable_trade_notify = $17, rules = $18, updated_at = $19 WHERE id = $20 RETURNING *"#,
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
        .fetch_one(&self.pool)
        .await
    }

    async fn delete_user(&self, id: i64) -> Result<u64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp_millis();
        let res = sqlx::query(r#"UPDATE "user" SET deleted_at = $1 WHERE id = $2"#)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn insert_subscribe(&self, data: &UserSubscribe) -> Result<UserSubscribe, sqlx::Error> {
        sqlx::query_as::<_, UserSubscribe>(
            "INSERT INTO user_subscribe (user_id, order_id, subscribe_id, start_time, expire_time,
               finished_at, traffic, download, upload, token, uuid, status, note, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
               RETURNING *",
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
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one_subscribe(&self, id: i64) -> Result<UserSubscribe, sqlx::Error> {
        sqlx::query_as::<_, UserSubscribe>("SELECT * FROM user_subscribe WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_subscribe_by_token(
        &self,
        token: &str,
    ) -> Result<UserSubscribe, sqlx::Error> {
        sqlx::query_as::<_, UserSubscribe>("SELECT * FROM user_subscribe WHERE token = $1")
            .bind(token)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_subscribe_by_order_id(
        &self,
        order_id: i64,
    ) -> Result<UserSubscribe, sqlx::Error> {
        sqlx::query_as::<_, UserSubscribe>("SELECT * FROM user_subscribe WHERE order_id = $1")
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
             LEFT JOIN subscribe s ON s.id = us.subscribe_id WHERE us.id = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one_user_subscribe(&self, id: i64) -> Result<SubscribeDetails, sqlx::Error> {
        sqlx::query_as::<_, SubscribeDetails>(
            "SELECT us.*, s.name AS subscribe_name FROM user_subscribe us \
             LEFT JOIN subscribe s ON s.id = us.subscribe_id WHERE us.id = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_subscribe(&self, data: &UserSubscribe) -> Result<UserSubscribe, sqlx::Error> {
        sqlx::query_as::<_, UserSubscribe>(
            "UPDATE user_subscribe SET user_id = $1, order_id = $2, subscribe_id = $3, start_time = $4,
               expire_time = $5, finished_at = $6, traffic = $7, download = $8, upload = $9, token = $10,
               uuid = $11, status = $12, note = $13, updated_at = $14 WHERE id = $15 RETURNING *",
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
        .fetch_one(&self.pool)
        .await
    }

    async fn delete_subscribe(&self, token: &str) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM user_subscribe WHERE token = $1")
            .bind(token)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn delete_subscribe_by_id(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM user_subscribe WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn insert_auth_method(&self, data: &AuthMethods) -> Result<AuthMethods, sqlx::Error> {
        sqlx::query_as::<_, AuthMethods>(
            "INSERT INTO user_auth_methods (user_id, auth_type, auth_identifier, verified, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(data.user_id)
        .bind(&data.auth_type)
        .bind(&data.auth_identifier)
        .bind(data.verified)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_user_auth_methods(
        &self,
        user_id: i64,
    ) -> Result<Vec<AuthMethods>, sqlx::Error> {
        sqlx::query_as::<_, AuthMethods>(
            "SELECT * FROM user_auth_methods WHERE user_id = $1 ORDER BY id ASC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    async fn update_auth_method(&self, data: &AuthMethods) -> Result<AuthMethods, sqlx::Error> {
        sqlx::query_as::<_, AuthMethods>(
            "UPDATE user_auth_methods SET user_id = $1, auth_type = $2, auth_identifier = $3,
             verified = $4, updated_at = $5 WHERE id = $6 RETURNING *",
        )
        .bind(data.user_id)
        .bind(&data.auth_type)
        .bind(&data.auth_identifier)
        .bind(data.verified)
        .bind(data.updated_at)
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
            sqlx::query("DELETE FROM user_auth_methods WHERE user_id = $1 AND auth_type = $2")
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
            "DELETE FROM user_auth_methods WHERE auth_type = $1 AND auth_identifier = $2",
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
            "SELECT * FROM user_auth_methods WHERE auth_type = $1 AND auth_identifier = $2",
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
            "SELECT * FROM user_auth_methods WHERE auth_type = $1 AND user_id = $2",
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
            "SELECT * FROM user_auth_methods WHERE user_id = $1 AND auth_type = $2",
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
            "UPDATE user_auth_methods SET user_id = $1 WHERE auth_type = $2 AND auth_identifier = $3",
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
        sqlx::query_as::<_, AuthMethods>(
            "INSERT INTO user_auth_methods (user_id, auth_type, auth_identifier, verified, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (auth_type, auth_identifier) DO UPDATE SET user_id = EXCLUDED.user_id,
             verified = EXCLUDED.verified, updated_at = EXCLUDED.updated_at RETURNING *",
        )
        .bind(data.user_id)
        .bind(&data.auth_type)
        .bind(&data.auth_identifier)
        .bind(data.verified)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn insert_device(&self, data: &Device) -> Result<Device, sqlx::Error> {
        sqlx::query_as::<_, Device>(
            "INSERT INTO user_device (ip, user_id, user_agent, identifier, online, enabled, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        )
        .bind(&data.ip)
        .bind(data.user_id)
        .bind(&data.user_agent)
        .bind(&data.identifier)
        .bind(data.online)
        .bind(data.enabled)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one_device(&self, id: i64) -> Result<Device, sqlx::Error> {
        sqlx::query_as::<_, Device>("SELECT * FROM user_device WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_device_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<Device>, sqlx::Error> {
        sqlx::query_as::<_, Device>("SELECT * FROM user_device WHERE identifier = $1")
            .bind(identifier)
            .fetch_optional(&self.pool)
            .await
    }

    async fn update_device(&self, data: &Device) -> Result<Device, sqlx::Error> {
        sqlx::query_as::<_, Device>(
            "UPDATE user_device SET ip = $1, user_id = $2, user_agent = $3, identifier = $4,
             online = $5, enabled = $6, updated_at = $7 WHERE id = $8 RETURNING *",
        )
        .bind(&data.ip)
        .bind(data.user_id)
        .bind(&data.user_agent)
        .bind(&data.identifier)
        .bind(data.online)
        .bind(data.enabled)
        .bind(data.updated_at)
        .bind(data.id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete_device(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM user_device WHERE id = $1")
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
            "SELECT * FROM user_device WHERE user_id = $1 ORDER BY updated_at DESC",
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
            sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM user_device WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(&self.pool)
                .await?;
        let items = sqlx::query_as::<_, Device>(
            "SELECT * FROM user_device WHERE user_id = $1 ORDER BY updated_at DESC LIMIT $2 OFFSET $3",
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
        sqlx::query_as::<_, DeviceOnlineRecord>(
            "INSERT INTO user_device_online_record (user_id, identifier, online_time, offline_time,
             online_seconds, duration_days, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        )
        .bind(data.user_id)
        .bind(&data.identifier)
        .bind(data.online_time)
        .bind(data.offline_time)
        .bind(data.online_seconds)
        .bind(data.duration_days)
        .bind(data.created_at)
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
             WHERE user_id = $1 AND online_time >= $2 AND online_time < $3 LIMIT 1",
        )
        .bind(user_id)
        .bind(start_time)
        .bind(end_time)
        .fetch_optional(&self.pool)
        .await
    }

    async fn insert_withdrawal(&self, data: &Withdrawal) -> Result<Withdrawal, sqlx::Error> {
        sqlx::query_as::<_, Withdrawal>(
            "INSERT INTO user_withdrawal (user_id, amount, content, status, reason, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        )
        .bind(data.user_id)
        .bind(data.amount)
        .bind(&data.content)
        .bind(data.status)
        .bind(&data.reason)
        .bind(data.created_at)
        .bind(data.updated_at)
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
        let mut idx = 0u32;
        if let Some(uid) = filter.user_id {
            idx += 1;
            clauses.push(format!("u.id = ${}", idx));
        }
        if filter.subscribe_id.is_some() {
            idx += 1;
            clauses.push(format!(
                "EXISTS (SELECT 1 FROM user_subscribe WHERE user_id = u.id AND subscribe_id = ${} AND status IN (0,1))",
                idx
            ));
        }
        if filter.user_subscribe_id.is_some() {
            idx += 1;
            clauses.push(format!(
                "EXISTS (SELECT 1 FROM user_subscribe WHERE user_id = u.id AND id = ${} AND status IN (0,1))",
                idx
            ));
        }
        if let Some(ref search) = filter.search {
            if !search.is_empty() {
                idx += 1;
                clauses.push(format!(
                    "(u.refer_code ILIKE ${} OR EXISTS (SELECT 1 FROM user_auth_methods WHERE user_id = u.id AND auth_identifier ILIKE ${}))",
                    idx, idx
                ));
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

        let count_sql = format!(r#"SELECT COUNT(*) FROM "user" u {}"#, where_str);
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
        if let Some(ref search) = filter.search {
            if !search.is_empty() {
                count_q = count_q.bind(format!("{}%", search));
            }
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            r#"SELECT * FROM "user" u {} ORDER BY u.id {} LIMIT ${} OFFSET ${}"#,
            where_str,
            dir,
            idx + 1,
            idx + 2,
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
        if let Some(ref search) = filter.search {
            if !search.is_empty() {
                list_q = list_q.bind(format!("{}%", search));
            }
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }

    async fn find_users_by_ids(&self, ids: &[i64]) -> Result<Vec<User>, sqlx::Error> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let placeholders = pg_placeholders(1, ids.len());
        let sql = format!(r#"SELECT * FROM "user" WHERE id IN ({})"#, placeholders);
        let mut q = sqlx::query_as::<_, User>(audit(&sql));
        for id in ids {
            q = q.bind(id);
        }
        q.fetch_all(&self.pool).await
    }

    async fn find_one_by_refer_code(&self, refer_code: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(r#"SELECT * FROM "user" WHERE refer_code = $1"#)
            .bind(refer_code)
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_one_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"SELECT u.* FROM "user" u INNER JOIN user_auth_methods a ON a.user_id = u.id
               WHERE a.auth_type = 'email' AND a.auth_identifier = $1"#,
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
        let placeholders = pg_placeholders(2, ids.len());
        let sql = format!(r#"UPDATE "user" SET deleted_at = $1 WHERE id IN ({})"#, placeholders);
        let mut q = sqlx::query(audit(&sql)).bind(now);
        for id in ids {
            q = q.bind(id);
        }
        let res = q.execute(&self.pool).await?;
        Ok(res.rows_affected())
    }

    async fn count_affiliates(&self, referer_id: i64) -> Result<i64, sqlx::Error> {
        let (total,) = sqlx::query_as::<_, (i64,)>(
            r#"SELECT COUNT(*) FROM "user" WHERE referer_id = $1"#,
        )
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
        let (total,) = sqlx::query_as::<_, (i64,)>(
            r#"SELECT COUNT(*) FROM "user" WHERE referer_id = $1"#,
        )
        .bind(referer_id)
        .fetch_one(&self.pool)
        .await?;
        let items = sqlx::query_as::<_, User>(
            r#"SELECT * FROM "user" WHERE referer_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"#,
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
        let placeholders = pg_placeholders(1, ids.len());
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
        let n = subscribe_ids.len();
        let in_ph = pg_placeholders(1, n);
        let day_idx = n + 1;
        let now_idx = n + 2;
        let sql = format!(
            "SELECT id FROM user_subscribe WHERE subscribe_id IN ({}) AND status IN (0,1) \
             AND (EXTRACT(DAY FROM TO_TIMESTAMP(start_time / 1000)) = 1 OR start_time >= ${}) \
             AND expire_time > ${}",
            in_ph, day_idx, now_idx,
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
        let placeholders = pg_placeholders(1, subscribe_ids.len());
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
        let placeholders = pg_placeholders(1, subscribe_ids.len());
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
        let placeholders = pg_placeholders(1, ids.len());
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
            "SELECT * FROM user_subscribe WHERE status = 1 AND expire_time <= $1",
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
        let placeholders = pg_placeholders(3, ids.len());
        let sql = format!(
            "UPDATE user_subscribe SET status = $1, finished_at = $2 WHERE id IN ({})",
            placeholders,
        );
        let mut q = sqlx::query(audit(&sql)).bind(status).bind(finished_at);
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
                 LEFT JOIN subscribe s ON s.id = us.subscribe_id WHERE us.user_id = $1 ORDER BY us.id DESC",
            )
            .bind(user_id)
            .fetch_all(&self.pool)
            .await;
        }
        let placeholders = pg_placeholders(2, statuses.len());
        let sql = format!(
            "SELECT us.*, s.name AS subscribe_name FROM user_subscribe us \
             LEFT JOIN subscribe s ON s.id = us.subscribe_id WHERE us.user_id = $1 AND us.status IN ({}) ORDER BY us.id DESC",
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
            "SELECT * FROM user_subscribe WHERE subscribe_id = $1 AND status IN (0,1)",
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
        let placeholders = pg_placeholders(1, statuses.len());
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
            "UPDATE user_subscribe SET status = 1 WHERE subscribe_id = $1 AND status = 0",
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
            "SELECT COUNT(*) FROM user_subscribe WHERE user_id = $1 AND subscribe_id = $2",
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
                "SELECT COUNT(*) FROM user_subscribe WHERE subscribe_id = $1",
            )
            .bind(subscribe_id)
            .fetch_one(&self.pool)
            .await?;
            return Ok(total);
        }
        let placeholders = pg_placeholders(2, statuses.len());
        let sql = format!(
            "SELECT COUNT(*) FROM user_subscribe WHERE subscribe_id = $1 AND status IN ({})",
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
            "UPDATE user_subscribe SET download = download + $1, upload = upload + $2 WHERE id = $3",
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
            r#"SELECT COUNT(*) FROM "user" WHERE created_at >= $1 AND created_at < $2"#,
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
            r#"SELECT COUNT(*) FROM "user" WHERE created_at >= $1 AND created_at < $2"#,
        )
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await?;
        Ok(total)
    }

    async fn query_register_user_total(&self) -> Result<i64, sqlx::Error> {
        let (total,) = sqlx::query_as::<_, (i64,)>(r#"SELECT COUNT(*) FROM "user""#)
            .fetch_one(&self.pool)
            .await?;
        Ok(total)
    }

    async fn count_enabled_users(&self) -> Result<i64, sqlx::Error> {
        let (total,) = sqlx::query_as::<_, (i64,)>(r#"SELECT COUNT(*) FROM "user" WHERE enable = $1"#)
            .bind(true)
            .fetch_one(&self.pool)
            .await?;
        Ok(total)
    }

    async fn query_admin_users(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(r#"SELECT * FROM "user" WHERE is_admin = $1"#)
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
        let placeholders = pg_placeholders(1, subscribe_ids.len());
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
        let mut clauses = vec!["a.auth_type = 'email'".to_string()];
        let mut idx = 0u32;
        if filter.register_start_time != 0 {
            idx += 1;
            clauses.push(format!("u.created_at >= ${}", idx));
        }
        if filter.register_end_time != 0 {
            idx += 1;
            clauses.push(format!("u.created_at <= ${}", idx));
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
            r#"SELECT a.auth_identifier FROM user_auth_methods a INNER JOIN "user" u ON u.id = a.user_id {}"#,
            where_str,
        );
        let mut q = sqlx::query_as::<_, (String,)>(audit(&sql));
        if filter.register_start_time != 0 {
            q = q.bind(filter.register_start_time);
        }
        if filter.register_end_time != 0 {
            q = q.bind(filter.register_end_time);
        }
        let rows = q.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|(s,)| s).collect())
    }

    async fn count_email_recipients(&self, filter: &EmailRecipientFilter) -> Result<i64, sqlx::Error> {
        if filter.scope == 5 {
            return Ok(0);
        }
        let mut clauses = vec!["a.auth_type = 'email'".to_string()];
        let mut idx = 0u32;
        if filter.register_start_time != 0 {
            idx += 1;
            clauses.push(format!("u.created_at >= ${}", idx));
        }
        if filter.register_end_time != 0 {
            idx += 1;
            clauses.push(format!("u.created_at <= ${}", idx));
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
            r#"SELECT COUNT(*) FROM user_auth_methods a INNER JOIN "user" u ON u.id = a.user_id {}"#,
            where_str,
        );
        let mut q = sqlx::query_as::<_, (i64,)>(audit(&sql));
        if filter.register_start_time != 0 {
            q = q.bind(filter.register_start_time);
        }
        if filter.register_end_time != 0 {
            q = q.bind(filter.register_end_time);
        }
        let (total,) = q.fetch_one(&self.pool).await?;
        Ok(total)
    }

    async fn query_subscribe_ids_by_filter(
        &self,
        filter: &SubscribeFilter,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let (sql, binds) = subscribe_filter_sql_pg(filter, "SELECT id FROM user_subscribe");
        let mut q = sqlx::query_as::<_, (i64,)>(audit(&sql));
        for b in &binds {
            q = q.bind(b);
        }
        let rows = q.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|(i,)| i).collect())
    }

    async fn count_subscribes_by_filter(&self, filter: &SubscribeFilter) -> Result<i64, sqlx::Error> {
        let (sql, binds) = subscribe_filter_sql_pg(filter, "SELECT COUNT(*) FROM user_subscribe");
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
            r#"SELECT TO_CHAR(TO_TIMESTAMP(u.created_at / 1000), 'YYYY-MM-DD') AS date,
                      COUNT(*) AS register,
                      COALESCE(MAX(n.new_order_users), 0) AS new_order_users,
                      COALESCE(MAX(r.renewal_order_users), 0) AS renewal_order_users
               FROM "user" u
               LEFT JOIN (
                   SELECT TO_CHAR(TO_TIMESTAMP(created_at / 1000), 'YYYY-MM-DD') AS date,
                          COUNT(DISTINCT user_id) AS new_order_users
                   FROM "order"
                   WHERE is_new = true AND created_at >= $1 AND created_at <= $2 AND status IN (2,5)
                   GROUP BY TO_CHAR(TO_TIMESTAMP(created_at / 1000), 'YYYY-MM-DD')
               ) n ON TO_CHAR(TO_TIMESTAMP(u.created_at / 1000), 'YYYY-MM-DD') = n.date
               LEFT JOIN (
                   SELECT TO_CHAR(TO_TIMESTAMP(created_at / 1000), 'YYYY-MM-DD') AS date,
                          COUNT(DISTINCT user_id) AS renewal_order_users
                   FROM "order"
                   WHERE is_new = false AND created_at >= $1 AND created_at <= $2 AND status IN (2,5)
                   GROUP BY TO_CHAR(TO_TIMESTAMP(created_at / 1000), 'YYYY-MM-DD')
               ) r ON TO_CHAR(TO_TIMESTAMP(u.created_at / 1000), 'YYYY-MM-DD') = r.date
               WHERE u.created_at >= $1 AND u.created_at <= $2
               GROUP BY TO_CHAR(TO_TIMESTAMP(u.created_at / 1000), 'YYYY-MM-DD')
               ORDER BY date ASC"#,
        )
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
            r#"SELECT TO_CHAR(TO_TIMESTAMP(u.created_at / 1000), 'YYYY-MM') AS date,
                      COUNT(*) AS register,
                      COALESCE(MAX(n.new_order_users), 0) AS new_order_users,
                      COALESCE(MAX(r.renewal_order_users), 0) AS renewal_order_users
               FROM "user" u
               LEFT JOIN (
                   SELECT TO_CHAR(TO_TIMESTAMP(created_at / 1000), 'YYYY-MM') AS date,
                          COUNT(DISTINCT user_id) AS new_order_users
                   FROM "order"
                   WHERE is_new = true AND created_at >= $1 AND status IN (2,5)
                   GROUP BY TO_CHAR(TO_TIMESTAMP(created_at / 1000), 'YYYY-MM')
               ) n ON TO_CHAR(TO_TIMESTAMP(u.created_at / 1000), 'YYYY-MM') = n.date
               LEFT JOIN (
                   SELECT TO_CHAR(TO_TIMESTAMP(created_at / 1000), 'YYYY-MM') AS date,
                          COUNT(DISTINCT user_id) AS renewal_order_users
                   FROM "order"
                   WHERE is_new = false AND created_at >= $1 AND status IN (2,5)
                   GROUP BY TO_CHAR(TO_TIMESTAMP(created_at / 1000), 'YYYY-MM')
               ) r ON TO_CHAR(TO_TIMESTAMP(u.created_at / 1000), 'YYYY-MM') = r.date
               WHERE u.created_at >= $1
               GROUP BY TO_CHAR(TO_TIMESTAMP(u.created_at / 1000), 'YYYY-MM')
               ORDER BY date ASC"#,
        )
        .bind(start)
        .fetch_all(&self.pool)
        .await
    }
}

fn subscribe_filter_sql_pg(filter: &SubscribeFilter, head: &str) -> (String, Vec<i64>) {
    let mut clauses = Vec::new();
    let mut idx = 0u32;
    let mut binds: Vec<i64> = Vec::new();
    if !filter.subscribers.is_empty() {
        let ph = pg_placeholders(1, filter.subscribers.len());
        idx = filter.subscribers.len() as u32;
        clauses.push(format!("subscribe_id IN ({})", ph));
        for s in &filter.subscribers {
            binds.push(*s);
        }
    }
    if filter.is_active == Some(true) {
        clauses.push("status IN (0,1,2)".to_string());
    }
    if filter.start_time != 0 {
        idx += 1;
        clauses.push(format!("start_time <= ${}", idx));
        binds.push(filter.start_time);
    }
    if filter.end_time != 0 {
        idx += 1;
        clauses.push(format!("expire_time >= ${}", idx));
        binds.push(filter.end_time);
    }
    let where_str = if clauses.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", clauses.join(" AND "))
    };
    (format!("{} {}", head, where_str), binds)
}
