//! DHAT heap profiler for CachedBuild.
//!
//! Loads `poolmanager.json`, builds a `CachedBuild`, then exits.
//! DHAT records every allocation with a backtrace and writes
//! `dhat-heap.json` on exit.
//!
//! Usage:
//!   cargo run --release --features dhat-heap --bin dhat-profile
//!
//! Then open the DHAT viewer to inspect the profile:
//!   https://nnethercote.github.io/dh_view/dh_view.html

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use serde_json::Value;
use std::fs;

fn main() {
    let _profiler = dhat::Profiler::new_heap();

    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "poolmanager.json".to_string());

    eprintln!("Loading {path}...");
    let data = fs::read_to_string(&path).expect("read fixture");
    let raw: Value = serde_json::from_str(&data).expect("parse JSON");

    eprintln!("Normalizing...");
    let ast = solidity_language_server::solc::normalize_solc_output(raw, None);

    eprintln!("Building CachedBuild...");
    let build = solidity_language_server::goto::CachedBuild::new(ast, 0);

    // Print basic stats so we know it worked
    eprintln!("  decl_index: {} entries", build.decl_index.len());
    eprintln!("  nodes: {} files", build.nodes.len());
    eprintln!(
        "  nodes total: {} entries",
        build.nodes.values().map(|m| m.len()).sum::<usize>()
    );

    // Keep build alive so DHAT t-end shows the steady-state memory
    // footprint (what the LSP server actually holds).
    eprintln!("Done. DHAT will write dhat-heap.json on exit.");
    eprintln!("(build kept alive for steady-state measurement)");
    std::mem::forget(build);
    // _profiler drops here, writing the profile
}
