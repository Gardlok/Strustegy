# Zero-copy request-line proof

This example parses a tiny, HTTP-like `STR/1` request line:

```text
GET /projects/rose STR/1
```

It is intentionally not an HTTP implementation. The small grammar keeps the
focus on Strustegy's type-level behavior.

## What it demonstrates

The parser starts with one borrowed byte slice and produces heterogeneous proof
evidence whose types are computed by GATs:

```text
&[u8]
  -> &'input str
  -> usize
  -> &'input str tokens
  -> Method
  -> RequestPath<'input>
  -> ProtocolVersion<'input>
  -> PathSegments<'input>
```

`Refine::Output<'input>` selects a different output family for each refiner,
while `Prove::Evidence<'input>` recursively assembles those outputs into one
statically known HList. Every string view remains tied to the original input
lifetime.

The example also uses const generics in `Token<INDEX>` and
`ExactTokenCount<EXPECTED>`, performs allocation-free route matching, and
checks at runtime that the borrowed views point inside the original byte
buffer.

## Run

```bash
cargo run --example request_line
```

Expected output resembles:

```text
input: GET /projects/rose STR/1
bytes: 24
tokens: 3
method: GET
path: /projects/rose
version: STR/1
path segments: 2
segments: ["projects", "rose"]
zero-copy: line, path, version, and segments borrow the input buffer
dispatch: load project "rose"
```
