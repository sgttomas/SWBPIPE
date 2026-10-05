//! RV88 (U6a review): independent existing-identity sweep. Identical source in
//! the base (7e4f5a51dd) and candidate (844448112f) lanes; only APIs present in
//! both are used. Output: one TSV line per (envelope, probe) to $RV88_OUT.
use open_pipe_stress_result_export::{derivative as d, semantic_contract as s};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const SUCCESSOR: &str = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1";
const METHOD: &str = "contribution_preserving_multiprecision_v1";

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        if p.is_dir() {
            if ["execution", "node_modules", "target", ".venv", ".git", "dist"].contains(&name.as_str()) {
                continue;
            }
            walk(&p, out);
        } else if name.ends_with(".json") {
            if std::fs::metadata(&p).map(|m| m.len()).unwrap_or(0) < 40_000_000 {
                out.push(p);
            }
        }
    }
}

/// (pointer, envelope, sibling invocation)
fn find(v: &Value, ptr: String, depth: usize, out: &mut Vec<(String, Value, Option<Value>)>) {
    if depth > 9 {
        return;
    }
    match v {
        Value::Object(o) => {
            let identified = o.get("producer").and_then(|p| p.get("semantic_contract_id")).is_some_and(Value::is_string);
            // Run 2 adds legacy raw envelopes, which carry no producer (schema 0.1.0).
            let legacy = std::env::var("RV88_LEGACY").is_ok()
                && o.get("schema_version").and_then(Value::as_str) == Some("0.1.0")
                && o.get("results").is_some_and(Value::is_array)
                && o.get("status").is_some();
            if identified || legacy {
                out.push((ptr.clone(), v.clone(), None));
            }
            for (k, x) in o {
                let before = out.len();
                find(x, format!("{ptr}/{k}"), depth + 1, out);
                if k == "source" && out.len() > before && out[before].0 == format!("{ptr}/source") {
                    out[before].2 = o.get("invocation").cloned();
                }
            }
        }
        Value::Array(a) => {
            for (i, x) in a.iter().enumerate() {
                find(x, format!("{ptr}/{i}"), depth + 1, out);
            }
        }
        _ => {}
    }
}

fn r<T>(x: Result<T, String>, f: impl Fn(T) -> String) -> String {
    match x {
        Ok(v) => format!("ok:{}", f(v)),
        Err(e) => format!("err:{}", e.replace(['\t', '\n'], " ")),
    }
}

fn base_document(p: &Path) -> Value {
    serde_json::from_str(
        &std::fs::read_to_string(p.join("fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json")).unwrap(),
    )
    .unwrap()
}

fn derive(p: &Path, raw: &Value, seed_receipt: Option<&Value>) -> Result<Value, String> {
    let model = raw["model_ref"].as_str().unwrap_or("model:absent");
    let carrier = d::checksum(raw, "attested_headless_producer_carrier", d::reference("test_carrier", "rv88"))?;
    let origin = json!({"origin_id":"rv88-sweep","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"RV88 sweep","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap_or("run:absent")),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    let mut base = base_document(p);
    base["result_envelope"]["envelope_id"] = json!("envelope:rv88-sweep");
    if let Some(rcpt) = seed_receipt {
        base["result_envelope"]["retained_precision"] = rcpt.clone();
    }
    d::derive_document(base, &json!({"project":{"id":model}}), raw, origin, None)
}

fn sha(v: &Value) -> String {
    format!("{:x}", Sha256::digest(serde_json::to_vec(v).unwrap()))
}

fn probes(p: &Path, key: &str, env: &Value, inv: Option<&Value>, rows: bool, out: &mut String) {
    let id = env["producer"]["semantic_contract_id"].as_str().unwrap_or("").to_string();
    let mut line = |field: &str, val: String| {
        writeln!(out, "{key}\t{field}\t{val}").unwrap();
    };
    line("id", id.clone());
    line("for_source", r(s::for_source(env), |(t, v)| format!("{}|{v}", t["semantic_contract_id"])));
    line("for_source_metadata", r(s::for_source_metadata(env), |(t, v)| format!("{}|{v}", t["semantic_contract_id"])));
    line("standing_reason", format!("{:?}", s::standing_reason(env)));
    line("is_fresh", format!("{}", s::is_fresh_identity(&id)));
    let nq: Vec<Value> = env["numerical_quality"]["cases"].as_array().map(|c| c.iter().map(|c| c["basis_ref"].clone()).collect()).unwrap_or_default();
    let sb: Vec<Value> = env["source_block_recovery"]["body"]["cases"].as_array().map(|c| c.iter().map(|c| c["basis_ref"].clone()).collect()).unwrap_or_default();
    line("standing_nq", s::numerical_use_standing(env, &nq).into());
    line("standing_empty", s::numerical_use_standing(env, &[]).into());
    line("standing_sb", s::numerical_use_standing(env, &sb).into());
    if let Some(inv) = inv {
        line("standing_nq_inv", s::numerical_use_standing_with_context(env, &nq, Some(inv)).into());
        line("standing_sb_inv", s::numerical_use_standing_with_context(env, &sb, Some(inv)).into());
    }
    if rows {
        let mut refused = Vec::new();
        for row in env["results"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
            if let Some(code) = s::rule_binding_refusal(env, row) {
                refused.push(format!("{}={code}", row["id"].as_str().unwrap_or("?")));
            }
        }
        line("binding_refusals", refused.join(","));
    }
    let derived = derive(p, env, None);
    line("derive", r(derived.clone(), |doc| sha(&doc)));
    if let Ok(doc) = &derived {
        line("validate_document", r(d::validate_document(doc, env), |_| String::new()));
        let mut seeded = doc.clone();
        seeded["result_envelope"]["retained_precision"] = json!({"receipt_sha256":"0".repeat(64)});
        line("validate_document_seeded_receipt", r(d::validate_document(&seeded, env), |_| String::new()));
    }
}

#[test]
fn rv88_sweep() {
    let Ok(out_path) = std::env::var("RV88_OUT") else { return };
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap();
    let mut files = Vec::new();
    walk(&p, &mut files);
    let mut out = String::new();
    let mut n_env = 0;
    let receipt: Value = serde_json::from_str::<Value>(
        &std::fs::read_to_string(std::env::var("RV88_SPARSE").unwrap()).unwrap(),
    )
    .unwrap()["source"]["retained_precision"]
        .clone();
    for f in files {
        let Ok(text) = std::fs::read_to_string(&f) else { continue };
        let Ok(v) = serde_json::from_str::<Value>(&text) else { continue };
        let mut found = Vec::new();
        find(&v, String::new(), 0, &mut found);
        let rel = f.strip_prefix(&p).unwrap().to_string_lossy().to_string();
        for (ptr, env, inv) in found {
            n_env += 1;
            let key = format!("{rel}#{ptr}");
            let id = env["producer"]["semantic_contract_id"].as_str().unwrap_or("");
            let successor = id == SUCCESSOR;
            probes(&p, &key, &env, inv.as_ref(), !successor, &mut out);
            if successor {
                continue;
            }
            // Downgrade forms on an existing identity (expected to change: the guard).
            let mut m = env.clone();
            m["retained_precision"] = receipt.clone();
            probes(&p, &format!("{key}!receipt"), &m, inv.as_ref(), false, &mut out);
            let mut m = env.clone();
            m["retained_precision"] = Value::Null;
            probes(&p, &format!("{key}!null"), &m, inv.as_ref(), false, &mut out);
            // U3 R-2: the ordinary envelope with the appended unavailable notice (PP retained_wire.rs text).
            if env["diagnostics"].is_array() {
                let case = env["numerical_quality"]["cases"][0]["basis_ref"]["ref_id"].as_str().unwrap_or("case").to_string();
                let mut m = env.clone();
                m["diagnostics"].as_array_mut().unwrap().push(json!({"id":format!("diagnostic:retained-precision:{case}:unavailable"),"code":"RETAINED_PRECISION_UNAVAILABLE","severity":"info","message":"Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics; the retained_precision receipt records the actual attempt and its typed cause.","source":"core/product_physics","affected_refs":[case]}));
                probes(&p, &format!("{key}!r2notice"), &m, inv.as_ref(), true, &mut out);
            }
            if env["results"].as_array().is_some_and(|a| !a.is_empty()) {
                let mut m = env.clone();
                m["results"][0]["recovery_method"] = json!(METHOD);
                probes(&p, &format!("{key}!token0"), &m, inv.as_ref(), false, &mut out);
                let last = env["results"].as_array().unwrap().len() - 1;
                let mut m = env.clone();
                m["results"][last]["recovery_method"] = json!(METHOD);
                probes(&p, &format!("{key}!tokenlast"), &m, inv.as_ref(), false, &mut out);
                // A different token value is not the W1 token: no new guard applies.
                let mut m = env.clone();
                m["results"][0]["recovery_method"] = json!("rv88_other_method");
                probes(&p, &format!("{key}!othertoken"), &m, inv.as_ref(), false, &mut out);
            }
        }
    }
    writeln!(out, "#envelopes\t{n_env}").unwrap();
    std::fs::write(out_path, out).unwrap();
}
