

pub mod strategy;
pub mod config;
pub mod builder;
pub mod validator;
pub mod strategies;
pub mod error;

pub use strategy::*;
pub use config::*;
pub use builder::*;
pub use validator::*;
pub use strategies::*;
pub use error::*;

use std::any::Any;
use std::marker::PhantomData;

pub struct Validation<T: 'static> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
    children: Vec<Validation<T>>,
}

impl<T: 'static> Validation<T> {
    // Creates a new Validation with no strategies or children
    pub fn new() -> Self {
        Validation {
            strategies: Vec::new(),
            children: Vec::new(),
        }
    }

    // Add a strategy to the validation. The strategy will be executed when the validation is
    // performed.
    pub fn add_strategy<S>(&mut self, strategy: S)
    where
        S: ValidationStrategy<T> + 'static,
    {
        self.strategies.push(Box::new(strategy));
    }

    // Add a list of strategies to the validation. The strategies will be executed when the
    // validation is performed.
    pub fn add_strategies<S>(&mut self, strategies: Vec<S>)
    where
        S: ValidationStrategy<T> + 'static,
    {
        for strategy in strategies {
            self.add_strategy(strategy);
        }
    }

    // Add a child validation to the validation. The child validation will be executed when the
    // validation of the parent is performed.
    pub fn add_child(&mut self, child: Validation<T>) {
        self.children.push(child);
    }

    // Add a list of child validations to the validation. The child validations will be executed
    // when the validation of the parent is performed.
    pub fn add_children(&mut self, children: Vec<Validation<T>>) {
        for child in children {
            self.add_child(child);
        }
    }

    // Perform the validation on the given data. Returns a list of errors that occurred during
    // validation.
    pub fn validate(&self, input: &T) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for strategy in &self.strategies {
            if !strategy.is_valid(input) {
                errors.push(strategy.err as ValidationError);
            }
        }

        for child in &self.children {
            errors.append(&mut child.validate(input));
        }

        errors
    }
}



