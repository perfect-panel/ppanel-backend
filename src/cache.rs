use redis::aio::ConnectionManager;
use tokio::sync::Mutex;

use crate::config::RedisConfig;

#[derive(Clone)]
pub struct Cache {
    con: std::sync::Arc<Mutex<ConnectionManager>>,
}

impl std::fmt::Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cache").finish_non_exhaustive()
    }
}

impl Cache {
    pub async fn new(cfg: &RedisConfig) -> Result<Self, redis::RedisError> {
        let dsn = format!("redis://:{}@{}", cfg.pass, cfg.host);
        let client = redis::Client::open(dsn)?;
        let mut con = client.get_connection_manager().await?;

        if cfg.db != 0 {
            redis::cmd("SELECT")
                .arg(cfg.db)
                .query_async::<()>(&mut con)
                .await?;
        }

        Ok(Self {
            con: std::sync::Arc::new(Mutex::new(con)),
        })
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, redis::RedisError> {
        let mut con = self.con.lock().await;
        redis::cmd("GET")
            .arg(key)
            .query_async(&mut *con)
            .await
            .map(|v: Option<String>| v)
    }

    pub async fn set_ex(
        &self,
        key: &str,
        value: &str,
        seconds: i64,
    ) -> Result<(), redis::RedisError> {
        let mut con = self.con.lock().await;
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .arg("EX")
            .arg(seconds)
            .query_async(&mut *con)
            .await
    }

    pub async fn del(&self, key: &str) -> Result<(), redis::RedisError> {
        let mut con = self.con.lock().await;
        redis::cmd("DEL")
            .arg(key)
            .query_async(&mut *con)
            .await
    }

    pub async fn exists(&self, key: &str) -> Result<bool, redis::RedisError> {
        let mut con = self.con.lock().await;
        redis::cmd("EXISTS")
            .arg(key)
            .query_async(&mut *con)
            .await
    }

    pub async fn incr(&self, key: &str) -> Result<i64, redis::RedisError> {
        let mut con = self.con.lock().await;
        redis::cmd("INCR")
            .arg(key)
            .query_async(&mut *con)
            .await
    }

    pub async fn expire(&self, key: &str, seconds: i64) -> Result<(), redis::RedisError> {
        let mut con = self.con.lock().await;
        redis::cmd("EXPIRE")
            .arg(key)
            .arg(seconds)
            .query_async(&mut *con)
            .await
    }

    pub async fn get_int(&self, key: &str) -> Result<Option<i64>, redis::RedisError> {
        let mut con = self.con.lock().await;
        redis::cmd("GET")
            .arg(key)
            .query_async(&mut *con)
            .await
            .map(|v: Option<String>| v.and_then(|s| s.parse().ok()))
    }

    pub async fn ttl(&self, key: &str) -> Result<i64, redis::RedisError> {
        let mut con = self.con.lock().await;
        redis::cmd("TTL")
            .arg(key)
            .query_async(&mut *con)
            .await
    }
}
