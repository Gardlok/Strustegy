


use std::error::Error;

mod inprogenitance;
mod iterating;
mod strategy;
mod indexing;
mod listing;
mod new;

use crate::strategy::{StrategyWithContext, StrategyFn};

mod test;

#[derive(Clone)]
pub enum Validaty<'a, T> {
    Valid(f32),        // ratio of valid progenies to total progenies
    Unknown(&'a [T]),  // Not yet determined and initial stage for target
    Error(OpError),    // Error in determining validity
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
// 
pub fn error<T >(message: &str) -> Result<T, OpError> {   Err(OpError::new(message))
}



// Bind (Monadic Operation)
pub fn bind<T, U, F>( x: Result<T, OpError>,  y: Result<U, OpError>,  f: F ) -> Result<U, OpError> where F: FnOnce(T, U) -> Result<U, OpError> {
    match (x, y) { 
        (Ok(x), Ok(y)) => f(x, y), 
        (Err(e), _) => Err(e),
        (_, Err(e)) => Err(e),
    }
}








// test
//
#[cfg(test)]
mod goals {
    use super::*;
// Library Goals.
// fn test_bind() {
//     assert_eq!(bind(Ok(1), |x| Ok(x + 1)), Ok(2));
//     assert_eq!(bind(Ok(1), |x| error("error")), error("error"));
//     assert_eq!(bind(error("error"), |x| Ok(1)), error("error"));
// }
}



