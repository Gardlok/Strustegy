


use std::any::{Any, TypeId, type_name};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::error::Error;
use std::hash::Hash;
use std::marker::PhantomData;

mod inprogenitance;
mod iterating;
mod strategy;

use arraydeque::ArrayDeque;
pub use iterating::*;
use syn::token::In;

use crate::iterating::{Map, CExtrator};

use crate::strategy::{StrategyWithContext, StrategyFn, CompositeStrategy, ConditionalStrategy, StrategyMap};

mod test;

use std::rc::Rc;
use std::sync::Arc;
use std::ops::Deref;








































// Validaty tracks the validity of a strategy to a target.
// 
#[derive(Clone)]
pub enum Validaty<'a, T> {
    Valid(f32),        // ratio of valid progenies to total progenies
    Unknown(&'a [T]),  // Not yet determined and initial stage for target
    Error(OpError),    // Error in determining validity
}

// Applicative Trait
//
pub trait Applicative<'a, T, S> where S: StrategyWithContext<'a, T> + Clone, T: Clone,
{
    type Validaty: 'a;
    type Strategies: 'a;
    type Output: 'a;

    fn valid(&self, target: &'a T) -> Validaty<Self::Validaty>;
    fn strategies(&self, target: &'a T) -> Self::Strategies;
    fn apply(&self, strategy_map: &dyn StrategyMap<'a, T, S>, target: &'a T) -> Self::Output;
}
impl<'a, T: Clone + 'a, S: StrategyWithContext<'a, T> + Clone + 'a> Applicative<'a, T, S> for S {
    type Validaty = Validaty<'a, T>;
    type Strategies = Vec<S>;
    type Output = Result<Vec<T>, OpError>;

    fn valid(&self, target: &'a T) -> Validaty<'_, Validaty<'a, T>> {

        todo!("Implement Validaty for StrategyWithContext");

        Validaty::Valid(0.0)
    }

    fn strategies(&self, target: &'a T) -> Self::Strategies {
        vec![self.clone()]
    }
    fn apply(&self, strategy_map: &dyn StrategyMap<'a, T, S>, target: &'a T) -> Self::Output {
        
        todo!("Implement apply for StrategyWithContext");

        Ok(vec![])


    }
}


// Op Error 
//
#[derive(Debug, Clone)]
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




