//! RV113 (RV-R) reviewer harness. Not part of any candidate: it is copied into the
//! reviewer's own archive copies only. It materializes every shared corpus entry
//! (and, optionally, reviewer probes) from the snapshot format rule, written here
//! independently (SHARED_SNAPSHOT_06C `format_change`; SHARED_SNAPSHOT_07E
//! `format_rule`), and records each reader verdict as one JSON line.
//!
//! Environment: RV113_OUT (census output, JSON lines); RV113_PROBES (probe entries,
//! JSON array) with RV113_PROBES_OUT (probe output, JSON lines).
use open_pipe_stress_result_export::retained_precision as rp;
use open_pipe_stress_result_export::semantic_contract as sc;
use open_pipe_stress_result_export::source_blocks::domain_hash;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::io::Write;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn corpus() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/results/retained_precision_cases.json"
    ))
    .unwrap()
}
/// The strict index rule: a JSON number (never a boolean), finite, integral,
/// >= 0 and not -0.
fn strict_index(v: &Value) -> Option<usize> {
    if !v.is_number() {
        return None;
    }
    let n = v.as_f64()?;
    if n.is_finite() && n >= 0.0 && n.fract() == 0.0 && !(n == 0.0 && n.is_sign_negative()) {
        Some(n as usize)
    } else {
        None
    }
}
fn step<'a>(v: &'a mut Value, key: &Value) -> Result<&'a mut Value, String> {
    if let Some(i) = strict_index(key) {
        let len = v.as_array().map(|a| a.len()).ok_or("index into a non-array")?;
        if i >= len {
            return Err(format!("index {i} out of range {len}"));
        }
        Ok(&mut v.as_array_mut().unwrap()[i])
    } else {
        let k = key.as_str().ok_or("non-string key")?;
        if !(v.is_object() || v.is_null()) {
            return Err(format!("key {k} into a non-object"));
        }
        Ok(&mut v[k])
    }
}
fn apply_edit(root: &mut Value, e: &Value) -> Result<(), String> {
    let path = e["path"].as_array().ok_or("path")?;
    let (last, parents) = path.split_last().ok_or("empty path")?;
    let mut at = root;
    for p in parents {
        at = step(at, p)?;
    }
    match e["op"].as_str() {
        Some("remove") => {
            if let Some(i) = strict_index(last) {
                let a = at.as_array_mut().ok_or("remove index from a non-array")?;
                if i >= a.len() {
                    return Err("remove out of range".into());
                }
                a.remove(i);
            } else {
                let o = at.as_object_mut().ok_or("remove key from a non-object")?;
                o.remove(last.as_str().ok_or("non-string key")?);
            }
            Ok(())
        }
        Some("set") => {
            if let Some(i) = strict_index(last) {
                let a = at.as_array_mut().ok_or("set index on a non-array")?;
                if i >= a.len() {
                    return Err("set out of range".into());
                }
                a[i] = e["value"].clone();
            } else {
                let k = last.as_str().ok_or("non-string key")?;
                if !(at.is_object() || at.is_null()) {
                    return Err("set key on a non-object".into());
                }
                at[k] = e["value"].clone();
            }
            Ok(())
        }
        other => Err(format!("op {other:?}")),
    }
}
fn h(domain: &str, payload: &Value) -> Value {
    Value::String(domain_hash(domain, payload).expect("domain hash"))
}
/// SHARED_SNAPSHOT_07E `format_rule`, in its order: 1 preparation hashes;
/// 2 selected source identities; 3 publication hash; 4 receipt hash.
fn rehash_all(source: &mut Value) {
    let has_body = source
        .get("retained_precision")
        .and_then(|r| r.get("body"))
        .is_some_and(Value::is_object);
    if !has_body {
        return;
    }
    let attempts = source["retained_precision"]["body"]["product_attempts"].clone();
    if let Some(sources) = source["retained_precision"]["body"]["sources"].as_array_mut() {
        for s in sources.iter_mut() {
            let Some(ai) = strict_index(&s["preparation"]["attempt_ref"]) else { continue };
            let Some(a) = attempts.as_array().and_then(|x| x.get(ai)).filter(|a| a.is_object()) else { continue };
            let members: Vec<Value> = a["preparation"]["members"].as_array().cloned().unwrap_or_default();
            if !members.iter().all(|m| m["result"]["kind"] == "prepared") {
                continue;
            }
            let payload = json!({
                "definition_id": a["definition_id"],
                "definition_sha256": rp::DEFINITION_HASH,
                "owner_ref": a["owner_ref"],
                "ordinary_attempt_ref": a["ordinary_attempt_ref"],
                "material_basis_ref": a["material_basis_ref"],
                "members": members.iter().map(|m| json!({
                    "member": m["member"], "old_source": m["old_source"],
                    "old_facts": m["old_facts"], "section": m["result"]["section"]})).collect::<Vec<_>>(),
            });
            s["preparation"]["sha256"] = h("retained_precision_preparation_v1", &payload);
        }
    }
    let sources = source["retained_precision"]["body"]["sources"].clone();
    if let Some(cases) = source["retained_precision"]["body"]["cases"].as_array_mut() {
        for c in cases.iter_mut() {
            if c["status"] != "selected" {
                continue;
            }
            let Some(si) = strict_index(&c["source_ref"]) else { continue };
            let Some(mut s) = sources.as_array().and_then(|x| x.get(si)).filter(|s| s.is_object()).cloned() else { continue };
            s.as_object_mut().unwrap().remove("index");
            c["source_identity_sha256"] = h("retained_precision_source_mp_v2", &s);
        }
    }
    let mut public = source.clone();
    public.as_object_mut().unwrap().remove("retained_precision");
    source["retained_precision"]["body"]["publication_sha256"] =
        h("retained_precision_publication_mp_v2", &public);
    let body = source["retained_precision"]["body"].clone();
    source["retained_precision"]["receipt_sha256"] = h("retained_precision_receipt_mp_v2", &body);
}
/// SHARED_SNAPSHOT_06C `format_change` steps 1-4, then `after_rehash` literally.
fn materialize(shared: &Value, entry: &Value) -> Result<(Value, Value), String> {
    let base = shared["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == entry["base"])
        .ok_or("unknown base")?;
    let mut source = base["source"].clone();
    for e in entry["edits"].as_array().cloned().unwrap_or_default() {
        apply_edit(&mut source, &e)?;
    }
    let mut invocation = base["invocation"].clone();
    let inv_edits = entry["invocation_edits"].as_array().cloned().unwrap_or_default();
    for e in &inv_edits {
        apply_edit(&mut invocation, e)?;
    }
    if !inv_edits.is_empty() {
        source["retained_precision"]["body"]["invocation"]["value"] =
            h("source_blocks_invocation_v1", &invocation);
    }
    if entry["rehash"] != "all" {
        return Err("rehash is not all".into());
    }
    rehash_all(&mut source);
    for e in entry["after_rehash"].as_array().cloned().unwrap_or_default() {
        apply_edit(&mut source, &e)?;
    }
    Ok((source, invocation))
}
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn verdict(r: Result<Result<rp::Validation, rp::ValidationError>, String>) -> Value {
    match r {
        Err(p) => json!({"panic": p}),
        Ok(Ok(v)) => json!({"ok": {
            "invocation_bound": v.invocation_bound,
            "numerical_eligible": v.numerical_eligible,
            "publication_sha256": v.publication_sha256,
            "classifications": v.classifications.len(),
            "classifications_sha256": sha(format!("{:?}", v.classifications).as_bytes()),
        }}),
        Ok(Err(e)) => json!({"err": {"gate": e.gate, "code": e.code, "detail": e.detail}}),
    }
}
fn guarded<T>(f: impl FnOnce() -> T) -> Result<T, String> {
    catch_unwind(AssertUnwindSafe(f)).map_err(|p| {
        p.downcast_ref::<String>()
            .cloned()
            .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_else(|| "panic".into())
    })
}
fn evaluate(shared: &Value, kind: &str, index: usize, entry: &Value) -> Value {
    let (source, invocation) = match materialize(shared, entry) {
        Ok(x) => x,
        Err(e) => return json!({"set": kind, "i": index, "id": entry["id"], "materialize_error": e}),
    };
    let input = serde_json::to_string(&json!([source, invocation])).unwrap();
    let bound = verdict(guarded(|| rp::validate(&source, Some(&invocation))));
    let unbound = verdict(guarded(|| rp::validate(&source, None)));
    let transport = verdict(guarded(|| rp::validate_transport_metadata(&source)));
    let order: Vec<Value> = source["retained_precision"]["body"]["cases"]
        .as_array()
        .map(|cs| cs.iter().map(|c| c["basis_ref"].clone()).collect())
        .unwrap_or_default();
    let standing = guarded(|| sc::numerical_use_standing_with_context(&source, &order, Some(&invocation)))
        .map(Value::from)
        .unwrap_or_else(|p| json!({"panic": p}));
    json!({"set": kind, "i": index, "id": entry["id"], "input_sha256": sha(input.as_bytes()),
           "bound": bound, "unbound": unbound, "transport": transport, "standing": standing})
}
fn write_lines(path: &str, lines: &[Value]) {
    let mut f = std::fs::File::create(path).unwrap();
    for l in lines {
        writeln!(f, "{}", serde_json::to_string(l).unwrap()).unwrap();
    }
}
#[test]
fn rv113_census() {
    let Ok(out) = std::env::var("RV113_OUT") else {
        eprintln!("RV113_OUT unset; census skipped");
        return;
    };
    let shared = corpus();
    let mut lines = Vec::new();
    for (i, case) in shared["cases"].as_array().unwrap().iter().enumerate() {
        // A base as committed: no edit and no rehash.
        let source = case["source"].clone();
        let invocation = case["invocation"].clone();
        let input = serde_json::to_string(&json!([source, invocation])).unwrap();
        let order: Vec<Value> = source["retained_precision"]["body"]["cases"]
            .as_array()
            .map(|cs| cs.iter().map(|c| c["basis_ref"].clone()).collect())
            .unwrap_or_default();
        lines.push(json!({"set": "base", "i": i, "id": case["id"], "input_sha256": sha(input.as_bytes()),
            "bound": verdict(guarded(|| rp::validate(&source, Some(&invocation)))),
            "unbound": verdict(guarded(|| rp::validate(&source, None))),
            "transport": verdict(guarded(|| rp::validate_transport_metadata(&source))),
            "standing": guarded(|| sc::numerical_use_standing_with_context(&source, &order, Some(&invocation))).map(Value::from).unwrap_or_else(|p| json!({"panic": p}))}));
    }
    for (i, m) in shared["mutations"].as_array().unwrap().iter().enumerate() {
        lines.push(evaluate(&shared, "mutation", i, m));
    }
    for (i, m) in shared["must_pass"].as_array().unwrap().iter().enumerate() {
        lines.push(evaluate(&shared, "must_pass", i, m));
    }
    write_lines(&out, &lines);
    eprintln!("rv113 census: {} lines -> {out}", lines.len());
}
#[test]
fn rv113_probes() {
    let (Ok(path), Ok(out)) = (std::env::var("RV113_PROBES"), std::env::var("RV113_PROBES_OUT")) else {
        eprintln!("RV113_PROBES unset; probes skipped");
        return;
    };
    let shared = corpus();
    let probes: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    let lines: Vec<Value> = probes
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, p)| evaluate(&shared, "probe", i, p))
        .collect();
    write_lines(&out, &lines);
    eprintln!("rv113 probes: {} lines -> {out}", lines.len());
}
