use std::path::PathBuf;

use clap::{Parser, Subcommand};
use dirp::{
    all_predicates, export_metadata, print_results, resolve_execution_order, run_predicates,
    DpContext,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Parser)]
#[command(
    name = "dirp",
    version,
    about = "Directory Predicates — check if a directory satisfies directory predicates"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check one or more directory predicates against the current working directory
    Check {
        /// Predicate IDs to check (e.g. dp-1000 dp-1001 dp-1002)
        ids: Vec<String>,
    },
    /// Run all lite predicates against the current working directory
    Analyze,
    /// Run as a Claude Code Stop hook, checking predicates and outputting decision JSON
    CcHook {
        /// Predicate IDs to check (e.g. dp-1000 dp-1001)
        ids: Vec<String>,
    },
    /// Export all predicate metadata as JSON
    Export,
}

#[derive(Deserialize)]
struct HookInput {
    cwd: Option<String>,
}

fn parse_dp_id(s: &str) -> Result<u32, String> {
    s.strip_prefix("dp-")
        .ok_or_else(|| format!("invalid predicate ID format: {s:?} (expected dp-N)"))?
        .parse::<u32>()
        .map_err(|e| format!("invalid predicate ID: {s:?}: {e}"))
}

fn main() {
    let cli = Cli::try_parse().unwrap_or_else(|e| {
        e.print().ok();
        std::process::exit(1);
    });

    match cli.command {
        Commands::Check { ids } => {
            if ids.is_empty() {
                eprintln!("error: no predicate IDs provided");
                std::process::exit(1);
            }

            let predicates = all_predicates();
            let target_ids: Vec<u32> = ids
                .iter()
                .map(|s| {
                    parse_dp_id(s).unwrap_or_else(|e| {
                        eprintln!("error: {e}");
                        std::process::exit(1);
                    })
                })
                .collect();

            let order = resolve_execution_order(&target_ids, &predicates).unwrap_or_else(|e| {
                eprintln!("error: {e}");
                std::process::exit(1);
            });

            let ctx = DpContext {
                path: PathBuf::from("."),
            };

            let results = run_predicates(&order, &predicates, &ctx);
            print_results(&target_ids, &results, &predicates);
        }
        Commands::Analyze => {
            let predicates = all_predicates();
            let mut target_ids: Vec<u32> = predicates
                .values()
                .filter(|p| p.lite)
                .map(|p| p.id)
                .collect();
            target_ids.sort();

            let order = resolve_execution_order(&target_ids, &predicates).unwrap_or_else(|e| {
                eprintln!("error: {e}");
                std::process::exit(1);
            });

            let ctx = DpContext {
                path: PathBuf::from("."),
            };

            let results = run_predicates(&order, &predicates, &ctx);
            print_results(&target_ids, &results, &predicates);
        }
        Commands::CcHook { ids } => {
            if ids.is_empty() {
                eprintln!("error: no predicate IDs provided");
                std::process::exit(1);
            }

            let hook_input: HookInput =
                serde_json::from_reader(std::io::stdin()).unwrap_or(HookInput { cwd: None });

            let dir = hook_input
                .cwd
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));

            let predicates = all_predicates();
            let target_ids: Vec<u32> = ids
                .iter()
                .map(|s| {
                    parse_dp_id(s).unwrap_or_else(|e| {
                        eprintln!("error: {e}");
                        std::process::exit(1);
                    })
                })
                .collect();

            let order = resolve_execution_order(&target_ids, &predicates).unwrap_or_else(|e| {
                eprintln!("error: {e}");
                std::process::exit(1);
            });

            let ctx = DpContext { path: dir };
            let results = run_predicates(&order, &predicates, &ctx);

            // Collect failures and count passes
            let mut failures: Vec<String> = Vec::new();
            let mut pass_count = 0;
            for &id in &target_ids {
                let pred = predicates[&id];
                match &results[&id] {
                    Ok(outcome) if outcome.verdict => {
                        pass_count += 1;
                    }
                    Ok(outcome) => {
                        let mut msg = format!("- dp-{id} ({}): FAIL", pred.name);
                        if let Some(reason) = &outcome.reason {
                            msg.push_str(&format!(" — {reason}"));
                        }
                        msg.push_str(&format!("\n  Description: {}", pred.description));
                        failures.push(msg);
                    }
                    Err(e) => {
                        let mut msg = format!("- dp-{id} ({}): ERROR — {e}", pred.name);
                        msg.push_str(&format!("\n  Description: {}", pred.description));
                        failures.push(msg);
                    }
                }
            }

            let total = target_ids.len();
            if failures.is_empty() {
                println!("{}", json!({}));
            } else {
                let reason = format!(
                    "The following directory predicates failed:\n\n{}\n\n{}/{} predicates passed. Please fix the failing predicates before finishing.",
                    failures.join("\n"),
                    pass_count,
                    total
                );
                println!(
                    "{}",
                    json!({
                        "decision": "block",
                        "reason": reason
                    })
                );
            }
        }
        Commands::Export => {
            let metas = export_metadata();
            let json = serde_json::to_string_pretty(&metas).unwrap();
            println!("{json}");
        }
    }
}
