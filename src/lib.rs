


use std::error::Error;


mod inprogenitance;
mod iterating;
mod strategy;
mod indexing;
mod listing;
mod new;



use crate::strategy::{StrategyWithContext, StrategyFn};

mod test;





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




