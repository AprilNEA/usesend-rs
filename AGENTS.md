# AGENTS.md

## Build & Test
- `cargo check --all` — type-check both crates
- `cargo test --all` — run all 18 unit tests + doc-tests
- `cargo test -p usesend-api -- <test_name>` — run a single test in usesend-api
- `cargo clippy --all --all-targets -- -D warnings` — lint (must pass with zero warnings)
- `cargo fmt --all -- --check` — format check

## Architecture
Cargo workspace with two crates:
- **`usesend-api`** — Low-level typed HTTP client. `Config` (with `Arc`-shared state, governor rate limiter) → `SharedConfig` → service structs (`EmailsSvc`, `DomainsSvc`, etc.). Types live in `usesend-api/src/types/`.
- **`usesend`** — High-level wrapper re-exporting `usesend_api::types` and wrapping each `*Svc` in thin service structs (`Emails`, `Domains`, etc.).

## Code Style
- Rust edition 2024, MSRV 1.88. Default TLS: `rustls-tls`.
- All request/params structs derive `bon::Builder` + `Serialize`/`Deserialize`. Use `#[builder(into)]` on String/StringOrVec fields.
- Optional fields: `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`.
- Typed IDs via `define_id_type!` macro (`DomainId`, `EmailId`, etc.) in `types/mod.rs`.
- `serde(rename_all = "camelCase")` on API-facing structs. Enums use `SCREAMING_SNAKE_CASE`.
- Imports: `use crate::...` internally; group by crate, alphabetical. No `use serde_json;` bare imports.
- Error handling: `ApiError` enum with `thiserror`, return `ApiResult<T>`. No `.unwrap()` in library code.
- License: `MIT OR Apache-2.0`. No comments unless code is complex.
