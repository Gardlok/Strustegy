

use std::sync::{Arc, RwLock, Mutex};
use std::fmt::{Debug, Display};
use std::marker::PhantomData;

use crate::validation::ValidationError;
use crate::validation::strategy::StrategyFn;
use crate::validation::strategy::Strategy;
use crate::validation::strategy::Scope;
use crate::validation::strategy::Proof;








pub enum Validness<T> {
    Valid(T),
    Invalid(T),
    NotChecked(T),
    Error(ValidationError),
}

impl<T> Validness<T> {
    pub fn value(&self) -> Option<&T> {
        match self {
            Validness::Valid(v) => Some(v),
            Validness::Invalid(v) => Some(v),
            Validness::NotChecked(v) => Some(v),
            Validness::Error(_) => None,
        }
    }

    pub fn into_result(self) -> Result<T, ValidationError> {
        match self {
            Validness::Valid(v) => Ok(v),
            Validness::Invalid(v) => Ok(v),
            Validness::NotChecked(v) => Ok(v),
            Validness::Error(e) => Err(e),
        }
    }
}




pub trait Target<'a> {
    type Value;
    type Proof: for<'s> Proof<'s, Self::Value>;

    fn value(&'a self) -> Self::Value;
    fn validate(&'a self, proof: &'a Self::Proof) -> bool;
}


// Target will be our GAT empowered functor, it will be a trait that is implemented for any
// type that can be validated. It will have a value method that returns a reference to the
// underlying value of the target, and a validate method that validates the target using
// a proof.
// INNARDS:
// (GAT) Value: 'a + ?Sized  
// (GAT) Proof: for<'s> Proof<'s, Self::Value> 
// fn value(&'a self) -> Self::Value;
// LIFETIME: ['targ, 'proo, 'valu] 
// OWNERSHIP: [self, proof, value] 
// CONCRETE: [Self::Value, Self::Proof]
impl<'a, T> Target<'a> for &'a mut T
where
    T: Target<'a>,
{
    type Value = T::Value;
    type Proof = T::Proof;

    fn value(&'a self) -> Self::Value {
        (**self).value()
    }

    fn validate(&'a self, proof: &'a Self::Proof) -> bool {
        (**self).validate(proof)
    }
}





// Validity //
//
// Validator will be our GAT empowered functor, it will be a trait that is implemented for any
// type that can be validated. It will have a value method that returns a reference to the
// underlying value of the target, and a validate method that validates the target using
// a proof.
pub trait Validator<'a, T> {
    type Scope: for<'s> Scope<'s, T>;

    fn validate(&'a self, scope: &Self::Scope, target: &T) -> bool;
}



// Validity //
//
// Validity is a struct that holds a target and a validator. It is used to validate the target
// using the validator. It is a GAT empowered functor, and it is a wrapper around a target and
// a validator. It has a validate method that validates the target using the validator.
pub struct Validity<'a, T, V: Validator<'a, T>> {
    validator: V,
    target: T,
    phantom: PhantomData<&'a T>,
}

// Supporting Creation
impl<'a, T, V: Validator<'a, T>> Validity<'a, T, V> {
    pub fn new(validator: V, target: T) -> Self {
        Validity {
            validator,
            target,
            phantom: PhantomData,
        }
    }
    pub fn validate(&'a self, scope: &V::Scope) -> Validness<&'a T> {
        if self.validator.validate(scope, &self.target) {
            Validness::Valid(&self.target)
        } else {
            Validness::Invalid(&self.target)
        }
    }
}

// Supporting Immutability
impl<'a, T, V: Validator<'a, T>> Validity<'a, T, V> {
    pub fn validator(&self) -> &V {
        &self.validator
    }

    pub fn target(&self) -> &T {
        &self.target
    }
}

// Supporting Mutability
impl<'a, T, V: Validator<'a, T>> Validity<'a, T, V> {
    pub fn validator_mut(&mut self) -> &mut V {
        &mut self.validator
    }
    pub fn target_mut(&mut self) -> &mut T {
        &mut self.target
    }
}       


// // Supporting Result
// impl <'a, T, V: Validator<'a, T>> Validity<'a, T, V> {
//     pub fn into_result(self) -> Result<T, ValidationError> {
//         match self.validate(&V::Scope::default()) {
//             Validness::Valid(v) => Ok(*v),
//             Validness::Invalid(v) => Ok(*v),
//             Validness::NotChecked(v) => Ok(*v),
//             Validness::Error(e) => Err(e),
//         }
//     }
// }

// Supporting Debug
impl<'a, T, V: Validator<'a, T>> Debug for Validity<'a, T, V>
where
    T: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Validity")
            .field("validator", &self.validator)
            .field("target", &self.target)
            .finish()
    }
}


