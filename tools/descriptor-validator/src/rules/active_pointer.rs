use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

use super::{
    descriptor_id, descriptor_type, validate_hash_fields, validate_no_raw_locator_fields,
    validate_required_fields, validate_string_equals,
};

const RULE_ID: &str = "active_pointer_static";

const REQUIRED_FIELDS: &[&str] = &[
    "descriptor_type",
    "descriptor_version",
    "descriptor_id",
    "descriptor_hash",
    "artifact_class",
    "schema_version",
    "pointer_type",
    "pointer_id",
    "target_activation_descriptor_id",
    "target_activation_descriptor_hash",
    "target_seed_bundle_hash",
    "target_manifest_hash",
    "target_snapshot_hash",
    "target_index_hash",
    "pointer_status",
    "atomic_switch_txn_id",
    "previous_pointer_id",
    "switched_at_bucket",
    "controller_approval_id",
];

const HASH_FIELDS: &[&str] = &[
    "descriptor_hash",
    "target_activation_descriptor_hash",
    "target_seed_bundle_hash",
    "target_manifest_hash",
    "target_snapshot_hash",
    "target_index_hash",
];

const TARGET_FIELDS: &[&str] = &[
    "target_activation_descriptor_id",
    "target_activation_descriptor_hash",
    "target_seed_bundle_hash",
    "target_manifest_hash",
    "target_snapshot_hash",
    "target_index_hash",
];

pub fn validate_active_pointer(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    if descriptor_type != "ActivePointer" {
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
        "pointer_type",
        "active_pointer",
        "pointer_type_not_active_pointer",
        RULE_ID,
    ));
    diagnostics.extend(validate_string_equals(
        descriptor,
        &descriptor_type,
        &descriptor_id,
        "pointer_status",
        "active",
        "pointer_status_not_active",
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
        TARGET_FIELDS,
        "raw_path_or_locator_in_active_pointer_target",
        RULE_ID,
    ));

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn accepts_passing_active_pointer() {
        let value = parse_json(
            r#"{
              "descriptor_type":"ActivePointer",
              "descriptor_version":"v0.2",
              "descriptor_id":"active-pointer-1",
              "descriptor_hash":"sha256:pointer1",
              "artifact_class":"activation_descriptor",
              "schema_version":"v0.2",
              "pointer_type":"active_pointer",
              "pointer_id":"active-pointer-1",
              "target_activation_descriptor_id":"activation-1",
              "target_activation_descriptor_hash":"sha256:activation1",
              "target_seed_bundle_hash":"bundle-sha256:seed1",
              "target_manifest_hash":"sha256:manifest1",
              "target_snapshot_hash":"sha256:snapshot1",
              "target_index_hash":"sha256:index1",
              "pointer_status":"active",
              "atomic_switch_txn_id":"txn-1",
              "previous_pointer_id":"active-pointer-0",
              "switched_at_bucket":"2026-04-26T14",
              "controller_approval_id":"approval-1"
            }"#,
        )
        .unwrap();

        assert!(validate_active_pointer(&value).is_empty());
    }

    #[test]
    fn rejects_bad_pointer_constants_and_raw_target() {
        let value = parse_json(
            r#"{
              "descriptor_type":"ActivePointer",
              "descriptor_id":"active-pointer-2",
              "descriptor_hash":"sha256:pointer2",
              "artifact_class":"activation_descriptor",
              "schema_version":"v0.2",
              "pointer_type":"snapshot_path",
              "pointer_id":"active-pointer-2",
              "target_activation_descriptor_id":"E:\\raw\\snapshot.sqlite3",
              "target_activation_descriptor_hash":"activation2",
              "target_seed_bundle_hash":"bundle-sha256:seed2",
              "target_manifest_hash":"sha256:manifest2",
              "target_snapshot_hash":"sha256:snapshot2",
              "target_index_hash":"sha256:index2",
              "pointer_status":"pending"
            }"#,
        )
        .unwrap();

        let diagnostics = validate_active_pointer(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "pointer_type_not_active_pointer"));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "pointer_status_not_active"));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "raw_path_or_locator_in_active_pointer_target"
        }));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "invalid_hash_shape"));
    }
}
