




// Compare target with partialeq, eq, or cmp
//
pub fn compare<T: PartialEq>(target: &T, value: &T) -> bool {
    target == value
}
pub fn compare_eq<T: Eq>(target: &T, value: &T) -> bool {
    target == value
}
pub fn compare_cmp<T: PartialOrd>(target: &T, value: &T) -> bool {
    target == value
}

// Operation Tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    use crate::Operation;
    use crate::StrategyObject;
    use crate::TargetObject;
    use crate::ParameterObject;

 
    #[test]
    fn test_operation1() {
        let mut binding = Operation::new(&1);
        let operation = binding
            .strategy(&|target| *target == 1)
            .strategy(&|target| *target == 2)
            .strategy(&|target| *target == 3)
            .parameter("foo", &"bar")
            .parameter("bar", &"foo");
        assert_eq!(operation.execute(), false);
    }


    #[test]
    fn it_works2<'a>() {
        let mut binding = Operation::new(&1);
        let operation = binding
        .strategy(&|x| *x == 1)
        .strategy(&|x| *x == 2);  
        assert_eq!(operation.execute(), false);
    }

    // Test Strategy
    //
    #[test]
    fn test_strategy() {
        let mut operation = Operation::new(&0);
        let strategy = |target: &i32| *target == 0;
        assert_eq!(strategy(&0), true);
        assert_eq!(strategy(&1), false);
        assert_eq!(operation.strategy(&strategy).execute(), true);
    }

    // Test Strategy
    //
    #[test]
    fn test_strategy_object() {
        let mut operation = Operation::new(&0);
        let strategy = |target: &i32| *target == 0;
        assert_eq!(strategy(&0), true);
        assert_eq!(strategy(&1), false);
        assert_eq!(operation.strategy(&strategy).execute(), true);
    }

    #[test]
    fn test() {
        let mut operation = Operation::new(&1);
        operation
            .strategy(&|target| compare(target, &1))
            .strategy(&|target| compare(target, &1))
            .strategy(&|target| compare(target, &1))
            .strategy(&|target| compare(target, &1))
            .execute();
    }
   

}













#[cfg(test)]
mod tests_with_context {
    use super::*;

    use crate::Operation;
    use crate::StrategyObject;
    use crate::StrategyFnWithContext;
    use crate::StrategyWithContext;
    use crate::OperationWithContext;
    use crate::TargetObject;
    use crate::ParameterObject;

    use std::any::Any;
    use std::collections::HashMap;




    struct TestStrategy;
    // for Operation execute
    impl TestStrategy {
        fn execute(&self, target: &i32, parameters: &HashMap<&str, &dyn Any>) -> bool {
            for strategy in self.strategies() {
                if !strategy.call(target, parameters) {
                    return false;
                }
            }
            true
        }
    }
    struct TestStrategyFn<F>(F);
    impl<'a, T, F> StrategyFnWithContext<'a, T> for TestStrategyFn<F>
    where
        F: Fn(&T, &HashMap<&'a str, &'a dyn Any>) -> bool,
    {
        type Params = HashMap<&'a str, &'a dyn Any>;

        fn call(&self, target: &T, params: &Self::Params) -> bool {
            (self.0)(target, params)
        }
    }
    impl<'a, T> StrategyWithContext<'a, T> for TestStrategy {
        type Params = HashMap<&'a str, &'a dyn Any>;
    
        fn strategies(&self) -> Vec<Box<dyn StrategyFnWithContext<'a, T, Params = Self::Params>>> {
            vec![
                Box::new(TestStrategyFn(|target: &T, params: &Self::Params| {
                    println!("params: {:?}", params);
                    true
                })),
                Box::new(TestStrategyFn(|target: &T, params: &Self::Params| { true })),
            ]
        }
    }

    #[test]
    fn test_operation_with_context_execute<'a>() {
        let target = "test target";
        let strategy = TestStrategy;
        let mut parameters = HashMap::new();
        parameters.insert("test_param", &"test_value" as &dyn Any);

        let operation = OperationWithContext::new(&target, strategy, parameters);

        assert!(operation.execute());
    }

}

