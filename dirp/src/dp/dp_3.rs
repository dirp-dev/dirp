use dirp_macro::dp;
use crate::{DpResults, ScanContext};

#[dp(id = 3, after = [1], lite = false, deprecated = false)]
/// # Rust Workspace
///
/// Checks whether the directory is a **Rust workspace** by verifying:
///
/// - A `Cargo.toml` file exists (via dp-1)
/// - The `Cargo.toml` contains a `[workspace]` section
fn rust_workspace(ctx: &ScanContext, prior: &DpResults) -> bool {
    // If dp-1 (has_cargo_toml) was false, this can't be a workspace
    if prior.get(&1) != Some(&true) {
        return false;
    }
    let cargo_toml = ctx.path.join("Cargo.toml");
    match std::fs::read_to_string(&cargo_toml) {
        Ok(content) => content.contains("[workspace]"),
        Err(_) => false,
    }
}
