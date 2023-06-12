

use crate::validation::error::ValidationError;


use std::any::{Any, TypeId};
use std::error::Error;
use std::marker::PhantomData;

use crate::validation::validity::Validity;
// use crate::validation::target::Target;
use crate::validation::strategy::{Strategy, GenericStrategy, GenericValidator, GenericProof};
use crate::validation::validator::Validator;
use crate::validation::logic::Scope;






pub trait Proof<'a, T> {
    type Strategy: Strategy<T>;
    fn validate(&'a self, strategy: &Self::Strategy, target: &T) -> bool;
}










