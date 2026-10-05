//! RV85 (disposable, archive-only): fixture sweep over the public API, run
//! identically in the base and candidate archives. Not maintained code.
use open_pipe_stress_product_physics::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::panic::{catch_unwind, AssertUnwindSafe};

fn sha(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }

fn env_out(r: Result<MechanicsEnvelope, String>) -> String {
    match r {
        Ok(e) => format!("ok:{}", sha(&serde_json::to_vec(&e).unwrap())),
        Err(s) => format!("err:{}", s),
    }
}
fn retained_out(r: Result<RetainedPreviewOutput, String>) -> (String, String) {
    match r {
        Ok(o) => {
            let adm = format!("{:?}", o.admission());
            let flags = o.admission().map(|a| format!("complete={} unknown={}", a.census_complete(), a.required_unknown_terms().len())).unwrap_or_else(|| "none".into());
            (format!("ok:{}", sha(&serde_json::to_vec(o.envelope()).unwrap())), format!("adm:{} {}", sha(adm.as_bytes()), flags))
        }
        Err(s) => (format!("err:{}", s), "adm:-".into()),
    }
}
fn guarded<T>(f: impl FnOnce() -> T) -> Result<T, ()> { catch_unwind(AssertUnwindSafe(f)).map_err(|_| ()) }

fn inputs() -> Vec<(String, Value)> {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let list = std::fs::read_to_string(std::env::var("RV85_SWEEP_LIST").unwrap()).unwrap();
    let mut out = Vec::new();
    for line in list.lines().filter(|l| !l.trim().is_empty()) {
        let (kind, rel) = line.split_once(' ').unwrap();
        let v: Value = serde_json::from_str(&std::fs::read_to_string(p.join(rel)).unwrap()).unwrap();
        match kind {
            "REQ" => out.push((rel.to_string(), v)),
            "MOD" => out.push((format!("{{model}}:{rel}"), json!({"model": v}))),
            _ => panic!("kind"),
        }
    }
    // Synthetic variants of the milestone (each exercises a distinct early branch).
    let m: Value = serde_json::from_str(&std::fs::read_to_string(p.join("fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap()).unwrap();
    let mut imposed = m.clone();
    imposed["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap()
        .push(json!({"id":"rv85-imposed","target":{"type":"support","support":"rv85-s"},"category":"displacement"}));
    out.push(("synthetic:imposed_displacement".into(), imposed));
    let mut deep = m.clone();
    let mut child = Value::Null;
    for _ in 0..70 { child = Value::Array(vec![child]); }
    deep["rv85_unknown_depth"] = child;
    out.push(("synthetic:deep_unknown_field".into(), deep));
    out.push(("synthetic:unparseable".into(), json!({"model": 5})));
    let mut relabel = m.clone();
    relabel["model"]["load_cases"][0]["label"] = json!("RV85 \u{00e9}tiquette \u{2014} 2");
    out.push(("synthetic:relabelled".into(), relabel));
    let mut doubled = m.clone();
    let c0 = doubled["model"]["load_cases"][0].clone();
    let mut c1 = c0.clone();
    c1["id"] = json!(format!("{}-rv85", c0["id"].as_str().unwrap()));
    doubled["model"]["load_cases"].as_array_mut().unwrap().push(c1);
    out.push(("synthetic:two_cases".into(), doubled));
    let mut no_materials = m.clone();
    no_materials.as_object_mut().unwrap().remove("materials");
    out.push(("synthetic:no_request_materials".into(), no_materials));
    out
}

#[test]
fn zz_rv85_sweep() {
    let out = std::env::var("RV85_SWEEP_OUT").unwrap();
    let mut rows = Vec::new();
    for (name, raw) in inputs() {
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let m = mode.as_str();
            // 1 typed default (mode-free; recorded under each mode label)
            let r = guarded(|| match serde_json::from_value::<LinearStaticPreviewRequest>(raw.clone()) {
                Ok(t) => env_out(Ok(run_linear_static_preview(t))),
                Err(e) => format!("parse:{e}"),
            }).unwrap_or_else(|_| "PANIC".into());
            rows.push(format!("{name}\ttyped_default\t{m}\t{r}\t-"));
            // 2 typed mode
            let r = guarded(|| match serde_json::from_value::<LinearStaticPreviewRequest>(raw.clone()) {
                Ok(t) => env_out(Ok(run_linear_static_preview_with_mode(t, mode))),
                Err(e) => format!("parse:{e}"),
            }).unwrap_or_else(|_| "PANIC".into());
            rows.push(format!("{name}\ttyped_mode\t{m}\t{r}\t-"));
            // 3 value
            let r = guarded(|| env_out(run_linear_static_preview_value_with_mode(raw.clone(), mode))).unwrap_or_else(|_| "PANIC".into());
            rows.push(format!("{name}\tvalue\t{m}\t{r}\t-"));
            // 4 retained Direct
            let (r, a) = guarded(|| retained_out(run_linear_static_preview_value_with_retained_direct(raw.clone(), mode))).unwrap_or_else(|_| ("PANIC".into(), "-".into()));
            rows.push(format!("{name}\tretained_direct\t{m}\t{r}\t{a}"));
            // 5 retained Headless (roots as runner/headless builds them)
            let invocation = json!({"request": &raw, "solver_mode": m});
            let request_id = format!("rv85-{name}");
            let (r, a) = guarded(|| {
                let ctx = RetainedHeadlessContext::from_borrowed_roots(&raw, &invocation, &request_id);
                retained_out(run_linear_static_preview_value_with_retained_headless(raw.clone(), mode, ctx))
            }).unwrap_or_else(|_| ("PANIC".into(), "-".into()));
            rows.push(format!("{name}\tretained_headless\t{m}\t{r}\t{a}"));
        }
    }
    std::fs::write(&out, rows.join("\n") + "\n").unwrap();
}
