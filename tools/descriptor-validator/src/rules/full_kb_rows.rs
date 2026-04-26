use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

pub fn validate_full_kb_rows(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    let descriptor_id = descriptor_id(descriptor);
    let mut diagnostics = Vec::new();

    if matches!(descriptor_type.as_str(), "QueryResult" | "RetrievalTrace") {
        match descriptor.get("full_kb_rows_included") {
            Some(value) if value.is_number_zero() => {}
            Some(_) => diagnostics.push(Diagnostic::new(
                &descriptor_type,
                &descriptor_id,
                "$.full_kb_rows_included",
                "full_kb_rows_included_not_zero",
                "full_kb_rows_guard",
            )),
            None => diagnostics.push(Diagnostic::new(
                &descriptor_type,
                &descriptor_id,
                "$.full_kb_rows_included",
                "missing_full_kb_rows_included",
                "full_kb_rows_guard",
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
                    path == "$" && matches!(descriptor_type, "QueryResult" | "RetrievalTrace");
                if key == "full_kb_rows_included"
                    && !child.is_number_zero()
                    && !top_level_required_field
                {
                    diagnostics.push(Diagnostic::new(
                        descriptor_type,
                        descriptor_id,
                        child_path.clone(),
                        "full_kb_rows_included_not_zero",
                        "full_kb_rows_guard",
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
        .or_else(|| descriptor.get("trace_id"))
        .and_then(JsonValue::as_str)
        .unwrap_or("unknown")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn requires_zero_for_query_result() {
        let value = parse_json(
            r#"{"descriptor_type":"QueryResult","descriptor_id":"q1","full_kb_rows_included":1}"#,
        )
        .unwrap();
        let diagnostics = validate_full_kb_rows(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.denied_class == "full_kb_rows_included_not_zero" }));
    }

    #[test]
    fn accepts_zero_for_retrieval_trace() {
        let value = parse_json(
            r#"{"descriptor_type":"RetrievalTrace","trace_id":"t1","full_kb_rows_included":0}"#,
        )
        .unwrap();
        assert!(validate_full_kb_rows(&value).is_empty());
    }
}
