//! Proof-carrying values produced by validation and refinement policies.

use core::fmt;
use core::marker::PhantomData;

/// A receipt that the wrapped value passed `Policy` when this wrapper was created.
///
/// This is not necessarily a perpetual invariant for arbitrary `T`. Interior
/// mutability, custom `Clone` implementations, or later mutation through types
/// such as `Cell`, `RefCell`, `Mutex`, and atomics can change observable state
/// without rerunning the policy. Durable domain invariants should normally use
/// private constructors and representations whose mutation surface preserves
/// the invariant.
///
/// Validation also does not establish authorization or a permanent external-
/// state fact. See the repository's `PROOF_MODEL.md` for the full model.
///
/// The constructor is crate-private so downstream callers cannot safely forge a
/// receipt without executing the policy associated with the public validation
/// API.
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
///
/// Different policy markers remain different types even when they validate the
/// same underlying value type.
///
/// ```compile_fail
/// use strustegy::prelude::*;
///
/// enum FirstPolicy {}
/// enum SecondPolicy {}
///
/// impl Policy<String> for FirstPolicy {
///     type Rules = HNil;
///
///     fn rules() -> Self::Rules {
///         HNil
///     }
/// }
///
/// let first = validate_all::<FirstPolicy, _>(String::from("example")).unwrap();
/// let _second: Validated<String, SecondPolicy> = first;
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

/// A borrowed input accompanied by evidence from one specific proof policy.
///
/// `Witnessed` records that the policy's statically selected refiners processed
/// the input and produced this evidence. Evidence may mix borrowed views and
/// owned values. Any borrowed evidence shares `'input`, so safe Rust prevents it
/// from outliving the source input.
///
/// The witness means only what those refiners actually implement. It does not
/// establish authorization, trust in external state, or formal verification.
/// See the repository's `PROOF_MODEL.md` for the full model.
///
/// Its fields are private, so downstream safe code cannot manufacture witnessed
/// input and arbitrary evidence directly.
///
/// ```compile_fail
/// use strustegy::{HNil, Witnessed};
///
/// enum ExamplePolicy {}
/// let input = String::from("unchecked");
/// let _forged = Witnessed::<str, ExamplePolicy, HNil> {
///     input: input.as_str(),
///     evidence: HNil,
///     _policy: core::marker::PhantomData,
/// };
/// ```
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

impl<Input: ?Sized, Policy, Evidence> AsRef<Input> for Witnessed<'_, Input, Policy, Evidence> {
    fn as_ref(&self) -> &Input {
        self.input
    }
}

impl<Input: ?Sized, Policy, Evidence> fmt::Debug for Witnessed<'_, Input, Policy, Evidence> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Witnessed(<redacted>)")
    }
}
