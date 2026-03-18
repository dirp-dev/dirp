# dirp — Directory Predicates

A CLI tool that checks whether a directory satisfies directory predicates (DPs).

## Usage

```bash
# Check specific predicates against the current directory
dirp check dp-1000 dp-1001 dp-1002

# Export all predicate metadata as JSON
dirp export
```

## Defining a Predicate

Each predicate is a single file in `dirp/src/dp/` (e.g. `dp_1000_has_cargo_toml.rs`). Files are auto-discovered at build time.

```rust
use dirp_macro::dp;
use crate::{DpResults, DpContext};

#[dp(id = 1000, lite = true)]
/// Directory contains a Cargo.toml file
fn has_cargo_toml(ctx: &DpContext, _prior: &DpResults) -> Result<bool, String> {
    Ok(ctx.path.join("Cargo.toml").exists())
}
```

### Attributes

| Field        | Default | Description                                                       |
|--------------|---------|-------------------------------------------------------------------|
| `id`         | —       | Unique integer ID, starting from 1000 (0-999 reserved). Required. |
| `after`      | `[]`    | List of DP IDs to run first; their results are passed via `prior` |
| `lite`       | `false` | Lightweight predicate flag                                        |
| `deprecated` | `false` | Marks the predicate as deprecated                                 |

The function name serves as the predicate name, and `///` doc comments (supports markdown) become the description. Both names and IDs must be unique — duplicates cause a compile error.

### Dependencies

Use `after` to declare dependencies. They are resolved via topological sort (cycles are an error), and results are available in the `prior: &DpResults` parameter:

```rust
#[dp(id = 10000, after = [1000], lite = false, deprecated = false)]
/// A Rust workspace
fn rust_workspace(ctx: &DpContext, prior: &DpResults) -> Result<bool, String> {
    if prior.get(&1000) != Some(&Ok(true)) {
        return Ok(false);
    }
    // ...
    Ok(true)
}
```

## Build

```bash
cargo build --release
```

## Development

```bash
# Format code
cargo fmt

# Run lints
cargo clippy

# Quick test run against this repo
cargo run -- check dp-1000 dp-1001 dp-1002

# Export metadata
cargo run -- export
```
