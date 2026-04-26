pub mod full_kb_rows;
pub mod leakage_count;
pub mod purge;
pub mod refresh_telemetry;
pub mod rollback;

use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

pub fn validate_first_wave_rules(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    diagnostics.extend(full_kb_rows::validate_full_kb_rows(descriptor));
    diagnostics.extend(leakage_count::validate_leakage_count(descriptor));
    diagnostics.extend(purge::validate_purge_descriptor(descriptor));
    diagnostics.extend(rollback::validate_rollback_descriptor(descriptor));
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
