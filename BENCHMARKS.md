# Performance baseline

This document records a reproducible stabilization baseline for Strustegy 0.1.0.
It is not a claim that static dispatch is free, that every optimizer will produce
the same code, or that all proof paths avoid allocation.

## Environment

Measurements were collected on July 19, 2026 on the `orion` workstation:

- Debian GNU/Linux 12 (bookworm)
- Linux 6.1.0-49-amd64
- AMD FX-8120 Eight-Core Processor
- 8 logical CPUs, 4 physical cores with 2 threads per core
- CPU frequency range: 1.4-3.1 GHz, boost enabled
- 14 GiB RAM and 975 MiB swap
- `rustc 1.85.0 (4d91de4e4 2025-02-17)`
- `cargo 1.85.0 (d73d2caf9 2024-12-31)`
- LLVM 19.1.7
- default release profile, without native-CPU tuning

The machine uses dynamic CPU frequency scaling. Absolute timings should be
treated as a local baseline rather than as performance guarantees.

## Runtime harness

The dependency-free harness uses `std::time::Instant` and `std::hint::black_box`.
It performs a short warm-up before timing a loop. Run it with:

```bash
STRUSTEGY_BENCH_ITERS=1000000 cargo bench --bench baseline
```

The project-slug case runs one tenth as many iterations because it intentionally
creates an owned `String`.

| Operation | Result | What is measured |
| --- | ---: | --- |
| Request-line parsing | 505.95 ns/op | UTF-8 proof, raw HList evidence, named projections, token/path/version checks |
| Project-slug preparation | 286.86 ns/op | UTF-8/shape proof, normalization into an owned string, and validation |
| Borrowed heterogeneous refinement | 47.53 ns/op | Trimmed ASCII identifier plus owned byte-length evidence |
| HList map, length 4 | 4.13 ns/op | Four statically dispatched `u32` transformations |
| HList map, length 16 | 18.76 ns/op | Sixteen statically dispatched `u32` transformations |
| Strategy composition, depth 1 | 1.20 ns/op | One wrapping increment with a black-boxed input |
| Strategy composition, depth 8 | 1.22 ns/op | Eight statically composed wrapping increments with a black-boxed input |

The depth-1 and depth-8 composition results are effectively indistinguishable at
this measurement resolution. The optimizer can inline and simplify these small
pure operations. This is evidence about this specific generated program, not a
general assertion that composition has zero cost.

## Allocation and borrowing expectations

The library keeps `#![forbid(unsafe_code)]`. A process-global allocation counter
would require an unsafe `GlobalAlloc` implementation, so none was added merely
to support this report.

The following expectations are instead checked by API structure and focused
pointer/range tests:

- Request-line text, path, protocol version, and path segments point inside
  the original byte buffer.
- `prove_projected` consumes stack-resident HList evidence and does not box its
  named output.
- HList construction, HList mapping, strategy composition, and the built-in
  borrowed refiners require no heap allocation by themselves.
- Valid request-line parsing uses borrowed `str` slices and fixed-size domain
  values. Its current valid path contains no explicit heap allocation.
- Project-slug preparation allocates when it creates the canonical owned
  `String`; this is the intended ownership boundary.
- `validate_all` allocates its error vector even on a successful call because it
  reserves capacity for the policy's statically known rule count.
  `validate_first` does not collect such a vector.

The pointer assertions live in `tests/refine.rs` and `tests/request_line.rs`. For
allocator-level confirmation outside the crate's safe surface, build a release
example and use an external profiler, for example:

```bash
cargo build --release --example request_line
heaptrack target/release/examples/request_line
```

Tool availability and profiler overhead vary by system, so those commands are
not part of the required test suite.

## Release binary size

Commands:

```bash
cargo build --release \
  --example project_slug \
  --example request_line
stat -c '%n %s bytes' \
  target/release/examples/project_slug \
  target/release/examples/request_line
size \
  target/release/examples/project_slug \
  target/release/examples/request_line
```

| Example | File size | Text | Data | BSS |
| --- | ---: | ---: | ---: | ---: |
| `project_slug` | 449,032 bytes | 350,408 | 13,320 | 344 |
| `request_line` | 447,440 bytes | 351,020 | 13,544 | 344 |

These executables include Rust's standard-library and formatting/runtime support;
the numbers are not the incremental size of Strustegy alone.

## Compile-depth baseline

The compile fixtures increase HList length and strategy-composition depth
together at 4, 8, 16, and 32. Each result uses a fresh target directory so it
includes a clean release check of the library and fixture:

```bash
rm -rf target/compile-depth-16
/usr/bin/time -f 'wall_seconds=%e max_rss_kib=%M' \
  env CARGO_TARGET_DIR=target/compile-depth-16 \
  cargo check --release --bench compile_depth_16
```

| HList length and composition depth | Cargo-reported time | Wall time | Maximum RSS |
| ---: | ---: | ---: | ---: |
| 4 | 0.33 s | 0.35 s | 144,344 KiB |
| 8 | 0.39 s | 0.41 s | 144,224 KiB |
| 16 | 0.34 s | 0.36 s | 146,636 KiB |
| 32 | 0.35 s | 0.37 s | 144,248 KiB |

No strong growth signal is visible through depth 32 on this machine. Wall time
remained between 0.35 and 0.41 seconds, and maximum resident memory remained
between roughly 141 and 143 MiB. These fixtures are still too small to establish
how arbitrary depth scales. Deep recursive HList types, large policy evidence
types, and nested composition can increase trait solving, monomorphization,
diagnostics, and downstream incremental rebuild cost. Future measurements should
isolate HList length from composition depth and test larger generated fixtures if
real consumers approach those sizes.

## Interpretation and limitations

- Results cover successful inputs; error paths can perform different work.
- The lightweight harness reports one aggregate timing rather than a statistical
  distribution. It is intentionally dependency-free, but less rigorous than a
  Criterion study.
- Shared-VM scheduling, CPU frequency, compiler version, optimization choices,
  and surrounding application code can change every result.
- Pointer identity demonstrates zero-copy borrowing, not the absence of every
  allocator call in the process.
- Static dispatch removes virtual dispatch and boxing requirements from these
  APIs. It can increase compile time and binary size through monomorphization.
- The request-line parser intentionally repeats tokenization across independent
  refiners; it does not claim single-pass parsing.

The next useful performance decision is whether larger representative policies
show meaningful compile-time or binary-size growth. Optimize only after measured
Strustegy workloads provide that evidence.
