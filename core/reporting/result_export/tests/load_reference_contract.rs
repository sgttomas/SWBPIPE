//! load-reference-1 readers on the four frozen producer envelopes, with the
//! adversarial cases shared with `tests/test_load_reference_readers.py`.
//! Both readers must reproduce every expectation in the case file. This proves
//! consumer admission and carrier preservation, not producer authentication.
use open_pipe_stress_result_export::{
    derivative as d, load_reference as lr, semantic_contract as s,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const CASES: &str = include_str!("fixtures/load_reference_mutations.json");
const TABLE: &str =
    include_str!("../../../../fixtures/results/semantic_contract_v0_3_load_reference_1.json");
const PHYSICS_PREFIX: &str = "SOURCE_LOAD_REFERENCE_PHYSICS_EVIDENCE";
const NONFINITE_MARKER: &str = "__LOAD_REFERENCE_NONFINITE__";
const RAW: [&str; 4] = [
    "connected-sparse",
    "connected-dense",
    "pressure-sparse",
    "pressure-dense",
];

fn source_text(name: &str) -> &'static str {
    match name {
        "connected-sparse" => include_str!(
            "../../../../fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json"
        ),
        "connected-dense" => include_str!(
            "../../../../fixtures/product_preview/load_reference/connected-dense_scrutiny.raw.json"
        ),
        "pressure-sparse" => include_str!(
            "../../../../fixtures/product_preview/load_reference/pressure-sparse_interactive.raw.json"
        ),
        "pressure-dense" => include_str!(
            "../../../../fixtures/product_preview/load_reference/pressure-dense_scrutiny.raw.json"
        ),
        "physics-sparse" => {
            include_str!("../../../../fixtures/results/physics_connected_mechanics_sparse.json")
        }
        _ => panic!("unknown source {name}"),
    }
}
fn source(name: &str) -> Value {
    serde_json::from_str(source_text(name)).unwrap()
}
fn request(name: &str) -> Value {
    serde_json::from_str(if name.starts_with("connected") {
        include_str!("../../../../fixtures/product_preview/load_reference/connected.request.json")
    } else {
        include_str!("../../../../fixtures/product_preview/load_reference/pressure.request.json")
    })
    .unwrap()
}
fn cases() -> Value {
    serde_json::from_str(CASES).unwrap()
}
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn tokens(path: &str) -> Vec<String> {
    assert!(path.starts_with('/'), "{path}");
    path[1..]
        .split('/')
        .map(|t| t.replace("~1", "/").replace("~0", "~"))
        .collect()
}
fn parent_path(path: &str) -> (String, String) {
    let t = tokens(path);
    let parent = t[..t.len() - 1]
        .iter()
        .map(|x| format!("/{}", x.replace('~', "~0").replace('/', "~1")))
        .collect::<String>();
    (parent, t.last().unwrap().clone())
}
fn resolve<'a>(doc: &'a Value, path: &str) -> &'a Value {
    doc.pointer(path)
        .unwrap_or_else(|| panic!("missing {path}"))
}
fn set(doc: &mut Value, path: &str, value: Value) {
    let (parent, last) = parent_path(path);
    let parent = doc
        .pointer_mut(&parent)
        .unwrap_or_else(|| panic!("missing parent of {path}"));
    match parent {
        Value::Array(a) => a[last.parse::<usize>().unwrap()] = value,
        Value::Object(o) => {
            o.insert(last, value);
        }
        _ => panic!("scalar parent of {path}"),
    }
}
fn matches(item: &Value, pattern: &Value) -> bool {
    pattern
        .as_object()
        .unwrap()
        .iter()
        .all(|(k, v)| item.get(k) == Some(v))
}
/// Mirror of the Python harness. Returns the mutated document and, for a
/// non-finite or `json_text` op, the literal that must replace the marker in
/// the JSON text.
fn apply(doc: &Value, ops: &Value) -> (Value, Option<String>) {
    let mut doc = doc.clone();
    let mut nonfinite = None;
    for op in ops.as_array().unwrap() {
        let path = op["path"].as_str().unwrap();
        match op["op"].as_str().unwrap() {
            "set" => set(&mut doc, path, op["value"].clone()),
            "remove" => {
                let (parent, last) = parent_path(path);
                match doc.pointer_mut(&parent).unwrap() {
                    Value::Array(a) => {
                        a.remove(last.parse::<usize>().unwrap());
                    }
                    Value::Object(o) => {
                        o.remove(&last).unwrap();
                    }
                    _ => panic!("scalar parent"),
                }
            }
            "append" => doc
                .pointer_mut(path)
                .unwrap()
                .as_array_mut()
                .unwrap()
                .push(op["value"].clone()),
            "copy" => {
                let value = resolve(&doc, op["from"].as_str().unwrap()).clone();
                set(&mut doc, path, value);
            }
            "copy_append" => {
                let value = resolve(&doc, op["from"].as_str().unwrap()).clone();
                doc.pointer_mut(path)
                    .unwrap()
                    .as_array_mut()
                    .unwrap()
                    .push(value);
            }
            "swap" => {
                let other = op["with"].as_str().unwrap();
                let (first, second) = (resolve(&doc, path).clone(), resolve(&doc, other).clone());
                set(&mut doc, path, second);
                set(&mut doc, other, first);
            }
            "graft" => {
                let value = resolve(
                    &source(op["source"].as_str().unwrap()),
                    op["from"].as_str().unwrap(),
                )
                .clone();
                set(&mut doc, path, value);
            }
            "remove_where" => doc
                .pointer_mut(path)
                .unwrap()
                .as_array_mut()
                .unwrap()
                .retain(|item| !matches(item, &op["match"])),
            "set_where" => {
                for item in doc.pointer_mut(path).unwrap().as_array_mut().unwrap() {
                    if matches(item, &op["match"]) {
                        item[op["key"].as_str().unwrap()] = op["value"].clone();
                    }
                }
            }
            "append_copy_where" => {
                let array = doc.pointer_mut(path).unwrap().as_array_mut().unwrap();
                let mut copy = array
                    .iter()
                    .find(|item| matches(item, &op["match"]))
                    .unwrap()
                    .clone();
                for (k, v) in op["set"].as_object().unwrap() {
                    copy[k] = v.clone();
                }
                array.push(copy);
            }
            "nonfinite" => {
                // serde_json::Value cannot hold a non-finite number at all.
                assert!(serde_json::Number::from_f64(f64::INFINITY).is_none());
                assert!(serde_json::Number::from_f64(f64::NAN).is_none());
                set(&mut doc, path, json!(NONFINITE_MARKER));
                nonfinite = Some(
                    match op["value"].as_str().unwrap() {
                        "Infinity" => "1e400",
                        "-Infinity" => "-1e400",
                        "NaN" => "NaN",
                        other => panic!("unknown non-finite {other}"),
                    }
                    .to_string(),
                );
            }
            "json_text" => {
                // A literal that a serde_json Value may be unable to hold.
                assert!(nonfinite.is_none(), "one text literal per case");
                set(&mut doc, path, json!(NONFINITE_MARKER));
                nonfinite = Some(op["value"].as_str().unwrap().to_string());
            }
            other => panic!("unknown op {other}"),
        }
    }
    (doc, nonfinite)
}
fn outcome(result: Result<impl Sized, String>) -> String {
    match result {
        Ok(_) => "accept".into(),
        Err(e) => e,
    }
}
fn agrees(actual: &str, expected: &str) -> bool {
    if expected == PHYSICS_PREFIX {
        actual.split(": ").next() == Some(PHYSICS_PREFIX)
    } else {
        actual == expected
    }
}
fn expected<'a>(case: &'a Value, kind: &str) -> &'a str {
    let specific = &case[format!("{kind}_rust")];
    if let Some(value) = specific.as_str() {
        return value;
    }
    if kind == "validator" {
        if let Some(value) = case["validator"].as_str() {
            return value;
        }
    }
    case["dispatch"].as_str().unwrap()
}
fn bases(raw: &Value) -> Vec<Value> {
    raw["numerical_quality"]["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["basis_ref"].clone())
        .collect()
}
fn derive(raw: &Value, model: &Value) -> Result<Value, String> {
    let base: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json"
    ))
    .unwrap();
    let carrier = d::checksum(
        raw,
        "attested_headless_producer_carrier",
        d::reference("test_carrier", "frozen-load-reference-producer-output"),
    )?;
    let origin = json!({"origin_id":"load-reference-reader-test","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Frozen producer bytes; consumer contract test only","actual_model_ref":d::reference("model_payload",raw["model_ref"].as_str().unwrap_or("")),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap_or("")),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    d::derive_document(base, model, raw, origin, None)
}
fn record(log: &mut Vec<Value>, entry: Value) {
    log.push(entry);
}
fn write_log(name: &str, log: &[Value]) {
    if let Ok(dir) = std::env::var("LOAD_REFERENCE_PARITY_OUT") {
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            std::path::Path::new(&dir).join(name),
            serde_json::to_string_pretty(log).unwrap() + "\n",
        )
        .unwrap();
    }
}

#[test]
fn frozen_inputs_table_and_schema_are_pinned() {
    for (name, digest) in [
        (
            "connected-sparse",
            "915965a446ff04c6c4b7f0209ffff34c4ac040a0d4b8243d875dc7217a9cb74c",
        ),
        (
            "connected-dense",
            "3824035cabf2d2a151756e04b46112ccbdcf4ce8237f49e68b5d3b36b8dfb2bf",
        ),
        (
            "pressure-sparse",
            "71be3e4fb1af54e962d5e4939a518fcab6f7896842ab4f08212a8212ae47f134",
        ),
        (
            "pressure-dense",
            "b46eeb8a5dc82bb9e0398b8d58db07eb73906eb25110d68b0032fd12df1d0707",
        ),
    ] {
        assert_eq!(sha(source_text(name).as_bytes()), digest, "{name}");
    }
    assert_eq!(sha(TABLE.as_bytes()), s::LOAD_REFERENCE_TABLE_SHA256);
    let table = s::load_reference_contract();
    assert_eq!(table["semantic_contract_id"], s::LOAD_REFERENCE_ID);
    assert_eq!(table["formulation_profile_id"], s::LOAD_REFERENCE_PROFILE);
    // Result rows use the physics-1 signatures unchanged.
    assert_eq!(table["rows"], s::physics_contract()["rows"]);
    assert_eq!(
        table["canonical_metadata_vocabulary"],
        s::physics_contract()["canonical_metadata_vocabulary"]
    );
    assert!(lr::transport_schema().is_ok());
    assert_eq!(
        lr::TRANSPORT_SCHEMA_SHA256,
        "640fd4477ac2c84f3c02268cfccc3958f51ee6508b3a67e5538b52d84899af65"
    );
}

#[test]
fn shared_adversarial_cases_match_in_rust() {
    let cases = cases();
    let mut log = Vec::new();
    for case in cases["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let (doc, nonfinite) = apply(&source(case["source"].as_str().unwrap()), &case["ops"]);
        let (dispatch, validator) = if let Some(literal) = &nonfinite {
            let text = serde_json::to_string(&doc)
                .unwrap()
                .replace(&format!("\"{NONFINITE_MARKER}\""), literal);
            assert!(!text.contains(NONFINITE_MARKER));
            let parsed = serde_json::from_str::<Value>(&text);
            assert!(parsed.is_err(), "{id}: text literal parsed");
            (
                "JSON_PARSE_REJECTED".to_string(),
                "JSON_PARSE_REJECTED".to_string(),
            )
        } else {
            (
                outcome(s::for_source(&doc)),
                outcome(s::validate_load_reference_evidence(&doc)),
            )
        };
        record(
            &mut log,
            json!({"id": id, "dispatch": dispatch, "validator": validator}),
        );
        assert!(
            agrees(&dispatch, expected(case, "dispatch")),
            "{id}: dispatch {dispatch} != {}",
            expected(case, "dispatch")
        );
        assert!(
            agrees(&validator, expected(case, "validator")),
            "{id}: validator {validator} != {}",
            expected(case, "validator")
        );
        if nonfinite.is_some() {
            continue;
        }
        let name = case["source"].as_str().unwrap();
        let model = if name == "physics-sparse" {
            serde_json::from_str::<Value>(include_str!(
                "../../../product_physics/tests/fixtures/exact_pressure_connected_request.json"
            ))
            .unwrap()["model"]
                .clone()
        } else {
            request(name)["model"].clone()
        };
        if dispatch == "accept" {
            assert_eq!(
                s::for_source(&doc).unwrap().0["semantic_contract_id"],
                s::LOAD_REFERENCE_ID
            );
            if let Some(code) = case["carrier_rust"].as_str() {
                // Both readers admit it; the checked-JSON carrier profile does not.
                assert_eq!(derive(&doc, &model).unwrap_err(), code, "{id}");
                assert_eq!(
                    s::numerical_use_standing(&doc, &bases(&doc)),
                    "numerically_eligible",
                    "{id}"
                );
            } else if doc["status"]["mechanics"] == "MECHANICS_SOLVED" {
                let document = derive(&doc, &model).unwrap_or_else(|e| panic!("{id}: {e}"));
                assert_eq!(
                    document["result_envelope"]["contract_evidence"],
                    doc["contract_evidence"]
                );
                assert_eq!(
                    s::numerical_use_standing(&doc, &bases(&doc)),
                    "numerically_eligible"
                );
            }
        } else {
            assert!(derive(&doc, &model).is_err(), "{id}: derived");
            assert_eq!(
                s::numerical_use_standing(&doc, &bases(&doc)),
                "unsupported",
                "{id}"
            );
        }
    }
    write_log("rust_outcomes.json", &log);
}

#[test]
fn table_bytes_and_transport_cases_match_in_rust() {
    let cases = cases();
    let mut log = Vec::new();
    for case in cases["table_cases"].as_array().unwrap() {
        let mut text = TABLE.to_string();
        if let Some(find) = case["find"].as_str() {
            assert!(text.contains(find));
            text = text.replacen(find, case["replace"].as_str().unwrap(), 1);
        }
        let result = outcome(s::verify_load_reference_table(text.as_bytes()));
        record(&mut log, json!({"id": case["id"], "table": result}));
        assert_eq!(result, case["expect"].as_str().unwrap(), "{}", case["id"]);
    }
    for case in cases["transport_cases"].as_array().unwrap() {
        let (doc, nonfinite) = apply(&source(case["source"].as_str().unwrap()), &case["ops"]);
        assert!(nonfinite.is_none());
        let mut meta = json!({"schema_version": doc["schema_version"]});
        for key in [
            "producer",
            "numerical_quality",
            "formulation_basis",
            "contract_evidence",
            "source_block_recovery",
            "carrier_evidence",
        ] {
            if let Some(v) = doc.get(key) {
                meta[key] = v.clone();
            }
        }
        let result = outcome(s::validate_load_reference_transport_metadata(&meta));
        record(&mut log, json!({"id": case["id"], "transport": result}));
        assert!(
            agrees(&result, case["expect"].as_str().unwrap()),
            "{}: {result}",
            case["id"]
        );
        if result == "accept" {
            assert!(s::for_source_metadata(&meta).is_ok());
        }
    }
    write_log("rust_table_transport_outcomes.json", &log);
}

#[test]
fn canonical_documents_preserve_source_metadata_and_evidence_verbatim() {
    for name in RAW {
        let raw = source(name);
        let model = request(name)["model"].clone();
        let doc = derive(&raw, &model).unwrap();
        let e = &doc["result_envelope"];
        assert_eq!(doc["schema_version"], "0.3.0");
        for key in [
            "producer",
            "numerical_quality",
            "formulation_basis",
            "contract_evidence",
        ] {
            assert_eq!(e[key], raw[key], "{name} {key}");
        }
        assert!(e.get("source_block_recovery").is_none());
        assert_eq!(
            e["semantic_contract_ref"],
            d::reference("semantic_contract", s::LOAD_REFERENCE_ID)
        );
        assert_eq!(
            e["row_accounting"].as_array().unwrap().len(),
            raw["results"].as_array().unwrap().len()
        );
        for account in e["row_accounting"].as_array().unwrap() {
            let row = &raw["results"][account["source_row_index"].as_u64().unwrap() as usize];
            if row["kind"].as_str().unwrap().ends_with("_v2") {
                assert_eq!(account["disposition"], "exported_quantity");
                let target = doc
                    .pointer(account["target_field_path"].as_str().unwrap())
                    .unwrap();
                assert_eq!(
                    target["magnitude"].as_f64().unwrap().to_bits(),
                    row["value"].as_f64().unwrap().to_bits()
                );
                assert_eq!(target["metadata"], row["metadata"]);
            }
        }
        let reloaded: Value = serde_json::from_slice(&serde_json::to_vec(&doc).unwrap()).unwrap();
        d::validate_document(&reloaded, &raw).unwrap();
        // Compact bytes plus a newline: the carriers are large and regenerated by rerun.
        let text = serde_json::to_string(&doc).unwrap() + "\n";
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
            "../../../fixtures/results/load_reference_{}.document.json",
            name.replace('-', "_")
        ));
        if std::env::var("LOAD_REFERENCE_WRITE_FIXTURES").as_deref() == Ok("1") {
            std::fs::write(&path, &text).unwrap();
        }
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            text,
            "{name} carrier bytes"
        );
        // Tampered carriers stay bound to the source even after rehash.
        for (pointer, value, code) in [
            (
                "/result_envelope/contract_evidence/load_reference_states/0/source_recovery/status",
                json!("joined"),
                "SOURCE_CONTRACT_EVIDENCE_BINDING_MISMATCH",
            ),
            (
                "/result_envelope/semantic_contract_ref/ref_id",
                json!(s::PHYSICS_ID),
                "SEMANTIC_CONTRACT_BINDING_MISMATCH",
            ),
            (
                "/result_envelope/formulation_basis/profile_id",
                json!("exact_straight_pressure_v2"),
                "SOURCE_METADATA_BINDING_MISMATCH",
            ),
        ] {
            let mut bad = doc.clone();
            *bad.pointer_mut(pointer).unwrap() = value;
            bad["result_envelope"]["reproducibility"]
                .as_object_mut()
                .unwrap()
                .remove("derivative_hash");
            bad["result_envelope"]["reproducibility"]["derivative_hash"] = d::checksum(
                &bad,
                "derivative_document_excludes_own_hash",
                d::reference(
                    "derivative_document",
                    bad["result_envelope"]["envelope_id"].as_str().unwrap(),
                ),
            )
            .unwrap();
            assert_eq!(
                d::validate_document(&bad, &raw).unwrap_err(),
                code,
                "{name} {pointer}"
            );
        }
    }
}

#[test]
fn header_only_dispatch_is_bound_to_one_profile_and_never_qualifies_raw_output() {
    let raw = source("pressure-sparse");
    let mut header = raw.clone();
    header.as_object_mut().unwrap().remove("results");
    assert_eq!(
        s::for_source_metadata(&header).unwrap().0["semantic_contract_id"],
        s::LOAD_REFERENCE_ID
    );
    assert!(s::for_source(&header).is_err());
    // Existing physics-1 dispatch is unchanged.
    let physics = source("physics-sparse");
    assert_eq!(
        s::for_source(&physics).unwrap().0["semantic_contract_id"],
        s::PHYSICS_ID
    );
    for id in [
        s::PRECISION_ID,
        s::PHYSICS_ID,
        s::PHYSICS_SOURCE_ID,
        "openpipestress.result_semantics/0.3.0/source-blocks-1",
    ] {
        let mut bad = physics.clone();
        bad["producer"]["semantic_contract_id"] = json!(id);
        bad["formulation_basis"]["profile_id"] = json!(s::LOAD_REFERENCE_PROFILE);
        assert_eq!(
            s::for_source_metadata(&bad).unwrap_err(),
            "SOURCE_FORMULATION_BASIS_UNSUPPORTED",
            "{id}"
        );
    }
}
