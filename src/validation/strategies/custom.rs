




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