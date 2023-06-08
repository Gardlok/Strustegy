
use std::any::{TypeId, Any};

mod validation;

use validation::{ValidationError, Validator, ValidationStrategy, StrategyMap, Strategy, Context, Validity,  StrategyContext};


#[cfg(test)]
use validation::tests;
use std::marker::PhantomData;
use dashmap::DashMap as HashMap;
use crossbeam::queue::SegQueue;



// pub trait Validation<T> {
//     fn validate(&self, input: &T) -> Validity<T>;
//     fn add_validator(&mut self, validator: Box<dyn ValidationStrategy<T>>);
//     fn remove_validator(&mut self, validator: TypeId) -> Result<(), ValidationError>;
// }