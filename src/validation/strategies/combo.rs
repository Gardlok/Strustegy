



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
