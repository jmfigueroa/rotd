use anyhow::Result;
use reqwest::blocking::Client;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// GitHub repository owner and name
const GITHUB_REPO_OWNER: &str = "jmfigueroa";
const GITHUB_REPO_NAME: &str = "rotd";

/// GitHub API URL for releases
fn github_releases_url() -> String {
    format!(
        "https://api.github.com/repos/{}/{}/releases",
        GITHUB_REPO_OWNER, GITHUB_REPO_NAME
    )
}

/// GitHub Release information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: String,
    pub published_at: String,
    pub body: String,
    pub html_url: String,
    pub assets: Vec<GitHubAsset>,
}

/// GitHub Release Asset
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

/// Release information used by the update function
#[derive(Debug, Clone, Serialize)]
pub struct ReleaseInfo {
    pub version: String,
    #[serde(skip_serializing)]
    pub semver: Version,
    pub published_at: String,
    pub name: String,
    pub description: String,
    pub download_url: String,
    pub html_url: String,
}

/// Fetch latest release information from GitHub
pub fn fetch_latest_release() -> Result<Option<ReleaseInfo>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("rotd-cli")
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to create HTTP client: {}", e))?;

    // Try to get the latest release
    let releases_url = github_releases_url();
    let response = client.get(&releases_url).send()
        .map_err(|e| {
            if e.is_timeout() {
                anyhow::anyhow!("Request timed out after 10 seconds. Check your internet connection.")
            } else if e.is_connect() {
                anyhow::anyhow!("Failed to connect to GitHub API. Check your internet connection and DNS.")
            } else {
                anyhow::anyhow!("Network error: {}", e)
            }
        })?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "GitHub API returned error {}: {}. This might be due to rate limiting or service issues.",
            response.status().as_u16(),
            response.status().canonical_reason().unwrap_or("Unknown error")
        ));
    }

    let releases: Vec<GitHubRelease> = response.json()
        .map_err(|e| anyhow::anyhow!("Failed to parse GitHub API response: {}", e))?;
    
    if releases.is_empty() {
        return Ok(None);
    }

    // Get the most recent release
    let latest_release = &releases[0];
    
    // Parse semver version from tag_name (removing 'v' prefix if present)
    let version_str = latest_release.tag_name.trim_start_matches('v');
    let semver = Version::parse(version_str)
        .map_err(|e| anyhow::anyhow!("Failed to parse version '{}' from release tag: {}", version_str, e))?;

    // Find suitable download asset (if any)
    let download_url = if let Some(asset) = latest_release.assets.iter().find(|a| {
        a.name.ends_with(".tar.gz") || a.name.ends_with(".zip")
    }) {
        asset.browser_download_url.clone()
    } else {
        latest_release.html_url.clone()
    };

    let release_info = ReleaseInfo {
        version: latest_release.tag_name.clone(),
        semver,
        published_at: latest_release.published_at.clone(),
        name: latest_release.name.clone(),
        description: latest_release.body.clone(),
        download_url,
        html_url: latest_release.html_url.clone(),
    };

    Ok(Some(release_info))
}

/// Check if update is available
pub fn check_update() -> Result<(bool, Option<ReleaseInfo>)> {
    // Get current version from Cargo.toml
    let current_version = env!("CARGO_PKG_VERSION");
    let current_semver = Version::parse(current_version)
        .map_err(|e| anyhow::anyhow!("Failed to parse current version '{}': {}", current_version, e))?;

    // Fetch latest release
    match fetch_latest_release()? {
        Some(latest) => {
            let update_available = latest.semver > current_semver;
            Ok((update_available, Some(latest)))
        }
        None => Ok((false, None)),
    }
}

/// Extract changes from release description (body)
pub fn extract_changes(body: &str) -> Vec<String> {
    body.lines()
        .filter(|line| {
            line.trim().starts_with('-') || 
            line.trim().starts_with('*') || 
            line.trim().starts_with('+')
        })
        .map(|line| line.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_changes() {
        let body = r#"
# Release v1.3.1

This release includes:

- Added task prioritization
- Fixed bug in test summary generation
* Improved error handling
+ New command for periodic reviews

## Details

More information here...
"#;

        let changes = extract_changes(body);
        assert_eq!(changes.len(), 4);
        assert!(changes.contains(&"- Added task prioritization".to_string()));
        assert!(changes.contains(&"- Fixed bug in test summary generation".to_string()));
        assert!(changes.contains(&"* Improved error handling".to_string()));
        assert!(changes.contains(&"+ New command for periodic reviews".to_string()));
    }
}