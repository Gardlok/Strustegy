#[macro_use]
mod validation;
use validation::*;



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

    // Remove a strategy from the validation. The strategy will no longer be executed when the
    // validation is performed.
    pub fn remove_strategy(&mut self, strategy: &dyn Any) {
        self.strategies.retain(|s| !std::ptr::eq(s.as_any(), strategy));
    }

    // Remove a child validation from the validation. The child validation will no longer be
    // executed when the validation of the parent is performed.
    pub fn remove_child(&mut self, child: &Validation<T>) {
        self.children.retain(|c| !std::ptr::eq(c, child));
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


    // Perform the validation on a single input. Returns a boolean indicating whether the input is
    // valid or not. The context is passed to the validation strategies and can be used to store 
    // state between validations. 
    pub fn is_valid_with_context<C>(&self, input: &T, context: &C) -> bool
    where
        C: 'static,
    {
        self.strategies.iter().all(|strategy| strategy.is_valid(input)) &&
        self.children.iter().all(|child| child.is_valid_with_context(input, context))
    }

    // Perform the validation on a single input. Returns a boolean indicating whether the input
    // is valid or not. 
    pub fn is_valid(&self, input: &T) -> bool {
        self.strategies.iter().all(|strategy| strategy.is_valid(input)) &&
        self.children.iter().all(|child| child.is_valid(input))
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










