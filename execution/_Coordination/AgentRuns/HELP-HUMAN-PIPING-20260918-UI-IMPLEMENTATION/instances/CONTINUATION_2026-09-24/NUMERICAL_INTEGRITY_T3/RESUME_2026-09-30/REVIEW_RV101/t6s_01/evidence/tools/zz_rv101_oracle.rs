//! RV101 reviewer oracle (scratch only; placed in the reviewer's archive copy, never in the slice tree).
//! Env-driven; with no env var set it does nothing.
//! - RV101_DERIVE_IN=<dir>: for every `<stem>.input.json` ({base, model, source, origin}) writes
//!   `<stem>.rust.json` (canonical JSON of Rust `derive_document`, request None) or `<stem>.rust.err`;
//!   and, when `<stem>.ts.json` exists (the TypeScript derivative), `<stem>.rust_validate_ts.txt`
//!   with Rust `validate_document(ts, source)`'s verdict.
//! - RV101_EXP_IN=<file>: one 16-hex-digit binary64 word per line; writes `<file>.rust.txt` with
//!   Rust `{:e}` of each word.
//! - RV101_DISC_IN=<file>: JSON array of [kind, unit, class, bits-or-null]; writes `<file>.rust.json`
//!   with Rust `class_disclosure`'s [code, message] or null for each.
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_result_export::{derivative as d, retained_precision as rp};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

#[test]
fn rv101_oracle() {
    if let Ok(dir) = std::env::var("RV101_DERIVE_IN") {
        let mut n = 0;
        let mut entries: Vec<_> = fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()).collect();
        entries.sort();
        for path in entries {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let Some(stem) = name.strip_suffix(".input.json") else { continue };
            let input: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
            let out_ok = Path::new(&dir).join(format!("{stem}.rust.json"));
            let out_err = Path::new(&dir).join(format!("{stem}.rust.err"));
            match d::derive_document(
                input["base"].clone(),
                &input["model"],
                &input["source"],
                input["origin"].clone(),
                None,
            ) {
                Ok(doc) => fs::write(out_ok, canonical_json(&doc)).unwrap(),
                Err(e) => fs::write(out_err, e).unwrap(),
            }
            let ts = Path::new(&dir).join(format!("{stem}.ts.json"));
            if ts.exists() {
                let ts_doc: Value = serde_json::from_str(&fs::read_to_string(&ts).unwrap()).unwrap();
                let verdict = match d::validate_document(&ts_doc, &input["source"]) {
                    Ok(()) => "OK".to_string(),
                    Err(e) => e,
                };
                fs::write(Path::new(&dir).join(format!("{stem}.rust_validate_ts.txt")), verdict).unwrap();
            }
            n += 1;
        }
        eprintln!("RV101 derive inputs: {n}");
    }
    if let Ok(file) = std::env::var("RV101_EXP_IN") {
        let text = fs::read_to_string(&file).unwrap();
        let mut out = String::new();
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let bits = u64::from_str_radix(line.trim(), 16).unwrap();
            out.push_str(&format!("{:e}\n", f64::from_bits(bits)));
        }
        fs::write(format!("{file}.rust.txt"), out).unwrap();
        eprintln!("RV101 exp words: {}", text.lines().count());
    }
    if let Ok(file) = std::env::var("RV101_DISC_IN") {
        let cases: Vec<Value> = serde_json::from_str(&fs::read_to_string(&file).unwrap()).unwrap();
        let mut out = Vec::new();
        for c in &cases {
            let kind = c[0].as_str().unwrap();
            let unit = c[1].as_str().unwrap();
            let class = match c[2].as_str().unwrap() {
                "relative_verified" => Some(rp::AccuracyClass::RelativeVerified),
                "absolute_verified" => Some(rp::AccuracyClass::AbsoluteVerified {
                    bound_bits: u64::from_str_radix(c[3].as_str().unwrap(), 16).unwrap(),
                }),
                "input_derived" => Some(rp::AccuracyClass::InputDerived),
                "non_quantity" => Some(rp::AccuracyClass::NonQuantity),
                "not_covered" => Some(rp::AccuracyClass::NotCovered),
                "none" => None,
                other => panic!("class {other}"),
            };
            out.push(match d::class_disclosure(kind, unit, class.as_ref()) {
                Some((code, message)) => json!([code, message]),
                None => Value::Null,
            });
        }
        fs::write(format!("{file}.rust.json"), serde_json::to_string(&out).unwrap()).unwrap();
        eprintln!("RV101 disclosure cases: {}", cases.len());
    }
}
