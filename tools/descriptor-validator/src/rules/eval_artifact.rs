use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

use super::{
    descriptor_id, descriptor_type, validate_bool_true, validate_number_at_least,
    validate_required_bool_false, validate_required_fields, validate_string_equals,
};

const RULE_ID: &str = "eval_artifact_router_gate";

const REQUIRED_FIELDS: &[&str] = &[
    "descriptor_type",
    "descriptor_version",
    "descriptor_id",
    "descriptor_hash",
    "artifact_class",
    "schema_version",
    "eval_artifact_id",
    "snapshot_hash",
    "index_hash",
    "intent_label_set_hash",
    "eval_truth_set_hash",
    "judged_against_model",
    "judged_at_bucket",
    "freshness_checked_at_bucket",
    "freshness_status",
    "stale_reason_code",
    "stale_if_model_changes",
    "stale_action",
    "auto_switch_allowed",
    "blocked_reason_codes",
    "intent_routing_accuracy",
    "pass_rate_overall",
    "pass_rate_per_intent",
    "tail_failure_rate",
    "max_tail_failure_rate_absolute",
    "min_samples_per_intent_in_eval_set",
    "max_per_intent_drop_pp",
    "future_qa_candidates_counted_as_truth",
];

pub fn validate_eval_artifact(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    if descriptor_type != "EvalArtifact" {
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
    diagnostics.extend(validate_string_equals(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "stale_action",
        "block_auto_switch",
        "stale_action_not_block_auto_switch",
        RULE_ID,
    ));
    diagnostics.extend(validate_bool_true(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "stale_if_model_changes",
        "stale_if_model_changes_not_true",
        RULE_ID,
    ));
    diagnostics.extend(validate_required_bool_false(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "future_qa_candidates_counted_as_truth",
        "future_qa_candidates_counted_as_truth_not_false",
        RULE_ID,
    ));
    diagnostics.extend(validate_number_at_least(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "min_samples_per_intent_in_eval_set",
        20.0,
        "min_samples_per_intent_below_floor",
        RULE_ID,
    ));
    diagnostics.extend(validate_non_empty_object(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "pass_rate_per_intent",
        "pass_rate_per_intent_missing_or_empty",
    ));

    diagnostics
}

fn validate_non_empty_object(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
    field: &str,
    denied_class: &str,
) -> Vec<Diagnostic> {
    match descriptor.get(field) {
        Some(JsonValue::Object(fields)) if !fields.is_empty() => Vec::new(),
        Some(_) | None => vec![Diagnostic::new(
            descriptor_type,
            descriptor_id,
            format!("$.{field}"),
            denied_class,
            RULE_ID,
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn accepts_passing_eval_artifact() {
        let value = parse_json(
            r#"{
              "descriptor_type":"EvalArtifact",
              "descriptor_version":"v0.2",
              "descriptor_id":"eval-1",
              "descriptor_hash":"sha256:eval",
              "artifact_class":"eval_descriptor",
              "schema_version":"v0.2",
              "eval_artifact_id":"eval-1",
              "snapshot_hash":"sha256:s",
              "index_hash":"sha256:i",
              "intent_label_set_hash":"sha256:intent",
              "eval_truth_set_hash":"sha256:truth",
              "judged_against_model":"judge-v1",
              "judged_at_bucket":"2026-04-26T12",
              "freshness_checked_at_bucket":"2026-04-26T12",
              "freshness_status":"fresh",
              "stale_reason_code":"unknown_freshness",
              "stale_if_model_changes":true,
              "stale_action":"block_auto_switch",
              "auto_switch_allowed":false,
              "blocked_reason_codes":[],
              "intent_routing_accuracy":1,
              "pass_rate_overall":1,
              "pass_rate_per_intent":{"generate_storyboard":1},
              "tail_failure_rate":0,
              "max_tail_failure_rate_absolute":0.02,
              "min_samples_per_intent_in_eval_set":20,
              "max_per_intent_drop_pp":0.05,
              "future_qa_candidates_counted_as_truth":false
            }"#,
        )
        .unwrap();

        assert!(validate_eval_artifact(&value).is_empty());
    }

    #[test]
    fn rejects_stale_action_truth_count_and_sample_floor() {
        let value = parse_json(
            r#"{
              "descriptor_type":"EvalArtifact",
              "descriptor_id":"eval-2",
              "stale_if_model_changes":false,
              "stale_action":"warn_only",
              "future_qa_candidates_counted_as_truth":true,
              "min_samples_per_intent_in_eval_set":12,
              "pass_rate_per_intent":{}
            }"#,
        )
        .unwrap();

        let diagnostics = validate_eval_artifact(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "stale_action_not_block_auto_switch"));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "future_qa_candidates_counted_as_truth_not_false"
        }));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "min_samples_per_intent_below_floor"));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "pass_rate_per_intent_missing_or_empty"
        }));
    }
}
