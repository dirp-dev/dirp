pub mod dp;

use std::collections::HashMap;
use std::path::PathBuf;

use serde::Serialize;

pub struct DpOutcome {
    pub verdict: bool,
    pub reason: Option<String>,
}

impl From<bool> for DpOutcome {
    fn from(v: bool) -> Self {
        DpOutcome {
            verdict: v,
            reason: None,
        }
    }
}

impl From<(bool, &str)> for DpOutcome {
    fn from((v, r): (bool, &str)) -> Self {
        DpOutcome {
            verdict: v,
            reason: Some(r.to_string()),
        }
    }
}

impl From<(bool, String)> for DpOutcome {
    fn from((v, r): (bool, String)) -> Self {
        DpOutcome {
            verdict: v,
            reason: Some(r),
        }
    }
}

pub type DpResult = Result<DpOutcome, String>;
pub type CheckFn = fn(&DpContext, &HashMap<u32, DpResult>) -> DpResult;

pub struct Predicate {
    pub id: u32,
    pub name: &'static str,
    pub description: &'static str,
    pub after: &'static [u32],
    pub lite: bool,
    pub deprecated: Option<u32>,
    pub check_fn: CheckFn,
}

inventory::collect!(Predicate);

pub struct DpContext {
    pub path: PathBuf,
}

#[derive(Serialize)]
pub struct PredicateMeta {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub after: Vec<u32>,
    pub lite: bool,
    pub deprecated: Option<u32>,
}

pub fn all_predicates() -> HashMap<u32, &'static Predicate> {
    let mut map = HashMap::new();
    for p in inventory::iter::<Predicate> {
        map.insert(p.id, p);
    }
    map
}

pub fn export_metadata() -> Vec<PredicateMeta> {
    let mut metas: Vec<PredicateMeta> = inventory::iter::<Predicate>()
        .map(|p| PredicateMeta {
            id: p.id,
            name: p.name.to_string(),
            description: p.description.to_string(),
            after: p.after.to_vec(),
            lite: p.lite,
            deprecated: p.deprecated,
        })
        .collect();
    metas.sort_by_key(|m| m.id);
    metas
}

/// Resolve which predicates need to run for the given target IDs.
/// Returns a topologically sorted list of predicate IDs, or an error if a cycle is detected.
pub fn resolve_execution_order(
    target_ids: &[u32],
    predicates: &HashMap<u32, &Predicate>,
) -> Result<Vec<u32>, String> {
    // Collect all needed IDs (targets + transitive deps)
    let mut needed: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut stack: Vec<u32> = target_ids.to_vec();

    while let Some(id) = stack.pop() {
        if needed.contains_key(&id) {
            continue;
        }
        let pred = predicates
            .get(&id)
            .ok_or_else(|| format!("unknown predicate: dp-{id}"))?;
        let deps: Vec<u32> = pred.after.to_vec();
        for &dep_id in &deps {
            if !needed.contains_key(&dep_id) {
                stack.push(dep_id);
            }
        }
        needed.insert(id, deps);
    }

    // Topological sort with cycle detection (Kahn's algorithm)
    // edge: dep -> id (dep must run before id), so in_degree[id] = number of deps
    let mut in_degree: HashMap<u32, usize> = needed.keys().map(|&id| (id, 0)).collect();
    for (&id, deps) in &needed {
        // id depends on each dep, so edge: dep -> id
        *in_degree.entry(id).or_insert(0) += deps.len();
    }

    let mut queue: Vec<u32> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(&id, _)| id)
        .collect();
    queue.sort(); // deterministic order

    let mut order = Vec::new();
    while let Some(id) = queue.pop() {
        order.push(id);
        // Find all nodes that depend on `id`
        for (&node, deps) in &needed {
            if deps.contains(&id) {
                let deg = in_degree.get_mut(&node).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    // Insert sorted
                    let pos = queue.binary_search(&node).unwrap_or_else(|p| p);
                    queue.insert(pos, node);
                }
            }
        }
    }

    if order.len() != needed.len() {
        let remaining: Vec<u32> = needed
            .keys()
            .filter(|id| !order.contains(id))
            .copied()
            .collect();
        return Err(format!(
            "cycle detected among predicates: {}",
            remaining
                .iter()
                .map(|id| format!("dp-{id}"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    Ok(order)
}

pub fn run_predicates(
    order: &[u32],
    predicates: &HashMap<u32, &Predicate>,
    dp_ctx: &DpContext,
) -> HashMap<u32, DpResult> {
    let mut results: HashMap<u32, DpResult> = HashMap::new();

    for &id in order {
        let pred = predicates[&id];
        let result = (pred.check_fn)(dp_ctx, &results);
        results.insert(id, result);
    }

    results
}

pub fn print_results(
    target_ids: &[u32],
    results: &HashMap<u32, DpResult>,
    predicates: &HashMap<u32, &Predicate>,
) {
    for &id in target_ids {
        let pred = predicates[&id];
        let result = &results[&id];
        match result {
            Ok(outcome) => {
                let status = if outcome.verdict { "PASS" } else { "FAIL" };
                print!("dp-{id} ({}) ... {status}", pred.name);
                if let Some(reason) = &outcome.reason {
                    print!(" — {reason}");
                }
                if let Some(new_id) = pred.deprecated {
                    print!(" (DEPRECATED: use dp-{new_id})");
                }
                println!();
            }
            Err(e) => {
                println!("dp-{id} ({}) ... ERROR — {e}", pred.name);
            }
        }
    }
}
