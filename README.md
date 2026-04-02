# dirp — Directory Predicates

A CLI tool that checks whether a directory satisfies directory predicates (DPs).

## Usage

```bash
# Check specific predicates against the current directory
dirp check dp-1000 dp-1001 dp-1002

# Run all lite predicates
dirp analyze

# Export all predicate metadata as JSON
dirp export
```

## Defining a Predicate

Each predicate is a single file in `dirp/src/dp/` (e.g. `dp_1000_has_cargo_toml.rs`). Files are auto-discovered at build time.

```rust
use dirp_macro::dp;
use crate::{DpContext, DpResult};
use std::collections::HashMap;

#[dp(id = 1000, lite = true)]
/// Directory contains a Cargo.toml file
fn has_cargo_toml(ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    Ok(ctx.path.join("Cargo.toml").exists().into())
}
```

The return type is `DpResult` (`Result<DpOutcome, String>`). Use `.into()` to convert:
- `Ok(true.into())` / `Ok(false.into())` — verdict without reason
- `Ok((false, "missing field").into())` — verdict with reason
- `Err("failed to read file".into())` — error

### Attributes

| Field        | Default | Description                                                       |
|--------------|---------|-------------------------------------------------------------------|
| `id`         | —       | Unique integer ID, starting from 1000 (0-999 reserved). Required. |
| `after`      | `[]`    | List of DP IDs to run first; their results are passed via `prior` |
| `lite`       | `false` | Lightweight predicate flag                                        |
| `deprecated` | `None`  | ID of the replacement predicate (marks this one as deprecated)    |

The function name serves as the predicate name, and `///` doc comments (supports markdown) become the description. Both names and IDs must be unique — duplicates cause a compile error.

### Dependencies

Use `after` to declare dependencies. They are resolved via topological sort (cycles are an error), and results are available in the `prior: &HashMap<u32, DpResult>` parameter:

```rust
#[dp(id = 10000, after = [1000], lite = false)]
/// A Rust workspace
fn rust_workspace(ctx: &DpContext, prior: &HashMap<u32, DpResult>) -> DpResult {
    match prior.get(&1000) {
        Some(Ok(outcome)) if !outcome.verdict => return Ok((false, "no Cargo.toml").into()),
        Some(Err(e)) => return Err(format!("dependency dp-1000 failed: {e}")),
        _ => {}
    }
    // ...
    Ok(true.into())
}
```

## Claude Code Integration

dirp can run as a [Claude Code Stop hook](https://code.claude.com/docs/en/hooks) to automatically check predicates when Claude finishes responding. If any predicates fail, Claude is blocked from stopping and receives feedback to fix the issues.

Add the following to your `.claude/settings.json` (project-level) or `~/.claude/settings.json` (user-level):

```json
{
  "hooks": {
    "Stop": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "dirp cc-hook dp-1000 dp-1001 dp-1002"
          }
        ]
      }
    ]
  }
}
```

Replace the predicate IDs with the ones you want to enforce. When any of the specified predicates fail, Claude will see a message like:

```
The following directory predicates failed:

- dp-1000 (has_cargo_toml): FAIL
  Description: Directory contains a Cargo.toml file, indicating a Rust project

0/1 predicates passed. Please fix the failing predicates before finishing.
```

## Build

```bash
cargo build --release
```

## Development

```bash
# Format code
cargo fmt
rustfmt dirp/src/dp/dp_*.rs

# Run lints
cargo clippy

# Quick test run against this repo
cargo run -- check dp-1000 dp-1001 dp-1002 dp-1003 dp-1004

# Run all lite predicates
cargo run -- analyze

# Export metadata
cargo run -- export
```
