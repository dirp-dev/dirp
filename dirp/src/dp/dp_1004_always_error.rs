use crate::{DpContext, DpResult};
use dirp_macro::dp;
use std::collections::HashMap;

#[dp(id = 1004)]
/// Always returns an error (for testing)
fn always_error(_ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    Err("intentional error for testing".into())
}
