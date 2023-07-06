

use std::marker::PhantomData;
use std::any::Any;
use std::collections::{HashMap, HashSet};

use crate::inprogenitance::{Inprogenitance, Progeny, Progenation, Progenitor, InprogenitanceOps, MyInprogenitanceBuilder};
use crate::inprogenitance::PhantomLifetime;



// StrategyFn Trait
//
pub trait StrategyFnWithContext<'a, T>
where
    T: Clone,
{
    type Params: 'a;
    fn call(&self, target: &'a T, params: &Self::Params) -> bool;
}

pub(crate) struct StrategyFn<'a, T> { f: Box<dyn Fn(&T, &()) -> bool + 'a> }
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
pub trait StrategyWithContext<'a, T> where T: Clone,
{ fn execute(&self, target: &'a T) -> bool; }
//
impl<'a, T, S> StrategyWithContext<'a, T> for S
where
    S: Fn(&'a T) -> bool + Clone + 'static, T: Clone + 'a
{
    fn execute(&self, target: &'a T) -> bool { self(target) }
}
//
impl<'a, T, S, F> StrategyWithContext<'a, T> for MapStrategy<'a, T, S, F>
where
    S: StrategyWithContext<'a, T>, T: Clone,
    F: Fn(&'a T) -> S + Clone,
{
    fn execute(&self, target: &'a T) -> bool { (self.f)(self.target).execute(target) }
}
//
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
//
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
pub struct MapStrategy<'a, T, S, F> where S: StrategyWithContext<'a, T>, T: Clone
{
    strategy: S,
    f: F,
    target: &'a T,
}
impl<'a, T, S, F> MapStrategy<'a, T, S, F> where S: StrategyWithContext<'a, T>, T: Clone
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
pub struct ConditionalStrategy<'a, T, S> where S: StrategyWithContext<'a, T>, T: Clone  
{
    condition: MapStrategy<'a, T, S, Box<dyn Fn(&'a T) -> bool>>,
    true_strategy: S,
    false_strategy: S,
    target: &'a T,
}
impl<'a, T, S> ConditionalStrategy<'a, T, S> where S: StrategyWithContext<'a, T>, T: Clone
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
pub struct CompositeStrategy<'a, T, S> where S: StrategyWithContext<'a, T> + Clone, T: Clone
{
    strategies: HashSet<S>,
    progenies: Vec<Progeny<'a, T, S>>,
}
impl<'a, T, S> CompositeStrategy<'a, T, S> where S: StrategyWithContext<'a, T> + Clone, T: Clone
{
    pub fn new(strategies: HashSet<S>) -> Self {
        Self {
            strategies,
            progenies: Vec::new(),
        }
    }
}
impl<'a, T, S> CompositeStrategy<'a, T, S> where S: StrategyWithContext<'a, T> + Clone, T: Clone
{
    pub fn add_progeny(&mut self, progeny: Progeny<'a, T, S>) {
        self.progenies.push(progeny);
    }
    pub fn execute(&self, target: &'a T) -> bool {
        self.strategies.iter().all(|strategy| strategy.execute(target))
    }
}
impl<'a, T, S> CompositeStrategy<'a, T, S> where S: StrategyWithContext<'a, T> + Clone, T: Clone
{
    pub fn progenies(&self) -> &Vec<Progeny<'a, T, S>> {
        &self.progenies
    }
}

pub trait StrategyMap<'a, T, S>  where S: StrategyWithContext<'a, T>, T: Clone
{
    fn get(&self, target: &'a T) -> S;
}
impl<'a, T, S, M> StrategyMap<'a, T, S> for &'a M
where
    T: Clone + 'a,
    S: StrategyWithContext<'a, T>, 
    M: Fn(&'a T) -> S + 'a,
{
    fn get(&self, target: &'a T) -> S { (self)(target) }
}
impl<'a, T, S, M> StrategyMap<'a, T, S> for Box<M>
where
    T: Clone + 'a,
    S: StrategyWithContext<'a, T>,
    M: Fn(&'a T) -> S + 'a,
{
    fn get(&self, target: &'a T) -> S { (self)(target) }
}
