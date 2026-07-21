# Mixed nested-HList deployment manifest

This example models an untrusted deployment-manifest request and turns fixed local
input into a typed trusted-boundary value. It performs no deployment, network,
filesystem, database, authorization, availability, or real checksum operation.

Run it with:

```bash
cargo run --example nested_manifest
```

## Complete typed shape

```rust
hlist_ty![
    ManifestHeader,
    hlist_ty![
        ProjectInputEvidence<'input>,
        EnvironmentInputEvidence<'input>,
    ],
    hlist_ty![
        Validated<String, ProjectNamePolicy>,
        Validated<String, EnvironmentNamePolicy>,
    ],
    ExecutionMode,
    hlist_ty![
        Validated<String, ArtifactNamePolicy>,
        Validated<String, ChecksumSyntaxPolicy>,
    ],
    hlist_ty![
        Validated<u64, TimeoutPolicy>,
        Validated<u8, RetryLimitPolicy>,
    ],
    BoundarySummary,
]
```

The outer HList mixes ordinary values with nested HLists. The groups are
independently typed and may be ragged even though this concrete manifest uses
two-cell groups. The structure is therefore best read as a statically typed tree
or document. It is not a rectangular table or matrix, and nothing enforces equal
row lengths.

## Borrowed and validated groups

The project and environment proof policies use `TrimmedAsciiIdentifier` plus
`ByteLen`. Their named projected evidence contains a trimmed `&str` that points
inside the original input and an owned source-byte count. The output GAT carries
the input lifetime, so safe Rust cannot retain projected evidence after the source
string is gone.

Canonical identity preparation lowercases ASCII letters while preserving digits,
`_`, and `-`, then validates the owned string under distinct project and
environment policy markers. `Validated` records that a value passed that policy at
construction time; it is a receipt, not authority.

Artifact names are limited to 96 bytes, use lowercase ASCII letters, digits,
`.`, `_`, and `-`, and must begin and end with an ASCII alphanumeric character.
Checksum syntax requires exactly 64 lowercase ASCII hexadecimal characters. It
validates textual syntax only.

Timeout is restricted to `1..=60_000` milliseconds. Retry limit is restricted to
`0..=5`.

## Nested indexing and patterns

The example first uses ordinary two-step static indexing:

```rust
let identity_group = manifest.get_at::<IdentityGroupIndex>();
let environment = identity_group.get_at::<SecondIndex>();
```

It then uses an example-local `Get2Ext` helper for concise two-level access. The
helper is not exported by Strustegy and the example supplies no evidence that it
belongs in the public API.

Nested `hlist_pat!` patterns destructure the complete tree while preserving the
meaning of every position.

## Shallow and nested borrowing

`manifest.refs()` borrows each top-level cell. Each nested HList is therefore one
borrowed top-level value. Calling `refs()` again on a selected nested group borrows
that group's individual cells. This deliberate shallow behavior avoids inventing
implicit recursive traversal semantics.

## What this does not prove

The example does not establish:

- deployment authorization;
- artifact existence;
- checksum correctness against real bytes;
- environment availability;
- current permission;
- freshness of any external state.

It also does not implement recursive mapping, matrix operations, transposition,
serialization, runtime schemas, or deep borrowing.
