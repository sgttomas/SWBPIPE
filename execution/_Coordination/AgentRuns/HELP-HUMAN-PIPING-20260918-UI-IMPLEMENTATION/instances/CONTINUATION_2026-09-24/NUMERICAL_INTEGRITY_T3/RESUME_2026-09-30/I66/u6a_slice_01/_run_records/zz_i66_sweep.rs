//! I66 U6a control 1 (disposable lane test, never committed): every mechanics
//! envelope found in the committed fixtures, through the public result_export
//! carrier API that exists at base. Base and candidate outputs are compared
//! byte for byte; only successor-identity documents may differ.
use open_pipe_stress_result_export::{derivative as d, semantic_contract as s};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::io::Write;

fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().map(|e| e.path()).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            walk(&path, out);
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push(path);
        }
    }
}
fn envelopes<'a>(v: &'a Value, pointer: String, out: &mut Vec<(String, &'a Value)>) {
    match v {
        Value::Object(o) => {
            if o.get("schema_version").is_some_and(Value::is_string)
                && o.get("results").is_some_and(Value::is_array)
                && o.get("status").is_some_and(Value::is_object)
            {
                out.push((pointer.clone(), v));
            }
            for (k, child) in o {
                envelopes(child, format!("{pointer}/{}", k.replace('~', "~0").replace('/', "~1")), out);
            }
        }
        Value::Array(a) => {
            for (i, child) in a.iter().enumerate() {
                envelopes(child, format!("{pointer}/{i}"), out);
            }
        }
        _ => {}
    }
}
fn sha(v: &Value) -> String {
    format!("{:x}", Sha256::digest(serde_json::to_vec(v).unwrap()))
}
fn outcome<T>(r: Result<T, String>, ok: impl FnOnce(T) -> String) -> String {
    match r {
        Ok(t) => format!("ok:{}", ok(t)),
        Err(e) => format!("err:{e}"),
    }
}
fn derive(raw: &Value) -> Result<Value, String> {
    let base: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json"
    ))
    .unwrap();
    let model = raw["model_ref"].as_str().unwrap_or("model:absent");
    let run = raw["run_id"].as_str().unwrap_or("run:absent");
    let carrier = d::checksum(raw, "attested_headless_producer_carrier", d::reference("sweep_carrier", "i66"))?;
    let origin = json!({"origin_id":"i66-sweep","origin_class":"attested_headless_producer","qualification_ref":d::reference("sweep","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"sweep","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",run),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    d::derive_document(base, &json!({"project":{"id":model}}), raw, origin, None)
}

#[test]
#[ignore]
fn zz_i66_sweep() {
    let out_path = std::env::var("I66_SWEEP").expect("I66_SWEEP");
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut files = Vec::new();
    for dir in ["fixtures", "core/reporting/result_export/tests/fixtures", "core/product_physics/tests/fixtures", "core/runner/headless/tests"] {
        walk(&root.join(dir), &mut files);
    }
    let mut out = std::fs::File::create(out_path).unwrap();
    let (mut n, mut successor) = (0, 0);
    for path in files {
        let rel = path.strip_prefix(&root).unwrap().display().to_string();
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
        let mut found = Vec::new();
        envelopes(&doc, String::new(), &mut found);
        for (pointer, raw) in found {
            n += 1;
            let id = raw["producer"]["semantic_contract_id"].as_str().unwrap_or("-");
            let kind = if id == "openpipestress.result_semantics/0.3.0/preview-physics-retained-1" || raw.get("retained_precision").is_some() {
                successor += 1;
                "successor"
            } else {
                "existing"
            };
            let bases: Vec<Value> = raw["numerical_quality"]["cases"].as_array().map(|c| c.iter().map(|c| c["basis_ref"].clone()).collect()).unwrap_or_default();
            let full = outcome(s::for_source(raw), |(t, v)| format!("{}|{v}", t["semantic_contract_id"].as_str().unwrap_or("-")));
            let meta = outcome(s::for_source_metadata(raw), |(t, v)| format!("{}|{v}", t["semantic_contract_id"].as_str().unwrap_or("-")));
            let reason = format!("{:?}", s::standing_reason(raw));
            let standing = s::numerical_use_standing(raw, &bases);
            let fresh = s::is_fresh_identity(id);
            let refusals: Vec<String> = raw["results"].as_array().unwrap().iter().filter_map(|row| s::rule_binding_refusal(raw, row).map(|r| format!("{}={r}", row["id"].as_str().unwrap_or("-")))).collect();
            let derived = match derive(raw) {
                Ok(doc) => format!("ok:{}:{}", sha(&doc), outcome(d::validate_document(&doc, raw), |_| "valid".into())),
                Err(e) => format!("err:{e}"),
            };
            writeln!(out, "{kind}\t{rel}#{pointer}\t{id}\t{full}\t{meta}\t{reason}\t{standing}\t{fresh}\t{}\t{derived}", sha(&json!(refusals))).unwrap();
        }
    }
    eprintln!("I66_SWEEP envelopes={n} successor={successor}");
}
