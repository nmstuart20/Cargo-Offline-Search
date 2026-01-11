use flate2::read::GzDecoder;
use serde::Deserialize;
use std::io::Read;
use std::path::Path;
use tar::Archive;

#[derive(Deserialize, Debug)]
struct CargoToml {
    package: Option<Package>,
}

#[derive(Deserialize, Debug)]
struct Package {
    description: Option<String>,
}

/// Extract the description from a .crate file
///
/// The .crate file is a gzip-compressed tarball containing the crate source.
/// The Cargo.toml is located at `{name}-{version}/Cargo.toml` within the archive.
pub fn extract_description(registry_path: &Path, name: &str, version: &str) -> Option<String> {
    let crate_filename = format!("{}-{}.crate", name, version);
    let crate_path = registry_path.join(&crate_filename);

    if !crate_path.exists() {
        return None;
    }

    let file = std::fs::File::open(&crate_path).ok()?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    let expected_path = format!("{}-{}/Cargo.toml", name, version);

    for entry in archive.entries().ok()? {
        let mut entry = entry.ok()?;
        let path = entry.path().ok()?;

        if path.to_string_lossy() == expected_path {
            let mut content = String::new();
            entry.read_to_string(&mut content).ok()?;

            let cargo_toml: CargoToml = toml::from_str(&content).ok()?;
            return cargo_toml.package?.description;
        }
    }

    None
}
