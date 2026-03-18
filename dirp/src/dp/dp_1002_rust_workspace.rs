use dirp_macro::dp;
use crate::{DpResults, DpContext};

#[dp(id = 1002, after = [1000])]
/// # Rust Workspace
///
/// Checks whether the directory is a **Rust workspace** by verifying:
///
/// - A `Cargo.toml` file exists (via dp-1000)
/// - The `Cargo.toml` contains a `[workspace]` section
fn rust_workspace(ctx: &DpContext, prior: &DpResults) -> Result<bool, String> {
    if prior.get(&1000) != Some(&Ok(true)) {
        return Ok(false);
    }
    let cargo_toml = ctx.path.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml)
        .map_err(|e| format!("failed to read Cargo.toml: {e}"))?;
    Ok(content.contains("[workspace]"))
}
