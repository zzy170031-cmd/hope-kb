use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationErrorCode {
    FixtureDirMissing,
    FixturePathNotDirectory,
    InvalidArgument,
}

impl ValidationErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FixtureDirMissing => "fixture_dir_missing",
            Self::FixturePathNotDirectory => "fixture_path_not_directory",
            Self::InvalidArgument => "invalid_argument",
        }
    }
}

impl fmt::Display for ValidationErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub code: ValidationErrorCode,
    pub message: String,
    pub path: Option<String>,
}

impl ValidationError {
    pub fn new(
        code: ValidationErrorCode,
        message: impl Into<String>,
        path: Option<impl Into<String>>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            path: path.map(Into::into),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.path {
            Some(path) => write!(f, "{}: {} ({})", self.code, self.message, path),
            None => write!(f, "{}: {}", self.code, self.message),
        }
    }
}

impl std::error::Error for ValidationError {}

pub type ValidationResult<T> = Result<T, ValidationError>;
