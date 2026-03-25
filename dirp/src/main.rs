use std::path::PathBuf;

use clap::{Parser, Subcommand};
use dirp::{
    all_predicates, export_metadata, print_results, resolve_execution_order, run_predicates,
    DpContext,
};

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

        /// Run all lite predicates
        #[arg(long)]
        lite: bool,
    },
    /// Export all predicate metadata as JSON
    Export,
}

fn parse_dp_id(s: &str) -> Result<u32, String> {
    s.strip_prefix("dp-")
        .ok_or_else(|| format!("invalid predicate ID format: {s:?} (expected dp-N)"))?
        .parse::<u32>()
        .map_err(|e| format!("invalid predicate ID: {s:?}: {e}"))
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { ids, lite } => {
            let predicates = all_predicates();

            let target_ids: Vec<u32> = if lite {
                let mut lite_ids: Vec<u32> = predicates
                    .values()
                    .filter(|p| p.lite)
                    .map(|p| p.id)
                    .collect();
                lite_ids.sort();
                lite_ids
            } else {
                if ids.is_empty() {
                    eprintln!("error: no predicate IDs provided");
                    std::process::exit(1);
                }
                ids.iter()
                    .map(|s| {
                        parse_dp_id(s).unwrap_or_else(|e| {
                            eprintln!("error: {e}");
                            std::process::exit(1);
                        })
                    })
                    .collect()
            };

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
        Commands::Export => {
            let metas = export_metadata();
            let json = serde_json::to_string_pretty(&metas).unwrap();
            println!("{json}");
        }
    }
}
