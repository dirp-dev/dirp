use crate::{DpContext, DpResult};
use dirp_macro::dp;
use std::collections::HashMap;

#[dp(id = 1014, lite = true)]
/// Directory contains a CLAUDE.md file
fn contain_claude_md(ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    Ok(ctx.path.join("CLAUDE.md").exists().into())
}
