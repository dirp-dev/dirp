# dirp — Directory Predicates

## Project Structure

Cargo workspace with two crates:
- `dirp/` — main binary (CLI + core logic)
- `dirp-macro/` — proc macro crate providing `#[dp(...)]`

## Key Files

- `dirp/src/main.rs` — CLI entry point (clap)
- `dirp/src/lib.rs` — core types (DpOutcome, DpResult, DpContext, Predicate), runner, graph resolution
- `dirp-macro/src/lib.rs` — `#[dp()]` proc macro
- `dirp/src/dp/` — one file per predicate, auto-discovered by `build.rs`
- `dirp/build.rs` — scans `dp/` dir, generates mod declarations, checks id/name uniqueness at compile time

## Adding a New Predicate

Create `dirp/src/dp/dp_{id}_{name}.rs`:

```rust
use crate::{DpContext, DpResult};
use dirp_macro::dp;
use std::collections::HashMap;

#[dp(id = NNNN)]
/// Description here (supports multiline markdown)
fn predicate_name(ctx: &DpContext, prior: &HashMap<u32, DpResult>) -> DpResult {
    Ok(ctx.path.join("some_file").exists().into())
}
```

Attributes: `id` (required), `after` (default `[]`), `lite` (default `false`), `deprecated` (default `None`, set to replacement DP id).

Return types via `.into()`: `true`/`false` for simple verdict, `(bool, "reason")` for verdict with reason. Use `Err("msg".into())` for errors.

IDs start at 1000 (0-999 reserved). File name must match pattern `dp_{id}_{fn_name}.rs`.

## Build & Dev Commands

```bash
cargo build
cargo fmt
rustfmt dirp/src/dp/dp_*.rs   # fmt doesn't auto-discover dp files
cargo clippy
cargo run -- check dp-1000 dp-1001 dp-1002 dp-1003 dp-1004
cargo run -- analyze   # run all lite predicates
cargo run -- cc-hook dp-1000 dp-1001  # Claude Code Stop hook (reads stdin)
cargo run -- export
```

## Conventions

- `prior` parameter gives access to dependency results; use `after` to declare deps
- Dependencies resolved via topological sort; cycles are a runtime error
- Duplicate DP ids or fn names cause compile errors (checked in build.rs)
- Deprecated DPs still run but print a warning pointing to the replacement
