use std::fs;
use std::path::Path;

use crate::model::{escape_json, ValidationStatus};
use crate::validate_fixture_dir;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureExpectation {
    Pass,
    Fail,
}

impl FixtureExpectation {
    fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
        }
    }

    fn expected_status(self) -> ValidationStatus {
        match self {
            Self::Pass => ValidationStatus::Passed,
            Self::Fail => ValidationStatus::Failed,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureMatrixLeafReport {
    pub expectation: String,
    pub leaf_name: String,
    pub descriptors_checked: usize,
    pub status: ValidationStatus,
    pub diagnostics_count: usize,
    pub errors_count: usize,
    pub outcome: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureMatrixError {
    pub code: String,
    pub expectation: Option<String>,
    pub leaf_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureMatrixReport {
    pub status: ValidationStatus,
    pub fixture_root: String,
    pub leaves: Vec<FixtureMatrixLeafReport>,
    pub errors: Vec<FixtureMatrixError>,
    pub warnings: Vec<String>,
}

impl FixtureMatrixReport {
    pub fn to_json_pretty(&self) -> String {
        let leaves = self
            .leaves
            .iter()
            .map(FixtureMatrixLeafReport::to_json_pretty)
            .collect::<Vec<_>>()
            .join(",\n");
        let errors = self
            .errors
            .iter()
            .map(FixtureMatrixError::to_json_pretty)
            .collect::<Vec<_>>()
            .join(",\n");
        let warnings = self
            .warnings
            .iter()
            .map(|warning| format!("    \"{}\"", escape_json(warning)))
            .collect::<Vec<_>>()
            .join(",\n");

        format!(
            "{{\n  \"status\": \"{}\",\n  \"fixture_root\": \"{}\",\n  \"leaves\": [\n{}\n  ],\n  \"errors\": [\n{}\n  ],\n  \"warnings\": [\n{}\n  ]\n}}",
            self.status,
            escape_json(&self.fixture_root),
            leaves,
            errors,
            warnings
        )
    }
}

impl FixtureMatrixLeafReport {
    fn to_json_pretty(&self) -> String {
        format!(
            "    {{\n      \"expectation\": \"{}\",\n      \"leaf_name\": \"{}\",\n      \"descriptors_checked\": {},\n      \"status\": \"{}\",\n      \"diagnostics_count\": {},\n      \"errors_count\": {},\n      \"outcome\": \"{}\"\n    }}",
            escape_json(&self.expectation),
            escape_json(&self.leaf_name),
            self.descriptors_checked,
            self.status,
            self.diagnostics_count,
            self.errors_count,
            escape_json(&self.outcome)
        )
    }
}

impl FixtureMatrixError {
    fn to_json_pretty(&self) -> String {
        let expectation = self
            .expectation
            .as_ref()
            .map(|value| format!("\"{}\"", escape_json(value)))
            .unwrap_or_else(|| "null".to_string());
        let leaf_name = self
            .leaf_name
            .as_ref()
            .map(|value| format!("\"{}\"", escape_json(value)))
            .unwrap_or_else(|| "null".to_string());

        format!(
            "    {{\n      \"code\": \"{}\",\n      \"expectation\": {},\n      \"leaf_name\": {}\n    }}",
            escape_json(&self.code),
            expectation,
            leaf_name
        )
    }
}

pub fn validate_fixture_matrix(root: impl AsRef<Path>) -> FixtureMatrixReport {
    let root = root.as_ref();
    let fixture_root = root.display().to_string();
    let mut leaves = Vec::new();
    let mut errors = Vec::new();

    if !root.exists() {
        errors.push(matrix_error("fixture_root_missing", None, None));
        return matrix_report(fixture_root, leaves, errors);
    }

    if !root.is_dir() {
        errors.push(matrix_error("fixture_root_not_directory", None, None));
        return matrix_report(fixture_root, leaves, errors);
    }

    for entry in sorted_dir_entries(root) {
        let Ok(entry) = entry else {
            errors.push(matrix_error("fixture_root_read_failed", None, None));
            continue;
        };
        let Ok(file_type) = entry.file_type() else {
            errors.push(matrix_error("fixture_root_read_failed", None, None));
            continue;
        };
        if !file_type.is_dir() {
            continue;
        }

        let name = entry.file_name();
        let name = name.to_string_lossy();
        let expectation = match name.as_ref() {
            "pass" => FixtureExpectation::Pass,
            "fail" => FixtureExpectation::Fail,
            _ => {
                errors.push(matrix_error("unknown_expectation_directory", None, None));
                continue;
            }
        };

        collect_expectation_leaves(&entry.path(), expectation, &mut leaves, &mut errors);
    }

    if leaves.is_empty() && errors.is_empty() {
        errors.push(matrix_error("fixture_matrix_empty", None, None));
    }

    matrix_report(fixture_root, leaves, errors)
}

fn collect_expectation_leaves(
    expectation_root: &Path,
    expectation: FixtureExpectation,
    leaves: &mut Vec<FixtureMatrixLeafReport>,
    errors: &mut Vec<FixtureMatrixError>,
) {
    for entry in sorted_dir_entries(expectation_root) {
        let Ok(entry) = entry else {
            errors.push(matrix_error(
                "expectation_directory_read_failed",
                Some(expectation),
                None,
            ));
            continue;
        };
        let Ok(file_type) = entry.file_type() else {
            errors.push(matrix_error(
                "expectation_directory_read_failed",
                Some(expectation),
                None,
            ));
            continue;
        };
        if !file_type.is_dir() {
            errors.push(matrix_error(
                "leaf_not_directory",
                Some(expectation),
                Some("redacted_entry"),
            ));
            continue;
        }

        let leaf_name = sanitize_leaf_name(&entry.file_name().to_string_lossy());
        let report = validate_fixture_dir(entry.path());
        let outcome = leaf_outcome(expectation, report.status, report.errors.len());

        if outcome != "matched_expected_status" {
            errors.push(matrix_error(&outcome, Some(expectation), Some(&leaf_name)));
        }

        leaves.push(FixtureMatrixLeafReport {
            expectation: expectation.as_str().to_string(),
            leaf_name,
            descriptors_checked: report.descriptors_checked,
            status: report.status,
            diagnostics_count: report.diagnostics.len(),
            errors_count: report.errors.len(),
            outcome,
        });
    }
}

fn leaf_outcome(
    expectation: FixtureExpectation,
    actual_status: ValidationStatus,
    errors_count: usize,
) -> String {
    if errors_count > 0 {
        return "leaf_error".to_string();
    }

    if actual_status == expectation.expected_status() {
        "matched_expected_status".to_string()
    } else {
        match expectation {
            FixtureExpectation::Pass => "pass_leaf_failed".to_string(),
            FixtureExpectation::Fail => "fail_leaf_passed".to_string(),
        }
    }
}

fn sorted_dir_entries(path: &Path) -> Vec<std::io::Result<fs::DirEntry>> {
    let Ok(entries) = fs::read_dir(path) else {
        return vec![Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "directory read failed",
        ))];
    };

    let mut entries = entries.collect::<Vec<_>>();
    entries.sort_by(|left, right| {
        let left_name = left
            .as_ref()
            .ok()
            .map(|entry| entry.file_name())
            .unwrap_or_default();
        let right_name = right
            .as_ref()
            .ok()
            .map(|entry| entry.file_name())
            .unwrap_or_default();
        left_name.cmp(&right_name)
    });
    entries
}

fn sanitize_leaf_name(raw: &str) -> String {
    let sanitized = raw
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.'))
        .collect::<String>();

    if sanitized.is_empty() || contains_denied_summary_token(&sanitized) {
        "redacted_leaf".to_string()
    } else {
        sanitized
    }
}

fn contains_denied_summary_token(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "prompt_body",
        "source_register",
        "raw_source",
        "raw_path",
        "secret",
        "credential",
        "api_key",
        "provider_config",
        "token",
    ]
    .iter()
    .any(|token| lower.contains(token))
}

fn matrix_error(
    code: &str,
    expectation: Option<FixtureExpectation>,
    leaf_name: Option<&str>,
) -> FixtureMatrixError {
    FixtureMatrixError {
        code: code.to_string(),
        expectation: expectation.map(|value| value.as_str().to_string()),
        leaf_name: leaf_name.map(sanitize_leaf_name),
    }
}

fn matrix_report(
    fixture_root: String,
    leaves: Vec<FixtureMatrixLeafReport>,
    errors: Vec<FixtureMatrixError>,
) -> FixtureMatrixReport {
    let status = if errors.is_empty() {
        ValidationStatus::Passed
    } else {
        ValidationStatus::Failed
    };
    FixtureMatrixReport {
        status,
        fixture_root,
        leaves,
        errors,
        warnings: vec![
            "matrix_runner_uses_explicit_offline_fixture_root_only".to_string(),
            "matrix_summary_is_leaf_level_and_sanitized".to_string(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn accepts_pass_and_expected_fail_leaves() {
        let root = temp_fixture_root("matrix-ok");
        write_fixture(
            &root.join("pass").join("lane_pass"),
            r#"{
              "descriptor_type":"QueryResult",
              "descriptor_version":"v0.2",
              "descriptor_hash":"sha256:pass",
              "artifact_class":"prompt_payload",
              "schema_version":"v0.2",
              "descriptor_id":"q-pass",
              "query_type":"runtime_query",
              "resolved_intent":"x",
              "intent_confidence":1,
              "intent_routing_status":"resolved",
              "selected_sample_ids":[],
              "selected_kb_rules":[],
              "kb_context_summary":"summary",
              "retrieval_trace_ref":"trace-pass",
              "fallback_reason_code":"on_empty_pool",
              "freshness_status":"fresh",
              "activation_status":"activated",
              "snapshot_hash":"sha256:s",
              "index_hash":"sha256:i",
              "full_kb_rows_included":0
            }"#,
        );
        write_fixture(
            &root.join("fail").join("lane_fail"),
            r#"{
              "descriptor_type":"QueryResult",
              "descriptor_version":"v0.2",
              "descriptor_hash":"sha256:fail",
              "artifact_class":"prompt_payload",
              "schema_version":"v0.2",
              "descriptor_id":"q-fail",
              "query_type":"runtime_query",
              "resolved_intent":"x",
              "intent_confidence":1,
              "intent_routing_status":"resolved",
              "selected_sample_ids":[],
              "selected_kb_rules":[],
              "kb_context_summary":"summary",
              "retrieval_trace_ref":"trace-fail",
              "fallback_reason_code":"on_empty_pool",
              "freshness_status":"fresh",
              "activation_status":"activated",
              "snapshot_hash":"sha256:s",
              "index_hash":"sha256:i",
              "full_kb_rows_included":1
            }"#,
        );

        let report = validate_fixture_matrix(&root);
        assert_eq!(report.status, ValidationStatus::Passed);
        assert_eq!(report.leaves.len(), 2);
        assert!(report.errors.is_empty());

        let json = report.to_json_pretty();
        assert!(!json.contains("sha256:pass"));
        assert!(!json.contains("sha256:fail"));
        assert!(!json.contains("source_register"));
        assert!(!json.contains("prompt_body"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn rejects_pass_failure_and_fail_success() {
        let root = temp_fixture_root("matrix-mismatch");
        write_fixture(
            &root.join("pass").join("lane_pass_bad"),
            r#"{
              "descriptor_type":"QueryResult",
              "descriptor_id":"q-pass-bad",
              "artifact_class":"prompt_payload",
              "full_kb_rows_included":1
            }"#,
        );
        write_fixture(
            &root.join("fail").join("lane_fail_bad"),
            r#"{
              "descriptor_type":"QueryResult",
              "descriptor_version":"v0.2",
              "descriptor_hash":"sha256:pass",
              "artifact_class":"prompt_payload",
              "schema_version":"v0.2",
              "descriptor_id":"q-fail-bad",
              "query_type":"runtime_query",
              "resolved_intent":"x",
              "intent_confidence":1,
              "intent_routing_status":"resolved",
              "selected_sample_ids":[],
              "selected_kb_rules":[],
              "kb_context_summary":"summary",
              "retrieval_trace_ref":"trace-fail-bad",
              "fallback_reason_code":"on_empty_pool",
              "freshness_status":"fresh",
              "activation_status":"activated",
              "snapshot_hash":"sha256:s",
              "index_hash":"sha256:i",
              "full_kb_rows_included":0
            }"#,
        );

        let report = validate_fixture_matrix(&root);
        assert_eq!(report.status, ValidationStatus::Failed);
        assert!(report
            .errors
            .iter()
            .any(|error| error.code == "pass_leaf_failed"));
        assert!(report
            .errors
            .iter()
            .any(|error| error.code == "fail_leaf_passed"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn rejects_leaf_error_and_unknown_expectation_directory() {
        let root = temp_fixture_root("matrix-errors");
        let bad_leaf = root.join("fail").join("bad_json");
        fs::create_dir_all(&bad_leaf).unwrap();
        fs::write(bad_leaf.join("fixture.json"), "{not-json").unwrap();
        fs::create_dir_all(root.join("maybe")).unwrap();

        let report = validate_fixture_matrix(&root);
        assert_eq!(report.status, ValidationStatus::Failed);
        assert!(report.errors.iter().any(|error| error.code == "leaf_error"));
        assert!(report
            .errors
            .iter()
            .any(|error| error.code == "unknown_expectation_directory"));

        let json = report.to_json_pretty();
        assert!(!json.contains("fixture.json"));
        assert!(!json.contains("not-json"));

        let _ = fs::remove_dir_all(root);
    }

    fn write_fixture(dir: &Path, raw: &str) {
        fs::create_dir_all(dir).unwrap();
        fs::write(dir.join("fixture.json"), raw).unwrap();
    }

    fn temp_fixture_root(label: &str) -> PathBuf {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "descriptor-validator-{label}-{}-{now}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }
}
