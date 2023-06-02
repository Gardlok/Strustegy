

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
use std::sync::Arc;

use dashmap::DashMap as HashMap;

use crate::validation::error::ValidationError;
use crate::validation::error::AnyValidationError;
use crate::validation::error::MultipleValidationError;

use crate::validation::Validation;


use crate::validator::Validator;

use crate::strategies::*;



// ValidationStrategy is a trait that defines the interface for validation strategies
// that can be used to validate input in a validation pipeline (see src/validation/strategies/combo.rs)
// 
// pub trait ValidationStrategy<T: 'static> {
//     fn is_valid(&self, input: &T) -> bool;
//     fn as_any(&self) -> &dyn Any;
// }
// // 
// impl<T: 'static> PartialEq for dyn ValidationStrategy<T> {
//     fn eq(&self, other: &dyn ValidationStrategy<T>) -> bool {
//         self.as_any().is(other.as_any())
//     }
// }
// //
// impl<T: 'static> Eq for dyn ValidationStrategy<T> {}
// //
// impl<T: 'static> Hash for dyn ValidationStrategy<T> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.as_any().hash(state)
//     }
// }

// ValidationStrategy is a trait that defines the interface for validation strategies to be used
pub trait ValidationStrategy<T: 'static> {
    fn is_valid(&self, input: &T) -> bool;
    fn as_any(&self) -> &dyn Any;
}
//
// impl<T: 'static> Eq for dyn ValidationStrategy<T> {}
// //
// impl<T: 'static> Hash for dyn ValidationStrategy<T> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.as_any().hash(state)  // TODO: This is not correct. We need to hash the actual types of the strategies.
//                                    // We can do this by using the Any::type_id method.
//     }
// }

// Hasher for ValidationStrategy
////////////////////////////////////////////
/// 



// Extra support for ValidationStrategy
impl<T: 'static> dyn ValidationStrategy<T> {
    // Creates a new ValidationStrategy from the given function. The function will be used to
    // validate the input. 
    pub fn new<F>(f: F) -> impl ValidationStrategy<T>
    where
        F: Fn(&T) -> bool + 'static,
    {
        struct Strategy<T, F> {
            f: F,
            _phantom: PhantomData<T>,
        }
        impl<T: 'static, F> ValidationStrategy<T> for Strategy<T, F>
        where
            F: Fn(&T) -> bool + 'static,
        {
            fn is_valid(&self, input: &T) -> bool {
                (self.f)(input)
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
        Strategy {
            f,
            _phantom: PhantomData,
        }
    }
}




// ValidationStrategy is a trait that defines the interface for validation strategies to be used
// by the Validator. The Validator will call the is_valid method on each strategy to determine
// whether the input is valid or not. 
// 
pub struct ComboValidationStrategy<T: 'static> {
    static_strategies: Vec<Box<dyn ValidationStrategy<T>>>,
    dynamic_strategies: Vec<Box<dyn ValidationStrategy<T>>>,
} 
impl<T: 'static> ComboValidationStrategy<T> {
    // Creates a new ComboValidationStrategy with the given static and dynamic strategies to be used
    // when validating input. The static strategies are executed first, followed by the dynamic
    // strategies. If any of the static strategies fail, the input is considered invalid and the
    // dynamic strategies are not executed. If any of the dynamic strategies fail, the input is 
    // considered invalid. If all of the static and dynamic strategies pass, the input is considered
    // valid. 
    pub fn new(static_strategies: Vec<Box<dyn ValidationStrategy<T>>>, dynamic_strategies: Vec<Box<dyn ValidationStrategy<T>>>) -> Self {
        ComboValidationStrategy {
            static_strategies,
            dynamic_strategies,
        }
    }
}
impl<T: 'static> ValidationStrategy<T> for ComboValidationStrategy<T> {
    // Validates the given input using the static and dynamic strategies. If any of the static
    // strategies fail, the input is considered invalid and the dynamic strategies are not executed.
    // If any of the dynamic strategies fail, the input is considered invalid. If all of the static
    // and dynamic strategies pass, the input is considered valid.
    //
    fn is_valid(&self, input: &T) -> bool {
        // All static strategies must pass if all dynamic strategies pass
        // If any of the static strategies fail, the input is invalid
        // If all of the static strategies pass, the dynamic strategies are executed
        // 
        if !self.static_strategies.iter().all(|strategy| strategy.is_valid(input)) {
            return false;
        }
        // All dynamic strategies must pass if all static strategies pass
        // If any of the dynamic strategies fail, the input is invalid
        // If all of the dynamic strategies pass, the input is valid
        // If any of the static strategies fail, the input is invalid
        // If all of the static strategies pass, the dynamic strategies are executed
        // 
        if !self.dynamic_strategies.iter().all(|strategy| strategy.is_valid(input)) {
            return false;
        }
        // If all static and dynamic strategies pass, the input is valid
        true
    }
    // Returns a reference to the underlying Any trait object for this ComboValidationStrategy
    // 
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// NestedValidationStrategy is a validation strategy that can be used to validate input
// in a validation pipeline (see src/validation/strategies/combo.rs) using a nested validation
// pipeline (see src/validation/validation.rs)
//
pub struct NestedValidationStrategy {
    nested_validation: Validation<i32>,
}
// NestedValidationStrategy is a validation strategy that can be used to validate input
// in a validation pipeline  using a nested validation pipeline 
//
impl ValidationStrategy<i32> for NestedValidationStrategy {
    fn is_valid(&self, data: &i32) -> bool {
        
        // TODO: Implement this method to validate the given input using the nested validation pipeline
        // Try: self.nested_validation.validate(data) <-- Don't work on this until you have implemented
        // the Validation struct to be able to compile this code. The Validation struct is defined in
        // src/validation/validation.rs and looks like this:
        pub struct Validation<T> {
            strategies: Vec<Box<dyn ValidationStrategy<T>>>,
        }
        impl<T> Validation<T> {
            pub fn new(strategies: Vec<Box<dyn ValidationStrategy<T>>>) -> Self {
                Validation {
                    strategies,
                }
            }
            pub fn validate(&self, data: &T) -> bool {
                self.strategies.iter().all(|strategy| strategy.is_valid(data))
            }
        }
        self.nested_validation.validate(data)
        self


    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


// ClosureValidationStrategy is a validation strategy that can be used to validate input
// in a validation pipeline (see src/validation/strategies/combo.rs) using a closure
// that returns a boolean value indicating whether the input is valid or not 
//
pub struct ClosureValidationStrategy<T> {
    validation_fn: Box<dyn Fn(&T) -> bool>, // The closure that will be used to validate input
    _phantom: PhantomData<T>,  // PhantomData is used to indicate that T is not used in this struct
}
// ClosureValidationStrategy is a validation strategy that can be used to validate input
// in a validation pipeline (see src/validation/strategies/combo.rs) using a closure
// that returns a boolean value indicating whether the input is valid or not
// 
impl<T> ClosureValidationStrategy<T> {
    pub fn new(validation_fn: Box<dyn Fn(&T) -> bool>) -> Self {
        ClosureValidationStrategy {
            validation_fn,
            _phantom: PhantomData,
        }
    }
}
// ValidationStrategy is a trait that defines the interface for validation strategies
// that can be used to validate input in a validation pipeline (see src/validation/strategies/combo.rs)
//
impl<T: 'static> ValidationStrategy<T> for ClosureValidationStrategy<T> {
    fn is_valid(&self, value: &T) -> bool {
        (self.validation_fn)(value)
    }
    fn as_any(&self) -> &dyn Any {
        todo!()
    }
}


// CustomValidationStrategy is a validation strategy that can be used to validate input
// in a validation pipeline (see src/validation/strategies/combo.rs) using a custom function
// that returns a boolean value indicating whether the input is valid or not 
// 
pub struct CustomValidationStrategy<T: 'static, F: Fn(&T) -> bool + 'static>(
    F,
    PhantomData<T>,
);
impl<T: 'static, F: Fn(&T) -> bool + 'static> CustomValidationStrategy<T, F> {
    pub fn new(strategy: F) -> Self {
        // PhantomData is used to indicate that the type T is used in the function signature
        // but is not actually used in the struct itself. This is necessary because the compiler
        // will complain if the type T is not used in the struct definition. 
        CustomValidationStrategy(strategy, PhantomData)
    }
}
// ValidationStrategy is a trait that defines the interface for validation strategies
// that can be used to validate input in a validation pipeline (see src/validation/strategies/combo.rs)
// 
impl<T: 'static, F> ValidationStrategy<T> for CustomValidationStrategy<T, F>
where
    F: Fn(&T) -> bool + 'static,
{
    fn is_valid(&self, input: &T) -> bool {
        (self.0)(input)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}





