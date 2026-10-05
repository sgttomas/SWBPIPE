//! RV95 scratch (never committed): the Rust reader on the live milestone successors,
//! with and without the invocation and with a mode-swapped invocation.
use open_pipe_stress_result_export::retained_precision::validate;
use serde_json::{json, Value};
#[test]
#[ignore]
fn zz_rv95_dump() {
    let dir = std::env::var("RV95_IN").expect("RV95_IN");
    let mut paths: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()).filter(|p| p.extension().map_or(false, |e| e == "json")).collect();
    paths.sort();
    let mut out = Vec::new();
    for p in paths {
        let doc: Value = serde_json::from_slice(&std::fs::read(&p).unwrap()).unwrap();
        let (src, inv) = (&doc["source"], &doc["invocation"]);
        let mut other = inv.clone();
        other["solver_mode"] = json!(if inv["solver_mode"] == "sparse_interactive" { "dense_scrutiny" } else { "sparse_interactive" });
        for (label, arg) in [("with_invocation", Some(inv)), ("without_invocation", None), ("mode_swapped", Some(&other))] {
            let file = p.file_name().unwrap().to_string_lossy().to_string();
            out.push(match validate(src, arg) {
                Ok(v) => json!({"file": file, "case": label, "ok": true, "invocation_bound": v.invocation_bound, "numerical_eligible": v.numerical_eligible,
                    "publication_sha256": v.publication_sha256, "n_classifications": v.classifications.len()}),
                Err(e) => json!({"file": file, "case": label, "ok": false, "error": format!("{e:?}")}),
            });
        }
    }
    std::fs::write(std::env::var("RV95_OUT").unwrap(), serde_json::to_string_pretty(&out).unwrap()).unwrap();
}
