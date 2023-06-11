
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



// pub trait Validator<'a, T> {
//     fn validate(&self, target: &'a T) -> Result<(), &'static str>;
// }

pub trait ValidationStrategy<'a, T> {
    type Validation: Validator;
    fn get_validator(&self) -> &Self::Validation;
}


