pub mod artifact_class;
pub mod denied_scan;
pub mod diagnostic;
pub mod error;
pub mod json_walk;
pub mod model;
pub mod rules;

use std::fs;
use std::path::{Path, PathBuf};

use artifact_class::validate_artifact_class;
use denied_scan::scan_denied_fields;
use error::{ValidationError, ValidationErrorCode};
use json_walk::{parse_json, JsonValue};
use model::ValidationReport;
use rules::validate_first_wave_rules;

pub fn validate_fixture_dir(path: impl AsRef<Path>) -> ValidationReport {
    let path = path.as_ref();
    let display_path = path.display().to_string();

    if !path.exists() {
        return ValidationReport::failed(
            Some(display_path.clone()),
            vec![ValidationError::new(
                ValidationErrorCode::FixtureDirMissing,
                "fixture directory does not exist",
                Some(display_path),
            )],
        );
    }

    if !path.is_dir() {
        return ValidationReport::failed(
            Some(display_path.clone()),
            vec![ValidationError::new(
                ValidationErrorCode::FixturePathNotDirectory,
                "fixture path is not a directory",
                Some(display_path),
            )],
        );
    }

    let mut errors = Vec::new();
    let mut diagnostics = Vec::new();
    let mut descriptors_checked = 0;

    for fixture_path in json_fixture_paths(path) {
        match fs::read_to_string(&fixture_path) {
            Ok(raw) => match parse_json(&raw) {
                Ok(value) => {
                    descriptors_checked +=
                        validate_json_root(&value, &fixture_path, &mut errors, &mut diagnostics);
                }
                Err(error) => errors.push(ValidationError::new(
                    ValidationErrorCode::InvalidArgument,
                    format!("invalid descriptor json: {}", error),
                    Some(fixture_path.display().to_string()),
                )),
            },
            Err(error) => errors.push(ValidationError::new(
                ValidationErrorCode::InvalidArgument,
                format!("could not read descriptor json: {}", error),
                Some(fixture_path.display().to_string()),
            )),
        }
    }

    ValidationReport::from_parts(Some(display_path), descriptors_checked, errors, diagnostics)
}

fn json_fixture_paths(path: &Path) -> Vec<PathBuf> {
    let mut paths = match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_file()
                    && path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext.eq_ignore_ascii_case("json"))
                        .unwrap_or(false)
            })
            .collect::<Vec<_>>(),
        Err(_) => Vec::new(),
    };
    paths.sort();
    paths
}

fn validate_json_root(
    value: &JsonValue,
    fixture_path: &Path,
    errors: &mut Vec<ValidationError>,
    diagnostics: &mut Vec<diagnostic::Diagnostic>,
) -> usize {
    match value {
        JsonValue::Object(_) => {
            validate_descriptor(value, diagnostics);
            1
        }
        JsonValue::Array(items) => {
            let mut count = 0;
            for (index, item) in items.iter().enumerate() {
                if matches!(item, JsonValue::Object(_)) {
                    validate_descriptor(item, diagnostics);
                    count += 1;
                } else {
                    errors.push(ValidationError::new(
                        ValidationErrorCode::InvalidArgument,
                        "descriptor array item is not an object",
                        Some(format!("{}[{}]", fixture_path.display(), index)),
                    ));
                }
            }
            count
        }
        _ => {
            errors.push(ValidationError::new(
                ValidationErrorCode::InvalidArgument,
                "descriptor fixture root must be an object or array",
                Some(fixture_path.display().to_string()),
            ));
            0
        }
    }
}

fn validate_descriptor(descriptor: &JsonValue, diagnostics: &mut Vec<diagnostic::Diagnostic>) {
    diagnostics.extend(scan_denied_fields(descriptor));
    diagnostics.extend(validate_artifact_class(descriptor));
    diagnostics.extend(validate_first_wave_rules(descriptor));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn validates_existing_fixture_dir_without_reading_artifacts() {
        let report = validate_fixture_dir(".");
        assert_eq!(report.status, model::ValidationStatus::Passed);
        assert_eq!(report.descriptors_checked, 0);
    }

    #[test]
    fn rejects_missing_fixture_dir() {
        let report = validate_fixture_dir("__missing_descriptor_fixture_dir__");
        assert_eq!(report.status, model::ValidationStatus::Failed);
        assert_eq!(report.errors.len(), 1);
        assert_eq!(
            report.errors[0].code,
            error::ValidationErrorCode::FixtureDirMissing
        );
    }

    #[test]
    fn validates_clean_descriptor_fixture() {
        let dir = temp_fixture_dir("clean");
        fs::write(
            dir.join("query.json"),
            r#"{
              "descriptor_type":"QueryResult",
              "descriptor_version":"v0.2",
              "descriptor_hash":"sha256:abc",
              "artifact_class":"prompt_payload",
              "schema_version":"v0.2",
              "descriptor_id":"q1",
              "query_type":"runtime_query",
              "resolved_intent":"x",
              "intent_confidence":1,
              "intent_routing_status":"resolved",
              "selected_sample_ids":[],
              "selected_kb_rules":[],
              "kb_context_summary":"summary",
              "retrieval_trace_ref":"t1",
              "fallback_reason_code":"on_empty_pool",
              "freshness_status":"fresh",
              "activation_status":"activated",
              "snapshot_hash":"sha256:s",
              "index_hash":"sha256:i",
              "full_kb_rows_included":0
            }"#,
        )
        .unwrap();

        let report = validate_fixture_dir(&dir);
        assert_eq!(report.status, model::ValidationStatus::Passed);
        assert_eq!(report.descriptors_checked, 1);
        assert!(report.diagnostics.is_empty());

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn rejects_denied_field_and_bad_full_kb_rows() {
        let dir = temp_fixture_dir("bad");
        fs::write(
            dir.join("query.json"),
            r#"{
              "descriptor_type":"QueryResult",
              "descriptor_id":"q1",
              "artifact_class":"prompt_payload",
              "full_kb_rows_included":1,
              "metadata":{"debug":{"prompt_body":"must_not_emit"}}
            }"#,
        )
        .unwrap();

        let report = validate_fixture_dir(&dir);
        assert_eq!(report.status, model::ValidationStatus::Failed);
        assert_eq!(report.descriptors_checked, 1);
        assert!(report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "raw_prompt_body"));
        assert!(report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "full_kb_rows_included_not_zero"));
        assert!(!report.to_json_pretty().contains("must_not_emit"));

        let _ = fs::remove_dir_all(dir);
    }

    fn temp_fixture_dir(label: &str) -> PathBuf {
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
