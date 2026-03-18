use dirp_macro::dp;
use crate::{DpResults, DpContext};

#[dp(id = 1001, lite = true)]
/// Directory contains a pyproject.toml file, indicating a Python project
fn has_pyproject_toml(ctx: &DpContext, _prior: &DpResults) -> Result<bool, String> {
    Ok(ctx.path.join("pyproject.toml").exists())
}
