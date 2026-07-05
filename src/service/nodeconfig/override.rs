//! Port of `server/internal/logic/nodeconfig/override.go`
//!
//! Provides helpers to:
//! - build `ServerNodeConfigValues` from global `NodeConfig`
//! - apply per-server override on top of global values
//! - convert `ServerConfigOverride` entity ↔ `ServerNodeConfigOverride` DTO

use crate::config::NodeConfig;
use crate::model::dto::node::{NodeDNS, NodeOutbound, ServerNodeConfigOverride, ServerNodeConfigValues};
use crate::model::entity::node::ServerConfigOverride;

// ── helpers ─────────────────────────────────────────────────────────────────

fn normalize_strings(values: Vec<String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    values
        .into_iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && seen.insert(s.clone()))
        .collect()
}

fn ensure_dns(values: Vec<NodeDNS>) -> Vec<NodeDNS> {
    values
        .into_iter()
        .filter(|d| !d.proto.trim().is_empty() && !d.address.trim().is_empty())
        .map(|d| NodeDNS {
            proto: d.proto.trim().to_string(),
            address: d.address.trim().to_string(),
            domains: normalize_strings(d.domains),
        })
        .collect()
}

fn ensure_outbound(values: Vec<NodeOutbound>) -> Vec<NodeOutbound> {
    values
        .into_iter()
        .filter(|o| !o.name.trim().is_empty() && !o.protocol.trim().is_empty())
        .map(|o| NodeOutbound {
            name: o.name.trim().to_string(),
            protocol: o.protocol.trim().to_string(),
            address: o.address.trim().to_string(),
            rules: normalize_strings(o.rules),
            ..o
        })
        .collect()
}

fn unmarshal_json<T: serde::de::DeserializeOwned>(value: &str, field: &str) -> anyhow::Result<T> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow::anyhow!("empty {field}"));
    }
    serde_json::from_str(trimmed)
        .map_err(|e| anyhow::anyhow!("unmarshal server node config {field}: {e}"))
}

fn marshal_json<T: serde::Serialize>(value: &T, field: &str) -> anyhow::Result<String> {
    serde_json::to_string(value)
        .map_err(|e| anyhow::anyhow!("marshal server node config {field}: {e}"))
}

// ── public API ───────────────────────────────────────────────────────────────

/// Build global `ServerNodeConfigValues` from the `NodeConfig` in config.yaml.
/// Mirrors Go `GlobalValues(c config.NodeConfig)`.
pub fn global_values(c: &NodeConfig) -> ServerNodeConfigValues {
    let dns = c.dns.iter().map(|d| NodeDNS {
        proto: d.proto.clone(),
        address: d.address.clone(),
        domains: normalize_strings(d.domains.clone()),
    }).collect();

    let outbound = c.outbound.iter().map(|o| NodeOutbound {
        name: o.name.clone(),
        protocol: o.protocol.clone(),
        address: o.address.clone(),
        port: o.port,
        user: Some(o.user.clone()).filter(|s| !s.is_empty()),
        password: o.password.clone(),
        uuid: Some(o.uuid.clone()).filter(|s| !s.is_empty()),
        cipher: Some(o.cipher.clone()).filter(|s| !s.is_empty()),
        security: Some(o.security.clone()).filter(|s| !s.is_empty()),
        sni: Some(o.sni.clone()).filter(|s| !s.is_empty()),
        allow_insecure: o.allow_insecure,
        fingerprint: Some(o.fingerprint.clone()).filter(|s| !s.is_empty()),
        transport: Some(o.transport.clone()).filter(|s| !s.is_empty()),
        host: Some(o.host.clone()).filter(|s| !s.is_empty()),
        path: Some(o.path.clone()).filter(|s| !s.is_empty()),
        service_name: Some(o.service_name.clone()).filter(|s| !s.is_empty()),
        flow: Some(o.flow.clone()).filter(|s| !s.is_empty()),
        uot: o.uot,
        uot_version: o.uot_version,
        congestion_controller: Some(o.congestion_controller.clone()).filter(|s| !s.is_empty()),
        udp_stream: o.udp_stream,
        reduce_rtt: o.reduce_rtt,
        heartbeat: o.heartbeat,
        reality_public_key: Some(o.reality_public_key.clone()).filter(|s| !s.is_empty()),
        reality_short_id: Some(o.reality_short_id.clone()).filter(|s| !s.is_empty()),
        spider_x: None,
        settings: None,
        stream_settings: None,
        rules: Vec::new(),
    }).collect();

    ServerNodeConfigValues {
        ip_strategy: c.ip_strategy.clone(),
        dns: ensure_dns(dns),
        block: normalize_strings(c.block.clone()),
        outbound: ensure_outbound(outbound),
    }
}

/// Apply per-server override on top of global values (in-place).
/// Mirrors Go `ApplyOverride`.
pub fn apply_override(
    values: &mut ServerNodeConfigValues,
    override_entity: &ServerConfigOverride,
) -> anyhow::Result<()> {
    if override_entity.id == 0 {
        return Ok(());
    }
    if let Some(ref s) = override_entity.ip_strategy {
        values.ip_strategy = s.clone();
    }
    if let Some(ref s) = override_entity.dns {
        let dns: Vec<NodeDNS> = unmarshal_json(s, "dns")?;
        values.dns = ensure_dns(dns);
    }
    if let Some(ref s) = override_entity.block {
        let block: Vec<String> = unmarshal_json(s, "block")?;
        values.block = normalize_strings(block);
    }
    if let Some(ref s) = override_entity.outbound {
        let outbound: Vec<NodeOutbound> = unmarshal_json(s, "outbound")?;
        values.outbound = ensure_outbound(outbound);
    }
    Ok(())
}

/// Build the DTO response for a server's override config.
/// Mirrors Go `OverrideResponse`.
pub fn override_response(
    override_entity: Option<&ServerConfigOverride>,
) -> anyhow::Result<ServerNodeConfigOverride> {
    let mut resp = ServerNodeConfigOverride {
        inherit_ip_strategy: true,
        ip_strategy: None,
        inherit_dns: true,
        dns: vec![],
        inherit_block: true,
        block: vec![],
        inherit_outbound: true,
        outbound: vec![],
    };
    let ov = match override_entity {
        Some(e) if e.id != 0 => e,
        _ => return Ok(resp),
    };
    if let Some(ref s) = ov.ip_strategy {
        resp.inherit_ip_strategy = false;
        resp.ip_strategy = Some(s.clone());
    }
    if let Some(ref s) = ov.dns {
        let dns: Vec<NodeDNS> = unmarshal_json(s, "dns")?;
        resp.inherit_dns = false;
        resp.dns = ensure_dns(dns);
    }
    if let Some(ref s) = ov.block {
        let block: Vec<String> = unmarshal_json(s, "block")?;
        resp.inherit_block = false;
        resp.block = normalize_strings(block);
    }
    if let Some(ref s) = ov.outbound {
        let outbound: Vec<NodeOutbound> = unmarshal_json(s, "outbound")?;
        resp.inherit_outbound = false;
        resp.outbound = ensure_outbound(outbound);
    }
    Ok(resp)
}

/// Convert a DTO override request into an entity ready for DB upsert.
/// Returns `(entity, all_inherited)`. Mirrors Go `OverrideModel`.
pub fn override_model(
    server_id: i64,
    req: &ServerNodeConfigOverride,
) -> anyhow::Result<(ServerConfigOverride, bool)> {
    use chrono::Utc;
    let now = Utc::now().timestamp_millis();
    let mut data = ServerConfigOverride {
        id: 0,
        server_id,
        ip_strategy: None,
        dns: None,
        block: None,
        outbound: None,
        created_at: now,
        updated_at: now,
    };
    if !req.inherit_ip_strategy {
        data.ip_strategy = req.ip_strategy.clone();
    }
    if !req.inherit_dns {
        data.dns = Some(marshal_json(&ensure_dns(req.dns.clone()), "dns")?);
    }
    if !req.inherit_block {
        data.block = Some(marshal_json(&normalize_strings(req.block.clone()), "block")?);
    }
    if !req.inherit_outbound {
        data.outbound = Some(marshal_json(&ensure_outbound(req.outbound.clone()), "outbound")?);
    }
    let all_inherited = data.ip_strategy.is_none()
        && data.dns.is_none()
        && data.block.is_none()
        && data.outbound.is_none();
    Ok((data, all_inherited))
}

/// Deep-clone a `ServerNodeConfigValues`.
/// Mirrors Go `CloneValues`.
pub fn clone_values(values: &ServerNodeConfigValues) -> ServerNodeConfigValues {
    let dns = values.dns.iter().map(|d| NodeDNS {
        proto: d.proto.clone(),
        address: d.address.clone(),
        domains: normalize_strings(d.domains.clone()),
    }).collect();
    let outbound = values.outbound.iter().map(|o| NodeOutbound {
        rules: normalize_strings(o.rules.clone()),
        ..o.clone()
    }).collect();
    ServerNodeConfigValues {
        ip_strategy: values.ip_strategy.clone(),
        dns: ensure_dns(dns),
        block: normalize_strings(values.block.clone()),
        outbound: ensure_outbound(outbound),
    }
}
