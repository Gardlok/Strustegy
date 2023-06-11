

use crate::validation::error::ValidationError;


use std::any::{Any, TypeId};
use std::error::Error;
use std::marker::PhantomData;

use crate::validation::validity::Validity;
use crate::validation::target::Target;
use crate::validation::strategy::{Strategy, GenericStrategy, HigherOrderStrategy};
use crate::validation::validator::{Validator, GenericValidator, HigherOrderValidator};
use crate::validation::logic::Scope;






pub trait Proof<'a, T> {
    type Strategy: Strategy<T>;
    fn validate(&'a self, strategy: &Self::Strategy, target: &T) -> bool;
}

pub struct GenericProof<'a, T, S: Strategy<T>> {
    strategy: S,
    _phantom: PhantomData<&'a T>,
}

pub struct HigherOrderProof<'a, T, P: Proof<'a, T>> {
    pub proofs: Vec<P>,
    _phantom: std::marker::PhantomData<&'a T>,
}







