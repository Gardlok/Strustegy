

use std::marker::PhantomData;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use arraydeque::ArrayDeque;

/////////
// An example of a generic function that takes a closure.
trait SortStrategy { fn sort(&self, data: &mut [i32]); }
struct ClosureSortStrategy<F: Fn(&mut [i32])>(F);
impl<F: Fn(&mut [i32])> SortStrategy for ClosureSortStrategy<F> {
    fn sort(&self, data: &mut [i32]) { (self.0)(data) }
}
fn sort_data<S: SortStrategy>(strategy: &S, data: &mut [i32]) { strategy.sort(data); }


struct StrategyStage<'a> { buffer: ArrayDeque<Box<dyn Fn(i32) -> i32 + 'a>, 3>, }
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
impl<'a, T, S, F> StrategyWithContext<'a, T> for MapStrategy<'a, T, S, F>
where
    S: StrategyWithContext<'a, T>, 
    F: Fn(&'a T) -> S,
{
    fn execute(&self, target: &'a T) -> bool { (self.f)(self.target).execute(target) }
}
//
impl<'a, T, S> StrategyWithContext<'a, T> for ConditionalStrategy<'a, T, S>
where
    S: StrategyWithContext<'a, T>, 
{
    fn execute(&self, target: &'a T) -> bool {
        if self.condition.strategy.execute(target) {
            self.true_strategy.execute(target)
        } else {
            self.false_strategy.execute(target)
        }
    }
}

// Dynamic Dispatched Strategy
pub(crate) struct StrategyFn<'a, T> { f: Box<dyn Fn(&T, &()) -> bool + 'a> }
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


// ---------------

// struct Stage<'a, T> {
//     buffer: ArrayDeque<Box<dyn Fn(i32) -> i32 + 'a>, const SIZE: usize>,
// }

// impl<'a> Stage<'a> {
//     fn lend_func(&mut self, f: Box<dyn Fn(i32) -> i32 + 'a>) {
//         self.buffer.push_back(f);
//     }

//     fn call(&self, arg: i32) -> i32 {
//         // Call the function in the buffer with the given argument
//         // This is just a placeholder, you'll need to add error handling and decide what to do when the buffer is empty
//         (self.buffer[0])(arg)
//     }
// }
// In this example, SIZE is the size of the ArrayDeque. You would replace this with the actual size you want for your buffer.
// Please note that this is a simplified example and you would need to add error handling and decide what to do when the buffer is empty. Also, keep in mind that the function pointers are stored in a Box to allow them to be stored in the ArrayDeque. This incurs a small heap allocation cost, but is necessary because the size of the function pointers is not known at compile time.
// This approach allows you to reuse the ArrayDeque and avoid unnecessary memory allocations. However, it also means that you need to manage the lifetimes of the function pointers and target values carefully to ensure that they live at least as long as the ArrayDeque.











// Mapping 
//
pub struct MapStrategy<'a, T, S, F> where S: StrategyWithContext<'a, T>, 
{
    strategy: S,
    f: F,
    target: &'a T,
}
impl<'a, T, S, F> MapStrategy<'a, T, S, F> where S: StrategyWithContext<'a, T>,
{
    pub fn new(strategy: S, f: F, target: &'a T) -> Self {
        Self {
            strategy,
            f,
            target,
        }
    }
}
// Conditionally dispatches strategy based on the result of the condition (bool)
pub struct ConditionalStrategy<'a, T, S> where S: StrategyWithContext<'a, T> 
{ 
    // condition determines which strategy to execute via a boolean
    // and choose between true_strategy and false_strategy
    condition: MapStrategy<'a, T, S, Box<dyn StrategyWithContext<'a, T>>>,

    // true_strategy and false_strategy are executed based on the condition
    // Generic S is used to allow different strategies to be used
    true_strategy: S,
    false_strategy: S,

    target: &'a T,
}
impl<'a, T, S> ConditionalStrategy<'a, T, S> where S: StrategyWithContext<'a, T>
{
    pub fn new(
        condition: MapStrategy<'a, T, S, Box<dyn StrategyWithContext<'a, T>>>,
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
pub fn conditional_strategy<'a, T, S>(cond_strategy: S, true_strategy: S, false_strategy: S, target: &'a T) -> ConditionalStrategy<'a, T, S>
where
    S: StrategyWithContext<'a, T> 
{
    ConditionalStrategy::new(
        MapStrategy::new(cond_strategy, Box::new(|_| true), target),
        true_strategy,
        false_strategy,
        target,
    )
}





// 
pub struct CompositeStrategy<'a, T, S, const N: usize> where S: StrategyWithContext<'a, T>
{
    strategies: ArrayDeque<S, N>,
    _phantom: PhantomData<&'a T>,

}
impl<'a, T, S, const N: usize> CompositeStrategy<'a, T, S, N> where S: StrategyWithContext<'a, T>
{
    pub fn new(strategies: ArrayDeque<S, N>) -> Self {
        Self { strategies, _phantom: PhantomData }
    }
}

// impl<'a, T, S> CompositeStrategy<'a, T, S> where S: StrategyWithContext<'a, T> + Clone, T: Clone
// {
//     pub fn add_progeny(&mut self, progeny: Progeny<'a, T, S>) {
//         self.progenies.push(progeny);
//     }
//     pub fn execute(&self, target: &'a T) -> bool {
//         self.strategies.iter().all(|strategy: &S| strategy.execute(target))
//     }
// }
// impl<'a, T, S> CompositeStrategy<'a, T, S> where S: StrategyWithContext<'a, T> + Clone, T: Clone
// {
//     pub fn progenies(&self) -> &Vec<Progeny<'a, T, S>> {
//         &self.progenies
//     }
// }
// impl<'a, T, S> StrategyWithContext<'a, T> for CompositeStrategy<'a, T, S>
// where
//     S: StrategyWithContext<'a, T> + Clone, T: Clone
// {
//     fn execute(&self, target: &'a T) -> bool {
//         self.strategies.iter().all(|strategy| strategy.execute(target))
//     }
// }










pub trait StrategyMap<'a, T, S>  where S: StrategyWithContext<'a, T>
{
    fn map(&self, target: &'a T) -> S;
}
impl<'a, T, S, M> StrategyMap<'a, T, S> for &'a M
where
    T: 'a,
    S: StrategyWithContext<'a, T>, 
    M: Fn(&'a T) -> S + 'a,
{
    fn map(&self, target: &'a T) -> S { (self)(target) }
}
impl<'a, T, S, M> StrategyMap<'a, T, S> for Box<M>
where
    T: 'a,
    S: StrategyWithContext<'a, T>,
    M: Fn(&'a T) -> S + 'a,
{
    fn map(&self, target: &'a T) -> S { (self)(target) }
}










// Strategy constructs
//
// 
pub fn strategy<'a, T, S>(strategy: S, target: &'a T) -> MapStrategy<'a, T, S, Box<dyn Fn(&'a T) -> bool>>
where
    S: StrategyWithContext<'a, T> + 'a
{
    MapStrategy::new(strategy, Box::new(|_| true), target)
}
// 

// 
// pub fn strategy_map<'a, T, S>(strategy: S) -> impl StrategyMap<'a, T, S>
// where
//     T: Clone + 'a,
//     S: StrategyWithContext<'a, T> + Clone + Eq + Hash + 'a
// {
//     Box::new(move |target| strategy ) // todo: clone is not good
// }
//
pub fn strategy_map_fn<'a, T, S, M>(strategy: S, f: M) -> impl StrategyMap<'a, T, S>
where
    T: 'a,
    S: StrategyWithContext<'a, T>  + 'a,
    M: Fn(&'a T) -> S + 'a
{
    Box::new(move |target| f(target))
}

 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy() {
        let target = &1;
        let strategy = strategy(|target: &i32| *target == 1, target);
        assert!(strategy.strategy.execute(target));
    }


    #[test]
    fn test_pattern() {
// ---------------
    fn generic_closure<F: Fn(i32)>(f: F) {
        f(0);
        f(1);
    }

    generic_closure(|x| println!("{}", x)); // A
    generic_closure(|x| { // B
        let y = x + 2;
        println!("{}", y);
    });


    fn closure_object(f: &dyn Fn(i32)) {
        f(0);
        f(1);
    }

    closure_object(&|x| println!("{}", x));
    closure_object(&|x| {
        let y = x + 2;
        println!("{}", y);
    });
    }







}

