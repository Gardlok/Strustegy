
use std::marker::PhantomData;
use crate::validation::Validator;
use crate::validation::error::ValidationError;
use crate::validation::Proof;
use crate::validation::Scope;


use crate::validation::strategy::Strategy;


//
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