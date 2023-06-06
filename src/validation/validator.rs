

use std::fmt::Debug;
use std::marker::PhantomData;
use std::any::Any;
use std::any::TypeId;
use dashmap::DashSet as HashSet;
use dashmap::DashMap as HashMap;

use std::sync::Arc;

use crate::validation::error::ValidationError;
use crate::validation::error::AnyValidationError;
use crate::validation::error::MultipleValidationError;

// use crate::validation::Validation;
use crate::StrategyMap;
use crate::ValidationStrategy;


use crate::strategies::*;




    // ValidationStrategy trait
// pub trait ValidationStrategy<T> {
//     fn validate(&self, data: &T) -> Result<bool, ValidationError>;
// }

// Validator struct


pub struct Validator<T> {
    data: T,
    strategies: StrategyMap<T>,
}

impl<T: 'static> Validator<T> {
    pub fn new(data: T) -> Self {
        Validator {
            data,
            strategies: StrategyMap::<T>::new(),
        }
    }

    pub fn add_strategy(&mut self, strategy: Box<dyn ValidationStrategy<T> + 'static>) {
        self.strategies.insert_strategy(strategy);
    }

    pub fn add_strategies(&mut self, strategies: Vec<Box<dyn ValidationStrategy<T> + 'static>>) {
        for strategy in strategies {
            self.add_strategy(strategy);
        }
    }

    pub fn remove_strategy(&mut self, strategy: &dyn ValidationStrategy<T>) -> Result<(), ValidationError> {
        let strategy_type_id = strategy.as_any().type_id();
        self.strategies.remove_strategy(&strategy_type_id);
        Ok(())
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        for entry in self.strategies.hash_map.iter() {
            let (_, strategy) = entry.pair();
            if !strategy.is_valid(&self.data) {
                return Err(ValidationError::new("Validation failed".to_string()));
            }
        }
        Ok(())
    }
}

// ValidatorFactory module

pub struct ValidatorFactory<T: 'static> {
    validators: Vec<Validator<T>>,
}

impl<T: 'static> ValidatorFactory<T> {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    pub fn create_validator(&mut self, data: T) -> &mut Validator<T> {
        let validator = Validator::new(data);
        self.validators.push(validator);
        self.validators.last_mut().unwrap()
    }

    pub fn remove_validator(&mut self, validator: &Validator<T>) {
        self.validators.retain(|v| !std::ptr::eq(v, validator));
    }

    pub fn remove_strategy(&mut self, validator: &mut Validator<T>, strategy: &dyn ValidationStrategy<T>) -> Result<(), ValidationError> {
        validator.remove_strategy(strategy)
    }
}































// trait ValidationStrategy<T> {
//     fn validate(&self, data: &T) -> bool;
// }


// pub struct Validator<T> {
//     // Stores the data that will be validated by the strategies in the vector
//     // T is a generic type that will be specified when the Validator is created
//     // 'static is a lifetime specifier that means the Validator will live for 
//     // the entire duration of the program (which is what we want) 
//     data: T,
//     // Stores a list of strategies that will be used to validate the data
//     // Box<dyn ValidationStrategy<T>> is a trait object that can hold any type
//     // that implements the ValidationStrategy<T> trait (which is all of them)
//     strategies: HashSet<<Box<dyn ValidationStrategy<T>>>>, 
// }

// impl<T: 'static> Validator<T> {
//     // Creates a new Validator with the given data and an empty vector of strategies
//     // TODO: Consider Atomic queue from crossbeam crate it could look like this:
//     pub fn new(data: T) -> Self {
//         Validator {
//             data,
//             strategies: Vec::new(),
//         }
//     }

//     // Adds a strategy to the validator using a Box<dyn ValidationStrategy<T>> 
//     // (which is a trait object that can hold any type that implements the ValidationStrategy<T> trait)


//     pub fn add_strategies(&mut self, strategies: Vec<Box<dyn ValidationStrategy<T>>>) {
//         for strategy in strategies {
//             self.add_strategy(strategy);
//         }
//     }

//     // Removes a strategy from the validator, if it exists, using the strategy's type id
//     pub fn remove_strategy(&mut self, strategy: &dyn Any) {
//         // Get the type id of the strategy
//         let strategy_type_id = strategy.type_id();
//         // Get the type id of the strategy's type
//         let strategy_type = TypeId::of::<dyn ValidationStrategy<T>>();
//         // Get the type id of the strategy's concrete type
//         let concrete_type = TypeId::of::<Box<dyn ValidationStrategy<T>>>();

//         // Check if the strategy is a Box<dyn ValidationStrategy<T>>
//         if strategy_type_id == strategy_type {
//             // If it is, then we can just remove it from the validator
//             self.strategies.retain(|s| !std::ptr::eq(s, strategy));
//         } else if strategy_type_id == concrete_type {
//             // If it is not, then we need to check if it is a Box<dyn ValidationStrategy<T>>
//             // If it is, then we can just remove it from the validator
//             self.strategies.retain(|s| !std::ptr::eq(s, strategy));
//         } else {
//             // If it is not, then we need to check if it is a Box<dyn ValidationStrategy<T>>
//             // If it is, then we can just remove it from the validator
//             self.strategies.retain(|s| !std::ptr::eq(s, strategy));
//         }

//     }
//     // Perform the validation on the data. Returns a boolean indicating whether the data is valid
//     // or not. 
//     pub fn validate(&self) -> bool {
//         self.strategies.iter().all(|strategy| strategy.is_valid(&self.data))
//     }
// }



// pub struct ValidatorFactory<T: 'static> {
//     validators: Vec<Validation<T>>,
// }

// impl<T: 'static> ValidatorFactory<T> {
//     pub fn new() -> Self {
//         Self {
//             validators: Vec::new(),
//         }
//     }

//     pub fn create_validator(&mut self) -> &mut Validation<T> {
//         let validator = Validation::new();
//         self.validators.push(validator);
//         self.validators.last_mut().unwrap()
//     }

//     pub fn remove_validator(&mut self, validator: &Validation<T>) {
//         self.validators.retain(|v| !std::ptr::eq(v, validator));
//     }

//     pub fn remove_strategy(&mut self, validator: &mut Validation<T>, strategy: &dyn Any) {
//         // remove the strategy via using the strategy's type id
//         // Get the type id of the strategy
//         let strategy_type_id = strategy.type_id();
//         // Get the type id of the strategy's type
//         let strategy_type = TypeId::of::<dyn ValidationStrategy<T>>();
//         // Get the type id of the strategy's concrete type
//         let concrete_type = TypeId::of::<Box<dyn ValidationStrategy<T>>>();

//         // Check if the strategy is a Box<dyn ValidationStrategy<T>>
//         if strategy_type_id == strategy_type {
//             // If it is, then we can just remove it from the validator
//             validator.remove_strategy(strategy);
//         } else if strategy_type_id == concrete_type {
//             // If it is not, then we need to check if it is a Box<dyn ValidationStrategy<T>>
//             // If it is, then we can just remove it from the validator
//             validator.remove_strategy(strategy);
//         } else {
//             // If it is not, then we need to check if it is a Box<dyn ValidationStrategy<T>>
//             // If it is, then we can just remove it from the validator
//             validator.remove_strategy(strategy);
//         }

//         // TODO: Figure out more type validation

//     }
// }