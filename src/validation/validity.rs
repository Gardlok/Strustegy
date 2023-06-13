
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::ops::Deref;
use dashmap::DashMap;

use std::collections::BTreeMap;
use std::error::Error;


use crate::validation::strategy::Strategy;
use crate::validation::strategy::Scope;
use crate::validation::strategy::Proof;
use crate::validation::strategy::Target;

use crate::validation::error::ValidationError;
// use crate::validation::logic::Functor;

use std::sync::{Arc, RwLock, Mutex};
use std::fmt::{Debug, Display};




// pub struct Validity<T> {
//     value: Option<T>,
//     error: Option<ValidationError>,
// }

// impl<T> Validity<T> {
//     // Create a new Validity context from a valid value
//     pub fn new(value: T) -> Self {
//         Validity {
//             value: Some(value),
//             error: None,
//         }
//     }

//     // Create a new Validity from an error
//     pub fn error(error: ValidationError) -> Self {
//         Validity {
//             value: None,
//             error: Some(error),
//         }
//     }

//     // Transform the value contained in the Validity
//     pub fn map<U, F>(self, f: F) -> Validity<U>
//     where
//         F: FnOnce(T) -> U,
//     {
//         match self.value {
//             Some(value) => Validity::new(f(value)),
//             None => Validity::error(self.error.unwrap()),
//         }
//     }

//     // Chain multiple operations that may fail
//     pub fn and_then<U, F>(self, f: F) -> Validity<U>
//     where
//         F: FnOnce(T) -> Validity<U>,
//     {
//         match self.value {
//             Some(value) => f(value),
//             None => Validity::error(self.error.unwrap()),
//         }
//     }
// }








pub struct Validity<'a, T> {
    value: Option<T>,
    error: Option<ValidationError>,
    phantom: PhantomData<&'a T>,
}

impl<T> Validity<'_, T> {
    // Create a new Validity context from a valid value
    pub fn new(value: T) -> Self {
        Validity {
            value: Some(value),
            error: None,
            phantom: PhantomData,
        }
    }

    // Create a new Validity from an error
    pub fn error(error: ValidationError) -> Self {
        Validity {
            value: None,
            error: Some(error),
            phantom: PhantomData,
        }
    }

    // Transform the value contained in the Validity
    pub fn map<U, F>(self, f: F) -> Validity<'static, U>
    where
        F: FnOnce(T) -> U,
    {
        match self.value {
            Some(value) => Validity::new(f(value)),
            None => Validity::error(self.error.unwrap()),
        }
    }

    // Chain multiple operations that may fail
    pub fn and_then<U, F>(self, f: F) -> Validity<'static, U>
    where
        F: FnOnce(T) -> Validity<'static, U>,
    {
        match self.value {
            Some(value) => f(value),
            None => Validity::error(self.error.unwrap()),
        }
    }
}



pub trait OptionFunctor {
    type Inner;
    type Output;

    fn map<F, B>(self, f: F) -> Self::Output
    where
        F: FnOnce(Self::Inner) -> B;
}

pub enum Validness<T> {
    Valid(T),
    Invalid(T),
    NotChecked(T),
    Error(ValidationError),
}




// Add methods to get the value regardless of the variant
impl<T> Validness<T> {
    pub fn value(&self) -> &T {
        match self {
            Validness::Valid(v) => v,
            Validness::Invalid(v) => v,
            Validness::NotChecked(v) => v,
            Validness::Error(_) => panic!("Cannot get value from Error variant"),
        }
    }

    // Add a method to convert the variant to an Option
    pub fn to_option(self) -> Option<T> {
        match self {
            Validness::Valid(v) => Some(v),
            Validness::Invalid(v) => Some(v),
            Validness::NotChecked(v) => Some(v),
            Validness::Error(_) => None,
        }
    }

    // Add a method to convert the variant to a Result
    pub fn to_result(self) -> Result<T, ValidationError> {
        match self {
            Validness::Valid(v) => Ok(v),
            Validness::Invalid(v) => Ok(v),
            Validness::NotChecked(v) => Ok(v),
            Validness::Error(e) => Err(e),
        }
    }
}


impl<T> OptionFunctor for Validness<T> {
    type Inner = T;
    type Output = Option<T>;

    fn map<F, B>(self, f: F) -> Self::Output
    where
        F: FnOnce(Self::Inner) -> B,
    {
        match self {
            Validness::Valid(v) => Some(f(v)),       // compiler not happy with these variants 
            Validness::Invalid(v) => Some(f(v)),
            Validness::NotChecked(v) => Some(f(v)),
            Validness::Error(_) => None,
        }
    }
}




// Now we can use this functor to convert a Validness<T> into an Option<T>
// let validness = Validness::Valid(1);
// let result = validness.map(|x| x + 1);

// assert_eq!(result, Some(2));

// fn validate<'b, 'c, T, P>(proof: P, target: &'b T) -> Validity<'c, T> 
// where
//     P: Proof<'b, T>,
// {
//     let functor = Scope {
//         proof,
//     };

//     functor.map(|proof_value| {
//         proof.strategy().apply(proof_value)
//     })
// }