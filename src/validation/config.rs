use std::fmt::Debug;
use std::marker::PhantomData;
use std::any::Any;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crate::validation::error::ValidationError;
use crate::validation::error::AnyValidationError;
use crate::validation::error::MultipleValidationError;

// use crate::validation::Validation;


use crate::validator::Validator;

use crate::strategies::*;


use crate::ValidationStrategy;


pub struct ValidationConfigBuilder<T> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T> ValidationConfigBuilder<T> {
    pub fn new() -> Self {
        ValidationConfigBuilder {
            strategies: Vec::new(),
        }
    }

    pub fn with_strategy(mut self, strategy: Box<dyn ValidationStrategy<T>>) -> Self {
        self.strategies.push(strategy);
        self
    }

    pub fn build(self) -> ValidationConfig<T> {
        ValidationConfig {
            strategies: self.strategies,
        }
    }
}

pub struct ValidationConfig<T> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T: 'static> ValidationConfig<T> {
    pub fn new() -> Self {
        Self { strategies: Vec::new() }
    }

    pub fn add_strategy(&mut self, strategy: Box<dyn ValidationStrategy<T>>) {
        self.strategies.push(strategy);
    }

    pub fn validate(&self, input: &T) -> bool {
        for strategy in &self.strategies {
            if !strategy.is_valid(input) {
                return false;
            }
        }
        true
    }
}


