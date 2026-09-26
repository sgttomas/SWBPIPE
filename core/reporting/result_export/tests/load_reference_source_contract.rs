//! load-reference-source-1 readers on the ten committed joined producer
//! envelopes, with the adversarial cases shared with
//! `tests/test_load_reference_source_readers.py`. Both readers must reproduce
//! every expectation in the case file. This proves consumer admission and
//! carrier preservation, not producer authentication or numerical eligibility.
//! All inputs are invented.
use open_pipe_stress_result_export::{
    derivative as d, physics_source as ps, semantic_contract as s, source_blocks::domain_hash,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const CASES: &str = include_str!("fixtures/load_reference_source_mutations.json");
const TABLE: &str = include_str!(
    "../../../../fixtures/results/semantic_contract_v0_3_load_reference_source_1.json"
);
const INHERITED: &str = "SOURCE_LOAD_REFERENCE_JOIN_PHYSICS_SOURCE";
const MARKER: &str = "__LOAD_REFERENCE_SOURCE_TEXT__";
const JOINED: [&str; 10] = [
    "n05-sparse",
    "n05-dense",
    "n06-sparse",
    "n06-dense",
    "fields-sparse",
    "fields-dense",
    "mixed-sparse",
    "mixed-dense",
    "eigen_motion-sparse",
    "eigen_motion-dense",
];

fn source_text(name: &str) -> &'static str {
    match name {
        "n05-sparse" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/n05-sparse_interactive.raw.json"
        ),
        "n05-dense" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/n05-dense_scrutiny.raw.json"
        ),
        "n06-sparse" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/n06-sparse_interactive.raw.json"
        ),
        "n06-dense" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/n06-dense_scrutiny.raw.json"
        ),
        "fields-sparse" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/fields-sparse_interactive.raw.json"
        ),
        "fields-dense" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/fields-dense_scrutiny.raw.json"
        ),
        "mixed-sparse" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/mixed-sparse_interactive.raw.json"
        ),
        "mixed-dense" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/mixed-dense_scrutiny.raw.json"
        ),
        "eigen_motion-sparse" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/eigen_motion-sparse_interactive.raw.json"
        ),
        "eigen_motion-dense" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/eigen_motion-dense_scrutiny.raw.json"
        ),
        "lr-connected-sparse" => include_str!(
            "../../../../fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json"
        ),
        "lr-fallback-sparse" => {
            include_str!("fixtures/load_reference_fallback_uz-sparse_interactive.raw.json")
        }
        "lr-fallback-dense" => {
            include_str!("fixtures/load_reference_fallback_uz-dense_scrutiny.raw.json")
        }
        "ps-n05-sparse" => include_str!(
            "../../../../fixtures/product_preview/physics_source/n05-sparse_interactive.raw.json"
        ),
        _ => panic!("unknown source {name}"),
    }
}
fn request_text(name: &str) -> &'static str {
    match name.rsplit_once('-').map_or(name, |(stem, _)| stem) {
        "n05" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/n05.request.json"
        ),
        "n06" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/n06.request.json"
        ),
        "fields" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/fields.request.json"
        ),
        "mixed" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/mixed.request.json"
        ),
        "eigen_motion" => include_str!(
            "../../../../fixtures/product_preview/load_reference_source/eigen_motion.request.json"
        ),
        "lr-connected" => {
            include_str!(
                "../../../../fixtures/product_preview/load_reference/connected.request.json"
            )
        }
        "lr-fallback" => include_str!("fixtures/load_reference_fallback_uz.request.json"),
        "ps-n05" => {
            include_str!("../../../../fixtures/product_preview/physics_source/n05.request.json")
        }
        other => panic!("unknown request {other}"),
    }
}
fn source(name: &str) -> Value {
    serde_json::from_str(source_text(name)).unwrap()
}
fn model(name: &str) -> Value {
    serde_json::from_str::<Value>(request_text(name)).unwrap()["model"].clone()
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
/// Recompute the receipt commitments so that a later check can be reached.
fn reseal(doc: &mut Value, op: &Value) {
    let evidence = doc["contract_evidence"].clone();
    if !op["physical"].is_null() {
        let exact = evidence["exact_cases"].as_array().unwrap();
        for (index, case) in doc["source_block_recovery"]["body"]["cases"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .enumerate()
        {
            let pressure: Vec<&Value> = evidence["pressure"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|p| p["load_case_id"] == exact[index]["load_case_id"])
                .collect();
            let (domain, payload) = match op["physical"].as_str().unwrap() {
                "joined" => (
                    "load_reference_source_case_evidence_v1",
                    json!({"exact_case": exact[index], "pressure": pressure, "load_reference_state": evidence["load_reference_states"][index]}),
                ),
                "joined_without_record" => (
                    "load_reference_source_case_evidence_v1",
                    json!({"exact_case": exact[index], "pressure": pressure}),
                ),
                "physics" => (
                    "physics_source_case_evidence_v1",
                    json!({"exact_case": exact[index], "pressure": pressure}),
                ),
                other => panic!("unknown reseal {other}"),
            };
            case["physical_evidence_sha256"] = json!(domain_hash(domain, &payload).unwrap());
        }
    }
    if op["publication"] == true {
        let mut publication = doc.clone();
        publication
            .as_object_mut()
            .unwrap()
            .remove("source_block_recovery");
        doc["source_block_recovery"]["body"]["publication_sha256"] =
            json!(domain_hash("source_blocks_publication_v1", &publication).unwrap());
    }
    if op["receipt"] == true {
        doc["source_block_recovery"]["receipt_sha256"] = json!(domain_hash(
            "source_blocks_receipt_v1",
            &doc["source_block_recovery"]["body"]
        )
        .unwrap());
    }
}
/// Mirror of the Python harness. Returns the mutated document and, for a
/// non-finite or `json_text` op, the literal that must replace the marker.
fn apply(doc: &Value, ops: &Value) -> (Value, Option<String>) {
    let mut doc = doc.clone();
    let mut literal = None;
    for op in ops.as_array().unwrap() {
        let path = op["path"].as_str().unwrap_or("");
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
                set(&mut doc, path, json!(MARKER));
                literal = Some(
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
                assert!(literal.is_none(), "one text literal per case");
                set(&mut doc, path, json!(MARKER));
                literal = Some(op["value"].as_str().unwrap().to_string());
            }
            "reseal" => reseal(&mut doc, op),
            other => panic!("unknown op {other}"),
        }
    }
    (doc, literal)
}
fn outcome<T>(result: Result<T, String>) -> String {
    match result {
        Ok(_) => "accept".into(),
        Err(e) => e,
    }
}
fn agrees(actual: &str, expected: &str) -> bool {
    if expected == INHERITED {
        actual.split(": ").next() == Some(INHERITED)
    } else {
        actual == expected
    }
}
fn expected<'a>(case: &'a Value, kind: &str) -> Option<&'a str> {
    if let Some(value) = case[format!("{kind}_rust")].as_str() {
        return Some(value);
    }
    match kind {
        "joined" => case["joined"]
            .as_str()
            .or_else(|| expected(case, "dispatch")),
        _ => case[kind].as_str(),
    }
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
        d::reference(
            "test_carrier",
            "committed-load-reference-source-producer-output",
        ),
    )?;
    let origin = json!({"origin_id":"load-reference-source-reader-test","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Committed producer bytes; consumer contract test only","actual_model_ref":d::reference("model_payload",raw["model_ref"].as_str().unwrap_or("")),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap_or("")),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    d::derive_document(base, model, raw, origin, None)
}
fn write_log(name: &str, log: &[Value]) {
    if let Ok(dir) = std::env::var("LOAD_REFERENCE_SOURCE_PARITY_OUT") {
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            std::path::Path::new(&dir).join(name),
            serde_json::to_string_pretty(log).unwrap() + "\n",
        )
        .unwrap();
    }
}

#[test]
fn pinned_table_identity_profile_and_policy() {
    assert_eq!(sha(TABLE.as_bytes()), s::LOAD_REFERENCE_SOURCE_TABLE_SHA256);
    assert_eq!(
        s::LOAD_REFERENCE_SOURCE_TABLE_SHA256,
        "d1628194a7730f427843b00228dd233cf92b8e7d26f3bc31c660a3ea59e28337"
    );
    let table = s::load_reference_source_contract();
    assert_eq!(table["semantic_contract_id"], s::LOAD_REFERENCE_SOURCE_ID);
    assert_eq!(
        table["formulation_profile_id"],
        s::LOAD_REFERENCE_SOURCE_PROFILE
    );
    assert_eq!(table["source_block_policy"], "LOAD-REFERENCE-SOURCE-1");
    // Result rows use the physics-source-1 signatures unchanged.
    assert_eq!(table["rows"], s::physics_source_contract()["rows"]);
    assert_eq!(
        table["canonical_metadata_vocabulary"],
        s::physics_source_contract()["canonical_metadata_vocabulary"]
    );
    for name in JOINED {
        let raw = source(name);
        assert_eq!(
            raw["producer"]["semantic_contract_id"],
            s::LOAD_REFERENCE_SOURCE_ID
        );
        assert_eq!(
            raw["source_block_recovery"]["body"]["policy"],
            "LOAD-REFERENCE-SOURCE-1"
        );
    }
}

#[test]
fn shared_adversarial_cases_match_in_rust() {
    let cases = cases();
    let mut log = Vec::new();
    for case in cases["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let name = case["source"].as_str().unwrap();
        let (doc, literal) = apply(&source(name), &case["ops"]);
        let mut entry = json!({"id": id});
        if let Some(literal) = &literal {
            let text = serde_json::to_string(&doc)
                .unwrap()
                .replace(&format!("\"{MARKER}\""), literal);
            assert!(!text.contains(MARKER));
            assert!(
                serde_json::from_str::<Value>(&text).is_err(),
                "{id}: text literal parsed"
            );
            for kind in ["dispatch", "joined"] {
                entry[kind] = json!("JSON_PARSE_REJECTED");
            }
        } else {
            entry["dispatch"] = json!(outcome(s::for_source(&doc)));
            entry["joined"] = json!(outcome(s::validate_load_reference_source_evidence(&doc)));
            if case.get("lr").is_some() {
                entry["lr"] = json!(outcome(s::validate_load_reference_evidence(&doc)));
            }
            if case.get("ps").is_some() {
                entry["ps"] = json!(outcome(ps::validate(&doc, None)));
            }
        }
        log.push(entry.clone());
        for kind in ["dispatch", "joined", "lr", "ps"] {
            if let Some(actual) = entry[kind].as_str() {
                let want = expected(case, kind).unwrap_or_else(|| panic!("{id}: no {kind}"));
                assert!(agrees(actual, want), "{id}: {kind} {actual} != {want}");
            }
        }
        if literal.is_some() {
            continue;
        }
        let contract = case["accept_contract"]
            .as_str()
            .unwrap_or(s::LOAD_REFERENCE_SOURCE_ID);
        if entry["dispatch"] == "accept" {
            assert_eq!(
                s::for_source(&doc).unwrap().0["semantic_contract_id"],
                contract,
                "{id}"
            );
            let document = derive(&doc, &model(name)).unwrap_or_else(|e| panic!("{id}: {e}"));
            let e = &document["result_envelope"];
            assert_eq!(e["contract_evidence"], doc["contract_evidence"], "{id}");
            if contract == s::LOAD_REFERENCE_SOURCE_ID {
                assert_eq!(e["source_block_recovery"], doc["source_block_recovery"]);
            } else {
                assert!(e.get("source_block_recovery").is_none(), "{id}");
            }
            assert_eq!(
                s::numerical_use_standing(&doc, &bases(&doc)),
                "needs_recompute",
                "{id}"
            );
        } else {
            assert!(derive(&doc, &model(name)).is_err(), "{id}: derived");
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
        let mut text = match case["file"].as_str() {
            None => TABLE.to_string(),
            Some("fixtures/results/semantic_contract_v0_3_physics_source_1.json") => include_str!(
                "../../../../fixtures/results/semantic_contract_v0_3_physics_source_1.json"
            )
            .to_string(),
            Some("fixtures/results/semantic_contract_v0_3_load_reference_1.json") => include_str!(
                "../../../../fixtures/results/semantic_contract_v0_3_load_reference_1.json"
            )
            .to_string(),
            Some(other) => panic!("unknown table file {other}"),
        };
        if let Some(find) = case["find"].as_str() {
            assert!(text.contains(find));
            text = text.replacen(find, case["replace"].as_str().unwrap(), 1);
        }
        let result = outcome(s::verify_load_reference_source_table(text.as_bytes()));
        log.push(json!({"id": case["id"], "table": result}));
        assert_eq!(result, case["expect"].as_str().unwrap(), "{}", case["id"]);
    }
    for case in cases["transport_cases"].as_array().unwrap() {
        let (doc, literal) = apply(&source(case["source"].as_str().unwrap()), &case["ops"]);
        assert!(literal.is_none());
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
        let result = outcome(s::validate_load_reference_source_transport_metadata(&meta));
        log.push(json!({"id": case["id"], "transport": result}));
        assert!(
            agrees(&result, case["expect"].as_str().unwrap()),
            "{}: {result}",
            case["id"]
        );
        if result == "accept" {
            assert_eq!(
                s::for_source_metadata(&meta).unwrap().0["semantic_contract_id"],
                s::LOAD_REFERENCE_SOURCE_ID
            );
        }
    }
    write_log("rust_table_transport_outcomes.json", &log);
}

#[test]
fn canonical_documents_preserve_source_metadata_receipt_and_evidence_verbatim() {
    for name in JOINED {
        let raw = source(name);
        let doc = derive(&raw, &model(name)).unwrap();
        let e = &doc["result_envelope"];
        assert_eq!(doc["schema_version"], "0.3.0");
        for key in [
            "producer",
            "numerical_quality",
            "formulation_basis",
            "contract_evidence",
            "source_block_recovery",
        ] {
            assert_eq!(e[key], raw[key], "{name} {key}");
        }
        assert_eq!(
            e["semantic_contract_ref"],
            d::reference("semantic_contract", s::LOAD_REFERENCE_SOURCE_ID)
        );
        assert_eq!(
            e["row_accounting"].as_array().unwrap().len(),
            raw["results"].as_array().unwrap().len()
        );
        for account in e["row_accounting"].as_array().unwrap() {
            let row = &raw["results"][account["source_row_index"].as_u64().unwrap() as usize];
            if account["disposition"] == "exported_quantity" {
                let target = doc
                    .pointer(account["target_field_path"].as_str().unwrap())
                    .unwrap();
                assert_eq!(
                    target["magnitude"].as_f64().unwrap().to_bits(),
                    row["value"].as_f64().unwrap().to_bits()
                );
            }
        }
        let reloaded: Value = serde_json::from_slice(&serde_json::to_vec(&doc).unwrap()).unwrap();
        d::validate_document(&reloaded, &raw).unwrap();
        // Compact bytes plus a newline: the carriers are large and regenerated by rerun.
        let text = serde_json::to_string(&doc).unwrap() + "\n";
        let (stem, mode) = name.rsplit_once('-').unwrap();
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
            "../../../fixtures/results/load_reference_source_{stem}_{mode}.document.json"
        ));
        if std::env::var("LOAD_REFERENCE_SOURCE_WRITE_FIXTURES").as_deref() == Ok("1") {
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
                json!("not_joined"),
                "SOURCE_CONTRACT_EVIDENCE_BINDING_MISMATCH",
            ),
            (
                "/result_envelope/source_block_recovery/body/policy",
                json!("PHYSICS-SOURCE-1"),
                "SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH",
            ),
            (
                "/result_envelope/semantic_contract_ref/ref_id",
                json!(s::PHYSICS_SOURCE_ID),
                "SEMANTIC_CONTRACT_BINDING_MISMATCH",
            ),
            (
                "/result_envelope/formulation_basis/profile_id",
                json!(s::LOAD_REFERENCE_PROFILE),
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
fn header_dispatch_binds_one_profile_and_existing_readers_refuse_joined_labels() {
    let raw = source("mixed-dense");
    let mut header = raw.clone();
    header.as_object_mut().unwrap().remove("results");
    assert_eq!(
        s::for_source_metadata(&header).unwrap().0["semantic_contract_id"],
        s::LOAD_REFERENCE_SOURCE_ID
    );
    assert!(s::for_source(&header).is_err());
    // No other contract accepts the joined profile.
    for id in [
        s::PRECISION_ID,
        s::PHYSICS_ID,
        s::PHYSICS_SOURCE_ID,
        s::LOAD_REFERENCE_ID,
        "openpipestress.result_semantics/0.3.0/source-blocks-1",
    ] {
        let mut bad = raw.clone();
        bad["producer"]["semantic_contract_id"] = json!(id);
        bad.as_object_mut().unwrap().remove("source_block_recovery");
        bad["formulation_basis"]["profile_id"] = json!(s::LOAD_REFERENCE_SOURCE_PROFILE);
        assert_eq!(
            s::for_source_metadata(&bad).unwrap_err(),
            "SOURCE_FORMULATION_BASIS_UNSUPPORTED",
            "{id}"
        );
    }
    for name in JOINED {
        let raw = source(name);
        assert_eq!(
            s::validate_load_reference_evidence(&raw).unwrap_err(),
            "SOURCE_LOAD_REFERENCE_FOREIGN_METHOD_EVIDENCE"
        );
        assert_eq!(
            ps::validate(&raw, None).unwrap_err(),
            "PHYSICS_SOURCE_RECEIPT_SHAPE"
        );
    }
}
