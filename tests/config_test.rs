use codachi::config::Config;
use tempfile::TempDir;

#[test]
fn test_default_config() {
    let config = Config::default();
    assert_eq!(config.lint_cmd, "cargo check --message-format=json");
    assert_eq!(config.lint_parser, "cargo");
    assert!(!config.watch_patterns.is_empty());
}

#[test]
fn test_load_config_from_toml() {
    let dir = TempDir::new().unwrap();
    let config_dir = dir.path().join(".codachi");
    std::fs::create_dir_all(&config_dir).unwrap();
    let config_path = config_dir.join("config.toml");

    std::fs::write(
        &config_path,
        r#"
lint_cmd = "eslint --format json ."
watch_patterns = ["**/*.js", "**/*.ts"]
lint_parser = "eslint"
"#,
    )
    .unwrap();

    let config = Config::load_from(&config_path).unwrap();
    assert_eq!(config.lint_cmd, "eslint --format json .");
    assert_eq!(config.lint_parser, "eslint");
    assert_eq!(config.watch_patterns, vec!["**/*.js", "**/*.ts"]);
}

#[test]
fn test_load_missing_config_returns_default() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join(".codachi/config.toml");
    let config = Config::load_from(&path).unwrap();
    assert_eq!(config.lint_cmd, "cargo check --message-format=json");
}
