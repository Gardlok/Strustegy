//! Built-in zero-copy refiners.

use crate::refine::Refine;
use crate::validate::ValidationError;

/// Return the trimmed input as a borrowed view.
#[derive(Debug, Clone, Copy, Default)]
pub struct Trimmed;

impl Refine<str> for Trimmed {
    type Output<'input>
        = &'input str
    where
        str: 'input;

    fn refine<'input>(&self, input: &'input str) -> Result<Self::Output<'input>, ValidationError> {
        Ok(input.trim())
    }
}

/// Return a non-empty trimmed view of the input.
#[derive(Debug, Clone, Copy, Default)]
pub struct TrimmedNonEmpty;

impl Refine<str> for TrimmedNonEmpty {
    type Output<'input>
        = &'input str
    where
        str: 'input;

    fn refine<'input>(&self, input: &'input str) -> Result<Self::Output<'input>, ValidationError> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            Err(ValidationError::new("trimmed_non_empty", "empty"))
        } else {
            Ok(trimmed)
        }
    }
}

/// Return a trimmed ASCII identifier view.
#[derive(Debug, Clone, Copy, Default)]
pub struct TrimmedAsciiIdentifier;

impl Refine<str> for TrimmedAsciiIdentifier {
    type Output<'input>
        = &'input str
    where
        str: 'input;

    fn refine<'input>(&self, input: &'input str) -> Result<Self::Output<'input>, ValidationError> {
        let trimmed = input.trim();
        let valid = !trimmed.is_empty()
            && trimmed
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-');

        if valid {
            Ok(trimmed)
        } else {
            Err(ValidationError::new(
                "trimmed_ascii_identifier",
                "invalid_identifier",
            ))
        }
    }
}

/// Interpret a byte slice as UTF-8 without allocating.
#[derive(Debug, Clone, Copy, Default)]
pub struct Utf8;

impl Refine<[u8]> for Utf8 {
    type Output<'input>
        = &'input str
    where
        [u8]: 'input;

    fn refine<'input>(&self, input: &'input [u8]) -> Result<Self::Output<'input>, ValidationError> {
        core::str::from_utf8(input).map_err(|_| ValidationError::new("utf8", "invalid_utf8"))
    }
}

/// Produce the byte length of string or byte-slice input as owned evidence.
#[derive(Debug, Clone, Copy, Default)]
pub struct ByteLen;

impl Refine<str> for ByteLen {
    type Output<'input>
        = usize
    where
        str: 'input;

    fn refine<'input>(&self, input: &'input str) -> Result<Self::Output<'input>, ValidationError> {
        Ok(input.len())
    }
}

impl Refine<[u8]> for ByteLen {
    type Output<'input>
        = usize
    where
        [u8]: 'input;

    fn refine<'input>(&self, input: &'input [u8]) -> Result<Self::Output<'input>, ValidationError> {
        Ok(input.len())
    }
}
