use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub lint_cmd: String,
    pub watch_patterns: Vec<String>,
    pub lint_parser: String,
    pub error_pattern: Option<String>,
    pub warning_pattern: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lint_cmd: "cargo check --message-format=json".to_string(),
            watch_patterns: vec!["**/*.rs".to_string(), "**/*.toml".to_string()],
            lint_parser: "cargo".to_string(),
            error_pattern: None,
            warning_pattern: None,
        }
    }
}

impl Config {
    pub fn load_from(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
