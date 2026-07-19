use strustegy::prelude::*;

#[test]
fn closure_adapter_runs_as_a_strategy() {
    let double = strategy_fn(|value: i32| value * 2);

    assert_eq!(double.apply(21), 42);
}

#[test]
fn closure_adapter_can_borrow_captured_configuration() {
    let prefix = String::from("value:");
    let describe = strategy_fn(|value: i32| format!("{prefix}{value}"));

    assert_eq!(describe.apply(7), "value:7");
}

#[test]
fn closure_strategies_compose_with_named_strategy_algebra() {
    let pipeline = strategy_fn(|value: i32| i64::from(value))
        .then(strategy_fn(|value: i64| format!("value:{value}")))
        .then(strategy_fn(|value: String| value.len()));

    assert_eq!(pipeline.apply(10_i32), 8_usize);
}

#[test]
fn closure_strategy_maps_an_hlist() {
    let values = hlist![1_i32, 2_i32, 3_i32];
    let doubled = values.hmap(&strategy_fn(|value: i32| value * 2));

    assert_eq!(doubled, hlist![2_i32, 4_i32, 6_i32]);
}

#[test]
fn adapter_exposes_and_returns_the_original_callable() {
    fn increment(value: u8) -> u8 {
        value + 1
    }

    let strategy = FnStrategy::new(increment as fn(u8) -> u8);
    assert_eq!((strategy.get_ref())(1), 2);

    let function = strategy.into_inner();
    assert_eq!(function(2), 3);
}
