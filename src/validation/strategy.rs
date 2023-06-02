

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







// ValidationStrategy is a trait that defines the interface for validation strategies
// that can be used to validate input in a validation pipeline (see src/validation/strategies/combo.rs)
// 
pub trait ValidationStrategy<T: 'static> { // T is the type of input that the strategy will validate
    fn is_valid(&self, input: &T) -> bool; // Returns true if the input is valid, false otherwise
    fn as_any(&self) -> &dyn Any;          // Returns a reference to the underlying type as a trait object
} // The 'static lifetime is used to indicate that the type T will live for the entire duration of the program


// 


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
// in a validation pipeline (see src/validation/strategies/combo.rs) using a nested validation
// pipeline (see src/validation/validation.rs)
//
impl ValidationStrategy<i32> for NestedValidationStrategy {
    fn is_valid(&self, data: &i32) -> bool {
        self.nested_validation.is_valid(data)
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







