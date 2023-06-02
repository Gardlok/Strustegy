

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

use crate::validation::Validation;


use crate::validator::Validator;

use crate::strategies::*;








/* the trait */

pub trait ValidationStrategy<T: 'static>: Any + Send + Sync {
    fn is_valid(&self, input: &T) -> bool;
    fn as_any(&self) -> &(dyn Any + 'static) where Self: 'static;
    fn eq_with_dyn(&self, other: &(dyn ValidationStrategy<T> + 'static)) -> bool;
    fn hash_with_dyn(&self, hasher: &mut (dyn Hasher + '_));
    // fn fetch_mesh(&self, base: &mut RenderBase, lod: Lod) -> Result<RenderFaceMeshLink, Error>;
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

pub struct Lookup<T> {
    hash_map: HashMap<Box<dyn ValidationStrategy<T> + 'static>, u32>,
}


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
                    self.eq(x)
                } else {
                    false
                }
            }

            fn hash_with_dyn(&self, hasher: &mut (dyn Hasher + '_)) {
                todo!()
            }
        }

        impl<T, F> Iterator for Strategy<T, F> {
            type Item = Box<dyn ValidationStrategy<T> + 'static>;
            fn next(&mut self) -> Option<Self::Item> {
                None
            }
        }




        // IntoIterator for Strategy<T, F> 
        impl<T, F> IntoIterator for Strategy<T, F> {  
            type Item = Box<dyn ValidationStrategy<T> + 'static>;
            type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'static>;
            fn into_iter(self) -> Self::IntoIter {
                Box::new(self)
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



pub trait IntoStrategyIterator <T>{
    type Item: ValidationStrategy<T> + 'static;
    type IntoIter: Iterator<Item = Self::Item> + 'static;
    fn into_strategy_iter(self) -> Self::IntoIter;
}

// IntoStrategyIterator
impl<T: 'static> IntoStrategyIterator<T> for dyn ValidationStrategy<T> + 'static {
    type Item = Box<dyn ValidationStrategy<T> + 'static>;
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'static>;
    fn into_strategy_iter(self) -> Self::IntoIter {
        Box::new(std::iter::once(Box::new(self)))
    }
}

impl<T: 'static> IntoStrategyIterator<T> for Box<dyn ValidationStrategy<T> + 'static> {
    type Item = Box<dyn ValidationStrategy<T> + 'static>;
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'static>;
    fn into_strategy_iter(self) -> Self::IntoIter {
        Box::new(std::iter::once(self))
    }
}

impl<T: 'static> IntoStrategyIterator<T> for Vec<Box<dyn ValidationStrategy<T> + 'static>> {
    type Item = Box<dyn ValidationStrategy<T> + 'static>;
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'static>;
    fn into_strategy_iter(self) -> Self::IntoIter {
        Box::new(self.into_iter())
    }
}






/* example of how to implement ViewableObjectMesh for your type */

#[derive(PartialEq, Eq, Hash)]
struct Foo {
    name: String,
    data: Vec<u8>,
}

impl<T: 'static> ValidationStrategy<T> for Foo {
    fn as_any(&self) -> &(dyn Any + 'static) {
        self as _
    }

    fn eq_with_dyn(&self, other: &(dyn ValidationStrategy<T> + 'static)) -> bool {
        if let Some(x) = other.downcast_ref::<Foo>() {
            self.eq(x)
        } else {
            false
        }
    }

    fn hash_with_dyn(&self, mut hasher: &mut (dyn Hasher + '_)) {
        // the extra &mut here is because Hash::hash requires the Hasher to be Sized
        // fortunately there is a blanket `impl Hasher for &mut X where X: Hasher`,
        // and `&mut (dyn Hasher + '_)` is Sized, so the added mutable borrow lets this work
        self.hash(&mut hasher)
    }

    fn is_valid(&self, input: &T) -> bool {
        todo!()
    }
}











