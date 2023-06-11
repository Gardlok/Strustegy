
use crate::validation::error::ValidationError;

use std::any::{Any, TypeId};
use std::marker::PhantomData;

use std::error::Error;

use crate::validation::validity::Validity;
use crate::validation::target::Target;
use crate::validation::strategy::Strategy;
use crate::validation::proof::Proof;


pub trait Validator {
    type Proof<'a>: 'a where Self: 'a;
    type Error;

    fn validate<'a>(&'a mut self, f: &mut dyn FnMut(&mut Self::Proof<'a>) -> bool) -> bool;
}

pub trait GenericValidator<T>{
    type Proof<'a>: 'a where Self: 'a;
    type Error;

    fn validate<'a>(&'a mut self, f: &mut dyn FnMut(&mut Self::Proof<'a>) -> bool) -> bool;
}

pub struct HigherOrderValidator<T, V: GenericValidator<T>> {
    validators: Vec<V>,
    _phantom: PhantomData<T>,
}

impl<T, V: GenericValidator<T>> HigherOrderValidator<T, V> {
    pub fn new(validators: Vec<V>) -> Self {
        HigherOrderValidator {
            validators,
            _phantom: PhantomData,
        }
    }
}

pub struct HigherOrderProof<'a, T, V: GenericValidator<T>> {
    validators: &'a mut Vec<V>,
    _phantom: PhantomData<T>,
}

impl<T, V: GenericValidator<T>> Validator for HigherOrderValidator<T, V> {
    type Proof<'a> = HigherOrderProof<'a, T, V> where V: 'a, T: 'a;
    type Error = V::Error;

    fn validate<'a>(&'a mut self, f: &mut dyn FnMut(&mut Self::Proof<'a>) -> bool) -> bool {
        let mut proof = HigherOrderProof {
            validators: &mut self.validators,
            _phantom: PhantomData,
        };
        f(&mut proof)
    }
}


