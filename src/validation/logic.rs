


use std::fmt::{Debug, Display};
use std::error::Error;
use crossbeam::atomic::AtomicCell;
use crossbeam_skiplist::SkipMap as TreeMap;
use dashmap::DashSet as HashSet;

use std::any::TypeId;
use std::any::Any;
use std::marker::PhantomData;

use crate::validation::error::ValidationError;
use crate::validation::validity::Validity;
// use crate::validation::target::Target;
use crate::validation::proof::Proof;
use crate::validation::validator::Validator;
use crate::validation::strategy::{Strategy, GenericStrategy, GenericProof, GenericValidator};




pub trait Target<'a> {
    type Value: 'a;
    fn value(&'a self) -> Self::Value;
}








// pub struct Operator {
//     pub name: &'static str,
//     pub symbol: &'static str,
//     pub arity: u8,
//     pub precedence: u8,
//     pub associativity: Associativity,
//     pub function: fn(&[bool]) -> bool,
// }

pub trait Scope<'a, T> {
    type Proof: for<'s> Proof<'s, T>;
    fn proof<'s>(&'s self) -> &'s Self::Proof;
    fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &T) -> bool;
}





// pub trait Operator {
//     type Operand<T>: IntoIterator<Item = T>;
// }



// trait Applicative: Target {
//     fn lift_a2<F, B, C>(self, b: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
//     where
//         F: FnMut(Self::Unwrapped, B) -> C;
// }

// impl<A> Applicative for Option<A> {
//     fn lift_a2<F, B, C>(self, b: Self::Wrapped<B>, mut f: F) -> Self::Wrapped<C>
//     where
//         F: FnMut(Self::Unwrapped, B) -> C
//     {
//         let a = self?;
//         let b = b?;
//         Some(f(a, b))
//     }
// }

// fn birth_year() -> Result<i32, String> {
//     Err("No birth year".to_string())
// }

// fn current_year() -> Result<i32, String> {
//     Err("No current year".to_string())
// }

// fn age() -> Result<i32, String> {
//     current_year().lift_a2(birth_year(), |cy, by| cy - by)
// }












pub trait CompositionOperator {
    type Collection<T>: IntoIterator<Item = T>;
}

pub struct ProofSet;
// impl CompositionOperator for ProofSet {
//     type Collection<T> = HashSet<T>;  // Or any other collection type
// }



// impl<'a, T, CF: CompositionOperator, CO> dyn Validator2<'a, T, CO> {  // Going dyn
//     fn validate_all(&self, target: &T) -> bool {          
                                                         
//         self.proofs.into_iter().all(|proof| proof.validate(target))
//     }

//     fn validate_any(&self, target: &T) -> bool {
//         self.proofs.into_iter().any(|proof| proof.validate(target))
//     }

//     // etc.
// }



















// Target level context
//
pub struct TargetContext<T: 'static + Sync + Send + Clone> {
    pub type_id: TypeId,  // Any TypeId for lookups 
    pub priority: u32,    // For sorting and Validation logicistics
    pub omitted: bool,    // All purpose omit switch
    pub value: T,         // 
}

