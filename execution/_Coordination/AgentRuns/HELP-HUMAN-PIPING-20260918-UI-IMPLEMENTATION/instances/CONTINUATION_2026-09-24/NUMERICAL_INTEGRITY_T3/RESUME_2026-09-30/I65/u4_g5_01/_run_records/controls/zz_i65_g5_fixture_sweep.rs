//! I65 U4 G5, control 1 evidence (scratch copies only, never committed): every
//! request-shaped fixture through every public route, in both modes, hashed. The
//! admission report is printed field by field over its public fields, which base
//! and candidate share; any private G5 field is excluded by construction.
use open_pipe_stress_product_physics::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};

fn sha(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }
fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() { walk(&p, out) } else if p.extension().map_or(false, |e| e == "json") { out.push(p) }
    }
}
fn guard<F: FnOnce() -> String>(f: F) -> String {
    catch_unwind(AssertUnwindSafe(f)).unwrap_or_else(|_| "PANIC".into())
}
fn env_bytes(e: &MechanicsEnvelope) -> String { sha(&serde_json::to_vec(e).unwrap()) }
fn report(r: Option<&RetainedAdmissionReport>) -> String {
    match r {
        None => "None".into(),
        Some(r) => format!("caller={:?} raw={:?} typed={:?} digest={:?} encoded_length={:?} headless={:?} profile={:?} allowance={:?} unknown={:?} complete={:?}",
            r.caller, r.raw, r.typed, r.captured_digest, r.captured_encoded_length, r.headless, r.profile, r.allowance,
            r.required_unknown_terms(), r.census_complete()),
    }
}
fn extra(o: &RetainedPreviewOutput) -> String { format!("successor={}", o.successor().is_some()) }

#[test]
#[ignore]
fn i65_g5_fixture_sweep() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
    let mut files = Vec::new();
    walk(&root, &mut files);
    let mut lines = Vec::new();
    let mut inputs = 0;
    for path in files {
        let rel = path.strip_prefix(&root).unwrap().display().to_string();
        let Ok(v) = serde_json::from_slice::<Value>(&std::fs::read(&path).unwrap()) else { continue };
        let raw = if v.get("model").and_then(|m| m.get("load_cases")).is_some() {
            v
        } else if v.get("load_cases").is_some() && (v.get("nodes").is_some() || v.get("components").is_some()) {
            json!({ "model": v })
        } else {
            continue;
        };
        inputs += 1;
        let typed = serde_json::from_value::<LinearStaticPreviewRequest>(raw.clone()).ok();
        lines.push(format!("{rel}\ttyped_default\t-\t{}", guard(|| match typed.clone() {
            Some(t) => env_bytes(&run_linear_static_preview(t)),
            None => "TYPED_PARSE_ERR".into(),
        })));
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let m = mode.as_str();
            lines.push(format!("{rel}\ttyped_mode\t{m}\t{}", guard(|| match typed.clone() {
                Some(t) => env_bytes(&run_linear_static_preview_with_mode(t, mode)),
                None => "TYPED_PARSE_ERR".into(),
            })));
            lines.push(format!("{rel}\tvalue_mode\t{m}\t{}", guard(|| match run_linear_static_preview_value_with_mode(raw.clone(), mode) {
                Ok(e) => env_bytes(&e),
                Err(s) => format!("ERR:{s}"),
            })));
            lines.push(format!("{rel}\tretained_direct\t{m}\t{}", guard(|| match run_linear_static_preview_value_with_retained_direct(raw.clone(), mode) {
                Ok(o) => format!("{} {} {}", env_bytes(o.envelope()), extra(&o), report(o.admission())),
                Err(s) => format!("ERR:{s}"),
            })));
            let invocation = json!({ "request": raw.clone(), "solver_mode": m });
            let request_id = String::from("i65-g5-sweep");
            lines.push(format!("{rel}\tretained_headless\t{m}\t{}", guard(|| {
                let ctx = RetainedHeadlessContext::from_borrowed_roots(&raw, &invocation, &request_id);
                match run_linear_static_preview_value_with_retained_headless(raw.clone(), mode, ctx) {
                    Ok(o) => format!("{} {} {}", env_bytes(o.envelope()), extra(&o), report(o.admission())),
                    Err(s) => format!("ERR:{s}"),
                }
            })));
        }
    }
    let out = std::env::var("I65_G5_SWEEP").expect("I65_G5_SWEEP");
    std::fs::write(&out, lines.join("\n") + "\n").unwrap();
    println!("I65_G5_SWEEP inputs={inputs} lines={}", lines.len());
}
