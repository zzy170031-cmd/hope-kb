use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

use super::{
    descriptor_id, descriptor_type, validate_hash_fields, validate_no_raw_locator_fields,
    validate_required_fields, validate_string_in,
};

const RULE_ID: &str = "rollback_pointer_static";

const REQUIRED_FIELDS: &[&str] = &[
    "descriptor_type",
    "descriptor_version",
    "descriptor_id",
    "descriptor_hash",
    "artifact_class",
    "schema_version",
    "rollback_pointer_id",
    "rollback_trigger_code",
    "current_failed_descriptor_id",
    "current_failed_descriptor_hash",
    "target_lkg_descriptor_id",
    "target_lkg_descriptor_hash",
    "rollback_status",
    "rolled_back_at_bucket",
    "sanitized_incident_id",
];

const HASH_FIELDS: &[&str] = &[
    "descriptor_hash",
    "current_failed_descriptor_hash",
    "target_lkg_descriptor_hash",
];

const POINTER_FIELDS: &[&str] = &[
    "rollback_pointer_id",
    "current_failed_descriptor_id",
    "current_failed_descriptor_hash",
    "target_lkg_descriptor_id",
    "target_lkg_descriptor_hash",
];

const ROLLBACK_STATUS: &[&str] = &["ready", "executed", "blocked"];

pub fn validate_rollback_pointer(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    if descriptor_type != "RollbackPointer" {
        return Vec::new();
    }
    let descriptor_id = descriptor_id(descriptor);
    let mut diagnostics = Vec::new();

    diagnostics.extend(validate_required_fields(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        REQUIRED_FIELDS,
        RULE_ID,
    ));
    diagnostics.extend(validate_string_in(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "rollback_status",
        ROLLBACK_STATUS,
        "invalid_rollback_status",
        RULE_ID,
    ));
    diagnostics.extend(validate_hash_fields(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        HASH_FIELDS,
        RULE_ID,
    ));
    diagnostics.extend(validate_no_raw_locator_fields(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        POINTER_FIELDS,
        "raw_path_or_locator_in_rollback_pointer",
        RULE_ID,
    ));

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn accepts_passing_rollback_pointer() {
        let value = parse_json(
            r#"{
              "descriptor_type":"RollbackPointer",
              "descriptor_version":"v0.2",
              "descriptor_id":"rollback-pointer-1",
              "descriptor_hash":"sha256:rollbackpointer1",
              "artifact_class":"retrieval_trace_log_telemetry_shadow_rollback",
              "schema_version":"v0.2",
              "rollback_pointer_id":"rollback-pointer-1",
              "rollback_trigger_code":"activation_failed",
              "current_failed_descriptor_id":"activation-failed-1",
              "current_failed_descriptor_hash":"sha256:failed1",
              "target_lkg_descriptor_id":"lkg-1",
              "target_lkg_descriptor_hash":"sha256:lkg1",
              "rollback_status":"ready",
              "rolled_back_at_bucket":"2026-04-26T14",
              "sanitized_incident_id":"incident-1"
            }"#,
        )
        .unwrap();

        assert!(validate_rollback_pointer(&value).is_empty());
    }

    #[test]
    fn rejects_bad_status_and_raw_pointer_target() {
        let value = parse_json(
            r#"{
              "descriptor_type":"RollbackPointer",
              "descriptor_id":"rollback-pointer-2",
              "descriptor_hash":"sha256:rollbackpointer2",
              "artifact_class":"retrieval_trace_log_telemetry_shadow_rollback",
              "schema_version":"v0.2",
              "rollback_pointer_id":"rollback-pointer-2",
              "rollback_trigger_code":"activation_failed",
              "current_failed_descriptor_id":"E:\\raw\\snapshot.sqlite3",
              "current_failed_descriptor_hash":"failed2",
              "target_lkg_descriptor_id":"source_register",
              "target_lkg_descriptor_hash":"sha256:lkg2",
              "rollback_status":"auto",
              "rolled_back_at_bucket":"2026-04-26T14",
              "sanitized_incident_id":"incident-2"
            }"#,
        )
        .unwrap();

        let diagnostics = validate_rollback_pointer(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "invalid_rollback_status"));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "raw_path_or_locator_in_rollback_pointer"
        }));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "invalid_hash_shape"));
    }
}
