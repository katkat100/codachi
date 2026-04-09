use codachi::events::linter::{parse_cargo_output, parse_regex_output, LintResult};

#[test]
fn test_parse_cargo_json_with_errors_and_warnings() {
    let output = r#"{"reason":"compiler-message","message":{"level":"error","message":"cannot find value `x`"}}
{"reason":"compiler-message","message":{"level":"warning","message":"unused variable"}}
{"reason":"compiler-message","message":{"level":"warning","message":"unused import"}}
{"reason":"compiler-message","message":{"level":"error","message":"type mismatch"}}
{"reason":"build-finished","success":false}
"#;
    let result = parse_cargo_output(output);
    assert_eq!(result.errors, 2);
    assert_eq!(result.warnings, 2);
}

#[test]
fn test_parse_cargo_json_clean_build() {
    let output = r#"{"reason":"build-finished","success":true}
"#;
    let result = parse_cargo_output(output);
    assert_eq!(result.errors, 0);
    assert_eq!(result.warnings, 0);
}

#[test]
fn test_parse_regex_output() {
    let output = "src/main.rs:10: error: something broke\nsrc/main.rs:20: warning: unused\nsrc/lib.rs:5: error: bad thing\n";
    let result = parse_regex_output(output, r"error:", r"warning:");
    assert_eq!(result.errors, 2);
    assert_eq!(result.warnings, 1);
}

#[test]
fn test_parse_regex_output_with_anchors() {
    let output = "error: something broke\nwarning: unused\nall good\n";
    let result = parse_regex_output(output, r"^error", r"^warning");
    assert_eq!(result.errors, 1);
    assert_eq!(result.warnings, 1);
}

#[test]
fn test_parse_regex_output_no_matches() {
    let output = "Compiling codachi v0.1.0\nFinished dev profile\n";
    let result = parse_regex_output(output, r"^error", r"^warning");
    assert_eq!(result.errors, 0);
    assert_eq!(result.warnings, 0);
}

#[test]
fn test_lint_result_is_clean() {
    let clean = LintResult { errors: 0, warnings: 0 };
    assert!(clean.is_clean());
    let dirty = LintResult { errors: 1, warnings: 0 };
    assert!(!dirty.is_clean());
}
