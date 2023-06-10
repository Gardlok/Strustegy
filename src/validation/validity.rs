
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::ops::Deref;
use dashmap::DashMap;

use crate::validation::*;

use crate::validation::error::ValidationError;

use std::sync::{Arc, RwLock, Mutex};

use std::error::Error;
use std::fmt::{Debug, Display};

pub trait Validity {
    type Target<'a, T>: 'a where Self: 'a; 
    type Validness<'a>: 'a where Self: 'a;
    type Error<E>: 'static + Send + Sync + Clone + Debug + Display + Error where Self: 'static;

    fn validate<'a, T>(&'a self, target: &'a T) -> Self::Validness<'a> where T: 'a; 

    fn validate_mut<'a, T>(&'a mut self, target: &'a mut T) -> Self::Validness<'a> where T: 'a;

    
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
