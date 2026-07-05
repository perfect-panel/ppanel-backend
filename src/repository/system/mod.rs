use crate::model::entity::system::System;

#[async_trait::async_trait]
pub trait SystemRepo: Send + Sync {
    async fn insert(&self, data: &System) -> Result<System, sqlx::Error>;
    async fn find_one(&self, id: i64) -> Result<System, sqlx::Error>;
    async fn update(&self, data: &System) -> Result<System, sqlx::Error>;
    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn get_by_category(&self, category: &str) -> Result<Vec<System>, sqlx::Error>;
    async fn update_value_by_category_key(
        &self,
        category: &str,
        key: &str,
        value: &str,
    ) -> Result<u64, sqlx::Error>;
    async fn find_one_by_category_key(
        &self,
        category: &str,
        key: &str,
    ) -> Result<System, sqlx::Error>;

    async fn get_sms_config(&self) -> Result<Vec<System>, sqlx::Error> {
        self.get_by_category("sms").await
    }
    async fn get_site_config(&self) -> Result<Vec<System>, sqlx::Error> {
        self.get_by_category("site").await
    }
    async fn get_subscribe_config(&self) -> Result<Vec<System>, sqlx::Error> {
        self.get_by_category("subscribe").await
    }
    async fn get_register_config(&self) -> Result<Vec<System>, sqlx::Error> {
        self.get_by_category("register").await
    }
    async fn get_verify_config(&self) -> Result<Vec<System>, sqlx::Error> {
        self.get_by_category("verify").await
    }
    async fn get_node_config(&self) -> Result<Vec<System>, sqlx::Error> {
        self.get_by_category("server").await
    }
    async fn get_invite_config(&self) -> Result<Vec<System>, sqlx::Error> {
        self.get_by_category("invite").await
    }
    async fn get_tos_config(&self) -> Result<Vec<System>, sqlx::Error> {
        self.get_by_category("tos").await
    }
    async fn get_currency_config(&self) -> Result<Vec<System>, sqlx::Error> {
        self.get_by_category("currency").await
    }
    async fn get_verify_code_config(&self) -> Result<Vec<System>, sqlx::Error> {
        self.get_by_category("verify_code").await
    }
    async fn get_log_config(&self) -> Result<Vec<System>, sqlx::Error> {
        self.get_by_category("log").await
    }
    async fn get_email_config(&self) -> Result<Vec<System>, sqlx::Error> {
        self.get_by_category("email").await
    }
    async fn update_node_multiplier_config(&self, config: &str) -> Result<u64, sqlx::Error> {
        self.update_value_by_category_key("server", "NodeMultiplierConfig", config)
            .await
    }
    async fn find_node_multiplier_config(&self) -> Result<System, sqlx::Error> {
        self.find_one_by_category_key("server", "NodeMultiplierConfig")
            .await
    }
}

pub mod pg;
pub mod mysql;
