

use std::marker::PhantomData;
use std::any::Any;
use std::any::TypeId;
use dashmap::DashMap as HashMap;

use crate::validation::{Validator, Context};
use crate::Validity;

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
    fn update_context(&self, context: &mut Context<StrategyContext>, value: &T) -> Result<(), ()> {Ok(())}
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


pub struct StrategyContext {
    pub type_id: TypeId,
    pub priority: u32,
    pub disabled: bool,
    pub current: Validity<&'static dyn Any>,  // TODO: This should be a reference to the current value
}




impl Default for StrategyContext {
    fn default() -> Self {
        Self {
            type_id: TypeId::of::<()>(),
            priority: 0,
            disabled: false,
            current: Validity::NotChecked,
        }
    }
}



impl Clone for StrategyContext {
    fn clone(&self) -> Self {
        Self {
            type_id: self.type_id,
            priority: self.priority,
            disabled: self.disabled,
            current: self.current.clone(),
        }
    }
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
