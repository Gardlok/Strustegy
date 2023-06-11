


use std::fmt::{Debug, Display};
use std::error::Error;
use crossbeam::atomic::AtomicCell;
use crossbeam_skiplist::SkipMap as TreeMap;

use std::any::TypeId;
use std::any::Any;

use crate::validation::error::ValidationError;
use crate::validation::validity::Validity;
use crate::validation::target::Target;
use crate::validation::proof::{Proof, GenericProof};
use crate::validation::validator::{Validator, HigherOrderValidator, GenericValidator};




pub trait Scope<'a, T> {
    type Proof: Proof<'a, T>;
    fn validate(&'a self, proof: &Self::Proof, target: &T) -> bool;
}

pub struct GenericScope<'a, T, P: Proof<'a, T>> {
    proof: P,
    _phantom: std::marker::PhantomData<&'a T>,
}

pub struct HigherOrderScope<'a, T, S: Scope<'a, T>> {
    pub scopes: Vec<S>,
    _phantom: std::marker::PhantomData<&'a T>,
}









// Target level context
//
pub struct TargetContext<T: 'static + Sync + Send + Clone> {
    pub type_id: TypeId,  // Any TypeId for lookups 
    pub priority: u32,    // For sorting and Validation logicistics
    pub omitted: bool,    // All purpose omit switch
    pub value: T,         // 
}





pub trait Color {
    fn name(&self) -> &'static str;
}

pub trait Paintable {
    type Color: Color;
}

pub trait ConditionalPaintable {
    type Color: Color;
    type Condition: Fn() -> bool;
}



//
pub struct Red {
    counter: u32,
}
impl Color for Red {
    fn name(&self) -> &'static str {
        "Red"
    }
}
impl Red {
    pub fn new() -> Self {
        Red { counter: 0 }
    }
    pub fn increment(&mut self) {
        self.counter += 1;
    }
    pub fn count(&self) -> u32 {
        self.counter
    }
}


//
pub struct Green {
    action: Box<dyn Fn()>,
}
impl Color for Green {
    fn name(&self) -> &'static str {
        "Green"
    }
}
impl Green {
    pub fn new(action: impl Fn() + 'static) -> Self {
        Green { action: Box::new(action) }
    }
    pub fn act(&self) {
        (self.action)();
    }
}

