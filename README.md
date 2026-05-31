# mise-zed 🛠️

A [Zed](https://zed.dev) extension for [mise](https://mise.jdx.dev/) (dev-tools, tasks, and environment variables).

This extension is heavily inspired by the excellent [mise-vscode](https://github.com/hverlin/mise-vscode) extension by [hverlin](https://github.com/hverlin) — many thanks for his work!

> [mise](https://mise.jdx.dev/) is a polyglot tool version manager, environment variables manager, and tasks runner.
>
> - Like asdf (or nvm/pyenv for any language), it manages dev tools like node, python, cmake, terraform, and hundreds more.
> - Like direnv, it manages environment variables for different project directories.
> - Like make, it manages tasks used to build and test projects.

## Features

### Snippets

- **TOML tasks** (`snippets/toml-tasks.json`) — snippets for creating tasks in `mise.toml`
- **Shell tasks** (`snippets/shell-tasks.json`) — `#MISE` file task config snippet

## Installation

1. Open Zed
2. Go to Extensions (`cmd+shift+X`)
3. Search for "mise"
4. Install

Or install via the command palette:

```
zed: extension install mise
```

## Requirements

- [mise](https://mise.jdx.dev/) must be installed and available on PATH
- Zed with WASM extension support

## Development

```bash
# Build the extension (WASM)
mise run build

# Check code
mise run check

# Lint
mise run lint

# Format
mise run fmt
```

## Credits

- [hverlin](https://github.com/hverlin) for [mise-vscode](https://github.com/hverlin/mise-vscode) which inspired this extension
- [jdx](https://github.com/jdx) for [mise](https://mise.jdx.dev/)

## License

MIT
