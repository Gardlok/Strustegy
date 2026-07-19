//! Proof-carrying values produced by policy validation.

use core::fmt;
use core::marker::PhantomData;

/// A value proven to satisfy `Policy` through Strustegy's validation entry points.
///
/// The constructor is crate-private so downstream callers cannot safely forge a
/// proof without executing the policy associated with the public validation API.
///
/// ```compile_fail
/// use strustegy::Validated;
///
/// enum ExamplePolicy {}
/// let _forged = Validated::<String, ExamplePolicy> {
///     value: String::from("unchecked"),
///     _policy: core::marker::PhantomData,
/// };
/// ```
#[must_use = "validated values should be consumed by trusted code"]
pub struct Validated<T, Policy> {
    value: T,
    _policy: PhantomData<fn() -> Policy>,
}

impl<T, Policy> Validated<T, Policy> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            value,
            _policy: PhantomData,
        }
    }

    /// Borrow the validated value.
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Consume the proof wrapper and return the validated value.
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: Clone, Policy> Clone for Validated<T, Policy> {
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<T: PartialEq, Policy> PartialEq for Validated<T, Policy> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq, Policy> Eq for Validated<T, Policy> {}

impl<T, Policy> AsRef<T> for Validated<T, Policy> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T, Policy> fmt::Debug for Validated<T, Policy> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Validated(<redacted>)")
    }
}
