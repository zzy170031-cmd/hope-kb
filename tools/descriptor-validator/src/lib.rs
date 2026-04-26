pub mod error;
pub mod model;

use std::path::Path;

use error::{ValidationError, ValidationErrorCode};
use model::ValidationReport;

pub fn validate_fixture_dir(path: impl AsRef<Path>) -> ValidationReport {
    let path = path.as_ref();
    let display_path = path.display().to_string();

    if !path.exists() {
        return ValidationReport::failed(
            Some(display_path.clone()),
            vec![ValidationError::new(
                ValidationErrorCode::FixtureDirMissing,
                "fixture directory does not exist",
                Some(display_path),
            )],
        );
    }

    if !path.is_dir() {
        return ValidationReport::failed(
            Some(display_path.clone()),
            vec![ValidationError::new(
                ValidationErrorCode::FixturePathNotDirectory,
                "fixture path is not a directory",
                Some(display_path),
            )],
        );
    }

    ValidationReport::passed(Some(display_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_existing_fixture_dir_without_reading_artifacts() {
        let report = validate_fixture_dir(".");
        assert_eq!(report.status, model::ValidationStatus::Passed);
        assert_eq!(report.descriptors_checked, 0);
    }

    #[test]
    fn rejects_missing_fixture_dir() {
        let report = validate_fixture_dir("__missing_descriptor_fixture_dir__");
        assert_eq!(report.status, model::ValidationStatus::Failed);
        assert_eq!(report.errors.len(), 1);
        assert_eq!(
            report.errors[0].code,
            error::ValidationErrorCode::FixtureDirMissing
        );
    }
}
