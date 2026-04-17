use anyhow::Result;
use std::process::Command;

#[derive(Debug, Clone, Default)]
pub struct LintResult {
    pub errors: u32,
    pub warnings: u32,
}

impl LintResult {
    pub fn is_clean(&self) -> bool {
        self.errors == 0 && self.warnings == 0
    }
}

pub fn run_lint(cmd: &str, project_dir: &std::path::Path) -> Result<String> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        anyhow::bail!("Empty lint command");
    }
    let output = Command::new(parts[0])
        .args(&parts[1..])
        .current_dir(project_dir)
        .output()?;

    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok(combined)
}

pub fn parse_cargo_output(output: &str) -> LintResult {
    let mut errors = 0u32;
    let mut warnings = 0u32;

    for line in output.lines() {
        let Ok(val) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if val.get("reason").and_then(|r| r.as_str()) != Some("compiler-message") {
            continue;
        }
        let Some(level) = val
            .get("message")
            .and_then(|m| m.get("level"))
            .and_then(|l| l.as_str())
        else {
            continue;
        };
        match level {
            "error" => errors += 1,
            "warning" => warnings += 1,
            _ => {}
        }
    }

    LintResult { errors, warnings }
}

pub fn parse_regex_output(output: &str, error_pattern: &str, warning_pattern: &str) -> LintResult {
    let mut errors = 0u32;
    let mut warnings = 0u32;

    let error_re = regex::Regex::new(error_pattern).unwrap_or_else(|_| regex::Regex::new("error").unwrap());
    let warning_re = regex::Regex::new(warning_pattern).unwrap_or_else(|_| regex::Regex::new("warning").unwrap());

    for line in output.lines() {
        if error_re.is_match(line) {
            errors += 1;
        }
        if warning_re.is_match(line) {
            warnings += 1;
        }
    }

    LintResult { errors, warnings }
}

/// Parse ESLint JSON output
pub fn parse_eslint_output(output: &str) -> LintResult {
    let mut errors = 0u32;
    let mut warnings = 0u32;

    // ESLint JSON output is an array of file results
    let Ok(results) = serde_json::from_str::<serde_json::Value>(output) else {
        return LintResult::default();
    };

    if let Some(arr) = results.as_array() {
        for file_result in arr {
            if let Some(count) = file_result.get("errorCount").and_then(|c| c.as_u64()) {
                errors += count as u32;
            }
            if let Some(count) = file_result.get("warningCount").and_then(|c| c.as_u64()) {
                warnings += count as u32;
            }
        }
    }

    LintResult { errors, warnings }
}

pub fn parse_output(output: &str, parser: &str, error_pattern: Option<&str>, warning_pattern: Option<&str>) -> LintResult {
    match parser {
        "cargo" => parse_cargo_output(output),
        "eslint" => parse_eslint_output(output),
        "regex" => parse_regex_output(
            output,
            error_pattern.unwrap_or("error"),
            warning_pattern.unwrap_or("warning"),
        ),
        _ => parse_regex_output(output, "error", "warning"),
    }
}
