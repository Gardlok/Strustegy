



use criterion::{black_box, criterion_group, criterion_main, Criterion};
use super::*;
use std::any::TypeId;
use crate::validation::*;


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


fn pressure_bench(c: &mut Criterion) {
    let mut strategies = (0..100_000)
        .map(|_| {
            let mut strategy = GeneralValidationStrategy::new();
            strategy.add_strategy(PositiveValidationStrategy, 1, false);
            strategy
        })
        .collect::<Vec<_>>();

    let mut positive_number = 5;

    c.bench_function("pressure_bench", |b| {
        b.iter(|| {
            strategies.iter_mut().for_each(|strategy| {
                let mut pos_num = black_box(positive_number);
                strategy.apply(&mut pos_num).unwrap();
            });
        })
    });
}

criterion_group!(benches, pressure_bench);
criterion_main!(benches);
