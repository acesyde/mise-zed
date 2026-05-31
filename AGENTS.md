# mise-zed — Agent Guide

## Overview

Zed extension for [mise](https://mise.jdx.dev). Single-file WASM plugin at `src/lib.rs`.

## Build & Dev Commands

| Command          | Purpose                                           |
| ---------------- | ------------------------------------------------- |
| `mise run build` | WASM build (`cargo build --target wasm32-wasip2`) |
| `mise run check` | `cargo check`                                     |
| `mise run lint`  | `cargo clippy -- -D warnings`                     |
| `mise run fmt`   | `cargo fmt`                                       |

All dev tasks are in `mise.toml` — use `mise run <task>`, not raw cargo directly.

## Architecture

- **Crate type**: `cdylib` targeting `wasm32-wasip2`.
- **Entrypoint**: `src/lib.rs` — minimal extension skeleton.
- **Snippets**:
  - `snippets/toml-tasks.json` — TOML task snippets
  - `snippets/shell-tasks.json` — Shell `#MISE` file task config snippet

## Key Dependencies

- `zed_extension_api = "0.7.0"` — Zed's WASM extension SDK

## Important Notes

- **No tests exist** yet. No CI workflows configured.
- **Rust edition**: 2021. Toolchain: `rust 1.94.0` (pinned in `mise.toml`)
- Extension is declared in `extension.toml` (name: `mise`, version: `0.1.0`).
