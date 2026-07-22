//! Trusted-boundary construction for the mixed nested manifest.

use strustegy::{
    Policy, Strategy, StrategyExt, Validated, ValidationError, hlist, prove_projected, strategy_fn,
    validate_first,
};

use crate::domain::{
    BoundarySummary, ExecutionMode, ManifestHeader, ManifestSourceKind, NestedManifest,
    RawDeployment,
};
use crate::policies::{
    ArtifactNamePolicy, BuildError, BuildPhase, ChecksumSyntaxPolicy, EnvironmentInputProof,
    EnvironmentNamePolicy, ProjectInputProof, ProjectNamePolicy, RetryLimitPolicy, TimeoutPolicy,
};

pub fn borrowed_view_inside(source: &str, view: &str) -> bool {
    let source_start = source.as_ptr() as usize;
    let source_end = source_start + source.len();
    let view_start = view.as_ptr() as usize;
    let view_end = view_start + view.len();

    source_start <= view_start && view_end <= source_end
}

fn canonicalize_and_validate<P>(value: &str) -> Result<Validated<String, P>, ValidationError>
where
    P: Policy<String>,
{
    let pipeline = strategy_fn(|identifier: &str| identifier.to_ascii_lowercase()).then(
        strategy_fn(|canonical: String| validate_first::<P, _>(canonical)),
    );

    pipeline.apply(value)
}

pub fn build_manifest<'input>(
    raw: &RawDeployment<'input>,
) -> Result<NestedManifest<'input>, BuildError> {
    let project_input = prove_projected::<ProjectInputProof, _>(raw.project)
        .map_err(|error| BuildError::new(BuildPhase::ProjectRefinement, error))?;
    let environment_input = prove_projected::<EnvironmentInputProof, _>(raw.environment)
        .map_err(|error| BuildError::new(BuildPhase::EnvironmentRefinement, error))?;

    debug_assert!(borrowed_view_inside(
        raw.project,
        project_input.trimmed_identifier
    ));
    debug_assert!(borrowed_view_inside(
        raw.environment,
        environment_input.trimmed_identifier
    ));

    let project = canonicalize_and_validate::<ProjectNamePolicy>(project_input.trimmed_identifier)
        .map_err(|error| BuildError::new(BuildPhase::ProjectValidation, error))?;
    let environment =
        canonicalize_and_validate::<EnvironmentNamePolicy>(environment_input.trimmed_identifier)
            .map_err(|error| BuildError::new(BuildPhase::EnvironmentValidation, error))?;

    let artifact = validate_first::<ArtifactNamePolicy, _>(String::from(raw.artifact_name))
        .map_err(|error| BuildError::new(BuildPhase::ArtifactNameValidation, error))?;
    let checksum = validate_first::<ChecksumSyntaxPolicy, _>(String::from(raw.checksum))
        .map_err(|error| BuildError::new(BuildPhase::ChecksumSyntaxValidation, error))?;
    let timeout = validate_first::<TimeoutPolicy, _>(raw.timeout_ms)
        .map_err(|error| BuildError::new(BuildPhase::TimeoutValidation, error))?;
    let retry_limit = validate_first::<RetryLimitPolicy, _>(raw.retry_limit)
        .map_err(|error| BuildError::new(BuildPhase::RetryLimitValidation, error))?;

    let header = ManifestHeader {
        manifest_version: 1,
        source_kind: ManifestSourceKind::LocalExampleInput,
    };
    let mode = if raw.dry_run {
        ExecutionMode::DryRun
    } else {
        ExecutionMode::ApplyRequested
    };
    let summary = BoundarySummary {
        validated_fields: 6,
        borrowed_evidence_fields: 2,
        semantic_groups: 4,
    };

    Ok(hlist![
        header,
        hlist![project_input, environment_input],
        hlist![project, environment],
        mode,
        hlist![artifact, checksum],
        hlist![timeout, retry_limit],
        summary,
    ])
}
