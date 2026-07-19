//! Static validation policies built from heterogeneous rule lists.

use core::fmt;
use std::error::Error;

use crate::hlist::{HCons, HList, HNil};
use crate::proof::Validated;

pub mod rules;

/// A redaction-safe validation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationError {
    rule: &'static str,
    code: &'static str,
}

impl ValidationError {
    pub const fn new(rule: &'static str, code: &'static str) -> Self {
        Self { rule, code }
    }

    pub const fn rule(&self) -> &'static str {
        self.rule
    }

    pub const fn code(&self) -> &'static str {
        self.code
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "validation rule '{}' failed with code '{}'",
            self.rule, self.code
        )
    }
}

impl Error for ValidationError {}

/// One or more validation failures collected without echoing rejected input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationErrors {
    errors: Vec<ValidationError>,
}

impl ValidationErrors {
    fn new(errors: Vec<ValidationError>) -> Self {
        Self { errors }
    }

    pub fn as_slice(&self) -> &[ValidationError] {
        &self.errors
    }

    pub fn into_vec(self) -> Vec<ValidationError> {
        self.errors
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "validation failed with {} error(s)",
            self.errors.len()
        )
    }
}

impl Error for ValidationErrors {}

/// One validation strategy for a borrowed value.
pub trait Rule<T> {
    fn check(&self, value: &T) -> Result<(), ValidationError>;
}

/// Recursive execution of a statically known HList of rules.
pub trait ValidateRules<T>: HList {
    fn check_first(&self, value: &T) -> Result<(), ValidationError>;

    fn check_all(&self, value: &T, errors: &mut Vec<ValidationError>);
}

impl<T> ValidateRules<T> for HNil {
    fn check_first(&self, _value: &T) -> Result<(), ValidationError> {
        Ok(())
    }

    fn check_all(&self, _value: &T, _errors: &mut Vec<ValidationError>) {}
}

impl<T, R, Tail> ValidateRules<T> for HCons<R, Tail>
where
    R: Rule<T>,
    Tail: ValidateRules<T>,
{
    fn check_first(&self, value: &T) -> Result<(), ValidationError> {
        self.head.check(value)?;
        self.tail.check_first(value)
    }

    fn check_all(&self, value: &T, errors: &mut Vec<ValidationError>) {
        if let Err(error) = self.head.check(value) {
            errors.push(error);
        }

        self.tail.check_all(value, errors);
    }
}

/// A policy fixes the exact rule-list type used to produce its proof.
///
/// Downstream crates may define their own local policy markers. They cannot
/// replace the rules of a policy marker owned by another crate because Rust's
/// coherence rules prevent overriding that foreign implementation.
pub trait Policy<T> {
    type Rules: ValidateRules<T>;

    fn rules() -> Self::Rules;
}

/// Validate with the first failing rule and return a policy proof on success.
pub fn validate_first<P, T>(value: T) -> Result<Validated<T, P>, ValidationError>
where
    P: Policy<T>,
{
    let rules = P::rules();
    rules.check_first(&value)?;
    Ok(Validated::new(value))
}

/// Evaluate every rule, collecting at most one error per statically known rule.
pub fn validate_all<P, T>(value: T) -> Result<Validated<T, P>, ValidationErrors>
where
    P: Policy<T>,
{
    let rules = P::rules();
    let mut errors = Vec::with_capacity(<P::Rules as HList>::LEN);
    rules.check_all(&value, &mut errors);

    if errors.is_empty() {
        Ok(Validated::new(value))
    } else {
        Err(ValidationErrors::new(errors))
    }
}
