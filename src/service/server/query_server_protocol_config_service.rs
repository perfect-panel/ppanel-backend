use std::sync::Arc;
use anyhow::anyhow;

use crate::config::Config;
use crate::model::dto::node::{NodeDNS, NodeOutbound};
use crate::model::dto::protocol::Protocol as DtoProtocol;
use crate::model::dto::server::QueryServerConfigResponse;
use crate::model::entity::node::Protocol as EntityProtocol;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn query_server_protocol_config(
    repos: Arc<Repositories>,
    config: Arc<Config>,
    server_id: i64,
    protocols_filter: Option<Vec<String>>,
) -> Result<QueryServerConfigResponse, anyhow::Error> {
    let server = repos
        .node
        .find_one_server(server_id)
        .await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

    let entity_protocols: Vec<EntityProtocol> = serde_json::from_str(&server.protocols)
        .map_err(|e| anyhow!("parse protocols: {e}"))?;

    let mut protocols: Vec<DtoProtocol> = entity_protocols
        .iter()
        .filter(|p| p.enable)
        .map(entity_to_dto)
        .collect();

    if let Some(ref filter) = protocols_filter {
        if !filter.is_empty() {
            let set: std::collections::HashSet<&str> = filter.iter().map(|s| s.as_str()).collect();
            protocols.retain(|p| set.contains(p.type_.as_str()));
        }
    }

    let override_ = repos
        .node
        .find_override_by_server(server_id)
        .await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

    let ip_strategy = override_
        .as_ref()
        .and_then(|o| o.ip_strategy.clone())
        .unwrap_or_else(|| config.node.ip_strategy.clone());

    let dns: Vec<NodeDNS> = override_
        .as_ref()
        .and_then(|o| o.dns.as_ref())
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_else(|| {
            config.node.dns.iter().map(|d| NodeDNS {
                proto: d.proto.clone(),
                address: d.address.clone(),
                domains: d.domains.clone(),
            }).collect()
        });

    let block: Vec<String> = override_
        .as_ref()
        .and_then(|o| o.block.as_ref())
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_else(|| config.node.block.clone());

    let outbound: Vec<NodeOutbound> = override_
        .as_ref()
        .and_then(|o| o.outbound.as_ref())
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_else(|| {
            config.node.outbound.iter().map(|o| NodeOutbound {
                name: o.name.clone(),
                protocol: o.protocol.clone(),
                address: o.address.clone(),
                port: o.port,
                user: if o.user.is_empty() { None } else { Some(o.user.clone()) },
                password: o.password.clone(),
                uuid: if o.uuid.is_empty() { None } else { Some(o.uuid.clone()) },
                cipher: if o.cipher.is_empty() { None } else { Some(o.cipher.clone()) },
                security: if o.security.is_empty() { None } else { Some(o.security.clone()) },
                sni: if o.sni.is_empty() { None } else { Some(o.sni.clone()) },
                allow_insecure: o.allow_insecure,
                fingerprint: if o.fingerprint.is_empty() { None } else { Some(o.fingerprint.clone()) },
                transport: if o.transport.is_empty() { None } else { Some(o.transport.clone()) },
                host: if o.host.is_empty() { None } else { Some(o.host.clone()) },
                path: if o.path.is_empty() { None } else { Some(o.path.clone()) },
                service_name: if o.service_name.is_empty() { None } else { Some(o.service_name.clone()) },
                flow: if o.flow.is_empty() { None } else { Some(o.flow.clone()) },
                uot: o.uot,
                uot_version: o.uot_version,
                congestion_controller: if o.congestion_controller.is_empty() { None } else { Some(o.congestion_controller.clone()) },
                udp_stream: o.udp_stream,
                reduce_rtt: o.reduce_rtt,
                heartbeat: o.heartbeat,
                reality_public_key: if o.reality_public_key.is_empty() { None } else { Some(o.reality_public_key.clone()) },
                reality_short_id: if o.reality_short_id.is_empty() { None } else { Some(o.reality_short_id.clone()) },
                spider_x: if o.spider_x.is_empty() { None } else { Some(o.spider_x.clone()) },
                settings: if o.settings.is_empty() { None } else { Some(o.settings.clone()) },
                stream_settings: if o.stream_settings.is_empty() { None } else { Some(o.stream_settings.clone()) },
                rules: o.rules.clone(),
            }).collect()
        });

    let total = protocols.len() as i64;
    Ok(QueryServerConfigResponse {
        traffic_report_threshold: config.node.traffic_report_threshold,
        push_interval: config.node.node_push_interval,
        pull_interval: config.node.node_pull_interval,
        ip_strategy,
        dns,
        block,
        outbound,
        protocols,
        total,
    })
}

fn entity_to_dto(p: &EntityProtocol) -> DtoProtocol {
    DtoProtocol {
        type_: p.type_.clone(),
        port: p.port as u16,
        enable: p.enable,
        security: p.security.clone(),
        sni: p.sni.clone(),
        allow_insecure: p.allow_insecure,
        fingerprint: p.fingerprint.clone(),
        reality_server_addr: p.reality_server_addr.clone(),
        reality_server_port: p.reality_server_port,
        reality_private_key: p.reality_private_key.clone(),
        reality_public_key: p.reality_public_key.clone(),
        reality_short_id: p.reality_short_id.clone(),
        transport: p.transport.clone(),
        host: p.host.clone(),
        path: p.path.clone(),
        service_name: p.service_name.clone(),
        cipher: p.cipher.clone(),
        server_key: p.server_key.clone(),
        flow: p.flow.clone(),
        uot: p.uot,
        uot_version: p.uot_version,
        accept_proxy_protocol: p.accept_proxy_protocol,
        hop_ports: p.hop_ports.clone(),
        hop_interval: p.hop_interval,
        obfs_password: p.obfs_password.clone(),
        disable_sni: p.disable_sni,
        reduce_rtt: p.reduce_rtt,
        udp_relay_mode: p.udp_relay_mode.clone(),
        congestion_controller: p.congestion_controller.clone(),
        multiplex: p.multiplex.clone(),
        padding_scheme: p.padding_scheme.clone(),
        up_mbps: p.up_mbps,
        down_mbps: p.down_mbps,
        obfs: p.obfs.clone(),
        obfs_host: p.obfs_host.clone(),
        obfs_path: p.obfs_path.clone(),
        xhttp_mode: p.xhttp_mode.clone(),
        xhttp_extra: p.xhttp_extra.clone(),
        encryption: p.encryption.clone(),
        encryption_mode: p.encryption_mode.clone(),
        encryption_rtt: p.encryption_rtt.clone(),
        encryption_ticket: p.encryption_ticket.clone(),
        encryption_server_padding: p.encryption_server_padding.clone(),
        encryption_private_key: p.encryption_private_key.clone(),
        encryption_client_padding: p.encryption_client_padding.clone(),
        encryption_password: p.encryption_password.clone(),
        ech_enable: p.ech_enable,
        ech_server_name: p.ech_server_name.clone(),
        ratio: p.ratio,
        cert_mode: p.cert_mode.clone(),
        cert_dns_provider: p.cert_dns_provider.clone(),
        cert_dns_env: p.cert_dns_env.clone(),
    }
}
