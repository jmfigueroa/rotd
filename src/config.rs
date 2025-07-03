use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RotdConfig {
    pub version: String,
    pub github_repo: String,
    pub default_score_threshold: u32,
}

impl Default for RotdConfig {
    fn default() -> Self {
        Self {
            version: "1.2.1".to_string(),
            github_repo: "https://github.com/jmfigueroa/rotd".to_string(),
            default_score_threshold: 6,
        }
    }
}