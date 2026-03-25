use crate::{DpContext, DpResult};
use dirp_macro::dp;
use std::collections::HashMap;

#[dp(id = 1009, lite = true)]
/// Directory contains a pyproject.toml file, indicating a Python project
fn python_project(ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    Ok(ctx.path.join("pyproject.toml").exists().into())
}
