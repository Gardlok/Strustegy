

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

use crate::strategies::*;








// A trait that defines the interface for validation strategies to be used by the Validator. The
// Validator will call the is_valid method on each strategy to determine whether the input is valid
// or not. This is our kingpin trait that all of the other traits will extend. The StrategyMap will
// use this trait to store the strategies and define any child or chained strategies that might
// exist. 

pub trait ValidationStrategy<T: 'static>: Any + Send + Sync {
    fn is_valid(&self, input: &T) -> bool;
    fn as_any(&self) -> &(dyn Any + 'static) where Self: 'static;
    fn eq_with_dyn(&self, other: &(dyn ValidationStrategy<T> + 'static)) -> bool;
    fn hash_with_dyn(&self, hasher: &mut (dyn Hasher + '_));

}

impl<T> dyn ValidationStrategy<T> + 'static {
    fn downcast_ref<D: 'static>(&self) -> Option<&D> {
        self.as_any().downcast_ref()
    }
}

impl<T> Hash for dyn ValidationStrategy<T> + 'static {
    fn hash<X: Hasher>(&self, hasher: &mut X) {
        self.hash_with_dyn(hasher as &mut (dyn Hasher + '_))
    }
}

impl<T> PartialEq<dyn ValidationStrategy<T> + 'static> for dyn ValidationStrategy<T> + 'static {
    fn eq(&self, other: &Self) -> bool {
        self.eq_with_dyn(other)
    }
}

impl<T> Eq for dyn ValidationStrategy<T> + 'static {}   



impl<T: 'static + Send + Sync> dyn ValidationStrategy<T> {
    // Creates a new ValidationStrategy from the given function. The function will be used to
    // validate the input. 
    pub fn new<F>(f: F) -> impl ValidationStrategy<T>
    where
        F: Fn(&T) -> bool + 'static + Send + Sync,
    {
        struct Strategy<T, F> {
            f: F,
            _phantom: PhantomData<T>,
        }
        impl<T: 'static + Sync + Send, F> ValidationStrategy<T> for Strategy<T, F>
        where
            F: Fn(&T) -> bool + 'static + Send + Sync,
        {
            fn is_valid(&self, input: &T) -> bool {
                (self.f)(input)
            }
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn eq_with_dyn(&self, other: &(dyn ValidationStrategy<T> + 'static)) -> bool {
                if let Some(x) = other.downcast_ref::<Strategy<T, F>>() {
                    &self.f as *const _ == &x.f as *const _
                } else {
                    false
                }
            }

            fn hash_with_dyn(&self, hasher: &mut (dyn Hasher + '_)) {
                todo!()
            }
        }
     
        impl<T, F> Debug for Strategy<T, F> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Strategy")
                    .field("f", &"Fn(&T) -> bool")
                    .finish()
            }
        }

        Strategy {
            f,
            _phantom: PhantomData,
        }
    }
}


pub struct StrategyMap<T> {
    hash_map: HashMap<Box<dyn ValidationStrategy<T> + 'static>, Vec<Box<dyn ValidationStrategy<T> + 'static>>>,
}

impl<T: 'static> StrategyMap<T> {
    pub fn new() -> Self {
        Self {
            hash_map: HashMap::new(),
        }
    }

    pub fn insert_strategy(&mut self, strategy: Box<dyn ValidationStrategy<T> + 'static>) {
        let strategy_type_id = strategy.type_id();
        let strategy_type = TypeId::of::<dyn ValidationStrategy<T>>();
        let concrete_type = TypeId::of::<Box<dyn ValidationStrategy<T>>>();

        if strategy_type_id == strategy_type || strategy_type_id == concrete_type {
            self.hash_map.insert(strategy, Vec::new());
        }
    }

    pub fn add_child_strategy(&mut self, parent: &dyn Any, child: Box<dyn ValidationStrategy<T> + 'static>) {
        let parent_type_id = parent.type_id();
        let parent_type = TypeId::of::<dyn ValidationStrategy<T>>();
        let parent_concrete_type = TypeId::of::<Box<dyn ValidationStrategy<T>>>();
    
        if parent_type_id == parent_type || parent_type_id == parent_concrete_type {
            let parent = parent.downcast_ref::<Box<dyn ValidationStrategy<T>>>().unwrap();
            if let Some(mut children) = self.hash_map.get_mut(parent) {
                children.push(child);
            }
        }
    }

    

    pub fn remove_strategy(&mut self, strategy: &dyn Any) {
        let strategy_type_id = strategy.type_id();
        let strategy_type = TypeId::of::<dyn ValidationStrategy<T>>();
        let concrete_type = TypeId::of::<Box<dyn ValidationStrategy<T>>>();
    
        if strategy_type_id == strategy_type || strategy_type_id == concrete_type {
            self.hash_map.remove(strategy.downcast_ref::<Box<dyn ValidationStrategy<T>>>().unwrap());
        }
    }

    pub fn remove_child_strategy(&mut self, parent: &dyn Any, child: &dyn Any) {
        let parent_type_id = parent.type_id();
        let parent_type = TypeId::of::<dyn ValidationStrategy<T>>();
        let parent_concrete_type = TypeId::of::<Box<dyn ValidationStrategy<T>>>();
    
        let child_type_id = child.type_id();
        let child_type = TypeId::of::<dyn ValidationStrategy<T>>();
        let child_concrete_type = TypeId::of::<Box<dyn ValidationStrategy<T>>>();
    
        if (parent_type_id == parent_type || parent_type_id == parent_concrete_type) && (child_type_id == child_type || child_type_id == child_concrete_type) {
            if let Some(mut children) = self.hash_map.get_mut(parent.downcast_ref::<Box<dyn ValidationStrategy<T>>>().unwrap()) {
                let child = child.downcast_ref::<Box<dyn ValidationStrategy<T>>>().unwrap();
                children.retain(|x| !std::ptr::eq(x.as_ref(), child.as_ref()));
            }
        }
    }

    pub fn contains_key(&self, strategy: &dyn Any) -> bool {
        let strategy_type_id = strategy.type_id();
        let strategy_type = TypeId::of::<dyn ValidationStrategy<T>>();
        let concrete_type = TypeId::of::<Box<dyn ValidationStrategy<T>>>();

        if strategy_type_id == strategy_type || strategy_type_id == concrete_type {
            self.hash_map.contains_key(strategy.downcast_ref::<Box<dyn ValidationStrategy<T>>>().unwrap())
        } else {
            false
        }
    }

    pub fn get(&self, strategy: &dyn Any) -> Option<&Vec<Box<dyn ValidationStrategy<T> + 'static>>> {
        let strategy_type_id = strategy.type_id();
        let strategy_type = TypeId::of::<dyn ValidationStrategy<T>>();
        let concrete_type = TypeId::of::<Box<dyn ValidationStrategy<T>>>();

        if strategy_type_id == strategy_type || strategy_type_id == concrete_type {
            self.hash_map.get(strategy.downcast_ref::<Box<dyn ValidationStrategy<T>>>().unwrap())
        } else {
            None
        }
    }
}