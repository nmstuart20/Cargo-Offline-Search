mod cli;
mod config;
mod crate_info;
mod description;
mod error;
mod registry;

use clap::Parser;

use cli::Args;
use crate_info::{matches_query, parse_crate_file, CrateInfo};
use error::Result;

fn run() -> Result<()> {
    let args = Args::parse();

    // Find and verify the local registry
    let registry_path = match &args.registry {
        Some(name) => config::find_registry_by_name(name)?,
        None => config::find_local_registry()?,
    };

    // Find all crate index files
    let crate_files = registry::find_crate_files(&registry_path)?;

    // Search and collect matching crates
    let mut results: Vec<CrateInfo> = Vec::new();

    for crate_file in crate_files {
        match parse_crate_file(&crate_file) {
            Ok(versions) => {
                if let Some(first) = versions.first() {
                    if matches_query(&first.name, &args.query) {
                        if let Some(info) = CrateInfo::from_versions(versions) {
                            results.push(info);
                        }
                    }
                }
            }
            Err(e) => {
                // Skip malformed entries but warn
                eprintln!("warning: {}", e);
            }
        }
    }

    // Sort results alphabetically
    results.sort_by(|a, b| a.name.cmp(&b.name));

    // Apply limit
    let limit = args.limit.unwrap_or(results.len());
    let results: Vec<_> = results.into_iter().take(limit).collect();

    if results.is_empty() {
        println!("No crates found matching '{}'", args.query);
        return Ok(());
    }

    // Display results
    for info in results {
        let latest = info.latest_version();

        // Try to extract description from the .crate file
        let description = description::extract_description(&registry_path, &info.name, latest);

        // Format output similar to cargo search
        if let Some(desc) = description {
            println!("{} = \"{}\"    # {}", info.name, latest, desc);
        } else {
            println!("{} = \"{}\"", info.name, latest);
        }

        if info.versions.len() > 1 {
            println!("    versions: {}", info.versions.join(", "));
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
