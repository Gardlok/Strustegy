


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