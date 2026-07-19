# Project slug example

This example turns untrusted bytes into an `AvailableProjectSlug` through five
statically typed stages:

1. `ProofPolicy<[u8]>` proves UTF-8 and records the original byte length.
2. `ProofPolicy<str>` returns a zero-copy trimmed identifier plus an owned
   segment count through `Prove::Evidence<'input>`.
3. `FnStrategy` canonicalizes the borrowed view into an owned lowercase slug.
4. `Policy<String>` validates the canonical slug and produces
   `Validated<String, ProjectSlugPolicy>`.
5. `AsyncStrategy` checks an in-memory registry and produces the final domain
   type accepted by trusted code.

The registry and executor use only the standard library. A real application can
replace `SlugRegistry::ensure_available` with database or service I/O while
leaving the static preparation pipeline unchanged.

Run it with:

```bash
cargo run --example project_slug
```
