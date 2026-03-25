use crate::{DpContext, DpResult};
use dirp_macro::dp;
use std::collections::HashMap;

#[dp(id = 1010, lite = true)]
/// Directory is an npm project with a src/ directory and package.json file
fn npm(ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    let has_src = ctx.path.join("src").is_dir();
    let has_package_json = ctx.path.join("package.json").exists();
    if has_src && has_package_json {
        Ok(true.into())
    } else {
        let mut missing = Vec::new();
        if !has_src {
            missing.push("src/");
        }
        if !has_package_json {
            missing.push("package.json");
        }
        Ok((false, format!("missing: {}", missing.join(", "))).into())
    }
}
