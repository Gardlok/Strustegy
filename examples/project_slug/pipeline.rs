//! Static proof, canonicalization, and validation stages.

use strustegy::{
    ByteLen, MaxBytes, NonEmpty, Policy, ProofPolicy, Refine, Rule, Strategy,
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

/// Require lowercase ASCII-alphanumeric segments separated by single hyphens.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct CanonicalSlugFormat;

impl Rule<String> for CanonicalSlugFormat {
    fn check(&self, value: &String) -> Result<(), ValidationError> {
        if value.is_empty() {
            // `NonEmpty` owns the empty-value error for this policy.
            return Ok(());
        }

        let mut previous_was_hyphen = false;

        for (index, byte) in value.bytes().enumerate() {
            match byte {
                b'a'..=b'z' | b'0'..=b'9' => previous_was_hyphen = false,
                b'-' if index != 0 && !previous_was_hyphen => previous_was_hyphen = true,
                _ => return Err(noncanonical_slug_error()),
            }
        }

        if previous_was_hyphen {
            Err(noncanonical_slug_error())
        } else {
            Ok(())
        }
    }
}

fn noncanonical_slug_error() -> ValidationError {
    ValidationError::new("canonical_slug_format", "noncanonical")
}

impl Policy<String> for ProjectSlugPolicy {
    type Rules = hlist_ty![NonEmpty, MaxBytes<32>, CanonicalSlugFormat];

    fn rules() -> Self::Rules {
        hlist![NonEmpty, MaxBytes::<32>, CanonicalSlugFormat]
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

#[cfg(test)]
mod tests {
    use strustegy::validate_all;

    use super::prepare_slug;
    use crate::types::ProjectSlugPolicy;

    #[test]
    fn canonical_policy_accepts_exact_canonical_form() {
        for value in ["rose", "rose-2", "strustegy-demo"] {
            assert!(
                validate_all::<ProjectSlugPolicy, _>(String::from(value)).is_ok(),
                "expected {value:?} to satisfy the canonical policy"
            );
        }
    }

    #[test]
    fn canonical_policy_rejects_noncanonical_forms() {
        for value in [
            "",
            "ABC_DEF",
            "abc_def",
            "-rose",
            "rose-",
            "rose--demo",
            "rose.demo",
            "rosé",
        ] {
            assert!(
                validate_all::<ProjectSlugPolicy, _>(String::from(value)).is_err(),
                "expected {value:?} to violate the canonical policy"
            );
        }

        assert!(
            validate_all::<ProjectSlugPolicy, _>("a".repeat(33)).is_err(),
            "expected an oversized slug to violate the canonical policy"
        );
    }

    #[test]
    fn preparation_normalizes_before_canonical_validation() {
        let slug = prepare_slug(b"  Strustegy_Demo  ").expect("input should canonicalize");

        assert_eq!(slug.get(), "strustegy-demo");
    }
}
