//! I83 B6 probe (scratch only): Rust first failure on the probe entries in I83_PROBES.
use open_pipe_stress_result_export::retained_precision as rp;
use serde_json::Value;
fn corpus() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/results/retained_precision_cases.json"
    ))
    .unwrap()
}
/// D32 (RV79-N-e) and the 07e format rule (RV78-N1): a rehash or edit-path
/// index is a strict integral value: a JSON number, never a boolean, finite,
/// integral, >= 0 and not -0.
fn index(v: &Value) -> Option<usize> {
    let n = v.as_f64()?;
    (n.is_finite() && n >= 0.0 && n.fract() == 0.0 && !(n == 0.0 && n.is_sign_negative()))
        .then_some(n as usize)
}
/// The 07e format rule (SHARED_SNAPSHOT_07E `format_rule`): 1 preparation
/// hashes, for each `sources[*].preparation.attempt_ref` that resolves and
/// whose members are all prepared; 2 source identities, for each selected
/// case whose `source_ref` resolves; 3 publication hash; 4 receipt hash. A
/// reference that is not an index, or does not resolve, is skipped.
fn rehash(source: &mut Value) {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    // Snapshot 07 format: an entry that removes retained_precision or its body
    // (a G0 pin) has nothing to rehash.
    if !source
        .get("retained_precision")
        .and_then(|r| r.get("body"))
        .is_some_and(Value::is_object)
    {
        return;
    }
    let b = &mut source["retained_precision"]["body"];
    let attempts = b["product_attempts"].clone();
    for s in b["sources"].as_array_mut().unwrap() {
        if let Some(ai) = index(&s["preparation"]["attempt_ref"]) {
            let a = &attempts[ai];
            // As G1: only an addressable attempt has a preparation digest.
            if !a.is_object() {
                continue;
            }
            if a["preparation"]["members"]
                .as_array()
                .unwrap()
                .iter()
                .all(|m| m["result"]["kind"] == "prepared")
            {
                let members: Vec<_> = a["preparation"]["members"].as_array().unwrap().iter().map(|m| serde_json::json!({"member":m["member"],"old_source":m["old_source"],"old_facts":m["old_facts"],"section":m["result"]["section"]})).collect();
                let payload = serde_json::json!({"definition_id":a["definition_id"],"definition_sha256":rp::DEFINITION_HASH,"owner_ref":a["owner_ref"],"ordinary_attempt_ref":a["ordinary_attempt_ref"],"material_basis_ref":a["material_basis_ref"],"members":members});
                s["preparation"]["sha256"] =
                    domain_hash("retained_precision_preparation_v1", &payload)
                        .unwrap()
                        .into();
            }
        }
    }
    let sources = b["sources"].clone();
    for c in b["cases"].as_array_mut().unwrap() {
        if c["status"] == "selected" {
            // Only a resolving source has an identity digest to recompute.
            let Some(mut s) = index(&c["source_ref"])
                .and_then(|i| sources.get(i))
                .filter(|s| s.is_object())
                .cloned()
            else {
                continue;
            };
            s.as_object_mut().unwrap().remove("index");
            c["source_identity_sha256"] = domain_hash("retained_precision_source_mp_v2", &s)
                .unwrap()
                .into();
        }
    }
    let mut public = source.clone();
    public.as_object_mut().unwrap().remove("retained_precision");
    source["retained_precision"]["body"]["publication_sha256"] =
        domain_hash("retained_precision_publication_mp_v2", &public)
            .unwrap()
            .into();
    source["retained_precision"]["receipt_sha256"] = domain_hash(
        "retained_precision_receipt_mp_v2",
        &source["retained_precision"]["body"],
    )
    .unwrap()
    .into();
}
fn edit(source: &mut Value, e: &Value) {
    let path = e["path"].as_array().unwrap();
    let mut parent = source;
    for p in &path[..path.len() - 1] {
        parent = if let Some(i) = index(p) {
            &mut parent[i]
        } else {
            &mut parent[p.as_str().unwrap()]
        };
    }
    let last = path.last().unwrap();
    if e["op"] == "remove" {
        if let Some(i) = index(last) {
            parent.as_array_mut().unwrap().remove(i);
        } else {
            parent
                .as_object_mut()
                .unwrap()
                .remove(last.as_str().unwrap());
        }
    } else if let Some(i) = index(last) {
        parent[i] = e["value"].clone();
    } else {
        parent[last.as_str().unwrap()] = e["value"].clone();
    }
}
#[test]
fn i83_probe() {
    let shared = corpus();
    let probes: Value = serde_json::from_str(&std::fs::read_to_string(std::env::var("I83_PROBES").unwrap()).unwrap()).unwrap();
    let mut out = Vec::new();
    for p in probes.as_array().unwrap() {
        let case = shared["cases"].as_array().unwrap().iter().find(|c| c["base"] == p["base"] || c["id"] == p["base"]).unwrap();
        let mut source = case["source"].clone();
        for e in p["edits"].as_array().unwrap() { edit(&mut source, e); }
        rehash(&mut source);
        let r = match rp::validate(&source, Some(&case["invocation"])) { Ok(_) => "pass".to_string(), Err(e) => format!("{} {}", e.gate, e.code) };
        let t = match open_pipe_stress_result_export::semantic_contract::for_source_metadata(&source) { Ok(_) => "ok".to_string(), Err(e) => e };
        out.push(serde_json::json!({"label": p["label"], "rust": r, "rust_transport": t}));
    }
    std::fs::write(std::env::var("I83_OUT").unwrap(), serde_json::to_string_pretty(&out).unwrap()).unwrap();
}
#[test]
fn i83_transport() {
    let Ok(dir) = std::env::var("I83_PROBE_DIR") else { return };
    let dir = std::path::PathBuf::from(dir);
    let index: Value = serde_json::from_str(&std::fs::read_to_string(dir.join("index.json")).unwrap()).unwrap();
    let mut out = serde_json::Map::new();
    for item in index.as_array().unwrap() {
        let probe: Value = serde_json::from_str(&std::fs::read_to_string(dir.join(item["file"].as_str().unwrap())).unwrap()).unwrap();
        let r = match open_pipe_stress_result_export::semantic_contract::for_source_metadata(&probe["source"]) { Ok(_) => "ok".to_string(), Err(e) => e };
        out.insert(probe["id"].as_str().unwrap().to_string(), Value::String(r));
    }
    std::fs::write(std::env::var("I83_TRANSPORT_OUT").unwrap(), serde_json::to_string(&Value::Object(out)).unwrap()).unwrap();
}
