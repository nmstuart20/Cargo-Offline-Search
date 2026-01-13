use reqwest::blocking::Client;
use serde::Deserialize;

use crate::error::{Error, Result};

const USERNAME_ENV: &str = "OFFLINE_SEARCH_USERNAME";
const PASSWORD_ENV: &str = "OFFLINE_SEARCH_PASSWORD";

#[derive(Deserialize, Debug)]
struct SearchResponse {
    items: Vec<SearchItem>,
}

#[derive(Deserialize, Debug)]
struct SearchItem {
    name: String,
    version: String,
}

pub struct RemoteResult {
    pub name: String,
    pub version: String,
}

fn get_credentials() -> Result<(String, String)> {
    let username = std::env::var(USERNAME_ENV).map_err(|_| Error::MissingCredentials)?;
    let password = std::env::var(PASSWORD_ENV).map_err(|_| Error::MissingCredentials)?;
    Ok((username, password))
}

pub fn search_remote(base_url: &str, repo: &str, query: &str) -> Result<Vec<RemoteResult>> {
    let (username, password) = get_credentials()?;

    // Warn if using HTTP instead of HTTPS
    if base_url.starts_with("http://") {
        eprintln!("warning: Using HTTP instead of HTTPS. Credentials will be sent in plain text.");
    }

    let url = format!(
        "{}/service/rest/v1/search?repository={}&name=*{}*",
        base_url.trim_end_matches('/'),
        repo,
        query
    );

    let client = Client::new();
    let response = client
        .get(&url)
        .basic_auth(&username, Some(&password))
        .send()?;

    if !response.status().is_success() {
        return Err(Error::HttpStatus {
            status: response.status().as_u16(),
        });
    }

    let body = response.text()?;
    let search_response: SearchResponse =
        serde_json::from_str(&body).map_err(Error::RemoteParse)?;

    let results = search_response
        .items
        .into_iter()
        .map(|item| RemoteResult {
            name: item.name,
            version: item.version,
        })
        .collect();

    Ok(results)
}
