//! Admin authMethod services — all 7 auth method management services.

use result::code_error::CodeError;
use result::error_code;

use crate::model::dto::auth::{
    AuthMethodConfig, GetAuthMethodConfigRequest, GetAuthMethodListResponse,
    UpdateAuthMethodConfigRequest,
};
use crate::model::entity::auth::Auth;
use crate::repository::auth::AuthRepo;

// ── get_auth_method_list ──────────────────────────────────────────────────────

pub async fn get_auth_method_list(repo: &dyn AuthRepo) -> Result<GetAuthMethodListResponse, anyhow::Error> {
    let methods = repo.get_list().await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;
    let list: Vec<AuthMethodConfig> = methods.into_iter().map(|m| AuthMethodConfig {
        id: m.id,
        method: m.method,
        config: serde_json::from_str(&m.config).unwrap_or_default(),
        enabled: m.enabled.unwrap_or(false),
    }).collect();
    Ok(GetAuthMethodListResponse { list })
}

// ── get_auth_method_config ────────────────────────────────────────────────────

pub async fn get_auth_method_config(
    repo: &dyn AuthRepo,
    req: GetAuthMethodConfigRequest,
) -> Result<AuthMethodConfig, anyhow::Error> {
    let m = repo.find_one_by_method(&req.method).await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;
    Ok(AuthMethodConfig {
        id: m.id,
        method: m.method,
        config: serde_json::from_str(&m.config).unwrap_or_default(),
        enabled: m.enabled.unwrap_or(false),
    })
}

// ── update_auth_method_config ─────────────────────────────────────────────────

pub async fn update_auth_method_config(
    repo: &dyn AuthRepo,
    req: UpdateAuthMethodConfigRequest,
) -> Result<(), anyhow::Error> {
    let mut auth = repo.find_one(req.id).await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;
    auth.config = serde_json::to_string(&req.config).unwrap_or_default();
    if let Some(e) = req.enabled { auth.enabled = Some(e); }
    repo.update(&auth).await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(error_code::DATABASE_UPDATE_ERROR, e.to_string())))?;
    Ok(())
}

// ── get_email_platform ────────────────────────────────────────────────────────

pub async fn get_email_platform() -> Result<Vec<String>, anyhow::Error> {
    // Mirror Go — hard-coded list of supported email platforms.
    Ok(vec!["smtp".to_string()])
}

// ── get_sms_platform ──────────────────────────────────────────────────────────

pub async fn get_sms_platform() -> Result<Vec<String>, anyhow::Error> {
    Ok(vec!["aliyun".to_string(), "tencent".to_string()])
}

// ── test_email_send ───────────────────────────────────────────────────────────

pub async fn test_email_send(
    _cfg: &crate::config::Config,
    _to: String,
) -> Result<(), anyhow::Error> {
    // TODO: use email crate to send a test message when email service is wired.
    tracing::info!("[test_email_send] stub called");
    Ok(())
}

// ── test_sms_send ─────────────────────────────────────────────────────────────

pub async fn test_sms_send(
    _cfg: &crate::config::Config,
    _to: String,
) -> Result<(), anyhow::Error> {
    // TODO: use SMS crate (Phase 4G) to send a test message.
    tracing::info!("[test_sms_send] stub called");
    Ok(())
}

// Suppress unused import warning.
#[allow(dead_code)]
fn _silence(a: Auth) { let _ = a; }