# Contributing to mise-zed

## Prerequisites

- **Rust** 1.94.0 (managed via `mise` — see `mise.toml`)
- **wasm32-wasip2 target**: `rustup target add wasm32-wasip2`
- **mise** CLI: https://mise.jdx.dev
- **Zed** editor (for local extension testing)

## Setup

```sh
mise install
```

## Dev Commands

All tasks are in `mise.toml`. Run them with `mise run <task>`:

| Command          | Description                                                 |
| ---------------- | ----------------------------------------------------------- |
| `mise run build` | Build WASM extension (`cargo build --target wasm32-wasip2`) |
| `mise run check` | Check compilation                                           |
| `mise run lint`  | Clippy with `-D warnings`                                   |
| `mise run fmt`   | Format Rust code                                            |
| `mise run clean` | Clean build artifacts                                       |

## Structure

- `src/lib.rs` — extension entrypoint, implements `zed::Extension` with 4 slash commands
- `extension.toml` — Zed extension manifest
- `grammars/` — TextMate JSON grammars for TOML, shell, Usage KDL, and Tera templates
- `snippets/` — Zed snippets for mise tasks

## Testing the Extension Locally

1. `mise run build` — builds the WASM extension
2. In Zed, open Extensions (`cmd+shift+X`) → **Install Dev Extension** → select the project directory
3. The extension appears with a "Dev" badge

## Guidelines

- Run `mise run fmt` then `mise run lint` before committing
- Keep `src/lib.rs` focused — this is a single-file WASM plugin
- Grammar files are plain JSON (TextMate format). No tree-sitter.
- No tests exist yet. Add them if you add non-trivial logic.
