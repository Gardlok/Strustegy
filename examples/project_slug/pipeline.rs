//! Static proof, canonicalization, and validation stages.

use strustegy::{
    AsciiIdentifier, ByteLen, MaxBytes, NonEmpty, Policy, ProofPolicy, Refine, Strategy,
    TrimmedAsciiIdentifier, Utf8, Validated, ValidationError, hlist, hlist_ty, prove, strategy_fn,
    validate_all,
};

use crate::types::{ProjectSlugPolicy, RegistrationError};

/// Evidence that the raw bytes are valid UTF-8, plus their original byte length.
enum RawInputProof {}

impl ProofPolicy<[u8]> for RawInputProof {
    type Refiners = hlist_ty![Utf8, ByteLen];

    fn refiners() -> Self::Refiners {
        hlist![Utf8, ByteLen]
    }
}

/// Produce an owned count alongside the borrowed, trimmed identifier view.
#[derive(Debug, Clone, Copy, Default)]
struct SegmentCount;

impl Refine<str> for SegmentCount {
    type Output<'input>
        = usize
    where
        str: 'input;

    fn refine<'input>(&self, input: &'input str) -> Result<Self::Output<'input>, ValidationError> {
        let count = input
            .trim()
            .split(&['-', '_'][..])
            .filter(|segment| !segment.is_empty())
            .count();

        Ok(count)
    }
}

/// Evidence that the string is a trimmed ASCII identifier, plus its segment count.
enum SlugShapeProof {}

impl ProofPolicy<str> for SlugShapeProof {
    type Refiners = hlist_ty![TrimmedAsciiIdentifier, SegmentCount];

    fn refiners() -> Self::Refiners {
        hlist![TrimmedAsciiIdentifier, SegmentCount]
    }
}

impl Policy<String> for ProjectSlugPolicy {
    type Rules = hlist_ty![NonEmpty, MaxBytes<32>, AsciiIdentifier];

    fn rules() -> Self::Rules {
        hlist![NonEmpty, MaxBytes::<32>, AsciiIdentifier]
    }
}

/// Convert untrusted bytes into an owned, policy-validated canonical slug.
pub fn prepare_slug(
    input: &[u8],
) -> Result<Validated<String, ProjectSlugPolicy>, RegistrationError> {
    let raw_evidence = prove::<RawInputProof, _>(input)?;
    let utf8 = raw_evidence.evidence().head;
    let input_bytes = raw_evidence.evidence().tail.head;

    let slug_evidence = prove::<SlugShapeProof, _>(utf8)?;
    let trimmed = slug_evidence.evidence().head;
    let segments = slug_evidence.evidence().tail.head;

    let canonicalize = strategy_fn(|value: &str| value.to_ascii_lowercase().replace('_', "-"));
    let canonical = canonicalize.apply(trimmed);

    println!("proof: {input_bytes} input bytes, {segments} slug segment(s)");

    Ok(validate_all::<ProjectSlugPolicy, _>(canonical)?)
}
