//! Mixed nested-HList deployment manifest example.

mod access;
mod build;
mod domain;
mod policies;

#[cfg(test)]
mod tests;

use strustegy::{GetExt, HList, hlist_pat};

use access::{
    ArtifactGroupIndex, FirstIndex, Get2Ext, HeaderIndex, IdentityGroupIndex, InputGroupIndex,
    LimitsGroupIndex, ModeIndex, SecondIndex, SummaryIndex,
};
use build::{borrowed_view_inside, build_manifest};
use domain::RawDeployment;

const CHECKSUM: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw = RawDeployment {
        project: "  ROSE_Core  ",
        environment: "  Prod-West  ",
        artifact_name: "rose-core_1.2.3.tar",
        checksum: CHECKSUM,
        timeout_ms: 30_000,
        retry_limit: 3,
        dry_run: true,
    };

    let manifest = build_manifest(&raw)?;

    let indexed_header = manifest.get_at::<HeaderIndex>();
    let indexed_mode = manifest.get_at::<ModeIndex>();
    let indexed_summary = manifest.get_at::<SummaryIndex>();
    println!(
        "outer indexing: version={}, source={:?}, mode={indexed_mode:?}, validated_fields={}",
        indexed_header.manifest_version,
        indexed_header.source_kind,
        indexed_summary.validated_fields,
    );

    let identity_group = manifest.get_at::<IdentityGroupIndex>();
    let environment = identity_group.get_at::<SecondIndex>();
    println!("two-step indexing: environment={}", environment.get());

    let project = manifest.get2::<IdentityGroupIndex, FirstIndex>();
    let artifact = manifest.get2::<ArtifactGroupIndex, FirstIndex>();
    let retry_limit = manifest.get2::<LimitsGroupIndex, SecondIndex>();
    println!(
        "local Get2Ext: project={}, artifact={}, retry_limit={}",
        project.get(),
        artifact.get(),
        retry_limit.get()
    );

    let outer_refs = manifest.refs();
    let hlist_pat![
        _header_ref,
        input_group_ref,
        _identity_group_ref,
        _mode_ref,
        _artifact_group_ref,
        _limits_group_ref,
        _summary_ref,
    ] = outer_refs;
    assert!(core::ptr::eq(
        input_group_ref,
        manifest.get_at::<InputGroupIndex>()
    ));

    let nested_input_refs = input_group_ref.refs();
    let hlist_pat![project_input_ref, environment_input_ref] = nested_input_refs;
    assert!(borrowed_view_inside(
        raw.project,
        project_input_ref.trimmed_identifier
    ));
    assert!(borrowed_view_inside(
        raw.environment,
        environment_input_ref.trimmed_identifier
    ));
    println!(
        "borrowing: outer refs are shallow; nested refs expose {} and {} borrowed evidence cells",
        project_input_ref.source_bytes, environment_input_ref.source_bytes
    );

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
    assert_eq!(checksum.get(), CHECKSUM);

    println!(
        "mixed typed tree: outer_cells=7, nested_groups={}, validated_fields={}, borrowed_evidence_fields={}",
        summary.semantic_groups, summary.validated_fields, summary.borrowed_evidence_fields
    );
    println!(
        "canonical values: project={}, environment={}, artifact={}, timeout_ms={}, retries={}, mode={mode:?}",
        project.get(),
        environment.get(),
        artifact.get(),
        timeout.get(),
        retry_limit.get()
    );
    println!(
        "Validation does not imply deployment authorization, artifact existence, checksum correctness, environment availability, current permission, or external-state freshness."
    );

    Ok(())
}
