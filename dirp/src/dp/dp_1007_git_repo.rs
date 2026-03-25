use crate::{DpContext, DpResult};
use dirp_macro::dp;
use std::collections::HashMap;

#[dp(id = 1007, lite = true)]
/// Directory is a Git repository with a .git directory and .gitignore file
fn git_repo(ctx: &DpContext, _prior: &HashMap<u32, DpResult>) -> DpResult {
    let has_git = ctx.path.join(".git").is_dir();
    let has_gitignore = ctx.path.join(".gitignore").exists();
    if has_git && has_gitignore {
        Ok(true.into())
    } else {
        let mut missing = Vec::new();
        if !has_git {
            missing.push(".git/");
        }
        if !has_gitignore {
            missing.push(".gitignore");
        }
        Ok((false, format!("missing: {}", missing.join(", "))).into())
    }
}
