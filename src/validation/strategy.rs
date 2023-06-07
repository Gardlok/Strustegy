

 // In order to be able to support multiple types, we'll need to define specific validation
// strategies for each type. The types of strategies we'll need are:
//
// - ValidationStrategy: A trait that defines the interface for validation strategies to be used by
//   the Validator. The Validator will call the is_valid method on each strategy to determine whether
//   the input is valid or not. This is our kingpin trait that all of the other traits will extend.
// - StaticValidationStrategy: A strategy that can be used to validate a single input. This is
//   useful for strategies that don't need to store state between validations.
// - DynamicValidationStrategy: A strategy that can be used to validate a single input. This is
//   useful for strategies that need to store state between validations.
// - ComboValidationStrategy: A strategy that combines multiple static and dynamic strategies to
//   be used when validating a single input. The static strategies are executed first, followed by
//   the dynamic strategies. If any of the static strategies fail, the input is considered invalid
//   and the dynamic strategies are not executed. If any of the dynamic strategies fail, the input
//   is considered invalid. If all of the static and dynamic strategies pass, the input is considered
//   valid.
// - WithContextStrategy: A trait that defines the interface for validation strategies that need to
//   store state between validations. The Validator will call the is_valid_with_context method on
//   each strategy to determine whether the input is valid or not. This trait extends the 
//   ValidationStrategy trait. 
// - TimeValidationStrategy: A strategy that can be used to validate a single input. This is useful
//   for strategies that need to validate a time. It is equipt with a time provider that can be
//   used to get the current time. This includes date and time. 
// - RegexValidationStrategy: A strategy that can be used to validate a single input. This is useful
//   for strategies that need to validate a string against a regular expression. 
// - LogicalValidationStrategy: A strategy that can be used to validate a single input. Useful for
//   strategies that need to validate a boolean value. Combined with the ComboValidationStrategy,
//   this can be used to create complex validation logic. 
// - IterValidationStrategy: A strategy that can be used to validate an iterator of inputs. Useful
//   for strategies that need to validate a list of inputs. Combined with the ComboValidationStrategy,
//   this can be used to create complex validation logic. 
// - MapValidationStrategy: A strategy that can be used to validate a single input. Useful for
//   strategies that need to validate a map of inputs. Great for complex validation logic.
// - IntoIterValidationStrategy: A strategy that takes one strategy and converts it into an iterator
//   over the input. Useful for strategies that need to validate a list of inputs. Combined with Rayon
//   and the IterValidationStrategy, this can be used to create complex validation logic that can be
//   executed in parallel. 





use std::any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::any::Any;
use std::any::TypeId;

use std::hash::{Hash, Hasher};
use std::os::unix::prelude::FileTypeExt;
use std::sync::Arc;

use dashmap::DashMap as HashMap;

use crate::validation::error::ValidationError;
use crate::validation::error::AnyValidationError;
use crate::validation::error::MultipleValidationError;

// use crate::validation::Validation;


use crate::validator::Validator;




// A trait that defines the interface for validation strategies to be used by the Validator. The
// Validator will call the is_valid method on each strategy to determine whether the input is valid
// or not. This is our kingpin trait that all of the other traits will extend. The StrategyMap will
// use this trait to store the strategies and define any child or chained strategies that might
// exist. 

pub struct Strategy<T, F: ?Sized> {
    f: Box<F>,
    _phantom: PhantomData<T>,
}

impl<T: 'static + Sync + Send, F> ValidationStrategy<T> for Strategy<T, F>
where
F: for<'a> Fn(&'a T) -> bool + 'static + Send + Sync,
{
    fn is_valid(&self, input: &T) -> bool {
        (self.f)(input)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait ValidationStrategy<T: 'static>: Any + Send + Sync {
    fn is_valid(&self, input: &T) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static + Send + Sync> dyn ValidationStrategy<T> {
    pub fn new<F>(f: F) -> Box<dyn ValidationStrategy<T>>
    where
        F: for<'a> Fn(&'a T) -> bool + 'static + Send + Sync,
    {
        Box::new(Strategy {
            f: Box::new(f),
            _phantom: PhantomData,
        })
    }

}


pub struct StrategyMap<T> {
    pub hash_map: HashMap<TypeId, Box<dyn ValidationStrategy<T> + 'static>>,
    
}

impl<T: 'static> StrategyMap<T> {
    pub fn new() -> Self {
        Self {
            hash_map: HashMap::new(),
        }
    }

    pub fn insert_strategy(&mut self, strategy: Box<dyn ValidationStrategy<T> + 'static>) {
        let strategy_type_id = strategy.as_any().type_id();
        self.hash_map.insert(strategy_type_id, strategy);
    }

    pub fn remove_strategy(&mut self, strategy: &dyn Any) {
        let strategy_type_id = strategy.type_id();
        self.hash_map.remove(&strategy_type_id);
    }

    pub fn contains_key(&self, strategy: &dyn Any) -> bool {
        let strategy_type_id = strategy.type_id();
        self.hash_map.contains_key(&strategy_type_id)
    }

    pub fn get_strategy_type_id(&self, strategy: &dyn Any) -> Option<TypeId> {
        let strategy_type_id = strategy.type_id();
        Some(strategy_type_id)
    }

    pub fn validate(&self, input: &T) -> bool {
        let mut is_valid = true;
    
        for entry in self.hash_map.iter() {
            let (_, strategy) = entry.pair();
            is_valid = is_valid && strategy.is_valid(input);
        }
    
        is_valid
    }


}