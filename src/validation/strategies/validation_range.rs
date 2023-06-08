use std::marker::PhantomData;
use std::any::TypeId;

use crate::validation::{ValidationStrategy, StrategyContext, Context};



#[derive(Clone)]
pub struct ValidationRangeStrategy<T> {
    pub min_validations: usize,
    pub max_validations: usize,
    pub validations: usize,
    _marker: PhantomData<T>,
}

impl<T> ValidationRangeStrategy<T> {
    pub fn new(min_validations: usize, max_validations: usize) -> Self {
        Self {
            min_validations,
            max_validations,
            validations: 0,
            _marker: PhantomData,
        }
    }
}

impl<T: Clone + Send + Sync + 'static> ValidationStrategy<T> for ValidationRangeStrategy<T> {
    fn is_valid(&self, _input: &T) -> bool {
        self.validations >= self.min_validations && self.validations <= self.max_validations
    }

    fn update_context(&self, context: &mut Context<StrategyContext>, _item: &T) -> Result<(), ()> {
        if self.validations < self.min_validations || self.validations > self.max_validations {
            context.alter_by_type_id(TypeId::of::<&Self>(), |strategy_context| {
                strategy_context.disabled = true;
            });
        }
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn ValidationStrategy<T>> where Self: 'static + Send + Sync + Clone {
        Box::new(self.clone())
    }
}
