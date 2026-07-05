use std::sync::Arc;
use anyhow::anyhow;
use base64::Engine;
use serde_json::Value;

use crate::cache::Cache;
use crate::config::Config;
use crate::model::dto::server::{GetServerConfigResponse, ServerBasic};
use crate::model::entity::node::Protocol;
use crate::repository::Repositories;
use crate::service::server::constant::{
    ANYTLS, HYSTERIA, HYSTERIA2, SERVER_CACHE_TTL_SECS, SERVER_CONFIG_CACHE_KEY,
    SHADOWSOCKS, TROJAN, TUIC, VLESS, VMESS,
};
use crate::service::server::meta::{generate_etag, RequestMeta, ResponseMeta};
use result::code_error::CodeError;
use result::error_code;

pub async fn get_server_config(
    repos: Arc<Repositories>,
    config: Arc<Config>,
    cache: Arc<Cache>,
    server_id: i64,
    protocol: &str,
    meta: RequestMeta,
) -> Result<(GetServerConfigResponse, ResponseMeta), anyhow::Error> {
    let mut resp_meta = ResponseMeta::new();
    let cache_key = format!("{}{server_id}:{protocol}", SERVER_CONFIG_CACHE_KEY);

    if let Ok(Some(cached)) = cache.get(&cache_key).await {
        if !cached.is_empty() {
            let etag = generate_etag(cached.as_bytes());
            if meta.if_none_match == etag {
                return Err(anyhow::anyhow!("304 Not Modified"));
            }
            resp_meta.set_header("ETag", &etag);
            let resp: GetServerConfigResponse = serde_json::from_str(&cached)
                .map_err(|e| anyhow!("json decode cache: {e}"))?;
            return Ok((resp, resp_meta));
        }
    }

    let server = repos
        .node
        .find_one_server(server_id)
        .await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

    let proto_req = if protocol == HYSTERIA2 { HYSTERIA } else { protocol };

    let protocols: Vec<Protocol> = serde_json::from_str(&server.protocols)
        .map_err(|e| anyhow!("parse protocols: {e}"))?;

    let cfg = protocols
        .iter()
        .find(|p| p.enable && p.type_ == proto_req)
        .map(build_protocol_config)
        .ok_or_else(|| anyhow!("protocol {} not found or disabled", protocol))?;

    let resp = GetServerConfigResponse {
        basic: ServerBasic {
            push_interval: config.node.node_push_interval,
            pull_interval: config.node.node_pull_interval,
        },
        protocol: protocol.to_string(),
        config: cfg,
    };

    let encoded = serde_json::to_string(&resp)
        .map_err(|e| anyhow!("json encode: {e}"))?;
    let etag = generate_etag(encoded.as_bytes());
    resp_meta.set_header("ETag", &etag);
    let _ = cache.set_ex(&cache_key, &encoded, SERVER_CACHE_TTL_SECS).await;

    if meta.if_none_match == etag {
        return Err(anyhow::anyhow!("304 Not Modified"));
    }

    Ok((resp, resp_meta))
}

fn build_protocol_config(p: &Protocol) -> Value {
    match p.type_.as_str() {
        SHADOWSOCKS => {
            let key = p.server_key.as_deref().unwrap_or_default();
            let encoded = base64::engine::general_purpose::STANDARD.encode(key.as_bytes());
            serde_json::json!({
                "port": p.port,
                "method": p.cipher.as_deref().unwrap_or_default(),
                "server_key": encoded,
            })
        }
        VLESS | VMESS | TROJAN => serde_json::json!({
            "port": p.port,
            "flow": p.flow.as_deref().unwrap_or_default(),
            "transport": p.transport.as_deref().unwrap_or_default(),
            "transport_config": {
                "path": p.path.as_deref().unwrap_or_default(),
                "host": p.host.as_deref().unwrap_or_default(),
                "service_name": p.service_name.as_deref().unwrap_or_default(),
                "disable_sni": p.disable_sni,
                "reduce_rtt": p.reduce_rtt,
                "udp_relay_mode": p.udp_relay_mode.as_deref().unwrap_or_default(),
                "congestion_controller": p.congestion_controller.as_deref().unwrap_or_default(),
            },
            "security": p.security.as_deref().unwrap_or_default(),
            "security_config": security_config_json(p),
        }),
        ANYTLS | TUIC => serde_json::json!({
            "port": p.port,
            "security_config": security_config_json(p),
        }),
        HYSTERIA => serde_json::json!({
            "port": p.port,
            "hop_ports": p.hop_ports.as_deref().unwrap_or_default(),
            "hop_interval": p.hop_interval,
            "obfs_password": p.obfs_password.as_deref().unwrap_or_default(),
            "security_config": security_config_json(p),
        }),
        _ => serde_json::json!({}),
    }
}

fn security_config_json(p: &Protocol) -> Value {
    serde_json::json!({
        "sni": p.sni.as_deref().unwrap_or_default(),
        "allow_insecure": p.allow_insecure,
        "fingerprint": p.fingerprint.as_deref().unwrap_or_default(),
        "reality_server_addr": p.reality_server_addr.as_deref().unwrap_or_default(),
        "reality_server_port": p.reality_server_port,
        "reality_private_key": p.reality_private_key.as_deref().unwrap_or_default(),
        "reality_public_key": p.reality_public_key.as_deref().unwrap_or_default(),
        "reality_short_id": p.reality_short_id.as_deref().unwrap_or_default(),
        "padding_scheme": p.padding_scheme.as_deref().unwrap_or_default(),
    })
}
