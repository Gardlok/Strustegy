


use std::error::Error;

mod inprogenitance;
mod iterating;
mod strategy;
mod indexing;
mod listing;
mod new;

use crate::strategy::{StrategyWithContext};

mod test;




// Validaty 
// Ah, the Validaty type. This is the type that is returned by the valid method of the Applicative trait.
// It is an enum with three variants:
// Valid: The strategy is valid for the target at x% confidence. (based on a strategy's result composition)
// Unknown: The strategy is not yet determined to be valid or invalid.
// Error: An error occurred while determining the validity of the strategy. (Todo: add error handling as a strategy/hlist)
//
#[derive(Clone)]
pub enum Validaty<'a, T> {
    Valid(f32),        // ratio of valid progenies to total progenies
    Unknown(&'a [T]),  // Not yet determined and initial stage for target
    Error(OpError),    // Error in determining validity
}




























// Op Error 
//
#[derive(Debug, Clone)]
#[derive(PartialEq)]
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
pub fn error<T>( msg: &str ) -> Result<T, OpError> { Err(OpError::new(msg)) }
//
#[test]
fn test_error() {
    assert_eq!(error::<i32>("error"), Err(OpError::new("error")));
}




// Bind (>>=) - this monad's bind operator can be used to chain operations
pub fn bind<T, U, F>( x: Result<T, OpError>,  f: F ) -> Result<U, OpError> where F: FnOnce(T) -> Result<U, OpError> {
    match x { 
        Ok(x) => f(x), 
        Err(e) => Err(e),
    }
}
//
#[macro_export]
macro_rules! bind {
    ( $x:expr, $f:expr ) => { bind($x, $f) };
    ( $x:expr, $f:expr, $($rest:expr),+ ) => { bind($x, |x| bind!( $f(x), $($rest),+ )) };
}
#[test]
fn test_bind() {
    assert_eq!(bind!(Ok(1), |x| Ok(x + 1)), Ok(2));
    assert_eq!(bind!(Ok(1), |x| error::<i32>("error")), error("error"));
    let x: Result<i32, OpError> = Ok(1);
    let y: Result<i32, OpError> = Ok(2);
    assert_eq!(bind!(x, |x| bind!(y, |y| Ok(x + y))), Ok(3));
    assert_eq!(bind!(error::<i32>("error"), |x| Ok(x + 1)), error("error"));

    assert_eq!(bind!(Ok(1), |x| Ok(x + 1), |x| Ok(x + 1)), Ok(3));
    assert_eq!(bind!(Ok(1), |x| Ok(x + 1), |x| error::<i32>("error")), error("error"));
    assert_eq!(bind!(Ok(1), |x| error::<i32>("error"), |x| Ok(x + 1)), error("error"));
}   

// BindExt (>>=) - this monad's bind operator can be used to chain operations
pub fn bindExt<T, U, F>( x: Result<T, OpError>,  f: F ) -> Result<U, OpError> where F: FnOnce(T) -> Result<U, OpError> {
    match x { 
        Ok(x) => f(x), 
        Err(e) => Err(e),
    }
}
//
#[macro_export]
macro_rules! bindExt {
    ( $x:expr, $f:expr ) => { bindExt($x, $f) };
    ( $x:expr, $f:expr, $($rest:expr),+ ) => { bindExt($x, |x| bindExt!( $f(x), $($rest),+ )) };
}
#[test]
fn test_bindExt() {
    assert_eq!(bindExt!(Ok(1), |x| Ok(x + 1)), Ok(2));
    assert_eq!(bindExt!(Ok(1), |x| error::<i32>("error")), error("error"));
    let x: Result<i32, OpError> = Ok(1);
    let y: Result<i32, OpError> = Ok(2);
    assert_eq!(bindExt!(x, |x| bindExt!(y, |y| Ok(x + y))), Ok(3));
    assert_eq!(bindExt!(error::<i32>("error"), |x| Ok(x + 1)), error("error"));

    assert_eq!(bindExt!(Ok(1), |x| Ok(x + 1), |x| Ok(x + 1)), Ok(3));
    assert_eq!(bindExt!(Ok(1), |x| Ok(x + 1), |x| error::<i32>("error")), error("error"));
    assert_eq!(bindExt!(Ok(1), |x| error::<i32>("error"), |x| Ok(x + 1)), error("error"));
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


mod prelude {
    pub use crate::inprogenitance::*;
    pub use crate::iterating::*;
    pub use crate::strategy::*;
    pub use crate::indexing::*;
    pub use crate::listing::*;
    pub use crate::new::*;
    pub use crate::Validaty;
    pub use crate::OpError;
    pub use crate::error;
    pub use crate::bind;
    pub use crate::bindExt;
}



