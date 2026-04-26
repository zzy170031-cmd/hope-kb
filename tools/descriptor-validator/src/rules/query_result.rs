use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

use super::{
    descriptor_id, descriptor_type, is_empty_array, string_field, validate_required_fields,
    validate_string_in,
};

const RULE_ID: &str = "query_result_stale_selection";

const REQUIRED_FIELDS: &[&str] = &[
    "descriptor_type",
    "descriptor_version",
    "descriptor_hash",
    "artifact_class",
    "schema_version",
    "query_type",
    "resolved_intent",
    "intent_confidence",
    "intent_routing_status",
    "selected_sample_ids",
    "selected_kb_rules",
    "kb_context_summary",
    "retrieval_trace_ref",
    "fallback_reason_code",
    "freshness_status",
    "activation_status",
    "snapshot_hash",
    "index_hash",
    "full_kb_rows_included",
];

const QUERY_TYPES: &[&str] = &["governance_query", "runtime_query"];

const STRICT_EMPTY_SELECTION_REASONS: &[&str] =
    &["on_index_miss", "on_stale_index", "on_stale_snapshot"];

pub fn validate_query_result(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    if descriptor_type != "QueryResult" {
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
    fn accepts_index_miss_with_empty_selection() {
        let value = parse_json(
            r#"{
              "descriptor_type":"QueryResult",
              "descriptor_version":"v0.2",
              "descriptor_hash":"sha256:q",
              "artifact_class":"prompt_payload",
              "schema_version":"v0.2",
              "descriptor_id":"q1",
              "query_type":"runtime_query",
              "resolved_intent":"generate_storyboard",
              "intent_confidence":1,
              "intent_routing_status":"resolved",
              "selected_sample_ids":[],
              "selected_kb_rules":[],
              "kb_context_summary":"summary",
              "retrieval_trace_ref":"trace-1",
              "fallback_reason_code":"on_index_miss",
              "freshness_status":"fresh",
              "activation_status":"activated",
              "snapshot_hash":"sha256:s",
              "index_hash":"sha256:i",
              "full_kb_rows_included":0
            }"#,
        )
        .unwrap();

        assert!(validate_query_result(&value).is_empty());
    }

    #[test]
    fn rejects_stale_index_with_selected_ids() {
        let value = parse_json(
            r#"{
              "descriptor_type":"QueryResult",
              "descriptor_version":"v0.2",
              "descriptor_hash":"sha256:q",
              "artifact_class":"prompt_payload",
              "schema_version":"v0.2",
              "descriptor_id":"q2",
              "query_type":"runtime_query",
              "resolved_intent":"generate_storyboard",
              "intent_confidence":1,
              "intent_routing_status":"resolved",
              "selected_sample_ids":["GS-1"],
              "selected_kb_rules":["RULE-1"],
              "kb_context_summary":"summary",
              "retrieval_trace_ref":"trace-2",
              "fallback_reason_code":"on_stale_index",
              "freshness_status":"stale_index",
              "activation_status":"activated",
              "snapshot_hash":"sha256:s",
              "index_hash":"sha256:i",
              "full_kb_rows_included":0
            }"#,
        )
        .unwrap();

        let diagnostics = validate_query_result(&value);
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
