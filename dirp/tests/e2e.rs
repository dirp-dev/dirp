use std::fs;
use std::process::Command;

fn dirp_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_dirp"))
}

fn dirp_check(dir: &std::path::Path, ids: &[&str]) -> std::process::Output {
    dirp_bin()
        .arg("check")
        .args(ids)
        .current_dir(dir)
        .output()
        .expect("failed to execute dirp")
}

// ── check: basic pass/fail ──────────────────────────────────────────

#[test]
fn check_has_cargo_toml_pass() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]").unwrap();

    let out = dirp_check(tmp.path(), &["dp-1000"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(out.status.success());
    assert!(stdout.contains("dp-1000 (has_cargo_toml) ... PASS"));
}

#[test]
fn check_has_cargo_toml_fail() {
    let tmp = tempfile::tempdir().unwrap();

    let out = dirp_check(tmp.path(), &["dp-1000"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(out.status.success());
    assert!(stdout.contains("dp-1000 (has_cargo_toml) ... FAIL"));
}

#[test]
fn check_has_pyproject_toml_pass() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("pyproject.toml"), "[project]").unwrap();

    let out = dirp_check(tmp.path(), &["dp-1001"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(out.status.success());
    assert!(stdout.contains("dp-1001 (has_pyproject_toml) ... PASS"));
}

#[test]
fn check_has_pyproject_toml_fail() {
    let tmp = tempfile::tempdir().unwrap();

    let out = dirp_check(tmp.path(), &["dp-1001"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(out.status.success());
    assert!(stdout.contains("dp-1001 (has_pyproject_toml) ... FAIL"));
}

// ── check: dependency resolution (dp-1002 depends on dp-1000) ───────

#[test]
fn check_rust_workspace_pass() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(
        tmp.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"a\"]\n",
    )
    .unwrap();

    let out = dirp_check(tmp.path(), &["dp-1002"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(out.status.success());
    assert!(stdout.contains("dp-1002 (rust_workspace) ... PASS"));
}

#[test]
fn check_rust_workspace_fail_no_workspace_section() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname = \"foo\"\n").unwrap();

    let out = dirp_check(tmp.path(), &["dp-1002"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(out.status.success());
    assert!(stdout.contains("dp-1002 (rust_workspace) ... FAIL"));
}

#[test]
fn check_rust_workspace_fail_no_cargo_toml() {
    let tmp = tempfile::tempdir().unwrap();

    let out = dirp_check(tmp.path(), &["dp-1002"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(out.status.success());
    assert!(
        stdout.contains("dp-1002 (rust_workspace) ... FAIL"),
        "stdout: {stdout}"
    );
    assert!(stdout.contains("no Cargo.toml"));
}

// ── check: deprecated predicate ─────────────────────────────────────

#[test]
fn check_deprecated_shows_marker() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]").unwrap();

    let out = dirp_check(tmp.path(), &["dp-1003"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(out.status.success());
    assert!(stdout.contains("DEPRECATED: use dp-1000"));
}

// ── check: multiple predicates at once ──────────────────────────────

#[test]
fn check_multiple_predicates() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]").unwrap();
    fs::write(tmp.path().join("pyproject.toml"), "[project]").unwrap();

    let out = dirp_check(tmp.path(), &["dp-1000", "dp-1001"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(out.status.success());
    assert!(stdout.contains("dp-1000 (has_cargo_toml) ... PASS"));
    assert!(stdout.contains("dp-1001 (has_pyproject_toml) ... PASS"));
}

#[test]
fn check_output_order_matches_input_order() {
    let tmp = tempfile::tempdir().unwrap();

    let out = dirp_check(tmp.path(), &["dp-1001", "dp-1000"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    let pos_1001 = stdout.find("dp-1001").unwrap();
    let pos_1000 = stdout.find("dp-1000").unwrap();
    assert!(
        pos_1001 < pos_1000,
        "output should follow input order, got: {stdout}"
    );
}

// ── check: error cases ──────────────────────────────────────────────

#[test]
fn check_no_ids_errors() {
    let out = dirp_bin()
        .arg("check")
        .output()
        .expect("failed to execute dirp");

    assert!(!out.status.success());
}

#[test]
fn check_invalid_id_format() {
    let tmp = tempfile::tempdir().unwrap();

    let out = dirp_check(tmp.path(), &["1000"]);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(!out.status.success());
    assert!(stderr.contains("invalid predicate ID format"));
}

#[test]
fn check_unknown_predicate() {
    let tmp = tempfile::tempdir().unwrap();

    let out = dirp_check(tmp.path(), &["dp-999999"]);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(!out.status.success());
    assert!(stderr.contains("unknown predicate"));
}

// ── check: ERROR output when a predicate returns Err ────────────────

#[test]
fn check_error_output() {
    let tmp = tempfile::tempdir().unwrap();

    let out = dirp_check(tmp.path(), &["dp-1004"]);
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(out.status.success());
    assert!(
        stdout.contains("dp-1004 (always_error) ... ERROR"),
        "stdout: {stdout}"
    );
    assert!(stdout.contains("intentional error for testing"));
}

// ── export ──────────────────────────────────────────────────────────

#[test]
fn export_returns_valid_json() {
    let out = dirp_bin()
        .arg("export")
        .output()
        .expect("failed to execute dirp");
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(out.status.success());
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("invalid JSON");
    assert!(parsed.is_array());
}

#[test]
fn export_contains_all_predicates() {
    let out = dirp_bin()
        .arg("export")
        .output()
        .expect("failed to execute dirp");
    let stdout = String::from_utf8_lossy(&out.stdout);

    let parsed: Vec<serde_json::Value> = serde_json::from_str(&stdout).unwrap();
    let ids: Vec<u64> = parsed.iter().map(|v| v["id"].as_u64().unwrap()).collect();

    assert!(ids.contains(&1000));
    assert!(ids.contains(&1001));
    assert!(ids.contains(&1002));
    assert!(ids.contains(&1003));
    assert!(ids.contains(&1004));
}

#[test]
fn export_sorted_by_id() {
    let out = dirp_bin()
        .arg("export")
        .output()
        .expect("failed to execute dirp");
    let stdout = String::from_utf8_lossy(&out.stdout);

    let parsed: Vec<serde_json::Value> = serde_json::from_str(&stdout).unwrap();
    let ids: Vec<u64> = parsed.iter().map(|v| v["id"].as_u64().unwrap()).collect();
    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ids, sorted);
}

#[test]
fn export_predicate_fields() {
    let out = dirp_bin()
        .arg("export")
        .output()
        .expect("failed to execute dirp");
    let stdout = String::from_utf8_lossy(&out.stdout);

    let parsed: Vec<serde_json::Value> = serde_json::from_str(&stdout).unwrap();
    let dp1000 = parsed.iter().find(|v| v["id"] == 1000).unwrap();

    assert_eq!(dp1000["name"], "has_cargo_toml");
    assert_eq!(dp1000["lite"], true);
    assert!(dp1000["deprecated"].is_null());
    assert_eq!(dp1000["after"], serde_json::json!([]));

    let dp1002 = parsed.iter().find(|v| v["id"] == 1002).unwrap();
    assert_eq!(dp1002["after"], serde_json::json!([1000]));
    assert_eq!(dp1002["lite"], false);

    let dp1003 = parsed.iter().find(|v| v["id"] == 1003).unwrap();
    assert_eq!(dp1003["deprecated"], 1000);
}
