use serde::Deserialize;
use std::path::Path;

use crate::error::{Error, Result};

/// Represents a single version entry from the index JSON-Lines
#[derive(Deserialize, Debug, Clone)]
pub struct CrateVersion {
    pub name: String,
    pub vers: String,
    #[serde(default)]
    pub yanked: bool,
}

/// Aggregated crate info for display
#[derive(Debug)]
pub struct CrateInfo {
    pub name: String,
    pub versions: Vec<String>,
}

impl CrateInfo {
    /// Create CrateInfo from a list of versions, sorting newest first
    pub fn from_versions(versions: Vec<CrateVersion>) -> Option<Self> {
        if versions.is_empty() {
            return None;
        }

        let name = versions[0].name.clone();

        // Collect non-yanked versions, then sort by semver (newest first)
        let mut version_strings: Vec<String> = versions
            .into_iter()
            .filter(|v| !v.yanked)
            .map(|v| v.vers)
            .collect();

        // Sort versions in descending order (simple string sort works for most cases)
        // For proper semver sorting, we'd need the semver crate
        version_strings.sort_by(|a, b| {
            // Try to compare as semver-like versions
            let a_parts: Vec<u32> = a.split('.').filter_map(|s| s.parse().ok()).collect();
            let b_parts: Vec<u32> = b.split('.').filter_map(|s| s.parse().ok()).collect();
            b_parts.cmp(&a_parts)
        });

        if version_strings.is_empty() {
            return None;
        }

        Some(CrateInfo {
            name,
            versions: version_strings,
        })
    }

    /// Get the latest version
    pub fn latest_version(&self) -> &str {
        self.versions.first().map(|s| s.as_str()).unwrap_or("")
    }
}

/// Parse a crate index file (JSON-Lines format)
pub fn parse_crate_file(path: &Path) -> Result<Vec<CrateVersion>> {
    let content = std::fs::read_to_string(path)?;

    content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            serde_json::from_str(line).map_err(|e| Error::CrateParse {
                path: path.to_path_buf(),
                source: e,
            })
        })
        .collect()
}

/// Check if a crate name matches the search query (case-insensitive partial match)
pub fn matches_query(crate_name: &str, query: &str) -> bool {
    crate_name.to_lowercase().contains(&query.to_lowercase())
}
