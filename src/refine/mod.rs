//! Zero-copy refinement and heterogeneous proof evidence.
//!
//! A [`Refine`] implementation may return evidence whose type is indexed by the
//! lifetime of the borrowed input. [`Prove`] recursively executes an HList of
//! refiners and computes a matching HList of evidence.

use crate::hlist::{HCons, HList, HNil};
use crate::proof::Witnessed;
use crate::validate::ValidationError;

pub mod refiners;

/// Refine borrowed input into lifetime-indexed evidence.
///
/// `Output<'input>` may borrow from `input`, but not from the refiner itself.
/// This permits policy constructors to build temporary refiner lists while the
/// returned evidence remains valid for the input lifetime.
pub trait Refine<Input: ?Sized> {
    type Output<'input>
    where
        Input: 'input;

    fn refine<'input>(&self, input: &'input Input)
    -> Result<Self::Output<'input>, ValidationError>;
}

/// Execute a heterogeneous HList of refiners against one borrowed input.
pub trait Prove<Input: ?Sized>: HList {
    type Evidence<'input>: HList
    where
        Input: 'input;

    fn prove<'input>(
        &self,
        input: &'input Input,
    ) -> Result<Self::Evidence<'input>, ValidationError>;
}

impl<Input: ?Sized> Prove<Input> for HNil {
    type Evidence<'input>
        = HNil
    where
        Input: 'input;

    fn prove<'input>(
        &self,
        _input: &'input Input,
    ) -> Result<Self::Evidence<'input>, ValidationError> {
        Ok(HNil)
    }
}

impl<Input, Head, Tail> Prove<Input> for HCons<Head, Tail>
where
    Input: ?Sized,
    Head: Refine<Input>,
    Tail: Prove<Input>,
{
    type Evidence<'input>
        = HCons<Head::Output<'input>, Tail::Evidence<'input>>
    where
        Input: 'input;

    fn prove<'input>(
        &self,
        input: &'input Input,
    ) -> Result<Self::Evidence<'input>, ValidationError> {
        Ok(HCons {
            head: self.head.refine(input)?,
            tail: self.tail.prove(input)?,
        })
    }
}

/// A proof policy fixes the exact refiner HList used to produce evidence.
pub trait ProofPolicy<Input: ?Sized> {
    type Refiners: Prove<Input>;

    fn refiners() -> Self::Refiners;
}

/// Execute a policy's refiners and return input-bound heterogeneous evidence.
///
/// The explicit return type preserves the exact GAT-computed evidence HList
/// associated with the selected proof policy.
#[allow(clippy::type_complexity)]
pub fn prove<'input, Policy, Input>(
    input: &'input Input,
) -> Result<
    Witnessed<'input, Input, Policy, <Policy::Refiners as Prove<Input>>::Evidence<'input>>,
    ValidationError,
>
where
    Input: ?Sized + 'input,
    Policy: ProofPolicy<Input>,
{
    let evidence = Policy::refiners().prove(input)?;
    Ok(Witnessed::new(input, evidence))
}

/// Project a policy's raw heterogeneous evidence into a named domain shape.
///
/// The policy remains responsible for the exact refiner HList, while this trait
/// gives ordinary callers a stable named output instead of requiring them to
/// manipulate recursive `HCons` types. The output GAT preserves any borrows from
/// the original input.
///
/// ```compile_fail
/// use strustegy::prelude::*;
///
/// enum TrimmedPolicy {}
///
/// impl ProofPolicy<str> for TrimmedPolicy {
///     type Refiners = hlist_ty![TrimmedNonEmpty];
///
///     fn refiners() -> Self::Refiners {
///         hlist![TrimmedNonEmpty]
///     }
/// }
///
/// impl ProjectEvidence<str> for TrimmedPolicy {
///     type Output<'input> = &'input str;
///
///     fn project<'input>(
///         _input: &'input str,
///         evidence: <Self::Refiners as Prove<str>>::Evidence<'input>,
///     ) -> Self::Output<'input> {
///         evidence.head
///     }
/// }
///
/// let projected = {
///     let input = String::from(" example ");
///     prove_projected::<TrimmedPolicy, _>(input.as_str()).unwrap()
/// };
/// println!("{projected}");
/// ```
pub trait ProjectEvidence<Input: ?Sized>: ProofPolicy<Input> {
    type Output<'input>
    where
        Input: 'input;

    fn project<'input>(
        input: &'input Input,
        evidence: <Self::Refiners as Prove<Input>>::Evidence<'input>,
    ) -> Self::Output<'input>;
}

/// Execute a proof policy and immediately project its evidence into the policy's
/// named output type.
///
/// Use [`prove`] when advanced code needs the raw HList evidence. Use this
/// function at ordinary domain boundaries where a named result is clearer.
pub fn prove_projected<'input, Policy, Input>(
    input: &'input Input,
) -> Result<Policy::Output<'input>, ValidationError>
where
    Input: ?Sized + 'input,
    Policy: ProjectEvidence<Input>,
{
    let witnessed = prove::<Policy, Input>(input)?;
    let (input, evidence) = witnessed.into_parts();
    Ok(Policy::project(input, evidence))
}
