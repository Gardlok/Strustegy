


use std::marker::PhantomData;
use std::any::Any;
use std::any::TypeId;
use dashmap::DashMap as HashMap;
use dashmap::DashSet as HashSet;
use crossbeam::atomic::AtomicCell; 
use crossbeam_skiplist::SkipMap as TreeMap;

use crate::validation::error::ValidationError;
use crate::validation::validity::Validity;



use std::error::Error;



// These are the traits that are used to define the validation logic.





//
pub trait Strategy<T> {
    fn apply(&self, target: &T) -> bool;
}

pub trait Functor {
    type Inner;
    type Output<'a, 'c>: Functor;

    fn map<'a, 'c, F, G, B>(self, f: F, g: G) -> Self::Output<'a, 'c>
    where
        F: FnOnce(Self::Inner) -> B,
        G: FnOnce(B) -> Self::Output<'a, 'c>;
}

//
pub trait PartFunctor {
    type Inner;
    type Output<'a, 'c>: PartFunctor;

    fn map<'a, 'c, F, B>(self, f: F) -> Self::Output<'a, 'c>
    where
        F: FnOnce(Self::Inner) -> B;
}



//
pub trait Scope<'a, T> {
    type Proof: for<'s> Proof<'s, T>;
    fn proof<'s>(&'s self) -> &'s Self::Proof;
    fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &T) -> bool;
}

//
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


// 
pub trait Proof<'a, T> {
    type Strategy: for<'s> Strategy<T>;  // 
                                        
    fn validate(&'a self, strategy: &Self::Strategy, target: &T) -> bool;
}



//
pub trait Validator<'a, T> {
    type Scope: for<'s> Scope<'s, T>;
    fn validate(&'a self, scope: &Self::Scope, target: &T) -> bool;
}


// pub trait Validator<'a, T> {
//     type Scope<'s>: Scope<'s, T> where Self: 's, Self: 'a;  // Scoped to lifetimes of the validator and the target
//     type Strategy<'s>: Strategy<T> where Self: 's;          // Scoped to the lifetime of the validator
//     type Proof<'s>: Proof<'s, T> where Self: 's;            // Scoped to the lifetime of the validator

//     fn validate(&'a self, scope: &Self::Scope<'a>, target: &T) -> bool;
// }



