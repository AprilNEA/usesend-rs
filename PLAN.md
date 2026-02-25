# usesend-rs 完整开发计划

## 现状总结

当前已完成的骨架：
- Cargo workspace（`usesend-api` + `usesend`）已搭建，`cargo check` 通过
- `usesend-api`：31 个 endpoint 方法全覆盖，5 组类型模块（domain/email/contact_book/contact/campaign）
- `usesend`：高层封装 + `EmailBuilder` fluent builder
- `StringOrVec` 多态字段处理、`ApiError` 错误枚举

---

## Phase 1：usesend-api 底层 crate 加固

### 1.1 架构重构 → Sub-Service 模式
**目标**：参考 resend-rs，将 `UseSendApiClient` 从一个 God Object 重构为 sub-service 字段模式。

```rust
pub struct UseSendApiClient {
    pub emails: EmailsSvc,
    pub domains: DomainsSvc,
    pub contact_books: ContactBooksSvc,
    pub contacts: ContactsSvc,
    pub campaigns: CampaignsSvc,
}
```

每个 service 是 `struct XxxSvc(pub(crate) Arc<Config>)` 的 newtype wrapper。
- 提取 `Config` struct（持有 `reqwest::Client`、`base_url`、`api_key`）
- 所有 service 共享 `Arc<Config>`
- `UseSendApiClient` 实现 `Clone`

**文件变更**：
- 新增 `usesend-api/src/config.rs`
- 拆分 `usesend-api/src/client.rs` → 各 service 模块内的 impl

### 1.2 类型系统改进

| 改进项 | 说明 |
|--------|------|
| **Typed IDs** | 定义 `DomainId(i64)`、`EmailId(String)`、`ContactBookId(String)`、`ContactId(String)`、`CampaignId(String)` newtype，而不是裸 `i64`/`String` |
| **`latestStatus` nullable** | OpenAPI 里 `latestStatus` 是 `nullable: true`，当前 `EmailListItem.latest_status` 是 `EmailEventStatus` 不支持 null → 改为 `Option<EmailEventStatus>` |
| **`domainId` 类型** | list emails response 里 `domainId` 是 `number, nullable`（不是 `serde_json::Value`）→ 改为 `Option<i64>` |
| **Eq/Hash derive** | 所有 enum 补充 `Eq, Hash` derive |
| **Default 实现** | `ListEmailsParams`、`ListContactsParams`、`ListCampaignsParams` 实现 `Default` |
| **DnsRecord.type** | 当前是 `String`，改为 `enum DnsRecordType { Mx, Txt }` |

### 1.3 Idempotency-Key 统一支持
- Batch send endpoint 同样支持 `Idempotency-Key` header（当前只给 single send 做了）
- 在 service 方法签名上用 `Option<&str>` 参数统一处理，或引入 `Idempotent<T>` wrapper

### 1.4 错误处理增强

```rust
pub enum ApiError {
    Http(#[from] reqwest::Error),
    Api { status: StatusCode, body: ErrorResponse },
    Conflict { message: String },           // 409 idempotency
    RateLimit { retry_after: Option<u64> },  // 429 with header parsing
    Unexpected { status: StatusCode, text: String },
    Deserialize { status: StatusCode, body: String, source: serde_json::Error },
}
```

新增：
- `RateLimit` variant（解析 `ratelimit-reset` header）
- `Deserialize` variant（保留原始 body 方便调试）

---

## Phase 2：usesend 高层 crate 人体工学

### 2.1 Sub-Service 暴露
跟随底层重构，高层 crate 也用 sub-service 字段：

```rust
pub struct UseSend {
    pub emails: Emails,
    pub domains: Domains,
    pub contact_books: ContactBooks,
    pub contacts: Contacts,
    pub campaigns: Campaigns,
}
```

每个模块提供 builder/简化方法。

### 2.2 Builder 覆盖扩展
当前只有 `EmailBuilder`，需要补充：
- `ContactBuilder` — 创建/更新联系人
- `CampaignBuilder` — 创建 campaign
- `ContactBookBuilder` — 创建 contact book

### 2.3 Default 构造 & 环境变量
```rust
// 从 USESEND_API_KEY 环境变量读取
impl Default for UseSend {
    fn default() -> Self { ... }
}
```

### 2.4 分页迭代器
为 list 类 endpoint 提供 `Stream` / `Iterator` 封装：

```rust
let mut stream = client.emails.list().page_size(50).stream();
while let Some(batch) = stream.next().await {
    // ...
}
```

---

## Phase 3：Feature Flags & 跨平台

### 3.1 TLS 选择
```toml
[features]
default = ["native-tls"]
native-tls = ["reqwest/native-tls"]
rustls-tls = ["reqwest/rustls-tls"]
```

### 3.2 Blocking 客户端（可选）
使用 `maybe-async` crate 同时支持 async 和 blocking：
```toml
blocking = ["reqwest/blocking", "maybe-async/is_sync"]
```

### 3.3 WASM 支持
- Feature gate `wasm`：去掉 native-tls，使用 `reqwest` 的 wasm target
- 条件编译 rate limiter（wasm 无 `std::time::Instant`）

---

## Phase 4：健壮性

### 4.1 Rate Limiting（客户端主动限流）
两层策略：
1. **主动限流**：`governor` crate，在 `Config::send()` 前等待令牌
2. **被动重试**：429 响应时读取 `ratelimit-reset` header，sleep 后重试

提供 `RetryOptions` 配置：
```rust
pub struct RetryOptions {
    pub max_retries: u32,        // default: 3
    pub base_delay_ms: u64,      // default: 1000
    pub jitter_range_ms: Range<u64>, // default: 0..30
}
```

### 4.2 Request/Response Logging
可选 `tracing` feature，在请求/响应时发出 span：
```toml
tracing = ["dep:tracing"]
```

---

## Phase 5：文档 & 测试 & 发布

### 5.1 文档
- 每个 pub item 加 `///` doc comment
- Crate-level doc（`//!`）包含 quickstart 示例
- `README.md` with badges（crates.io, docs.rs, CI）
- `CHANGELOG.md`

### 5.2 测试策略
| 层级 | 内容 |
|------|------|
| **单元测试** | `StringOrVec` serde 往返、ID newtype、builder 构建 |
| **集成测试** | Mock server（`wiremock`）验证每个 endpoint 的请求 URL/method/header/body |
| **Doc tests** | 所有 builder 示例可编译 |

### 5.3 CI
- GitHub Actions：`cargo check`、`cargo test`、`cargo clippy`、`cargo fmt --check`
- 矩阵：`stable` + `nightly`，features 组合（`default`, `blocking`, `rustls-tls`）

### 5.4 发布
- `usesend-api` 先发（纯类型 + 底层 client，面向需要精确控制的用户）
- `usesend` 后发（面向普通用户，推荐 crate）
- 版本跟随 useSend API 版本，语义化版本

---

## 执行优先级 & 依赖关系

```
Phase 1.1 (sub-service 重构)
    │
    ├── Phase 1.2 (类型改进) ──── 可并行
    ├── Phase 1.3 (idempotency)
    └── Phase 1.4 (错误增强)
           │
Phase 2 (高层 crate) ← 依赖 Phase 1 完成
    │
Phase 3 (feature flags) ← 可与 Phase 2 并行
    │
Phase 4 (rate limiting) ← 依赖 Phase 1.1 的 Config 架构
    │
Phase 5 (文档/测试/发布) ← 最后
```

---

## 文件结构规划（最终）

```
usesend-rs/
├── Cargo.toml                     # workspace
├── README.md
├── CHANGELOG.md
├── LICENSE
├── usesend-api/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── config.rs              # Config, ConfigBuilder, send()
│       ├── client.rs              # UseSendApiClient (sub-service fields)
│       ├── error.rs               # ApiError, ApiResult
│       ├── types/
│       │   ├── mod.rs             # StringOrVec, shared types, ID newtypes
│       │   ├── domain.rs
│       │   ├── email.rs
│       │   ├── contact_book.rs
│       │   ├── contact.rs
│       │   └── campaign.rs
│       ├── services/
│       │   ├── mod.rs
│       │   ├── domains.rs         # DomainsSvc
│       │   ├── emails.rs          # EmailsSvc
│       │   ├── contact_books.rs   # ContactBooksSvc
│       │   ├── contacts.rs        # ContactsSvc
│       │   └── campaigns.rs       # CampaignsSvc
│       └── rate_limit.rs          # RetryOptions, send_with_retry
├── usesend/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── client.rs              # UseSend (sub-service fields)
│       ├── emails.rs              # Emails service + EmailBuilder
│       ├── domains.rs             # Domains service
│       ├── contact_books.rs       # ContactBooks service
│       ├── contacts.rs            # Contacts service + ContactBuilder
│       └── campaigns.rs           # Campaigns service + CampaignBuilder
└── examples/
    ├── send_email.rs
    ├── batch_send.rs
    ├── manage_domains.rs
    └── contacts_crud.rs
```
