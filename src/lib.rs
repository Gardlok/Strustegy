
use std::any::{Any, TypeId, type_name};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::marker::PhantomData;



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

pub trait Target<'a, T> {
    type Phantom<'p>: PhantomLifetime<'p>;
    fn target(&'a self) -> &'a T;
}  
//
pub trait TargetLifetime<'a, T> {
    type Phantom<'p>: PhantomLifetime<'p>;
}
//
pub trait TargetEnum<'a, T> {
    type Phantom<'p>: PhantomLifetime<'p>;
    fn target(&'a self) -> &'a dyn Any;
}
//
pub struct TargetObject<'a, T>(PhantomData<&'a mut T>);
impl<'a, T> TargetObject<'a, T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

// Strategy Trait
//
pub trait Strategy<'a, T> {
    type Life: StrategyLifetime<'a, T>;
    fn strategy(&'a self) -> &'a dyn Fn(&'a T) -> bool;
}
// Strategy Trait 
//
pub trait StrategyLifetime<'a, T> {
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

pub trait StrategyFnWithContext<'a, T> {
    type Params: 'a;
    fn call(&self, target: &T, params: &Self::Params) -> bool;
}
pub trait StrategyWithContext<'a, T> {
    type Params: 'a;
    fn strategies(&self) -> Vec<Box<dyn StrategyFnWithContext<'a, T, Params = Self::Params>>>;
}

// Operation (root context object)
//
pub struct Operation<'a, T> {
    target: &'a T,
    strategies: Vec<&'a dyn Fn(&'a T) -> bool>,
    parameters: HashMap<&'a str, &'a dyn Any>,
}
impl<'a, T> Operation<'a, T> {
    pub fn new(target: &'a T) -> Self {
        Self {
            target,
            strategies: Vec::new(),
            parameters: HashMap::new(),
        }
    }
}
impl<'a, T> Operation<'a, T> {
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
        true
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
        Self { target, strategy, parameters, }
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

/////////////// Strategy Prototypes ///////////////
struct StandardStrategy;
impl StandardStrategy {
    fn execute(&self, target: &i32, parameters: &HashMap<&str, &dyn Any>) -> bool {   
        for strategy in self.strategies() {
            if !strategy.call(target, parameters) {
                return false;
            }
        }
        true
    }
}
struct StandardStrategyFn<F>(F);
impl<'a, T, F> StrategyFnWithContext<'a, T> for StandardStrategyFn<F>
    where F: Fn(&T, &HashMap<&'a str, &'a dyn Any>) -> bool,
    {
    type Params = HashMap<&'a str, &'a dyn Any>;
    fn call(&self, target: &T, params: &Self::Params) -> bool {
        (self.0)(target, params)
    }
}
impl<'a, T> StrategyWithContext<'a, T> for StandardStrategy {
    type Params = HashMap<&'a str, &'a dyn Any>;
    // MVP
    fn strategies(&self) -> Vec<Box<dyn StrategyFnWithContext<'a, T, Params = Self::Params>>> {
        vec![
            Box::new(StandardStrategyFn(|target: &T, params: &Self::Params| { true })),
            Box::new(StandardStrategyFn(|target: &T, params: &Self::Params| { true })),
        ]
    }
}


// A function that uses s






