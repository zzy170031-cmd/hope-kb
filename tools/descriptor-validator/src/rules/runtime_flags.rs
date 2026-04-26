use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

use super::{descriptor_id, descriptor_type};

pub(crate) const RUNTIME_FLAG_FIELDS: &[&str] = &[
    "media_generation_triggered",
    "image_generation_triggered",
    "video_generation_triggered",
    "graphrag_used",
    "hybrid_search_used",
    "rerank_used",
    "runtime_llm_summarize_used",
    "expensive_path_used",
];

const RULE_ID: &str = "runtime_flags_must_be_false";

pub fn validate_runtime_flags(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    let descriptor_id = descriptor_id(descriptor);
    let mut diagnostics = Vec::new();

    scan_runtime_flags(
        descriptor,
        "$",
        &descriptor_type,
        &descriptor_id,
        &mut diagnostics,
    );

    diagnostics
}

fn scan_runtime_flags(
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

                if RUNTIME_FLAG_FIELDS.contains(&key.as_str())
                    && !matches!(child.as_bool(), Some(false))
                {
                    diagnostics.push(Diagnostic::new(
                        descriptor_type,
                        descriptor_id,
                        child_path.clone(),
                        format!("{key}_not_false"),
                        RULE_ID,
                    ));
                }

                scan_runtime_flags(
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
                scan_runtime_flags(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn accepts_false_runtime_flags() {
        let value = parse_json(
            r#"{
              "descriptor_type":"QueryResult",
              "descriptor_id":"q1",
              "media_generation_triggered":false,
              "graphrag_used":false,
              "runtime_llm_summarize_used":false,
              "expensive_path_used":false
            }"#,
        )
        .unwrap();

        assert!(validate_runtime_flags(&value).is_empty());
    }

    #[test]
    fn rejects_true_and_non_bool_runtime_flags() {
        let value = parse_json(
            r#"{
              "descriptor_type":"EvalArtifact",
              "descriptor_id":"eval1",
              "media_generation_triggered":true,
              "nested":{"graphrag_used":"false"}
            }"#,
        )
        .unwrap();

        let diagnostics = validate_runtime_flags(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "media_generation_triggered_not_false"));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "graphrag_used_not_false"));
    }
}
