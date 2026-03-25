use crate::{DpContext, DpResult};
use dirp_macro::dp;
use std::collections::HashMap;

#[dp(id = 1013, lite = true)]
/// Directory contains a .github/workflows/ directory, indicating GitHub CI/Actions configuration
fn github_ci(ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    Ok(ctx.path.join(".github").join("workflows").is_dir().into())
}
