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
