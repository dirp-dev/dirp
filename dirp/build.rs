use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let dp_dir = Path::new("src/dp");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("dp_mods.rs");

    let mut mods = Vec::new();
    if let Ok(entries) = fs::read_dir(dp_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with("dp_") && name.ends_with(".rs") {
                let mod_name = name.trim_end_matches(".rs").to_string();
                mods.push(mod_name);
            }
        }
    }
    mods.sort();

    // Parse each DP file to extract id and fn name, check for duplicates
    let mut seen_ids: HashMap<String, String> = HashMap::new(); // id -> file
    let mut seen_names: HashMap<String, String> = HashMap::new(); // name -> file
    let mut errors = Vec::new();

    for m in &mods {
        let path = dp_dir.join(format!("{m}.rs"));
        let content = fs::read_to_string(&path).unwrap_or_default();

        // Extract id from #[dp(id = N, ...)]
        if let Some(id) = extract_dp_id(&content) {
            if let Some(prev_file) = seen_ids.insert(id.clone(), format!("{m}.rs")) {
                errors.push(format!(
                    "duplicate dp id {id}: found in both {prev_file} and {m}.rs"
                ));
            }
        }

        // Extract fn name after #[dp(...)]
        if let Some(name) = extract_dp_fn_name(&content) {
            if let Some(prev_file) = seen_names.insert(name.clone(), format!("{m}.rs")) {
                errors.push(format!(
                    "duplicate dp name \"{name}\": found in both {prev_file} and {m}.rs"
                ));
            }
        }
    }

    if !errors.is_empty() {
        for e in &errors {
            println!("cargo:warning={e}");
        }
        let joined = errors.join("; ").replace('"', "'");
        let content = format!("compile_error!(\"dp uniqueness check failed: {joined}\");\n");
        fs::write(&dest, content).unwrap();
        println!("cargo:rerun-if-changed=src/dp");
        return;
    }

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let abs_dp_dir = Path::new(&manifest_dir).join("src/dp");

    let content: String = mods
        .iter()
        .map(|m| {
            let path = abs_dp_dir.join(format!("{m}.rs"));
            let path = path.display().to_string().replace('\\', "/");
            format!("#[path = \"{path}\"]\nmod {m};\n")
        })
        .collect();

    fs::write(&dest, content).unwrap();

    // Re-run if dp directory changes
    println!("cargo:rerun-if-changed=src/dp");
}

/// Extract the id value from `#[dp(id = N, ...)]`
fn extract_dp_id(content: &str) -> Option<String> {
    let dp_attr = content.find("#[dp(")?;
    let rest = &content[dp_attr..];
    let close = rest.find(')')?;
    let inside = &rest[5..close]; // after "#[dp("

    for part in inside.split(',') {
        let part = part.trim();
        if let Some(val) = part.strip_prefix("id") {
            let val = val.trim().strip_prefix('=')?.trim();
            return Some(val.to_string());
        }
    }
    None
}

/// Extract the function name from `fn name(...)` that follows `#[dp(...)]`
fn extract_dp_fn_name(content: &str) -> Option<String> {
    let dp_attr = content.find("#[dp(")?;
    let rest = &content[dp_attr..];
    let fn_pos = rest.find("fn ")?;
    let after_fn = &rest[fn_pos + 3..];
    let end = after_fn.find('(')?;
    Some(after_fn[..end].trim().to_string())
}
