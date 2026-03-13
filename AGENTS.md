# Repository Guidelines

## Project Structure & Module Organization
`wp-primitives` is a single Rust library crate. Public exports live in `src/lib.rs`. Parsing helpers are grouped by concern under `src/atom.rs`, `src/comment.rs`, `src/symbol.rs`, `src/utils.rs`, and the `src/fun/`, `src/net/`, and `src/scope/` submodules. Benchmarks live in `benches/`. Keep unit tests next to the implementation they verify.

## Build, Test, and Development Commands
Use `cargo check` for fast type validation, `cargo build` for a full compile, and `cargo test` to run unit coverage. Before submitting changes, run `cargo fmt --all` and `cargo clippy --all-targets --all-features -- -D warnings`. Use `cargo bench` when changing parser hot paths.

## Coding Style & Naming Conventions
Follow default Rust formatting. Use `snake_case` for modules and functions, `UpperCamelCase` for types and enums, and `SCREAMING_SNAKE_CASE` for constants. Prefer small, composable parser helpers. Public APIs should stay syntax-oriented and avoid domain-specific semantic types.

## Testing Guidelines
Add unit tests beside the module being changed. Name tests `test_*` and focus on parser success, failure, and boundary behavior. When modifying parsing hot paths, add or update Criterion benchmarks if the change could affect throughput.

## Commit & Pull Request Guidelines
Write concise imperative commit subjects, ideally below 70 characters, for example `Add escaped scope helper for angle brackets`. PRs should summarize the parser boundary being changed and list the verification commands that were run.

## Security & Configuration Tips
Treat all input as untrusted. Prefer parser errors over panics. Keep dependencies minimal and justify new ones because this crate is intended to remain lightweight and reusable.
