use crate::{DpContext, DpResult};
use dirp_macro::dp;
use std::collections::HashMap;

#[dp(id = 1003, deprecated = 1000)]
/// Old check for Cargo.toml (superseded by dp-1000)
fn has_cargo_toml_deprecated(ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    Ok(ctx.path.join("Cargo.toml").exists().into())
}
