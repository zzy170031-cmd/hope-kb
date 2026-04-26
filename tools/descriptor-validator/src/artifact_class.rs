use std::str::FromStr;

use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;
use crate::model::{ArtifactClass, DescriptorType};

pub fn validate_artifact_class(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let descriptor_type_raw = string_field(descriptor, "descriptor_type").unwrap_or("unknown");
    let descriptor_id = descriptor_id(descriptor);
    let artifact_class_raw = string_field(descriptor, "artifact_class");

    let mut diagnostics = Vec::new();

    let descriptor_type = match DescriptorType::from_str(descriptor_type_raw) {
        Ok(value) => value,
        Err(()) => {
            diagnostics.push(Diagnostic::new(
                descriptor_type_raw,
                descriptor_id.clone(),
                "$.descriptor_type",
                "unknown_descriptor_type",
                "artifact_class_allowlist",
            ));
            return diagnostics;
        }
    };

    let Some(artifact_class_raw) = artifact_class_raw else {
        diagnostics.push(Diagnostic::new(
            descriptor_type.as_str(),
            descriptor_id,
            "$.artifact_class",
            "missing_artifact_class",
            "artifact_class_allowlist",
        ));
        return diagnostics;
    };

    let artifact_class = match ArtifactClass::from_str(artifact_class_raw) {
        Ok(value) => value,
        Err(()) => {
            diagnostics.push(Diagnostic::new(
                descriptor_type.as_str(),
                descriptor_id,
                "$.artifact_class",
                "unknown_artifact_class",
                "artifact_class_allowlist",
            ));
            return diagnostics;
        }
    };

    let expected = expected_artifact_class(descriptor_type);
    if artifact_class != expected {
        diagnostics.push(Diagnostic::new(
            descriptor_type.as_str(),
            descriptor_id,
            "$.artifact_class",
            format!("artifact_class_mismatch_expected_{}", expected.as_str()),
            "artifact_class_allowlist",
        ));
    }

    diagnostics
}

fn expected_artifact_class(descriptor_type: DescriptorType) -> ArtifactClass {
    match descriptor_type {
        DescriptorType::SourceDeltaBatch => ArtifactClass::GovernanceDescriptor,
        DescriptorType::ActivationDescriptor
        | DescriptorType::ActivePointer
        | DescriptorType::LastKnownGoodDescriptor => ArtifactClass::ActivationDescriptor,
        DescriptorType::QueryResult => ArtifactClass::PromptPayload,
        DescriptorType::RetrievalTrace
        | DescriptorType::RollbackPointer
        | DescriptorType::RefreshTelemetryRecord
        | DescriptorType::PurgeDescriptor
        | DescriptorType::RollbackDescriptor => {
            ArtifactClass::RetrievalTraceLogTelemetryShadowRollback
        }
        DescriptorType::FutureQACandidate => ArtifactClass::FutureQaDescriptor,
        DescriptorType::EvalArtifact => ArtifactClass::EvalDescriptor,
    }
}

fn string_field<'a>(descriptor: &'a JsonValue, field: &str) -> Option<&'a str> {
    descriptor.get(field).and_then(JsonValue::as_str)
}

fn descriptor_id(descriptor: &JsonValue) -> String {
    string_field(descriptor, "descriptor_id")
        .or_else(|| string_field(descriptor, "candidate_id"))
        .or_else(|| string_field(descriptor, "trace_id"))
        .or_else(|| string_field(descriptor, "eval_artifact_id"))
        .unwrap_or("unknown")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn accepts_expected_artifact_class() {
        let value = parse_json(
            r#"{"descriptor_type":"QueryResult","descriptor_id":"q1","artifact_class":"prompt_payload"}"#,
        )
        .unwrap();
        assert!(validate_artifact_class(&value).is_empty());
    }

    #[test]
    fn rejects_mismatched_artifact_class() {
        let value = parse_json(
            r#"{"descriptor_type":"QueryResult","descriptor_id":"q1","artifact_class":"activation_descriptor"}"#,
        )
        .unwrap();
        let diagnostics = validate_artifact_class(&value);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].field_path, "$.artifact_class");
    }
}
