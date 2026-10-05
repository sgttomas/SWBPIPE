//! RV93 (review of U3 grant 2): scratch-only route sweep, compiled identically into the base
//! (0c7827b6ad) and candidate (664f8df7b7) copies. Every request-shaped fixture plus RV93's
//! own milestone variants, through five routes in both modes; each row records the sha256 of
//! the route's bytes and, for the retained entries, the full admission report (Debug), the
//! private W1 result and the publication's bytes. Never committed.
use super::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};

fn sha(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn files(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut v: Vec<_> = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
    v.sort();
    for p in v {
        if p.is_dir() { files(&p, out) } else if p.extension().is_some_and(|e| e == "json") { out.push(p) }
    }
}
fn caught<T>(f: impl FnOnce() -> T) -> Result<T, String> { catch_unwind(AssertUnwindSafe(f)).map_err(|_| "PANIC".to_string()) }
fn env_bytes(e: &MechanicsEnvelope) -> Vec<u8> { serde_json::to_vec(e).unwrap() }
fn retained(out: Result<Result<RetainedPreviewOutput, String>, String>) -> String {
    let o = match out { Err(p) => return p, Ok(Err(e)) => return format!("ERR:{e}"), Ok(Ok(o)) => o };
    let report = format!("{:?}", o.admission());
    let w1 = format!("{:?}", o.retained().map(|r| r.as_ref().map(|_| "successor")));
    let env = sha(&env_bytes(o.envelope()));
    let (kind, published) = match o.into_publication() {
        RetainedPublication::Successor(v) => ("Successor", serde_json::to_vec(&v).unwrap()),
        RetainedPublication::Ordinary(e) => ("Ordinary", env_bytes(&e)),
    };
    format!("{kind}\tpub={}\tenv={env}\tw1={w1}\treport_sha={}\treport={report}", sha(&published), sha(report.as_bytes()))
}
fn variants() -> Vec<(String, Value)> {
    let m: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap();
    let mut out = Vec::new();
    let mut add = |name: &str, f: &dyn Fn(&mut Value)| { let mut v = m.clone(); f(&mut v); out.push((format!("rv93_variant/{name}"), v)); };
    add("milestone", &|_| {});
    add("schema_0.2.0", &|v| v["model"]["schema_version"] = json!("0.2.0"));
    add("schema_0.3.0", &|v| v["model"]["schema_version"] = json!("0.3.0"));
    add("two_cases", &|v| { let mut c = v["model"]["load_cases"][0].clone(); c["id"] = json!("case-b"); v["model"]["load_cases"].as_array_mut().unwrap().push(c); });
    add("combination", &|v| v["model"]["combinations"] = json!([{"id":"k","basis":"mechanics","terms":[{"load_case":"case","factor":1.5}]}]));
    add("support_family", &|v| v["model"]["supports"][0]["family"] = json!("hanger_rigid"));
    add("bad_document_kind", &|v| v["model"]["document_kind"] = json!("not.a.kind"));
    add("bad_category", &|v| v["model"]["load_cases"][0]["primitive_loads"][0]["category"] = json!("not_a_category"));
    add("no_supports", &|v| v["model"]["supports"] = json!([]));
    add("spring_only", &|v| { let s = v["model"]["supports"].as_array().unwrap()[1..].to_vec(); v["model"]["supports"] = json!(s); });
    add("tiny_spring", &|v| v["model"]["supports"][1]["stiffness"]["value"]["value"] = json!(1e-300));
    add("loads_x2", &|v| for l in v["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap() { let x = l["magnitude"]["value"].as_f64().unwrap(); l["magnitude"]["value"] = json!(x * 2.0); });
    add("first_load_only", &|v| { let l = v["model"]["load_cases"][0]["primitive_loads"][0].clone(); v["model"]["load_cases"][0]["primitive_loads"] = json!([l]); });
    add("case_renamed", &|v| v["model"]["load_cases"][0]["id"] = json!("case-renamed"));
    add("load_force_dimension", &|v| { v["model"]["load_cases"][0]["primitive_loads"][0]["dimension"] = json!("temperature"); });
    add("imposed_displacement", &|v| v["model"]["load_cases"][0]["primitive_loads"][0]["category"] = json!("imposed_displacement"));
    out
}

#[test]
#[ignore]
fn zz_rv93_route_sweep() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
    let mut paths = Vec::new();
    files(&root, &mut paths);
    let mut inputs: Vec<(String, Value)> = Vec::new();
    for p in paths {
        let Ok(v) = serde_json::from_slice::<Value>(&std::fs::read(&p).unwrap()) else { continue };
        let raw = if v.get("model").and_then(|m| m.get("load_cases")).is_some() { v }
            else if v.get("load_cases").is_some() && v.get("nodes").is_some() { json!({"model": v}) }
            else { continue };
        inputs.push((p.strip_prefix(&root).unwrap().display().to_string(), raw));
    }
    inputs.extend(variants());
    let mut rows = Vec::new();
    for (name, raw) in &inputs {
        let typed = serde_json::from_value::<LinearStaticPreviewRequest>(raw.clone());
        let td = match &typed { Ok(t) => caught(|| sha(&env_bytes(&run_linear_static_preview(t.clone())))).unwrap_or_else(|e| e), Err(_) => "TYPED_PARSE_ERR".into() };
        rows.push(format!("{name}\ttyped_default\t-\t{td}"));
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let m = mode.as_str();
            let tm = match &typed { Ok(t) => caught(|| sha(&env_bytes(&run_linear_static_preview_with_mode(t.clone(), mode)))).unwrap_or_else(|e| e), Err(_) => "TYPED_PARSE_ERR".into() };
            rows.push(format!("{name}\ttyped_mode\t{m}\t{tm}"));
            let value = match caught(|| run_linear_static_preview_value_with_mode(raw.clone(), mode)) {
                Ok(Ok(e)) => sha(&env_bytes(&e)), Ok(Err(e)) => format!("ERR:{e}"), Err(p) => p,
            };
            rows.push(format!("{name}\tvalue\t{m}\t{value}"));
            rows.push(format!("{name}\tdirect\t{m}\t{}", retained(caught(|| run_linear_static_preview_value_with_retained_direct(raw.clone(), mode)))));
            let invocation = json!({"request": raw.clone(), "solver_mode": m});
            let id = String::from("rv93-sweep");
            rows.push(format!("{name}\theadless\t{m}\t{}", retained(caught(|| run_linear_static_preview_value_with_retained_headless(raw.clone(), mode,
                RetainedHeadlessContext::from_borrowed_roots(raw, &invocation, &id))))));
        }
    }
    let out = std::env::var("RV93_SWEEP").expect("RV93_SWEEP");
    std::fs::write(&out, rows.join("\n") + "\n").unwrap();
    println!("RV93_SWEEP inputs={} rows={}", inputs.len(), rows.len());
}
