



#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{ValidationStrategy, Validation, ValidationError, Validator, StrategyMap, Strategy};
    use std::any::TypeId;

    #[macro_use]
    use crate::strategy_fn;
    
    use std::fmt::Debug;
    
    // Define a custom data type for testing
    #[derive(Debug)]
    struct TestData {
        value: i32,
    }
    
    // Define the validation strategies using the `strategy_fn` macro
    strategy_fn!(GreaterThanZeroStrategy, |data: &TestData| data.value > 0);
    
    strategy_fn!(EvenNumberStrategy, |data: &TestData| data.value % 2 == 0);
        
    #[test]
    fn test_greater_than_zero_strategy() {
        let strategy = GreaterThanZeroStrategy::new(|data: &TestData| data.value > 0);

        let valid_data = TestData { value: 10 };
        let invalid_data = TestData { value: -5 };

        assert!(strategy.is_valid(&valid_data));
        assert!(!strategy.is_valid(&invalid_data));
    }

    #[test]
    fn test_even_number_strategy() {
        let strategy = EvenNumberStrategy::new(|data: &TestData| data.value % 2 == 0);

        let even_data = TestData { value: 10 };
        let odd_data = TestData { value: 7 };

        assert!(strategy.is_valid(&even_data));
        assert!(!strategy.is_valid(&odd_data));
    }


}


