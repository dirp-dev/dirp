use crate::{DpContext, DpResult};
use dirp_macro::dp;
use std::collections::HashMap;

#[dp(id = 1012, lite = true)]
/// Directory contains a Dockerfile
fn contain_dockerfile(ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    Ok(ctx.path.join("Dockerfile").exists().into())
}
