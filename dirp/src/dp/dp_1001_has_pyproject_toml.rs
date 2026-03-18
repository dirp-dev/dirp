use std::collections::HashMap;
use crate::{DpContext, DpResult};
use dirp_macro::dp;

#[dp(id = 1001, lite = true)]
/// Directory contains a pyproject.toml file, indicating a Python project
fn has_pyproject_toml(ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    Ok(ctx.path.join("pyproject.toml").exists().into())
}
