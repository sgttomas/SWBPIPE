//! DEL-04-09 I3a (Design §5.3): the Rust modal reader over the committed
//! corpus `fixtures/results/modal_corpus.json`. Each entry tampers one
//! relation of a committed modal fixture (or none); the reader must return the
//! entry's `expected` verdict, which the independent Python reader
//! (`tests/test_modal_evidence.py`) is held to as well, and the standing the
//! entry targets must fail. All values are invented.
use open_pipe_stress_modal_result::{
    modal_reader_semantic_table, modal_reader_sha256, read_modal_result, read_modal_result_text, ModalReadStatus, ModalSupplied,
    MODAL_RESULT_SEMANTICS_SHA256,
};
use serde_json::{json, Value};

const CORPUS: &str = include_str!("../../../../fixtures/results/modal_corpus.json");
/// Point-sum evaluation-order boundaries (Design §5.3.4), in the corpus entry format.
const SUM_ORDER: &str = include_str!("../../../../fixtures/results/modal_sum_order_boundaries.json");
const SEMANTIC_TABLE: &str = include_str!("../../../../fixtures/results/semantic_contract_modal_1.json");
/// The feasibility oracle's cases and verdicts (`tests/modal_feasibility_oracle.py`).
const ORACLE_CASES: &str = include_str!("../../../../fixtures/results/modal_oracle_cases.json");
/// Producer-formed edges (`core/product_physics/src/modal/tests_document.rs`).
const PRODUCER_EDGES: &str = include_str!("../../../../fixtures/results/modal_producer_edges.json");

fn fixture(name: &str) -> &'static str {
    match name {
        "modal_ex_n05.json" => include_str!("../../../../fixtures/results/modal_ex_n05.json"),
        "modal_ex_n05.request.json" => include_str!("../../../../fixtures/results/modal_ex_n05.request.json"),
        "modal_arc.json" => include_str!("../../../../fixtures/results/modal_arc.json"),
        "modal_arc.request.json" => include_str!("../../../../fixtures/results/modal_arc.request.json"),
        "modal_component_body.json" => include_str!("../../../../fixtures/results/modal_component_body.json"),
        "modal_component_body.request.json" => include_str!("../../../../fixtures/results/modal_component_body.request.json"),
        "modal_state_one_way_pressed.json" => include_str!("../../../../fixtures/results/modal_state_one_way_pressed.json"),
        "modal_state_one_way_pressed.request.json" => include_str!("../../../../fixtures/results/modal_state_one_way_pressed.request.json"),
        "modal_state_friction_sliding.json" => include_str!("../../../../fixtures/results/modal_state_friction_sliding.json"),
        "modal_state_friction_sliding.request.json" => include_str!("../../../../fixtures/results/modal_state_friction_sliding.request.json"),
        "modal_cantilever_whole_spectrum.json" => include_str!("../../../../fixtures/results/modal_cantilever_whole_spectrum.json"),
        "modal_cantilever_whole_spectrum.request.json" => include_str!("../../../../fixtures/results/modal_cantilever_whole_spectrum.request.json"),
        "modal_cantilever_cluster_extension.json" => include_str!("../../../../fixtures/results/modal_cantilever_cluster_extension.json"),
        "modal_cantilever_cluster_extension.request.json" => include_str!("../../../../fixtures/results/modal_cantilever_cluster_extension.request.json"),
        "modal_cantilever_below_cutoff.json" => include_str!("../../../../fixtures/results/modal_cantilever_below_cutoff.json"),
        "modal_cantilever_below_cutoff.request.json" => include_str!("../../../../fixtures/results/modal_cantilever_below_cutoff.request.json"),
        "modal_cantilever_subnormal_cutoff.json" => include_str!("../../../../fixtures/results/modal_cantilever_subnormal_cutoff.json"),
        "modal_cantilever_subnormal_cutoff.request.json" => include_str!("../../../../fixtures/results/modal_cantilever_subnormal_cutoff.request.json"),
        "modal_cantilever_sensitive.json" => include_str!("../../../../fixtures/results/modal_cantilever_sensitive.json"),
        "modal_cantilever_sensitive.request.json" => include_str!("../../../../fixtures/results/modal_cantilever_sensitive.request.json"),
        "modal_refused_unseparated.json" => include_str!("../../../../fixtures/results/modal_refused_unseparated.json"),
        "modal_refused_unseparated.request.json" => include_str!("../../../../fixtures/results/modal_refused_unseparated.request.json"),
        "modal_refused_no_bound.json" => include_str!("../../../../fixtures/results/modal_refused_no_bound.json"),
        "modal_refused_no_bound.request.json" => include_str!("../../../../fixtures/results/modal_refused_no_bound.request.json"),
        other => panic!("no committed modal fixture `{other}`"),
    }
}

fn pointer_parts(path: &str) -> Vec<String> {
    path.split('/').skip(1).map(|p| p.replace("~1", "/").replace("~0", "~")).collect()
}

fn parent_of<'a>(doc: &'a mut Value, parts: &[String]) -> &'a mut Value {
    let mut node = doc;
    for part in &parts[..parts.len() - 1] {
        node = match node {
            Value::Array(items) => &mut items[part.parse::<usize>().unwrap()],
            other => other.get_mut(part.as_str()).unwrap_or_else(|| panic!("no member `{part}`")),
        };
    }
    node
}

/// The corpus operations (module docs).
fn apply(doc: &mut Value, operations: &[Value]) {
    for op in operations {
        match op["op"].as_str().unwrap() {
            "replace" | "add" | "scale" => {
                let parts = pointer_parts(op["path"].as_str().unwrap());
                let last = parts.last().unwrap().clone();
                let parent = parent_of(doc, &parts);
                let slot = match parent {
                    Value::Array(items) => &mut items[last.parse::<usize>().unwrap()],
                    Value::Object(map) => {
                        if op["op"] == "add" {
                            assert!(!map.contains_key(&last), "add of an existing member");
                            map.insert(last.clone(), Value::Null);
                        } else {
                            assert!(map.contains_key(&last), "{op}");
                        }
                        map.get_mut(&last).unwrap()
                    }
                    _ => panic!("{op}"),
                };
                if op["op"] == "scale" {
                    let factor = op["factor"].as_f64().unwrap();
                    *slot = json!(slot.as_f64().unwrap() * factor);
                } else {
                    *slot = op["value"].clone();
                }
            }
            "scale_rows" => {
                let field = op["field"].as_str().unwrap();
                let factor = op["factor"].as_f64().unwrap();
                let rows = doc["tables"][op["table"].as_str().unwrap()]["rows"].as_array_mut().unwrap();
                for row in rows {
                    if op["where"].as_object().unwrap().iter().all(|(k, v)| row[k] == *v) {
                        let value = &mut row[field]["value"];
                        *value = json!(value.as_f64().unwrap() * factor);
                    }
                }
            }
            "withhold" => {
                doc["tables"][op["table"].as_str().unwrap()] = json!({"status": "withheld", "reason": "corpus: payload withheld"});
            }
            "remove_rows" => {
                let rows = doc["tables"][op["table"].as_str().unwrap()]["rows"].as_array_mut().unwrap();
                let matches = |row: &Value| op["where"].as_object().unwrap().iter().all(|(k, v)| row[k] == *v);
                let before = rows.len();
                rows.retain(|row| !matches(row));
                assert!(rows.len() < before, "{op}: removes a row");
                if let Some(prefix) = op["renumber_prefix"].as_str() {
                    for (i, row) in rows.iter_mut().enumerate() {
                        row["ordinal"] = json!(i);
                        row["row_id"] = json!(format!("{prefix}{i}"));
                    }
                }
            }
            "set_rows" => {
                // Sets one field of every row the `where` members select.
                let rows = doc["tables"][op["table"].as_str().unwrap()]["rows"].as_array_mut().unwrap();
                let mut hits = 0;
                for row in rows.iter_mut().filter(|row| op["where"].as_object().unwrap().iter().all(|(k, v)| row[k] == *v)) {
                    row[op["field"].as_str().unwrap()] = op["value"].clone();
                    hits += 1;
                }
                assert!(hits > 0, "{op}: sets a row");
            }
            other => panic!("unknown corpus operation `{other}`"),
        }
    }
}

fn reseal_table(table: &mut Value) {
    if table["status"] != "published" {
        return;
    }
    let mut checksums = Vec::new();
    for row in table["rows"].as_array_mut().unwrap() {
        let mut body = row.as_object().unwrap().clone();
        body.remove("checksum");
        let checksum = modal_reader_sha256(&Value::Object(body)).unwrap();
        row["checksum"] = json!(checksum);
        checksums.push(json!(checksum));
    }
    table["row_count"] = json!(checksums.len());
    table["digest"] = json!(modal_reader_sha256(&Value::Array(checksums)).unwrap());
}

/// Recomputes every row checksum and table digest.
fn reseal(doc: &mut Value) {
    if let Some(tables) = doc.get_mut("tables").and_then(Value::as_object_mut) {
        for table in tables.values_mut() {
            reseal_table(table);
        }
    }
    if let Some(table) = doc.get_mut("refusals") {
        reseal_table(table);
    }
}

/// The verdict's statuses, as the corpus states them.
fn statuses(verdict: &Value) -> Value {
    let relations: serde_json::Map<String, Value> =
        verdict["relations"].as_object().unwrap().iter().map(|(k, v)| (k.clone(), v["status"].clone())).collect();
    json!({"closure": verdict["closure"]["status"], "binding": verdict["binding"]["status"], "relations": relations,
        "health": verdict["health"], "decision": verdict["decision"], "refusal_codes": verdict["refusal_codes"]})
}

/// The document text an entry reads: the committed text with its raw text
/// replacements (for what a parsed value cannot carry: duplicate members,
/// non-JSON numbers), or the mutated value serialized.
fn entry_text(entry: &Value) -> String {
    let raw = fixture(entry["document"].as_str().unwrap());
    if let Some(replacements) = entry.get("text_operations").and_then(Value::as_array) {
        assert!(entry["operations"].as_array().unwrap().is_empty() && entry["reseal"] == false);
        let mut text = raw.to_string();
        for r in replacements {
            let (find, with) = (r["find"].as_str().unwrap(), r["replace"].as_str().unwrap());
            assert!(text.contains(find), "{r}");
            text = text.replacen(find, with, 1);
        }
        return text;
    }
    let mut doc: Value = serde_json::from_str(raw).unwrap();
    apply(&mut doc, entry["operations"].as_array().unwrap());
    if entry["reseal"] == true {
        reseal(&mut doc);
    }
    serde_json::to_string(&doc).unwrap()
}

fn run(entry: &Value) -> (Value, Value) {
    let text = entry_text(entry);
    let request = entry["request"].as_str().map(|name| {
        let mut request: Value = serde_json::from_str(fixture(name)).unwrap();
        apply(&mut request, entry["request_operations"].as_array().unwrap());
        request
    });
    let verdict = read_modal_result_text(&text, ModalSupplied { request: request.as_ref() }).modal_verdict_json();
    (statuses(&verdict), verdict)
}

/// `run` without the request: the payload relations alone, the model-derived
/// standings scoped.
fn run_payload(entry: &Value) -> (Value, Value) {
    let mut entry = entry.clone();
    entry["request"] = Value::Null;
    run(&entry)
}

/// The targeted standing's difference names the entry's reason.
fn reason_holds(entry: &Value, verdict: &Value) -> bool {
    let Some(needle) = entry.get("difference_contains").and_then(Value::as_str) else { return true };
    let intent = entry["intent"].as_str().unwrap();
    let standing = match intent {
        "closure" | "binding" => &verdict[intent],
        relation => &verdict["relations"][relation],
    };
    standing["difference"].as_str().is_some_and(|d| d.to_lowercase().contains(&needle.to_lowercase()))
}

fn intent_holds(entry: &Value, got: &Value) -> bool {
    let none_failed = || got["relations"].as_object().unwrap().values().all(|s| s != "failed");
    match entry["intent"].as_str().unwrap() {
        "none" => ["consistent", "consistent_with_unresolved", "consistent_with_scoped_unsupported", "refusal_consistent"]
            .contains(&got["decision"].as_str().unwrap()),
        "scoped" => ["consistent_with_unresolved", "consistent_with_scoped_unsupported"].contains(&got["decision"].as_str().unwrap()) && none_failed(),
        // Left open by an operand's stated set: accepted, never passed.
        "unresolved" => got["decision"] == "consistent_with_unresolved" && none_failed(),
        "closure" | "binding" => got[entry["intent"].as_str().unwrap()] == "failed" && got["decision"] == "rejected",
        relation => got["relations"][relation] == "failed" && got["decision"] == "rejected",
    }
}

/// Each entry's targeted standing, stated reason and `expected` verdict; every
/// entry is read before a caller fails, so one run names them all.
fn entry_failures(entries: &[Value]) -> Vec<String> {
    let mut failures = Vec::new();
    for entry in entries {
        let (got, verdict) = run(entry);
        let difference = |name: &str| verdict["relations"].get(name).or(verdict.get(name)).map(|s| s["difference"].clone());
        let intent = entry["intent"].as_str().unwrap();
        if !intent_holds(entry, &got) {
            failures.push(format!("{}: the targeted standing does not hold: {got} ({:?})", entry["id"], difference(intent)));
        } else if !reason_holds(entry, &verdict) {
            failures.push(format!("{}: not failed for its stated reason: {:?}", entry["id"], difference(intent)));
        } else if got != entry["expected"] {
            failures.push(format!("{}: {got} is not the expected {}", entry["id"], entry["expected"]));
        }
    }
    failures
}

#[test]
fn the_rust_reader_returns_every_corpus_verdict() {
    let corpus: Value = serde_json::from_str(CORPUS).unwrap();
    let entries = corpus["entries"].as_array().unwrap();
    assert!(entries.len() >= 50);
    let failures = entry_failures(entries);
    assert!(failures.is_empty(), "{} corpus entries:\n{}", failures.len(), failures.join("\n"));
}

/// The point-sum boundaries: this reader's verdicts, which the Python reader is
/// held to (`tests/test_modal_evidence.py`), on both sides of each of the four
/// relations that sum (m, T_k / free-DOF mass, the cumulative fraction, Γ / m_eff).
#[test]
fn the_rust_reader_returns_every_sum_order_boundary_verdict() {
    let corpus: Value = serde_json::from_str(SUM_ORDER).unwrap();
    let entries = corpus["entries"].as_array().unwrap();
    let relations: std::collections::BTreeSet<&str> = entries.iter().map(|e| e["sum_relation"].as_str().unwrap()).collect();
    assert_eq!(relations, ["cumulative_fraction", "effective_mass_excitation", "generalized_mass", "total_mass"].into_iter().collect());
    let failures = entry_failures(entries);
    assert!(failures.is_empty(), "{} sum-order entries:\n{}", failures.len(), failures.join("\n"));
}

/// The reader against the feasibility oracle (Design §5.3.4), which is
/// supporting evidence only: the reader never fails a family the oracle finds
/// feasible (a false refusal), and never passes or scopes one the oracle
/// proves infeasible: it fails it, or leaves it unresolved where binary64
/// cannot decide what the oracle's rational arithmetic can. Closure passes and
/// no other standing fails.
#[test]
fn the_rust_reader_agrees_with_the_feasibility_oracle() {
    let cases: Value = serde_json::from_str(ORACLE_CASES).unwrap();
    let cases = cases["cases"].as_array().unwrap();
    assert!(cases.len() >= 500);
    let mut failures = Vec::new();
    for case in cases {
        // With the bound request, the model-derived mass is held as well: never weaker than
        // the oracle (an infeasible family still fails), though it may fail a tampering
        // the payload alone admits.
        let (bound, bound_verdict) = run(case);
        for (family, truth) in case["oracle"].as_object().unwrap() {
            let status = bound["relations"][family].as_str().unwrap();
            if truth == "infeasible" && !(status == "failed" || (status == "unresolved" && case["unresolved_allowed"] == true)) {
                failures.push(format!("{}: {family}: oracle infeasible, reader with the request {status} ({})", case["id"],
                    bound_verdict["relations"][family]["difference"]));
            }
        }
        // The oracle decides the payload's own feasibility: compared without the request.
        let (got, verdict) = run_payload(case);
        if got["closure"] != "passed" {
            failures.push(format!("{}: closure {}", case["id"], verdict["closure"]["difference"]));
            continue;
        }
        for (family, truth) in case["oracle"].as_object().unwrap() {
            let status = got["relations"][family].as_str().unwrap();
            let agrees = if truth == "infeasible" { status == "failed" || (status == "unresolved" && case["unresolved_allowed"] == true) } else { status != "failed" };
            if !agrees {
                failures.push(format!("{}: {family}: oracle {truth}, reader {status} ({})", case["id"], verdict["relations"][family]["difference"]));
            }
        }
        for other in ["normalization", "counts", "units"] {
            if got["relations"][other] == "failed" {
                failures.push(format!("{}: {other} failed ({})", case["id"], verdict["relations"][other]["difference"]));
            }
        }
    }
    assert!(failures.is_empty(), "{} disagreements:\n{}", failures.len(), failures.iter().take(60).cloned().collect::<Vec<_>>().join("\n"));
}

/// The producer's own edge output (false-refusal controls): accepted, no
/// relation failed, and every relation that touches a flagged value left
/// unresolved or passed, never failed.
#[test]
fn the_rust_reader_accepts_every_producer_edge() {
    let edges: Value = serde_json::from_str(PRODUCER_EDGES).unwrap();
    let entries = edges["entries"].as_array().unwrap();
    assert!(entries.len() >= 3);
    for entry in entries {
        // Their values are invented at the payload's arithmetic edges, not the bound model's
        // mass: read without the request (the payload domain).
        let (got, verdict) = run_payload(entry);
        assert!(got["decision"] != "rejected", "{}: {verdict:#}", entry["id"]);
        assert_eq!(verdict["validation"]["status"], "not_fully_validated", "{}", entry["id"]);
    }
}

#[test]
fn every_tampered_relation_is_targeted_and_both_readers_share_one_corpus() {
    let corpus: Value = serde_json::from_str(CORPUS).unwrap();
    let intents: std::collections::BTreeSet<&str> =
        corpus["entries"].as_array().unwrap().iter().map(|e| e["intent"].as_str().unwrap()).collect();
    for needed in ["none", "scoped", "unresolved", "closure", "binding", "mass", "normalization", "participation", "counts", "clusters", "sturm", "evidence", "standing", "units"] {
        assert!(intents.contains(needed), "the corpus targets `{needed}`");
    }
}

#[test]
fn the_readers_semantic_table_is_the_committed_table() {
    let committed: Value = serde_json::from_str(SEMANTIC_TABLE).unwrap();
    assert_eq!(modal_reader_semantic_table(), committed);
    assert_eq!(modal_reader_sha256(&committed).as_deref(), Some(MODAL_RESULT_SEMANTICS_SHA256));
}

/// Every committed fixture, with its request: closure and binding pass, no
/// relation fails and a solved document's units pass. A linear one is then
/// consistent and fully validated, its subject census re-derived from the
/// bound request; a linearized state case stays consistent only up to the
/// qualification of its reported support states; a refused one is
/// refusal-consistent.
#[test]
fn every_committed_fixture_reads_validated_up_to_its_scope() {
    let names = ["ex_n05", "arc", "component_body", "state_one_way_pressed", "state_friction_sliding", "cantilever_whole_spectrum",
        "cantilever_cluster_extension", "cantilever_below_cutoff", "cantilever_subnormal_cutoff", "cantilever_sensitive", "refused_unseparated", "refused_no_bound"];
    for name in names {
        let doc: Value = serde_json::from_str(fixture(&format!("modal_{name}.json"))).unwrap();
        let request: Value = serde_json::from_str(fixture(&format!("modal_{name}.request.json"))).unwrap();
        let verdict = read_modal_result(&doc, ModalSupplied { request: Some(&request) });
        let json = verdict.modal_verdict_json();
        assert_eq!((verdict.closure.status, verdict.binding.status), (ModalReadStatus::Passed, ModalReadStatus::Passed), "{name}: {json:#}");
        assert!(verdict.relations.iter().all(|r| r.status != ModalReadStatus::Failed), "{name}: {json:#}");
        if doc["status"] == "refused" {
            assert_eq!(verdict.decision, "refusal_consistent", "{name}");
            assert!(!verdict.refusal_codes.is_empty());
        } else {
            // Every value has its catalog unit (Design §5.3.5).
            assert_eq!(verdict.relations[8].status, ModalReadStatus::Passed, "{name}: {json:#}");
            assert!(verdict.relations[8].scope.is_empty(), "{name}");
            if name.starts_with("state_") {
                // A linearized state case's partition comes from the reported support states, whose
                // qualification no payload establishes: the model-derived mass, participation and the
                // evidence formed on that partition stay scoped (Design, selected mass assurance).
                assert_eq!(verdict.decision, "consistent_with_scoped_unsupported", "{name}: {json:#}");
                assert_eq!(json["validation"]["unmet"], json!(["mass", "participation", "evidence"]), "{name}: {json:#}");
                assert_eq!(verdict.modal_validation(), "not_fully_validated", "{name}");
            } else {
                // The subject census re-derived from the bound request closes the last scope.
                assert_eq!(verdict.decision, "consistent", "{name}: {json:#}");
                assert_eq!(json["validation"]["unmet"], json!([]), "{name}: {json:#}");
                assert_eq!(verdict.modal_validation(), "fully_validated", "{name}");
            }
            let expected = if name == "cantilever_sensitive" { "sensitive" } else { "checks_passed" };
            assert_eq!(verdict.health, expected, "{name}");
        }
    }
}

/// Writes each entry's `expected` from this reader (run explicitly to
/// regenerate; every expectation is then reviewed against its intent and the
/// Python reader is held to it).
#[test]
#[ignore = "writes the corpus expectations; run explicitly after a reviewed corpus change"]
fn write_modal_corpus_expectations() {
    let mut corpus: Value = serde_json::from_str(CORPUS).unwrap();
    for entry in corpus["entries"].as_array_mut().unwrap() {
        let (got, _) = run(entry);
        entry["expected"] = got;
    }
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/results/modal_corpus.json");
    let mut text = serde_json::to_string_pretty(&corpus).unwrap();
    text.push('\n');
    std::fs::write(path, text).unwrap();
}

/// Writes each sum-order boundary entry's `expected` from this reader (run
/// explicitly to regenerate; the Python reader is then held to it).
#[test]
#[ignore = "writes the sum-order boundary expectations; run explicitly after a reviewed change"]
fn write_modal_sum_order_expectations() {
    let mut corpus: Value = serde_json::from_str(SUM_ORDER).unwrap();
    for entry in corpus["entries"].as_array_mut().unwrap() {
        let (got, _) = run(entry);
        entry["expected"] = got;
    }
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/results/modal_sum_order_boundaries.json");
    let mut text = serde_json::to_string_pretty(&corpus).unwrap();
    text.push('\n');
    std::fs::write(path, text).unwrap();
}

/// Review of 56a5484e8: a flagged formation-separation ratio violates its plain-binary64
/// producer domain in the sturm relation, as it does in the Python reader (verdict parity:
/// closure passes, sturm fails).
#[test]
fn a_flagged_formation_separation_ratio_fails_sturm_not_closure() {
    let base = "/tables/sturm_candidates/rows/0/formation_separation_ratio";
    let entry = json!({
        "document": "modal_component_body.json",
        "operations": [{"op": "replace", "path": format!("{base}/value"), "value": 0.0},
                       {"op": "replace", "path": format!("{base}/below_normal"), "value": true}],
        "reseal": true,
        "request": "modal_component_body.request.json",
        "request_operations": [],
    });
    let (got, _) = run(&entry);
    assert_eq!(got["closure"], "passed", "{got}");
    assert_eq!(got["relations"]["sturm"], "failed", "{got}");
    assert_eq!(got["decision"], "rejected", "{got}");
}
