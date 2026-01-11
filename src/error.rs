use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cargo config not found. Checked: {}", .0.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", "))]
    ConfigNotFound(Vec<PathBuf>),

    #[error("Failed to parse cargo config: {0}")]
    ConfigParse(#[from] toml::de::Error),

    #[error("No local registry configured. Ensure [source.crates-io] has 'replace-with' pointing to a source with 'local-registry'")]
    NoLocalRegistry,

    #[error("Registry index not found at: {0}")]
    RegistryNotFound(PathBuf),

    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to walk directory: {0}")]
    WalkDir(#[from] walkdir::Error),

    #[error("Failed to parse crate metadata in {path}: {source}")]
    CrateParse {
        path: PathBuf,
        source: serde_json::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
