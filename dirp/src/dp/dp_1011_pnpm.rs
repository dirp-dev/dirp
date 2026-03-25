use crate::{DpContext, DpResult};
use dirp_macro::dp;
use std::collections::HashMap;

#[dp(id = 1011, lite = true)]
/// Directory uses pnpm with pnpm-lock.yaml and pnpm-workspace.yaml files
fn pnpm(ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    let has_lock = ctx.path.join("pnpm-lock.yaml").exists();
    let has_workspace = ctx.path.join("pnpm-workspace.yaml").exists();
    if has_lock && has_workspace {
        Ok(true.into())
    } else {
        let mut missing = Vec::new();
        if !has_lock {
            missing.push("pnpm-lock.yaml");
        }
        if !has_workspace {
            missing.push("pnpm-workspace.yaml");
        }
        Ok((false, format!("missing: {}", missing.join(", "))).into())
    }
}
