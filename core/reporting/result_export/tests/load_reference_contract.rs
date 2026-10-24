//! The current pressure-2 reader on documents carrying the optional
//! load/reference-state group: the four current producer envelopes (B7b's v3
//! twins of the retired load-reference-1 statements, same physical inputs,
//! 0.7.0/3.0.0/exact_pressure_v3), with the adversarial cases shared with the
//! Python verifier and the base-reader corpus. Both readers must reproduce every
//! expectation in the case file. This proves consumer admission and carrier
//! preservation, not producer authentication.
use open_pipe_stress_result_export::{
    derivative as d, load_reference as lr, reader_verdict as rv, semantic_contract as s,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const CASES: &str = include_str!("fixtures/load_reference_mutations.json");
const TABLE: &str =
    include_str!("../../../../fixtures/results/semantic_contract_v0_3_pressure_2.json");
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
            "../../../../fixtures/product_preview/pressure/load_reference/connected-sparse_interactive.raw.json"
        ),
        "connected-dense" => include_str!(
            "../../../../fixtures/product_preview/pressure/load_reference/connected-dense_scrutiny.raw.json"
        ),
        "pressure-sparse" => include_str!(
            "../../../../fixtures/product_preview/pressure/load_reference/pressure-sparse_interactive.raw.json"
        ),
        "pressure-dense" => include_str!(
            "../../../../fixtures/product_preview/pressure/load_reference/pressure-dense_scrutiny.raw.json"
        ),
        "physics-sparse" => {
            include_str!("../../../../fixtures/results/pressure_connected_mechanics_sparse.json")
        }
        // T4-U4b review: the directional pressurized arc envelope of the base corpus.
        "u4b-arc" => include_str!("../../../../fixtures/results/pressure_u4b_arc_sparse.json"),
        _ => panic!("unknown source {name}"),
    }
}
fn source(name: &str) -> Value {
    serde_json::from_str(source_text(name)).unwrap()
}
fn request(name: &str) -> Value {
    serde_json::from_str(if name.starts_with("connected") {
        include_str!("../../../../fixtures/product_preview/pressure/load_reference/connected.request.json")
    } else {
        include_str!("../../../../fixtures/product_preview/pressure/load_reference/pressure.request.json")
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
fn current_inputs_table_and_schema_are_pinned() {
    // B7b: the regenerated 0.5.0/v3 producer envelopes (fixtures commit 2d0e2210c1);
    // T4-U6: regenerated again on the D-5 producer (both section-basis families);
    // model 0.6.0: again for the pressure-2 identity (identity bytes only);
    // T4-U4b: again for the section stress rows on every member;
    // O4 (DEL-04-08 `o4-source-extension.md` §3.3): again for limitation [7] only;
    // connected: silent-zero diagnostic added; no number moved (two LOAD_STATE_SOURCE_EXCLUDED
    // warnings, DEL-04-07 Design pressure-contract.md "Load cases that apply nothing");
    // W-7 global end rows added; no other number moved (connected +48 rows, pressure +24 rows).
    for (name, digest) in [
        (
            "connected-sparse",
            "9494dfad726e83b36c67705a282da4c4cfdd8951632d24bc05e0c8e8276ace66",
        ),
        (
            "connected-dense",
            "2bfa5363081b93788a9a910368071aecdec970a295ba621377231566d592ed15",
        ),
        (
            "pressure-sparse",
            "861cb4f728d3450329f30b8b531416926133acfae8482701baac580c925b5bde",
        ),
        (
            "pressure-dense",
            "31a217d3044731453a34d4dff8abf39de05d0c69c39f7f3bc502baecb3637ee0",
        ),
    ] {
        assert_eq!(sha(source_text(name).as_bytes()), digest, "{name}");
        let raw = source(name);
        assert_eq!(raw["producer"]["semantic_contract_id"], s::PRESSURE_ID, "{name}");
        for record in raw["contract_evidence"]["load_reference_states"].as_array().unwrap() {
            assert_eq!((record["contract"].as_str(), record["profile"].as_str()), (Some(lr::RECORD_CONTRACT), Some(lr::RECORD_PROFILE)), "{name}");
        }
    }
    // Result rows use pressure-2's own reviewed table (B7b, O-10).
    assert_eq!(sha(TABLE.as_bytes()), s::PRESSURE_TABLE_SHA256);
    let table = s::pressure_rows_contract();
    assert_eq!(table["semantic_contract_id"], s::PRESSURE_ID);
    assert_eq!(table["formulation_profile_id"], s::PRESSURE_PROFILE);
    assert!(lr::transport_schema().is_ok());
    // B7b A5: load_reference_state.schema.json lost its v2 definitions.
    assert_eq!(
        lr::TRANSPORT_SCHEMA_SHA256,
        "5415d8f6d8856dba3470ab46423ba54d253e14a49680031d877a333b6d5d00a0"
    );
    assert_eq!(
        sha(include_bytes!("../../../../schemas/load_reference_state.schema.json")),
        lr::TRANSPORT_SCHEMA_SHA256
    );
}

#[test]
fn shared_adversarial_cases_match_in_rust() {
    let cases = cases();
    let mut log = Vec::new();
    // Every disagreement is collected and reported together.
    let mut misses = Vec::new();
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
                outcome(s::validate_pressure_evidence(&doc)),
            )
        };
        record(
            &mut log,
            json!({"id": id, "dispatch": dispatch, "validator": validator}),
        );
        if !agrees(&dispatch, expected(case, "dispatch")) {
            misses.push(format!("{id}: dispatch {dispatch} != {}", expected(case, "dispatch")));
        }
        if !agrees(&validator, expected(case, "validator")) {
            misses.push(format!("{id}: validator {validator} != {}", expected(case, "validator")));
        }
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
                s::PRESSURE_ID
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
    assert!(misses.is_empty(), "{} disagreements:\n{}", misses.len(), misses.join("\n"));
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
        let result = outcome(s::verify_pressure_table(text.as_bytes()));
        record(&mut log, json!({"id": case["id"], "table": result}));
        assert_eq!(result, case["expect"].as_str().unwrap(), "{}", case["id"]);
    }
    // B7b: pressure-2 has no transport (metadata-only) reader, so the retired
    // load-reference-1 transport cases have no current twin: the case file states
    // none, and a transported statement is refused by name while its header alone
    // still dispatches (header only; never a qualification of raw output).
    assert_eq!(cases["transport_cases"], json!([]));
    for name in RAW {
        let doc = source(name);
        let mut meta = json!({"schema_version": doc["schema_version"]});
        for key in ["producer", "numerical_quality", "formulation_basis", "contract_evidence"] {
            meta[key] = doc[key].clone();
        }
        let result = rv::transport("pressure", &meta);
        record(&mut log, json!({"id": name, "transport": result}));
        assert_eq!(result["error"]["code"], rv::PRESSURE_TRANSPORT_UNSUPPORTED, "{name}");
        assert!(s::for_source_metadata(&meta).is_ok(), "{name}");
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
            d::reference("semantic_contract", s::PRESSURE_ID)
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
            "../../../fixtures/results/pressure_load_state_{}.document.json",
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
                json!(s::PRESSURE_SOURCE_ID),
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
        s::PRESSURE_ID
    );
    assert!(s::for_source(&header).is_err());
    // One pressure-2 identity and profile with and without the group: the
    // ordinary statement dispatches to the same table.
    let physics = source("physics-sparse");
    assert!(physics["contract_evidence"].get("load_reference_states").is_none());
    assert_eq!(
        s::for_source(&physics).unwrap().0["semantic_contract_id"],
        s::PRESSURE_ID
    );
    // B7b: every retired v2 identity is refused by name at the header, whatever
    // profile it claims.
    for id in s::RETIRED_EXACT_V2_IDS {
        for profile in [s::PRESSURE_PROFILE, "exact_straight_pressure_v2", "resolved_straight_load_state_v1"] {
            for statement in [&physics, &raw] {
                let mut bad = statement.clone();
                bad["producer"]["semantic_contract_id"] = json!(id);
                bad["formulation_basis"]["profile_id"] = json!(profile);
                let refusal = s::for_source_metadata(&bad).unwrap_err();
                assert!(refusal.starts_with(&format!("{}: ", s::SOURCE_EXACT_PRESSURE_V2_RETIRED)), "{id} {profile}: {refusal}");
            }
        }
    }
}
