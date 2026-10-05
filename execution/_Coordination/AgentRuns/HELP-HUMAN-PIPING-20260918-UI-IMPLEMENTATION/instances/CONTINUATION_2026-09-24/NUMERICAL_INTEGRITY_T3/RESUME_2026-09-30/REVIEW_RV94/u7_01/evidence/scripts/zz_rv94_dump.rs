//! RV94 scratch-only dumper (never committed): reads RV94_INPUTS (JSONL), writes RV94_OUT.
use open_pipe_stress_result_export::{retained_precision as rp, semantic_contract as s};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::io::{BufRead, Write};

fn class_name(c: &rp::AccuracyClass) -> &'static str {
    match c {
        rp::AccuracyClass::RelativeVerified => "relative_verified",
        rp::AccuracyClass::AbsoluteVerified { .. } => "absolute_verified",
        rp::AccuracyClass::InputDerived => "input_derived",
        rp::AccuracyClass::NonQuantity => "non_quantity",
        rp::AccuracyClass::NotCovered => "not_covered",
    }
}

#[test]
#[ignore]
fn zz_rv94_dump() {
    let inputs = std::env::var("RV94_INPUTS").expect("RV94_INPUTS");
    let out_path = std::env::var("RV94_OUT").expect("RV94_OUT");
    let mut out = std::fs::File::create(out_path).unwrap();
    let file = std::io::BufReader::new(std::fs::File::open(inputs).unwrap());
    let mut n = 0;
    for line in file.lines() {
        let r: Value = serde_json::from_str(&line.unwrap()).unwrap();
        let src = &r["source"];
        let inv = if r["invocation"].is_null() { None } else { Some(&r["invocation"]) };
        let req: Vec<Value> = r["requested"].as_array().unwrap().clone();
        let reader = match rp::validate(src, inv) {
            Ok(v) => {
                let mut counts = serde_json::Map::new();
                for c in &v.classifications {
                    let k = class_name(&c.class);
                    let n = counts.get(k).and_then(Value::as_u64).unwrap_or(0) + 1;
                    counts.insert(k.into(), json!(n));
                }
                let digest = format!("{:x}", Sha256::digest(format!("{:?}", v.classifications).as_bytes()));
                json!({"ok": true, "invocation_bound": v.invocation_bound, "numerical_eligible": v.numerical_eligible,
                       "publication_sha256": v.publication_sha256, "class_counts": counts, "classes_sha256": digest})
            }
            Err(e) => json!({"ok": false, "gate": e.gate, "code": e.code}),
        };
        let token = s::numerical_use_standing_with_context(src, &req, inv);
        let summary = s::classification_summary(src, inv);
        writeln!(out, "{}", json!({"id": r["id"], "reader": reader, "token": token, "summary": summary})).unwrap();
        n += 1;
    }
    eprintln!("RV94 dumped {n}");
}
