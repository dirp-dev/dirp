use dirp_macro::dp;
use crate::{DpResults, ScanContext};

#[dp(id = 1002, after = [1000])]
/// # Rust Workspace
///
/// Checks whether the directory is a **Rust workspace** by verifying:
///
/// - A `Cargo.toml` file exists (via dp-1000)
/// - The `Cargo.toml` contains a `[workspace]` section
fn rust_workspace(ctx: &ScanContext, prior: &DpResults) -> bool {
    // If dp-1000 (has_cargo_toml) was false, this can't be a workspace
    if prior.get(&1000) != Some(&true) {
        return false;
    }
    let cargo_toml = ctx.path.join("Cargo.toml");
    match std::fs::read_to_string(&cargo_toml) {
        Ok(content) => content.contains("[workspace]"),
        Err(_) => false,
    }
}
