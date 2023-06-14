

use std::marker::PhantomData;
use std::any::Any;
use std::any::TypeId;
use dashmap::DashMap as HashMap;
use dashmap::DashSet as HashSet;
use crossbeam::atomic::AtomicCell; 
use crossbeam_skiplist::SkipMap as TreeMap;

use crate::validation::error::ValidationError;
// use crate::validation::validity::Validity;

use std::error::Error;

/// This is a trait that defines a specific way to validate a target. It is
/// abstract and reusable, and can be implemented for any type that can be 
/// validated.
pub trait Strategy<T>: Fn(&T) -> bool {}

impl<T, F> Strategy<T> for F where F: Fn(&T) -> bool {}

/// StrategyPlan is a struct that wraps a Strategy and allows it to be used as a function.
/// This provides flexibility and allows for higher-order functions and closures to be used as strategies.
pub struct StrategyPlan<T: for<'a> Target<'a>, F: Fn(&T) -> bool> {
    pub strategy_fn: F,
    pub _phantom: PhantomData<T>,
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

/// Scope: This is a trait that defines a scope for validation. A Scope is 
/// associated with a Proof and a Target, and has a proof method that returns
/// a reference to the proof, and a validate method that validates a target
/// using the proof.
pub trait Scope<'a, T> {
    type Proof: for<'s> Proof<'s, T>;
    fn proof<'s>(&'s self) -> &'s Self::Proof;
    fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &T) -> bool;
}

pub trait Target<'a> {
    type Value: 'a;
    fn value(&'a self) -> Self::Value;
}

pub struct TargetContext<T: for<'a> Target<'a>> {
    pub target: T,
    pub type_id: TypeId,
    pub priority: u32,
    pub omitted: bool,
}

/// Proof is a trait that defines a proof of validity for a target.
/// A Proof is associated with a Strategy and a Target, and has a validate
/// method that applies the strategy to the target and returns a boolean 
/// indicating whether the target is valid.
pub trait Proof<'a, T> {
    type Strategy: for<'s> Strategy<T>;  
                                        
    fn validate(&'a self, strategy: &Self::Strategy, target: &T) -> bool;
}

pub trait Validator<'a, T> {
    type Scope: for<'s> Scope<'s, T>;
    fn validate(&'a self, scope: &Self::Scope, target: &T) -> bool;
}







// use std::marker::PhantomData;
// use std::any::Any;
// use std::any::TypeId;
// use dashmap::DashMap as HashMap;
// use dashmap::DashSet as HashSet;
// use crossbeam::atomic::AtomicCell; 
// use crossbeam_skiplist::SkipMap as TreeMap;

// use crate::validation::error::ValidationError;
// use crate::validation::validity::Validity;



// use std::error::Error;



// // These are the traits that are used to define the validation logic.





// //
// pub trait Strategy<T> {
//     fn apply(&self, target: &T) -> bool;
// }

// pub trait Functor {
//     type Inner;
//     type Output<'a, 'c>: Functor;

//     fn map<'a, 'c, F, G, B>(self, f: F, g: G) -> Self::Output<'a, 'c>
//     where
//         F: FnOnce(Self::Inner) -> B,
//         G: FnOnce(B) -> Self::Output<'a, 'c>;
// }

// //
// pub trait PartFunctor {
//     type Inner;
//     type Output<'a, 'c>: PartFunctor;

//     fn map<'a, 'c, F, B>(self, f: F) -> Self::Output<'a, 'c>
//     where
//         F: FnOnce(Self::Inner) -> B;
// }



// //
// pub trait Scope<'a, T> {
//     type Proof: for<'s> Proof<'s, T>;
//     fn proof<'s>(&'s self) -> &'s Self::Proof;
//     fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &T) -> bool;
// }

// //
// pub trait Target<'a> {
//     type Value: 'a;
//     fn value(&'a self) -> Self::Value;
// }
// pub struct TargetContext<T: for<'a> Target<'a>> {
//     pub target: T,
//     pub type_id: TypeId,
//     pub priority: u32,
//     pub omitted: bool,
// }


// // 
// pub trait Proof<'a, T> {
//     type Strategy: for<'s> Strategy<T>;  // 
                                        
//     fn validate(&'a self, strategy: &Self::Strategy, target: &T) -> bool;
// }

// //
// pub trait Validator<'a, T> {
//     type Scope: for<'s> Scope<'s, T>;
//     fn validate(&'a self, scope: &Self::Scope, target: &T) -> bool;
// }



