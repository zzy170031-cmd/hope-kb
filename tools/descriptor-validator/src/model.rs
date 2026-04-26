use std::fmt;
use std::str::FromStr;

use crate::diagnostic::Diagnostic;
use crate::error::ValidationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptorType {
    SourceDeltaBatch,
    ActivationDescriptor,
    ActivePointer,
    LastKnownGoodDescriptor,
    RollbackPointer,
    QueryResult,
    RetrievalTrace,
    FutureQACandidate,
    EvalArtifact,
    RefreshTelemetryRecord,
    PurgeDescriptor,
    RollbackDescriptor,
}

impl DescriptorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SourceDeltaBatch => "SourceDeltaBatch",
            Self::ActivationDescriptor => "ActivationDescriptor",
            Self::ActivePointer => "ActivePointer",
            Self::LastKnownGoodDescriptor => "LastKnownGoodDescriptor",
            Self::RollbackPointer => "RollbackPointer",
            Self::QueryResult => "QueryResult",
            Self::RetrievalTrace => "RetrievalTrace",
            Self::FutureQACandidate => "FutureQACandidate",
            Self::EvalArtifact => "EvalArtifact",
            Self::RefreshTelemetryRecord => "RefreshTelemetryRecord",
            Self::PurgeDescriptor => "PurgeDescriptor",
            Self::RollbackDescriptor => "RollbackDescriptor",
        }
    }
}

impl fmt::Display for DescriptorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for DescriptorType {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "SourceDeltaBatch" | "source_delta_batch" => Ok(Self::SourceDeltaBatch),
            "ActivationDescriptor" | "activation_descriptor" => Ok(Self::ActivationDescriptor),
            "ActivePointer" | "active_pointer" => Ok(Self::ActivePointer),
            "LastKnownGoodDescriptor" | "last_known_good_descriptor" => {
                Ok(Self::LastKnownGoodDescriptor)
            }
            "RollbackPointer" | "rollback_pointer" => Ok(Self::RollbackPointer),
            "QueryResult" | "query_result" => Ok(Self::QueryResult),
            "RetrievalTrace" | "retrieval_trace" => Ok(Self::RetrievalTrace),
            "FutureQACandidate" | "future_qa_candidate" => Ok(Self::FutureQACandidate),
            "EvalArtifact" | "eval_artifact" => Ok(Self::EvalArtifact),
            "RefreshTelemetryRecord" | "refresh_telemetry_record" => {
                Ok(Self::RefreshTelemetryRecord)
            }
            "PurgeDescriptor" | "purge_descriptor" => Ok(Self::PurgeDescriptor),
            "RollbackDescriptor" | "rollback_descriptor" => Ok(Self::RollbackDescriptor),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactClass {
    PromptPayload,
    RetrievalTraceLogTelemetryShadowRollback,
    ActivationDescriptor,
    GovernanceDescriptor,
    EvalDescriptor,
    FutureQaDescriptor,
}

impl ArtifactClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PromptPayload => "prompt_payload",
            Self::RetrievalTraceLogTelemetryShadowRollback => {
                "retrieval_trace_log_telemetry_shadow_rollback"
            }
            Self::ActivationDescriptor => "activation_descriptor",
            Self::GovernanceDescriptor => "governance_descriptor",
            Self::EvalDescriptor => "eval_descriptor",
            Self::FutureQaDescriptor => "future_qa_descriptor",
        }
    }
}

impl fmt::Display for ArtifactClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ArtifactClass {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "prompt_payload" => Ok(Self::PromptPayload),
            "retrieval_trace_log_telemetry_shadow_rollback" => {
                Ok(Self::RetrievalTraceLogTelemetryShadowRollback)
            }
            "activation_descriptor" => Ok(Self::ActivationDescriptor),
            "governance_descriptor" => Ok(Self::GovernanceDescriptor),
            "eval_descriptor" => Ok(Self::EvalDescriptor),
            "future_qa_descriptor" => Ok(Self::FutureQaDescriptor),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreshnessStatus {
    Fresh,
    StaleSource,
    StaleIndex,
    StaleEval,
    StaleSnapshot,
    ActivationFailed,
    Unknown,
    Blocked,
}

impl FreshnessStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::StaleSource => "stale_source",
            Self::StaleIndex => "stale_index",
            Self::StaleEval => "stale_eval",
            Self::StaleSnapshot => "stale_snapshot",
            Self::ActivationFailed => "activation_failed",
            Self::Unknown => "unknown",
            Self::Blocked => "blocked",
        }
    }
}

impl fmt::Display for FreshnessStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for FreshnessStatus {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "fresh" => Ok(Self::Fresh),
            "stale_source" => Ok(Self::StaleSource),
            "stale_index" => Ok(Self::StaleIndex),
            "stale_eval" => Ok(Self::StaleEval),
            "stale_snapshot" => Ok(Self::StaleSnapshot),
            "activation_failed" => Ok(Self::ActivationFailed),
            "unknown" => Ok(Self::Unknown),
            "blocked" => Ok(Self::Blocked),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationStatus {
    Candidate,
    Validating,
    Verified,
    Activated,
    Failed,
    RolledBack,
    Superseded,
}

impl ActivationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::Validating => "validating",
            Self::Verified => "verified",
            Self::Activated => "activated",
            Self::Failed => "failed",
            Self::RolledBack => "rolled_back",
            Self::Superseded => "superseded",
        }
    }
}

impl fmt::Display for ActivationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ActivationStatus {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "candidate" => Ok(Self::Candidate),
            "validating" => Ok(Self::Validating),
            "verified" => Ok(Self::Verified),
            "activated" => Ok(Self::Activated),
            "failed" => Ok(Self::Failed),
            "rolled_back" => Ok(Self::RolledBack),
            "superseded" => Ok(Self::Superseded),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FallbackReasonCode {
    OnEmptyPool,
    OnLowScore,
    OnIndexMiss,
    OnStaleIndex,
    OnStaleSnapshot,
    OnEvalStale,
    OnActivationFailed,
    NoKbContext,
}

impl FallbackReasonCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OnEmptyPool => "on_empty_pool",
            Self::OnLowScore => "on_low_score",
            Self::OnIndexMiss => "on_index_miss",
            Self::OnStaleIndex => "on_stale_index",
            Self::OnStaleSnapshot => "on_stale_snapshot",
            Self::OnEvalStale => "on_eval_stale",
            Self::OnActivationFailed => "on_activation_failed",
            Self::NoKbContext => "no_kb_context",
        }
    }
}

impl fmt::Display for FallbackReasonCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for FallbackReasonCode {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "on_empty_pool" => Ok(Self::OnEmptyPool),
            "on_low_score" => Ok(Self::OnLowScore),
            "on_index_miss" => Ok(Self::OnIndexMiss),
            "on_stale_index" => Ok(Self::OnStaleIndex),
            "on_stale_snapshot" => Ok(Self::OnStaleSnapshot),
            "on_eval_stale" => Ok(Self::OnEvalStale),
            "on_activation_failed" => Ok(Self::OnActivationFailed),
            "no_kb_context" => Ok(Self::NoKbContext),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationStatus {
    Passed,
    Failed,
}

impl ValidationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
        }
    }
}

impl fmt::Display for ValidationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    pub status: ValidationStatus,
    pub fixture_dir: Option<String>,
    pub descriptors_checked: usize,
    pub errors: Vec<ValidationError>,
    pub diagnostics: Vec<Diagnostic>,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    pub fn passed(fixture_dir: Option<String>) -> Self {
        Self {
            status: ValidationStatus::Passed,
            fixture_dir,
            descriptors_checked: 0,
            errors: Vec::new(),
            diagnostics: Vec::new(),
            warnings: vec![
                "first_wave_offline_descriptor_rules_available".to_string(),
                "runtime_artifacts_snapshot_sqlite_raw_kb_and_source_register_not_read".to_string(),
            ],
        }
    }

    pub fn failed(fixture_dir: Option<String>, errors: Vec<ValidationError>) -> Self {
        Self {
            status: ValidationStatus::Failed,
            fixture_dir,
            descriptors_checked: 0,
            errors,
            diagnostics: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn from_parts(
        fixture_dir: Option<String>,
        descriptors_checked: usize,
        errors: Vec<ValidationError>,
        diagnostics: Vec<Diagnostic>,
    ) -> Self {
        let status = if errors.is_empty() && diagnostics.is_empty() {
            ValidationStatus::Passed
        } else {
            ValidationStatus::Failed
        };
        Self {
            status,
            fixture_dir,
            descriptors_checked,
            errors,
            diagnostics,
            warnings: vec![
                "runtime_artifacts_snapshot_sqlite_raw_kb_and_source_register_not_read".to_string(),
                "diagnostics_do_not_include_matched_values".to_string(),
            ],
        }
    }

    pub fn to_json_pretty(&self) -> String {
        let fixture_dir = match &self.fixture_dir {
            Some(value) => format!("\"{}\"", escape_json(value)),
            None => "null".to_string(),
        };
        let warnings = self
            .warnings
            .iter()
            .map(|warning| format!("    \"{}\"", escape_json(warning)))
            .collect::<Vec<_>>()
            .join(",\n");
        let errors = self
            .errors
            .iter()
            .map(|error| {
                let path = match &error.path {
                    Some(path) => format!("\"{}\"", escape_json(path)),
                    None => "null".to_string(),
                };
                format!(
                    "    {{\n      \"code\": \"{}\",\n      \"message\": \"{}\",\n      \"path\": {}\n    }}",
                    error.code.as_str(),
                    escape_json(&error.message),
                    path
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");
        let diagnostics = self
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.to_json_pretty(4))
            .collect::<Vec<_>>()
            .join(",\n");

        format!(
            "{{\n  \"status\": \"{}\",\n  \"fixture_dir\": {},\n  \"descriptors_checked\": {},\n  \"errors\": [\n{}\n  ],\n  \"diagnostics\": [\n{}\n  ],\n  \"warnings\": [\n{}\n  ]\n}}",
            self.status,
            fixture_dir,
            self.descriptors_checked,
            errors,
            diagnostics,
            warnings
        )
    }
}

pub(crate) fn escape_json(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if c.is_control() => escaped.push_str(&format!("\\u{:04x}", c as u32)),
            c => escaped.push(c),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_canonical_enums() {
        assert_eq!(
            "ActivationDescriptor".parse::<DescriptorType>(),
            Ok(DescriptorType::ActivationDescriptor)
        );
        assert_eq!(
            "RollbackPointer".parse::<DescriptorType>(),
            Ok(DescriptorType::RollbackPointer)
        );
        assert_eq!(
            "retrieval_trace_log_telemetry_shadow_rollback".parse::<ArtifactClass>(),
            Ok(ArtifactClass::RetrievalTraceLogTelemetryShadowRollback)
        );
        assert_eq!(
            "governance_descriptor".parse::<ArtifactClass>(),
            Ok(ArtifactClass::GovernanceDescriptor)
        );
        assert_eq!(
            "eval_descriptor".parse::<ArtifactClass>(),
            Ok(ArtifactClass::EvalDescriptor)
        );
        assert_eq!(
            "future_qa_descriptor".parse::<ArtifactClass>(),
            Ok(ArtifactClass::FutureQaDescriptor)
        );
        assert_eq!(
            "stale_snapshot".parse::<FreshnessStatus>(),
            Ok(FreshnessStatus::StaleSnapshot)
        );
        assert_eq!(
            "rolled_back".parse::<ActivationStatus>(),
            Ok(ActivationStatus::RolledBack)
        );
        assert_eq!(
            "no_kb_context".parse::<FallbackReasonCode>(),
            Ok(FallbackReasonCode::NoKbContext)
        );
    }

    #[test]
    fn report_json_is_structured() {
        let report = ValidationReport::passed(Some("fixtures".to_string()));
        let json = report.to_json_pretty();
        assert!(json.contains("\"status\": \"passed\""));
        assert!(json.contains("\"fixture_dir\": \"fixtures\""));
        assert!(json.contains("snapshot_sqlite_raw_kb_and_source_register_not_read"));
    }
}
