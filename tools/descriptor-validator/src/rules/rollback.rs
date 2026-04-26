use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

use super::{
    descriptor_id, descriptor_type, looks_like_raw_path_or_locator, string_array,
    validate_required_fields,
};

const RULE_ID: &str = "rollback_sanitized_only";

const REQUIRED_FIELDS: &[&str] = &[
    "descriptor_type",
    "descriptor_version",
    "descriptor_id",
    "descriptor_hash",
    "artifact_class",
    "schema_version",
    "rollback_trigger",
    "catastrophic_trigger",
    "rollback_trigger_code",
    "sanitized_incident_id",
    "timestamp_bucket",
    "disabled_path_name",
    "previous_path_name",
    "remediation_state",
    "rollback_pointer_id",
    "target_lkg_descriptor_id",
    "target_lkg_descriptor_hash",
];

const REQUIRED_ACTIONS: &[&str] = &[
    "stop_emitting_candidate_payload",
    "disable_candidate_retrieval_runtime_path",
    "keep_previous_deterministic_or_fallback_path_active",
    "require_controller_review_before_reenable",
];

const SANITIZED_POINTER_FIELDS: &[&str] = &[
    "disabled_path_name",
    "previous_path_name",
    "rollback_pointer_id",
    "target_lkg_descriptor_id",
    "target_lkg_descriptor_hash",
];

pub fn validate_rollback_descriptor(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    if descriptor_type != "RollbackDescriptor" {
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
    diagnostics.extend(validate_required_actions(
        descriptor,
        &descriptor_type,
        &descriptor_id,
    ));
    diagnostics.extend(validate_sanitized_pointer_fields(
        descriptor,
        &descriptor_type,
        &descriptor_id,
    ));

    diagnostics
}

fn validate_required_actions(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
) -> Vec<Diagnostic> {
    let actions = string_array(descriptor, "actions")
        .or_else(|| string_array(descriptor, "required_actions"));
    let Some(actions) = actions else {
        return vec![Diagnostic::new(
            descriptor_type,
            descriptor_id,
            "$.actions",
            "missing_required_actions",
            RULE_ID,
        )];
    };

    let mut diagnostics = Vec::new();
    for required_action in REQUIRED_ACTIONS {
        if !actions.contains(required_action) {
            diagnostics.push(Diagnostic::new(
                descriptor_type,
                descriptor_id,
                "$.actions",
                format!("missing_action_{required_action}"),
                RULE_ID,
            ));
        }
    }
    diagnostics
}

fn validate_sanitized_pointer_fields(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for field in SANITIZED_POINTER_FIELDS {
        let Some(value) = descriptor.get(field).and_then(JsonValue::as_str) else {
            continue;
        };
        if looks_like_raw_path_or_locator(value) {
            diagnostics.push(Diagnostic::new(
                descriptor_type,
                descriptor_id,
                format!("$.{field}"),
                "raw_path_or_locator_in_rollback_record",
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
    fn accepts_passing_rollback_descriptor() {
        let value = parse_json(
            r#"{
              "descriptor_type":"RollbackDescriptor",
              "descriptor_version":"v0.2",
              "descriptor_id":"rb-1",
              "descriptor_hash":"sha256:abc",
              "artifact_class":"retrieval_trace_log_telemetry_shadow_rollback",
              "schema_version":"v0.2",
              "rollback_trigger":"on_threshold_breach_or_gate_failure",
              "catastrophic_trigger":"on_any_structural_leak_or_secret_leak",
              "rollback_trigger_code":"leakage_detected",
              "sanitized_incident_id":"incident-1",
              "timestamp_bucket":"2026-04-26T12",
              "disabled_path_name":"candidate_retrieval_runtime_path",
              "previous_path_name":"deterministic_fallback_path",
              "remediation_state":"blocked_pending_review",
              "rollback_pointer_id":"rollback-pointer-1",
              "target_lkg_descriptor_id":"activation-lkg-1",
              "target_lkg_descriptor_hash":"sha256:lkg",
              "actions":[
                "stop_emitting_candidate_payload",
                "disable_candidate_retrieval_runtime_path",
                "keep_previous_deterministic_or_fallback_path_active",
                "require_controller_review_before_reenable"
              ]
            }"#,
        )
        .unwrap();

        assert!(validate_rollback_descriptor(&value).is_empty());
    }

    #[test]
    fn rejects_raw_path_and_missing_action() {
        let value = parse_json(
            r#"{
              "descriptor_type":"RollbackDescriptor",
              "descriptor_id":"rb-2",
              "rollback_trigger":"on_threshold_breach_or_gate_failure",
              "catastrophic_trigger":"on_any_structural_leak_or_secret_leak",
              "rollback_trigger_code":"leakage_detected",
              "sanitized_incident_id":"incident-2",
              "timestamp_bucket":"2026-04-26T12",
              "disabled_path_name":"E:\\secret\\snapshot.sqlite3",
              "previous_path_name":"deterministic_fallback_path",
              "remediation_state":"blocked",
              "rollback_pointer_id":"rollback-pointer-2",
              "target_lkg_descriptor_id":"activation-lkg-2",
              "target_lkg_descriptor_hash":"sha256:lkg",
              "actions":["stop_emitting_candidate_payload"]
            }"#,
        )
        .unwrap();

        let diagnostics = validate_rollback_descriptor(&value);
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "raw_path_or_locator_in_rollback_record"
        }));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class.starts_with("missing_action_")));
        assert!(diagnostics
            .iter()
            .all(|diagnostic| !diagnostic.to_string().contains("secret")));
    }
}
