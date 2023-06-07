

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





use std::marker::PhantomData;
use std::any::Any;
use std::any::TypeId;

use dashmap::DashMap as HashMap;
use crossbeam::queue::ArrayQueue as Context;



use crate::{Validator, Validity, Validation, ValidatorContext};

pub struct Strategy<T, F: ?Sized> {
    f: Box<F>,
    _phantom: PhantomData<T>,
}

impl<T: 'static + Sync + Send + Clone, F> ValidationStrategy<T> for Strategy<T, F>
where
F: for<'a> Fn(&'a T) -> bool + 'static + Send + Sync + Clone,
{
    fn is_valid(&self, input: &T) -> bool {
        (self.f)(input)
    }


    fn clone_box(&self) -> Box<dyn ValidationStrategy<T>> {
        Box::new(self.clone())
    }

}

pub trait ValidationStrategy<T: 'static>: Any + Send + Sync where T: 'static + Send + Sync + Clone {
    fn is_valid(&self, input: &T) -> bool;
    // fn as_any(&'static self) -> &(dyn Any + Send + Sync);
    fn update_context(&self, context: &mut ValidatorContext, value: &T) -> Result<(), ()> {Ok(())}
    fn clone_box(&self) -> Box<dyn ValidationStrategy<T>> where Self: 'static + Send + Sync + Clone {
        Box::new(self.clone())
    }
}

impl<T: 'static + Send + Sync + Clone > dyn ValidationStrategy<T> + Send + Sync  {
    pub fn new<F>(f: F) -> Box<dyn ValidationStrategy<T>>
    where
        F: for<'a> Fn(&'a T) -> bool + 'static + Send + Sync + Clone,
    {
        Box::new(Strategy {
            f: Box::new(f),
            _phantom: PhantomData,
        })
    }

}

impl<T: 'static + Send + Sync, F> Clone for Strategy<T, F>
where
    F: for<'a> Fn(&'a T) -> bool + 'static + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        Strategy {
            f: self.f.clone(),
            _phantom: PhantomData,
        }
    }
}

pub struct StrategyContext<C> {
    pub type_id: TypeId,
    pub priority: u32,
    pub disabled: bool,
    pub current: Validity<&'static dyn Any>,  // TODO: This should be a reference to the current value
    pub context: Context<C>,
}




pub struct StrategyMap<T> where T: 'static + Send + Sync + Clone{
    pub hash_map: HashMap<TypeId, Box<dyn ValidationStrategy<T> + 'static>>,
    
}

impl<T: 'static + Send + Sync + Clone> StrategyMap<T> where T: 'static + Send + Sync + Clone {
    pub fn new() -> Self {
        Self {
            hash_map: HashMap::new(),
        }
    }

    pub fn insert_strategy(&mut self, strategy: Box<dyn ValidationStrategy<T> + 'static>) where Self: 'static + Send + Sync + Clone {
        let strategy_type_id = strategy.type_id();
        self.hash_map.insert(strategy_type_id, strategy);
    }

    pub fn remove_strategy(&mut self, strategy: TypeId) {
        self.hash_map.remove(&strategy);
    }

    pub fn contains_key(&self, strategy: &dyn Any) -> bool {
        let strategy_type_id = strategy.type_id();
        self.hash_map.contains_key(&strategy_type_id)
    }

    pub fn get_strategy_type_id(&self, strategy: &dyn Any) -> Option<TypeId> {
        let strategy_type_id = strategy.type_id();
        Some(strategy_type_id)
    }



    pub fn validate<'a>(&'a self, input: &'a T) -> Validity<T> {
        let mut is_valid = true;
        let mut not_valid = vec![];
        // let input_clone = input.clone();

        for entry in self.hash_map.iter() {
            let (type_id, strategy) = (entry.key().clone(), entry.value());
        
            let this_is_valid = strategy.is_valid(input);
            if !this_is_valid {
                not_valid.push(type_id);
            };
            
            is_valid = is_valid && this_is_valid;
        }   
        
        if not_valid.len() > 0 {
            Validity::Invalid((input.clone(), not_valid))
        } else {
            Validity::Valid(input.clone())
        }
    }

    

}


impl<T: 'static + Send + Sync> Clone for StrategyMap<T> where T: 'static + Send + Sync + Clone {
    fn clone(&self) -> Self {
        let mut hash_map = HashMap::new();
        for entry in self.hash_map.iter() {
            let (key, value) = entry.pair();
            hash_map.insert(*key, value.clone());
        }
        StrategyMap::<T>{ hash_map }
    }
}

impl<T: 'static + Send + Sync + Clone> Default for StrategyMap<T> where T: 'static + Send + Sync + Clone {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: 'static + Send + Sync> Clone for Box<dyn ValidationStrategy<T> + 'static> where T: 'static + Send + Sync + Clone {
    fn clone(&self) -> Self where Self: 'static + Send + Sync + Clone{
        self.clone()
    }
}

impl<T: 'static + Send + Sync> Into <Box<dyn ValidationStrategy<T> + 'static>> for Strategy<T, fn(&T) -> bool> where T: 'static + Send + Sync + Clone {
    fn into(self) -> Box<dyn ValidationStrategy<T> + 'static> {
        Box::new(self)
    }
}

// impl<T: 'static + Send + Sync> From <Box<dyn ValidationStrategy<T> + 'static>> for Strategy<T, fn(&T) -> bool> where T: 'static + Send + Sync + Clone {
//     fn from(strategy: Box<dyn ValidationStrategy<T> + 'static>) -> Self {
//         let strategy_type_id = strategy.type_id();
//         let strategy_fn = |input: &T| {
//             strategy.as_any().downcast_ref::<Strategy<T, fn(&T) -> bool>>().unwrap()
           
//         };
//         Strategy::<T, fn(&T) -> bool> 
//         {
//             f: Box::new(strategy_fn),
//             _phantom: PhantomData,
//         }
//     }
// }