
use crate::validation::error::ValidationError;

use std::any::{Any, TypeId};
use std::marker::PhantomData;

use std::error::Error;

use crate::validation::validity::Validity;
use crate::validation::target::Target;
use crate::validation::strategy::{Strategy, GenericStrategy};
use crate::validation::proof::{Proof, GenericProof};
use crate::validation::logic::{Scope, HigherOrderScope};




pub trait Validator<'a, T> {
    type Scope<'s>: Scope<'s, T> where Self: 's, Self: 'a;
    type Strategy<'s>: Strategy<T> where Self: 's;
    type Proof<'s>: Proof<'s, T> where Self: 's;

    fn validate(&'a self, scope: &Self::Scope<'a>, target: &T) -> bool;
}


// Generic //
pub struct GenericValidator<'a, T, S: Scope<'a, T>> {
    scope: S,
    _phantom: PhantomData<&'a T>,
}


// Higher Order //
pub struct HigherOrderValidator<'a, T, S: Scope<'a, T>> {
    scope: S,
    _phantom: PhantomData<&'a T>,
}

