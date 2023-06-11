

use crate::validation::error::ValidationError;


use std::any::{Any, TypeId};
use std::error::Error;
use std::marker::PhantomData;




pub trait Proof {
    fn validate(&mut self, f: &mut dyn FnMut(&mut dyn Any) -> bool) -> bool;
}

