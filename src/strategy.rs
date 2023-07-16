

use std::marker::PhantomData;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use arraydeque::ArrayDeque;

use crate::Validaty;
use crate::OpError;
use crate::listing::{HList, HCons, HNil};

use crate::HListGate;
use crate::hlist;




// Applicative Trait - This is our Bread and Butter
// Base trait for all strategies that can be applied to a target.
pub trait Applicative<'a, T, S> where S: StrategyWithContext<'a, T> + Clone, T: Clone + 'a,
{
    type Output;
    fn bind<F, U>(self, f: F) -> Self::Output where F: FnOnce(&'a [T]) -> U;
    fn bind_ext<F, U>(self, f: F) -> Self::Output where F: FnOnce(&'a [T]) -> Result<U, OpError>;
}

// Core Strategy Component 
// 
pub trait StrategyWithContext<'a, T> { fn execute(&self, target: &'a T) -> bool; }
//
impl<'a, T: 'a, S> StrategyWithContext<'a, T> for S
where
    S: Fn(&'a T) -> Result<(), OpError>
{
    fn execute(&self, target: &'a T) -> bool {
        match self(target) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}


// Dynamic Dispatched Strategy
pub struct Strategy<'a, T> { strategy: Box<dyn StrategyWithContext<'a, T> + 'a> }
impl<'a, T> Strategy<'a, T> {
    pub fn new<S>(strategy: S) -> Self where S: StrategyWithContext<'a, T> + 'a,
    {
        Self { strategy: Box::new(strategy) }
    }
}
impl<'a, T> StrategyWithContext<'a, T> for Strategy<'a, T> {
    fn execute(&self, target: &'a T) -> bool { self.strategy.execute(target) }
}

pub struct StrategyFn<'a, T> { f: Box<dyn Fn(&T, &()) -> bool + 'a> }

impl<'a, T> StrategyFn<'a, T> {
    pub fn new<F>(f: F) -> Self where F: Fn(&T, &()) -> bool + 'a,
    {
        Self { f: Box::new(f) }
    }
}
impl<'a, T> StrategyFn<'a, T> {
    pub fn call(&self, target: &T, params: &()) -> bool {
        (self.f)(target, params)
    }
}

impl<'a, T> StrategyWithContext<'a, T> for StrategyFn<'a, T> {
    fn execute(&self, target: &'a T) -> bool { self.call(target, &()) }
}




use crate::inprogenitance::{RcLike, List, Node};

// Type Erased Strategy and Strategy List //


pub type DynStrategy<'a, T> = Strategy<'a, T>;
pub type DynStrategyList<'a, T, C> = List<'a, DynStrategy<'a, T>, T, C>;
pub fn dyn_strategy<'a, T, S>(strategy: S) -> DynStrategy<'a, T> where S: StrategyWithContext<'a, T> + 'a,
{
    DynStrategy::new(strategy)
}
