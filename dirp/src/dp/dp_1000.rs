use dirp_macro::dp;
use crate::{DpResults, ScanContext};

#[dp(id = 1000, lite = true)]
/// Directory contains a Cargo.toml file, indicating a Rust project
fn has_cargo_toml(ctx: &ScanContext, _prior: &DpResults) -> bool {
    ctx.path.join("Cargo.toml").exists()
}
