use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

use super::{descriptor_id, descriptor_type, validate_required_fields};

const RULE_ID: &str = "refresh_telemetry_required_fields";

const REQUIRED_FIELDS: &[&str] = &[
    "descriptor_type",
    "descriptor_version",
    "descriptor_id",
    "descriptor_hash",
    "artifact_class",
    "schema_version",
    "snapshot_hash",
    "index_hash",
    "activation_status",
    "freshness_status",
    "stale_reason_code",
    "fallback_reason_code",
    "source_delta_count",
    "payload_bytes",
    "selected_sample_ids",
    "selected_kb_rules",
    "lint_rule_ids",
    "leakage_count",
    "timestamp_bucket",
];

pub fn validate_refresh_telemetry_record(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    if descriptor_type != "RefreshTelemetryRecord" {
        return Vec::new();
    }
    let descriptor_id = descriptor_id(descriptor);

    validate_required_fields(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        REQUIRED_FIELDS,
        RULE_ID,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn accepts_passing_refresh_telemetry_record() {
        let value = parse_json(
            r#"{
              "descriptor_type":"RefreshTelemetryRecord",
              "descriptor_version":"v0.2",
              "descriptor_id":"rt-1",
              "descriptor_hash":"sha256:abc",
              "artifact_class":"retrieval_trace_log_telemetry_shadow_rollback",
              "schema_version":"v0.2",
              "snapshot_hash":"sha256:s",
              "index_hash":"sha256:i",
              "activation_status":"activated",
              "freshness_status":"fresh",
              "stale_reason_code":"unknown_freshness",
              "fallback_reason_code":"on_empty_pool",
              "source_delta_count":0,
              "payload_bytes":100,
              "selected_sample_ids":[],
              "selected_kb_rules":[],
              "lint_rule_ids":[],
              "leakage_count":0,
              "timestamp_bucket":"2026-04-26T12"
            }"#,
        )
        .unwrap();

        assert!(validate_refresh_telemetry_record(&value).is_empty());
    }

    #[test]
    fn rejects_missing_refresh_telemetry_required_fields() {
        let value = parse_json(
            r#"{
              "descriptor_type":"RefreshTelemetryRecord",
              "descriptor_id":"rt-2",
              "artifact_class":"retrieval_trace_log_telemetry_shadow_rollback",
              "leakage_count":0
            }"#,
        )
        .unwrap();

        let diagnostics = validate_refresh_telemetry_record(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.field_path == "$.snapshot_hash"));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.field_path == "$.timestamp_bucket"));
    }
}
