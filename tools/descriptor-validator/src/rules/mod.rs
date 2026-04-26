pub mod activation;
pub mod active_pointer;
pub mod auto_switch;
pub mod cross_descriptor;
pub mod eval_artifact;
pub mod full_kb_rows;
pub mod future_qa_candidate;
pub mod last_known_good;
pub mod leakage_count;
pub mod purge;
pub mod query_result;
pub mod refresh_telemetry;
pub mod retrieval_trace;
pub mod rollback;
pub mod rollback_pointer;
pub mod runtime_flags;

use crate::descriptor_set::DescriptorSet;
use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

pub fn validate_first_wave_rules(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    diagnostics.extend(activation::validate_activation_descriptor(descriptor));
    diagnostics.extend(active_pointer::validate_active_pointer(descriptor));
    diagnostics.extend(runtime_flags::validate_runtime_flags(descriptor));
    diagnostics.extend(auto_switch::validate_auto_switch(descriptor));
    diagnostics.extend(eval_artifact::validate_eval_artifact(descriptor));
    diagnostics.extend(query_result::validate_query_result(descriptor));
    diagnostics.extend(retrieval_trace::validate_retrieval_trace(descriptor));
    diagnostics.extend(future_qa_candidate::validate_future_qa_candidate(
        descriptor,
    ));
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

pub fn validate_cross_descriptor_rules(descriptor_set: &DescriptorSet<'_>) -> Vec<Diagnostic> {
    cross_descriptor::validate_cross_descriptor_bindings(descriptor_set)
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
        .or_else(|| descriptor.get("candidate_id"))
        .or_else(|| descriptor.get("eval_artifact_id"))
        .or_else(|| descriptor.get("rollback_pointer_id"))
        .or_else(|| descriptor.get("trace_id"))
        .and_then(JsonValue::as_str)
        .unwrap_or("unknown")
        .to_string()
}

pub(crate) fn string_field<'a>(descriptor: &'a JsonValue, field: &str) -> Option<&'a str> {
    descriptor.get(field).and_then(JsonValue::as_str)
}

pub(crate) fn bool_field(descriptor: &JsonValue, field: &str) -> Option<bool> {
    descriptor.get(field).and_then(JsonValue::as_bool)
}

pub(crate) fn number_field(descriptor: &JsonValue, field: &str) -> Option<f64> {
    let JsonValue::Number(raw) = descriptor.get(field)? else {
        return None;
    };
    raw.parse::<f64>().ok()
}

pub(crate) fn is_empty_array(descriptor: &JsonValue, field: &str) -> Option<bool> {
    let JsonValue::Array(values) = descriptor.get(field)? else {
        return None;
    };
    Some(values.is_empty())
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

pub(crate) fn validate_optional_bool_false(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
    field: &str,
    denied_class: &str,
    rule_id: &str,
) -> Vec<Diagnostic> {
    match descriptor.get(field).and_then(JsonValue::as_bool) {
        None | Some(false) => Vec::new(),
        Some(true) => vec![Diagnostic::new(
            descriptor_type,
            descriptor_id,
            format!("$.{field}"),
            denied_class,
            rule_id,
        )],
    }
}

pub(crate) fn validate_required_bool_false(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
    field: &str,
    denied_class: &str,
    rule_id: &str,
) -> Vec<Diagnostic> {
    match descriptor.get(field).and_then(JsonValue::as_bool) {
        Some(false) => Vec::new(),
        Some(true) | None => vec![Diagnostic::new(
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

pub(crate) fn validate_number_at_least(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
    field: &str,
    minimum: f64,
    denied_class: &str,
    rule_id: &str,
) -> Vec<Diagnostic> {
    match number_field(descriptor, field) {
        Some(value) if value >= minimum => Vec::new(),
        Some(_) | None => vec![Diagnostic::new(
            descriptor_type,
            descriptor_id,
            format!("$.{field}"),
            denied_class,
            rule_id,
        )],
    }
}

pub(crate) fn validate_number_between(
    descriptor: &JsonValue,
    descriptor_type: &str,
    descriptor_id: &str,
    field: &str,
    min: f64,
    max: f64,
    denied_class: &str,
    rule_id: &str,
) -> Vec<Diagnostic> {
    match number_field(descriptor, field) {
        Some(value) if (min..=max).contains(&value) => Vec::new(),
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
