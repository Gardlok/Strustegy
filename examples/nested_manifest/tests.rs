use strustegy::{GetExt, HList, Validated, hlist_pat};

use crate::access::{
    ArtifactGroupIndex, FirstIndex, Get2Ext, HeaderIndex, IdentityGroupIndex, InputGroupIndex,
    LimitsGroupIndex, ModeIndex, SecondIndex, SummaryIndex,
};
use crate::build::{borrowed_view_inside, build_manifest};
use crate::domain::{
    ArtifactGroup, ExecutionMode, IdentityGroup, InputEvidenceGroup, LimitsGroup,
    ManifestSourceKind, RawDeployment,
};
use crate::policies::{
    ArtifactNamePolicy, BuildPhase, ChecksumSyntaxPolicy, ProjectNamePolicy,
};

const CHECKSUM: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn valid_raw() -> RawDeployment<'static> {
    RawDeployment {
        project: "  ROSE_Core  ",
        environment: "  Prod-West  ",
        artifact_name: "rose-core_1.2.3.tar",
        checksum: CHECKSUM,
        timeout_ms: 30_000,
        retry_limit: 3,
        dry_run: true,
    }
}

#[test]
fn valid_input_builds_the_complete_mixed_tree() {
    let manifest = build_manifest(&valid_raw()).expect("valid manifest should build");

    assert_eq!(manifest.len(), 7);
    assert_eq!(<InputEvidenceGroup<'static> as HList>::LEN, 2);
    assert_eq!(IdentityGroup::LEN, 2);
    assert_eq!(ArtifactGroup::LEN, 2);
    assert_eq!(LimitsGroup::LEN, 2);

    assert_eq!(
        manifest
            .get2::<IdentityGroupIndex, FirstIndex>()
            .get(),
        "rose_core"
    );
    assert_eq!(
        manifest
            .get2::<IdentityGroupIndex, SecondIndex>()
            .get(),
        "prod-west"
    );
    assert_eq!(
        manifest.get_at::<HeaderIndex>().source_kind,
        ManifestSourceKind::LocalExampleInput
    );
    assert_eq!(*manifest.get_at::<ModeIndex>(), ExecutionMode::DryRun);
    assert_eq!(manifest.get_at::<SummaryIndex>().validated_fields, 6);
}

#[test]
fn borrowed_evidence_points_inside_the_original_inputs() {
    let raw = valid_raw();
    let manifest = build_manifest(&raw).expect("valid manifest should build");
    let inputs = manifest.get_at::<InputGroupIndex>();
    let project = inputs.get_at::<FirstIndex>();
    let environment = inputs.get_at::<SecondIndex>();

    assert!(borrowed_view_inside(
        raw.project,
        project.trimmed_identifier
    ));
    assert!(borrowed_view_inside(
        raw.environment,
        environment.trimmed_identifier
    ));
    assert_eq!(project.source_bytes, raw.project.len());
    assert_eq!(environment.source_bytes, raw.environment.len());

    fn require_input_bound<'input>(
        _source: &'input str,
        _evidence: crate::domain::ProjectInputEvidence<'input>,
    ) {
    }
    require_input_bound(raw.project, *project);
}

#[test]
fn two_step_and_local_get2_indexing_agree() {
    let manifest = build_manifest(&valid_raw()).expect("valid manifest should build");

    let identity_group = manifest.get_at::<IdentityGroupIndex>();
    let two_step_environment = identity_group.get_at::<SecondIndex>();
    let get2_environment = manifest.get2::<IdentityGroupIndex, SecondIndex>();

    assert!(core::ptr::eq(two_step_environment, get2_environment));
    assert_eq!(get2_environment.get(), "prod-west");
    assert_eq!(
        manifest
            .get2::<ArtifactGroupIndex, FirstIndex>()
            .get(),
        "rose-core_1.2.3.tar"
    );
    assert_eq!(
        *manifest
            .get2::<LimitsGroupIndex, SecondIndex>()
            .get(),
        3
    );
}

#[test]
fn nested_pattern_destructures_every_cell() {
    let manifest = build_manifest(&valid_raw()).expect("valid manifest should build");

    let hlist_pat![
        header,
        hlist_pat![project_input, environment_input],
        hlist_pat![project, environment],
        mode,
        hlist_pat![artifact, checksum],
        hlist_pat![timeout, retry_limit],
        summary,
    ] = manifest;

    assert_eq!(header.manifest_version, 1);
    assert_eq!(project_input.trimmed_identifier, "ROSE_Core");
    assert_eq!(environment_input.trimmed_identifier, "Prod-West");
    assert_eq!(project.get(), "rose_core");
    assert_eq!(environment.get(), "prod-west");
    assert_eq!(mode, ExecutionMode::DryRun);
    assert_eq!(artifact.get(), "rose-core_1.2.3.tar");
    assert_eq!(checksum.get(), CHECKSUM);
    assert_eq!(*timeout.get(), 30_000);
    assert_eq!(*retry_limit.get(), 3);
    assert_eq!(summary.semantic_groups, 4);
}

#[test]
fn invalid_project_identifiers_are_rejected() {
    for project in ["   ", "rose/core", "rose core"] {
        let mut raw = valid_raw();
        raw.project = project;
        let error = build_manifest(&raw).expect_err("project should fail");
        assert_eq!(error.phase(), BuildPhase::ProjectRefinement);
    }

    let oversized = "a".repeat(49);
    let mut raw = valid_raw();
    raw.project = &oversized;
    let error = build_manifest(&raw).expect_err("oversized project should fail");
    assert_eq!(error.phase(), BuildPhase::ProjectValidation);
}

#[test]
fn invalid_environment_input_is_rejected() {
    let mut raw = valid_raw();
    raw.environment = "prod/west";
    let error = build_manifest(&raw).expect_err("environment should fail");
    assert_eq!(error.phase(), BuildPhase::EnvironmentRefinement);
}

#[test]
fn artifact_policy_rejects_invalid_characters() {
    for artifact_name in ["rose/core.tar", "Rose-Core.tar", "-rose.tar"] {
        let mut raw = valid_raw();
        raw.artifact_name = artifact_name;
        let error = build_manifest(&raw).expect_err("artifact syntax should fail");
        assert_eq!(error.phase(), BuildPhase::ArtifactNameValidation);
    }
}

#[test]
fn checksum_policy_requires_exact_lowercase_hex_syntax() {
    for checksum in [
        "abc",
        "g123456789abcdef0123456789abcdef0123456789abcdef0123456789abcde",
        "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF",
    ] {
        let mut raw = valid_raw();
        raw.checksum = checksum;
        let error = build_manifest(&raw).expect_err("checksum syntax should fail");
        assert_eq!(error.phase(), BuildPhase::ChecksumSyntaxValidation);
    }
}

#[test]
fn numeric_policies_enforce_documented_bounds() {
    for timeout_ms in [0, 60_001] {
        let mut raw = valid_raw();
        raw.timeout_ms = timeout_ms;
        let error = build_manifest(&raw).expect_err("timeout should fail");
        assert_eq!(error.phase(), BuildPhase::TimeoutValidation);
    }

    let mut raw = valid_raw();
    raw.retry_limit = 6;
    let error = build_manifest(&raw).expect_err("retry limit should fail");
    assert_eq!(error.phase(), BuildPhase::RetryLimitValidation);

    let mut raw = valid_raw();
    raw.retry_limit = 0;
    let manifest = build_manifest(&raw).expect("zero retries should remain valid");
    assert_eq!(
        *manifest
            .get2::<LimitsGroupIndex, SecondIndex>()
            .get(),
        0
    );
}

#[test]
fn errors_and_validated_debug_output_are_redaction_safe() {
    let rejected = "Private/Project.SECRET";
    let mut raw = valid_raw();
    raw.project = rejected;
    let error = build_manifest(&raw).expect_err("project should fail");

    assert!(!error.to_string().contains(rejected));
    assert!(!format!("{error:?}").contains(rejected));
    assert_eq!(error.validation_error().rule(), "trimmed_ascii_identifier");

    let validated: Validated<String, ProjectNamePolicy> =
        strustegy::validate_first::<ProjectNamePolicy, _>(String::from("private_project"))
            .expect("valid project should validate");
    assert_eq!(format!("{validated:?}"), "Validated(<redacted>)");

    let checksum: Validated<String, ChecksumSyntaxPolicy> =
        strustegy::validate_first::<ChecksumSyntaxPolicy, _>(String::from(CHECKSUM))
            .expect("valid checksum syntax should validate");
    assert_eq!(format!("{checksum:?}"), "Validated(<redacted>)");

    let artifact: Validated<String, ArtifactNamePolicy> =
        strustegy::validate_first::<ArtifactNamePolicy, _>(String::from("private.tar"))
            .expect("valid artifact should validate");
    assert_eq!(format!("{artifact:?}"), "Validated(<redacted>)");
}
