

use std::error::Error;
use std::marker::PhantomData;
use std::any::Any;
use std::any::TypeId;
use crossbeam::thread::Scope;
use dashmap::DashMap as HashMap;
use dashmap::DashSet as HashSet;
use crossbeam::atomic::AtomicCell; 
use crossbeam_skiplist::SkipMap as TreeMap;

use crate::validation::error::ValidationError;



use crate::validation::validator::*;
use crate::validation::target::*;
use crate::validation::proof::*;


pub trait Strategy {
    type Target;
    type Error;

    fn apply(&mut self, target: &mut Self::Target) -> Result<(), Self::Error>;
}




pub struct StrategyContext<'a, T: 'static + Sync + Send + Clone, F> {
    pub strategy: &'a dyn Strategy<Target = T, Error = ValidationError>,
    pub type_id: TypeId,
    pub priority: u32,
    pub omitted: bool,
    pub _phantom: PhantomData<&'a F>,
}




pub struct ScopeStrategy<F, S>
where
    F: FnMut(&mut dyn Any) -> bool,
    S: Strategy<Target = dyn Any, Error = std::convert::Infallible>,
{
    pub proof: F,
    pub strategy: S,
}



impl<F, S> Validator for ScopeStrategy<F, S>
where

    F: FnMut(&mut dyn Any) -> bool,
    S: Strategy<Target = dyn Any, Error = std::convert::Infallible>,
{
    type Proof<'a> = ScopedProof<'a, F, S> where F: 'a, S: 'a;
    type Error = std::convert::Infallible;

    fn validate<'a>(&'a mut self, f: &mut dyn FnMut(&mut Self::Proof<'a>) -> bool) -> bool {
        let mut proof = ScopedProof { strategy: self };
        f(&mut proof)
    }
}





// pub struct ScopedProof<'r>(&'r mut ScopeStrategy);

// impl Proof for ScopedProof<'_> {
//     fn validate(&self, f: &mut dyn FnMut(&mut dyn Any) -> bool) -> bool {
//         f(self.0)
//     }
// }

pub struct ScopedProof<'a, F, S>
where
    F: FnMut(&mut dyn Any) -> bool,
    S: Strategy<Target = dyn Any, Error = std::convert::Infallible>,
{
    strategy: &'a mut ScopeStrategy<F, S>,
}


impl<'a, F, S> Proof for ScopedProof<'a, F, S>
where
    F: FnMut(&mut dyn Any) -> bool + 'static,
    S: Strategy<Target = dyn Any, Error = std::convert::Infallible> + 'static,
{
    fn validate(&mut self, f: &mut dyn FnMut(&mut dyn Any) -> bool) -> bool {
        f(self.strategy)
    }
}



// Custom validation strategy //

pub struct CustomValidationStrategy<T: 'static, F: Fn(&T) -> bool + 'static>(
    F,
    PhantomData<T>,
);

impl<T: 'static, F: Fn(&T) -> bool + 'static> CustomValidationStrategy<T, F> {
    pub fn new(strategy: F) -> Self {
        CustomValidationStrategy(strategy, PhantomData)
    }
}



pub trait ValidationConfig<T: 'static> {
    fn is_valid(&self, input: &T) -> bool;
    fn as_any(&self) -> &dyn Any;
}
impl<T: 'static, F: Fn(&T) -> bool + 'static> ValidationConfig<T> for CustomValidationStrategy<T, F> {
    fn is_valid(&self, input: &T) -> bool {
        (self.0)(input)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T: 'static, F: Fn(&T) -> bool + 'static> Strategy for CustomValidationStrategy<T, F> {
    type Target = T;
    type Error = ValidationError;

    fn apply(&mut self, target: &mut Self::Target) -> Result<(), Self::Error> {
        if self.is_valid(target) {
            Ok(())
        } else {
            Err(ValidationError::strategy_error(
                TypeId::of::<Self>(),
                "Validation failed".to_string(),
            ))
        }
    }
}
pub struct GeneralValidationStrategy<T: 'static> {
    pub strategies: HashMap<TypeId, Box<dyn Strategy<Target = T, Error = ValidationError>>>,
    pub priority_map: TreeMap<u32, TypeId>,
    pub omitted_strategies: HashSet<TypeId>,
}

impl<T: 'static> GeneralValidationStrategy<T> {
    pub fn new() -> Self {
        GeneralValidationStrategy {
            strategies: HashMap::new(),
            priority_map: TreeMap::new(),
            omitted_strategies: HashSet::new(),
        }
    }

    pub fn add_strategy<S: 'static + Strategy<Target = T, Error = ValidationError>>(
        &mut self,
        strategy: S,
        priority: u32,
        omitted: bool,
    ) {
        let type_id = TypeId::of::<S>();
        self.strategies.insert(
            type_id,
            Box::new(strategy),
        );
        self.priority_map.insert(priority, type_id);
        if omitted {
            self.omitted_strategies.insert(type_id);
        }
    }
}

impl<T: 'static> Strategy for GeneralValidationStrategy<T> {
    type Target = T;
    type Error = ValidationError;

    fn apply(&mut self, target: &mut Self::Target) -> Result<(), Self::Error> {
        let mut errors = Vec::new();
        for entry in self.priority_map.iter() {
            let type_id = entry.value();
            if !self.omitted_strategies.contains(type_id) {
                if let Some(mut strategy) = self.strategies.get_mut(type_id) {
                    if let Err(error) = strategy.apply(target) {
                        errors.push(error);
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationError::multiple_errors(errors))
        }
    }
}