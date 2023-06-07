
use crate::validation::error::ValidationError;
use crate::validation::error::AnyValidationError;
use crate::validation::error::MultipleValidationError;

use crate::Validation;
use crate::ValidationStrategy;
use crate::StrategyMap;
use crate::config::ValidationConfig;
use crate::config::ValidationConfigBuilder;




pub struct ValidationBuilder<T: 'static> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T: 'static> ValidationBuilder<T> {
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
        }
    }

    pub fn add_strategy<S: ValidationStrategy<T> + 'static>(mut self, strategy: S) -> Self {
        self.strategies.push(Box::new(strategy));
        self
    }

    pub fn add_strategies<S: ValidationStrategy<T> + 'static>(mut self, strategies: Vec<S>) -> Self {
        for strategy in strategies {
            self.strategies.push(Box::new(strategy));
        }
        self
    }   

    pub fn build(self) -> Validation<T> {
        Validation {
            strategies: StrategyMap::new(),
            children: Vec::new(),
        }
    }
}

pub struct ValidationStrategyBuilder<T: 'static> {
    strategies: Vec<Box<dyn ValidationStrategy<T>>>,
}

impl<T: 'static> ValidationStrategyBuilder<T> {
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
        }
    }

    pub fn add_strategy<S: ValidationStrategy<T> + 'static>(mut self, strategy: S) -> Self {
        self.strategies.push(Box::new(strategy));
        self
    }

    pub fn build(self) -> Vec<Box<dyn ValidationStrategy<T>>> {
        self.strategies
    }
}


