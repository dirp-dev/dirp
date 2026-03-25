use crate::{DpContext, DpResult};
use dirp_macro::dp;
use std::collections::HashMap;

#[dp(id = 1008, lite = true)]
/// Directory contains a .env.example file
fn contain_env_example(ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    Ok(ctx.path.join(".env.example").exists().into())
}
