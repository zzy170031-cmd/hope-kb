use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

use super::{
    descriptor_id, descriptor_type, validate_hash_fields, validate_no_raw_locator_fields,
    validate_required_fields, validate_string_equals, validate_string_in,
};

const RULE_ID: &str = "activation_descriptor_static";

const REQUIRED_FIELDS: &[&str] = &[
    "descriptor_type",
    "descriptor_version",
    "descriptor_id",
    "descriptor_hash",
    "artifact_class",
    "schema_version",
    "seed_bundle_hash",
    "manifest_hash",
    "snapshot_hash",
    "index_hash",
    "snapshot_version",
    "snapshot_name",
    "record_counts",
    "runtime_selection_predicates",
    "quarantine_exclusion_predicates",
    "candidate_exclusion_predicates",
    "source_delta_batch_hash",
    "source_delta_count",
    "source_freshness_digest",
    "activation_status",
    "freshness_status",
    "verification_status",
    "active_pointer",
    "last_known_good_snapshot",
    "last_known_good_activation_descriptor",
    "rollback_pointer",
    "activation_verified_at_bucket",
    "controller_approval_id",
];

const HASH_FIELDS: &[&str] = &[
    "descriptor_hash",
    "seed_bundle_hash",
    "manifest_hash",
    "snapshot_hash",
    "index_hash",
    "source_delta_batch_hash",
    "source_freshness_digest",
];

const POINTER_FIELDS: &[&str] = &[
    "active_pointer",
    "last_known_good_snapshot",
    "last_known_good_activation_descriptor",
    "rollback_pointer",
];

const ACTIVATION_STATUS: &[&str] = &[
    "candidate",
    "validating",
    "verified",
    "activated",
    "failed",
    "rolled_back",
    "superseded",
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

pub fn validate_activation_descriptor(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    if descriptor_type != "ActivationDescriptor" {
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
        "artifact_class",
        "activation_descriptor",
        "activation_artifact_class_mismatch",
        RULE_ID,
    ));
    diagnostics.extend(validate_string_in(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "activation_status",
        ACTIVATION_STATUS,
        "invalid_activation_status",
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
        "raw_path_or_locator_in_activation_pointer",
        RULE_ID,
    ));
    diagnostics.extend(validate_status_pair(
        descriptor,
        &descriptor_type,
        &descriptor_id,
    ));

    diagnostics
}

fn validate_status_pair(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
) -> Vec<Diagnostic> {
    let activation = descriptor
        .get("activation_status")
        .and_then(JsonValue::as_str);
    let freshness = descriptor
        .get("freshness_status")
        .and_then(JsonValue::as_str);
    let mut diagnostics = Vec::new();

    if activation == Some("activated") && freshness != Some("fresh") {
        diagnostics.push(Diagnostic::new(
            descriptor_type,
            descriptor_id,
            "$.freshness_status",
            "activated_requires_fresh",
            RULE_ID,
        ));
    }
    if activation == Some("failed") && freshness == Some("fresh") {
        diagnostics.push(Diagnostic::new(
            descriptor_type,
            descriptor_id,
            "$.freshness_status",
            "failed_activation_must_not_be_fresh",
            RULE_ID,
        ));
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn accepts_passing_activation_descriptor() {
        let value = parse_json(
            r#"{
              "descriptor_type":"ActivationDescriptor",
              "descriptor_version":"v0.2",
              "descriptor_id":"activation-1",
              "descriptor_hash":"sha256:activation1",
              "artifact_class":"activation_descriptor",
              "schema_version":"v0.2",
              "seed_bundle_hash":"bundle-sha256:seed1",
              "manifest_hash":"sha256:manifest1",
              "snapshot_hash":"sha256:snapshot1",
              "index_hash":"sha256:index1",
              "snapshot_version":"v0.2",
              "snapshot_name":"verified-runtime-selection",
              "record_counts":{"prompt_sample":1},
              "runtime_selection_predicates":["accepted_only"],
              "quarantine_exclusion_predicates":["exclude_quarantine"],
              "candidate_exclusion_predicates":["exclude_candidate"],
              "source_delta_batch_hash":"sha256:sourcebatch1",
              "source_delta_count":0,
              "source_freshness_digest":"sha256:freshness1",
              "activation_status":"activated",
              "freshness_status":"fresh",
              "verification_status":"passed",
              "active_pointer":{"descriptor_id":"activation-1","descriptor_hash":"sha256:activation1"},
              "last_known_good_snapshot":{"descriptor_id":"activation-0","descriptor_hash":"sha256:activation0"},
              "last_known_good_activation_descriptor":{"descriptor_id":"activation-0","descriptor_hash":"sha256:activation0"},
              "rollback_pointer":{"descriptor_id":"rollback-pointer-0","descriptor_hash":"sha256:rollback0"},
              "activation_verified_at_bucket":"2026-04-26T14",
              "controller_approval_id":"approval-1"
            }"#,
        )
        .unwrap();

        assert!(validate_activation_descriptor(&value).is_empty());
    }

    #[test]
    fn rejects_bad_status_hash_and_raw_pointer() {
        let value = parse_json(
            r#"{
              "descriptor_type":"ActivationDescriptor",
              "descriptor_id":"activation-2",
              "descriptor_hash":"snapshot.sqlite3",
              "artifact_class":"activation_descriptor",
              "seed_bundle_hash":"bundle-sha256:seed2",
              "manifest_hash":"manifest2",
              "snapshot_hash":"sha256:snapshot2",
              "index_hash":"sha256:index2",
              "source_delta_batch_hash":"sha256:sourcebatch2",
              "source_freshness_digest":"sha256:freshness2",
              "activation_status":"activated",
              "freshness_status":"stale_snapshot",
              "active_pointer":"E:\\raw\\snapshot.sqlite3"
            }"#,
        )
        .unwrap();

        let diagnostics = validate_activation_descriptor(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "activated_requires_fresh"));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "invalid_hash_shape"));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "raw_path_or_locator_in_activation_pointer"
        }));
        assert!(diagnostics
            .iter()
            .all(|diagnostic| !diagnostic.to_string().contains("snapshot.sqlite3")));
    }

    #[test]
    fn rejects_failed_activation_marked_fresh() {
        let value = parse_json(
            r#"{
              "descriptor_type":"ActivationDescriptor",
              "descriptor_id":"activation-3",
              "descriptor_hash":"sha256:activation3",
              "artifact_class":"activation_descriptor",
              "seed_bundle_hash":"bundle-sha256:seed3",
              "manifest_hash":"sha256:manifest3",
              "snapshot_hash":"sha256:snapshot3",
              "index_hash":"sha256:index3",
              "source_delta_batch_hash":"sha256:sourcebatch3",
              "source_freshness_digest":"sha256:freshness3",
              "activation_status":"failed",
              "freshness_status":"fresh"
            }"#,
        )
        .unwrap();

        let diagnostics = validate_activation_descriptor(&value);
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "failed_activation_must_not_be_fresh"
        }));
    }
}
