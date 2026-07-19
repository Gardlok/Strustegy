use strustegy::prelude::*;

async fn run() {
    let pipeline = async_strategy_fn(async |value: i32| i64::from(value)).then_async(
        async_strategy_fn(async |value: i64| format!("value:{value}")),
    );

    assert_eq!(pipeline.apply_async(7).await, "value:7");
}

fn main() {
    // Strustegy is executor-agnostic. Call `run()` from Tokio, async-std,
    // smol, or the executor already used by your application.
    let _future = run();
}
