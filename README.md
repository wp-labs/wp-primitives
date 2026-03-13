# wp-primitives

![CI](https://github.com/wp-labs/wp-primitives/workflows/CI/badge.svg)
![License](https://img.shields.io/badge/License-Apache%202.0-green.svg)

## Overview
`wp-primitives` is a small Rust library that packages the low-level parsing building blocks shared across the Warp Parse stack. It stays intentionally narrow: token extraction, balanced-scope helpers, symbol parsers, function-call argument parsing, lightweight comment stripping, and a few network-oriented readers.

This crate does not carry WPL/OML AST or runtime semantics. It is the bottom layer that higher-level crates can reuse without pulling in language-specific types.

## Modules
- `atom`: common token readers such as identifiers, paths, quoted snippets, and parenthesized values.
- `comment`: comment stripping helpers for DSL-like source text.
- `fun`: generic helpers for parsing `f()`, `f(x)`, and `f(x, y)` style calls.
- `net`: IP-oriented parsers and helpers.
- `scope`: balanced delimiter evaluators for nested scopes.
- `symbol`: reusable symbol and comparison parsers.
- `utils`: small parser utilities and scope extraction helpers.

## Quick Start
```toml
[dependencies]
wp-primitives = "0.1"
```

```rust
use wp_primitives::atom::take_var_name;
use wp_primitives::Parser;

let mut input = "  field_name rest";
let name = take_var_name.parse_next(&mut input).unwrap();

assert_eq!(name, "field_name");
assert_eq!(input, " rest");
```

## Design Boundary
- Keep only syntax-agnostic parsing primitives here.
- Do not add language AST, runtime evaluators, `DataType`, or other domain-specific semantic types.
- If a parser starts depending on business metadata instead of raw text shape, it likely belongs in a higher-level crate.

## Development
Use `cargo check`, `cargo test`, `cargo fmt --all`, and `cargo clippy --all-targets --all-features -- -D warnings` before sending changes. Benchmarks live under `benches/` and use Criterion.

## License
Distributed under the Apache License 2.0. See [`LICENSE`](LICENSE).
