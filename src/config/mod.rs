pub mod cache_key;

use serde::Deserialize;
use std::path::Path;

// ═══════════════════════════════════════════════════════════════════════════
//  Top-level Config
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    #[serde(default = "default_model")]
    pub model: String,

    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default)]
    pub debug: bool,

    #[serde(default)]
    pub transport: TransportConfig,

    #[serde(default)]
    pub tls: Tls,

    #[serde(rename = "JwtAuth")]
    pub jwt_auth: JwtAuth,

    #[serde(default)]
    pub logger: LogConfig,

    #[serde(default)]
    pub database: DatabaseConfig,

    pub mysql: Option<DatabaseConfig>,

    #[serde(default)]
    pub redis: RedisConfig,

    #[serde(default)]
    pub site: SiteConfig,

    #[serde(default)]
    pub node: NodeConfig,

    #[serde(default)]
    pub mobile: MobileConfig,

    #[serde(default)]
    pub email: EmailConfig,

    #[serde(default)]
    pub device: DeviceConfig,

    #[serde(default)]
    pub verify: Verify,

    #[serde(rename = "VerifyCode")]
    pub verify_code: VerifyCode,

    #[serde(default)]
    pub register: RegisterConfig,

    #[serde(default)]
    pub subscribe: SubscribeConfig,

    #[serde(default)]
    pub invite: InviteConfig,

    #[serde(default)]
    pub telegram: Telegram,

    #[serde(default)]
    pub log: Log,

    #[serde(default)]
    pub currency: Currency,

    #[serde(default)]
    pub plugin: PluginConfig,

    #[serde(default)]
    pub trace: TraceConfig,

    #[serde(default)]
    pub administrator: Administrator,
}

impl Config {
    pub fn load() -> Self {
        let path = std::env::var("PPANEL_CONFIG").unwrap_or_else(|_| "config.yaml".to_string());
        Self::from_file(&path)
    }

    pub fn from_file(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path).unwrap_or_else(|e| {
            panic!("failed to read config file {}: {e}", path.display())
        });
        serde_yaml::from_str(&content).unwrap_or_else(|e| {
            panic!("failed to parse config file {}: {e}", path.display())
        })
    }

    pub fn database_config(&self) -> &DatabaseConfig {
        if self.database.addr.is_some() || !self.database.dbname.is_empty() {
            &self.database
        } else if let Some(ref mysql) = self.mysql {
            mysql
        } else {
            &self.database
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
//  Default helpers
// ═══════════════════════════════════════════════════════════════════════════

fn default_model() -> String { "prod".into() }
fn default_host() -> String { "0.0.0.0".into() }
fn default_port() -> u16 { 8080 }

// ═══════════════════════════════════════════════════════════════════════════
//  Sub-config structs
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RedisConfig {
    #[serde(default = "default_redis_host")]
    pub host: String,
    #[serde(default)]
    pub pass: String,
    #[serde(default)]
    pub db: i32,
}

fn default_redis_host() -> String { "localhost:6379".into() }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransportConfig {
    #[serde(default = "default_transport_driver")]
    pub driver: String,
}

fn default_transport_driver() -> String { "hertz".into() }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JwtAuth {
    #[serde(default)]
    pub access_secret: String,
    #[serde(default = "default_access_expire")]
    pub access_expire: i64,
}

fn default_access_expire() -> i64 { 604800 }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Verify {
    #[serde(default)]
    pub turnstile_site_key: String,
    #[serde(default)]
    pub turnstile_secret: String,
    #[serde(default)]
    pub login_verify: bool,
    #[serde(default)]
    pub register_verify: bool,
    #[serde(default)]
    pub reset_password_verify: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubscribeConfig {
    #[serde(default)]
    pub single_model: bool,
    #[serde(default = "default_subscribe_path")]
    pub subscribe_path: String,
    #[serde(default)]
    pub subscribe_domain: String,
    #[serde(default)]
    pub pan_domain: bool,
    #[serde(default)]
    pub user_agent_limit: bool,
    #[serde(default)]
    pub user_agent_list: String,
    #[serde(default = "default_show_tutorial")]
    pub show_tutorial: bool,
}

fn default_subscribe_path() -> String { "/v1/subscribe/config".into() }
fn default_show_tutorial() -> bool { true }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegisterConfig {
    #[serde(default)]
    pub stop_register: bool,
    #[serde(default)]
    pub enable_trial: bool,
    #[serde(default)]
    pub trial_subscribe: i64,
    #[serde(default)]
    pub trial_time: i64,
    #[serde(default)]
    pub trial_time_unit: String,
    #[serde(default)]
    pub ip_register_limit: i64,
    #[serde(default)]
    pub ip_register_limit_duration: i64,
    #[serde(default)]
    pub enable_ip_register_limit: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailConfig {
    #[serde(rename = "Enable", default = "default_email_enable")]
    pub enable: bool,
    #[serde(default)]
    pub platform: String,
    #[serde(default)]
    pub platform_config: String,
    #[serde(default)]
    pub enable_verify: bool,
    #[serde(default)]
    pub enable_notify: bool,
    #[serde(default)]
    pub enable_domain_suffix: bool,
    #[serde(default)]
    pub domain_suffix_list: String,
    #[serde(default)]
    pub verify_email_template: String,
    #[serde(default)]
    pub expiration_email_template: String,
    #[serde(default)]
    pub maintenance_email_template: String,
    #[serde(default)]
    pub traffic_exceed_email_template: String,
}

fn default_email_enable() -> bool { true }

#[derive(Debug, Clone, Deserialize)]
pub struct MobileConfig {
    #[serde(rename = "Enable", default = "default_mobile_enable")]
    pub enable: bool,
    #[serde(default)]
    pub platform: String,
    #[serde(default)]
    pub platform_config: String,
    #[serde(default)]
    pub enable_verify: bool,
    #[serde(default)]
    pub enable_whitelist: bool,
    #[serde(default)]
    pub whitelist: Vec<String>,
}

fn default_mobile_enable() -> bool { true }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeviceConfig {
    #[serde(default = "default_device_enable")]
    pub enable: bool,
    #[serde(default)]
    pub show_ads: bool,
    #[serde(default)]
    pub enable_security: bool,
    #[serde(default)]
    pub only_real_device: bool,
    #[serde(default)]
    pub security_secret: String,
}

fn default_device_enable() -> bool { true }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SiteConfig {
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub site_name: String,
    #[serde(default)]
    pub site_desc: String,
    #[serde(default)]
    pub site_logo: String,
    #[serde(default)]
    pub keywords: String,
    #[serde(default)]
    pub custom_html: String,
    #[serde(default)]
    pub custom_data: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeConfig {
    #[serde(default)]
    pub node_secret: String,
    #[serde(default = "default_node_pull_interval")]
    pub node_pull_interval: i64,
    #[serde(default = "default_node_push_interval")]
    pub node_push_interval: i64,
    #[serde(default)]
    pub traffic_report_threshold: i64,
    #[serde(default)]
    pub ip_strategy: String,
    #[serde(default)]
    pub dns: Vec<NodeDns>,
    #[serde(default)]
    pub block: Vec<String>,
    #[serde(default)]
    pub outbound: Vec<NodeOutbound>,
}

fn default_node_pull_interval() -> i64 { 60 }
fn default_node_push_interval() -> i64 { 60 }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NodeDns {
    pub proto: String,
    pub address: String,
    #[serde(default)]
    pub domains: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NodeOutbound {
    pub name: String,
    pub protocol: String,
    pub address: String,
    pub port: i64,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub cipher: String,
    #[serde(default)]
    pub security: String,
    #[serde(default)]
    pub sni: String,
    #[serde(default)]
    pub allow_insecure: bool,
    #[serde(default)]
    pub fingerprint: String,
    #[serde(default)]
    pub transport: String,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub service_name: String,
    #[serde(default)]
    pub flow: String,
    #[serde(default)]
    pub uot: bool,
    #[serde(default)]
    pub uot_version: i32,
    #[serde(default)]
    pub congestion_controller: String,
    #[serde(default)]
    pub udp_stream: bool,
    #[serde(default)]
    pub reduce_rtt: bool,
    #[serde(default)]
    pub heartbeat: i32,
    #[serde(default)]
    pub reality_public_key: String,
    #[serde(default)]
    pub reality_short_id: String,
    #[serde(default)]
    pub spider_x: String,
    #[serde(default)]
    pub settings: String,
    #[serde(default)]
    pub stream_settings: String,
    #[serde(default)]
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InviteConfig {
    #[serde(default)]
    pub forced_invite: bool,
    #[serde(default)]
    pub referral_percentage: i64,
    #[serde(default)]
    pub only_first_purchase: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Telegram {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub bot_id: i64,
    #[serde(default)]
    pub bot_name: String,
    #[serde(default)]
    pub bot_token: String,
    #[serde(default)]
    pub enable_notify: bool,
    #[serde(default)]
    pub web_hook_domain: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tls {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub cert_file: String,
    #[serde(default)]
    pub key_file: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VerifyCode {
    #[serde(default = "default_verify_code_expire")]
    pub expire_time: i64,
    #[serde(default = "default_verify_code_limit")]
    pub limit: i64,
    #[serde(default = "default_verify_code_interval")]
    pub interval: i64,
}

fn default_verify_code_expire() -> i64 { 300 }
fn default_verify_code_limit() -> i64 { 15 }
fn default_verify_code_interval() -> i64 { 60 }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Log {
    #[serde(default = "default_log_auto_clear")]
    pub auto_clear: bool,
    #[serde(default = "default_log_clear_days")]
    pub clear_days: i64,
}

fn default_log_auto_clear() -> bool { true }
fn default_log_clear_days() -> i64 { 7 }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Currency {
    #[serde(default = "default_currency_unit")]
    pub unit: String,
    #[serde(default = "default_currency_symbol")]
    pub symbol: String,
    #[serde(default)]
    pub access_key: String,
}

fn default_currency_unit() -> String { "CNY".into() }
fn default_currency_symbol() -> String { "¥".into() }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PluginConfig {
    #[serde(default = "default_plugin_enabled")]
    pub enabled: bool,
    #[serde(default = "default_plugin_directory")]
    pub directory: String,
    #[serde(default = "default_plugin_max_memory")]
    pub max_memory_mb: i64,
    #[serde(default = "default_plugin_timeout")]
    pub timeout_sec: i64,
    #[serde(default)]
    pub allow_list: Vec<String>,
    #[serde(default)]
    pub block_list: Vec<String>,
}

fn default_plugin_enabled() -> bool { true }
fn default_plugin_directory() -> String { "plugins".into() }
fn default_plugin_max_memory() -> i64 { 64 }
fn default_plugin_timeout() -> i64 { 30 }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Administrator {
    #[serde(default = "default_admin_email")]
    pub email: String,
    #[serde(default = "default_admin_password")]
    pub password: String,
}

fn default_admin_email() -> String { "admin@ppanel.dev".into() }
fn default_admin_password() -> String { "password".into() }

// ─── Trace / OpenTelemetry ───────────────────────────────────────────────────

/// Mirrors `pkg/trace/config.go → Config`.
///
/// Supported batcher values: `"jaeger"` | `"zipkin"` | `"otlpgrpc"` | `"otlphttp"` | `"stdout"`
/// Leave `endpoint` empty (or set `disabled = true`) to disable tracing.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TraceConfig {
    /// Service name reported to the tracing backend (default "ppanel").
    #[serde(default = "default_trace_name")]
    pub name: String,

    /// Exporter endpoint URL (e.g. `http://localhost:14268/api/traces` for Jaeger HTTP).
    #[serde(default)]
    pub endpoint: String,

    /// Fraction of traces to sample, 0.0–1.0 (default 1.0 = 100 %).
    #[serde(default = "default_trace_sampler")]
    pub sampler: f64,

    /// Exporter backend: `jaeger` | `otlpgrpc` | `otlphttp` | `stdout`.
    #[serde(default = "default_trace_batcher")]
    pub batcher: String,

    /// Extra headers forwarded to the OTLP exporter.
    #[serde(default)]
    pub otlp_headers: std::collections::HashMap<String, String>,

    /// URL path override for OTLP HTTP (e.g. `"/v1/traces"`).
    #[serde(default)]
    pub otlp_http_path: String,

    /// Use TLS for OTLP HTTP transport.
    #[serde(default)]
    pub otlp_http_secure: bool,

    /// Disable tracing entirely (shortcut to skip initialisation).
    #[serde(default)]
    pub disabled: bool,
}

fn default_trace_name() -> String { "ppanel".into() }
fn default_trace_sampler() -> f64 { 1.0 }
fn default_trace_batcher() -> String { "stdout".into() }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LogConfig {
    #[serde(default = "default_log_service_name")]
    pub service_name: String,
    #[serde(default = "default_log_mode")]
    pub mode: String,
    #[serde(default = "default_log_encoding")]
    pub encoding: String,
    #[serde(default = "default_log_time_format")]
    pub time_format: String,
    #[serde(default = "default_log_path")]
    pub path: String,
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default)]
    pub max_content_length: u32,
    #[serde(default)]
    pub compress: bool,
    #[serde(default = "default_log_stat")]
    pub stat: bool,
    #[serde(default)]
    pub keep_days: i32,
    #[serde(default = "default_log_stack_cooldown")]
    pub stack_cooldown_millis: i32,
    #[serde(default)]
    pub max_backups: i32,
    #[serde(default)]
    pub max_size: i32,
    #[serde(default = "default_log_rotation")]
    pub rotation: String,
    #[serde(default = "default_log_file_time_format")]
    pub file_time_format: String,
}

fn default_log_service_name() -> String { "PPanel".into() }
fn default_log_mode() -> String { "file".into() }
fn default_log_encoding() -> String { "json".into() }
fn default_log_time_format() -> String { "2006-01-02 15:04:05.000".into() }
fn default_log_path() -> String { "logs".into() }
fn default_log_level() -> String { "info".into() }
fn default_log_stat() -> bool { true }
fn default_log_stack_cooldown() -> i32 { 100 }
fn default_log_rotation() -> String { "daily".into() }
fn default_log_file_time_format() -> String { "2006-01-02T15:04:05.000Z07:00".into() }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DatabaseConfig {
    #[serde(default = "default_db_driver")]
    pub driver: String,
    #[serde(default)]
    pub addr: Option<String>,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub dbname: String,
    #[serde(default = "default_db_config")]
    pub config: String,
    #[serde(default = "default_db_max_idle")]
    pub max_idle_conns: i32,
    #[serde(default = "default_db_max_open")]
    pub max_open_conns: i32,
    #[serde(default = "default_db_slow_threshold")]
    pub slow_threshold: i64,
}

fn default_db_driver() -> String { "mysql".into() }
fn default_db_config() -> String { "charset=utf8mb4&parseTime=true&loc=Asia%2FShanghai".into() }
fn default_db_max_idle() -> i32 { 10 }
fn default_db_max_open() -> i32 { 10 }
fn default_db_slow_threshold() -> i64 { 1000 }

// ═══════════════════════════════════════════════════════════════════════════
//  Default trait — enables #[serde(default)] on all optional fields
// ═══════════════════════════════════════════════════════════════════════════

macro_rules! impl_default {
    ($ty:ty { $($field:ident: $val:expr),* $(,)? }) => {
        impl Default for $ty {
            fn default() -> Self {
                Self { $($field: $val),* }
            }
        }
    };
}

impl_default!(Config {
    model: default_model(),
    host: default_host(),
    port: default_port(),
    debug: false,
    transport: TransportConfig::default(),
    tls: Tls::default(),
    jwt_auth: JwtAuth::default(),
    logger: LogConfig::default(),
    database: DatabaseConfig::default(),
    mysql: None,
    redis: RedisConfig::default(),
    site: SiteConfig::default(),
    node: NodeConfig::default(),
    mobile: MobileConfig::default(),
    email: EmailConfig::default(),
    device: DeviceConfig::default(),
    verify: Verify::default(),
    verify_code: VerifyCode::default(),
    register: RegisterConfig::default(),
    subscribe: SubscribeConfig::default(),
    invite: InviteConfig::default(),
    telegram: Telegram::default(),
    log: Log::default(),
    currency: Currency::default(),
    plugin: PluginConfig::default(),
    trace: TraceConfig::default(),
    administrator: Administrator::default(),
});

impl_default!(RedisConfig { host: default_redis_host(), pass: String::new(), db: 0 });
impl_default!(TransportConfig { driver: default_transport_driver() });
impl_default!(JwtAuth { access_secret: String::new(), access_expire: default_access_expire() });
impl_default!(Verify { turnstile_site_key: String::new(), turnstile_secret: String::new(), login_verify: false, register_verify: false, reset_password_verify: false });
impl_default!(SubscribeConfig { single_model: false, subscribe_path: default_subscribe_path(), subscribe_domain: String::new(), pan_domain: false, user_agent_limit: false, user_agent_list: String::new(), show_tutorial: default_show_tutorial() });
impl_default!(RegisterConfig { stop_register: false, enable_trial: false, trial_subscribe: 0, trial_time: 0, trial_time_unit: String::new(), ip_register_limit: 0, ip_register_limit_duration: 0, enable_ip_register_limit: false });
impl_default!(EmailConfig { enable: default_email_enable(), platform: String::new(), platform_config: String::new(), enable_verify: false, enable_notify: false, enable_domain_suffix: false, domain_suffix_list: String::new(), verify_email_template: String::new(), expiration_email_template: String::new(), maintenance_email_template: String::new(), traffic_exceed_email_template: String::new() });
impl_default!(MobileConfig { enable: default_mobile_enable(), platform: String::new(), platform_config: String::new(), enable_verify: false, enable_whitelist: false, whitelist: Vec::new() });
impl_default!(DeviceConfig { enable: default_device_enable(), show_ads: false, enable_security: false, only_real_device: false, security_secret: String::new() });
impl_default!(SiteConfig { host: String::new(), site_name: String::new(), site_desc: String::new(), site_logo: String::new(), keywords: String::new(), custom_html: String::new(), custom_data: String::new() });
impl_default!(NodeConfig { node_secret: String::new(), node_pull_interval: default_node_pull_interval(), node_push_interval: default_node_push_interval(), traffic_report_threshold: 0, ip_strategy: String::new(), dns: Vec::new(), block: Vec::new(), outbound: Vec::new() });
impl_default!(NodeDns { proto: String::new(), address: String::new(), domains: Vec::new() });
impl_default!(NodeOutbound { name: String::new(), protocol: String::new(), address: String::new(), port: 0, user: String::new(), password: String::new(), uuid: String::new(), cipher: String::new(), security: String::new(), sni: String::new(), allow_insecure: false, fingerprint: String::new(), transport: String::new(), host: String::new(), path: String::new(), service_name: String::new(), flow: String::new(), uot: false, uot_version: 0, congestion_controller: String::new(), udp_stream: false, reduce_rtt: false, heartbeat: 0, reality_public_key: String::new(), reality_short_id: String::new(), spider_x: String::new(), settings: String::new(), stream_settings: String::new(), rules: Vec::new() });
impl_default!(InviteConfig { forced_invite: false, referral_percentage: 0, only_first_purchase: false });
impl_default!(Telegram { enable: false, bot_id: 0, bot_name: String::new(), bot_token: String::new(), enable_notify: false, web_hook_domain: String::new() });
impl_default!(Tls { enable: false, cert_file: String::new(), key_file: String::new() });
impl_default!(VerifyCode { expire_time: default_verify_code_expire(), limit: default_verify_code_limit(), interval: default_verify_code_interval() });
impl_default!(Log { auto_clear: default_log_auto_clear(), clear_days: default_log_clear_days() });
impl_default!(Currency { unit: default_currency_unit(), symbol: default_currency_symbol(), access_key: String::new() });
impl_default!(PluginConfig { enabled: default_plugin_enabled(), directory: default_plugin_directory(), max_memory_mb: default_plugin_max_memory(), timeout_sec: default_plugin_timeout(), allow_list: Vec::new(), block_list: Vec::new() });
impl_default!(Administrator { email: default_admin_email(), password: default_admin_password() });
impl_default!(LogConfig {
    service_name: default_log_service_name(),
    mode: default_log_mode(),
    encoding: default_log_encoding(),
    time_format: default_log_time_format(),
    path: default_log_path(),
    level: default_log_level(),
    max_content_length: 0,
    compress: false,
    stat: default_log_stat(),
    keep_days: 0,
    stack_cooldown_millis: default_log_stack_cooldown(),
    max_backups: 0,
    max_size: 0,
    rotation: default_log_rotation(),
    file_time_format: default_log_file_time_format(),
});
impl_default!(DatabaseConfig { driver: default_db_driver(), addr: None, username: String::new(), password: String::new(), dbname: String::new(), config: default_db_config(), max_idle_conns: default_db_max_idle(), max_open_conns: default_db_max_open(), slow_threshold: default_db_slow_threshold() });
