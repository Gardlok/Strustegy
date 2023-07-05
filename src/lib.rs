
use std::any::{Any, TypeId, type_name};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::error::Error;
use std::hash::Hash;
use std::marker::PhantomData;

mod inprogenitance;
mod iterating;

pub use iterating::*;


use crate::inprogenitance::{Inprogenitance, Progeny, Progenation, Progenitor, InprogenitanceOps, MyInprogenitanceBuilder};

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
// Strategy Trait 
//
pub trait Strategy<'a, T> {
    type Life: StrategyLifetime<'a, T>;
    fn strategy(&'a self) -> &'a dyn Fn(&'a T) -> bool;
}
// StrategyFn Trait
//
pub trait StrategyFnWithContext<'a, T>
where
    T: Clone,
{
    type Params: 'a;
    fn call(&self, target: &'a T, params: &Self::Params) -> bool;
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

// Inprogenitance Operation Struct and Implementation
//
pub struct InprogenitanceOperation<'a, T: 'a + Clone, R: Clone, U: StrategyWithContext<'a, T>> {
    progenies: Vec<Progeny<'a, T, R>>,
    strategy: U,
}
impl<'a, T: 'a + Clone, R: Clone, U: StrategyWithContext<'a, T>> InprogenitanceOperation<'a, T, R, U> {
    pub fn new(strategy: U) -> Self {
        Self {
            progenies: Vec::new(),
            strategy,
        }
    }
}
impl<'a, T: 'a + Clone, R: Clone, U: StrategyWithContext<'a, T>> InprogenitanceOperation<'a, T, R, U> {
    fn progeny(&mut self, progeny: Progeny<'a, T, R>) -> &mut Self {
        self.progenies.push(progeny);
        self
    }
    fn progenies(&mut self, progenies: Vec<Progeny<'a, T, R>>) -> &mut Self {
        self.progenies = progenies;
        self
    }
    fn strategy(&mut self, strategy: U) -> &mut Self {
        self.strategy = strategy;
        self
    }
}



struct StrategyFn<'a, T> { f: Box<dyn Fn(&T, &()) -> bool + 'a> }
//
impl<'a, T> StrategyFn<'a, T> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&T, &()) -> bool + 'a,
    {
        Self { f: Box::new(f) }
    }
}
impl<'a, T> StrategyFn<'a, T> {
    pub fn call(&self, target: &T, params: &()) -> bool {
        (self.f)(target, params)
    }
}

// 
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
impl<'a, T, F> StrategyWithContext<'a, T> for StandardStrategy<'a, T, F>
where
    F: Fn(&'a T) -> bool, T: Clone
{
    fn execute(&self, target: &'a T) -> bool {
        (self.strategy)(target)
    }
}

// Core Strategy Component 
//
pub trait StrategyWithContext<'a, T>
where
    T: Clone,
{
    fn execute(&self, target: &'a T) -> bool;
}
impl<'a, T, S> StrategyWithContext<'a, T> for S
where
    S: Fn(&'a T) -> bool + Clone + 'static, T: Clone + 'a
{
    fn execute(&self, target: &'a T) -> bool {
        self(target)
    }
}
impl<'a, T, S, F> StrategyWithContext<'a, T> for MapStrategy<'a, T, S, F>
where
    S: StrategyWithContext<'a, T>, T: Clone,
    F: Fn(&'a T) -> S + Clone,
{
    fn execute(&self, target: &'a T) -> bool {
        (self.f)(self.target).execute(target)
    }
}
impl<'a, T, S> StrategyWithContext<'a, T> for ConditionalStrategy<'a, T, S>
where
    S: StrategyWithContext<'a, T>, T: Clone
{
    fn execute(&self, target: &'a T) -> bool {
        if self.condition.strategy.execute(target) {
            self.true_strategy.execute(target)
        } else {
            self.false_strategy.execute(target)
        }
    }
}
impl<'a, T, S> StrategyWithContext<'a, T> for CompositeStrategy<'a, T, S>
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone
{
    fn execute(&self, target: &'a T) -> bool {
        self.strategies.iter().all(|strategy| strategy.execute(target))
    }
}

// Mapping 
//
#[derive(Clone)]
pub struct MapStrategy<'a, T, S, F>
where
    S: StrategyWithContext<'a, T>, T: Clone
{
    strategy: S,
    f: F,
    target: &'a T,
}
impl<'a, T, S, F> MapStrategy<'a, T, S, F>
where
    S: StrategyWithContext<'a, T>, T: Clone
{
    pub fn new(strategy: S, f: F, target: &'a T) -> Self {
        Self {
            strategy,
            f,
            target,
        }
    }
}
//
pub struct ConditionalStrategy<'a, T, S>
where
    S: StrategyWithContext<'a, T>, T: Clone
{
    condition: MapStrategy<'a, T, S, Box<dyn Fn(&'a T) -> bool>>,
    true_strategy: S,
    false_strategy: S,
    target: &'a T,
}
impl<'a, T, S> ConditionalStrategy<'a, T, S>
where
    S: StrategyWithContext<'a, T>, T: Clone
{
    pub fn new(
        condition: MapStrategy<'a, T, S, Box<dyn Fn(&'a T) -> bool>>,
        true_strategy: S,
        false_strategy: S,
        target: &'a T,
    ) -> Self {
        Self {
            condition,
            true_strategy,
            false_strategy,
            target,
        }
    }
}
//
#[derive(Clone)]
pub struct CompositeStrategy<'a, T, S>
where
    S: StrategyWithContext<'a, T> + Clone, 
    T: Clone
{
    strategies: HashSet<S>,
    progenies: Vec<Progeny<'a, T, S>>,
}
impl<'a, T, S> CompositeStrategy<'a, T, S>
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone
{
    pub fn new(strategies: HashSet<S>) -> Self {
        Self {
            strategies,
            progenies: Vec::new(),
        }
    }
}
impl<'a, T, S> CompositeStrategy<'a, T, S>
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone
{
    pub fn add_progeny(&mut self, progeny: Progeny<'a, T, S>) {
        self.progenies.push(progeny);
    }
    pub fn execute(&self, target: &'a T) -> bool {
        self.strategies.iter().all(|strategy| strategy.execute(target))
    }
}
impl<'a, T, S> CompositeStrategy<'a, T, S>
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone
{
    pub fn progenies(&self) -> &Vec<Progeny<'a, T, S>> {
        &self.progenies
    }
}

pub trait StrategyMap<'a, T, S>
where
    S: StrategyWithContext<'a, T>, T: Clone
{
    fn get(&self, target: &'a T) -> S;
}
impl<'a, T, S, M> StrategyMap<'a, T, S> for &'a M
where
    S: StrategyWithContext<'a, T>, T: Clone + 'a,
    M: Fn(&'a T) -> S + 'a,
{
    fn get(&self, target: &'a T) -> S {
        (self)(target)
    }
}
impl<'a, T, S, M> StrategyMap<'a, T, S> for Box<M>
where
    S: StrategyWithContext<'a, T>, T: Clone + 'a,
    M: Fn(&'a T) -> S + 'a,
{
    fn get(&self, target: &'a T) -> S {
        (self)(target)
    }
}

// Map patterns to strategies
//
pub trait StrategyMapPattern<'a, T, S, P, M>
where
    S: StrategyWithContext<'a, T> + Clone + 'a + std::cmp::Eq + std::hash::Hash, T: Clone + 'a,
    P: for <'b> ProgenyOp<'b, T, S>,
    M: StrategyMap<'a, T, S>, CompositeStrategy<'a, T, S>: StrategyWithContext<'a, T> 
{
    fn map(&self, strategy_map: &dyn StrategyMap<'a, T, S>, target: &'a T) -> CompositeStrategy<'a, T, S>;
}
impl<'a, T, S, P, M> StrategyMapPattern<'a, T, S, P, M> for P
where
    S: StrategyWithContext<'a, T> + Clone + 'a + std::cmp::Eq + std::hash::Hash, T: Clone + 'a,
    P: for <'b> ProgenyOp<'b, T, S>,
    M: StrategyMap<'a, T, S>, CompositeStrategy<'a, T, S>: StrategyWithContext<'a, T> 
{
    fn map(&self, strategy_map: &dyn StrategyMap<'a, T, S>, target: &'a T) -> CompositeStrategy<'a, T, S> {
        let mut composite_strategy = CompositeStrategy::new(HashSet::new());
        let progenies = self.progeny(strategy_map, target);
        for progeny in progenies {
            composite_strategy.add_progeny(progeny);
        }
        composite_strategy
    }
}



// Progeny Operation
//
pub trait ProgenyOp<'a, T, S>
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone,
{
    fn progeny(&self, strategy_map: &dyn StrategyMap<'a, T, S>, target: &'a T) -> Vec<Progeny<'a, T, S>>;
}
impl<'a, T, S, P> ProgenyOp<'a, T, S> for P
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone,
    P: for <'b> ProgenyOp<'b, T, S>,
{
    fn progeny(&self, strategy_map: &dyn StrategyMap<'a, T, S>, target: &'a T) -> Vec<Progeny<'a, T, S>> {
        self.progeny(strategy_map, target)
    }
}

// The Leaf of Progeny Operation
pub struct ProgenyOpLeaf<'a, T, S>
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone,
{
    strategy: S,
    phantom: PhantomData<&'a T>,
}
impl<'a, T, S> ProgenyOpLeaf<'a, T, S>
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone,
{
    pub fn new(strategy: S) -> Self {
        Self {
            strategy,
            phantom: PhantomData,
        }
    }
}

pub struct ProgenyOpBuilder<'a, T, S>
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone,
{
    strategy: Option<S>,
    phantom: PhantomData<&'a T>,
}
impl<'a, T, S> ProgenyOpBuilder<'a, T, S>
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone,
{
    pub fn new() -> Self {
        Self {
            strategy: None,
            phantom: PhantomData,
        }
    }
    pub fn strategy(mut self, strategy: S) -> Self {
        self.strategy = Some(strategy);
        self
    }
    pub fn build(self) -> Result<ProgenyOpLeaf<'a, T, S>, &'static str> {
        match self.strategy {
            Some(strategy) => Ok(ProgenyOpLeaf::new(strategy)),
            None => Err("Strategy is missing"),
        }
    }
}

// Builders for ProgenyOp
//
// builder functions to be used by the StrategyMapPattern
//
pub fn progeny<'a, T, S, P>(progeny_op: P) -> P
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone,
    P: for <'b> ProgenyOp<'b, T, S>,
{
    progeny_op
}
pub fn progeny_op<'a, T, S, P>(progeny_op: P) -> P
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone,
    P: for <'b> ProgenyOp<'b, T, S>,
{
    progeny_op
}
pub fn progeny_op_box<'a, T, S, P>(progeny_op: P) -> Box<P>
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone,
    P: for <'b> ProgenyOp<'b, T, S>,
{
    Box::new(progeny_op)
}
pub fn progeny_op_leaf<'a, T, S>(strategy: S) -> Result<ProgenyOpLeaf<'a, T, S>, &'static str>
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone,
{
    ProgenyOpBuilder::new().strategy(strategy).build()
}
pub fn progeny_op_leaf_box<'a, T, S>(strategy: S) -> Result<Box<ProgenyOpLeaf<'a, T, S>>, &'static str>
where
    S: StrategyWithContext<'a, T> + Clone, T: Clone,
{
    match progeny_op_leaf(strategy) {
        Ok(progeny_op_leaf) => Ok(Box::new(progeny_op_leaf)),
        Err(err) => Err(err),
    }
}
