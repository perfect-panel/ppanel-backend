use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::cache::Cache;
use crate::config::Config;
use crate::model::entity::user::{AuthMethods, Device};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct BindDeviceService {
    repos: Arc<Repositories>,
    _config: Arc<Config>,
    _cache: Arc<Cache>,
}

impl BindDeviceService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>, cache: Arc<Cache>) -> Self {
        Self {
            repos,
            _config: config,
            _cache: cache,
        }
    }

    pub async fn bind_device_to_user(
        &self,
        identifier: &str,
        ip: &str,
        user_agent: &str,
        current_user_id: i64,
    ) -> Result<(), anyhow::Error> {
        if identifier.is_empty() {
            return Ok(());
        }

        let now = Utc::now().timestamp_millis();

        match self
            .repos
            .user
            .find_one_device_by_identifier(identifier)
            .await
        {
            Ok(None) => {
                self.create_device_for_user(identifier, ip, user_agent, current_user_id, now)
                    .await?;
            }
            Ok(Some(existing)) => {
                if existing.user_id == current_user_id {
                    self.repos
                        .user
                        .update_device(&Device {
                            id: existing.id,
                            ip: ip.to_string(),
                            user_id: current_user_id,
                            user_agent: Some(user_agent.to_string()),
                            identifier: identifier.to_string(),
                            online: existing.online,
                            enabled: existing.enabled,
                            created_at: existing.created_at,
                            updated_at: now,
                        })
                        .await
                        .map_err(|e| {
                            anyhow!(CodeError::new_err_code_msg(
                                error_code::DATABASE_UPDATE_ERROR,
                                e.to_string()
                            ))
                        })?;
                } else {
                    self.rebind_device_to_new_user(
                        identifier,
                        ip,
                        user_agent,
                        current_user_id,
                        existing.user_id,
                        now,
                    )
                    .await?;
                }
            }
            Err(e) => {
                return Err(anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                )));
            }
        }

        Ok(())
    }

    async fn create_device_for_user(
        &self,
        identifier: &str,
        ip: &str,
        user_agent: &str,
        user_id: i64,
        now: i64,
    ) -> Result<(), anyhow::Error> {
        let _ = self
            .repos
            .user
            .insert_auth_method(&AuthMethods {
                id: 0,
                user_id,
                auth_type: "device".to_string(),
                auth_identifier: identifier.to_string(),
                verified: true,
                created_at: now,
                updated_at: now,
            })
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_INSERT_ERROR,
                    e.to_string()
                ))
            })?;

        let _ = self
            .repos
            .user
            .insert_device(&Device {
                id: 0,
                ip: ip.to_string(),
                user_id,
                user_agent: Some(user_agent.to_string()),
                identifier: identifier.to_string(),
                online: false,
                enabled: true,
                created_at: now,
                updated_at: now,
            })
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_INSERT_ERROR,
                    e.to_string()
                ))
            })?;

        Ok(())
    }

    async fn rebind_device_to_new_user(
        &self,
        identifier: &str,
        ip: &str,
        user_agent: &str,
        new_user_id: i64,
        old_user_id: i64,
        now: i64,
    ) -> Result<(), anyhow::Error> {
        let other_methods = self
            .repos
            .user
            .find_user_auth_methods(old_user_id)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;

        let non_device_methods: Vec<_> = other_methods
            .iter()
            .filter(|m| m.auth_type != "device")
            .collect();

        if non_device_methods.is_empty() {
            let old_user = self.repos.user.find_one_user(old_user_id).await.map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;
            let mut updated = old_user;
            updated.enable = false;
            self.repos.user.update_user(&updated).await.map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_UPDATE_ERROR,
                    e.to_string()
                ))
            })?;
        }

        self.repos
            .user
            .delete_user_auth_method_by_identifier("device", identifier)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_DELETED_ERROR,
                    e.to_string()
                ))
            })?;

        let _ = self
            .repos
            .user
            .insert_auth_method(&AuthMethods {
                id: 0,
                user_id: new_user_id,
                auth_type: "device".to_string(),
                auth_identifier: identifier.to_string(),
                verified: true,
                created_at: now,
                updated_at: now,
            })
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_INSERT_ERROR,
                    e.to_string()
                ))
            })?;

        let _ = self
            .repos
            .user
            .insert_device(&Device {
                id: 0,
                ip: ip.to_string(),
                user_id: new_user_id,
                user_agent: Some(user_agent.to_string()),
                identifier: identifier.to_string(),
                online: false,
                enabled: true,
                created_at: now,
                updated_at: now,
            })
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_INSERT_ERROR,
                    e.to_string()
                ))
            })?;

        Ok(())
    }
}
