


#[macro_use]


pub use crate::validation::*;
// use crate::strategy_fn;
pub use std::any::TypeId;

#[cfg(test)]
mod tests {
    use super::*;

    struct PositiveValidationStrategy;

    impl Strategy for PositiveValidationStrategy {
        type Target = i32;
        type Error = ValidationError;

        fn apply(&mut self, target: &mut Self::Target) -> Result<(), Self::Error> {
            if *target > 0 {
                Ok(())
            } else {
                Err(ValidationError::strategy_error(
                    TypeId::of::<Self>(),
                    "Number is not positive".to_string(),
                ))
            }
        }
    }

    #[test]
    fn test_general_validation_strategy() {
        let mut strategy = GeneralValidationStrategy::new();
        strategy.add_strategy(PositiveValidationStrategy, 1, false);

        let mut positive_number = 5;
        assert!(strategy.apply(&mut positive_number).is_ok());

        let mut negative_number = -5;
        assert!(strategy.apply(&mut negative_number).is_err());
    }

    #[test]
    fn test_general_validation_strategy_with_omitted_strategy() {
        let mut strategy = GeneralValidationStrategy::new();
        strategy.add_strategy(PositiveValidationStrategy, 1, true);

        let mut negative_number = -5;
        assert!(strategy.apply(&mut negative_number).is_ok());
    }


    #[cfg(test)]
    mod tests {
        use super::*;
    
        struct FibonacciValidationStrategy;
    
        impl Strategy for FibonacciValidationStrategy {
            type Target = (i32, i64);
            type Error = ValidationError;
    
            fn apply(&mut self, target: &mut Self::Target) -> Result<(), Self::Error> {
                let (n, expected) = *target;
                let sqrt_5 = 5f64.sqrt();
                let phi = (1.0 + sqrt_5) / 2.0;
                let psi = (1.0 - sqrt_5) / 2.0;
                let result = ((phi.powi(n) - psi.powi(n)) / sqrt_5).round() as i64;
                if result == expected {
                    Ok(())
                } else {
                    Err(ValidationError::strategy_error(
                        TypeId::of::<Self>(),
                        format!("Fibonacci number at index {} is {}, expected {}", n, result, expected),
                    ))
                }
            }
        }

    #[test]
    fn test_fibonacci_with_general_validation_strategy() {
        let mut strategy = GeneralValidationStrategy::new();
        strategy.add_strategy(FibonacciValidationStrategy, 1, false);

        let fibonacci_sequence = vec![
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 5),
            (6, 8),
            (7, 13),
            (8, 21),
            (9, 34),
            (10, 55),
        ];

        for (n, expected) in fibonacci_sequence {
            let mut target = (n, expected);
            assert!(strategy.apply(&mut target).is_ok());
        }
    }





    #[test]
    fn test_general_validation_strategy() {
        let mut strategy = GeneralValidationStrategy::new();

        // Add a strategy that checks if the input is greater than 5
        strategy.add_strategy(
            CustomValidationStrategy::new(|&x: &i32| x > 5),
            1,
            false,
        );

        // Add a strategy that checks if the input is even
        strategy.add_strategy(
            CustomValidationStrategy::new(|&x: &i32| x % 2 == 0),
            2,
            false,
        );

        // Test with a valid input
        let mut input = 6;
        assert!(strategy.apply(&mut input).is_ok());

        // Test with an invalid input
        let mut input = 3;
        assert!(strategy.apply(&mut input).is_err());
    }
}
}