

use std::marker::PhantomData;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use arraydeque::ArrayDeque;

use crate::Validaty;
use crate::OpError;





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
//
impl<'a, T, S> Applicative<'a, T, S> for S where S: StrategyWithContext<'a, T> + Clone + 'a, T: Clone + 'a,
{
    type Validaty = Validaty<'a, T>;
    type Strategies = HashSet<S>;
    type Output = Vec<Validaty<'a, T>>;

    fn valid(&self, target: &'a T) -> Validaty<Self::Validaty> {
        let valid = self.execute(target);
        if valid { Validaty::Valid(1.0) } else { Validaty::Valid(0.0) }
    }
    fn strategies(&self, target: &'a T) -> Self::Strategies {
        let mut strategies = HashSet::new();
        // strategies.insert(self.clone());
        strategies
    }
}











/////////
// An example of a generic function that takes a closure.
trait SortStrategy { fn sort(&self, data: &mut [i32]); }
struct ClosureSortStrategy<F: Fn(&mut [i32])>(F);
impl<F: Fn(&mut [i32])> SortStrategy for ClosureSortStrategy<F> {
    fn sort(&self, data: &mut [i32]) { (self.0)(data) }
}
fn sort_data<S: SortStrategy>(strategy: &S, data: &mut [i32]) { strategy.sort(data); }

/////





pub struct StrategyStage<'a> { buffer: ArrayDeque<Box<dyn Fn(i32) -> i32 + 'a>, 3>, } 
impl<'a> StrategyStage<'a> {
    fn lend_func(&mut self, f: Box<dyn Fn(i32) -> i32 + 'a>) {
        self.buffer.push_back(f);
    }

    fn call(&self, arg: i32) -> i32 {
        // Call the function in the buffer with the given argument
        // This is just a placeholder, you'll need to add error handling and decide what to do when the buffer is empty
        (self.buffer[0])(arg)
    }
}
///////////



// Core Strategy Component 
// 
pub trait StrategyWithContext<'a, T>
    { fn execute(&self, target: &'a T) -> bool; }
//
impl<'a, T: 'a, S> StrategyWithContext<'a, T> for S
where
    S: Fn(&'a T) -> bool
{
    fn execute(&self, target: &'a T) -> bool { self(target) }
}
//



// Dynamic Dispatched Strategy
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

// Static Dispatched Strategy
pub struct StandardStrategy<'a, T, F> where F: Fn(&'a T) -> bool,
{
    strategy: F,
    phantom: PhantomData<&'a T>,
}
impl<'a, T, F> StrategyWithContext<'a, T> for StandardStrategy<'a, T, F>
where
    F: Fn(&'a T) -> bool,
{
    fn execute(&self, target: &'a T) -> bool { (self.strategy)(target) }
}
impl<'a, T, F> StandardStrategy<'a, T, F> where F: Fn(&'a T) -> bool,
{ // Todo: add more methods to StandardStrategy supporting user defined strategies
    pub fn new(strategy: F) -> Self {
        Self {
            strategy,
            phantom: PhantomData,
        }
    }
}



// Conditionally dispatches strategy based on the result of the condition (bool)
pub struct ConditionalStrategy<'a, T, S> where S: StrategyWithContext<'a, T> 
{ 
    // condition determines which strategy to execute via a boolean
    // and choose between true_strategy and false_strategy
    condition: S,

    // true_strategy and false_strategy are executed based on the condition
    // Generic S is used to allow different strategies to be used
    true_strategy: S,
    false_strategy: S,

    target: &'a T,
}



