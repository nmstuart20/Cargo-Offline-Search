# offline-search

A CLI tool for searching crates in a local offline cargo registry.

## Usage

```bash
offline-search <query> [--limit <n>]
```

Search for crates by name (case-insensitive, partial match):

```bash
offline-search serde
offline-search tokio --limit 10
```

## Installation

```bash
cargo install --path .
```

## Requirements

Expects a local cargo registry at `~/.cargo/registry/index/`.
