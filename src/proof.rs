//! Proof-carrying values produced by validation and refinement policies.

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

/// A borrowed input accompanied by policy-specific heterogeneous evidence.
///
/// The input and any borrowed evidence share `'input`, so the compiler prevents
/// evidence from outliving the value from which it was derived.
#[must_use = "witnessed values should be consumed by trusted code"]
pub struct Witnessed<'input, Input: ?Sized, Policy, Evidence> {
    input: &'input Input,
    evidence: Evidence,
    _policy: PhantomData<fn() -> Policy>,
}

impl<'input, Input: ?Sized, Policy, Evidence> Witnessed<'input, Input, Policy, Evidence> {
    pub(crate) fn new(input: &'input Input, evidence: Evidence) -> Self {
        Self {
            input,
            evidence,
            _policy: PhantomData,
        }
    }

    /// Borrow the original input.
    pub const fn get(&self) -> &'input Input {
        self.input
    }

    /// Borrow the heterogeneous evidence produced by the policy.
    pub const fn evidence(&self) -> &Evidence {
        &self.evidence
    }

    /// Consume the wrapper and return its evidence.
    pub fn into_evidence(self) -> Evidence {
        self.evidence
    }

    /// Consume the wrapper and return the input reference and evidence.
    pub fn into_parts(self) -> (&'input Input, Evidence) {
        (self.input, self.evidence)
    }
}

impl<'input, Input: ?Sized, Policy, Evidence> AsRef<Input>
    for Witnessed<'input, Input, Policy, Evidence>
{
    fn as_ref(&self) -> &Input {
        self.input
    }
}

impl<'input, Input: ?Sized, Policy, Evidence> fmt::Debug
    for Witnessed<'input, Input, Policy, Evidence>
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Witnessed(<redacted>)")
    }
}
