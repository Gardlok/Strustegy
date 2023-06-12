
use crate::validation::error::ValidationError;

use std::any::{Any, TypeId};
use std::marker::PhantomData;

use std::error::Error;

use crate::validation::validity::Validity;
// use crate::validation::target::Target;
use crate::validation::strategy::{Strategy, GenericStrategy, GenericProof};
use crate::validation::proof::{Proof, };
use crate::validation::logic::Scope;

use crate::validation::logic::CompositionOperator;





pub trait Validator<'a, T> {
    type Scope<'s>: Scope<'s, T> where Self: 's, Self: 'a;  // Scoped to lifetimes of the validator and the target
    type Strategy<'s>: Strategy<T> where Self: 's;          // Scoped to the lifetime of the validator
    type Proof<'s>: Proof<'s, T> where Self: 's;            // Scoped to the lifetime of the validator

    fn validate(&'a self, scope: &Self::Scope<'a>, target: &T) -> bool;
}

