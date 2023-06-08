



use crate::{ValidationError, Validator, ValidationStrategy, StrategyMap, Strategy, Context, StrategyContext};



use std::marker::PhantomData;
use dashmap::DashMap as HashMap;
use crossbeam::queue::{ArrayQueue, SegQueue};
use crossbeam::sync::ShardedLock;
use std::any::{TypeId, Any};

use std::sync::Arc;

use std::sync::atomic::{AtomicUsize, Ordering};





pub struct CustomValidationStrategy<T: 'static, F: Fn(&T) -> bool + 'static>(
    F,
    PhantomData<T>,
);

impl<T: 'static, F: Fn(&T) -> bool + 'static> CustomValidationStrategy<T, F> {
    fn new(strategy: F) -> Self {
        CustomValidationStrategy(strategy, PhantomData)
    }
}

impl<T: 'static + Clone + Sync + Send, F> ValidationStrategy<T> for CustomValidationStrategy<T, F>
where
    F: Fn(&T) -> bool + 'static + Clone + Sync + Send,
{
    fn is_valid(&self, input: &T) -> bool {
        (self.0)(input)
    }


}

trait ValidationConfig<T: 'static> {
    fn is_valid(&self, input: &T) -> bool;
    fn as_any(&self) -> &dyn Any;
}




pub struct ValidatorFactory<T: 'static> {
    validators: Vec<Validation<T>>,
}

impl<T: 'static> ValidatorFactory<T> {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    pub fn create_validator(&mut self) -> &mut Validation<T> where T: Clone + Send + Sync + 'static{
        let validator = Validation::new();
        self.validators.push(validator);
        self.validators.last_mut().unwrap()
    }

    pub fn remove_validator(&mut self, validator: &Validation<T>) {
        self.validators.retain(|v| !std::ptr::eq(v, validator));
    }

    pub fn remove_strategy(&mut self, validator: &mut Validation<T>, strategy: &dyn Any) where T: Clone + Send + Sync + 'static {
        validator.remove_strategy(strategy);
    }
}



#[derive(Clone)]
pub enum Validity<T> {
    Valid(T),
    Invalid((T, Vec<TypeId>)),
    NotChecked,
}



impl<T> Into<bool> for Validity<T> {
    fn into(self) -> bool {
        match self {
            Validity::Valid(_) => true,
            Validity::Invalid(_) => false,
            Validity::NotChecked => false,
        }
    }
}



impl<T> Validity<T> {
    pub fn is_valid(&self) -> bool {
        match self {
            Validity::Valid(_) => true,
            Validity::Invalid(_) => false,
            Validity::NotChecked => false,
        }
    }

    pub fn is_invalid(&self) -> bool {
        match self {
            Validity::Valid(_) => false,
            Validity::Invalid(_) => true,
            Validity::NotChecked => false,
        }
    }
}

pub trait ValidateOption<T: Clone + Send + Sync>: Sized {
    fn validate(self, validator: &Validator<T>) -> Option<T>;
}

impl<T> ValidateOption<T> for Option<T> 
where
    T: Clone + Send + Sync + 'static,
{
    fn validate(self, validator: &Validator<T>) -> Option<T> {
        match &self {
            Some(value) => match validator.validate(value) {
                Validity::Valid(_) => Some(value.clone()),
                _ => None,  // Return None if validation fails or is not checked
            },
            None => None,
        }
    }
}

// impl Arc<SegQueue<>

