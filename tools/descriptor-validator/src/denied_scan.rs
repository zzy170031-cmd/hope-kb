use crate::diagnostic::Diagnostic;
use crate::json_walk::{is_json_like_string, parse_json, JsonValue};

pub fn scan_denied_fields(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type = descriptor_type(descriptor);
    let descriptor_id = descriptor_id(descriptor);
    let mut diagnostics = Vec::new();
    walk_value(
        descriptor,
        "$",
        &descriptor_type,
        &descriptor_id,
        &mut diagnostics,
    );
    diagnostics
}

fn walk_value(
    value: &JsonValue,
    path: &str,
    descriptor_type: &str,
    descriptor_id: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match value {
        JsonValue::Object(fields) => {
            for (key, child) in fields {
                let child_path = format_object_path(path, key);
                if let Some(denied_class) = denied_class_for(key, &child_path) {
                    diagnostics.push(Diagnostic::new(
                        descriptor_type,
                        descriptor_id,
                        child_path.clone(),
                        denied_class,
                        "denied_field_scan",
                    ));
                }
                walk_value(
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
                let child_path = format!("{path}[{index}]");
                walk_value(
                    child,
                    &child_path,
                    descriptor_type,
                    descriptor_id,
                    diagnostics,
                );
            }
        }
        JsonValue::String(raw) if is_json_like_string(raw) => {
            if let Ok(parsed) = parse_json(raw) {
                let child_path = format!("{path}<json>");
                walk_value(
                    &parsed,
                    &child_path,
                    descriptor_type,
                    descriptor_id,
                    diagnostics,
                );
            }
        }
        _ => {}
    }
}

fn denied_class_for(key: &str, path: &str) -> Option<&'static str> {
    let normalized = normalize_key(key);
    let path_pattern = normalize_path_pattern(path);

    if path_pattern.ends_with("source_register[].path") {
        return Some("source_register_path");
    }
    if path_pattern.ends_with("sources[].path") {
        return Some("source_path");
    }

    match normalized.as_str() {
        "raw_kb_rows" | "raw_rows" => Some("raw_kb_rows"),
        "raw_prompt_body" | "prompt_body" | "full_prompt" => Some("raw_prompt_body"),
        "story_input" => Some("model_visible_story_input"),
        "accepted_ephemeral_context" => Some("accepted_ephemeral_context"),
        "selected_sample_excerpts" => Some("selected_sample_excerpts"),
        "source_original_text" | "raw_source_text" => Some("raw_source_text"),
        "source_register" | "full_source_register" => Some("full_source_register"),
        "provenance_locator" => Some("provenance_locator"),
        "path" | "url_or_path" | "local_path" | "absolute_path" => Some("local_or_source_path"),
        "overlay_json" => Some("overlay_json"),
        "raw_graph" | "graph_neighborhood" => Some("raw_graph"),
        "provider_config" | "provider_headers" => Some("provider_config"),
        "request_body" | "response_body" => Some("provider_or_request_body"),
        "api_key" | "api_key_ref" => Some("api_key_or_ref"),
        "secret" | "secret_ref" => Some("secret_or_ref"),
        "credential" | "credential_ref" => Some("credential_or_ref"),
        "token" => Some("token"),
        "env_var_name" => Some("env_var_name"),
        "matched_value" | "matched_value_snippet" => Some("matched_value_snippet"),
        _ => None,
    }
}

fn normalize_key(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .replace(' ', "_")
        .replace('-', "_")
}

fn normalize_path_pattern(path: &str) -> String {
    let mut normalized = String::new();
    let mut chars = path.trim_start_matches("$.").chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '[' {
            normalized.push_str("[]");
            while let Some(next) = chars.peek() {
                if *next == ']' {
                    chars.next();
                    break;
                }
                chars.next();
            }
        } else {
            normalized.push(ch.to_ascii_lowercase());
        }
    }
    normalized
}

fn format_object_path(parent: &str, key: &str) -> String {
    if parent == "$" {
        format!("$.{key}")
    } else {
        format!("{parent}.{key}")
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
        .or_else(|| descriptor.get("candidate_id"))
        .or_else(|| descriptor.get("trace_id"))
        .or_else(|| descriptor.get("eval_artifact_id"))
        .and_then(JsonValue::as_str)
        .unwrap_or("unknown")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn detects_nested_denied_field() {
        let value = parse_json(
            r#"{"descriptor_type":"QueryResult","descriptor_id":"q1","metadata":{"debug":{"prompt_body":"hidden"}}}"#,
        )
        .unwrap();
        let diagnostics = scan_denied_fields(&value);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].field_path, "$.metadata.debug.prompt_body");
        assert_eq!(diagnostics[0].denied_class, "raw_prompt_body");
    }

    #[test]
    fn parses_serialized_json_without_emitting_value() {
        let value = parse_json(
            r#"{"descriptor_type":"RetrievalTrace","descriptor_id":"t1","payload":"{\"source_register\":{\"path\":\"x\"}}"}"#,
        )
        .unwrap();
        let diagnostics = scan_denied_fields(&value);
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.field_path == "$.payload<json>.source_register"
                && diagnostic.denied_class == "full_source_register"
        }));
        assert!(diagnostics
            .iter()
            .all(|diagnostic| !diagnostic.to_string().contains('x')));
    }
}
