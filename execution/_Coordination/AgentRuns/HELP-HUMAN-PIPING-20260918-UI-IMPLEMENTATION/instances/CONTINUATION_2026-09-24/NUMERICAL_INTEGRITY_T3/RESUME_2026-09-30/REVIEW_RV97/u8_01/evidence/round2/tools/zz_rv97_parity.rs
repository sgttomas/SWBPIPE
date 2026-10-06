//! RV97 round-2 probe (review copy only; never committed): the Rust reader on RV97's identical
//! documents. Reads every `*.json` in RV97_DOCS ({"label","source","invocation"|null}) and writes one
//! JSON line per document to RV97_OUT: the first failure (gate, code), or the eligibility and the
//! classifications in a reader-neutral form.
use open_pipe_stress_result_export::retained_precision::{validate, AccuracyClass};
use serde_json::{json, Value};
use std::io::Write;

#[test]
fn zz_rv97_parity() {
    let dir = std::env::var("RV97_DOCS").expect("RV97_DOCS");
    let out = std::env::var("RV97_OUT").expect("RV97_OUT");
    let mut files: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()).filter(|p| p.extension().map_or(false, |x| x == "json")).collect();
    files.sort();
    let mut w = std::io::BufWriter::new(std::fs::File::create(&out).unwrap());
    let hex = |b: u64| format!("{b:016x}");
    for path in &files {
        let doc: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        let invocation = if doc["invocation"].is_null() { None } else { Some(&doc["invocation"]) };
        let line = match validate(&doc["source"], invocation) {
            Ok(v) => {
                let rows: Vec<Value> = v.classifications.iter().map(|c| {
                    let (class, bound) = match c.class {
                        AccuracyClass::RelativeVerified => ("relative_verified", None),
                        AccuracyClass::AbsoluteVerified { bound_bits } => ("absolute_verified", Some(hex(bound_bits))),
                        AccuracyClass::InputDerived => ("input_derived", None),
                        AccuracyClass::NonQuantity => ("non_quantity", None),
                        AccuracyClass::NotCovered => ("not_covered", None),
                    };
                    json!([c.result_id, class, hex(c.normalized_bits), c.scale_bits.map(hex), bound])
                }).collect();
                json!({"label": doc["label"], "ok": true, "bound": v.invocation_bound, "eligible": v.numerical_eligible,
                    "publication_sha256": v.publication_sha256, "classifications": rows})
            }
            Err(e) => json!({"label": doc["label"], "ok": false, "gate": e.gate, "code": e.code}),
        };
        writeln!(w, "{}", serde_json::to_string(&line).unwrap()).unwrap();
    }
    println!("RV97_RUST documents={}", files.len());
}
