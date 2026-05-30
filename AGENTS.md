# mise-zed — Agent Guide

## Overview

Zed extension for [mise](https://mise.jdx.dev) (dev-tools, tasks, env vars). Single-file WASM plugin at `src/lib.rs`.

## Build & Dev Commands

| Command          | Purpose                                           |
| ---------------- | ------------------------------------------------- |
| `mise run build` | WASM build (`cargo build --target wasm32-wasip1`) |
| `mise run check` | `cargo check`                                     |
| `mise run lint`  | `cargo clippy -- -D warnings`                     |
| `mise run fmt`   | `cargo fmt`                                       |

All dev tasks are in `mise.toml` — use `mise run <task>`, not raw cargo directly.

## Architecture

- **Crate type**: `cdylib` targeting `wasm32-wasip1`. The extension runs inside Zed's WASM runtime.
- **Entrypoint**: `src/lib.rs` (~150 loc). Implements `zed::Extension` trait with 4 slash commands:
  - `/tools` — `mise ls --offline --json`
  - `/tasks` — `mise tasks ls --json`
  - `/env` — `mise env --json`
  - `/doctor` — `mise doctor`
- **Grammars** (TextMate `.json`, NOT tree-sitter):
  - `grammars/toml/` — TOML syntax (MIT, adapted from TOMBI)
  - `grammars/mise_shell/` — Shell syntax for mise file tasks (MIT, from vscode Shell Script)
  - `grammars/usage_kdl/` — Usage KDL config (from vscode-kdl)
  - `grammars/tera-mise.json` — Tera template injection grammar for mise strings
- **Snippets**:
  - `snippets/toml-tasks.json` — TOML task snippets (`task`, `task_array`, `task_script`, etc.)
  - `snippets/shell-tasks.json` — Shell `#MISE` file task config snippet

## Key Dependencies

- `zed_extension_api = "0.7.0"` — Zed's WASM extension SDK
- `serde` / `serde_json` — JSON parsing for `mise` CLI output

## Important Notes

- **No tests exist** yet. No CI workflows configured.
- **Rust edition**: 2021. Toolchain: `rust 1.94.0` (pinned in `mise.toml`)
- **No rustfmt/clippy config** — uses defaults.
- Extension is declared in `extension.toml` (name: `mise`, version: `0.1.0`). This is the Zed extension manifest.
- The `mise` binary must be on PATH at runtime — the extension calls it via `zed::process::Command`.
- Grammar files are plain JSON (TextMate format), not compiled WASM. Zed loads them as language grammars directly.
