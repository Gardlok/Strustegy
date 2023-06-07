

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



pub struct Validation<T: 'static> {
    pub strategies: StrategyMap<T>,
    pub children: Vec<Validation<T>>,
}

impl<T: 'static> Validation<T> {
    pub fn new() -> Self {
        Self {
            strategies: StrategyMap::<T>::new(),
            children: Vec::new(),
        }
    }

    pub fn add_strategy(&mut self, strategy: Box<dyn ValidationStrategy<T>>) {
        self.strategies.insert_strategy(strategy);
    }

    pub fn add_strategies(&mut self, strategies: Vec<Box<dyn ValidationStrategy<T>>>) {
        for strategy in strategies {
            self.add_strategy(strategy);
        }
    }

    pub fn remove_strategy(&mut self, strategy: &dyn Any) {
        self.strategies.remove_strategy(strategy);
    }

    pub fn add_child(&mut self, child: Validation<T>) {
        self.children.push(child);
    }

    pub fn add_children(&mut self, children: Vec<Validation<T>>) {
        for child in children {
            self.add_child(child);
        }
    }

    pub fn remove_child(&mut self, child: &Validation<T>) {
        self.children.retain(|c| !std::ptr::eq(c, child));
    }

    pub fn remove_children(&mut self, children: &[Validation<T>]) {
        for child in children {
            self.remove_child(child);
        }
    }


    // Perform the validation on a list of inputs. Returns a list of booleans indicating whether
    // each input is valid or not.
    pub fn batch_process(&self, inputs: &[T]) -> Vec<bool> {
        inputs.iter().map(|input| self.is_valid(input)).collect()
    }

    // Perform the validation on a list of inputs. Returns a list of booleans indicating whether
    // each input is valid or not. The context is passed to the validation strategies and can be
    // used to store state between validations. This is useful when the validation strategies
    // need to be stateful.
    pub fn batch_process_with_context<C>(&self, inputs: &[T], context: &C) -> Vec<bool>
    where
        C: 'static,
    {
        inputs
            .iter()
            .map(|input| self.is_valid_with_context(input, context))
            .collect()
    }


    pub fn is_valid(&self, input: &T) -> bool {
        self.strategies.validate(input) &&
        self.children.iter().all(|child| child.is_valid(input))
    }

    pub fn is_valid_with_context<C>(&self, input: &T, context: &C) -> bool
    where
        C: 'static,
    {
        self.strategies.validate(input) &&
        self.children.iter().all(|child| child.is_valid_with_context(input, context))
    }
}





pub enum ValidationLogic<T> {
    // All strategies must be valid for the input to be valid
    All(Vec<Box<dyn ValidationStrategy<T>>>), 
    // Any strategy must be valid for the input to be valid
    Any(Vec<Box<dyn ValidationStrategy<T>>>),
    // The input must be valid for all strategies to be valid (the opposite of All)
    None(Vec<Box<dyn ValidationStrategy<T>>>),
    // The input must be valid for any strategy to be valid (the opposite of Any)
    Not(Vec<Box<dyn ValidationStrategy<T>>>),
}
    
impl<T: 'static> ValidationLogic<T> {
    pub fn validate(&self, input: &T) -> bool {
        match self {
            ValidationLogic::All(strategies) => strategies.iter().all(|strategy| strategy.is_valid(input)),
            ValidationLogic::Any(strategies) => strategies.iter().any(|strategy| strategy.is_valid(input)),
            ValidationLogic::None(strategies) => strategies.iter().all(|strategy| !strategy.is_valid(input)),
            ValidationLogic::Not(strategies) => strategies.iter().any(|strategy| !strategy.is_valid(input)),
        }
    }
}

