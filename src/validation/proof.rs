

use crate::validation::error::ValidationError;


use std::any::{Any, TypeId};
use std::error::Error;
use std::marker::PhantomData;

use crate::validation::validity::Validity;
use crate::validation::target::Target;
use crate::validation::strategy::Strategy;
use crate::validation::proof::Proof;




// pub trait Proof {
//     fn validate(&mut self, f: &mut dyn FnMut(&mut Self) -> bool) -> bool;
// }

pub trait Proof<'a, T> {
    type Strategy: Strategy<T>;
    fn validate(&'a self, strategy: &Self::Strategy, target: &T) -> bool;
}
