# offline-search

A CLI tool for searching crates in a local cargo registry or remote registry server.

## Usage

Search for crates by name (case-insensitive, partial match):

```bash
offline-search serde
offline-search tokio --limit 10
```

Search an alternate local registry (configured in `.cargo/config.toml`):

```bash
offline-search my-crate --registry my-internal-registry
```

Search a remote registry server:

```bash
offline-search my-crate --url https://registry.example.com --repo my-rust-repo
```

## Installation

```bash
cargo install --path .
```

## Configuration

### Default Registry

Expects a local cargo registry configured in `~/.cargo/config.toml`:

```toml
[source.crates-io]
replace-with = "local"

[source.local]
local-registry = "/path/to/registry"
```

### Alternate Registries

Configure named registries in `~/.cargo/config.toml`:

```toml
[registries.my-internal-registry]
index = "/path/to/alternate/registry"
```

### Remote Registry

Use `--url` and `--repo` flags with credentials set via environment variables:

- `OFFLINE_SEARCH_USERNAME` - Username for authentication
- `OFFLINE_SEARCH_PASSWORD` - Password for authentication
