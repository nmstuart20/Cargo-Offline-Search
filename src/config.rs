use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::error::{Error, Result};

#[derive(Deserialize, Debug)]
pub struct CargoConfig {
    pub source: Option<HashMap<String, Source>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Source {
    pub local_registry: Option<String>,
    pub replace_with: Option<String>,
}

/// Find the cargo config file location
fn find_config_path() -> Option<PathBuf> {
    // Check $CARGO_HOME/config.toml first
    if let Ok(cargo_home) = std::env::var("CARGO_HOME") {
        let cargo_home = PathBuf::from(cargo_home);
        let path = cargo_home.join("config.toml");
        if path.exists() {
            return Some(path);
        }
        // Also check config without .toml extension
        let path = cargo_home.join("config");
        if path.exists() {
            return Some(path);
        }
    }

    // Fall back to ~/.cargo/config.toml
    if let Some(home) = dirs::home_dir() {
        let path = home.join(".cargo").join("config.toml");
        if path.exists() {
            return Some(path);
        }
        // Also check config without .toml extension
        let path = home.join(".cargo").join("config");
        if path.exists() {
            return Some(path);
        }
    }

    None
}

/// Get the list of paths that were checked for the config file
fn get_checked_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(cargo_home) = std::env::var("CARGO_HOME") {
        paths.push(PathBuf::from(&cargo_home).join("config.toml"));
        paths.push(PathBuf::from(cargo_home).join("config"));
    }

    if let Some(home) = dirs::home_dir() {
        paths.push(home.join(".cargo").join("config.toml"));
        paths.push(home.join(".cargo").join("config"));
    }

    paths
}

/// Find and return the local registry path from cargo config
pub fn find_local_registry() -> Result<PathBuf> {
    let config_path =
        find_config_path().ok_or_else(|| Error::ConfigNotFound(get_checked_paths()))?;

    let content = std::fs::read_to_string(&config_path)?;
    let config: CargoConfig = toml::from_str(&content)?;

    let sources = config.source.ok_or(Error::NoLocalRegistry)?;

    // Find the crates-io source and get its replace-with value
    let crates_io = sources.get("crates-io").ok_or(Error::NoLocalRegistry)?;
    let replace_with = crates_io
        .replace_with
        .as_ref()
        .ok_or(Error::NoLocalRegistry)?;

    // Find the referenced source and get its local-registry path
    let replacement_source = sources.get(replace_with).ok_or(Error::NoLocalRegistry)?;
    let registry_path = replacement_source
        .local_registry
        .as_ref()
        .ok_or(Error::NoLocalRegistry)?;

    let path = PathBuf::from(registry_path);

    // Verify the registry path exists
    if !path.exists() {
        return Err(Error::RegistryNotFound(path));
    }

    Ok(path)
}
