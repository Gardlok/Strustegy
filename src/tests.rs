

#[cfg(test)]
mod tests_basic {
    use super::*;

    use crate::validation::strategy::StrategyPlan;
    use crate::validation::strategy::Target;
    use crate::validation::Scope;
    use crate::validation::Validator;
    use crate::validation::Proof;
    use std::marker::PhantomData;

    // Define a target
    pub struct MyTarget {
        value: i32,
    }

    impl<'a> Target<'a> for MyTarget {
        type Value = i32;
        fn value(&'a self) -> Self::Value {
            self.value
        }
    }
    // Define a proof
    struct MyProof {
        strategy: Box<dyn for<'a> Fn(&'a MyTarget) -> bool>,
    }

    impl<'a> Proof<'_, MyTarget> for MyProof {
        type Strategy = Box<dyn for<'b> Fn(&'b MyTarget) -> bool>;
        fn validate(&self, strategy: &Self::Strategy, target: &MyTarget) -> bool {
            strategy(target)
        }
    }

    // Define a scope
    struct MyScope {
        proof: MyProof,
    }

    impl<'a> Scope<'a, MyTarget> for MyScope {
        type Proof = MyProof;
        fn proof<'s>(&'s self) -> &'s Self::Proof {
            &self.proof
        }
        fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &MyTarget) -> bool {
            proof.validate(&proof.strategy, target)
        }
    }

    // Define a validator
    struct MyValidator {
        scope: MyScope,
    }

    impl<'a> Validator<'_, MyTarget> for MyValidator {
        type Scope = MyScope;

        fn validate(&self, scope: &Self::Scope, target: &MyTarget) -> bool {
            scope.validate(&scope.proof, target)
        }
    }

    // Now, let's use our custom validator to validate a target
    #[test]
    fn test_my_validator() {
        let target = MyTarget { value: 5 };
        let strategy = Box::new(|target: &MyTarget| target.value() > 0);
        let proof = MyProof { strategy: strategy };
        let scope = MyScope { proof: proof };
        let validator = MyValidator { scope: scope };

        assert!(validator.validate(&validator.scope, &target));

        // Define your strategy function
        let strategy_fn = |target: &MyTarget| -> bool {
            target.value() > 0
        };

        // Create a StrategyPlan
        let strategy_plan = StrategyPlan {
            strategy_fn,
            _phantom: PhantomData,
        };

        // Now you can use strategy_plan as a strategy
        let target = MyTarget { value: 10 };
        let is_valid = (strategy_plan.strategy_fn)(&target);
        assert!(is_valid);


    }





}

