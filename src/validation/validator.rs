

    use std::any::Any;
use std::any::TypeId;

use crate::validation::error::ValidationError;


use crate::StrategyMap;
use crate::ValidationStrategy;


pub struct Validator<T> {
    strategies: StrategyMap<T>,
}

impl<T: 'static> Validator<T> {
    pub fn new() -> Self {
        Validator {
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
        self.strategies.remove_strategy(strategy_type_id);
        Ok(())
    }

    pub fn validate<'a>(&'a self, data: &'a T ) -> Validity<&T> {
        self.strategies.validate(&data)

    }
}


pub struct Validation<T: 'static> {
    pub validators: Vec<Validator<T>>,
    pub children: Vec<Validation<T>>,
}

impl<T: 'static> Validation<T> {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_validator(&mut self, validator: Validator<T>) {
        self.validators.push(validator);
    }


    pub fn remove_validator(&mut self, validator: &Validator<T>) {
        self.validators.retain(|v| !std::ptr::eq(v, validator));
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
    // pub fn batch_process_with_context<C>(&self, inputs: &[T], context: &C) -> Vec<bool>
    // where
    //     C: 'static,
    // {
    //     inputs
    //         .iter()
    //         .map(|input| self.is_valid_with_context(input, context))
    //         .collect()
    // }


    pub fn is_valid(&self, input: &T) -> bool {
        self.validators.iter().all(|validator| validator.validate(input).into()) &&
        self.children.iter().all(|child| child.is_valid(input))
    }

    // pub fn is_valid_with_context<C>(&self, input: &T, context: &C) -> bool
    // where
    //     C: 'static,
    // {
    //     self.validators.iter().all(|validator| validator.validate_with_context(input, context).into()) &&
    //     self.children.iter().all(|child| child.is_valid_with_context(input, context))
    // }
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

pub enum Validity<T> {
    Valid(T),
    Invalid((T, Vec<TypeId>)),
}

impl<T> Into<bool> for Validity<T> {
    fn into(self) -> bool {
        match self {
            Validity::Valid(_) => true,
            Validity::Invalid(_) => false,
        }
    }
}

impl<T> Into<Option<Vec<TypeId>>> for Validity<T> {
    fn into(self) -> Option<Vec<TypeId>> {
        match self {
            Validity::Valid(_) => None,
            Validity::Invalid(invalid_strategies) => invalid_strategies.1.into(),
        }
    }
}

impl<T> Validity<T> {
    pub fn is_valid(&self) -> bool {
        match self {
            Validity::Valid(_) => true,
            Validity::Invalid(_) => false,
        }
    }

    pub fn is_invalid(&self) -> bool {
        match self {
            Validity::Valid(_) => false,
            Validity::Invalid(_) => true,
        }
    }
}