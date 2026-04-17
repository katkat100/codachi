use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::language::{detect_project_type, get_preset, ProjectType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub lint_cmd: String,
    pub watch_patterns: Vec<String>,
    pub lint_parser: String,
    pub error_pattern: Option<String>,
    pub warning_pattern: Option<String>,
    #[serde(skip)]
    pub detected_language: Option<ProjectType>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lint_cmd: "cargo check --message-format=json".to_string(),
            watch_patterns: vec!["**/*.rs".to_string(), "**/*.toml".to_string()],
            lint_parser: "cargo".to_string(),
            error_pattern: None,
            warning_pattern: None,
            detected_language: None,
        }
    }
}

impl Config {
    /// Load config from file, or auto-detect from project directory
    pub fn load_from(path: &Path) -> Result<Self> {
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&content)?;
            return Ok(config);
        }

        // No config file - use defaults (will be overridden by auto-detect)
        Ok(Self::default())
    }

    /// Auto-detect language and apply preset if no custom config exists
    pub fn auto_detect(config_path: &Path, project_dir: &Path) -> Result<Self> {
        // If custom config exists, use it
        if config_path.exists() {
            return Self::load_from(config_path);
        }

        // Auto-detect project type
        let project_type = detect_project_type(project_dir);
        let preset = get_preset(&project_type);

        Ok(Self {
            lint_cmd: preset.lint_cmd.to_string(),
            watch_patterns: preset.watch_patterns.iter().map(|s| s.to_string()).collect(),
            lint_parser: preset.lint_parser.to_string(),
            error_pattern: preset.error_pattern.map(|s| s.to_string()),
            warning_pattern: preset.warning_pattern.map(|s| s.to_string()),
            detected_language: Some(project_type),
        })
    }
}
