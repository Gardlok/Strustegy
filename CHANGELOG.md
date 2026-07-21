# Changelog

All notable changes to Strustegy will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project intends to follow semantic versioning once the public API stabilizes.

## [Unreleased]

### Added

- GAT-backed HList shared and mutable views.
- Static HList indexing.
- Type-directed `Strategy` abstraction.
- Strategy composition with `Compose` and `.then(...)`.
- HList strategy mapping for owned, shared, and mutable values.
- Policy-backed validation and non-forgeable `Validated` values.
- GAT-backed zero-copy `Refine` outputs and HList proof evidence.
- `Witnessed` borrowed proof values with redacted diagnostics.
- `FnStrategy` adapters for functions and shared closures.
- `AsyncStrategy`, async-closure adapters, static async composition, and synchronous strategy lifting.
- A documented proof and threat model for `Witnessed` and `Validated`.
- Policy-owned named evidence projection through `ProjectEvidence` and `prove_projected`.
- Short-circuiting synchronous and asynchronous `Result` composition.
- A lightweight runtime, binary-size, allocation-expectation, and compile-depth baseline.
- A mixed nested-HList deployment-manifest example with borrowed evidence and policy-backed values.
