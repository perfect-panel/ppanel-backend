# AGENTS.md

`ppanel-backend` 项目约定，供 AI agent 及开发者遵循。

## ppanel-backend

基于 axum 的 Rust 重写项目，Go 源码位于 `../server`。Go 代码库是行为正确性的唯一参考——当 Rust handler 需要对齐 Go 语义时，从对应的 Go 包移植，而非重新设计。

---

## 一、返回值必须通过 `result` crate

所有 HTTP handler 的响应都必须通过 `result` crate 生成。**禁止**在 handler 中手写 `axum` 响应结构体、自行封装 JSON 或拼装 `axum::Json`——统一使用已有工具，以保持响应格式与 Go `result` 包一致。

```
crates/result/src/
├── lib.rs              # pub mod code_error / error_code / http_result
├── error_code.rs       # 错误码常量 (SUCCESS, ERROR, INVALID_PARAMS, ...) + map_err_msg / is_code_err
├── code_error.rs       # CodeError (new_err_code / new_err_code_msg / new_err_msg) + STATUS_NOT_MODIFIED
└── http_result.rs      # ResponseSuccessBean / ResponseErrorBean / HttpResult + build/http/param 工具函数
```

### API 一览

`result::http_result`
- `build_http_result(resp, err) -> HttpResult` — 核心构造函数；出错时从 `anyhow::Error` 链中提取
  `CodeError`，找不到则回退为 `ERROR` / `"Internal Server Error"`。始终返回 HTTP 200，业务码放在
  body 的 `code` 字段（与 Go 行为一致）。
- `build_param_error_result(err) -> HttpResult` — HTTP 200，业务码 `INVALID_PARAMS`。
- `HttpResult` 实现了 `IntoResponse`，可直接作为 handler 的返回值。

`result::code_error`
- `CodeError::new_err_code(code)` — 消息由 `map_err_msg` 自动查表。
- `CodeError::new_err_code_msg(code, msg)` — 显式指定 code + 消息。
- `CodeError::new_err_msg(msg)` — 业务码默认为 `ERROR`。

`result::error_code`
- 仅包含命名常量，如 `SUCCESS`、`ERROR`、`INVALID_PARAMS`、`USER_NOT_EXIST`。
  必须使用这些常量，**禁止魔术数字**。

### Handler 用法模板

```rust
use result::code_error::CodeError;
use result::error_code;
use result::http_result::{build_http_result, HttpResult};

pub async fn handler(State(state): State<AppState>, Json(req): Json<Req>) -> HttpResult {
    let res = some_service(req).await
        .map_err(|_| anyhow::Error::new(CodeError::new_err_code(error_code::USER_NOT_EXIST)));
    build_http_result(res.ok(), res.err())
}
```

规则：
- HTTP 状态码固定 200，业务码在 body `code` 字段，禁止用 HTTP 状态码表示业务错误。
- 成功路径 → `Some(data)`；错误路径 → `CodeError` 包装进 `anyhow::Error`。

---

## 二、日志系统（两层）

### 2.1 业务审计日志 — Telemetry facade

**位置**：`src/service/telemetry.rs`

所有业务事件写入 `system_logs` 表，必须通过 `Telemetry` facade，**禁止**在 service 层直接构造 `SystemLog` 并调用 `repos.log.insert()`。

```rust
use crate::service::telemetry::Telemetry;

// 登录成功
Telemetry::login(&repos, user_id, "email", &ip, &user_agent, true).await;

// 注册成功
Telemetry::register(&repos, user_id, "email", &email, &ip, &user_agent).await;

// 余额变动（type_ 用 BALANCE_TYPE_* 常量）
Telemetry::balance(&repos, user_id, BALANCE_TYPE_RECHARGE, amount, Some(order_no), balance).await;
```

**全部 14 种方法**：

| 方法 | LogType | 优先级 |
|------|---------|--------|
| `login` | LOGIN (30) | P0 |
| `register` | REGISTER (31) | P0 |
| `balance` | BALANCE (32) | P1 |
| `commission` | COMMISSION (33) | P1 |
| `gift` | GIFT (34) | P1 |
| `subscribe_access` | SUBSCRIBE (20) | P1 |
| `subscribe_traffic` | SUBSCRIBE_TRAFFIC (21) | P2 |
| `server_traffic` | SERVER_TRAFFIC (22) | P2 |
| `reset_subscribe` | RESET_SUBSCRIBE (23) | P2 |
| `email_message` | EMAIL_MESSAGE (10) | P2 |
| `mobile_message` | MOBILE_MESSAGE (11) | P2 |
| `user_traffic_rank` | USER_TRAFFIC_RANK (40) | P3 |
| `server_traffic_rank` | SERVER_TRAFFIC_RANK (41) | P3 |
| `traffic_stat` | TRAFFIC_STAT (42) | P3 |

子类型常量定义在 `src/model/entity/log.rs`（`BALANCE_TYPE_*`、`COMMISSION_TYPE_*` 等）。

### 2.2 应用操作日志 — tracing

运维/调试日志使用 `tracing::info!` / `tracing::error!`，由 `main.rs` 根据 `LogConfig` 初始化。
请求日志由 `src/middleware/logger_middleware.rs`（`tower-http TraceLayer`）自动完成，**handler 无需额外代码**。

---

## 三、中间件

### 3.1 DeviceContext（设备上下文）

**位置**：`src/middleware/device_middleware.rs`

从 HTTP headers 提取客户端元数据并注入 `Extension<DeviceContext>`，**永不拒绝请求**：

| Header | DeviceContext 字段 |
|--------|-------------------|
| `X-Original-Forwarded-For` / `X-Forwarded-For` | `ip` |
| `User-Agent` | `user_agent` |
| `Identifier` | `identifier` |
| `Login-Type` | `login_type` |

Handler 通过 `Extension(device): Extension<DeviceContext>` 提取，然后覆盖 JSON body 中的对应字段：

```rust
pub async fn user_login(
    State(state): State<AppState>,
    Extension(device): Extension<DeviceContext>,
    Json(mut req): Json<UserLoginRequest>,
) -> HttpResult {
    if !device.ip.is_empty() { req.ip = device.ip; }
    if !device.user_agent.is_empty() { req.user_agent = device.user_agent; }
    if !device.identifier.is_empty() { req.identifier = device.identifier; }
    // ...
}
```

### 3.2 AuthContext（认证上下文）

**位置**：`src/middleware/auth_middleware.rs`

验证 Bearer JWT，校验 Redis session，查询用户状态，注入 `Extension<AuthContext>`。
admin 路由自动检查 `user.is_admin`。

### 3.3 路由分组中间件映射

| 路由前缀 | 中间件 |
|---------|--------|
| `/v1/auth/*` | `device_middleware` |
| `/v1/auth/oauth/*` | 无 |
| `/v1/admin/*` | `auth_middleware` |
| `/v1/public/*` | `auth_middleware` + `device_middleware` |
| `/v1/common/*` | `device_middleware` |
| `/v1/server/*`、`/v1/telegram`、`/v1/subscribe/*` | 无用户认证 |

---

## 四、本地 crate 一览

| crate | 路径 | 说明 |
|-------|------|------|
| `result` | `crates/result` | HTTP 响应信封、错误码 |
| `jwt` | `crates/jwt` | JWT 生成/验证（`Claims::new` / `generate_token` / `validate_token`） |
| `password` | `crates/password` | 密码哈希，移植自 `../server/pkg/tool/encryption.go` |
| `oauth` | `crates/oauth` | OAuth2 封装（Google/Apple via arctic-oauth，Telegram HMAC 验证） |
| `email` | `crates/email` | 邮件发送 |
| `ip` | `crates/ip` | IP 工具 |
| `payment` | `crates/payment` | 支付集成 |

### jwt crate 用法

```rust
// 生成 token（返回 (Claims, expire_seconds)）
let (claims, seconds) = jwt::Claims::new(user_id, session_id, login_type);
let token = jwt::generate_token(&claims, &config.jwt_auth.access_secret)?;

// 验证 token
let claims = jwt::validate_token(&token, &config.jwt_auth.access_secret)?;
```

### password crate 用法

```rust
// 编码（新密码）
let hash = password::encode_password(&plain_text)?;

// 校验（支持 md5 / sha256 / md5salt / sha256salt / default(PBKDF2) / bcrypt）
let ok = password::multi_password_verify(&algo, &salt, &plain, &stored_hash);
```

### oauth crate 用法

```rust
// Google — PKCE 授权 URL
let google = oauth::Google::new(&client_id, &client_secret, &redirect_uri);
let url = google.authorization_url(&state, &["openid", "email", "profile"], &code_verifier);
let tokens = google.validate_authorization_code(&code, &code_verifier).await?;
let info = oauth::OAuthUserInfo::from_google(&tokens)?;

// Apple — 授权 URL（手动构造）+ token 交换
let apple = oauth::Apple::new(&client_id, &team_id, &key_id, &pkcs8_der, &redirect_uri)?;
let tokens = apple.validate_authorization_code(&code).await?;
let info = oauth::OAuthUserInfo::from_apple(&tokens)?;

// Telegram — HMAC 验证 base64 回调
let auth_data = oauth::parse_base64_and_validate(tg_auth_result, bot_token.as_bytes())?;
let info = oauth::OAuthUserInfo::from_telegram(&auth_data);
```

---

## 五、repository 层约定

- 所有 repo 方法均通过 trait 对象调用（`Box<dyn XxxRepo>`），dialect-agnostic。
- `find_one_by_method`（AuthRepo）返回 `Result<Auth, sqlx::Error>`，**不是** `Option`——找不到时返回 `sqlx::Error::RowNotFound`。
- 动态 SQL 字符串必须用 `repository::audit(sql)` 包装（`sqlx::AssertSqlSafe`）。

---

## 六、移植进度（截至 2026-07-05）

| 模块 | 状态 |
|------|------|
| 日志系统（Telemetry + tracing + TraceLayer） | ✅ 完成 |
| 中间件（auth / device / logger） | ✅ 完成 |
| 中间件（cors / notify / server / pan_domain / trace） | ✅ 完成 |
| crates/sms（AlibabaCloud / Twilio / SmsBao / Abosend） | ✅ 完成 |
| src/adapter（gtmpl 模板引擎 + Proxy/Client/Adapter） | ✅ 完成 |
| service/auth（login / register / reset / device / telephone） | ✅ 完成 |
| service/auth/oauth（Google / Apple / Telegram） | ✅ 完成 |
| service/common（heartbeat / globalConfig / stat / client / ads / privacy / tos） | ✅ 完成 |
| service/server（getConfig / getUserList / pushStatus / pushTraffic / pushOnline / queryProtocol） | ✅ 完成 |
| service/nodeconfig（GlobalValues / ApplyOverride / OverrideResponse / OverrideModel / CloneValues） | ✅ 完成 |
| service/subscribe（subscribeLogic + userAgent UA 匹配） | ✅ 完成 |
| service/notify（Alipay RSA2 / ePay MD5 / Stripe webhook） | ✅ 完成 |
| service/telegram（bot / template / telegram_service） | ✅ 完成 |
| service/admin/ads | ✅ 完成 |
| service/admin/announcement | ✅ 完成 |
| service/admin/document | ✅ 完成 |
| service/admin/coupon | ✅ 完成 |
| service/admin/payment | ✅ 完成 |
| service/admin/auth_method | ✅ 完成 |
| service/admin/application（含 adapter 模板预览） | ✅ 完成 |
| service/admin/console | ✅ 完成 |
| service/admin/tool | ✅ 完成 |
| service/admin/marketing（批量邮件 / quota 任务） | ✅ 完成 |
| service/admin/order | ✅ 完成 |
| service/admin/ticket | ✅ 完成 |
| service/admin/log（全部 14 种日志类型） | ✅ 完成 |
| service/admin/server（节点 / 服务器 CRUD + 协议配置） | ✅ 完成 |
| service/admin/subscribe（订阅计划 CRUD + 排序） | ✅ 完成 |
| service/admin/system（全部 26 个配置读写） | ✅ 完成 |
| service/admin/user（全部 28 个用户管理操作） | ✅ 完成 |
| service/public/announcement | ✅ 完成 |
| service/public/document | ✅ 完成 |
| service/public/payment | ✅ 完成 |
| service/public/subscribe | ✅ 完成 |
| service/public/ticket | ✅ 完成 |
| service/public/portal（购买流程） | ✅ 完成 |
| service/public/order（全部 12 个订单操作） | ✅ 完成 |
| service/public/user（全部 30 个用户自助操作） | ✅ 完成 |
| queue/service（email / sms / order / traffic / subscription / task） | ✅ 完成 |
| scheduler（4 个定时任务注册） | ✅ 完成 |
| handler/auth（所有端点，DeviceContext 注入） | ✅ 完成 |
| handler/common（全部 10 个端点） | ✅ 完成 |
| handler/server（全部 7 个端点） | ✅ 完成 |
| handler/admin（全部子域，~150 个端点） | ✅ 完成 |
| handler/public（全部子域，~90 个端点） | ✅ 完成 |
| handler/subscribe（泛域名订阅） | ✅ 完成 |
| handler/notify（Alipay / ePay / Stripe 回调） | ✅ 完成 |
| handler/telegram | ✅ 完成 |
| routes.rs（按分组应用中间件） | ✅ 完成 |
| repository 层（全部 16 个域） | ✅ 完成 |
| plugin API | ⏳ 暂不实现（已确认跳过） |

### 编译状态

`cargo check` → **0 errors**（截至 2026-07-05）

### 已知 TODO 项（功能存根）

- plugin 相关 handler：已确认暂不实现

