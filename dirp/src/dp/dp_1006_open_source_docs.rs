use crate::{DpContext, DpResult};
use dirp_macro::dp;
use std::collections::HashMap;

#[dp(id = 1006, lite = true)]
/// Directory contains open source documentation files:
/// LICENSE, CONTRIBUTING.md, and CHANGELOG.md
fn open_source_docs(ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    let files = ["LICENSE", "CONTRIBUTING.md", "CHANGELOG.md"];
    let missing: Vec<&str> = files
        .iter()
        .filter(|f| !ctx.path.join(f).exists())
        .copied()
        .collect();
    if missing.is_empty() {
        Ok(true.into())
    } else {
        Ok((false, format!("missing: {}", missing.join(", "))).into())
    }
}
