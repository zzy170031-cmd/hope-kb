pub mod full_kb_rows;
pub mod leakage_count;

use crate::diagnostic::Diagnostic;
use crate::json_walk::JsonValue;

pub fn validate_first_wave_rules(descriptor: &JsonValue) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    diagnostics.extend(full_kb_rows::validate_full_kb_rows(descriptor));
    diagnostics.extend(leakage_count::validate_leakage_count(descriptor));
    diagnostics
}
