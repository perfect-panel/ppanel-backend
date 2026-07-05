//! Adapter module — ports Go `server/adapter` package to Rust.
//!
//! Provides:
//! - [`Proxy`]       – per-proxy configuration struct (mirrors Go `Proxy`)
//! - [`User`]        – subscriber info (mirrors Go `User`)
//! - [`ClientConfig`] – template-rendering config
//! - [`Client`]      – renders a Go-template with a sprig subset
//! - [`Adapter`]     – converts `Node`+`Server` entities into `Vec<Proxy>`

use std::collections::HashMap;

use anyhow::Context as _;
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use chrono::TimeZone as _;

use crate::model::entity::node::{Node, Protocol as NodeProtocol, Server};

// ─────────────────────────────────────────────────────────────────────────────
// Proxy
// ─────────────────────────────────────────────────────────────────────────────

/// Full proxy configuration, mirroring the Go `Proxy` struct in `adapter/client.go`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Proxy {
    pub sort: i32,
    pub name: String,
    pub server: String,
    pub port: i32,
    #[serde(rename = "Type")]
    pub type_: String,
    pub tags: Vec<String>,

    // Security
    pub security: Option<String>,
    pub sni: Option<String>,
    pub allow_insecure: bool,
    pub fingerprint: Option<String>,
    pub reality_server_addr: Option<String>,
    pub reality_server_port: i32,
    pub reality_private_key: Option<String>,
    pub reality_public_key: Option<String>,
    pub reality_short_id: Option<String>,

    // Transport
    pub transport: Option<String>,
    pub host: Option<String>,
    pub path: Option<String>,
    pub service_name: Option<String>,

    // Shadowsocks
    pub method: Option<String>,
    pub server_key: Option<String>,
    pub uot: bool,
    pub uot_version: i32,

    // Vmess/Vless/Trojan
    pub flow: Option<String>,

    // Hysteria2
    pub hop_ports: Option<String>,
    pub hop_interval: i32,
    pub obfs_password: Option<String>,
    pub up_mbps: i32,
    pub down_mbps: i32,

    // TUIC
    pub disable_sni: bool,
    pub reduce_rtt: bool,
    pub udp_relay_mode: Option<String>,
    pub congestion_controller: Option<String>,

    // AnyTLS
    pub padding_scheme: Option<String>,

    // Mieru
    pub multiplex: Option<String>,

    // Vless xhttp
    pub xhttp_mode: Option<String>,
    pub xhttp_extra: Option<String>,

    // Encryption
    pub encryption: Option<String>,
    pub encryption_mode: Option<String>,
    pub encryption_rtt: Option<String>,
    pub encryption_ticket: Option<String>,
    pub encryption_server_padding: Option<String>,
    pub encryption_private_key: Option<String>,
    pub encryption_client_padding: Option<String>,
    pub encryption_password: Option<String>,

    // ECH
    pub ech_enable: bool,
    pub ech_server_name: Option<String>,

    // Misc
    pub ratio: f64,
    pub cert_mode: Option<String>,
    pub cert_dns_provider: Option<String>,
    pub cert_dns_env: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// User
// ─────────────────────────────────────────────────────────────────────────────

/// Subscriber / user info passed to templates (mirrors Go `User`).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct User {
    pub password: String,
    /// Unix timestamp (seconds).
    pub expired_at: i64,
    pub download: i64,
    pub upload: i64,
    pub traffic: i64,
    pub subscribe_url: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// ClientConfig
// ─────────────────────────────────────────────────────────────────────────────

/// Configuration for a [`Client`] instance.
#[derive(Debug, Clone, Default)]
pub struct ClientConfig {
    pub site_name: String,
    pub subscribe_name: String,
    /// Output format, e.g. `"base64"`, `"yaml"`, `"json"`.
    pub output_format: String,
    pub params: HashMap<String, String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Client
// ─────────────────────────────────────────────────────────────────────────────

/// Renders a Go-compatible template with a sprig-subset function map.
pub struct Client {
    pub config: ClientConfig,
}

impl Client {
    /// Render `template` against `proxies` and `user`.
    ///
    /// Mirrors Go `(*Client).Build()`.
    pub fn build(&self, template: &str, proxies: &[Proxy], user: &User) -> anyhow::Result<String> {
        let mut tmpl = gtmpl::Template::default();

        // Register sprig-subset functions.
        tmpl.add_func("toJson", sprig_to_json);
        tmpl.add_func("b64enc", sprig_b64enc);
        tmpl.add_func("date", sprig_date);

        tmpl.parse(template)
            .map_err(|e| anyhow::anyhow!("template parse error: {e}"))?;

        // Serialize each proxy to a serde_json::Value, then lift to gtmpl::Value.
        let proxy_values: Vec<gtmpl::Value> = proxies
            .iter()
            .map(|p| {
                let json = serde_json::to_value(p)
                    .context("serialize Proxy to JSON")?;
                Ok(json_to_gtmpl(json))
            })
            .collect::<anyhow::Result<_>>()?;

        let user_value = json_to_gtmpl(
            serde_json::to_value(user).context("serialize User to JSON")?,
        );

        let params_value = {
            let map: HashMap<String, gtmpl::Value> = self
                .config
                .params
                .iter()
                .map(|(k, v)| (k.clone(), gtmpl::Value::String(v.clone())))
                .collect();
            gtmpl::Value::Map(map)
        };

        let mut ctx: HashMap<String, gtmpl::Value> = HashMap::new();
        ctx.insert(
            "SiteName".into(),
            gtmpl::Value::String(self.config.site_name.clone()),
        );
        ctx.insert(
            "SubscribeName".into(),
            gtmpl::Value::String(self.config.subscribe_name.clone()),
        );
        ctx.insert(
            "OutputFormat".into(),
            gtmpl::Value::String(self.config.output_format.clone()),
        );
        ctx.insert("Proxies".into(), gtmpl::Value::Array(proxy_values));
        ctx.insert("UserInfo".into(), user_value);
        ctx.insert("Params".into(), params_value);

        let rendered = tmpl
            .render(&gtmpl::Context::from(gtmpl::Value::Map(ctx)))
            .map_err(|e| anyhow::anyhow!("template render error: {e}"))?;

        if self.config.output_format == "base64" {
            return Ok(B64.encode(rendered.as_bytes()));
        }

        Ok(rendered)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// JSON ↔ gtmpl::Value conversion
// ─────────────────────────────────────────────────────────────────────────────

fn json_to_gtmpl(v: serde_json::Value) -> gtmpl::Value {
    match v {
        serde_json::Value::Null => gtmpl::Value::Nil,
        serde_json::Value::Bool(b) => gtmpl::Value::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                gtmpl::Value::Number(gtmpl_value::Number::from(i))
            } else if let Some(f) = n.as_f64() {
                gtmpl::Value::Number(gtmpl_value::Number::from(f))
            } else {
                gtmpl::Value::Number(gtmpl_value::Number::from(0_i64))
            }
        }
        serde_json::Value::String(s) => gtmpl::Value::String(s),
        serde_json::Value::Array(arr) => {
            gtmpl::Value::Array(arr.into_iter().map(json_to_gtmpl).collect())
        }
        serde_json::Value::Object(map) => {
            let m: HashMap<String, gtmpl::Value> =
                map.into_iter().map(|(k, v)| (k, json_to_gtmpl(v))).collect();
            gtmpl::Value::Map(m)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Sprig-subset template functions
// ─────────────────────────────────────────────────────────────────────────────

/// `toJson` — serialises first argument to a JSON string.
fn sprig_to_json(args: &[gtmpl::Value]) -> Result<gtmpl::Value, gtmpl_value::FuncError> {
    let v = args
        .first()
        .ok_or_else(|| gtmpl_value::FuncError::AtLeastXArgs("toJson".into(), 1))?;

    // Convert gtmpl::Value back through serde to produce JSON.
    let json_val = gtmpl_value_to_json(v.clone());
    let s = serde_json::to_string(&json_val).unwrap_or_else(|_| "null".into());
    Ok(gtmpl::Value::String(s))
}

/// `b64enc` — base64-encodes first argument as a UTF-8 string.
fn sprig_b64enc(args: &[gtmpl::Value]) -> Result<gtmpl::Value, gtmpl_value::FuncError> {
    let v = args
        .first()
        .ok_or_else(|| gtmpl_value::FuncError::AtLeastXArgs("b64enc".into(), 1))?;
    let s = match v {
        gtmpl::Value::String(s) => s.clone(),
        other => format!("{other:?}"),
    };
    Ok(gtmpl::Value::String(B64.encode(s.as_bytes())))
}

/// `date` — formats a Unix timestamp using a Go-style layout string.
///
/// Signature: `date <layout-string> <unix-timestamp>`
fn sprig_date(args: &[gtmpl::Value]) -> Result<gtmpl::Value, gtmpl_value::FuncError> {
    if args.len() < 2 {
        return Err(gtmpl_value::FuncError::AtLeastXArgs("date".into(), 2));
    }
    let layout = match &args[0] {
        gtmpl::Value::String(s) => s.as_str(),
        _ => {
            return Err(gtmpl_value::FuncError::Generic(
                "date: first arg must be a string layout".into(),
            ))
        }
    };
    let ts: i64 = match &args[1] {
        gtmpl::Value::Number(n) => n
            .as_i64()
            .unwrap_or_else(|| n.as_f64().map(|f| f as i64).unwrap_or(0)),
        _ => {
            return Err(gtmpl_value::FuncError::Generic(
                "date: second arg must be a number (Unix timestamp)".into(),
            ))
        }
    };

    let dt = chrono::Utc
        .timestamp_opt(ts, 0)
        .single()
        .unwrap_or_else(chrono::Utc::now);

    // Map common Go reference-time tokens to chrono format specifiers.
    let chrono_fmt = go_layout_to_chrono(layout);
    Ok(gtmpl::Value::String(dt.format(&chrono_fmt).to_string()))
}

/// Translate a Go time-layout string to a chrono format string.
///
/// Only the most common reference-time tokens are mapped.
fn go_layout_to_chrono(layout: &str) -> String {
    layout
        .replace("2006", "%Y")
        .replace("01", "%m")
        .replace("02", "%d")
        .replace("15", "%H")
        .replace("04", "%M")
        .replace("05", "%S")
        .replace("Jan", "%b")
        .replace("Monday", "%A")
        .replace("Mon", "%a")
}

/// Convert a `gtmpl::Value` to a `serde_json::Value` (best-effort).
fn gtmpl_value_to_json(v: gtmpl::Value) -> serde_json::Value {
    match v {
        gtmpl::Value::Nil | gtmpl::Value::NoValue => serde_json::Value::Null,
        gtmpl::Value::Bool(b) => serde_json::Value::Bool(b),
        gtmpl::Value::String(s) => serde_json::Value::String(s),
        gtmpl::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                serde_json::Value::Number(i.into())
            } else if let Some(f) = n.as_f64() {
                serde_json::Number::from_f64(f)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            }
        }
        gtmpl::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(gtmpl_value_to_json).collect())
        }
        gtmpl::Value::Map(map) | gtmpl::Value::Object(map) => {
            let obj: serde_json::Map<String, serde_json::Value> = map
                .into_iter()
                .map(|(k, v)| (k, gtmpl_value_to_json(v)))
                .collect();
            serde_json::Value::Object(obj)
        }
        // Functions have no meaningful JSON representation.
        gtmpl::Value::Function(_) => serde_json::Value::Null,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Adapter
// ─────────────────────────────────────────────────────────────────────────────

/// Converts node+server entities into a sorted list of [`Proxy`] values.
pub struct Adapter;

impl Adapter {
    /// Build a `Vec<Proxy>` from `(Node, Server)` pairs.
    ///
    /// Mirrors Go `(*Adapter).Proxies()`.
    pub fn proxies(pairs: &[(Node, Server)]) -> Vec<Proxy> {
        let mut out: Vec<Proxy> = Vec::new();

        for (node, server) in pairs {
            // Deserialise the JSON protocols array stored in `server.protocols`.
            let protocols: Vec<NodeProtocol> = match serde_json::from_str(&server.protocols) {
                Ok(v) => v,
                Err(e) => {
                    tracing::error!(
                        server_id = server.id,
                        error = %e,
                        "failed to parse server protocols JSON"
                    );
                    continue;
                }
            };

            // Find the protocol entry whose `type_` matches `node.protocol`.
            let proto = match protocols.iter().find(|p| p.type_ == node.protocol) {
                Some(p) => p,
                None => {
                    tracing::warn!(
                        node_id = node.id,
                        protocol = %node.protocol,
                        "no matching protocol entry in server.protocols"
                    );
                    continue;
                }
            };

            let tags: Vec<String> = if node.tags.is_empty() {
                vec![]
            } else {
                node.tags.split(',').map(str::trim).map(String::from).collect()
            };

            out.push(Proxy {
                sort: node.sort,
                name: node.name.clone(),
                server: node.address.clone(),
                port: node.port,
                type_: node.protocol.clone(),
                tags,
                security: proto.security.clone(),
                sni: proto.sni.clone(),
                allow_insecure: proto.allow_insecure,
                fingerprint: proto.fingerprint.clone(),
                reality_server_addr: proto.reality_server_addr.clone(),
                reality_server_port: proto.reality_server_port,
                reality_private_key: proto.reality_private_key.clone(),
                reality_public_key: proto.reality_public_key.clone(),
                reality_short_id: proto.reality_short_id.clone(),
                transport: proto.transport.clone(),
                host: proto.host.clone(),
                path: proto.path.clone(),
                service_name: proto.service_name.clone(),
                method: proto.cipher.clone(),
                server_key: proto.server_key.clone(),
                uot: proto.uot,
                uot_version: proto.uot_version,
                flow: proto.flow.clone(),
                hop_ports: proto.hop_ports.clone(),
                hop_interval: proto.hop_interval,
                obfs_password: proto.obfs_password.clone(),
                up_mbps: proto.up_mbps,
                down_mbps: proto.down_mbps,
                disable_sni: proto.disable_sni,
                reduce_rtt: proto.reduce_rtt,
                udp_relay_mode: proto.udp_relay_mode.clone(),
                congestion_controller: proto.congestion_controller.clone(),
                padding_scheme: proto.padding_scheme.clone(),
                multiplex: proto.multiplex.clone(),
                xhttp_mode: proto.xhttp_mode.clone(),
                xhttp_extra: proto.xhttp_extra.clone(),
                encryption: proto.encryption.clone(),
                encryption_mode: proto.encryption_mode.clone(),
                encryption_rtt: proto.encryption_rtt.clone(),
                encryption_ticket: proto.encryption_ticket.clone(),
                encryption_server_padding: proto.encryption_server_padding.clone(),
                encryption_private_key: proto.encryption_private_key.clone(),
                encryption_client_padding: proto.encryption_client_padding.clone(),
                encryption_password: proto.encryption_password.clone(),
                ech_enable: proto.ech_enable,
                ech_server_name: proto.ech_server_name.clone(),
                ratio: proto.ratio,
                cert_mode: proto.cert_mode.clone(),
                cert_dns_provider: proto.cert_dns_provider.clone(),
                cert_dns_env: proto.cert_dns_env.clone(),
            });
        }

        // Sort by `node.sort` ascending (mirrors Go slice sort in original code).
        out.sort_by_key(|p| p.sort);
        out
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_default() {
        let _ = Proxy::default();
    }

    #[test]
    fn test_b64enc() {
        let args = vec![gtmpl::Value::String("hello".into())];
        let result = sprig_b64enc(&args).expect("b64enc should succeed");
        assert_eq!(result, gtmpl::Value::String("aGVsbG8=".into()));
    }

    #[test]
    fn test_client_build_simple() {
        let config = ClientConfig {
            site_name: "TestSite".into(),
            subscribe_name: "TestSub".into(),
            output_format: "text".into(),
            params: HashMap::new(),
        };
        let client = Client { config };

        let proxies = vec![Proxy::default(), Proxy::default()];
        let user = User::default();

        // Go template: render the count of proxies.
        let out = client
            .build("{{ len .Proxies }}", &proxies, &user)
            .expect("build should succeed");
        assert_eq!(out.trim(), "2");
    }

    #[test]
    fn test_client_build_base64() {
        let config = ClientConfig {
            output_format: "base64".into(),
            ..Default::default()
        };
        let client = Client { config };
        let out = client
            .build("hello", &[], &User::default())
            .expect("build should succeed");
        // base64("hello") == "aGVsbG8="
        assert_eq!(out, "aGVsbG8=");
    }
}
