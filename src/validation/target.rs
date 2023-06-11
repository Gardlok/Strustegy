
use crate::validation::error::ValidationError;
use crate::validation::validity::Validity;

use std::any::{Any, TypeId};



pub trait Target<'a> {
    type Value<T>: Validity where Self: 'a; 
    type Error;

    fn value<T>(&'a self) -> Self::Value<T>;
}

pub struct TargetContext<T: for<'a> Target<'a>> {
    pub target: T,
    pub type_id: TypeId,
    pub priority: u32,
    pub omitted: bool,
}

impl<'a, T: for<'b> Target<'b> + 'static> TargetContext<T> {
    pub fn new(target: T) -> Self {
        TargetContext {
            target,
            type_id: TypeId::of::<& T>(),
            priority: 0,
            omitted: false,
        }
    }
}

impl<'a, T: for<'b> Target<'b> + 'static> Target<'a> for TargetContext<T> {
    type Value<V> = <T as Target<'a>>::Value<V>;
    type Error = ValidationError;

    fn value<V>(&'a self) -> Self::Value<V> {
        self.target.value()
    }
}



