use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

use super::{
    descriptor_id, descriptor_type, validate_bool_true, validate_optional_bool_false,
    validate_required_bool_false, validate_required_fields, validate_string_equals,
};

const RULE_ID: &str = "future_qa_candidate_non_truth";

const REQUIRED_FIELDS: &[&str] = &[
    "descriptor_type",
    "descriptor_version",
    "descriptor_id",
    "descriptor_hash",
    "artifact_class",
    "schema_version",
    "candidate_id",
    "source_query_type",
    "source_intent",
    "candidate_question",
    "candidate_expected_behavior",
    "candidate_failure_mode",
    "selected_sample_ids",
    "selected_kb_rules",
    "kb_context_summary",
    "retrieval_trace_ref",
    "review_status",
    "created_at_bucket",
    "judged_against_model",
    "judged_at_bucket",
    "source_freshness_status",
    "freshness_status",
    "freshness_reason_code",
    "stale_reason_code",
    "stale_if_model_changes",
    "stale_action",
    "is_eval_truth",
    "enters_eval_truth_by_default",
    "requires_human_review",
    "requires_git_promotion",
    "requires_rebuild_and_validation",
];

pub fn validate_future_qa_candidate(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    if descriptor_type != "FutureQACandidate" {
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
    diagnostics.extend(validate_required_bool_false(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "is_eval_truth",
        "future_qa_candidate_is_eval_truth",
        RULE_ID,
    ));
    diagnostics.extend(validate_required_bool_false(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "enters_eval_truth_by_default",
        "future_qa_candidate_enters_eval_truth_by_default",
        RULE_ID,
    ));
    diagnostics.extend(validate_optional_bool_false(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "future_qa_candidates_counted_as_truth",
        "future_qa_candidates_counted_as_truth_not_false",
        RULE_ID,
    ));
    diagnostics.extend(validate_optional_bool_false(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "promoted_by_git",
        "future_qa_candidate_in_place_promotion_not_allowed",
        RULE_ID,
    ));
    diagnostics.extend(validate_bool_true(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "stale_if_model_changes",
        "future_qa_stale_if_model_changes_not_true",
        RULE_ID,
    ));
    diagnostics.extend(validate_string_equals(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "stale_action",
        "block_auto_switch",
        "future_qa_stale_action_not_block_auto_switch",
        RULE_ID,
    ));
    diagnostics.extend(validate_bool_true(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "requires_human_review",
        "future_qa_requires_human_review_not_true",
        RULE_ID,
    ));
    diagnostics.extend(validate_bool_true(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "requires_git_promotion",
        "future_qa_requires_git_promotion_not_true",
        RULE_ID,
    ));
    diagnostics.extend(validate_bool_true(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "requires_rebuild_and_validation",
        "future_qa_requires_rebuild_and_validation_not_true",
        RULE_ID,
    ));

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn accepts_non_truth_future_qa_candidate() {
        let value = parse_json(
            r#"{
              "descriptor_type":"FutureQACandidate",
              "descriptor_version":"v0.2",
              "descriptor_id":"fqa-1",
              "descriptor_hash":"sha256:fqa",
              "artifact_class":"future_qa_descriptor",
              "schema_version":"v0.2",
              "candidate_id":"fqa-1",
              "source_query_type":"runtime_query",
              "source_intent":"generate_storyboard",
              "candidate_question":"question",
              "candidate_expected_behavior":"behavior",
              "candidate_failure_mode":"stale_eval",
              "selected_sample_ids":[],
              "selected_kb_rules":[],
              "kb_context_summary":"summary",
              "retrieval_trace_ref":"trace-1",
              "review_status":"candidate",
              "created_at_bucket":"2026-04-26T12",
              "judged_against_model":"judge-v1",
              "judged_at_bucket":"2026-04-26T12",
              "source_freshness_status":"fresh",
              "freshness_status":"fresh",
              "freshness_reason_code":"none",
              "stale_reason_code":"unknown_freshness",
              "stale_if_model_changes":true,
              "stale_action":"block_auto_switch",
              "is_eval_truth":false,
              "enters_eval_truth_by_default":false,
              "requires_human_review":true,
              "requires_git_promotion":true,
              "requires_rebuild_and_validation":true,
              "promoted_by_git":false
            }"#,
        )
        .unwrap();

        assert!(validate_future_qa_candidate(&value).is_empty());
    }

    #[test]
    fn rejects_truth_and_in_place_promotion() {
        let value = parse_json(
            r#"{
              "descriptor_type":"FutureQACandidate",
              "descriptor_id":"fqa-2",
              "is_eval_truth":true,
              "enters_eval_truth_by_default":true,
              "future_qa_candidates_counted_as_truth":true,
              "promoted_by_git":true,
              "requires_human_review":false,
              "requires_git_promotion":false,
              "requires_rebuild_and_validation":false
            }"#,
        )
        .unwrap();

        let diagnostics = validate_future_qa_candidate(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "future_qa_candidate_is_eval_truth"));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "future_qa_candidate_enters_eval_truth_by_default"
        }));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "future_qa_candidate_in_place_promotion_not_allowed"
        }));
    }
}
