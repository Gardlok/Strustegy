

use std::marker::PhantomData;
use std::any::Any;
use std::any::TypeId;
use dashmap::DashMap as HashMap;
use dashmap::DashSet as HashSet;
use crossbeam::atomic::AtomicCell; 
use crossbeam_skiplist::SkipMap as TreeMap;

use crate::validation::error::ValidationError;
use crate::validation::validity::Validity;
use crate::validation::validity::*;


use std::error::Error;


// Template //
// DESC: Template for a new component in higher-Fn-like validation system
// LIFE&OWNS: ([**Set of types**], [**Set of types**], [**Set of types**])
// IO: (**thing**) -> **thing**
// INNARDS:
// (**thing**)
// fn **method**
// ph **phantom constraint**
// NB&SB: [**Set of types**] -> [**Set of types**]
//
// ** Body of Code **
//
// End Template //




/// This is a trait that defines a specific way to validate a target. It is
/// abstract and reusable, and can be implemented for any type that can be 
/// validated.
pub trait StrategyFn<T>: Fn(&T) -> bool {}
impl<T, F> StrategyFn<T> for F where F: Fn(&T) -> bool {}

// Support for Strategy as a StrategyFn (for use in StrategyPlan) 
// impl<T: for<'a> Target<'a>, F: Fn(&T) -> bool + 'static> StrategyFn<T> for Strategy<T, F> {}


/// StrategyPlan is a struct that wraps a Strategy and allows it to be used as a function.
/// This provides flexibility and allows for higher-order functions and closures to be used as strategies.
pub struct Strategy<T: for<'a> Target<'a>, F: Fn(&T) -> bool> {
    pub _fn: Box<dyn StrategyFn<T>>,
    pub _ph: PhantomData<(F, T)>,
}

// Basic support for Strategy as a function
impl<T: for<'a> Target<'a>, F: Fn(&T) -> bool + 'static> Strategy<T, F> {
    pub fn new(strategy_fn: F) -> Self {
        Self {
            _fn: Box::new(strategy_fn),
            _ph: PhantomData,
        }
    }

    pub fn apply(&self, target: &T) -> bool {
        (self._fn)(target)
    }
}



// TargetContext is a struct that wraps a Target and allows it to be used as a function.
// This provides flexibility and allows for higher-order functions and closures to be used as targets.
// The type constraints are moved to the where clause, separating them from the generic type parameters. This approach simplifies the syntax and makes 
// the code more readable. It allows you to specify the required traits and associated types for the generic types T, S, and F without directly 
// constraining the types themselves.
pub struct TargetContext<'a, T, S, F>
where
    T: Target<'a>,
    S: Scope<'a, T::Value>,
    F: StrategyFn<T::Value>,
{
    pub target: &'a T,
    pub scope: &'a S,
    pub strategy: &'a F,
    pub _phantom: PhantomData<&'a T>,
}



/// This trait allows for higher-order functions and closures to be used as strategies. It is used in the 
/// implementation of StrategyPlan and Validity.
pub trait Functor {
    type Inner;
    type Output<'a, 'c>: Functor;

    fn map<'a, 'c, F, G, B>(self, f: F, g: G) -> Self::Output<'a, 'c>
    where
        F: FnOnce(Self::Inner) -> B,
        G: FnOnce(B) -> Self::Output<'a, 'c>;
}
/// This trait allows for partial application of functions. It is used in the implementation of Functor.
pub trait PartFunctor {
    type Inner;
    type Output<'a, 'c>: PartFunctor;

    fn map<'a, 'c, F, B>(self, f: F) -> Self::Output<'a, 'c>
    where
        F: FnOnce(Self::Inner) -> B;
}

// Default support for Functor as a PartFunctor
impl<T> Functor for T where T: PartFunctor {
    type Inner = T::Inner;
    type Output<'a, 'c> = T::Output<'a, 'c>;

    fn map<'a, 'c, F, G, B>(self, f: F, g: G) -> Self::Output<'a, 'c>
    where
        F: FnOnce(Self::Inner) -> B,
        G: FnOnce(B) -> Self::Output<'a, 'c>,
    {
        todo!("Implement Functor for PartFunctor")
    }
}


/// Scope: This is a trait that defines a scope for validation. A Scope is 
/// associated with a Proof and a Target, and has a proof method that returns
/// a reference to the proof, and a validate method that validates a target
/// using the proof.
pub trait Scope<'a, T> {
    type Proof: for<'s> Proof<'s, T>;

    fn proof<'s>(&'s self) -> &'s Self::Proof;
    fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &T) -> bool;
}

/// Proof is a trait that defines a proof of validity for a target.
/// A Proof is associated with a Strategy and a Target, and has a validate
/// method that applies the strategy to the target and returns a boolean 
/// indicating whether the target is valid.
pub trait Proof<'a, T> {
    type StrategyFn: for<'s> StrategyFn<T>;  
                                        
    fn validate(&'a self, strategy: &'a Self::StrategyFn, target: &T) -> bool;
}


///// Defaults /////
//
//
// Default support for Proof as a StrategyFn
impl<'a, T> Scope<'a, T> for &'a T
where
    T: Scope<'a, T>,
{
    type Proof = T::Proof;

    fn proof<'s>(&'s self) -> &'s Self::Proof {
        (**self).proof()
    }
    fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &T) -> bool {
        (**self).validate(proof, target)
    }
}

// Default support for Scope as a Proof
impl<'a, T> Proof<'a, T> for &'a T
where
    T: Proof<'a, T>,
{
    type StrategyFn = T::StrategyFn;

    fn validate(&'a self, strategy: &'a Self::StrategyFn, target: &T) -> bool {
        (**self).validate(strategy, target)
    }
}



//////////////////////////////////////////
// Help the compiler out with the lifetimes. This is a workaround for the fact that the compiler can't infer the lifetimes of the generic types.
impl<'a, T: for<'s> Target<'s>, S: for<'s> Scope<'s, <T as Target<'a>>::Value>, F: for<'s> StrategyFn<<T as Target<'a>>::Value>> TargetContext<'a, T, S, F> {
    pub fn new(target: &'a T, scope: &'a S, strategy: &'a F) -> Self {
        Self {
            target,
            scope,
            strategy,
            _phantom: PhantomData,
        }
    }
}






////////////////////////////////////////// GATs and Fn's 
// 
//
// EACH OF THESE FUNCTIONS IS A HIGHER-ORDER FUNCTION THAT CAN BE USED AS A STRATEGY.
//
// EACH FUNCTION REQUIRES A TEST CASE TO BE PROVIDED AS AN ARGUMENT.

//////////////////////////////////////////
//
// This function takes a TargetContext and returns a closure that takes a value and returns a boolean.
// The closure is a higher-order function that can be used as a strategy.
// The closure is generic over the lifetime of the TargetContext, and the lifetime of the value.
// It serves as a proof of validity for the target. 

// Lifetime:   ['a, 'targ, 'scope, 'proo, 'valu]
// Ownership:  [self, target, scope, proof, value]
// Concrete:   [Self::Value, Self::Proof]
// Generic:    [Self::Target, Self::Scope, Self::StrategyFn]
// Associated: [Self::Value, Self::Proof, Self::StrategyFn]
// Trait:      [Target, Scope, StrategyFn]
// 
//
pub fn strategy< 'a, 

    T: for<'s> Target<'s>, 
    S: for<'s> Scope<'s, <T as Target<'a>>::Value>, 
    F: for<'s> StrategyFn<<T as Target<'a>>::Value>
    >(
    target_context: &'a TargetContext<'a, T, S, F>,  
    ) -> impl for<'s> Fn(&'s <T as Target<'a>>::Value
    ) -> bool + 'a 
    {
    move |value| {
    
        let proof = target_context.scope.proof();
    
        target_context.scope.validate(proof, value)
    }
}

////////////////////////////////////////// Validate
//
// A function helping a validator to validate a target
pub fn validate<'a, 
    T: for<'s> Target<'s>, 
    S: for<'s> Scope<'s, <T as Target<'a>>::Value>, 
    F: for<'s> StrategyFn<<T as Target<'a>>::Value>
    >(
    target_context: &'a TargetContext<'a, T, S, F>,
    value: &'a <T as Target<'a>>::Value,
    ) -> bool {
    let proof = target_context.scope.proof();

    target_context.scope.validate(proof, value)
}

////////////////////////////////////////// Validator Group
//
// A function to group targets and run them through a validator
pub fn validator_group<'a, 
    T: for<'s> Target<'s>, 
    S: for<'s> Scope<'s, <T as Target<'a>>::Value>, 
    F: for<'s> StrategyFn<<T as Target<'a>>::Value>
    >(
    target_context: &'a TargetContext<'a, T, S, F>,
    values: &'a Vec<<T as Target<'a>>::Value>,
    ) -> bool {
    values.iter().all(|value| validate(target_context, value))
}

////////////////////////////////////////// Validator
//
// A function generating a validator from a target
// pub fn validator<'a, 
//     T: for<'s> Target<'s>, 
//     S: for<'s> Scope<'s, <T as Target<'a>>::Value>, 
//     F: for<'s> StrategyFn<<T as Target<'a>>::Value>
//     >(
//     target_context: &'a TargetContext<'a, T, S, F>,
//     ) -> impl for<'s> Fn(&'s Vec<<T as Target<'a>>::Value>) -> bool + 'a {
//     move | values| validator_group(target_context, values)
// }
////////////////////////////////////////// TargetContext
//
// A function generating a TargetContext from a target
pub fn target_context<'a, 
    T: for<'s> Target<'s>, 
    S: for<'s> Scope<'s, <T as Target<'a>>::Value>, 
    F: for<'s> StrategyFn<<T as Target<'a>>::Value>
    >(
    target: &'a T,
    scope: &'a S,
    strategy: &'a F,
    ) -> TargetContext<'a, T, S, F> {
    TargetContext {
        target,
        scope,
        strategy,
        _phantom: PhantomData,
    }
}


// Testing Functions //
#[cfg(test)]
mod test_functions {
    use super::*;
    use std::collections::HashMap;

    // Mock Target
    pub struct MockTarget<'a> {
        pub name: &'a str,
        pub index: &'a i32,
        pub container: &'a Vec<i32>,
        pub nested_container: &'a Vec<Vec<i32>>,
        pub flag_one: &'a bool,
        pub flag_two: &'a bool,
        pub flag_three: &'a bool,
    }

    // Mock Scope
    pub struct MockScope<'a> {
        pub name: &'a str,
        pub index: &'a i32,
        pub container: &'a Vec<i32>,
        pub nested_container: &'a Vec<Vec<i32>>,
        pub flag_one: &'a bool,
        pub flag_two: &'a bool,
        pub flag_three: &'a bool,
    }


    // Helper Functions
    pub fn mock_target<'a>(
        name: &'a str,
        index: &'a i32,
        container: &'a Vec<i32>,
        nested_container: &'a Vec<Vec<i32>>,
        flag_one: &'a bool,
        flag_two: &'a bool,
        flag_three: &'a bool,
    ) -> MockTarget<'a> {
        MockTarget {
            name,
            index,
            container,
            nested_container,
            flag_one,
            flag_two,
            flag_three,
        }
    }

    #[test]
    fn test_mock_target() {
        let name = "name";
        let index = &1;
        let container = &vec![1, 2, 3];
        let nested_container = &vec![vec![1, 2, 3], vec![1, 2, 3]];
        let flag_one = &true;
        let flag_two = &true;
        let flag_three = &true;
        let mock_target = mock_target(name, index, container, nested_container, flag_one, flag_two, flag_three);
        assert_eq!(mock_target.name, "name");
        assert_eq!(mock_target.index, &1);
        assert_eq!(mock_target.container, &vec![1, 2, 3]);
        assert_eq!(mock_target.nested_container, &vec![vec![1, 2, 3], vec![1, 2, 3]]);
        assert_eq!(mock_target.flag_one, &true);
        assert_eq!(mock_target.flag_two, &true);
        assert_eq!(mock_target.flag_three, &true);
    }


    // #[test]
    // fn test_strategy() {
    //     let target = &1;
    //     let scope = &1;
    //     let strategy = &|value| value == &1;
    //     let target_context = target_context(target, scope, strategy);
    //     let value = &1;

    //     let result = validate(&target_context, value);
    //     assert_eq!(result, true);
    // }

//     #[test]
//     fn test_validate() {
//         let target = &1;
//         let scope = &1;
//         let strategy = &|value| value == &1;
//         let target_context = target_context(target, scope, strategy);
//         let value = &1;
//         let result = validate(&target_context, value);
//         assert_eq!(result, true);
//     }

//     #[test]
//     fn test_validate_group() {
//         let target = &1;
//         let scope = &1;
//         let strategy = &|value| value == &1;
//         let target_context = target_context(target, scope, strategy);
//         let values = vec![&1, &1, &1];
//         let result = validate_group(&target_context, &values);
//         assert_eq!(result, vec![true, true, true]);
//     }
// }
}
