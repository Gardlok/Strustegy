use std::env;
use std::hint::black_box;
use std::time::{Duration, Instant};

use strustegy::prelude::*;

#[allow(dead_code)]
#[path = "../examples/project_slug/mod.rs"]
mod project_slug;
#[allow(dead_code)]
#[path = "../examples/request_line/mod.rs"]
mod request_line;
struct BorrowedProof;

impl ProofPolicy<str> for BorrowedProof {
    type Refiners = hlist_ty![TrimmedAsciiIdentifier, ByteLen];

    fn refiners() -> Self::Refiners {
        hlist![TrimmedAsciiIdentifier, ByteLen]
    }
}

fn measure<T>(name: &str, iterations: u64, mut operation: impl FnMut() -> T) {
    for _ in 0..iterations.min(10_000) {
        black_box(operation());
    }

    let started = Instant::now();
    for _ in 0..iterations {
        black_box(operation());
    }
    let elapsed = started.elapsed();
    let nanos = elapsed.as_nanos() as f64 / iterations as f64;

    println!("{name:<34} {nanos:>12.2} ns/op  ({iterations} iterations)");
}

fn composition_depth(iterations: u64) {
    let increment = strategy_fn(|value: u32| value.wrapping_add(1));
    let depth_one = increment;
    let depth_eight = increment
        .then(increment)
        .then(increment)
        .then(increment)
        .then(increment)
        .then(increment)
        .then(increment)
        .then(increment);

    measure("strategy composition depth 1", iterations, || {
        depth_one.apply(black_box(1))
    });
    measure("strategy composition depth 8", iterations, || {
        depth_eight.apply(black_box(1))
    });
}

fn hlist_mapping(iterations: u64) {
    let increment = strategy_fn(|value: u32| value.wrapping_add(1));

    measure("HList map length 4", iterations, || {
        hlist![
            black_box(1_u32),
            black_box(2_u32),
            black_box(3_u32),
            black_box(4_u32),
        ]
        .hmap(&increment)
    });
    measure("HList map length 16", iterations, || {
        hlist![
            black_box(1_u32),
            black_box(2_u32),
            black_box(3_u32),
            black_box(4_u32),
            black_box(5_u32),
            black_box(6_u32),
            black_box(7_u32),
            black_box(8_u32),
            black_box(9_u32),
            black_box(10_u32),
            black_box(11_u32),
            black_box(12_u32),
            black_box(13_u32),
            black_box(14_u32),
            black_box(15_u32),
            black_box(16_u32),
        ]
        .hmap(&increment)
    });
}

fn main() {
    let iterations = env::var("STRUSTEGY_BENCH_ITERS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(250_000);

    println!("Strustegy lightweight baseline");
    println!("timer resolution: {:?}", Duration::from_nanos(1));

    let request = b"GET /projects/demo STR/1";
    measure("request-line parsing", iterations, || {
        request_line::policy::parse_request(black_box(request)).expect("valid request")
    });

    let slug = b"  Strustegy_Demo  ";
    measure("project-slug preparation", iterations / 10, || {
        project_slug::pipeline::prepare_slug(black_box(slug)).expect("valid slug")
    });

    let borrowed = "  sync_status  ";
    measure("borrowed heterogeneous refinement", iterations, || {
        prove::<BorrowedProof, _>(black_box(borrowed)).expect("valid borrowed proof")
    });

    hlist_mapping(iterations);
    composition_depth(iterations);
}
