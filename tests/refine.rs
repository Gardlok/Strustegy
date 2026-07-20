use strustegy::prelude::*;

pub enum ToolNameProof {}

impl ProofPolicy<str> for ToolNameProof {
    type Refiners = hlist_ty![TrimmedAsciiIdentifier, ByteLen];

    fn refiners() -> Self::Refiners {
        hlist![TrimmedAsciiIdentifier, ByteLen]
    }
}

#[test]
fn individual_refiner_returns_a_zero_copy_view() {
    let input = String::from("  sync_status  ");
    let refined = TrimmedNonEmpty
        .refine(input.as_str())
        .expect("input should refine");

    assert_eq!(refined, "sync_status");
    assert_eq!(refined.as_ptr(), input.as_ptr().wrapping_add(2));
}

#[test]
fn proof_policy_computes_heterogeneous_input_bound_evidence() {
    let input = String::from(" sync_status ");
    let witnessed = prove::<ToolNameProof, _>(input.as_str())
        .expect("tool name should satisfy the proof policy");

    let evidence = witnessed.evidence();

    assert_eq!(witnessed.get(), input.as_str());
    assert_eq!(evidence.head, "sync_status");
    assert_eq!(evidence.tail.head, input.len());

    let typed: hlist_ty![&str, usize] = witnessed.into_evidence();
    assert_eq!(typed, hlist!["sync_status", input.len()]);
}

#[test]
fn proof_evidence_can_be_decomposed_without_copying_input() {
    let input = String::from(" status ");
    let witnessed = prove::<ToolNameProof, _>(input.as_str())
        .expect("tool name should satisfy the proof policy");

    let (original, evidence) = witnessed.into_parts();

    assert_eq!(original, " status ");
    assert_eq!(evidence.head, "status");
    assert_eq!(evidence.tail.head, 8);
}

#[test]
fn invalid_refinement_returns_redaction_safe_error_codes() {
    let input = String::from(" not/allowed ");
    let error = prove::<ToolNameProof, _>(input.as_str()).expect_err("slash must be rejected");

    assert_eq!(error.rule(), "trimmed_ascii_identifier");
    assert_eq!(error.code(), "invalid_identifier");
    assert!(!error.to_string().contains(&input));
}

#[test]
fn utf8_refiner_borrows_the_original_byte_slice() {
    let bytes = b"rose".to_vec();
    let text = Utf8
        .refine(bytes.as_slice())
        .expect("bytes should be valid UTF-8");

    assert_eq!(text, "rose");
    assert_eq!(text.as_ptr(), bytes.as_ptr());
}

#[test]
fn invalid_utf8_is_rejected_without_echoing_bytes() {
    let bytes = [0xff, 0xfe];
    let error = Utf8
        .refine(bytes.as_slice())
        .expect_err("invalid UTF-8 should fail");

    assert_eq!(error.rule(), "utf8");
    assert_eq!(error.code(), "invalid_utf8");
}

#[test]
fn witnessed_debug_output_is_redacted() {
    let input = String::from("sync_status");
    let witnessed = prove::<ToolNameProof, _>(input.as_str())
        .expect("tool name should satisfy the proof policy");

    assert_eq!(format!("{witnessed:?}"), "Witnessed(<redacted>)");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToolEvidence<'input> {
    name: &'input str,
    source_bytes: usize,
}

impl ProjectEvidence<str> for ToolNameProof {
    type Output<'input> = ToolEvidence<'input>;

    fn project<'input>(
        _input: &'input str,
        evidence: <Self::Refiners as Prove<str>>::Evidence<'input>,
    ) -> Self::Output<'input> {
        let hlist_pat![name, source_bytes] = evidence;
        ToolEvidence { name, source_bytes }
    }
}

#[test]
fn named_evidence_projection_preserves_borrowed_identity() {
    let input = String::from("  sync_status  ");
    let projected =
        prove_projected::<ToolNameProof, _>(input.as_str()).expect("tool name should project");

    assert_eq!(projected.name, "sync_status");
    assert_eq!(projected.source_bytes, input.len());
    assert_eq!(projected.name.as_ptr(), input.as_ptr().wrapping_add(2));
}
