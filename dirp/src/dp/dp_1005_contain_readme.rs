use crate::{DpContext, DpResult};
use dirp_macro::dp;
use std::collections::HashMap;

#[dp(id = 1005, lite = true)]
/// Directory contains a README.md file
fn contain_readme(ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    Ok(ctx.path.join("README.md").exists().into())
}
