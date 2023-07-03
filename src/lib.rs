
use std::any::{Any, TypeId, type_name};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::Infallible;
use std::error::Error;
use std::marker::PhantomData;

mod inprogenitance;



mod test;

// The library must use the same lifetime as the caller, so that the caller can use the library's
// lifetime to create a reference to the library's data. 
// Map Trait 
//
pub struct Map<I, F> {
    iter: I,
    f: F,
}
// GAT Trait 
//
pub trait Gat<'a> { 
	type Item;
}
// Superceding Trait 
//
pub trait Super<'a> {
    type Super: 'a;
    
    fn super_(&'a self) -> &'a Self::Super;
}
// Scoping Object 
//
pub struct Scope<'a, T>(PhantomData<&'a mut T>);
impl<'a, T> Scope<'a, T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
// ParameterKey Trait (Variable) 
//
pub trait ParameterPair<'a, T, U> {
    type Phantom<'p>: PhantomLifetime<'p>;
    fn key(&'a self) -> &'a T;
    fn value(&'a self) -> &'a U;
}
// Parameterized Trait (Variable)
//
pub trait Parameterized<'a, T> {
    type Phantom<'p>: PhantomLifetime<'p>;
    fn parameters(&'a self) -> &'a HashMap<&'a str, &'a dyn Any>;
}
// ParameterEnum Trait (Variable)
//
pub trait ParameterEnum<'a, T> {
    type Phantom<'p>: PhantomLifetime<'p>;
    fn parameter(&'a self) -> &'a dyn Any;
}
// Parameterized Lifetime Trait - supporting hidden parameters.
//
pub trait ParameterizedLifetime<'a, T> {
    type Phantom<'p>: PhantomLifetime<'p>;
}
// Phantom Lifetime Trait - supports strategic lifetimes 
//                          delegated to the parameterized.
pub trait PhantomLifetime<'a> {
    type Phantom<'p>: PhantomLifetime<'p>;
}
// Lifetime Trait 
//
pub trait Lifetime<'a> {}
impl<'a> Lifetime<'a> for &'a dyn Sealed {}
impl<'a> Lifetime<'a> for &'a mut dyn Sealed {}
impl<'a> Lifetime<'a> for Box<dyn Sealed> {}
// Bounds Trait 
//
pub trait Bounds<T> {}
impl<'a, T> Bounds<T> for &'a T {}
impl<'a, T> Bounds<T> for &'a mut T {}
impl<T> Bounds<T> for Box<T> {}
// Sealed Trait 
//
pub trait Sealed {}
impl<T> Sealed for T {} 

// Strategy Trait 
//
pub trait Strategy<'a, T> {
    type Life: StrategyLifetime<'a, T>;
    fn strategy(&'a self) -> &'a dyn Fn(&'a T) -> bool;
}
//
pub trait StrategyFnWithContext<'a, T> {
    type Params: 'a;
    fn call(&self, target: &T, params: &Self::Params) -> bool;
}
// Strategy Lifetime Trait
//
pub trait StrategyLifetime<'a, T> {
    // Ensure the lifetime of the strategy is the same as the lifetime of the caller.
    type Phantom<'p>: PhantomLifetime<'p>;
}
// Strategy control object for contexting
//
pub struct StrategyObject<'a, T>(PhantomData<&'a mut T>);
impl<'a, T> StrategyObject<'a, T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
//
pub trait StrategyWithContext<'a, T> {
    type Params: 'a;
    fn strategies(&self) -> Vec<Box<dyn StrategyFnWithContext<'a, T, Params = Self::Params>>>;
}
impl<'a, T> StrategyWithContext<'a, T> for () {
    // When creating a strategy object, the caller can specify a strategy object that is empty.
    type Params = (); 
    fn strategies(&self) -> Vec<Box<dyn StrategyFnWithContext<'a, T, Params = Self::Params>>> {
        vec![]
    }
}





// Op Error Struct and Implementation
//
#[derive(Debug)]
pub struct OpError {
    message: String,
}
impl OpError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
impl std::fmt::Display for OpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "OpError: {}", self.message)
    }
}
impl Error for OpError {}

// Operation Struct and Implementation
//
pub struct Operation<'a, T, S = ()>
where
    S: StrategyWithContext<'a, T>,
{
    target: &'a T,
    strategies: Vec<&'a dyn Fn(&'a T) -> bool>,
    parameters: HashMap<&'a str, &'a dyn Any>,
    strategy: Option<S>,
    strategy_parameters: Option<S::Params>,
}
impl<'a, T> Operation<'a, T> {
    pub fn new(target: &'a T) -> Self {
        Self {
            target,
            strategies: Vec::new(),
            parameters: HashMap::new(),
            strategy: None,
            strategy_parameters: None,
        }
    }

    // Change Target
    pub fn target(&'a mut self, target: &'a T) -> &'a mut Self {
        self.target = target;
        self
    }

    pub fn strategy(&'a mut self, strategy: &'a dyn Fn(&'a T) -> bool) -> &'a mut Self {
        self.strategies.push(strategy);
        self
    }

    pub fn parameter(&'a mut self, key: &'a str, value: &'a dyn Any) -> &'a mut Self {
        self.parameters.insert(key, value);
        self
    }

    pub fn execute(&'a self) -> bool {
        for strategy in &self.strategies {
            if !strategy(self.target) {
                return false;
            }
        }

        if let Some(strategy) = &self.strategy {
            for strategy_fn in strategy.strategies() {
                if !strategy_fn.call(self.target, self.strategy_parameters.as_ref().unwrap()) {
                    return false;
                }
            }
        }

        true
    }

    pub fn add_target(&'a mut self, target: &'a T) -> &'a mut Self {
        self.target = target;
        self
    }

}

impl<'a, T, S> Operation<'a, T, S>
where
    S: StrategyWithContext<'a, T>,
{
    pub fn with_context(target: &'a T, strategy: S, parameters: S::Params) -> Self {
        Self {
            target,
            strategies: Vec::new(),
            parameters: HashMap::new(),
            strategy: Some(strategy),
            strategy_parameters: Some(parameters),
        }
    }
}
pub struct OperationWithContext<'a, T, S>
where
    S: StrategyWithContext<'a, T>,
{
    pub target: &'a T,
    pub strategy: S,
    pub parameters: S::Params,
}
impl<'a, T, S> OperationWithContext<'a, T, S>
where
    S: StrategyWithContext<'a, T>,
{
    pub fn new(target: &'a T, strategy: S, parameters: S::Params) -> Self {
        Self { target, strategy, parameters }
    }

    pub fn execute(&self) -> bool {
        for strategy_fn in self.strategy.strategies() {
            if !strategy_fn.call(self.target, &self.parameters) {
                return false;
            }
        }
        true
    }
}


// Lending Iterator
//                   
pub trait LendingIterator<'a> {
    type Item;

    fn next(&'a mut self, f: &'a dyn Fn(&mut Self, &mut Self::Item)) -> Option<Self::Item>;  
}

// ContextExtendingIterator - 
//
pub trait ContextExtendingIterator<'a>
where
    Self: Sized,
{
    type Item;
    type Gats: 'a + for<'b> Gat<'b, Item = Self::Item>;
    fn gats(&'a self) -> Self::Gats;
    fn map<'b, G>(self, f: G) -> Map<Self, G> 
    where
        G: FnMut(&mut Self::Item),
        Self: Sized,
    {
        Map { iter: self, f }
    }
}
impl<'a, I, F> ContextExtendingIterator<'a> for Map<I, F>  
where
    I: ContextExtendingIterator<'a>,
    F: FnMut(&mut I::Item),

{
    type Item = I::Item;
    type Gats = I::Gats;

    fn gats(&'a self) -> Self::Gats {
        self.iter.gats()
    }
    fn map<'b, G>(self, f: G) -> Map<Self, G> 
    where
        G: FnMut(&mut Self::Item),
        Self: Sized,
    {
        Map { iter: self, f }
    }
}

// OperationExtendingIterator 
//
pub trait OperationExtendingIterator<'a>
where
    Self: Sized,
{
    type Item;
    type Gats: 'a + for<'b> Gat<'b, Item = Self::Item>;
    fn gats(&'a self) -> Self::Gats;  
    fn map<'b, G>(self, f: G) -> Map<Self, G> 
    where
        G: FnMut(&mut Self::Item),
        Self: Sized,
    {
        Map { iter: self, f }
    }
}
impl<'a, I, F> OperationExtendingIterator<'a> for Map<I, F>  
where
    I: OperationExtendingIterator<'a>,
    F: FnMut(&mut I::Item),

{
    type Item = I::Item;
    type Gats = I::Gats;

    fn gats(&'a self) -> Self::Gats {
        self.iter.gats()
    }
    fn map<'b, G>(self, f: G) -> Map<Self, G> 
    where
        G: FnMut(&mut Self::Item),
        Self: Sized,
    {
        Map { iter: self, f }
    }
}

// ParameterObject
//
pub struct ParameterObject<'a> {
    parameters: HashMap<&'a str, &'a dyn Any>,
}
impl<'a> ParameterObject<'a> {
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
        }
    }
}
impl<'a> ParameterObject<'a> {
    pub fn parameter(&'a mut self, key: &'a str, value: &'a dyn Any) -> &'a mut Self {
        self.parameters.insert(key, value);
        self
    }
}
impl<'a> ParameterObject<'a> {
    pub fn get(&self, key: &'a str) -> Option<&&'a dyn Any> {
        self.parameters.get(key)
    }
}


/////////////// Strategic Ops

struct StrategyFn<'a, T>
{ 
    f: Box<dyn Fn(&T, &()) -> bool + 'a>,
   
 }
 impl<'a, T> StrategyFnWithContext<'a, T> for StrategyFn<'a, T> {
    type Params = ();

    fn call(&self, target: &T, _params: &Self::Params) -> bool {
        (self.f)(target, &())
    }
}





// StandardStrategy
//
pub struct StandardStrategy<'a, T, F>
where
    F: Fn(&'a T) -> bool,
{
    strategy: F,
    phantom: PhantomData<&'a T>,
}
impl<'a, T, F> StandardStrategy<'a, T, F>
where
    F: Fn(&'a T) -> bool,
{
    pub fn new(strategy: F) -> Self {
        Self {
            strategy,
            phantom: PhantomData,
        }
    }
}


pub struct CompositeStrategy<'a, T, S>
where
    S: StrategyWithContext<'a, T>,
{
    strategies: Vec<S>,
    phantom: PhantomData<&'a T>,
}
impl<'a, T, S> StrategyWithContext<'a, T> for CompositeStrategy<'a, T, S>
where
    S: StrategyWithContext<'a, T>,
{
    type Params = S::Params;

    fn strategies(&self) -> Vec<Box<dyn StrategyFnWithContext<'a, T, Params = Self::Params>>> {
        let mut all_strategies = Vec::new();
        for strategy in &self.strategies {
            all_strategies.extend(strategy.strategies());
        }
        all_strategies
    }
}

pub struct ConditionalStrategy<'a, T, S>
where
    S: StrategyWithContext<'a, T>,
{
    condition: Box<dyn Fn(&'a T) -> bool>,
    true_strategy: S,
    false_strategy: S,
    target: &'a T,
}
impl<'a, T, S> StrategyWithContext<'a, T> for ConditionalStrategy<'a, T, S>
where
    S: StrategyWithContext<'a, T>,
{
    type Params = S::Params;

    fn strategies(&self) -> Vec<Box<dyn StrategyFnWithContext<'a, T, Params = Self::Params>>> {
        if (self.condition)(self.target) {
            self.true_strategy.strategies()
        } else {
            self.false_strategy.strategies()
        }
    }
}

pub struct MapStrategy<'a, T, S, F>
where
    S: StrategyWithContext<'a, T>,
    F: Fn(&'a T) -> S,
{
    strategy: S,
    f: F,
    target: &'a T,
}
impl<'a, T, S, F> StrategyWithContext<'a, T> for MapStrategy<'a, T, S, F>
where
    S: StrategyWithContext<'a, T>,
    F: Fn(&'a T) -> S,
{
    type Params = S::Params;

    fn strategies(&self) -> Vec<Box<dyn StrategyFnWithContext<'a, T, Params = Self::Params>>> {
        (self.f)(self.target).strategies()
    }
}
impl<'a, T, S, F> MapStrategy<'a, T, S, F>
where
    S: StrategyWithContext<'a, T>,
    F: Fn(&'a T) -> S,
{
    pub fn new(strategy: S, f: F, target: &'a T) -> Self {
        Self {
            strategy,
            f,
            target,
        }
    }
}
impl<'a, T, S, F> MapStrategy<'a, T, S, F>
where
    S: StrategyWithContext<'a, T>,
    F: Fn(&'a T) -> S,
{
    pub fn map<'b, G>(self, f: G) -> MapStrategy<'a, T, Self, G>
    where
        G: Fn(&'a T) -> Self,
    {

        let target = self.target.clone();  // 

        MapStrategy::new(self, f, target)
    }
}

// Tests for MapStrategy, ConditionalStrategy, CompositeStrategy
//
#[cfg(test)]
mod tests_strats {
    use super::*;



    // #[test]
    // fn test_composite_strategy() {
    //     let composite_strategy = CompositeStrategy {
    //         strategies: vec![TrueStrategy, FalseStrategy],
    //         phantom: PhantomData,
    //     };

    //     let operation = Operation::with_context(&(), composite_strategy, ());
    //     assert_eq!(operation.execute(), false);
    // }

    // #[test]
    // fn test_conditional_strategy() {
    //     let conditional_strategy = ConditionalStrategy {
    //         condition: Box::new(|_| true),
    //         true_strategy: TrueStrategy,
    //         false_strategy: FalseStrategy,
    //         target: &(),
    //     };

    //     let operation = Operation::with_context(&(), conditional_strategy, ());
    //     assert_eq!(operation.execute(), true);
    // }

    // #[test]
    // fn test_map_strategy() {
    //     let map_strategy = MapStrategy::new(TrueStrategy, |target| {
    //         if *target == 0 {
    //             TrueStrategy
    //         } else {
    //             FalseStrategy
    //         }
    //     }, &0);

    //     let operation = Operation::with_context(&0, map_strategy, ());
    //     assert_eq!(operation.execute(), true);
    // }
}













//////////////// Operator ////////////////




// Validator Operation
//
pub trait Validator<'a, 'r> {
    type Target<'t>: Validate<'r>;
    type Error: std::error::Error;

    fn validate<'b: 'a, F, T>(&'b mut self, f: F) -> Result<T, Self::Error>
    where F: FnOnce(&mut Self::Target<'r>) -> T;
}
//
// pub trait Validate<'a> { fn validate<'b>(&'a self) -> Result<bool, OpError>; }
pub trait Validate<'a> { fn validate<'b>(&'a self) -> bool; }
//
pub struct ValidatorOp<'a, I> { target: I, op: Operation<'a, I> }
//
impl<'a, I> ValidatorOp<'a, I> {
    pub fn new(target: I, op: Operation<'a, I>) -> Self {
        Self { target, op }
    }
}
//
impl<'a, I> Validator<'a, 'a> for ValidatorOp<'a, I>
where
    I: Validate<'a>,
{
    type Target<'t> = I;
    type Error = OpError;

    fn validate<'b: 'a, F, R>(&'b mut self, f: F) -> Result<R, Self::Error>
        where F: FnOnce(&mut Self::Target<'a>) -> R,
    {  
            Ok( f(&mut self.target) )
    }
}


impl<'a> Validate<'a> for i32 {
    fn validate<'b>(&'a self) -> bool {
        
        if self % 2 == 0 { true } else { false }
    
    }
}

// Tests for ValidatorOp
//
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_validator_operation_strategy() {
        let mut validator = ValidatorOp::new(2, Operation::new(&2));
        let result = validator.validate(|target| { target.validate() });
        
        assert_eq!(result.unwrap(), true);
    }
}

