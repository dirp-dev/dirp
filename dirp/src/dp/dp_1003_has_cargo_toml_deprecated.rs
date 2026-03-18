use crate::{DpContext, DpResult, DpResults};
use dirp_macro::dp;

#[dp(id = 1003, deprecated = 1000)]
/// Old check for Cargo.toml (superseded by dp-1000)
fn has_cargo_toml_deprecated(ctx: &DpContext, _prior: &DpResults) -> DpResult {
    Ok(ctx.path.join("Cargo.toml").exists().into())
}
