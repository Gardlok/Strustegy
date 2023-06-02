

use std::fmt::Debug;
use std::marker::PhantomData;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::dashmap::DashSet as HashSet;
use std::sync::Arc;

use crate::validation::error::ValidationError;
use crate::validation::error::AnyValidationError;
use crate::validation::error::MultipleValidationError;

use crate::validation::Validation;

use crate::ValidationStrategy;

use crate::strategies::*;

trait ValidatorStrategy<T> {
    fn validate(&self, data: &T) -> bool;
}

pub struct Validator<T> {
    // Stores the data that will be validated by the strategies in the vector
    // T is a generic type that will be specified when the Validator is created
    // 'static is a lifetime specifier that means the Validator will live for 
    // the entire duration of the program (which is what we want) 
    data: T,
    // Stores a list of strategies that will be used to validate the data
    // Box<dyn ValidatorStrategy<T>> is a trait object that can hold any type
    // that implements the ValidatorStrategy<T> trait (which is all of them)
    strategies: HashSet<<Box<dyn ValidationStrategy<T>>>::TypeId>,
}

impl<T: 'static> Validator<T> {
    // Creates a new Validator with the given data and an empty vector of strategies
    // TODO: Consider Atomic queue from crossbeam crate it could look like this:
    pub fn new(data: T) -> Self {
        Validator {
            data,
            strategies: Vec::new(),
        }
    }

    // Adds a strategy to the validator using a Box<dyn ValidatorStrategy<T>> 
    // (which is a trait object that can hold any type that implements the ValidatorStrategy<T> trait)


    pub fn add_strategies(&mut self, strategies: Vec<Box<dyn ValidationStrategy<T>>>) {
        for strategy in strategies {
            self.add_strategy(strategy);
        }
    }

    // Removes a strategy from the validator, if it exists, using the strategy's type id
    pub fn remove_strategy(&mut self, strategy: &dyn Any) {
        // Get the type id of the strategy
        let strategy_type_id = strategy.type_id();
        // Get the type id of the strategy's type
        let strategy_type = TypeId::of::<dyn ValidationStrategy<T>>();
        // Get the type id of the strategy's concrete type
        let concrete_type = TypeId::of::<Box<dyn ValidationStrategy<T>>>();

        // Check if the strategy is a Box<dyn ValidationStrategy<T>>
        if strategy_type_id == strategy_type {
            // If it is, then we can just remove it from the validator
            self.strategies.retain(|s| !std::ptr::eq(s, strategy));
        } else if strategy_type_id == concrete_type {
            // If it is not, then we need to check if it is a Box<dyn ValidationStrategy<T>>
            // If it is, then we can just remove it from the validator
            self.strategies.retain(|s| !std::ptr::eq(s, strategy));
        } else {
            // If it is not, then we need to check if it is a Box<dyn ValidationStrategy<T>>
            // If it is, then we can just remove it from the validator
            self.strategies.retain(|s| !std::ptr::eq(s, strategy));
        }

    }
    // Perform the validation on the data. Returns a boolean indicating whether the data is valid
    // or not. 
    pub fn validate(&self) -> bool {
        self.strategies.iter().all(|strategy| strategy.is_valid(&self.data))
    }
}



pub struct ValidatorFactory<T: 'static> {
    validators: Vec<Validation<T>>,
}

impl<T: 'static> ValidatorFactory<T> {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    pub fn create_validator(&mut self) -> &mut Validation<T> {
        let validator = Validation::new();
        self.validators.push(validator);
        self.validators.last_mut().unwrap()
    }

    pub fn remove_validator(&mut self, validator: &Validation<T>) {
        self.validators.retain(|v| !std::ptr::eq(v, validator));
    }

    pub fn remove_strategy(&mut self, validator: &mut Validation<T>, strategy: &dyn Any) {
        // remove the strategy via using the strategy's type id
        // Get the type id of the strategy
        let strategy_type_id = strategy.type_id();
        // Get the type id of the strategy's type
        let strategy_type = TypeId::of::<dyn ValidationStrategy<T>>();
        // Get the type id of the strategy's concrete type
        let concrete_type = TypeId::of::<Box<dyn ValidationStrategy<T>>>();

        // Check if the strategy is a Box<dyn ValidationStrategy<T>>
        if strategy_type_id == strategy_type {
            // If it is, then we can just remove it from the validator
            validator.remove_strategy(strategy);
        } else if strategy_type_id == concrete_type {
            // If it is not, then we need to check if it is a Box<dyn ValidationStrategy<T>>
            // If it is, then we can just remove it from the validator
            validator.remove_strategy(strategy);
        } else {
            // If it is not, then we need to check if it is a Box<dyn ValidationStrategy<T>>
            // If it is, then we can just remove it from the validator
            validator.remove_strategy(strategy);
        }

        // TODO: Figure out more type validation

    }
}