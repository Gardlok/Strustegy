




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

    // use crate::Operation;

    // use crate::TargetObject;
    // use crate::ParameterObject;

 
    // #[test]
    // fn test_operation1() {
    //     let mut binding = Operation::new(&1);
    //     let operation = binding
    //         .strategy(&|target| *target == 1)
    //         .strategy(&|target| *target == 2)
    //         .strategy(&|target| *target == 3)
    //         .parameter("foo", &"bar")
    //         .parameter("bar", &"foo");
    //     assert_eq!(operation.execute(), false);
    // }


    // #[test]
    // fn it_works2<'a>() {
    //     let mut binding = Operation::new(&1);
    //     let operation = binding
    //     .strategy(&|x| *x == 1)
    //     .strategy(&|x| *x == 2);  
    //     assert_eq!(operation.execute(), false);
    // }

    // // Test Strategy
    // //
    // #[test]
    // fn test_strategy() {
    //     let mut operation = Operation::new(&0);
    //     let strategy = |target: &i32| *target == 0;
    //     assert_eq!(strategy(&0), true);
    //     assert_eq!(strategy(&1), false);
    //     assert_eq!(operation.strategy(&strategy).execute(), true);
    // }

    // // Test Strategy
    // //
    // #[test]
    // fn test_strategy_object() {
    //     let mut operation = Operation::new(&0);
    //     let strategy = |target: &i32| *target == 0;
    //     assert_eq!(strategy(&0), true);
    //     assert_eq!(strategy(&1), false);
    //     assert_eq!(operation.strategy(&strategy).execute(), true);
    // }

    // #[test]
    // fn test() {
    //     let mut operation = Operation::new(&1);
    //     operation
    //         .strategy(&|target| compare(target, &1))
    //         .strategy(&|target| compare(target, &1))
    //         .strategy(&|target| compare(target, &1))
    //         .strategy(&|target| compare(target, &1))
    //         .execute();
    // }
   

}


#[cfg(test)]
mod tests_with_context {



    use crate::StrategyWithContext;


    use std::any::{Any, TypeId};
    use std::collections::HashMap;

    struct TestStrategy;
    struct TestStrategyFn<F>(F);
    // impl<'a, T, F> StrategyFnWithContext<'a, T> for TestStrategyFn<F>
    // where
    //     F: Fn(&T, &HashMap<&'a str, &'a dyn Any>) -> bool, T: Clone
    // {
    //     type Params = HashMap<&'a str, &'a dyn Any>;

    //     fn call(&self, target: &T, params: &Self::Params) -> bool {
    //         (self.0)(target, params)
    //     }
    // }
    
    // #[test]
    // fn test_strategy_with_context() {
    //     let strategy = TestStrategy;
    //     let mut parameters = HashMap::new();
    //     parameters.insert("test_param", &"test_value" as &dyn Any);

    //     let strategy_with_context = <dyn StrategyWithContext<&str>>::new(&strategy, parameters);

    //     assert!(strategy_with_context.call(&"test_target"));
    // }



}
//     #[test]
//     fn test_operation_with_context_execute<'a>() {
//         let target = "test target";
//         let strategy = TestStrategy;
//         let mut parameters = HashMap::new();
//         parameters.insert("test_param", &"test_value" as &dyn Any);

//         let operation = OperationWithContext::new(&target, strategy, parameters);

//         assert!(operation.execute());
//     }

// }


// Integration Tests
//
#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::any::Any;

    // use crate::Operation;
    // use crate::TargetObject;
    // use crate::ParameterObject;

    // #[test]
    // fn test_operation1() {
    //     let mut binding = Operation::new(&1);
    //     let operation = binding
    //         .strategy(&|target| *target == 1)
    //         .strategy(&|target| *target == 2)
    //         .strategy(&|target| *target == 3)
    //         .parameter("foo", &"bar")
    //         .parameter("bar", &"foo");
    //     assert_eq!(operation.execute(), false);
    // }

    // #[test]
    // fn test_operation2() {
    //     let mut binding = Operation::new(&1);
    //     let operation = binding
    //         .strategy(&|target| *target == 1)
    //         .strategy(&|target| *target == 2)
    //         .strategy(&|target| *target == 3)
    //         .parameter("foo", &"bar")
    //         .parameter("bar", &"foo");
    //     assert_eq!(operation.execute(), false);
    // }

    // #[test]
    // fn test_operation3() {
    //     let mut binding = Operation::new(&1);
    //     let operation = binding
    //         .strategy(&|target| *target == 1)
    //         .strategy(&|target| *target == 2)
    //         .strategy(&|target| *target == 3)
    //         .parameter("foo", &"bar")
    //         .parameter("bar", &"foo");
    //     assert_eq!(operation.execute(), false);
    // }

    // // true
    // #[test]
    // fn test_operation4() {
    //     let mut binding = Operation::new(&1);
    //     let operation = binding
    //         .strategy(&|target| *target == 1)
    //         .strategy(&|target| *target == 1)
    //         .strategy(&|target| *target == 1)
    //         .parameter("foo", &"bar")
    //         .parameter("bar", &"foo");
    //     assert_eq!(operation.execute(), true);
    // }


}


// Test code
#[cfg(test)]
mod inprogenitance_tests {
    use std::marker::PhantomData;

    use super::*;
    // use crate::inprogenitance::{Inprogenitance,  Progeny, InprogenitanceImpl};

    // #[test]
    // fn test_inprogenitance() {

    //     // Progeny<'a, T: Clone, R: Clone>
    //     let progeny: Progeny<'_, &i32, bool> = Progeny {
    //         value: &&1,
    //         progenitor: None,
    //         operations: vec![],  
    //         result: None,
    //     };
        

    //     let inprogenitance: InprogenitanceImpl<'_, &i32, bool> = InprogenitanceImpl {
    //         value: &1,
    //         progeny: vec![progeny],
    //         _marker: PhantomData,
    //     };

    //     // create an instance 
    //     let mut my_inprogenitance: InprogenitanceImpl<'_, &i32, bool> = InprogenitanceImpl {
    //         value: &1,
    //         progeny: vec![],
    //         _marker: PhantomData,
    //     };


    //     assert_eq!(my_inprogenitance.progeny.pop(), 
    //         Some(Progeny {
    //             value: &&1,
    //             progenitor:None,
    //             operations:vec![],
    //             result:None, 
    //         }));


    //     assert_eq!(my_inprogenitance.value, &1);

    // }

    #[test]
    fn test_2() {

        // // Create a new MyInprogenitance instance.
        // let mut my_inprogenitance = MyInprogenitanceBuilder::<&i32, bool>::new()
        //     .value(&1)
        //     .progeny(Progeny {
        //         value: &&1,
        //         progenitor: None,
        //         operations: vec![],
        //         result: None,
        //     })
        //     .build();
        
        // // assert_eq!(my_inprogenitance.unwrap(), &1);








        


        

    }

    
    #[test]
    fn test_inprogenitance3<'a>() {
        // let mut my_inprogenitance: InprogenitanceImpl<'a, &i32, bool> = InprogenitanceImpl {
        //     value: &1,
        //     progeny: vec![],
        //     _marker: PhantomData,
        // };


        // my_inprogenitance.progenate(Progeny {
        //     value: &&6,
        //     progenitor: None, 
        //     operations: vec![],
        //     result: None,
        // });




    }

}