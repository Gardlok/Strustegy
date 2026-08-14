//! Small dependency-free validation rules.

use super::{Rule, ValidationError};

/// Require a string-like value to contain at least one byte.
#[derive(Debug, Clone, Copy, Default)]
pub struct NonEmpty;

impl<T> Rule<T> for NonEmpty
where
    T: AsRef<str>,
{
    fn check(&self, value: &T) -> Result<(), ValidationError> {
        if value.as_ref().is_empty() {
            Err(ValidationError::new("non_empty", "empty"))
        } else {
            Ok(())
        }
    }
}

/// Bound a string-like value by its UTF-8 byte length.
#[derive(Debug, Clone, Copy, Default)]
pub struct MaxBytes<const MAX: usize>;

impl<T, const MAX: usize> Rule<T> for MaxBytes<MAX>
where
    T: AsRef<str>,
{
    fn check(&self, value: &T) -> Result<(), ValidationError> {
        if value.as_ref().len() > MAX {
            Err(ValidationError::new("max_bytes", "too_long"))
        } else {
            Ok(())
        }
    }
}

/// Bound a string-like value by its Unicode scalar-value (`char`) count.
///
/// This is distinct from UTF-8 byte length and from grapheme-cluster or
/// user-perceived-character counts.
#[derive(Debug, Clone, Copy, Default)]
pub struct MaxUnicodeScalars<const MAX: usize>;

impl<T, const MAX: usize> Rule<T> for MaxUnicodeScalars<MAX>
where
    T: AsRef<str>,
{
    fn check(&self, value: &T) -> Result<(), ValidationError> {
        if value.as_ref().chars().count() > MAX {
            Err(ValidationError::new("max_unicode_scalars", "too_long"))
        } else {
            Ok(())
        }
    }
}

/// Require a `u64` value to lie within the closed interval `MIN..=MAX`.
#[derive(Debug, Clone, Copy, Default)]
pub struct InclusiveU64<const MIN: u64, const MAX: u64>;

impl<const MIN: u64, const MAX: u64> Rule<u64> for InclusiveU64<MIN, MAX> {
    fn check(&self, value: &u64) -> Result<(), ValidationError> {
        if MIN <= *value && *value <= MAX {
            Ok(())
        } else {
            Err(ValidationError::new("inclusive_u64", "out_of_range"))
        }
    }
}

/// Permit only ASCII alphanumeric characters, `_`, and `-`.
#[derive(Debug, Clone, Copy, Default)]
pub struct AsciiIdentifier;

impl<T> Rule<T> for AsciiIdentifier
where
    T: AsRef<str>,
{
    fn check(&self, value: &T) -> Result<(), ValidationError> {
        let valid = value
            .as_ref()
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-');

        if valid {
            Ok(())
        } else {
            Err(ValidationError::new(
                "ascii_identifier",
                "invalid_character",
            ))
        }
    }
}
