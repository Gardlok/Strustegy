


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
        // TODO: Implement this. This needs to be able to call the validate method on the nested_validation
        //       field and return the result of that call.  

        true
    }
    fn as_any(&self) -> &dyn Any {
        &self.nested_validation.as_any()  // TODO: This is not correct. We need to return a reference to the Any trait object for this struct.
        //      We can do this by returning a reference to the Any trait object for the nested_validation field.
        //      We can get a reference to the Any trait object for the nested_validation field by calling the
        //      as_any method on the nested_validation field. 
    }
}
