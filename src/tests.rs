

#[cfg(test)]
mod tests_basic {
    use super::*;

    use crate::validation::strategy::StrategyFn;
    use crate::validation::strategy::Strategy;
    use crate::validation::Target;
    use crate::validation::Scope;
    use crate::validation::Validator;
    use crate::validation::Proof;
    use std::marker::PhantomData;




    // Long thorough test of the basic functionality of the validation library
//     #[test]
//     fn test_basic() {
//         // Define a target
//         pub struct MyTarget { value: i32 }

//         impl<'a> Target<'a> for MyTarget {
//             type Value = i32;
//             type Proof = MyProof;

            

//             fn value(&'a self) -> Self::Value {
//                 self.value
//             }

//             fn validate(&'a self, proof: &'a Self::Proof) -> bool {
//                 todo!()
//             }
//         }

//         // Define a strategy
//         struct MyStrategy { _phantom: PhantomData<MyTarget>}
//         impl<'a> StrategyFn::Strategy<'a, MyTarget> for MyStrategy {
//             type StrategyFn = Box<dyn for<'b> Fn(&'b MyTarget) -> bool>;
//             fn strategy(&self) -> Self::StrategyFn {
//                 Box::new(|target| target.value() > 0)
//             }
//         }

//         // Build another way

//         struct MyStrategy2 { _phantom: PhantomData<MyTarget>}
//         impl<'a> Strategy<'a, MyTarget> for MyStrategy2 {
//             type StrategyFn = Box<dyn for<'b> Fn(&'b MyTarget) -> bool>;
//             fn strategy(&self) -> Self::StrategyFn {
//                 Box::new(|target| target.value() > 0)
//             }
//         }
        
    
//         // Define a proof
//         struct MyProof { strategy: Box<dyn for<'a> Fn(&'a MyTarget) -> bool> }

//         impl<'a> Proof<'_, MyTarget> for MyProof {
//             type StrategyFn = Box<dyn for<'b> Fn(&'b MyTarget) -> bool>;
//             fn validate(&self, strategy: &Self::StrategyFn, target: &MyTarget) -> bool {
//                 strategy(target)
//             }
//         }

//         // Define a scope
//         struct MyScope { proof: MyProof }

//         impl<'a> Scope<'a, MyTarget> for MyScope {
//             type Proof = MyProof;
//             fn proof<'s>(&'s self) -> &'s Self::Proof {
//                 &self.proof
//             }
//             fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &MyTarget) -> bool {
//                 proof.validate(&proof.strategy, target)
//             }
//         }

//         // Define a validator
//         struct MyValidator<'a> { scope: MyScope, _phantom: PhantomData<&'a MyTarget> }

//         impl<'a> Validator<'a, MyTarget> for MyValidator<'a> {
//             type Scope = MyScope;
//             fn validate(&'a self, scope: &Self::Scope, target: &MyTarget) -> bool {
//                 let proof = scope.proof();
//                 scope.validate(proof, target)
//             }
//         }

//     }
// }










    // Define a target




// #[cfg(test)]
// mod strategy_plan_tests {

//     use super::*;
//     use crate::validation::strategy::StrategyFn;
//     use crate::validation::strategy::Strategy;
//     use crate::validation::Target;
//     use crate::validation::Scope;
//     use crate::validation::Validator;
//     use crate::validation::Proof;
//     use std::marker::PhantomData;

//     // Define a target
//     pub struct MyTarget { value: i32 }

//     impl<'a> Target<'a> for MyTarget {
//         type Value = i32;
//         type Proof = MyProof;

        

//         fn value(&'a self) -> Self::Value {
//             self.value
//         }

//         fn validate(&'a self, proof: &'a Self::Proof) -> bool {
//             todo!()
//         }
//     }
  


//     // Define a strategy
//     struct MyStrategy { _phantom: PhantomData<MyTarget>}

//     // Define a proof
//     struct MyProof { strategy: Box<dyn for<'a> Fn(&'a MyTarget) -> bool> }   

//     impl<'a> Proof<'_, MyTarget> for MyProof {
//         type StrategyFn = Box<dyn for<'b> Fn(&'b MyTarget) -> bool>;
//         fn validate(&self, strategy: &Self::StrategyFn, target: &MyTarget) -> bool {
//             strategy(target)
//         }
//     }

//     // Define a scope
//     struct MyScope { proof: MyProof }

//     impl<'a> Scope<'a, MyTarget> for MyScope {
//         type Proof = MyProof;
//         fn proof<'s>(&'s self) -> &'s Self::Proof {
//             &self.proof
//         }
//         fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &MyTarget) -> bool {
//             proof.validate(&proof.strategy, target)
//         }
//     }


//     struct MyTargetProof { strategy: Box<dyn for<'a> Fn(&'a MyTarget) -> bool> }
//     struct MyTargetScope { proof: MyTargetProof }

//     // Define a validator
//     struct MyValidator { scope: MyScope }
//     impl<'a> Validator<'_, MyTarget> for MyValidator {
//         type Scope = MyScope; // 

//         fn validate(&self, scope: &Self::Scope, target: &MyTarget) -> bool {
//             scope.validate(&scope.proof, target)
//         }
//     }

//     // Now, let's use our custom validator to validate a target
//     #[test]
//     fn test_my_validator() {
//         let target = MyTarget { value: 5 };
//         let strategy = Box::new(|target: &MyTarget| target.value() > 0);
//         let proof = MyProof { strategy: strategy };
//         let scope = MyScope { proof: proof };
//         let validator = MyValidator { scope: scope };

//         assert!(validator.validate(&validator.scope, &target));
//     }
// }
}