use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

pub fn validate_leakage_count(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    let descriptor_id = descriptor_id(descriptor);
    let mut diagnostics = Vec::new();

    if descriptor_type == "RefreshTelemetryRecord" {
        match descriptor.get("leakage_count") {
            Some(value) if value.is_number_zero() => {}
            Some(_) => diagnostics.push(Diagnostic::new(
                &descriptor_type,
                &descriptor_id,
                "$.leakage_count",
                "leakage_count_not_zero",
                "leakage_count_guard",
            )),
            None => diagnostics.push(Diagnostic::new(
                &descriptor_type,
                &descriptor_id,
                "$.leakage_count",
                "missing_leakage_count",
                "leakage_count_guard",
            )),
        }
    }

    scan_present_fields(
        descriptor,
        "$",
        &descriptor_type,
        &descriptor_id,
        &mut diagnostics,
    );

    diagnostics
}

fn scan_present_fields(
    value: &JsonValue,
    path: &str,
    descriptor_type: &str,
    descriptor_id: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match value {
        JsonValue::Object(fields) => {
            for (key, child) in fields {
                let child_path = if path == "$" {
                    format!("$.{key}")
                } else {
                    format!("{path}.{key}")
                };
                let top_level_required_field =
                    path == "$" && descriptor_type == "RefreshTelemetryRecord";
                if key == "leakage_count" && !child.is_number_zero() && !top_level_required_field {
                    diagnostics.push(Diagnostic::new(
                        descriptor_type,
                        descriptor_id,
                        child_path.clone(),
                        "leakage_count_not_zero",
                        "leakage_count_guard",
                    ));
                }
                scan_present_fields(
                    child,
                    &child_path,
                    descriptor_type,
                    descriptor_id,
                    diagnostics,
                );
            }
        }
        JsonValue::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                scan_present_fields(
                    child,
                    &format!("{path}[{index}]"),
                    descriptor_type,
                    descriptor_id,
                    diagnostics,
                );
            }
        }
        _ => {}
    }
}

fn descriptor_type(descriptor: &JsonValue) -> String {
    descriptor
        .get("descriptor_type")
        .and_then(JsonValue::as_str)
        .unwrap_or("unknown")
        .to_string()
}

fn descriptor_id(descriptor: &JsonValue) -> String {
    descriptor
        .get("descriptor_id")
        .and_then(JsonValue::as_str)
        .unwrap_or("unknown")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn requires_zero_for_refresh_telemetry() {
        let value = parse_json(
            r#"{"descriptor_type":"RefreshTelemetryRecord","descriptor_id":"rt1","leakage_count":2}"#,
        )
        .unwrap();
        let diagnostics = validate_leakage_count(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "leakage_count_not_zero"));
    }

    #[test]
    fn accepts_zero_for_refresh_telemetry() {
        let value = parse_json(
            r#"{"descriptor_type":"RefreshTelemetryRecord","descriptor_id":"rt1","leakage_count":0}"#,
        )
        .unwrap();
        assert!(validate_leakage_count(&value).is_empty());
    }
}
