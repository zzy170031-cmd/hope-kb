pub mod activation;
pub mod active_pointer;
pub mod full_kb_rows;
pub mod last_known_good;
pub mod leakage_count;
pub mod purge;
pub mod refresh_telemetry;
pub mod rollback;
pub mod rollback_pointer;

use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

pub fn validate_first_wave_rules(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    diagnostics.extend(activation::validate_activation_descriptor(descriptor));
    diagnostics.extend(active_pointer::validate_active_pointer(descriptor));
    diagnostics.extend(full_kb_rows::validate_full_kb_rows(descriptor));
    diagnostics.extend(last_known_good::validate_last_known_good_descriptor(
        descriptor,
    ));
    diagnostics.extend(leakage_count::validate_leakage_count(descriptor));
    diagnostics.extend(purge::validate_purge_descriptor(descriptor));
    diagnostics.extend(rollback::validate_rollback_descriptor(descriptor));
    diagnostics.extend(rollback_pointer::validate_rollback_pointer(descriptor));
    diagnostics.extend(refresh_telemetry::validate_refresh_telemetry_record(
        descriptor,
    ));
    diagnostics
}

pub(crate) fn descriptor_type(descriptor: &JsonValue) -> String {
    descriptor
        .get("descriptor_type")
        .and_then(JsonValue::as_str)
        .unwrap_or("unknown")
        .to_string()
}

pub(crate) fn descriptor_id(descriptor: &JsonValue) -> String {
    descriptor
        .get("descriptor_id")
        .or_else(|| descriptor.get("rollback_pointer_id"))
        .or_else(|| descriptor.get("trace_id"))
        .and_then(JsonValue::as_str)
        .unwrap_or("unknown")
        .to_string()
}

pub(crate) fn validate_required_fields(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
    required_fields: &[&str],
    rule_id: &str,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for field in required_fields {
        if descriptor.get(field).is_none() {
            diagnostics.push(Diagnostic::new(
                descriptor_type,
                descriptor_id,
                format!("$.{field}"),
                "missing_required_field",
                rule_id,
            ));
        }
    }
    diagnostics
}

pub(crate) fn validate_bool_true(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
    field: &str,
    denied_class: &str,
    rule_id: &str,
) -> Vec<Diagnostic> {
    match descriptor.get(field).and_then(JsonValue::as_bool) {
        Some(true) => Vec::new(),
        Some(false) | None => vec![Diagnostic::new(
            descriptor_type,
            descriptor_id,
            format!("$.{field}"),
            denied_class,
            rule_id,
        )],
    }
}

pub(crate) fn validate_zero(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
    field: &str,
    denied_class: &str,
    rule_id: &str,
) -> Vec<Diagnostic> {
    match descriptor.get(field) {
        Some(value) if value.is_number_zero() => Vec::new(),
        Some(_) | None => vec![Diagnostic::new(
            descriptor_type,
            descriptor_id,
            format!("$.{field}"),
            denied_class,
            rule_id,
        )],
    }
}

pub(crate) fn validate_string_equals(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
    field: &str,
    expected: &str,
    denied_class: &str,
    rule_id: &str,
) -> Vec<Diagnostic> {
    match descriptor.get(field).and_then(JsonValue::as_str) {
        Some(value) if value == expected => Vec::new(),
        Some(_) | None => vec![Diagnostic::new(
            descriptor_type,
            descriptor_id,
            format!("$.{field}"),
            denied_class,
            rule_id,
        )],
    }
}

pub(crate) fn validate_string_in(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
    field: &str,
    allowed: &[&str],
    denied_class: &str,
    rule_id: &str,
) -> Vec<Diagnostic> {
    match descriptor.get(field).and_then(JsonValue::as_str) {
        Some(value) if allowed.contains(&value) => Vec::new(),
        Some(_) | None => vec![Diagnostic::new(
            descriptor_type,
            descriptor_id,
            format!("$.{field}"),
            denied_class,
            rule_id,
        )],
    }
}

pub(crate) fn validate_hash_fields(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
    fields: &[&str],
    rule_id: &str,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for field in fields {
        let Some(value) = descriptor.get(field).and_then(JsonValue::as_str) else {
            continue;
        };
        if !is_descriptor_hash_like(value) {
            diagnostics.push(Diagnostic::new(
                descriptor_type,
                descriptor_id,
                format!("$.{field}"),
                "invalid_hash_shape",
                rule_id,
            ));
        }
    }
    diagnostics
}

pub(crate) fn validate_no_raw_locator_fields(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
    fields: &[&str],
    denied_class: &str,
    rule_id: &str,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for field in fields {
        if let Some(value) = descriptor.get(field) {
            collect_raw_locator_diagnostics(
                value,
                &format!("$.{field}"),
                descriptor_type,
                descriptor_id,
                denied_class,
                rule_id,
                &mut diagnostics,
            );
        }
    }
    diagnostics
}

pub(crate) fn string_array<'a>(descriptor: &'a JsonValue, field: &str) -> Option<Vec<&'a str>> {
    let JsonValue::Array(values) = descriptor.get(field)? else {
        return None;
    };

    let mut strings = Vec::with_capacity(values.len());
    for value in values {
        strings.push(value.as_str()?);
    }
    Some(strings)
}

pub(crate) fn looks_like_raw_path_or_locator(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    value.contains(":\\")
        || value.contains(":/")
        || value.starts_with("\\\\")
        || value.starts_with('/')
        || lower.contains(".sqlite")
        || lower.contains("source_register")
        || lower.contains("seed/v")
        || lower.contains("seed\\v")
}

fn is_descriptor_hash_like(value: &str) -> bool {
    let Some(digest) = value
        .strip_prefix("sha256:")
        .or_else(|| value.strip_prefix("bundle-sha256:"))
    else {
        return false;
    };
    !digest.is_empty()
        && digest
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
        && !looks_like_raw_path_or_locator(digest)
}

fn collect_raw_locator_diagnostics(
    value: &JsonValue,
    field_path: &str,
    descriptor_type: &str,
    descriptor_id: &str,
    denied_class: &str,
    rule_id: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match value {
        JsonValue::String(raw) if looks_like_raw_path_or_locator(raw) => {
            diagnostics.push(Diagnostic::new(
                descriptor_type,
                descriptor_id,
                field_path,
                denied_class,
                rule_id,
            ));
        }
        JsonValue::Array(values) => {
            for (index, item) in values.iter().enumerate() {
                collect_raw_locator_diagnostics(
                    item,
                    &format!("{field_path}[{index}]"),
                    descriptor_type,
                    descriptor_id,
                    denied_class,
                    rule_id,
                    diagnostics,
                );
            }
        }
        JsonValue::Object(fields) => {
            for (key, item) in fields {
                collect_raw_locator_diagnostics(
                    item,
                    &format!("{field_path}.{key}"),
                    descriptor_type,
                    descriptor_id,
                    denied_class,
                    rule_id,
                    diagnostics,
                );
            }
        }
        _ => {}
    }
}
