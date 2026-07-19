//! Domain types that make trusted boundaries explicit.

use std::error::Error;
use std::fmt;

use strustegy::{Validated, ValidationError, ValidationErrors};

/// The static syntactic policy for an owned canonical project slug.
pub(crate) enum ProjectSlugPolicy {}

/// A project slug that is both policy-valid and currently available.
#[derive(Debug)]
pub struct AvailableProjectSlug {
    slug: Validated<String, ProjectSlugPolicy>,
}

impl AvailableProjectSlug {
    pub(crate) fn new(slug: Validated<String, ProjectSlugPolicy>) -> Self {
        Self { slug }
    }

    pub fn as_str(&self) -> &str {
        self.slug.get()
    }
}

#[derive(Debug)]
pub enum RegistrationError {
    Refinement(ValidationError),
    Validation(ValidationErrors),
    Unavailable,
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Refinement(error) => error.fmt(formatter),
            Self::Validation(errors) => errors.fmt(formatter),
            Self::Unavailable => formatter.write_str("project slug is already registered"),
        }
    }
}

impl Error for RegistrationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Refinement(error) => Some(error),
            Self::Validation(errors) => Some(errors),
            Self::Unavailable => None,
        }
    }
}

impl From<ValidationError> for RegistrationError {
    fn from(error: ValidationError) -> Self {
        Self::Refinement(error)
    }
}

impl From<ValidationErrors> for RegistrationError {
    fn from(errors: ValidationErrors) -> Self {
        Self::Validation(errors)
    }
}
