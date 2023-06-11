
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use super::*;
use std::any::TypeId;
use crate::validation::*;


pub struct PositiveValidationStrategy;
pub struct NegativeValidationStrategy;
pub struct ZeroValidationStrategy;

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

impl Strategy for NegativeValidationStrategy {
    type Target = i32;
    type Error = ValidationError;

    fn apply(&mut self, target: &mut Self::Target) -> Result<(), Self::Error> {
        if *target < 0 {
            Ok(())
        } else {
            Err(ValidationError::strategy_error(
                TypeId::of::<Self>(),
                "Number is not negative".to_string(),
            ))
        }
    }
}

impl Strategy for ZeroValidationStrategy {
    type Target = i32;
    type Error = ValidationError;

    fn apply(&mut self, target: &mut Self::Target) -> Result<(), Self::Error> {
        if *target == 0 {
            Ok(())
        } else {
            Err(ValidationError::strategy_error(
                TypeId::of::<Self>(),
                "Number is not zero".to_string(),
            ))
        }
    }
}






fn flip_and_read(c: &mut Criterion) {
    let mut strategy = GeneralValidationStrategy::new();
    strategy.add_strategy(PositiveValidationStrategy, 1, false);
    strategy.add_strategy(NegativeValidationStrategy, 2, false);
    strategy.add_strategy(ZeroValidationStrategy, 3, false);

    let positive_number = 5;
    let negative_number = -5;
    let zero_number = 0;

    c.bench_function("pressure_bench", |b| {
        b.iter(|| {
            for _ in 0..3 { // 3 cycles
                let mut pos_num = black_box(positive_number);
                let mut neg_num = black_box(negative_number);
                let mut zero_num = black_box(zero_number);

                // Apply, read, apply a "flip", read again
                strategy.apply(&mut pos_num).unwrap();
                strategy.apply(&mut neg_num).unwrap();
                strategy.apply(&mut zero_num).unwrap();

                pos_num = -pos_num;
                neg_num = -neg_num;
                zero_num = 0;

                strategy.apply(&mut pos_num).unwrap();
                strategy.apply(&mut neg_num).unwrap();
                strategy.apply(&mut zero_num).unwrap();
            }

            // Clear the validations and start over
            strategy.strategies.clear();
            strategy.priority_map.clear();
            strategy.omitted_strategies.clear();

            strategy.add_strategy(PositiveValidationStrategy, 1, false);
            strategy.add_strategy(NegativeValidationStrategy, 2, false);
            strategy.add_strategy(ZeroValidationStrategy, 3, false);
        })
    });
}

criterion_group!(benches, flip_and_read);
criterion_main!(benches);
