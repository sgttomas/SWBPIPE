//! The diagnostics presentation facts (`diagnostics_presentation`): each fact is its whole-value
//! definition on the same bytes, the cache is the held snapshot's own, and S carries the facts of
//! the share it read. The webview's equality with its whole-value helpers is checked over the same
//! replies by `suppliedDiagnostics.native.test.tsx` and `resultSummaryParity.test.ts`.
use super::*;
use crate::command_ingress::test_support::headers;
use crate::command_ingress::INGRESS_ARGS_JSON_V1;
use crate::native_replay_tests::{held, registry, restored};
use crate::solve_jobs::{HeldResult, SolveJobRegistry};
use serde_json::{json, Value};
use tauri::ipc::InvokeBody;

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn read(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The whole-value definitions, read independently: the population, and the route reader's
/// acceptance of the source alone (`reader_validate_json`, the wasm export's function, on the text).
/// `source` is `text` parsed by the caller; the dispatch it read is returned for the binding check.
fn expected(text: &str, source: &Value) -> (Vec<String>, bool, Value) {
    let mut ids: Vec<String> = open_pipe_stress_result_export::document_combination_identities(source).into_iter().collect();
    ids.sort();
    let dispatch: Value = serde_json::from_str(&open_pipe_stress_operation_applier::reader_dispatch_json(text, None, None).unwrap()).unwrap();
    let reader = match dispatch["route"].as_str().unwrap() {
        "preview_physics" => Some("preview-physics"),
        "pressure" => Some("pressure"),
        "pressure_source" => Some("pressure-source"),
        _ => None,
    };
    let roster = !ids.is_empty()
        && source.get("retained_precision").is_none()
        && reader.is_some_and(|reader| {
            let outcome: Value = serde_json::from_str(&open_pipe_stress_operation_applier::reader_validate_json(reader, text, None, None).unwrap()).unwrap();
            outcome["ok"] == true
        });
    (ids, roster, dispatch)
}

/// The certificate corpus's documents, their review claims (a supplied note naming one of the
/// document's combinations, which the closed roster refuses) and the forgeries, the retained
/// successors with and without the certificate member, and a result without combinations.
/// Built once per process from the committed fixtures (immutable input texts); each call returns
/// its own copy.
fn sources() -> Vec<(String, String)> {
    static SOURCES: std::sync::OnceLock<Vec<(String, String)>> = std::sync::OnceLock::new();
    SOURCES.get_or_init(build_sources).clone()
}
fn build_sources() -> Vec<(String, String)> {
    let corpus: Value = serde_json::from_str(&read("fixtures/results/combination_certificate_corpus.json")).unwrap();
    let forgeries: Value = serde_json::from_str(&read("fixtures/results/combination_certificate_forgeries.json")).unwrap();
    let mut out = Vec::new();
    for entry in corpus["documents"].as_array().unwrap() {
        let name = entry["name"].as_str().unwrap();
        let document = &entry["document"];
        out.push((name.to_string(), serde_json::to_string(document).unwrap()));
        let ids = open_pipe_stress_result_export::document_combination_identities(document);
        let mut ids: Vec<&String> = ids.iter().collect();
        ids.sort();
        if let Some(id) = ids.first() {
            let mut claimed = document.clone();
            // A claim under the document's own diagnostic source.
            let source = document["diagnostics"][0]["source"].clone();
            claimed["diagnostics"].as_array_mut().unwrap().push(json!({"id": "diagnostic:review:claim", "code": "REVIEW_NOTE", "severity": "info",
                "source": source, "affected_refs": [id], "message": format!("combination {id}: all derived rows are bounded by 1e-30.")}));
            out.push((format!("{name}+claim"), serde_json::to_string(&claimed).unwrap()));
        }
    }
    for (index, forgery) in forgeries["forgeries"].as_array().unwrap().iter().enumerate() {
        let name = forgery["document"].as_str().unwrap();
        let mut document = corpus["documents"].as_array().unwrap().iter().find(|e| e["name"] == name).unwrap()["document"].clone();
        for edit in forgery["edits"].as_array().unwrap() {
            apply(&mut document, edit);
        }
        out.push((format!("forgery {index} {name}"), serde_json::to_string(&document).unwrap()));
    }
    for path in ["fixtures/results/retained_precision_combination_successor_sparse_interactive.json", "fixtures/results/retained_precision_combination_successor_sparse_interactive_pre_certificate.json"] {
        let document: Value = serde_json::from_str(&read(path)).unwrap();
        out.push((path.into(), serde_json::to_string(&document["source"]).unwrap()));
    }
    out.push(("no combinations".into(), read("fixtures/results/preview_physics_connected_sparse.json")));
    out
}

/// One JSON-pointer edit as `tests/test_combination_certificate.py` `_apply_one` applies it.
fn apply(document: &mut Value, edit: &Value) {
    let pointer = edit["pointer"].as_str().unwrap();
    let (parent, last) = pointer.rsplit_once('/').unwrap();
    let last = last.replace("~1", "/").replace("~0", "~");
    let target = document.pointer_mut(parent).unwrap_or_else(|| panic!("{pointer}"));
    let value = edit["value"].clone();
    match (edit["op"].as_str().unwrap(), target) {
        ("add", Value::Array(items)) if last == "-" => items.push(value),
        ("add", Value::Array(items)) => items.insert(last.parse().unwrap(), value),
        ("remove", Value::Array(items)) => { items.remove(last.parse().unwrap()); }
        ("remove", Value::Object(members)) => { members.remove(&last); }
        (_, Value::Array(items)) => items[last.parse::<usize>().unwrap()] = value,
        (_, Value::Object(members)) => { members.insert(last, value); }
        (op, other) => panic!("{op} {pointer} on {other}"),
    }
}

#[test]
fn each_fact_is_its_whole_value_definition_on_the_same_bytes() {
    let (mut admitted, mut refused, mut retained, mut empty) = (0, 0, 0, 0);
    for (name, text) in sources() {
        let source: Value = serde_json::from_str(&text).unwrap();
        let facts = facts(&text, &source, None, None);
        let (ids, roster, dispatch) = expected(&text, &source);
        assert_eq!(facts["combination_identities"], json!(ids), "{name}");
        assert_eq!(facts["roster_checked"], json!(roster), "{name}");
        assert_eq!(facts["certificate_attested"], json!(false), "{name}: no native snapshot holds a retained registration");
        assert_eq!(facts["binding"]["result_bytes"], json!(text.len()), "{name}");
        assert_eq!(facts["binding"]["source_sha256"], json!(hex(&Sha256::digest(text.as_bytes()))), "{name}");
        assert_eq!(facts["binding"]["route"], dispatch["route"], "{name}: the route the dispatch states");
        assert_eq!(facts["binding"]["reader_table"], dispatch["binding"], "{name}: the table the dispatch states");
        assert_eq!(facts["binding"]["recorded_table"], Value::Null, "{name}");
        if source.get("retained_precision").is_some() {
            retained += 1;
            assert!(!ids.is_empty() && facts["roster_checked"] == false, "{name}: a retained successor is never roster-checked");
        } else if ids.is_empty() {
            empty += 1;
        } else if roster {
            admitted += 1;
        } else {
            refused += 1;
        }
        if name.ends_with("+claim") {
            assert_eq!(facts["roster_checked"], json!(false), "{name}: a supplied claim naming a combination is a roster extra");
        }
    }
    assert!(admitted >= 5 && refused >= 10 && retained == 2 && empty == 1, "admitted {admitted}, refused {refused}, retained {retained}, empty {empty}");
}

#[test]
fn a_shared_id_document_reads_its_population_without_ambiguous_identities() {
    let corpus: Value = serde_json::from_str(&read("fixtures/results/combination_certificate_corpus.json")).unwrap();
    let document = &corpus["documents"].as_array().unwrap().iter().find(|e| e["name"] == "pf_shared_id").unwrap()["document"];
    let text = serde_json::to_string(document).unwrap();
    let facts = facts(&text, document, None, None);
    let (ids, roster, _) = expected(&text, &serde_json::from_str(&text).unwrap());
    assert_eq!(facts["combination_identities"], json!(ids));
    assert_eq!(facts["roster_checked"], json!(roster));
    assert!(!ids.is_empty());
}

/// A snapshot's cache is its own: a fact cached on one share is read there and never by another
/// share of the same bytes (another job, a restore, a new solve).
#[test]
fn the_cache_is_the_held_snapshots_own() {
    let claimed = sources().into_iter().find(|(name, _)| name == "ex_cantilever+claim").unwrap().1;
    let source: Value = serde_json::from_str(&claimed).unwrap();
    let (registry, _gate) = registry();
    let share = |text: &str| HeldResult::restored(text.to_string(), registry.acquire_result_parse(text.len() as u64).unwrap());
    let (a, b) = (share(&claimed), share(&claimed));
    assert_eq!(facts(&claimed, &source, None, Some(a.presentation()))["roster_checked"], json!(false));
    // A forged cache entry on another share (as a mutation reusing facts across snapshots would read it).
    let c = share(&claimed);
    c.presentation().roster_checked.set(true).unwrap();
    c.presentation().source_sha256.set([7; 32]).unwrap();
    assert_eq!(facts(&claimed, &source, None, Some(c.presentation()))["roster_checked"], json!(true), "the share's own cache is read");
    assert_eq!(facts(&claimed, &source, None, Some(b.presentation()))["roster_checked"], json!(false), "another share reads afresh");
    assert_eq!(facts(&claimed, &source, None, Some(b.presentation()))["binding"]["source_sha256"], json!(hex(&Sha256::digest(claimed.as_bytes()))));
}

fn summary(registry: &SolveJobRegistry, job_id: &str) -> Result<Value, String> {
    let args = json!({"jobId": job_id, "modelJson": "null", "standing": null});
    let body = InvokeBody::Raw(serde_json::to_vec(&args).unwrap());
    let headers = headers(Some(INGRESS_ARGS_JSON_V1));
    let r = registry.clone();
    match crate::result_summary::reserve(registry, &body, &headers).and_then(|(raw, lease)| crate::result_summary::run(r, raw, lease)) {
        Ok(bytes) => Ok(crate::result_planes::decode(&bytes).unwrap().header),
        Err(error) => Err(serde_json::to_string(&error).unwrap()),
    }
}

/// S carries the facts of the share it read: a solve's, a restored run's under its recorded table;
/// after disposal there is no S and no fact; a new job of the same bytes reads its own share.
#[test]
fn s_carries_the_facts_of_the_share_it_read() {
    let canonical = sources().into_iter().find(|(name, _)| name == "pf_held").unwrap().1;
    let (registry, gate) = registry();
    let job = held(&registry, &gate, &canonical);
    let source: Value = serde_json::from_str(&canonical).unwrap();
    let header = summary(&registry, &job).unwrap();
    assert_eq!(header[MEMBER], facts(&canonical, &source, None, None));
    // Poison this share's cache, then dispose of it: a new job of the same bytes reads its own.
    let share = registry.held_result(&job).unwrap();
    let truth = header[MEMBER]["roster_checked"].as_bool().unwrap();
    drop(share);
    assert!(registry.dispose_result(&job).unwrap().released);
    assert!(summary(&registry, &job).is_err(), "a disposed handle has no S");
    let (registry2, gate2) = crate::native_replay_tests::registry();
    let again = held(&registry2, &gate2, &canonical);
    let share = registry2.held_result(&again).unwrap();
    share.presentation().roster_checked.set(!truth).unwrap();
    drop(share);
    assert_eq!(summary(&registry2, &again).unwrap()[MEMBER]["roster_checked"], json!(!truth), "S reads its own share's cache");
    let (registry3, gate3) = crate::native_replay_tests::registry();
    let third = held(&registry3, &gate3, &canonical);
    assert_eq!(summary(&registry3, &third).unwrap()[MEMBER]["roster_checked"], json!(truth), "another share of the same bytes reads afresh");

    // A restored run read under its recorded table (the registry's tenth variant, main's exact
    // successor at 252bef969) states that table; its facts stay source-only, read under the
    // current tables as the webview's reader reads a whole historical result.
    let fixture: Value = serde_json::from_str(&read("fixtures/persistence/recorded_pre_rt_a4_pf/physics_retained_1/main_252bef969/retained_precision_exact_successor_sparse_interactive.json")).unwrap();
    let text = serde_json::to_string(&fixture["source"]).unwrap();
    let table = json!({"id": "openpipestress.result_semantics/0.3.0/physics-retained-1", "sha256": "85c2678a78e794b44761c11efb73c215645c7e2560d802ac8ad6075e7804c691"});
    let handle = restored(&registry3, &text, &table);
    let header = summary(&registry3, &handle).unwrap();
    let source: Value = serde_json::from_str(&text).unwrap();
    let recorded = crate::analysis_run_v03_validate::RecordedTable::of(open_pipe_stress_result_export::recorded_tables::variant_of(&table).unwrap());
    assert_eq!(header[MEMBER], facts(&text, &source, Some(recorded), None));
    assert_eq!(header[MEMBER]["binding"]["recorded_table"], table);
    let (ids, roster, _) = expected(&text, &source);
    assert_eq!(header[MEMBER]["combination_identities"], json!(ids));
    assert_eq!(header[MEMBER]["roster_checked"], json!(roster));
    assert_eq!(header[MEMBER]["roster_checked"], json!(false), "a retained successor");
    assert_eq!(header[MEMBER]["certificate_attested"], json!(false), "a restored run never registers");
}
