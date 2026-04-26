use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

use super::{
    descriptor_id, descriptor_type, string_array, validate_bool_true, validate_required_fields,
    validate_string_equals, validate_zero,
};

const RULE_ID: &str = "purge_zero_residue";

const REQUIRED_FIELDS: &[&str] = &[
    "descriptor_type",
    "descriptor_version",
    "descriptor_id",
    "descriptor_hash",
    "artifact_class",
    "schema_version",
    "local_telemetry_only",
    "purge_required",
    "purge_scope",
    "purge_action",
    "purge_status",
    "files_removed_or_empty",
    "bytes_remaining",
    "raw_event_backups_remaining",
    "reconstructable_sensitive_content_remaining",
    "timestamp_bucket",
];

const ALLOWED_PURGE_SCOPE: &[&str] = &[
    "local_aggregate_files",
    "local_shadow_aggregate_files",
    "local_rollback_aggregate_files",
    "sidecar_backup_files",
];

const ALLOWED_PURGE_ACTION: &[&str] = &["remove_file", "truncate_to_empty"];

pub fn validate_purge_descriptor(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    if descriptor_type != "PurgeDescriptor" {
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
    diagnostics.extend(validate_bool_true(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "local_telemetry_only",
        "local_telemetry_only_not_true",
        RULE_ID,
    ));
    diagnostics.extend(validate_bool_true(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "purge_required",
        "purge_required_not_true",
        RULE_ID,
    ));
    diagnostics.extend(validate_bool_true(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "files_removed_or_empty",
        "files_removed_or_empty_not_true",
        RULE_ID,
    ));
    diagnostics.extend(validate_string_equals(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "purge_status",
        "passed",
        "purge_status_not_passed",
        RULE_ID,
    ));

    for field in [
        "bytes_remaining",
        "raw_event_backups_remaining",
        "reconstructable_sensitive_content_remaining",
    ] {
        diagnostics.extend(validate_zero(
            descriptor,
            &descriptor_type,
            &descriptor_id,
            field,
            "purge_residue_not_zero",
            RULE_ID,
        ));
    }

    diagnostics.extend(validate_allowed_string_array(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "purge_scope",
        ALLOWED_PURGE_SCOPE,
        "invalid_purge_scope",
    ));
    diagnostics.extend(validate_allowed_string_array(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "purge_action",
        ALLOWED_PURGE_ACTION,
        "invalid_purge_action",
    ));

    diagnostics
}

fn validate_allowed_string_array(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
    field: &str,
    allowed: &[&str],
    denied_class: &str,
) -> Vec<Diagnostic> {
    let Some(values) = string_array(descriptor, field) else {
        return vec![Diagnostic::new(
            descriptor_type,
            descriptor_id,
            format!("$.{field}"),
            denied_class,
            RULE_ID,
        )];
    };

    let mut diagnostics = Vec::new();
    for (index, value) in values.iter().enumerate() {
        if !allowed.contains(value) {
            diagnostics.push(Diagnostic::new(
                descriptor_type,
                descriptor_id,
                format!("$.{field}[{index}]"),
                denied_class,
                RULE_ID,
            ));
        }
    }
    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn accepts_passing_purge_descriptor() {
        let value = parse_json(
            r#"{
              "descriptor_type":"PurgeDescriptor",
              "descriptor_version":"v0.2",
              "descriptor_id":"purge-1",
              "descriptor_hash":"sha256:abc",
              "artifact_class":"retrieval_trace_log_telemetry_shadow_rollback",
              "schema_version":"v0.2",
              "local_telemetry_only":true,
              "purge_required":true,
              "purge_scope":["local_aggregate_files","sidecar_backup_files"],
              "purge_action":["remove_file","truncate_to_empty"],
              "purge_status":"passed",
              "files_removed_or_empty":true,
              "bytes_remaining":0,
              "raw_event_backups_remaining":0,
              "reconstructable_sensitive_content_remaining":0,
              "timestamp_bucket":"2026-04-26T12"
            }"#,
        )
        .unwrap();

        assert!(validate_purge_descriptor(&value).is_empty());
    }

    #[test]
    fn rejects_purge_residue_and_invalid_scope() {
        let value = parse_json(
            r#"{
              "descriptor_type":"PurgeDescriptor",
              "descriptor_id":"purge-2",
              "local_telemetry_only":true,
              "purge_required":true,
              "purge_scope":["raw_event_files"],
              "purge_action":["delete_everything"],
              "purge_status":"failed",
              "files_removed_or_empty":false,
              "bytes_remaining":12,
              "raw_event_backups_remaining":1,
              "reconstructable_sensitive_content_remaining":1,
              "timestamp_bucket":"2026-04-26T12"
            }"#,
        )
        .unwrap();

        let diagnostics = validate_purge_descriptor(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "purge_residue_not_zero"));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "invalid_purge_scope"));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "invalid_purge_action"));
        assert!(diagnostics
            .iter()
            .all(|diagnostic| !diagnostic.to_string().contains("delete_everything")));
    }
}
