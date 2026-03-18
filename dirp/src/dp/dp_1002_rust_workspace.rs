use crate::{DpContext, DpResult, DpResults};
use dirp_macro::dp;

#[dp(id = 1002, after = [1000])]
/// # Rust Workspace
///
/// Checks whether the directory is a **Rust workspace** by verifying:
///
/// - A `Cargo.toml` file exists (via dp-1000)
/// - The `Cargo.toml` contains a `[workspace]` section
fn rust_workspace(ctx: &DpContext, prior: &DpResults) -> DpResult {
    match prior.get(&1000) {
        Some(Ok(outcome)) if !outcome.verdict => return Ok((false, "no Cargo.toml").into()),
        Some(Err(e)) => return Err(format!("dependency dp-1000 failed: {e}")),
        _ => {}
    }
    let cargo_toml = ctx.path.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml)
        .map_err(|e| format!("failed to read Cargo.toml: {e}"))?;
    Ok(content.contains("[workspace]").into())
}
