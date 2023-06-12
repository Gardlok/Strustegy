

use std::error::Error;
use std::marker::PhantomData;
use std::any::Any;
use std::any::TypeId;
use dashmap::DashMap as HashMap;
use dashmap::DashSet as HashSet;
use crossbeam::atomic::AtomicCell; 
use crossbeam_skiplist::SkipMap as TreeMap;

use crate::validation::error::ValidationError;



use crate::validation::validator::*;
use crate::validation::target::*;
use crate::validation::proof::*;

use crate::validation::validity::Validity;
// use crate::validation::proof::Proof;
// use crate::validation::logic::Scope;
// use crate::validation::validator::Validator;






pub trait Target<'a> {
    type Value: 'a;
    fn value(&'a self) -> Self::Value;
}

pub trait Strategy<T> {
    fn apply(&self, target: &T) -> bool;
}

pub trait Proof<'a, T> {
    type Strategy: for<'s> Strategy<T>;
    fn validate(&'a self, strategy: &Self::Strategy, target: &T) -> bool;
}

pub trait Scope<'a, T> {
    type Proof: for<'s> Proof<'s, T>;
    fn proof<'s>(&'s self) -> &'s Self::Proof;
    fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &T) -> bool;
}

pub trait Validator<'a, T> {
    type Scope: for<'s> Scope<'s, T>;
    fn validate(&'a self, scope: &Self::Scope, target: &T) -> bool;
}


// This strategy is a simple wrapper around a function that takes a target and returns a boolean
// This is the most basic strategy, and can be used to create more complex strategies
pub struct GenericValidator<'a, T, S: for<'s> Scope<'s, T>> {
    pub scope: S,
    pub(crate) _phantom: PhantomData<&'a T>,
}


pub struct GenericStrategy<'a, T, P: Proof<'a, T>> {
    pub proof: P,
    _phantom: std::marker::PhantomData<&'a T>,
}


pub struct GenericScope<'a, T, P: for<'s> Proof<'s, T>> {
    pub proof: P,
    _phantom: PhantomData<&'a T>,
}


pub struct GenericProof<'a, T, S: Strategy<T>> {
    pub strategy: S,
    _phantom: PhantomData<&'a T>,
}


// These are the implementations of the traits for the generic strategy
// enabling the generic strategy to be used as a concrete strategy and
// to be used as a generic strategy. Without that ability, the generic
// strategy would be useless. 
impl <'a, T, S: Strategy<T>> GenericProof<'a, T, S> {
    pub fn new(strategy: S) -> Self {
        Self {
            strategy,
            _phantom: PhantomData,
        }
    }
}


impl<'a, T, P: for<'s> Proof<'s, T>> GenericStrategy<'a, T, P> {
    pub fn new(proof: P) -> Self {
        Self {
            proof,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, P: for<'s> Proof<'s, T>> GenericScope<'a, T, P> {
    pub fn new(proof: P) -> Self {
        Self {
            proof,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, S: for<'s> Scope<'s, T>> GenericValidator<'a, T, S> {
    pub fn new(scope: S) -> Self {
        Self {
            scope,
            _phantom: PhantomData,
        }
    }
}




impl<'a, T, P: for<'s> Proof<'s, T, Strategy = P>> Scope<'a, T> for GenericScope<'a, T, P> {
    type Proof = P;
    fn proof<'s>(&'s self) -> &'s Self::Proof {
        &self.proof
    }
    fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &T) -> bool {
        proof.validate(&self.proof, target)
    }
}





// These traits support the use of the generic strategy as a concrete strategy
impl<'a, T, S: for<'s> Scope<'s, T>> Validator<'a, T> for GenericValidator<'a, T, S> {
    type Scope = S;
    fn validate(&'a self, scope: &Self::Scope, target: &T) -> bool {  // This is the same as the generic scope validate
        let proof = scope.proof();
        scope.validate(proof, target)
    }
}




impl<'a, T, P: for<'s> Proof<'s, T, Strategy = P>> Scope<'a, T> for GenericStrategy<'a, T, P> {
    type Proof = P;
    fn proof<'s>(&'s self) -> &'s Self::Proof {
        &self.proof
    }
    fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &T) -> bool {
        proof.validate(&self.proof, target)
    }
}



