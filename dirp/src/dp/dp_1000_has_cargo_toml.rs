use crate::{DpContext, DpResult, DpResults};
use dirp_macro::dp;

#[dp(id = 1000, lite = true)]
/// Directory contains a Cargo.toml file, indicating a Rust project
fn has_cargo_toml(ctx: &DpContext, _prior: &DpResults) -> DpResult {
    Ok(ctx.path.join("Cargo.toml").exists().into())
}
