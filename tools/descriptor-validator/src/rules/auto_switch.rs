use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

use super::{
    bool_field, descriptor_id, descriptor_type, runtime_flags::RUNTIME_FLAG_FIELDS, string_field,
};

const RULE_ID: &str = "auto_switch_stale_blocking";

const STALE_OR_BLOCKED_FRESHNESS: &[&str] = &[
    "stale_source",
    "stale_index",
    "stale_eval",
    "stale_snapshot",
    "activation_failed",
    "unknown",
    "blocked",
];

pub fn validate_auto_switch(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let Some(auto_switch_allowed) = descriptor.get("auto_switch_allowed") else {
        return Vec::new();
    };

    let descriptor_type = descriptor_type(descriptor);
    let descriptor_id = descriptor_id(descriptor);
    let mut diagnostics = Vec::new();

    let Some(auto_switch_allowed) = auto_switch_allowed.as_bool() else {
        diagnostics.push(Diagnostic::new(
            &descriptor_type,
            &descriptor_id,
            "$.auto_switch_allowed",
            "auto_switch_allowed_not_bool",
            RULE_ID,
        ));
        return diagnostics;
    };

    let freshness_status = string_field(descriptor, "freshness_status");
    if auto_switch_allowed {
        if freshness_status != Some("fresh") {
            diagnostics.push(Diagnostic::new(
                &descriptor_type,
                &descriptor_id,
                "$.auto_switch_allowed",
                "auto_switch_requires_fresh_status",
                RULE_ID,
            ));
        }

        if bool_field(descriptor, "all_gates_pass") != Some(true) {
            diagnostics.push(Diagnostic::new(
                &descriptor_type,
                &descriptor_id,
                "$.all_gates_pass",
                "auto_switch_requires_all_gates_pass",
                RULE_ID,
            ));
        }

        if string_field(descriptor, "controller_approval_id")
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .is_none()
        {
            diagnostics.push(Diagnostic::new(
                &descriptor_type,
                &descriptor_id,
                "$.controller_approval_id",
                "auto_switch_requires_controller_approval_id",
                RULE_ID,
            ));
        }

        for flag in RUNTIME_FLAG_FIELDS {
            match bool_field(descriptor, flag) {
                Some(false) => {}
                Some(true) => diagnostics.push(Diagnostic::new(
                    &descriptor_type,
                    &descriptor_id,
                    format!("$.{flag}"),
                    "auto_switch_runtime_flag_not_false",
                    RULE_ID,
                )),
                None => diagnostics.push(Diagnostic::new(
                    &descriptor_type,
                    &descriptor_id,
                    format!("$.{flag}"),
                    "auto_switch_missing_runtime_flag",
                    RULE_ID,
                )),
            }
        }
    } else if freshness_status
        .map(|value| STALE_OR_BLOCKED_FRESHNESS.contains(&value))
        .unwrap_or(false)
    {
        // Stale or blocked descriptors are correctly pinned to false.
    }

    if auto_switch_allowed
        && freshness_status
            .map(|value| STALE_OR_BLOCKED_FRESHNESS.contains(&value))
            .unwrap_or(false)
    {
        diagnostics.push(Diagnostic::new(
            &descriptor_type,
            &descriptor_id,
            "$.auto_switch_allowed",
            "auto_switch_not_false_when_stale",
            RULE_ID,
        ));
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn accepts_fresh_approved_auto_switch() {
        let value = parse_json(
            r#"{
              "descriptor_type":"EvalArtifact",
              "descriptor_id":"eval1",
              "freshness_status":"fresh",
              "auto_switch_allowed":true,
              "all_gates_pass":true,
              "controller_approval_id":"ctrl-1",
              "media_generation_triggered":false,
              "image_generation_triggered":false,
              "video_generation_triggered":false,
              "graphrag_used":false,
              "hybrid_search_used":false,
              "rerank_used":false,
              "runtime_llm_summarize_used":false,
              "expensive_path_used":false
            }"#,
        )
        .unwrap();

        assert!(validate_auto_switch(&value).is_empty());
    }

    #[test]
    fn rejects_stale_auto_switch_true() {
        let value = parse_json(
            r#"{
              "descriptor_type":"EvalArtifact",
              "descriptor_id":"eval2",
              "freshness_status":"stale_eval",
              "auto_switch_allowed":true,
              "all_gates_pass":true,
              "controller_approval_id":"ctrl-1",
              "media_generation_triggered":false,
              "image_generation_triggered":false,
              "video_generation_triggered":false,
              "graphrag_used":false,
              "hybrid_search_used":false,
              "rerank_used":false,
              "runtime_llm_summarize_used":false,
              "expensive_path_used":false
            }"#,
        )
        .unwrap();

        let diagnostics = validate_auto_switch(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "auto_switch_not_false_when_stale"));
    }

    #[test]
    fn rejects_auto_switch_without_approval_and_flags() {
        let value = parse_json(
            r#"{
              "descriptor_type":"EvalArtifact",
              "descriptor_id":"eval3",
              "freshness_status":"fresh",
              "auto_switch_allowed":true,
              "all_gates_pass":false,
              "controller_approval_id":"",
              "media_generation_triggered":false
            }"#,
        )
        .unwrap();

        let diagnostics = validate_auto_switch(&value);
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "auto_switch_requires_all_gates_pass"));
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.denied_class == "auto_switch_requires_controller_approval_id"
        }));
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.denied_class == "auto_switch_missing_runtime_flag"));
    }
}
