

use std::marker::PhantomData;

use crate::validation::strategy::StrategyFn;
use crate::validation::strategy::Strategy;
use crate::validation::strategy::Scope;
use crate::validation::strategy::Proof;
use crate::validation::validity::*; 

use crate::validation::error::ValidationError;



pub struct GenericValidator<'a, T, S: for<'s> Scope<'s, T>> {
    pub scope: S,
    pub(crate) _phantom: PhantomData<&'a T>,
}

pub struct GenericStrategy<'a, T, P: Proof<'a, T>> {
    pub proof: P,
    _phantom: std::marker::PhantomData<&'a T>,
}

pub struct GenericScope<'a, T, P: for<'s> Proof<'s, T>> {
    pub proof: P,
    _phantom: PhantomData<&'a T>,
}

pub struct GenericProof<'a, T, S: StrategyFn<T>> {  
    pub strategy: S,                              
    _phantom: PhantomData<&'a T>,                 
}

impl <'a, T, S: StrategyFn<T>> GenericProof<'a, T, S> {
    pub fn new(strategy: S) -> Self {
        Self {
            strategy,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, P: for<'s> Proof<'s, T>> GenericStrategy<'a, T, P> {
    pub fn new(proof: P) -> Self {
        
        Self {
            proof,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, P: for<'s> Proof<'s, T>> GenericScope<'a, T, P> {
    pub fn new(proof: P) -> Self {
        Self {
            proof,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, S: for<'s> Scope<'s, T>> GenericValidator<'a, T, S> {
    pub fn new(scope: S) -> Self {
        Self {
            scope,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, P: for<'s> Proof<'s, T, StrategyFn = P>> Scope<'a, T> for GenericScope<'a, T, P> {
    type Proof = P;
    fn proof<'s>(&'s self) -> &'s Self::Proof {
        &self.proof
    }
    fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &T) -> bool {
        proof.validate(&self.proof, target)
    }
}

impl<'a, T, S: for<'s> Scope<'s, T> + 'a> Validator<'a, T> for GenericValidator<'a, T, S> {
    type Scope = S;
    fn validate(&'a self, scope: &Self::Scope, target: &T) -> bool {  // This is the same as the generic scope validate
        let proof = scope.proof();
        scope.validate(proof, target)
    }
}

impl<'a, T, P: for<'s> Proof<'s, T, StrategyFn = P>> Scope<'a, T> for GenericStrategy<'a, T, P> {
    type Proof = P;
    fn proof<'s>(&'s self) -> &'s Self::Proof {
        &self.proof
    }
    fn validate<'s>(&'s self, proof: &'s Self::Proof, target: &T) -> bool {
        proof.validate(&self.proof, target)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    struct TestTarget {
        value: i32,
    }

    impl TestTarget {
        fn value(&self) -> i32 {
            self.value
        }
    }

    struct TestProof<F: Fn(&TestTarget) -> bool> {
        strategy: F,
    }

    impl<'a, F: Fn(&TestTarget) -> bool> Proof<'a, TestTarget> for TestProof<F> {
        type StrategyFn = F;

        fn validate(&'a self, _strategy: &Self::StrategyFn, target: &TestTarget) -> bool {
            (self.strategy)(target)
        }
    }

    #[test]
    fn test_generic_strategy() {
        // Define a strategy that checks if the target's value is positive
        let strategy = |target: &TestTarget| target.value() > 0;

        // Create a TestProof with the strategy
        let proof = TestProof { strategy };

        // Create a GenericStrategy with the proof
        let strategy = GenericStrategy::new(proof);

        // Create a target with a positive value
        let target = TestTarget { value: 1 };

        // Validate the target
        let result = strategy.proof.validate(&strategy.proof.strategy, &target);

        // Check that the result is true
        assert!(result);
    }


    #[test]
    fn test_more() {

        let strategy = |target: &TestTarget| target.value() > 0;

        let proof = TestProof { strategy };

        let strategy = GenericStrategy::new(proof);

        let target = TestTarget { value: 1 };

        let result = strategy.proof.validate(&strategy.proof.strategy, &target);

        assert!(result);

    }





}
