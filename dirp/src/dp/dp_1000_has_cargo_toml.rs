use dirp_macro::dp;
use crate::{DpResults, DpContext};

#[dp(id = 1000, lite = true)]
/// Directory contains a Cargo.toml file, indicating a Rust project
fn has_cargo_toml(ctx: &DpContext, _prior: &DpResults) -> Result<bool, String> {
    Ok(ctx.path.join("Cargo.toml").exists())
}
