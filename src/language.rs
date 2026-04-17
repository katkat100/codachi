use std::path::Path;

/// Detected project language/framework
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectType {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Go,
    Ruby,
    Java,
    CSharp,
    Cpp,
    Unknown,
}

impl ProjectType {
    /// Display name for the project type
    pub fn name(&self) -> &'static str {
        match self {
            ProjectType::Rust => "Rust",
            ProjectType::JavaScript => "JavaScript",
            ProjectType::TypeScript => "TypeScript",
            ProjectType::Python => "Python",
            ProjectType::Go => "Go",
            ProjectType::Ruby => "Ruby",
            ProjectType::Java => "Java",
            ProjectType::CSharp => "C#",
            ProjectType::Cpp => "C++",
            ProjectType::Unknown => "Unknown",
        }
    }
}

/// Preset configuration for a language
#[derive(Debug, Clone)]
pub struct LangPreset {
    pub lint_cmd: &'static str,
    pub lint_parser: &'static str,
    pub watch_patterns: &'static [&'static str],
    pub error_pattern: Option<&'static str>,
    pub warning_pattern: Option<&'static str>,
}

/// Get the preset configuration for a project type
pub fn get_preset(project_type: &ProjectType) -> LangPreset {
    match project_type {
        ProjectType::Rust => LangPreset {
            lint_cmd: "cargo check --message-format=json",
            lint_parser: "cargo",
            watch_patterns: &["**/*.rs", "**/Cargo.toml"],
            error_pattern: None,
            warning_pattern: None,
        },
        ProjectType::JavaScript => LangPreset {
            lint_cmd: "npx eslint . --format json",
            lint_parser: "eslint",
            watch_patterns: &["**/*.js", "**/*.jsx", "**/*.mjs", "**/*.cjs"],
            error_pattern: None,
            warning_pattern: None,
        },
        ProjectType::TypeScript => LangPreset {
            lint_cmd: "npx tsc --noEmit",
            lint_parser: "regex",
            watch_patterns: &["**/*.ts", "**/*.tsx"],
            error_pattern: Some(r"error TS\d+:"),
            warning_pattern: Some(r"warning TS\d+:"),
        },
        ProjectType::Python => LangPreset {
            lint_cmd: "python -m py_compile",
            lint_parser: "regex",
            watch_patterns: &["**/*.py"],
            error_pattern: Some(r"(?i)error|SyntaxError|IndentationError"),
            warning_pattern: Some(r"(?i)warning"),
        },
        ProjectType::Go => LangPreset {
            lint_cmd: "go build ./...",
            lint_parser: "regex",
            watch_patterns: &["**/*.go"],
            error_pattern: Some(r":\d+:\d+:.*"),
            warning_pattern: None,
        },
        ProjectType::Ruby => LangPreset {
            lint_cmd: "ruby -c",
            lint_parser: "regex",
            watch_patterns: &["**/*.rb", "**/Gemfile"],
            error_pattern: Some(r"(?i)syntax error"),
            warning_pattern: Some(r"(?i)warning"),
        },
        ProjectType::Java => LangPreset {
            lint_cmd: "javac -Xlint:all",
            lint_parser: "regex",
            watch_patterns: &["**/*.java"],
            error_pattern: Some(r"error:"),
            warning_pattern: Some(r"warning:"),
        },
        ProjectType::CSharp => LangPreset {
            lint_cmd: "dotnet build",
            lint_parser: "regex",
            watch_patterns: &["**/*.cs", "**/*.csproj"],
            error_pattern: Some(r"error CS\d+:"),
            warning_pattern: Some(r"warning CS\d+:"),
        },
        ProjectType::Cpp => LangPreset {
            lint_cmd: "make",
            lint_parser: "regex",
            watch_patterns: &["**/*.cpp", "**/*.hpp", "**/*.c", "**/*.h"],
            error_pattern: Some(r"error:"),
            warning_pattern: Some(r"warning:"),
        },
        ProjectType::Unknown => LangPreset {
            lint_cmd: "echo 'No linter configured'",
            lint_parser: "regex",
            watch_patterns: &["**/*"],
            error_pattern: Some(r"(?i)error"),
            warning_pattern: Some(r"(?i)warning"),
        },
    }
}

/// Detect the project type based on files in the directory
pub fn detect_project_type(project_dir: &Path) -> ProjectType {
    // Check for language-specific marker files (in priority order)
    let markers: &[(&str, ProjectType)] = &[
        // Rust
        ("Cargo.toml", ProjectType::Rust),
        // TypeScript (check before JS)
        ("tsconfig.json", ProjectType::TypeScript),
        // JavaScript/Node
        ("package.json", ProjectType::JavaScript),
        // Python
        ("pyproject.toml", ProjectType::Python),
        ("setup.py", ProjectType::Python),
        ("requirements.txt", ProjectType::Python),
        ("Pipfile", ProjectType::Python),
        // Go
        ("go.mod", ProjectType::Go),
        // Ruby
        ("Gemfile", ProjectType::Ruby),
        // Java
        ("pom.xml", ProjectType::Java),
        ("build.gradle", ProjectType::Java),
        // C#
        ("*.csproj", ProjectType::CSharp),
        ("*.sln", ProjectType::CSharp),
        // C++
        ("CMakeLists.txt", ProjectType::Cpp),
        ("Makefile", ProjectType::Cpp),
    ];

    for (marker, project_type) in markers {
        if marker.starts_with('*') {
            // Glob pattern - check if any matching file exists
            let pattern = marker.trim_start_matches('*');
            if let Ok(entries) = std::fs::read_dir(project_dir) {
                for entry in entries.flatten() {
                    if entry.path().to_string_lossy().ends_with(pattern) {
                        return project_type.clone();
                    }
                }
            }
        } else {
            // Exact filename
            if project_dir.join(marker).exists() {
                return project_type.clone();
            }
        }
    }

    ProjectType::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_detect_rust_project() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("Cargo.toml"), "[package]").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Rust);
    }

    #[test]
    fn test_detect_javascript_project() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("package.json"), "{}").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::JavaScript);
    }

    #[test]
    fn test_detect_typescript_project() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("package.json"), "{}").unwrap();
        fs::write(dir.path().join("tsconfig.json"), "{}").unwrap();
        // TypeScript should take priority over JavaScript
        assert_eq!(detect_project_type(dir.path()), ProjectType::TypeScript);
    }

    #[test]
    fn test_detect_python_project() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("requirements.txt"), "flask").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Python);
    }

    #[test]
    fn test_detect_go_project() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("go.mod"), "module test").unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Go);
    }

    #[test]
    fn test_unknown_project() {
        let dir = tempdir().unwrap();
        assert_eq!(detect_project_type(dir.path()), ProjectType::Unknown);
    }

    #[test]
    fn test_preset_has_lint_cmd() {
        let preset = get_preset(&ProjectType::Rust);
        assert!(!preset.lint_cmd.is_empty());
    }
}
