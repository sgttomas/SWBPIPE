//! b7-plan R2: the native base-reader outcomes over the shared base corpus,
//! committed as `fixtures/results/base_reader_outcomes.json`. Each entry is the
//! `reader_verdict` outcome of one corpus statement: the dispatch entry (route,
//! table binding, header refusal, standing), its readers' validations and
//! transport checks, and each row's semantics for the unmutated statements
//! (by digest).
//! The desktop's wasm engine must give these same outcomes (R2's equivalence
//! gate); this test pins them and checks them against every Rust expectation
//! the corpus files state.
//!
//! `BASE_READER_OUTCOMES_WRITE=1` rewrites the committed file;
//! `BASE_READER_INPUTS_OUT=<dir>` also writes the materialized inputs
//! (`inputs.jsonl`) for a comparison outside Rust.
//!
//! B7b: the corpus is built from the current fixtures only. The exact-route
//! statements are the v3 twins (0.8.0/3.0.0/exact_pressure_v3) read by the
//! pressure-2 and pressure-source-1 readers, with and without the
//! load/reference-state group; the retired v2 identities appear only as named
//! refusals (the mutation files' relabel cases). The ledger cases apply to the
//! received joined statements directly (the v2 projection is gone).
use open_pipe_stress_result_export::{reader_verdict as rv, source_blocks::domain_hash};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

const OUTCOMES: &str = "fixtures/results/base_reader_outcomes.json";
const MARKER: &str = "__BASE_READER_TEXT_LITERAL__";
const LR_PHYSICS: &str = "SOURCE_LOAD_REFERENCE_PHYSICS_EVIDENCE";

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn read_text(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}
fn read(path: &str) -> Value {
    serde_json::from_str(&read_text(path)).unwrap()
}
/// A model file or a request's model.
fn model_at(path: &str) -> Value {
    let v = read(path);
    if v.get("model").is_some() { v["model"].clone() } else { v }
}

// RFC 6901 helpers and the corpus ops (the union of the corpus files' grammars:
// load_reference*_mutations.json, the tamper vector and the two moved case files).
fn tokens(path: &str) -> Vec<String> {
    path[1..].split('/').map(|t| t.replace("~1", "/").replace("~0", "~")).collect()
}
fn parent_path(path: &str) -> (String, String) {
    let t = tokens(path);
    let parent = t[..t.len() - 1].iter().map(|x| format!("/{}", x.replace('~', "~0").replace('/', "~1"))).collect();
    (parent, t.last().unwrap().clone())
}
fn set(doc: &mut Value, path: &str, value: Value) {
    let (parent, last) = parent_path(path);
    match doc.pointer_mut(&parent).unwrap_or_else(|| panic!("missing parent of {path}")) {
        Value::Array(a) => a[last.parse::<usize>().unwrap()] = value,
        Value::Object(o) => {
            o.insert(last, value);
        }
        _ => panic!("scalar parent of {path}"),
    }
}
fn matches(item: &Value, pattern: &Value) -> bool {
    pattern.as_object().unwrap().iter().all(|(k, v)| item.get(k) == Some(v))
}
/// The mutation files' `reseal` op (B7b: the current case-evidence domain, the
/// record bound by its case id, as the reader binds it).
fn reseal(doc: &mut Value, op: &Value) {
    let evidence = doc["contract_evidence"].clone();
    if !op["physical"].is_null() {
        let exact = evidence["exact_cases"].as_array().unwrap();
        for (index, case) in doc["source_block_recovery"]["body"]["cases"].as_array_mut().unwrap().iter_mut().enumerate() {
            let pressure: Vec<&Value> = evidence["pressure"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|p| p["load_case_id"] == exact[index]["load_case_id"])
                .collect();
            let record = evidence["load_reference_states"]
                .as_array()
                .and_then(|r| r.iter().find(|r| r["load_case_id"] == exact[index]["load_case_id"]))
                .cloned()
                .unwrap_or(Value::Null);
            let (domain, payload) = match op["physical"].as_str().unwrap() {
                "joined" => ("pressure_source_case_evidence_v1",
                    json!({"exact_case": exact[index], "pressure": pressure, "load_reference_state": record})),
                "joined_without_record" => ("pressure_source_case_evidence_v1", json!({"exact_case": exact[index], "pressure": pressure})),
                "physics" => ("physics_source_case_evidence_v1", json!({"exact_case": exact[index], "pressure": pressure})),
                other => panic!("unknown reseal {other}"),
            };
            case["physical_evidence_sha256"] = json!(domain_hash(domain, &payload).unwrap());
        }
    }
    if op["publication"] == true {
        let mut publication = doc.clone();
        publication.as_object_mut().unwrap().remove("source_block_recovery");
        doc["source_block_recovery"]["body"]["publication_sha256"] =
            json!(domain_hash("source_blocks_publication_v1", &publication).unwrap());
    }
    if op["receipt"] == true {
        doc["source_block_recovery"]["receipt_sha256"] =
            json!(domain_hash("source_blocks_receipt_v1", &doc["source_block_recovery"]["body"]).unwrap());
    }
}
/// The mutated document and, for a `nonfinite` or `json_text` op, the literal
/// that replaces the marker in its text.
fn apply(doc: &Value, ops: &Value, sources: &dyn Fn(&str) -> Value) -> (Value, Option<String>) {
    let mut doc = doc.clone();
    let mut literal = None;
    for op in ops.as_array().unwrap() {
        let path = op["path"].as_str().unwrap_or("");
        match op["op"].as_str().unwrap() {
            "set" | "replace" => set(&mut doc, path, op["value"].clone()),
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
            "append" => doc.pointer_mut(path).unwrap().as_array_mut().unwrap().push(op["value"].clone()),
            "add" => {
                let (parent, last) = parent_path(path);
                match doc.pointer_mut(&parent).unwrap() {
                    Value::Object(o) => {
                        o.insert(last, op["value"].clone());
                    }
                    Value::Array(a) if last == "-" => a.push(op["value"].clone()),
                    Value::Array(a) => a.insert(last.parse::<usize>().unwrap(), op["value"].clone()),
                    _ => panic!("scalar parent"),
                }
            }
            "reverse" => doc.pointer_mut(path).unwrap().as_array_mut().unwrap().reverse(),
            "copy_append" => {
                let value = doc.pointer(op["from"].as_str().unwrap()).unwrap().clone();
                doc.pointer_mut(path).unwrap().as_array_mut().unwrap().push(value);
            }
            "swap" => {
                let other = op["with"].as_str().unwrap();
                let (a, b) = (doc.pointer(path).unwrap().clone(), doc.pointer(other).unwrap().clone());
                set(&mut doc, path, b);
                set(&mut doc, other, a);
            }
            "graft" => {
                let value = sources(op["source"].as_str().unwrap()).pointer(op["from"].as_str().unwrap()).unwrap().clone();
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
                let mut copy = array.iter().find(|item| matches(item, &op["match"])).unwrap().clone();
                for (k, v) in op["set"].as_object().unwrap() {
                    copy[k] = v.clone();
                }
                array.push(copy);
            }
            "nonfinite" => {
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
                set(&mut doc, path, json!(MARKER));
                literal = Some(op["value"].as_str().unwrap().to_string());
            }
            "reseal" => reseal(&mut doc, op),
            other => panic!("unknown op {other}"),
        }
    }
    (doc, literal)
}

/// One corpus statement and what is read of it.
struct Input {
    id: String,
    source: Result<Value, String>,
    model: Option<Value>,
    invocation: Option<Value>,
    readers: Vec<&'static str>,
    transports: Vec<&'static str>,
    rows: bool,
}
fn route_reader(route: &str) -> Option<&'static str> {
    Some(match route {
        "preview_physics" => "preview-physics",
        "pressure" => "pressure",
        "pressure_source" => "pressure-source",
        _ => return None,
    })
}
/// The transported statement: the members a carrier keeps (the corpus's
/// transport cases' projection).
fn transported(doc: &Value) -> Value {
    let mut meta = json!({"schema_version": doc["schema_version"]});
    for key in ["producer", "numerical_quality", "formulation_basis", "contract_evidence", "source_block_recovery", "carrier_evidence"] {
        if let Some(v) = doc.get(key) {
            meta[key] = v.clone();
        }
    }
    meta
}
fn materialized(doc: Value, literal: Option<String>) -> Result<Value, String> {
    match literal {
        None => Ok(doc),
        Some(literal) => {
            let text = serde_json::to_string(&doc).unwrap().replace(&format!("\"{MARKER}\""), &literal);
            assert!(serde_json::from_str::<Value>(&text).is_err(), "text literal parsed");
            Err(text)
        }
    }
}

/// A base statement: its route's reader, transport and rows.
fn base(inputs: &mut Vec<Input>, id: String, source: Value, model: Option<Value>, invocation: Option<Value>) {
    let reader = rv::route(&source).ok().and_then(|(r, _)| route_reader(r));
    inputs.push(Input {
        id,
        source: Ok(source),
        model,
        invocation,
        readers: reader.into_iter().collect(),
        transports: reader.into_iter().collect(),
        rows: true,
    });
}

fn corpus() -> &'static [Input] {
    static CORPUS: std::sync::OnceLock<Vec<Input>> = std::sync::OnceLock::new();
    CORPUS.get_or_init(build_corpus)
}

fn build_corpus() -> Vec<Input> {
    let mut inputs = Vec::new();
    // Result fixtures, each with its model.
    let results: [(&str, Option<&str>); 17] = [
        ("preview_physics_connected_sparse", Some("fixtures/model_operations/precision_connected_ui_model.json")),
        ("preview_physics_connected_dense", Some("fixtures/model_operations/precision_connected_ui_model.json")),
        ("preview_physics_invented_sparse", Some("core/product_physics/tests/fixtures/preview_physics_invented_model.json")),
        ("preview_physics_invented_dense", Some("core/product_physics/tests/fixtures/preview_physics_invented_model.json")),
        ("preview_physics_unicode_ids_sparse", Some("core/product_physics/tests/fixtures/preview_physics_unicode_ids_model.json")),
        ("pressure_connected_mechanics_sparse", Some("core/product_physics/tests/fixtures/exact_pressure_connected_request.json")),
        ("pressure_connected_mechanics_dense", Some("core/product_physics/tests/fixtures/exact_pressure_connected_request.json")),
        ("pressure_connected_ui_mechanics_sparse", Some("fixtures/model_operations/exact_pressure_authoring_model.json")),
        ("pressure_connected_ui_mechanics_dense", Some("fixtures/model_operations/exact_pressure_authoring_model.json")),
        ("pressure_thermal_ui_mechanics_sparse", Some("fixtures/model_operations/physics_thermal_ui_model.json")),
        ("pressure_thermal_ui_mechanics_dense", Some("fixtures/model_operations/physics_thermal_ui_model.json")),
        // T4-U4b review item 5: a directional pressure-2 arc and a preview arc envelope.
        ("pressure_u4b_arc_sparse", Some("core/product_physics/tests/fixtures/u4b_l90_p_model.json")),
        ("preview_physics_u4b_arc_sparse", Some("core/product_physics/tests/fixtures/u4b_z_preview_rev_model.json")),
        ("../product_preview/invented_mechanics_result_preview_physics_1_sparse", None),
        ("../product_preview/invented_mechanics_result_preview_physics_1_dense", None),
        ("../product_preview/invented_demo_result_preview_physics_1_sparse", None),
        ("../product_preview/invented_demo_result_preview_physics_1_dense", None),
    ];
    for (name, model) in results {
        let path = format!("fixtures/results/{name}.json");
        base(&mut inputs, format!("results/{}", name.trim_start_matches("../")), read(&path), model.map(model_at), None);
    }
    // Raw source fixtures with their requests: the model, and the invocation for
    // the source routes.
    let modes = ["sparse_interactive", "dense_scrutiny"];
    for (dir, names, invoked) in [
        ("pressure_source/physics_source", &["n05", "n06", "fields", "mixed", "mixed_units", "n05_units", "n05_unicode"][..], true),
        ("pressure/load_reference", &["connected", "pressure"][..], false),
        // The joined load/reference-state statements are read by the one source
        // reader, which takes their invocation (never numerically eligible).
        ("pressure_source/load_reference_source", &["n05", "n06", "fields", "mixed", "eigen_motion"][..], true),
    ] {
        for name in names {
            let request = read(&format!("fixtures/product_preview/{dir}/{name}.request.json"));
            for mode in modes {
                let source = read(&format!("fixtures/product_preview/{dir}/{name}-{mode}.raw.json"));
                let invocation = invoked.then(|| json!({"request": request, "solver_mode": mode}));
                base(&mut inputs, format!("{dir}/{name}-{mode}"), source, Some(request["model"].clone()), invocation);
            }
        }
    }
    // B4/D1 (b): the preview route's retired source-blocks-1 statements left the corpus.
    let fallback = model_at("core/reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json");
    for mode in modes {
        let path = format!("core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-{mode}.raw.json");
        base(&mut inputs, format!("load_reference_fallback_uz-{mode}"), read(&path), Some(fallback.clone()), None);
    }
    // W-16 slice b: the shared pressure-2 combination corpus, each envelope with its model.
    for case in read("fixtures/results/pressure_v3_combination_reader_corpus.json")["cases"].as_array().unwrap() {
        base(&mut inputs, format!("combination/{}", case["label"].as_str().unwrap()), case["envelope"].clone(), Some(case["model"].clone()), None);
    }
    // The shared tamper vector (preview-physics).
    let vector = read("fixtures/results/preview_physics_tamper_vector.json");
    for variant in vector["variants"].as_array().unwrap() {
        let base_doc = read(vector["bases"][variant["base"].as_str().unwrap()].as_str().unwrap());
        let (doc, literal) = apply(&base_doc, &variant["ops"], &|_| unreachable!());
        assert!(literal.is_none());
        // Codex round 2: a variant with a transport expectation is also read by transport.
        let transports = if variant.get("transport").is_some() { vec!["preview-physics"] } else { vec![] };
        inputs.push(Input { id: format!("tamper/{}", variant["id"].as_str().unwrap()), source: Ok(doc), model: None,
            invocation: None, readers: vec!["preview-physics"], transports, rows: false });
    }
    // load_reference*_mutations.json: each case under its reader(s), and the
    // transport cases on the transported members.
    for (file, reader, extra) in [
        ("core/reporting/result_export/tests/fixtures/load_reference_mutations.json", "pressure", false),
        ("core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json", "pressure-source", true),
    ] {
        let cases = read(file);
        let sources: BTreeMap<String, Value> = cases["sources"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), read(v.as_str().unwrap())))
            .collect();
        let lookup = |name: &str| sources[name].clone();
        let prefix = if extra { "lrs" } else { "lr" };
        for case in cases["cases"].as_array().unwrap() {
            let (doc, literal) = apply(&sources[case["source"].as_str().unwrap()], &case["ops"], &lookup);
            let mut readers = vec![reader];
            if extra && case.get("lr").is_some() {
                readers.push("pressure");
            }
            assert!(case.get("ps").is_none(), "B7b folded 'ps' into 'joined'");
            inputs.push(Input { id: format!("{prefix}/{}", case["id"].as_str().unwrap()), source: materialized(doc, literal),
                model: None, invocation: None, readers, transports: vec![], rows: false });
        }
        for case in cases["transport_cases"].as_array().unwrap() {
            let (doc, literal) = apply(&sources[case["source"].as_str().unwrap()], &case["ops"], &lookup);
            assert!(literal.is_none());
            inputs.push(Input { id: format!("{prefix}-transport/{}", case["id"].as_str().unwrap()), source: Ok(transported(&doc)),
                model: None, invocation: None, readers: vec![], transports: vec![reader], rows: false });
        }
    }
    // The two case files the TypeScript tests authored, now shared.
    for case in read("fixtures/results/loadReferenceReaderCases.cases.json").as_array().unwrap() {
        let (doc, literal) = apply(&read(case["source"].as_str().unwrap()), &case["ops"], &|_| unreachable!());
        assert!(literal.is_none());
        inputs.push(Input { id: format!("lr-reader/{}", case["id"].as_str().unwrap()), source: Ok(doc), model: None,
            invocation: None, readers: vec!["pressure"], transports: vec![], rows: false });
    }
    // The ledger cases: the source reader's ledger checks on the received joined
    // statement (B7b: no projection; the ops read the same members).
    let mut sources: BTreeMap<String, Value> = BTreeMap::new();
    for case in read("fixtures/results/loadReferenceSourceLedger.cases.json").as_array().unwrap() {
        let name = case["source"].as_str().unwrap();
        let received = sources
            .entry(name.into())
            .or_insert_with(|| read(&format!("fixtures/product_preview/pressure_source/load_reference_source/{name}.raw.json")))
            .clone();
        let (mut doc, literal) = apply(&received, &case["ops"], &|_| unreachable!());
        assert!(literal.is_none());
        if let Some(publication) = case["reseal_publication"].as_bool() {
            reseal(&mut doc, &json!({"physical": null, "publication": publication, "receipt": true}));
        }
        inputs.push(Input { id: format!("ledger/{}", case["id"].as_str().unwrap()), source: Ok(doc), model: None,
            invocation: None, readers: vec!["pressure-source"], transports: vec![], rows: false });
    }
    inputs
}

fn outcome_of(input: &Input) -> Value {
    let source = match &input.source {
        Err(_) => return json!({"id": input.id, "parse": "JSON_PARSE_REJECTED"}),
        Ok(source) => source,
    };
    let (model, invocation) = (input.model.as_ref(), input.invocation.as_ref());
    let mut entry = Map::new();
    entry.insert("id".into(), json!(input.id));
    entry.insert("dispatch".into(), rv::dispatch(source, model, invocation));
    if !input.readers.is_empty() {
        let validate: Map<String, Value> =
            input.readers.iter().map(|r| (r.to_string(), rv::validate(r, source, model, invocation))).collect();
        entry.insert("validate".into(), Value::Object(validate));
    }
    if !input.transports.is_empty() {
        let transport: Map<String, Value> = input.transports.iter().map(|r| (r.to_string(), rv::transport(r, source))).collect();
        entry.insert("transport".into(), Value::Object(transport));
    }
    if input.rows {
        let rows: Vec<Value> = source["results"]
            .as_array()
            .map(|rows| rows.iter().map(|row| rv::row_semantics(row, Some(source))).collect())
            .unwrap_or_default();
        // Each row's semantics, pinned by digest (the full outcomes are the
        // table's rows and run to megabytes): the sha256 of the outcomes' JSON
        // text as an array, and each row's signature id (or refusal code).
        use sha2::{Digest, Sha256};
        let digest = format!("{:x}", Sha256::digest(serde_json::to_string(&rows).unwrap().as_bytes()));
        let ids: Vec<&Value> =
            rows.iter().map(|r| if r["ok"] == true { &r["signature"]["signature_id"] } else { &r["error"]["code"] }).collect();
        entry.insert("rows".into(), json!({"sha256": digest, "signature_ids": ids}));
    }
    Value::Object(entry)
}

/// An outcome's text as the corpus files state Rust outcomes: "accept" or the
/// reader's whole error text.
fn text(outcome: &Value) -> String {
    if outcome["ok"] == true {
        return "accept".into();
    }
    let e = &outcome["error"];
    e["detail"].as_str().or(e["code"].as_str()).unwrap().to_string()
}
fn agrees(actual: &str, expected: &str) -> bool {
    if expected == LR_PHYSICS {
        actual.split(": ").next() == Some(expected)
    } else {
        actual == expected
    }
}

/// The corpus files' Rust expectations against the outcomes: each reader's
/// stated outcome, and the raw dispatch (`for_source`: header and reader) as the
/// dispatch entry's header refusal or, when the header admits, the reader's.
fn check_expectations(outcomes: &BTreeMap<String, Value>) {
    let mut checked = 0;
    // Every reader disagreement is collected and reported together.
    let mut misses: Vec<String> = Vec::new();
    for (file, prefix, reader) in [
        ("core/reporting/result_export/tests/fixtures/load_reference_mutations.json", "lr", "pressure"),
        ("core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json", "lrs", "pressure-source"),
    ] {
        let cases = read(file);
        let joined = prefix == "lrs";
        for case in cases["cases"].as_array().unwrap() {
            let id = case["id"].as_str().unwrap();
            let entry = &outcomes[&format!("{prefix}/{id}")];
            let stated = |kind: &str| -> Option<String> {
                case[format!("{kind}_rust")].as_str().or(case[kind].as_str()).map(String::from)
            };
            let dispatch = stated("dispatch").unwrap();
            let own = if joined { stated("joined") } else { stated("validator") }.unwrap_or_else(|| dispatch.clone());
            if entry.get("parse").is_some() {
                assert_eq!((dispatch.as_str(), own.as_str()), ("JSON_PARSE_REJECTED", "JSON_PARSE_REJECTED"), "{id}");
                checked += 1;
                continue;
            }
            let got = text(&entry["validate"][reader]);
            if !agrees(&got, &own) {
                misses.push(format!("{prefix}/{id}: {reader} {got} != {own}"));
            }
            for (kind, other) in [("lr", "pressure")] {
                if let Some(want) = joined.then(|| stated(kind)).flatten() {
                    let got = text(&entry["validate"][other]);
                    if !agrees(&got, &want) {
                        misses.push(format!("{prefix}/{id}: {kind} {got} != {want}"));
                    }
                }
            }
            // The dispatch entry: an admitted raw dispatch is an admitted header
            // whose route's reader admits it; a refused one is refused by the
            // header or by that reader.
            let d = &entry["dispatch"];
            let header = d["ok"] == true;
            let route_reader = route_reader(d["route"].as_str().unwrap());
            if dispatch == "accept" {
                if !header {
                    misses.push(format!("{prefix}/{id}: header refused {}", d["error"]));
                }
                if route_reader == Some(reader) && got != "accept" {
                    misses.push(format!("{prefix}/{id}: {got} != dispatch accept"));
                }
            } else if header && route_reader == Some(reader) && !agrees(&got, &dispatch) {
                misses.push(format!("{prefix}/{id}: {got} != dispatch {dispatch}"));
            }
            checked += 1;
        }
        for case in cases["transport_cases"].as_array().unwrap() {
            let id = case["id"].as_str().unwrap();
            let got = text(&outcomes[&format!("{prefix}-transport/{id}")]["transport"][reader]);
            if !agrees(&got, case["expect"].as_str().unwrap()) {
                misses.push(format!("{prefix}-transport/{id}: {got} != {}", case["expect"]));
            }
            checked += 1;
        }
    }
    let vector = read("fixtures/results/preview_physics_tamper_vector.json");
    for variant in vector["variants"].as_array().unwrap() {
        let id = variant["id"].as_str().unwrap();
        let entry = &outcomes[&format!("tamper/{id}")];
        let admitted = entry["dispatch"]["ok"] == true && entry["validate"]["preview-physics"]["ok"] == true;
        assert_eq!(admitted, variant["expect"] == "accepted", "tamper/{id}");
        checked += 1;
        if let Some(expect) = variant.get("transport") {
            assert_eq!(entry["transport"]["preview-physics"]["ok"] == true, expect == "accepted", "tamper/{id} transport");
            checked += 1;
        }
    }
    // B7b: the load/reference-state cases (210 retired-reader cases re-authored,
    // plus 2 added named refusals; their 14 transport cases retired with the v2
    // transport reader), the joined cases (116 less 2 retired relabels, and their
    // 14 transport cases) and the tamper vector. T4-U4b (§5.1): four pressure and
    // three preview tampers of the new kinds; review 2: the basis-twin order swap;
    // review 6: the preview arc withholding revived; review 3 and 5: six pressure
    // tampers on the directional arc envelope and the preview station cover.
    // Codex round 2: nine malformed directional records, each read and
    // transported, and the revival also transported.
    assert_eq!(checked, 223 + 114 + 14 + 36 + 10);
    assert!(misses.is_empty(), "{} disagreements:\n{}", misses.len(), misses.join("\n"));
}

#[test]
fn base_reader_outcomes_are_the_committed_corpus_outcomes() {
    let inputs = corpus();
    let mut ids = std::collections::HashSet::new();
    assert!(inputs.iter().all(|i| ids.insert(i.id.clone())), "duplicate id");
    let entries: Vec<Value> = inputs.iter().map(outcome_of).collect();
    let by_id: BTreeMap<String, Value> = entries.iter().map(|e| (e["id"].as_str().unwrap().to_string(), e.clone())).collect();
    check_expectations(&by_id);
    // Every unmutated statement passes its route's reader with its own model and
    // invocation.
    for input in inputs.iter().filter(|i| i.rows) {
        let entry = &by_id[&input.id];
        assert_eq!(entry["dispatch"]["ok"], true, "{}", input.id);
        for (reader, outcome) in entry["validate"].as_object().into_iter().flatten() {
            assert_eq!(outcome["ok"], true, "{} {reader}: {outcome}", input.id);
        }
    }
    let document = json!({
        "description": "b7-plan R2: the native reader_verdict outcomes of the shared base corpus (tests/base_reader_outcomes.rs builds the inputs from the corpus files). The desktop's wasm engine gives these same outcomes.",
        "entries": entries,
    });
    let text = serde_json::to_string_pretty(&document).unwrap() + "\n";
    if let Ok(dir) = std::env::var("BASE_READER_INPUTS_OUT") {
        std::fs::create_dir_all(&dir).unwrap();
        let mut lines = String::new();
        for input in inputs {
            // Preserve the source JSON texts, including negative zero and unsafe
            // integers; a value-preserving numeric re-serialization gives the
            // same verdict, as the A2 test below checks.
            let text = |v: &Value| serde_json::to_string(v).unwrap();
            let (source, rows) = match &input.source {
                Ok(v) => (json!(text(v)), json!(v["results"].as_array().map(|r| r.iter().map(text).collect::<Vec<_>>()))),
                Err(t) => (json!(t), Value::Null),
            };
            lines += &(json!({"id": input.id, "source_json": source, "parsed": input.source.is_ok(),
                "model_json": input.model.as_ref().map(text), "invocation_json": input.invocation.as_ref().map(text),
                "readers": input.readers, "transports": input.transports, "rows": input.rows, "rows_json": rows})
                .to_string()
                + "\n");
        }
        std::fs::write(std::path::Path::new(&dir).join("inputs.jsonl"), lines).unwrap();
    }
    if std::env::var("BASE_READER_OUTCOMES_WRITE").is_ok() {
        std::fs::write(root().join(OUTCOMES), &text).unwrap();
    }
    assert!(read_text(OUTCOMES) == text, "{OUTCOMES} differs from the native outcomes");
}

/// b7-plan R2 A2: the JCS form of a value (`canonical_json`, the form every
/// hash-by-value copy has) and the `JSON.stringify` form (ECMAScript numbers; an
/// integer beyond 2^53 is a binary64 there).
fn jcs_form(value: &Value) -> Value {
    serde_json::from_str(&open_pipe_stress_canonical_json::canonical_json(value)).unwrap()
}
fn stringify_form(value: &Value) -> Value {
    match value {
        Value::Number(n) => {
            let safe = n.as_u64().map(|u| u <= 1 << 53).or(n.as_i64().map(|i| i.unsigned_abs() <= 1 << 53));
            if safe == Some(true) {
                return value.clone();
            }
            serde_json::from_str(&open_pipe_stress_canonical_json::ecma_number_to_string(n.as_f64().unwrap())).unwrap()
        }
        Value::Array(items) => Value::Array(items.iter().map(stringify_form).collect()),
        Value::Object(map) => Value::Object(map.iter().map(|(k, v)| (k.clone(), stringify_form(v))).collect()),
        other => other.clone(),
    }
}

/// Whether a re-serialized value holds the same binary64 values, bit for bit, as
/// the original: false where it wrote `-0` as `0` or rounded an integer beyond 2^53.
fn same_bits(a: &Value, b: &Value) -> bool {
    fn integer(n: &serde_json::Number) -> Option<i128> {
        n.as_i64().map(i128::from).or_else(|| n.as_u64().map(i128::from))
    }
    fn exact(f: f64, i: i128) -> bool {
        f.fract() == 0.0 && !(f == 0.0 && f.is_sign_negative()) && f.abs() < 1.7e38 && f as i128 == i
    }
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => match (integer(x), integer(y)) {
            (Some(i), Some(j)) => i == j,
            (Some(i), None) => exact(y.as_f64().unwrap(), i),
            (None, Some(j)) => exact(x.as_f64().unwrap(), j),
            (None, None) => x.as_f64().unwrap().to_bits() == y.as_f64().unwrap().to_bits(),
        },
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(x, y)| same_bits(x, y)),
        (Value::Object(x), Value::Object(y)) => x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| same_bits(v, w))),
        _ => a == b,
    }
}

/// A2: the readers read numbers by value. Every corpus statement, with its model
/// and invocation, re-serialized in either form, gives its committed outcome.
#[test]
fn base_reader_outcomes_hold_for_re_serialized_inputs() {
    let (mut differing, mut changed) = (Vec::new(), Vec::new());
    for input in corpus() {
        let Ok(source) = &input.source else { continue };
        let want = outcome_of(&input);
        // Each form for the whole input, and for the source or its model and
        // invocation alone (a caller may hold them in different forms).
        let forms: [(&str, fn(&Value) -> Value, fn(&Value) -> Value); 4] = [
            ("jcs", jcs_form, jcs_form),
            ("stringify", stringify_form, stringify_form),
            ("source-only", stringify_form, |v| v.clone()),
            ("inputs-only", |v| v.clone(), stringify_form),
        ];
        for (form, f, g) in forms {
            let moved = Input {
                id: input.id.clone(),
                source: Ok(f(source)),
                model: input.model.as_ref().map(g),
                invocation: input.invocation.as_ref().map(g),
                readers: input.readers.clone(),
                transports: input.transports.clone(),
                rows: input.rows,
            };
            let preserved = [(Some(source), moved.source.as_ref().ok()), (input.model.as_ref(), moved.model.as_ref()), (input.invocation.as_ref(), moved.invocation.as_ref())]
                .iter()
                .all(|(a, b)| a.zip(*b).is_none_or(|(a, b)| same_bits(a, b)));
            // A form that changed a value (a `-0` written `0`, an integer beyond 2^53
            // rounded) reads another statement; where its outcome differs it is listed.
            let got = outcome_of(&moved);
            if got != want && !preserved {
                changed.push(format!("{} [{form}]", input.id));
            } else if got != want {
                differing.push(format!("{} [{form}]: {} -> {}", input.id, serde_json::to_string(&want).unwrap().chars().take(600).collect::<String>(), serde_json::to_string(&got).unwrap().chars().take(600).collect::<String>()));
            }
        }
    }
    if let Ok(path) = std::env::var("A2_BY_VALUE_OUT") {
        std::fs::write(format!("{path}/base.txt"), differing.join("\n")).unwrap();
        std::fs::write(format!("{path}/base_value_changed.txt"), changed.join("\n")).unwrap();
    }
    // Only the unsafe-integer mutation's JSON.stringify forms change an outcome.
    assert_eq!(changed, ["lrs/NUM-unsafe-integer-in-publication [stringify]", "lrs/NUM-unsafe-integer-in-publication [source-only]"]);
    assert!(differing.is_empty(), "{} outcomes differ:\n{}", differing.len(), differing.iter().take(40).cloned().collect::<Vec<_>>().join("\n"));
}
