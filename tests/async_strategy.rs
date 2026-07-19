use core::future::Future;
use core::task::{Context, Poll, Waker};
use std::pin::pin;
use std::thread;

use strustegy::prelude::*;

fn block_on<F>(future: F) -> F::Output
where
    F: Future,
{
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = pin!(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => thread::yield_now(),
        }
    }
}

#[test]
fn async_closure_adapter_runs_without_boxing() {
    let double = async_strategy_fn(async |value: i32| value * 2);

    assert_eq!(block_on(double.apply_async(21)), 42);
}

#[test]
fn async_closure_can_borrow_from_its_environment() {
    let prefix = String::from("value:");
    let describe = async_strategy_fn(async |value: i32| format!("{prefix}{value}"));

    assert_eq!(block_on(describe.apply_async(7)), "value:7");
}

#[test]
fn asynchronous_strategies_compose_statically() {
    let pipeline = async_strategy_fn(async |value: i32| i64::from(value))
        .then_async(async_strategy_fn(async |value: i64| {
            format!("value:{value}")
        }))
        .then_async(async_strategy_fn(async |value: String| value.len()));

    assert_eq!(block_on(pipeline.apply_async(10)), 8_usize);
}

#[test]
fn synchronous_strategy_can_be_lifted_into_async_composition() {
    let increment = strategy_fn(|value: i32| value + 1);
    let pipeline =
        into_async(increment).then_async(async_strategy_fn(async |value: i32| value * 2));

    assert_eq!(block_on(pipeline.apply_async(20)), 42);
}

#[test]
fn async_adapter_exposes_and_returns_the_original_callable() {
    async fn increment(value: u8) -> u8 {
        value + 1
    }

    let strategy = AsyncFnStrategy::new(increment);
    assert_eq!(block_on((strategy.get_ref())(1)), 2);

    let function = strategy.into_inner();
    assert_eq!(block_on(function(2)), 3);
}
