# PPanel Backend (Rust)

<div align="center">

[![License](https://img.shields.io/github/license/perfect-panel/ppanel-backend)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2021_edition-orange)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue)](https://github.com/tokio-rs/axum)
[![sqlx](https://img.shields.io/badge/sqlx-0.9-green)](https://github.com/launchbadge/sqlx)

**PPanel 服务端的 Rust 重写版本 — 基于 Axum + sqlx，同时支持 MySQL / MariaDB 和 PostgreSQL。**

</div>

---

## 📋 概述

`ppanel-backend` 是 [PPanel 服务端 (Go)](https://github.com/perfect-panel/backend) 的 Rust 移植版本，使用 **Axum 0.8 + sqlx 0.9 + Tokio** 构建，与 Go 版本保持完整的 API 兼容性。

### 与 Go 版本的差异与改进

| 维度 | Go 版 | Rust 版 |
|---|---|---|
| HTTP 框架 | Hertz | Axum 0.8 |
| ORM | GORM | sqlx 0.9（编译期 SQL 检查） |
| 异步运行时 | goroutine | Tokio |
| 内存安全 | GC | 所有权系统，无 GC 暂停 |

### 核心特性

- **多协议支持**：Shadowsocks、V2Ray、VLESS、Trojan、Hysteria2、TUIC 等
- **双数据库后端**：MySQL / MariaDB 和 PostgreSQL 共用一套代码，运行时自动检测方言
- **完整 API 兼容**：与 Go 版本接口一一对应，可无缝替换
- **OpenTelemetry 可观测性**：stdout / OTLP gRPC / OTLP HTTP 链路追踪
- **Redis 限流**：验证码发送间隔（60 秒）+ 每日上限（15 次）
- **Cloudflare Turnstile**：登录 / 注册 / 重置密码三处人机验证
- **异步任务队列**：基于 asynq（Redis），兼容 Go 版任务类型键名
- **定时调度器**：订阅检查（60s）、流量重置（00:30）、流量统计（00:00）、汇率更新（01:00）

---

## 🚀 快速开始

### 前提条件

- **Rust** 1.75+（推荐使用 `rustup` 安装）
- **MySQL 8.0+ / MariaDB 10.6+** 或 **PostgreSQL 14+**
- **Redis** 6.0+
- **Git**

### 从源码运行

```bash
# 1. 克隆仓库
git clone https://github.com/perfect-panel/ppanel-backend.git
cd ppanel-backend

# 2. 复制并编辑配置
cp config.example.yaml config.yaml
# 编辑 config.yaml，填写数据库、Redis、JWT 等配置

# 3. 编译并运行
cargo run --release
```

服务默认监听 `0.0.0.0:8080`，`/health` 路由返回 `ok`。

### 配置文件

配置文件路径默认为 `config.yaml`，可通过环境变量 `PPANEL_CONFIG` 覆盖：

```bash
PPANEL_CONFIG=/etc/ppanel/config.yaml cargo run --release
```

最小配置示例（MySQL）：

```yaml
Host: 0.0.0.0
Port: 8080
Model: prod          # prod | dev（dev 模式跳过 Turnstile 验证）

JwtAuth:
  AccessSecret: "your-secret-here"
  AccessExpire: 86400

Database:
  Driver: mysql
  Addr: "127.0.0.1:3306"
  Username: ppanel
  Password: your-password
  DBName: ppanel

Redis:
  Host: "127.0.0.1:6379"

Administrator:
  Email: admin@example.com
  Password: changeme
```

PostgreSQL 只需把 `Driver` 改为 `postgres`，`Addr` 改为 PG 格式即可。

---

## 🗄 数据库迁移

启动时自动执行迁移，无需额外命令。

迁移文件位于：
- `migrations/mysql/`   — MySQL / MariaDB
- `migrations/postgres/` — PostgreSQL

每个版本同时提供 `*.sql`（正向）和 `*.down.sql`（回滚）。

### 从 MySQL 迁移到 PostgreSQL

使用内置工具：

```bash
cargo build --release -p mysql2postgres
./target/release/mysql2postgres \
  --mysql  "user:pass@tcp(127.0.0.1:3306)/ppanel" \
  --postgres "postgres://user:pass@127.0.0.1/ppanel" \
  --dry-run   # 先预览，确认无误后去掉此参数
```

---

## 📁 目录结构

```
ppanel-backend/
├── crates/               # 独立库 crate
│   ├── email/            # 邮件发送
│   ├── ip/               # IP 地理位置
│   ├── jwt/              # JWT 签发 / 验证
│   ├── oauth/            # OAuth2（Google / Apple / Telegram）
│   ├── password/         # 密码哈希（PBKDF2 / bcrypt / MD5 / SHA-256）
│   ├── payment/          # 支付平台（Alipay / ePay / Stripe）
│   ├── result/           # HTTP 统一响应格式与错误码
│   ├── sms/              # 短信（阿里云 / Twilio / SmsBao / AboSend）
│   └── turnstile/        # Cloudflare Turnstile 验证
├── migrations/           # 数据库迁移 SQL
│   ├── mysql/
│   └── postgres/
├── src/
│   ├── adapter/          # 代理协议适配器（生成订阅链接）
│   ├── config/           # 配置结构体
│   ├── handler/          # HTTP 路由与处理器
│   │   ├── admin/        # 管理员 API
│   │   ├── auth/         # 认证 API
│   │   ├── common/       # 公共 API
│   │   ├── notify/       # 支付回调
│   │   ├── public/       # 用户 API
│   │   └── server/       # 节点 API
│   ├── middleware/        # HTTP 中间件
│   ├── model/            # 数据模型（entity + DTO）
│   ├── queue/            # 异步任务队列
│   ├── repository/       # 数据访问层（MySQL + PG 双实现）
│   ├── scheduler/        # 定时任务
│   ├── service/          # 业务逻辑
│   │   ├── admin/
│   │   ├── auth/
│   │   ├── common/
│   │   ├── public/
│   │   ├── server/
│   │   └── telegram/
│   ├── exchange_rate.rs  # 汇率换算工具
│   ├── tracing_otel.rs   # OpenTelemetry 初始化
│   └── main.rs
├── tools/
│   └── mysql2postgres/   # MySQL → PostgreSQL 数据迁移工具
├── Cargo.toml
└── config.example.yaml
```

---

## 🔗 API 兼容性

本版本与 Go 服务端 API 完全兼容，可直接对接：

- **前端**：[PPanel Web](https://github.com/perfect-panel/frontend)
- **用户界面预览**：[user.ppanel.dev](https://user.ppanel.dev)
- **管理界面预览**：[admin.ppanel.dev](https://admin.ppanel.dev)
- **Swagger 文档**：[ppanel.dev/zh-CN/swagger/ppanel](https://ppanel.dev/zh-CN/swagger/ppanel)

---

## 🧪 开发

### 运行测试

```bash
cargo test
```

### 代码检查

```bash
cargo check
cargo clippy
```

### 构建发布版

```bash
cargo build --release
# 产物：./target/release/ppanel-backend
```

### OpenTelemetry 链路追踪

在配置文件中添加：

```yaml
Trace:
  Name: ppanel
  Batcher: stdout          # stdout | otlpgrpc | otlphttp
  Endpoint: ""             # 留空 + stdout 时输出到控制台
  Sampler: 1.0
  Disabled: false
```

---

## 🤝 贡献

欢迎 PR 和 Issue。移植工作参考 Go 原版 (`../server`)，Rust 实现遵循 `AGENTS.md` 中记录的约定。

## 📄 许可证

本项目采用 [Apache License 2.0](LICENSE) 授权。

## Star History

<a href="https://www.star-history.com/?repos=perfect-panel%2Fppanel-backend&type=timeline&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=perfect-panel/ppanel-backend&type=timeline&theme=dark&logscale&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=perfect-panel/ppanel-backend&type=timeline&logscale&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=perfect-panel/ppanel-backend&type=timeline&logscale&legend=top-left" />
 </picture>
</a>
