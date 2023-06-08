


use std::clone::Clone;
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use dashmap::DashMap;



pub struct Context<T: Clone>(DashMap<TypeId, T>);

impl<T: Clone + Any> Context<T> {
    pub fn new(capacity: usize) -> Self {
        Context(DashMap::with_capacity(capacity))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }



    pub fn insert(&self, type_id: TypeId, value: T) {
        self.0.insert(type_id, value);
    }

    pub fn get_by_type_id(&self, type_id: TypeId) -> Option<T> {
        self.0.get(&type_id).map(|c| c.value().clone())
    }

    pub fn alter_by_type_id<F>(&self, type_id: TypeId, alter_fn: F)
    where
        F: FnOnce(&mut T),
    {
        if let Some(mut strategy_context) = self.0.get_mut(&type_id) {
            alter_fn(&mut *strategy_context);
        }
    }


}


