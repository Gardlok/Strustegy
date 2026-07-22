//! Domain values and the complete mixed nested-HList type.

use strustegy::{Validated, hlist_ty};

use crate::policies::{
    ArtifactNamePolicy, ChecksumSyntaxPolicy, EnvironmentNamePolicy, ProjectNamePolicy,
    RetryLimitPolicy, TimeoutPolicy,
};

#[derive(Debug, Clone, Copy)]
pub struct RawDeployment<'input> {
    pub project: &'input str,
    pub environment: &'input str,
    pub artifact_name: &'input str,
    pub checksum: &'input str,
    pub timeout_ms: u64,
    pub retry_limit: u8,
    pub dry_run: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestSourceKind {
    LocalExampleInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestHeader {
    pub manifest_version: u8,
    pub source_kind: ManifestSourceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    DryRun,
    ApplyRequested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundarySummary {
    pub validated_fields: usize,
    pub borrowed_evidence_fields: usize,
    pub semantic_groups: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectInputEvidence<'input> {
    pub trimmed_identifier: &'input str,
    pub source_bytes: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnvironmentInputEvidence<'input> {
    pub trimmed_identifier: &'input str,
    pub source_bytes: usize,
}

pub type InputEvidenceGroup<'input> = hlist_ty![
    ProjectInputEvidence<'input>,
    EnvironmentInputEvidence<'input>
];

pub type IdentityGroup = hlist_ty![
    Validated<String, ProjectNamePolicy>,
    Validated<String, EnvironmentNamePolicy>,
];

pub type ArtifactGroup = hlist_ty![
    Validated<String, ArtifactNamePolicy>,
    Validated<String, ChecksumSyntaxPolicy>,
];

pub type LimitsGroup = hlist_ty![
    Validated<u64, TimeoutPolicy>,
    Validated<u8, RetryLimitPolicy>,
];

pub type NestedManifest<'input> = hlist_ty![
    ManifestHeader,
    InputEvidenceGroup<'input>,
    IdentityGroup,
    ExecutionMode,
    ArtifactGroup,
    LimitsGroup,
    BoundarySummary,
];
