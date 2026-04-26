use crate::descriptor_set::{descriptor_id, descriptor_type, string_field, DescriptorSet};
use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

const RULE_ID: &str = "cross_descriptor_binding";
const DUPLICATE_RULE_ID: &str = "duplicate_descriptor_identity";

pub fn validate_cross_descriptor_bindings(descriptor_set: &DescriptorSet<'_>) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    diagnostics.extend(validate_duplicate_descriptor_identities(descriptor_set));
    for descriptor in descriptor_set.descriptors() {
        match descriptor_type(descriptor) {
            "ActivePointer" => {
                diagnostics.extend(validate_active_pointer_binding(descriptor, descriptor_set));
            }
            "LastKnownGoodDescriptor" => {
                diagnostics.extend(validate_last_known_good_binding(descriptor, descriptor_set));
            }
            "RollbackPointer" => {
                diagnostics.extend(validate_rollback_pointer_binding(
                    descriptor,
                    descriptor_set,
                ));
            }
            _ => {}
        }
    }
    diagnostics
}

fn validate_duplicate_descriptor_identities(descriptor_set: &DescriptorSet<'_>) -> Vec<Diagnostic> {
    descriptor_set
        .duplicate_descriptors()
        .map(|descriptor| {
            Diagnostic::new(
                descriptor_type(descriptor),
                descriptor_id(descriptor),
                "$.descriptor_id",
                "duplicate_descriptor_identity",
                DUPLICATE_RULE_ID,
            )
        })
        .collect()
}

fn validate_active_pointer_binding(
    descriptor: &JsonValue,
    descriptor_set: &DescriptorSet<'_>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let Some(target) = resolve_target(
        descriptor,
        descriptor_set,
        "target_activation_descriptor_id",
        "target_activation_descriptor_hash",
        "active_pointer_target_missing_activation_descriptor",
        "active_pointer_target_not_activation_descriptor",
        Some("ActivationDescriptor"),
        &mut diagnostics,
    ) else {
        return diagnostics;
    };

    require_string_equals(
        descriptor,
        target,
        "target_seed_bundle_hash",
        "seed_bundle_hash",
        "active_pointer_seed_bundle_hash_mismatch",
        &mut diagnostics,
    );
    require_string_equals(
        descriptor,
        target,
        "target_manifest_hash",
        "manifest_hash",
        "active_pointer_manifest_hash_mismatch",
        &mut diagnostics,
    );
    require_string_equals(
        descriptor,
        target,
        "target_snapshot_hash",
        "snapshot_hash",
        "active_pointer_snapshot_hash_mismatch",
        &mut diagnostics,
    );
    require_string_equals(
        descriptor,
        target,
        "target_index_hash",
        "index_hash",
        "active_pointer_index_hash_mismatch",
        &mut diagnostics,
    );

    if string_field(target, "verification_status") != Some("passed") {
        push(
            &mut diagnostics,
            descriptor,
            "$.target_activation_descriptor_id",
            "active_pointer_target_verification_not_passed",
        );
    }

    if !matches!(
        string_field(target, "activation_status"),
        Some("verified" | "activated")
    ) {
        push(
            &mut diagnostics,
            descriptor,
            "$.target_activation_descriptor_id",
            "active_pointer_target_not_verified_or_activated",
        );
    }

    diagnostics
}

fn validate_last_known_good_binding(
    descriptor: &JsonValue,
    descriptor_set: &DescriptorSet<'_>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let Some(target) = resolve_target(
        descriptor,
        descriptor_set,
        "lkg_descriptor_id",
        "lkg_descriptor_hash",
        "lkg_target_missing_activation_descriptor",
        "lkg_target_not_activation_descriptor",
        Some("ActivationDescriptor"),
        &mut diagnostics,
    ) else {
        return diagnostics;
    };

    validate_lkg_activation_target(
        descriptor,
        target,
        "$.lkg_descriptor_id",
        "lkg_target",
        &mut diagnostics,
    );
    require_string_equals(
        descriptor,
        target,
        "lkg_seed_bundle_hash",
        "seed_bundle_hash",
        "lkg_seed_bundle_hash_mismatch",
        &mut diagnostics,
    );
    require_string_equals(
        descriptor,
        target,
        "lkg_manifest_hash",
        "manifest_hash",
        "lkg_manifest_hash_mismatch",
        &mut diagnostics,
    );
    require_string_equals(
        descriptor,
        target,
        "lkg_snapshot_hash",
        "snapshot_hash",
        "lkg_snapshot_hash_mismatch",
        &mut diagnostics,
    );
    require_string_equals(
        descriptor,
        target,
        "lkg_index_hash",
        "index_hash",
        "lkg_index_hash_mismatch",
        &mut diagnostics,
    );

    diagnostics
}

fn validate_rollback_pointer_binding(
    descriptor: &JsonValue,
    descriptor_set: &DescriptorSet<'_>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    if let Some(current_failed) = resolve_target(
        descriptor,
        descriptor_set,
        "current_failed_descriptor_id",
        "current_failed_descriptor_hash",
        "rollback_current_failed_missing_activation_descriptor",
        "rollback_current_failed_not_activation_descriptor",
        Some("ActivationDescriptor"),
        &mut diagnostics,
    ) {
        if string_field(current_failed, "verification_status") != Some("failed") {
            push(
                &mut diagnostics,
                descriptor,
                "$.current_failed_descriptor_id",
                "rollback_current_failed_verification_not_failed",
            );
        }
        if string_field(current_failed, "activation_status") != Some("failed") {
            push(
                &mut diagnostics,
                descriptor,
                "$.current_failed_descriptor_id",
                "rollback_current_failed_activation_status_not_failed",
            );
        }
        if !matches!(
            string_field(current_failed, "freshness_status"),
            Some("activation_failed" | "blocked")
        ) {
            push(
                &mut diagnostics,
                descriptor,
                "$.current_failed_descriptor_id",
                "rollback_current_failed_freshness_not_failed",
            );
        }
    }

    let Some(target_lkg) = resolve_target(
        descriptor,
        descriptor_set,
        "target_lkg_descriptor_id",
        "target_lkg_descriptor_hash",
        "rollback_target_lkg_missing",
        "rollback_target_lkg_invalid_type",
        None,
        &mut diagnostics,
    ) else {
        return diagnostics;
    };

    match descriptor_type(target_lkg) {
        "LastKnownGoodDescriptor" => {
            validate_lkg_record_for_rollback(descriptor, target_lkg, &mut diagnostics);
            if let Some(activation) = resolve_target(
                target_lkg,
                descriptor_set,
                "lkg_descriptor_id",
                "lkg_descriptor_hash",
                "rollback_lkg_activation_target_missing",
                "rollback_lkg_activation_target_not_activation_descriptor",
                Some("ActivationDescriptor"),
                &mut diagnostics,
            ) {
                validate_lkg_activation_target(
                    descriptor,
                    activation,
                    "$.target_lkg_descriptor_id",
                    "rollback_lkg_activation_target",
                    &mut diagnostics,
                );
            }
        }
        "ActivationDescriptor" => {
            validate_lkg_activation_target(
                descriptor,
                target_lkg,
                "$.target_lkg_descriptor_id",
                "rollback_target_activation",
                &mut diagnostics,
            );
        }
        _ => push(
            &mut diagnostics,
            descriptor,
            "$.target_lkg_descriptor_id",
            "rollback_target_lkg_invalid_type",
        ),
    }

    diagnostics
}

fn validate_lkg_record_for_rollback(
    owner: &JsonValue,
    lkg: &JsonValue,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if string_field(lkg, "lkg_activation_status") != Some("activated") {
        push(
            diagnostics,
            owner,
            "$.target_lkg_descriptor_id",
            "rollback_target_lkg_activation_status_not_activated",
        );
    }
    if string_field(lkg, "lkg_freshness_status") != Some("fresh") {
        push(
            diagnostics,
            owner,
            "$.target_lkg_descriptor_id",
            "rollback_target_lkg_freshness_not_fresh",
        );
    }
}

fn validate_lkg_activation_target(
    owner: &JsonValue,
    target: &JsonValue,
    owner_field_path: &str,
    denied_prefix: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if string_field(target, "verification_status") != Some("passed") {
        push(
            diagnostics,
            owner,
            owner_field_path,
            format!("{denied_prefix}_verification_not_passed"),
        );
    }
    if string_field(target, "activation_status") != Some("activated") {
        push(
            diagnostics,
            owner,
            owner_field_path,
            format!("{denied_prefix}_activation_status_not_activated"),
        );
    }
    if string_field(target, "freshness_status") != Some("fresh") {
        push(
            diagnostics,
            owner,
            owner_field_path,
            format!("{denied_prefix}_freshness_not_fresh"),
        );
    }
}

fn require_string_equals(
    owner: &JsonValue,
    target: &JsonValue,
    owner_field: &str,
    target_field: &str,
    denied_class: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(owner_value) = string_field(owner, owner_field) else {
        return;
    };
    let Some(target_value) = string_field(target, target_field) else {
        return;
    };
    if owner_value != target_value {
        push(diagnostics, owner, format!("$.{owner_field}"), denied_class);
    }
}

fn resolve_target<'a>(
    owner: &JsonValue,
    descriptor_set: &'a DescriptorSet<'_>,
    id_field: &str,
    hash_field: &str,
    missing_class: &str,
    wrong_type_class: &str,
    expected_type: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<&'a JsonValue> {
    let Some(target_id) = string_field(owner, id_field) else {
        return None;
    };
    let Some(target_hash) = string_field(owner, hash_field) else {
        return None;
    };

    let Some(target) = descriptor_set.get(target_id, target_hash) else {
        push(diagnostics, owner, format!("$.{id_field}"), missing_class);
        return None;
    };

    if let Some(expected_type) = expected_type {
        if descriptor_type(target) != expected_type {
            push(
                diagnostics,
                owner,
                format!("$.{id_field}"),
                wrong_type_class,
            );
            return None;
        }
    }

    Some(target)
}

fn push(
    diagnostics: &mut Vec<Diagnostic>,
    descriptor: &JsonValue,
    field_path: impl Into<String>,
    denied_class: impl Into<String>,
) {
    diagnostics.push(Diagnostic::new(
        descriptor_type(descriptor),
        descriptor_id(descriptor),
        field_path,
        denied_class,
        RULE_ID,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn accepts_cross_bound_activation_pointer_lkg_and_rollback() {
        let descriptors = parse_set(&[
            passing_activation(),
            passing_active_pointer(),
            passing_lkg(),
            failed_activation(),
            passing_rollback_pointer(),
        ]);
        let set = DescriptorSet::new(&descriptors);

        assert!(validate_cross_descriptor_bindings(&set).is_empty());
    }

    #[test]
    fn rejects_active_pointer_hash_mismatch() {
        let descriptors = parse_set(&[
            passing_activation(),
            r#"{
              "descriptor_type":"ActivePointer",
              "descriptor_id":"active-pointer-1",
              "descriptor_hash":"sha256:pointer1",
              "target_activation_descriptor_id":"activation-1",
              "target_activation_descriptor_hash":"sha256:activation1",
              "target_seed_bundle_hash":"bundle-sha256:seed1",
              "target_manifest_hash":"sha256:other",
              "target_snapshot_hash":"sha256:snapshot1",
              "target_index_hash":"sha256:index1"
            }"#,
        ]);
        let set = DescriptorSet::new(&descriptors);
        let diagnostics = validate_cross_descriptor_bindings(&set);

        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "active_pointer_manifest_hash_mismatch"
        }));
    }

    #[test]
    fn rejects_lkg_target_not_activated_fresh() {
        let descriptors = parse_set(&[
            failed_activation(),
            r#"{
              "descriptor_type":"LastKnownGoodDescriptor",
              "descriptor_id":"lkg-1",
              "descriptor_hash":"sha256:lkg1",
              "lkg_descriptor_id":"activation-failed-1",
              "lkg_descriptor_hash":"sha256:activationfailed1",
              "lkg_seed_bundle_hash":"bundle-sha256:seed1",
              "lkg_manifest_hash":"sha256:manifest1",
              "lkg_snapshot_hash":"sha256:snapshot1",
              "lkg_index_hash":"sha256:index1",
              "lkg_activation_status":"activated",
              "lkg_freshness_status":"fresh"
            }"#,
        ]);
        let set = DescriptorSet::new(&descriptors);
        let diagnostics = validate_cross_descriptor_bindings(&set);

        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "lkg_target_verification_not_passed"));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "lkg_target_activation_status_not_activated"
        }));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "lkg_target_freshness_not_fresh"));
    }

    #[test]
    fn rejects_rollback_pointer_current_candidate_not_failed() {
        let descriptors = parse_set(&[
            passing_activation(),
            passing_lkg(),
            r#"{
              "descriptor_type":"RollbackPointer",
              "descriptor_id":"rollback-pointer-1",
              "descriptor_hash":"sha256:rollbackpointer1",
              "current_failed_descriptor_id":"activation-1",
              "current_failed_descriptor_hash":"sha256:activation1",
              "target_lkg_descriptor_id":"lkg-1",
              "target_lkg_descriptor_hash":"sha256:lkg1"
            }"#,
        ]);
        let set = DescriptorSet::new(&descriptors);
        let diagnostics = validate_cross_descriptor_bindings(&set);

        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "rollback_current_failed_verification_not_failed"
        }));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "rollback_current_failed_activation_status_not_failed"
        }));
    }

    #[test]
    fn rejects_duplicate_descriptor_identity_without_hash_leak() {
        let descriptors = parse_set(&[
            passing_activation(),
            r#"{
              "descriptor_type":"ActivePointer",
              "descriptor_id":"activation-1",
              "descriptor_hash":"sha256:activation1"
            }"#,
        ]);
        let set = DescriptorSet::new(&descriptors);
        let diagnostics = validate_cross_descriptor_bindings(&set);

        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "duplicate_descriptor_identity"
                && diagnostic.rule_id == "duplicate_descriptor_identity"
        }));
        assert!(diagnostics
            .iter()
            .all(|diagnostic| !diagnostic.to_string().contains("sha256:activation1")));
    }

    fn parse_set(raw_descriptors: &[&str]) -> Vec<JsonValue> {
        raw_descriptors
            .iter()
            .map(|raw| parse_json(raw).unwrap())
            .collect()
    }

    fn passing_activation() -> &'static str {
        r#"{
          "descriptor_type":"ActivationDescriptor",
          "descriptor_id":"activation-1",
          "descriptor_hash":"sha256:activation1",
          "seed_bundle_hash":"bundle-sha256:seed1",
          "manifest_hash":"sha256:manifest1",
          "snapshot_hash":"sha256:snapshot1",
          "index_hash":"sha256:index1",
          "verification_status":"passed",
          "activation_status":"activated",
          "freshness_status":"fresh"
        }"#
    }

    fn failed_activation() -> &'static str {
        r#"{
          "descriptor_type":"ActivationDescriptor",
          "descriptor_id":"activation-failed-1",
          "descriptor_hash":"sha256:activationfailed1",
          "seed_bundle_hash":"bundle-sha256:seed1",
          "manifest_hash":"sha256:manifest1",
          "snapshot_hash":"sha256:snapshot1",
          "index_hash":"sha256:index1",
          "verification_status":"failed",
          "activation_status":"failed",
          "freshness_status":"activation_failed"
        }"#
    }

    fn passing_active_pointer() -> &'static str {
        r#"{
          "descriptor_type":"ActivePointer",
          "descriptor_id":"active-pointer-1",
          "descriptor_hash":"sha256:pointer1",
          "target_activation_descriptor_id":"activation-1",
          "target_activation_descriptor_hash":"sha256:activation1",
          "target_seed_bundle_hash":"bundle-sha256:seed1",
          "target_manifest_hash":"sha256:manifest1",
          "target_snapshot_hash":"sha256:snapshot1",
          "target_index_hash":"sha256:index1"
        }"#
    }

    fn passing_lkg() -> &'static str {
        r#"{
          "descriptor_type":"LastKnownGoodDescriptor",
          "descriptor_id":"lkg-1",
          "descriptor_hash":"sha256:lkg1",
          "lkg_descriptor_id":"activation-1",
          "lkg_descriptor_hash":"sha256:activation1",
          "lkg_seed_bundle_hash":"bundle-sha256:seed1",
          "lkg_manifest_hash":"sha256:manifest1",
          "lkg_snapshot_hash":"sha256:snapshot1",
          "lkg_index_hash":"sha256:index1",
          "lkg_activation_status":"activated",
          "lkg_freshness_status":"fresh"
        }"#
    }

    fn passing_rollback_pointer() -> &'static str {
        r#"{
          "descriptor_type":"RollbackPointer",
          "descriptor_id":"rollback-pointer-1",
          "descriptor_hash":"sha256:rollbackpointer1",
          "current_failed_descriptor_id":"activation-failed-1",
          "current_failed_descriptor_hash":"sha256:activationfailed1",
          "target_lkg_descriptor_id":"lkg-1",
          "target_lkg_descriptor_hash":"sha256:lkg1"
        }"#
    }
}
