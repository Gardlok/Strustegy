//! Example-local proof policies, validation policies, rules, and build errors.

use core::fmt;
use std::error::Error;

use strustegy::{
    ByteLen, InclusiveU64, MaxBytes, NonEmpty, ProjectEvidence, ProofPolicy, Prove, Rule,
    TrimmedAsciiIdentifier, ValidationError, hlist, hlist_pat, hlist_ty, validation_policy,
};

use crate::domain::{EnvironmentInputEvidence, ProjectInputEvidence};

pub enum ProjectInputProof {}

impl ProofPolicy<str> for ProjectInputProof {
    type Refiners = hlist_ty![TrimmedAsciiIdentifier, ByteLen];

    fn refiners() -> Self::Refiners {
        hlist![TrimmedAsciiIdentifier, ByteLen]
    }
}

impl ProjectEvidence<str> for ProjectInputProof {
    type Output<'input> = ProjectInputEvidence<'input>;

    fn project<'input>(
        _input: &'input str,
        evidence: <Self::Refiners as Prove<str>>::Evidence<'input>,
    ) -> Self::Output<'input> {
        let hlist_pat![trimmed_identifier, source_bytes] = evidence;
        ProjectInputEvidence {
            trimmed_identifier,
            source_bytes,
        }
    }
}

pub enum EnvironmentInputProof {}

impl ProofPolicy<str> for EnvironmentInputProof {
    type Refiners = hlist_ty![TrimmedAsciiIdentifier, ByteLen];

    fn refiners() -> Self::Refiners {
        hlist![TrimmedAsciiIdentifier, ByteLen]
    }
}

impl ProjectEvidence<str> for EnvironmentInputProof {
    type Output<'input> = EnvironmentInputEvidence<'input>;

    fn project<'input>(
        _input: &'input str,
        evidence: <Self::Refiners as Prove<str>>::Evidence<'input>,
    ) -> Self::Output<'input> {
        let hlist_pat![trimmed_identifier, source_bytes] = evidence;
        EnvironmentInputEvidence {
            trimmed_identifier,
            source_bytes,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CanonicalIdentifierSyntax;

impl Rule<String> for CanonicalIdentifierSyntax {
    fn check(&self, value: &String) -> Result<(), ValidationError> {
        if value.is_empty() {
            return Ok(());
        }

        let valid = value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-')
        });

        if valid {
            Ok(())
        } else {
            Err(ValidationError::new(
                "canonical_identifier_syntax",
                "invalid_character",
            ))
        }
    }
}

pub enum ProjectNamePolicy {}

validation_policy! {
    ProjectNamePolicy: String => [NonEmpty, MaxBytes<48>, CanonicalIdentifierSyntax]
}

pub enum EnvironmentNamePolicy {}

validation_policy! {
    EnvironmentNamePolicy: String => [NonEmpty, MaxBytes<32>, CanonicalIdentifierSyntax]
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ArtifactNameSyntax;

impl Rule<String> for ArtifactNameSyntax {
    fn check(&self, value: &String) -> Result<(), ValidationError> {
        if value.is_empty() {
            return Ok(());
        }

        let bytes = value.as_bytes();
        let endpoints_are_alphanumeric = bytes.first().is_some_and(u8::is_ascii_alphanumeric)
            && bytes.last().is_some_and(u8::is_ascii_alphanumeric);
        let characters_are_allowed = bytes.iter().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        });

        if endpoints_are_alphanumeric && characters_are_allowed {
            Ok(())
        } else {
            Err(ValidationError::new(
                "artifact_name_syntax",
                "invalid_syntax",
            ))
        }
    }
}

pub enum ArtifactNamePolicy {}

validation_policy! {
    ArtifactNamePolicy: String => [NonEmpty, MaxBytes<96>, ArtifactNameSyntax]
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LowercaseSha256Syntax;

impl Rule<String> for LowercaseSha256Syntax {
    fn check(&self, value: &String) -> Result<(), ValidationError> {
        let valid = value.len() == 64
            && value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'));

        if valid {
            Ok(())
        } else {
            Err(ValidationError::new(
                "lowercase_sha256_syntax",
                "invalid_syntax",
            ))
        }
    }
}

pub enum ChecksumSyntaxPolicy {}

validation_policy! {
    ChecksumSyntaxPolicy: String => [LowercaseSha256Syntax]
}

pub enum TimeoutPolicy {}

validation_policy! {
    TimeoutPolicy: u64 => [InclusiveU64<1, 60_000>]
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RetryLimitRange;

impl Rule<u8> for RetryLimitRange {
    fn check(&self, value: &u8) -> Result<(), ValidationError> {
        if *value <= 5 {
            Ok(())
        } else {
            Err(ValidationError::new("retry_limit_range", "out_of_range"))
        }
    }
}

pub enum RetryLimitPolicy {}

validation_policy! {
    RetryLimitPolicy: u8 => [RetryLimitRange]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildPhase {
    ProjectRefinement,
    EnvironmentRefinement,
    ProjectValidation,
    EnvironmentValidation,
    ArtifactNameValidation,
    ChecksumSyntaxValidation,
    TimeoutValidation,
    RetryLimitValidation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildError {
    phase: BuildPhase,
    source: ValidationError,
}

impl BuildError {
    pub const fn new(phase: BuildPhase, source: ValidationError) -> Self {
        Self { phase, source }
    }

    #[cfg(test)]
    pub const fn phase(&self) -> BuildPhase {
        self.phase
    }

    #[cfg(test)]
    pub const fn validation_error(&self) -> ValidationError {
        self.source
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let field = match self.phase {
            BuildPhase::ProjectRefinement => "project refinement",
            BuildPhase::EnvironmentRefinement => "environment refinement",
            BuildPhase::ProjectValidation => "project validation",
            BuildPhase::EnvironmentValidation => "environment validation",
            BuildPhase::ArtifactNameValidation => "artifact-name validation",
            BuildPhase::ChecksumSyntaxValidation => "checksum-syntax validation",
            BuildPhase::TimeoutValidation => "timeout validation",
            BuildPhase::RetryLimitValidation => "retry-limit validation",
        };

        write!(formatter, "manifest build failed during {field}")
    }
}

impl Error for BuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
