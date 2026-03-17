use dirp_macro::dp;
use crate::{DpResults, ScanContext};

#[dp(id = 1001, lite = true)]
/// Directory contains a pyproject.toml file, indicating a Python project
fn has_pyproject_toml(ctx: &ScanContext, _prior: &DpResults) -> bool {
    ctx.path.join("pyproject.toml").exists()
}
