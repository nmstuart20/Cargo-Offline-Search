use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

use crate::error::{Error, Result};

/// Check if a directory entry is a crate index file (not .git, not config.json)
fn is_crate_file(entry: &DirEntry) -> bool {
    if !entry.file_type().is_file() {
        return false;
    }

    let name = entry.file_name().to_string_lossy();

    // Skip hidden files and config.json
    !name.starts_with('.') && name != "config.json"
}

/// Find all crate index files in the registry
pub fn find_crate_files(registry_path: &Path) -> Result<Vec<PathBuf>> {
    let index_path = registry_path.join("index");

    if !index_path.exists() {
        return Err(Error::RegistryNotFound(index_path));
    }

    let mut crate_files = Vec::new();

    for entry in WalkDir::new(&index_path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            // Skip .git directory entirely
            let name = e.file_name().to_string_lossy();
            name != ".git"
        })
    {
        let entry = entry?;
        if is_crate_file(&entry) {
            crate_files.push(entry.into_path());
        }
    }

    Ok(crate_files)
}
