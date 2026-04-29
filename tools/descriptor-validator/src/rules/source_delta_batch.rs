use std::collections::HashSet;

use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

use super::{
    descriptor_id, descriptor_type, string_array, validate_hash_fields, validate_required_fields,
    validate_string_equals, validate_string_in,
};

const RULE_ID: &str = "source_delta_batch_static";

const REQUIRED_FIELDS: &[&str] = &[
    "descriptor_type",
    "descriptor_version",
    "descriptor_id",
    "descriptor_hash",
    "artifact_class",
    "schema_version",
    "source_delta_batch_hash",
    "source_delta_count",
    "source_freshness_digest",
    "freshness_status",
    "stale_reason_code",
    "source_ids",
    "content_hashes",
    "previous_content_hashes",
    "review_status_summary",
    "effective_confidence_summary",
    "ingested_at_bucket",
    "leakage_count",
    "all_sources_runtime_excluded",
];

const ALLOWED_FIELDS: &[&str] = &[
    "descriptor_type",
    "descriptor_version",
    "descriptor_id",
    "descriptor_hash",
    "artifact_class",
    "schema_version",
    "source_delta_batch_hash",
    "source_delta_count",
    "source_freshness_digest",
    "freshness_status",
    "stale_reason_code",
    "source_ids",
    "content_hashes",
    "previous_content_hashes",
    "review_status_summary",
    "effective_confidence_summary",
    "ingested_at_bucket",
    "controller_review_id",
    "reviewed_at_bucket",
    "review_status_counts",
    "confidence_bucket_counts",
    "rejected_source_delta_count",
    "accepted_source_delta_count",
    "quarantined_source_delta_count",
    "limited_source_delta_count",
    "all_required_reviews_present",
    "all_hashes_normalized",
    "all_sources_locator_sanitized",
    "all_sources_runtime_excluded",
    "activation_requested",
    "activation_blocked_reason_codes",
    "leakage_count",
    "notes_summary_ref",
];

const HASH_FIELDS: &[&str] = &[
    "descriptor_hash",
    "source_delta_batch_hash",
    "source_freshness_digest",
];

const FRESHNESS_STATUS: &[&str] = &[
    "fresh",
    "stale_source",
    "stale_index",
    "stale_eval",
    "stale_snapshot",
    "activation_failed",
    "unknown",
    "blocked",
];

const STALE_REASON_CODE: &[&str] = &[
    "source_delta_pending_review",
    "source_delta_rejected",
    "snapshot_rebuild_required",
    "index_hash_mismatch",
    "index_predicate_mismatch",
    "eval_truth_stale",
    "eval_model_or_judge_stale",
    "snapshot_hash_mismatch",
    "activation_descriptor_invalid",
    "activation_policy_blocked",
    "unknown_freshness",
];

const STATUS_COUNT_FIELDS: &[&str] = &[
    "accepted_source_delta_count",
    "rejected_source_delta_count",
    "quarantined_source_delta_count",
    "limited_source_delta_count",
];

pub fn validate_source_delta_batch_descriptor(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    if descriptor_type != "SourceDeltaBatch" {
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
    diagnostics.extend(validate_unknown_fields(
        descriptor,
        &descriptor_type,
        &descriptor_id,
    ));
    diagnostics.extend(validate_string_equals(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "artifact_class",
        "governance_descriptor",
        "source_delta_artifact_class_mismatch",
        RULE_ID,
    ));
    diagnostics.extend(validate_string_in(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "freshness_status",
        FRESHNESS_STATUS,
        "invalid_freshness_status",
        RULE_ID,
    ));
    diagnostics.extend(validate_string_in(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "stale_reason_code",
        STALE_REASON_CODE,
        "invalid_stale_reason_code",
        RULE_ID,
    ));
    diagnostics.extend(validate_hash_fields(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        HASH_FIELDS,
        RULE_ID,
    ));
    diagnostics.extend(validate_hash_array(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "content_hashes",
    ));
    diagnostics.extend(validate_hash_array(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "previous_content_hashes",
    ));
    diagnostics.extend(validate_count_consistency(
        descriptor,
        &descriptor_type,
        &descriptor_id,
    ));
    diagnostics.extend(validate_activation_preconditions(
        descriptor,
        &descriptor_type,
        &descriptor_id,
    ));
    diagnostics.extend(validate_runtime_exclusion(
        descriptor,
        &descriptor_type,
        &descriptor_id,
    ));
    diagnostics.extend(validate_leakage_zero(
        descriptor,
        &descriptor_type,
        &descriptor_id,
    ));

    diagnostics
}

fn validate_unknown_fields(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
) -> Vec<Diagnostic> {
    let allowed = ALLOWED_FIELDS.iter().copied().collect::<HashSet<_>>();
    let mut diagnostics = Vec::new();
    let Some(fields) = descriptor.fields() else {
        return diagnostics;
    };

    for (field, _) in fields {
        if !allowed.contains(field.as_str()) {
            diagnostics.push(Diagnostic::new(
                descriptor_type,
                descriptor_id,
                format!("$.{field}"),
                "source_delta_unknown_field",
                RULE_ID,
            ));
        }
    }

    diagnostics
}

fn validate_hash_array(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
    field: &str,
) -> Vec<Diagnostic> {
    let Some(values) = string_array(descriptor, field) else {
        return vec![Diagnostic::new(
            descriptor_type,
            descriptor_id,
            format!("$.{field}"),
            "invalid_hash_array",
            RULE_ID,
        )];
    };

    values
        .iter()
        .enumerate()
        .filter_map(|(index, value)| {
            is_hash_like(value).then_some(()).map_or_else(
                || {
                    Some(Diagnostic::new(
                        descriptor_type,
                        descriptor_id,
                        format!("$.{field}[{index}]"),
                        "invalid_hash_shape",
                        RULE_ID,
                    ))
                },
                |_| None,
            )
        })
        .collect()
}

fn validate_count_consistency(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let Some(source_delta_count) = integer_field(descriptor, "source_delta_count") else {
        diagnostics.push(Diagnostic::new(
            descriptor_type,
            descriptor_id,
            "$.source_delta_count",
            "source_delta_count_not_integer",
            RULE_ID,
        ));
        return diagnostics;
    };

    for field in ["source_ids", "content_hashes", "previous_content_hashes"] {
        if let Some(values) = string_array(descriptor, field) {
            if values.len() as i64 != source_delta_count {
                diagnostics.push(Diagnostic::new(
                    descriptor_type,
                    descriptor_id,
                    format!("$.{field}"),
                    "source_delta_count_mismatch",
                    RULE_ID,
                ));
            }
        }
    }

    let mut status_sum = 0;
    let mut saw_status_count = false;
    for field in STATUS_COUNT_FIELDS {
        if let Some(value) = integer_field(descriptor, field) {
            saw_status_count = true;
            status_sum += value;
        }
    }
    if status_sum > source_delta_count {
        diagnostics.push(Diagnostic::new(
            descriptor_type,
            descriptor_id,
            "$.source_delta_count",
            "source_delta_status_count_exceeds_total",
            RULE_ID,
        ));
    }
    if bool_field(descriptor, "all_required_reviews_present") == Some(true)
        && saw_status_count
        && status_sum != source_delta_count
    {
        diagnostics.push(Diagnostic::new(
            descriptor_type,
            descriptor_id,
            "$.source_delta_count",
            "source_delta_reviewed_status_count_mismatch",
            RULE_ID,
        ));
    }

    diagnostics
}

fn validate_activation_preconditions(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
) -> Vec<Diagnostic> {
    if bool_field(descriptor, "activation_requested") == Some(true)
        && bool_field(descriptor, "all_required_reviews_present") != Some(true)
    {
        return vec![Diagnostic::new(
            descriptor_type,
            descriptor_id,
            "$.all_required_reviews_present",
            "source_delta_activation_without_review",
            RULE_ID,
        )];
    }
    Vec::new()
}

fn validate_runtime_exclusion(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
) -> Vec<Diagnostic> {
    if bool_field(descriptor, "all_sources_runtime_excluded") == Some(true) {
        return Vec::new();
    }
    vec![Diagnostic::new(
        descriptor_type,
        descriptor_id,
        "$.all_sources_runtime_excluded",
        "source_delta_runtime_exclusion_missing",
        RULE_ID,
    )]
}

fn validate_leakage_zero(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
) -> Vec<Diagnostic> {
    match descriptor.get("leakage_count") {
        Some(value) if value.is_number_zero() => Vec::new(),
        Some(_) | None => vec![Diagnostic::new(
            descriptor_type,
            descriptor_id,
            "$.leakage_count",
            "leakage_count_not_zero",
            RULE_ID,
        )],
    }
}

fn bool_field(descriptor: &JsonValue, field: &str) -> Option<bool> {
    descriptor.get(field).and_then(JsonValue::as_bool)
}

fn integer_field(descriptor: &JsonValue, field: &str) -> Option<i64> {
    let JsonValue::Number(raw) = descriptor.get(field)? else {
        return None;
    };
    raw.parse::<i64>().ok()
}

fn is_hash_like(value: &str) -> bool {
    let Some(digest) = value.strip_prefix("sha256:") else {
        return false;
    };
    !digest.is_empty()
        && digest
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn accepts_passing_source_delta_batch() {
        let value = parse_json(PASSING_SOURCE_DELTA_BATCH).unwrap();
        assert!(validate_source_delta_batch_descriptor(&value).is_empty());
    }

    #[test]
    fn rejects_count_mismatch_and_activation_without_review() {
        let value = parse_json(
            r#"{
              "descriptor_type":"SourceDeltaBatch",
              "descriptor_version":"v0.2",
              "descriptor_id":"source-delta-bad-count",
              "descriptor_hash":"sha256:sourceDeltaBadCount",
              "artifact_class":"governance_descriptor",
              "schema_version":"v0.2",
              "source_delta_batch_hash":"sha256:batchBadCount",
              "source_delta_count":2,
              "source_freshness_digest":"sha256:freshnessBadCount",
              "freshness_status":"stale_source",
              "stale_reason_code":"source_delta_pending_review",
              "source_ids":["source-a"],
              "content_hashes":["sha256:contentA"],
              "previous_content_hashes":["sha256:previousA"],
              "review_status_summary":"pending",
              "effective_confidence_summary":"unknown",
              "ingested_at_bucket":"2026-04-30T10",
              "activation_requested":true,
              "all_required_reviews_present":false,
              "all_sources_runtime_excluded":true,
              "leakage_count":0
            }"#,
        )
        .unwrap();

        let diagnostics = validate_source_delta_batch_descriptor(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "source_delta_count_mismatch"));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "source_delta_activation_without_review"
        }));
    }

    #[test]
    fn rejects_bad_hash_leakage_and_unknown_field() {
        let value = parse_json(
            r#"{
              "descriptor_type":"SourceDeltaBatch",
              "descriptor_version":"v0.2",
              "descriptor_id":"source-delta-bad-hash",
              "descriptor_hash":"sha256:sourceDeltaBadHash",
              "artifact_class":"governance_descriptor",
              "schema_version":"v0.2",
              "source_delta_batch_hash":"sha256:batchBadHash",
              "source_delta_count":1,
              "source_freshness_digest":"sha256:freshnessBadHash",
              "freshness_status":"fresh",
              "stale_reason_code":"unknown_freshness",
              "source_ids":["source-a"],
              "content_hashes":["not-a-hash"],
              "previous_content_hashes":["sha256:previousA"],
              "review_status_summary":"reviewed",
              "effective_confidence_summary":"high",
              "ingested_at_bucket":"2026-04-30T10",
              "all_sources_runtime_excluded":true,
              "leakage_count":1,
              "unexpected_debug_field":"synthetic"
            }"#,
        )
        .unwrap();

        let diagnostics = validate_source_delta_batch_descriptor(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "invalid_hash_shape"));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "leakage_count_not_zero"));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "source_delta_unknown_field"));
        assert!(diagnostics
            .iter()
            .all(|diagnostic| !diagnostic.to_string().contains("synthetic")));
    }

    const PASSING_SOURCE_DELTA_BATCH: &str = r#"{
      "descriptor_type":"SourceDeltaBatch",
      "descriptor_version":"v0.2",
      "descriptor_id":"source-delta-pass",
      "descriptor_hash":"sha256:sourceDeltaPass",
      "artifact_class":"governance_descriptor",
      "schema_version":"v0.2",
      "source_delta_batch_hash":"sha256:batchPass",
      "source_delta_count":1,
      "source_freshness_digest":"sha256:freshnessPass",
      "freshness_status":"fresh",
      "stale_reason_code":"unknown_freshness",
      "source_ids":["source-a"],
      "content_hashes":["sha256:contentA"],
      "previous_content_hashes":["sha256:previousA"],
      "review_status_summary":"reviewed",
      "effective_confidence_summary":"high",
      "ingested_at_bucket":"2026-04-30T10",
      "review_status_counts":{"reviewed":1},
      "confidence_bucket_counts":{"high":1},
      "accepted_source_delta_count":1,
      "rejected_source_delta_count":0,
      "quarantined_source_delta_count":0,
      "limited_source_delta_count":0,
      "all_required_reviews_present":true,
      "all_hashes_normalized":true,
      "all_sources_locator_sanitized":true,
      "all_sources_runtime_excluded":true,
      "activation_requested":true,
      "activation_blocked_reason_codes":[],
      "leakage_count":0,
      "notes_summary_ref":"source-delta-summary-1"
    }"#;
}
