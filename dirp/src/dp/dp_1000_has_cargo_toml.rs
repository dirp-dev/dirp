use crate::{DpContext, DpResult};
use dirp_macro::dp;
use std::collections::HashMap;

#[dp(id = 1000, lite = true)]
/// Directory contains a Cargo.toml file, indicating a Rust project
fn has_cargo_toml(ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    Ok(ctx.path.join("Cargo.toml").exists().into())
}
