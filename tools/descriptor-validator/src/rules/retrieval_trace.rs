use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

use super::{
    descriptor_id, descriptor_type, is_empty_array, string_field, validate_number_between,
    validate_required_fields, validate_string_in,
};

const RULE_ID: &str = "retrieval_trace_stale_selection";

const REQUIRED_FIELDS: &[&str] = &[
    "descriptor_type",
    "descriptor_version",
    "descriptor_hash",
    "artifact_class",
    "schema_version",
    "trace_id",
    "query_type",
    "resolved_intent",
    "candidate_pool_size",
    "raw_bm25_score",
    "max_in_pool",
    "normalized_score",
    "min_candidate_score",
    "fallback_reason_code",
    "freshness_status",
    "activation_status",
    "stale_reason_code",
    "snapshot_hash",
    "index_hash",
    "selected_sample_ids",
    "selected_kb_rules",
    "payload_bytes",
    "source_delta_count",
    "full_kb_rows_included",
    "expensive_path_used",
];

const QUERY_TYPES: &[&str] = &["governance_query", "runtime_query"];

const STRICT_EMPTY_SELECTION_REASONS: &[&str] =
    &["on_index_miss", "on_stale_index", "on_stale_snapshot"];

pub fn validate_retrieval_trace(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    if descriptor_type != "RetrievalTrace" {
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
        "query_type",
        QUERY_TYPES,
        "invalid_query_type",
        RULE_ID,
    ));
    diagnostics.extend(validate_number_between(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "normalized_score",
        0.0,
        1.0,
        "normalized_score_out_of_range",
        RULE_ID,
    ));
    diagnostics.extend(validate_stale_selection_empty(
        descriptor,
        &descriptor_type,
        &descriptor_id,
    ));

    diagnostics
}

fn validate_stale_selection_empty(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
) -> Vec<Diagnostic> {
    let fallback_reason = string_field(descriptor, "fallback_reason_code")
        .or_else(|| string_field(descriptor, "fallback_reason"));
    let freshness_status = string_field(descriptor, "freshness_status");

    let must_be_empty = fallback_reason
        .map(|value| STRICT_EMPTY_SELECTION_REASONS.contains(&value))
        .unwrap_or(false)
        || matches!(freshness_status, Some("stale_index" | "stale_snapshot"));

    if !must_be_empty {
        return Vec::new();
    }

    let mut diagnostics = Vec::new();
    for field in ["selected_sample_ids", "selected_kb_rules"] {
        if is_empty_array(descriptor, field) != Some(true) {
            diagnostics.push(Diagnostic::new(
                descriptor_type,
                descriptor_id,
                format!("$.{field}"),
                "selected_ids_must_be_empty_for_stale_or_index_miss",
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
    fn accepts_stale_snapshot_with_empty_selection() {
        let value = parse_json(
            r#"{
              "descriptor_type":"RetrievalTrace",
              "descriptor_version":"v0.2",
              "descriptor_hash":"sha256:t",
              "artifact_class":"retrieval_trace_log_telemetry_shadow_rollback",
              "schema_version":"v0.2",
              "trace_id":"t1",
              "query_type":"runtime_query",
              "resolved_intent":"generate_storyboard",
              "candidate_pool_size":0,
              "raw_bm25_score":0,
              "max_in_pool":0,
              "normalized_score":0,
              "min_candidate_score":0.15,
              "fallback_reason_code":"on_stale_snapshot",
              "freshness_status":"stale_snapshot",
              "activation_status":"activated",
              "stale_reason_code":"snapshot_hash_mismatch",
              "snapshot_hash":"sha256:s",
              "index_hash":"sha256:i",
              "selected_sample_ids":[],
              "selected_kb_rules":[],
              "payload_bytes":0,
              "source_delta_count":0,
              "full_kb_rows_included":0,
              "expensive_path_used":false
            }"#,
        )
        .unwrap();

        assert!(validate_retrieval_trace(&value).is_empty());
    }

    #[test]
    fn rejects_stale_index_selection_and_bad_score() {
        let value = parse_json(
            r#"{
              "descriptor_type":"RetrievalTrace",
              "descriptor_version":"v0.2",
              "descriptor_hash":"sha256:t",
              "artifact_class":"retrieval_trace_log_telemetry_shadow_rollback",
              "schema_version":"v0.2",
              "trace_id":"t2",
              "query_type":"runtime_query",
              "resolved_intent":"generate_storyboard",
              "candidate_pool_size":1,
              "raw_bm25_score":9,
              "max_in_pool":9,
              "normalized_score":1.5,
              "min_candidate_score":0.15,
              "fallback_reason_code":"on_stale_index",
              "freshness_status":"stale_index",
              "activation_status":"activated",
              "stale_reason_code":"index_hash_mismatch",
              "snapshot_hash":"sha256:s",
              "index_hash":"sha256:i",
              "selected_sample_ids":["GS-1"],
              "selected_kb_rules":["RULE-1"],
              "payload_bytes":120,
              "source_delta_count":0,
              "full_kb_rows_included":0,
              "expensive_path_used":false
            }"#,
        )
        .unwrap();

        let diagnostics = validate_retrieval_trace(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "normalized_score_out_of_range"));
        assert_eq!(
            diagnostics
                .iter()
                .filter(|diagnostic| {
                    diagnostic.denied_class == "selected_ids_must_be_empty_for_stale_or_index_miss"
                })
                .count(),
            2
        );
    }
}
