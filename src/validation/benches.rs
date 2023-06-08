
#![feature(test)]
use test::Bencher;


#[cfg(test)]
mod benches {
    use test::Bencher;
    use super::*;

    #[bench]
    fn bench_validation_strategy_map_validate(b: &mut Bencher) {
        let mut map = StrategyMap::<i32>::new();
        let strategy = Strategy::new(|x| *x > 0);
        map.insert_strategy(Box::new(strategy));

        b.iter(|| {
            map.validate(&1);
        });
    }

    #[bench]
    fn bench_validation_validate(b: &mut Bencher) {
        let mut validation = Validation::<i32>::new();
        let strategy = Strategy::new(|x| *x > 0);
        validation.add_validator(Validator::new().add_strategy(Box::new(strategy)));

        b.iter(|| {
            validation.is_valid(&1);
        });
    }

    #[bench]
    fn bench_validation_strategy_new(b: &mut Bencher) {
        b.iter(|| {
            let _strategy = ValidationStrategy::new(|_| true);
        });
    }

    #[bench]
    fn bench_validation_strategy_is_valid(b: &mut Bencher) {
        let strategy = ValidationStrategy::new(|_| true);
        b.iter(|| {
            let _is_valid = strategy.is_valid(&());
        });
    }

    #[bench]
    fn bench_validation_strategy_update_context(b: &mut Bencher) {
        let strategy = ValidationStrategy::new(|_| true);
        let mut context = Context::<StrategyContext>::new(1);
        b.iter(|| {
            let _ = strategy.update_context(&mut context, &());
        });
    }

    #[bench]
    fn bench_validation_strategy_is_valid(b: &mut Bencher) {
        let strategy = ValidationStrategy::new(|_| true);
        b.iter(|| {
            let _ = strategy.is_valid(&());
        });
    }

    #[bench]
    fn bench_validation_strategy_clone_box(b: &mut Bencher) {
        let strategy = ValidationStrategy::new(|_| true);
        b.iter(|| {
            let _ = strategy.clone_box();
        });
    }

    #[bench]
    fn bench_validator_add_strategy(b: &mut Bencher) {
        let mut validator = Validator::<String>::new();
        let strategy = Strategy::<String, fn(&String) -> bool>::new(|_| true);
        b.iter(|| {
            validator.add_strategy(Box::new(strategy));
        });
    }

    #[bench]
    fn bench_validator_remove_strategy(b: &mut Bencher) {
        let mut validator = Validator::<String>::new();
        let strategy = Strategy::<String, fn(&String) -> bool>::new(|_| true);
        validator.add_strategy(Box::new(strategy));
        b.iter(|| {
            validator.remove_strategy(&strategy).unwrap();
        });
    }
}