# dirp — Directory Predicates

A CLI tool that checks whether a directory satisfies defined predicates (DPs).

## Usage

```bash
# Check specific predicates against the current directory
dirp check dp-1 dp-2 dp-3

# Export all predicate metadata as JSON
dirp export
```

## Defining a Predicate

Each predicate is a single file in `dirp/src/dp/` (e.g. `dp_1.rs`). Files are auto-discovered at build time.

```rust
use dirp_macro::dp;
use crate::{DpResults, ScanContext};

#[dp(id = 1, after = [], lite = true, deprecated = false)]
/// Directory contains a Cargo.toml file
fn has_cargo_toml(ctx: &ScanContext, _prior: &DpResults) -> bool {
    ctx.path.join("Cargo.toml").exists()
}
```

### Attributes

| Field        | Description                                                       |
|--------------|-------------------------------------------------------------------|
| `id`         | Unique integer ID, starting from 1                                |
| `after`      | List of DP IDs to run first; their results are passed via `prior` |
| `lite`       | Lightweight predicate flag                                        |
| `deprecated` | Marks the predicate as deprecated                                 |

The function name serves as the predicate name, and `///` doc comments (supports markdown) become the description. Both names and IDs must be unique — duplicates cause a compile error.

### Dependencies

Use `after` to declare dependencies. They are resolved via topological sort (cycles are an error), and results are available in the `prior: &DpResults` parameter:

```rust
#[dp(id = 3, after = [1], lite = false, deprecated = false)]
/// A Rust workspace
fn rust_workspace(ctx: &ScanContext, prior: &DpResults) -> bool {
    if prior.get(&1) != Some(&true) {
        return false;
    }
    // ...
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
cargo run -- check dp-1 dp-2 dp-3

# Export metadata
cargo run -- export
```
