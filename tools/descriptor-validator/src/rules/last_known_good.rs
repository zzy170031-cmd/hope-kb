use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

use super::{
    descriptor_id, descriptor_type, validate_hash_fields, validate_no_raw_locator_fields,
    validate_required_fields, validate_string_equals,
};

const RULE_ID: &str = "last_known_good_static";

const REQUIRED_FIELDS: &[&str] = &[
    "descriptor_type",
    "descriptor_version",
    "descriptor_id",
    "descriptor_hash",
    "artifact_class",
    "schema_version",
    "lkg_descriptor_id",
    "lkg_descriptor_hash",
    "lkg_seed_bundle_hash",
    "lkg_manifest_hash",
    "lkg_snapshot_hash",
    "lkg_index_hash",
    "lkg_verified_at_bucket",
    "lkg_activation_status",
    "lkg_freshness_status",
    "lkg_reason_code",
];

const HASH_FIELDS: &[&str] = &[
    "descriptor_hash",
    "lkg_descriptor_hash",
    "lkg_seed_bundle_hash",
    "lkg_manifest_hash",
    "lkg_snapshot_hash",
    "lkg_index_hash",
];

const LKG_POINTER_FIELDS: &[&str] = &[
    "lkg_descriptor_id",
    "lkg_descriptor_hash",
    "lkg_seed_bundle_hash",
    "lkg_manifest_hash",
    "lkg_snapshot_hash",
    "lkg_index_hash",
];

pub fn validate_last_known_good_descriptor(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    if descriptor_type != "LastKnownGoodDescriptor" {
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
        "lkg_activation_status",
        "activated",
        "lkg_activation_status_not_activated",
        RULE_ID,
    ));
    diagnostics.extend(validate_string_equals(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "lkg_freshness_status",
        "fresh",
        "lkg_freshness_status_not_fresh",
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
        LKG_POINTER_FIELDS,
        "raw_path_or_locator_in_last_known_good",
        RULE_ID,
    ));

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn accepts_passing_last_known_good_descriptor() {
        let value = parse_json(
            r#"{
              "descriptor_type":"LastKnownGoodDescriptor",
              "descriptor_version":"v0.2",
              "descriptor_id":"lkg-1",
              "descriptor_hash":"sha256:lkg1",
              "artifact_class":"activation_descriptor",
              "schema_version":"v0.2",
              "lkg_descriptor_id":"activation-1",
              "lkg_descriptor_hash":"sha256:activation1",
              "lkg_seed_bundle_hash":"bundle-sha256:seed1",
              "lkg_manifest_hash":"sha256:manifest1",
              "lkg_snapshot_hash":"sha256:snapshot1",
              "lkg_index_hash":"sha256:index1",
              "lkg_verified_at_bucket":"2026-04-26T14",
              "lkg_activation_status":"activated",
              "lkg_freshness_status":"fresh",
              "lkg_reason_code":"previous_verified_activation"
            }"#,
        )
        .unwrap();

        assert!(validate_last_known_good_descriptor(&value).is_empty());
    }

    #[test]
    fn rejects_non_fresh_lkg_and_raw_locator() {
        let value = parse_json(
            r#"{
              "descriptor_type":"LastKnownGoodDescriptor",
              "descriptor_id":"lkg-2",
              "descriptor_hash":"sha256:lkg2",
              "artifact_class":"activation_descriptor",
              "schema_version":"v0.2",
              "lkg_descriptor_id":"seed/v0.2/manifest.json",
              "lkg_descriptor_hash":"activation2",
              "lkg_seed_bundle_hash":"bundle-sha256:seed2",
              "lkg_manifest_hash":"sha256:manifest2",
              "lkg_snapshot_hash":"sha256:snapshot2",
              "lkg_index_hash":"sha256:index2",
              "lkg_activation_status":"verified",
              "lkg_freshness_status":"stale_snapshot",
              "lkg_reason_code":"previous_verified_activation"
            }"#,
        )
        .unwrap();

        let diagnostics = validate_last_known_good_descriptor(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "lkg_activation_status_not_activated"));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "lkg_freshness_status_not_fresh"));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "raw_path_or_locator_in_last_known_good"
        }));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "invalid_hash_shape"));
    }
}
