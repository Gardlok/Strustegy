

use std::any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::any::Any;
use std::any::TypeId;

use std::hash::{Hash, Hasher};
use std::os::unix::prelude::FileTypeExt;


// use crate::ValidationError;
// use crate::AnyValidationError;
// use crate::MultipleValidationError;

use dashmap::DashSet as HashSet;
use dashmap::DashMap as HashMap;

mod validation;
use validation::*;

#[cfg(test)]
use tests;






// Macro to convert a function into a strategy
#[macro_export]
macro_rules! strategy_fn {
    ($name:ident, $closure:expr) => {
        
        pub struct $name<T> {
            strategy: std::sync::Arc<dyn Fn(&T) -> bool + Sync + Send>,
        }

        impl<T> $name<T> {
            pub fn new(strategy: impl Fn(&T) -> bool + Sync + Send + 'static) -> Self {
                $name {
                    strategy: std::sync::Arc::new(strategy),
                }
            }

            pub fn is_valid(&self, input: &T) -> bool {
                (self.strategy)(input)
            }
        }
    };
}






