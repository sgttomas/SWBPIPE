//! RT-A3 §4 (DEL-05-02 Work `combination-certificate.md` §3): the reader of
//! `contract_evidence.combination_certificate` over the shared corpus
//! (`fixtures/results/combination_certificate_corpus.json`, written by PP
//! `tests/combination_certificate.rs`). The producer documents are admitted with and
//! without their models; every forgery of `fixtures/results/combination_certificate_forgeries.json`
//! (the list the Python mirror applies, `tests/test_combination_certificate.py`) is
//! refused with the same code.
use open_pipe_stress_result_export::reader_verdict;
use serde_json::Value;

const CORPUS: &str = include_str!("../../../../fixtures/results/combination_certificate_corpus.json");
const FORGERIES: &str = include_str!("../../../../fixtures/results/combination_certificate_forgeries.json");

fn documents() -> Vec<Value> {
    let corpus: Value = serde_json::from_str(CORPUS).unwrap();
    corpus["documents"].as_array().unwrap().clone()
}

#[test]
fn producer_documents_are_admitted_with_and_without_their_models() {
    for e in documents() {
        let reader = e["reader"].as_str().unwrap();
        for model in [None, Some(&e["model"])] {
            let verdict = reader_verdict::validate(reader, &e["document"], model, None);
            assert_eq!(verdict["ok"], true, "{}: {verdict}", e["name"]);
        }
    }
}

#[test]
fn every_forgery_is_refused_with_the_shared_code() {
    let forgeries: Value = serde_json::from_str(FORGERIES).unwrap();
    let list = forgeries["forgeries"].as_array().unwrap();
    assert!(list.len() >= 20);
    let documents = documents();
    for forgery in list {
        let e = documents.iter().find(|e| e["name"] == forgery["document"]).unwrap();
        let mut document = e["document"].clone();
        for edit in forgery["edits"].as_array().unwrap() {
            apply(&mut document, edit);
        }
        let verdict = reader_verdict::validate(e["reader"].as_str().unwrap(), &document, None, None);
        match forgery["code"].as_str() {
            None => assert_eq!(verdict["ok"], true, "{}: {verdict}", forgery["what"]),
            Some(code) => {
                let code = format!("SOURCE_COMBINATION_CERTIFICATE_{code}");
                assert!(verdict["ok"] == false && verdict.to_string().contains(&code), "{}: {verdict}", forgery["what"]);
            }
        }
    }
}

fn apply(document: &mut Value, edit: &Value) {
    let pointer = edit["pointer"].as_str().unwrap();
    let (parent, last) = pointer.rsplit_once('/').unwrap();
    let last = last.replace("~1", "/").replace("~0", "~");
    let target = document.pointer_mut(parent).unwrap_or_else(|| panic!("{pointer}"));
    match (edit["op"].as_str().unwrap(), target) {
        ("remove", Value::Array(items)) => {
            items.remove(last.parse::<usize>().unwrap());
        }
        ("remove", Value::Object(map)) => {
            map.remove(&last);
        }
        (_, Value::Array(items)) => items[last.parse::<usize>().unwrap()] = edit["value"].clone(),
        (_, Value::Object(map)) => {
            map.insert(last, edit["value"].clone());
        }
        _ => panic!("{pointer}"),
    }
}

#[test]
fn model_binding_refuses_a_held_subtrahend_missing_from_the_record() {
    let e = documents().into_iter().find(|e| e["name"] == "pf_held").unwrap();
    assert_eq!(reader_verdict::validate("preview-physics", &e["document"], Some(&e["model"]), None)["ok"], true);
    let e = documents().into_iter().find(|e| e["name"] == "pf_cantilever").unwrap();
    let mut model = e["model"].clone();
    for c in model["combinations"].as_array_mut().unwrap() {
        if c["id"] == "comb:double" {
            c["subtrahend_support_state"] = serde_json::json!("minuend_final");
        }
    }
    let verdict = reader_verdict::validate("preview-physics", &e["document"], Some(&model), None);
    assert_eq!(verdict["ok"], false, "{verdict}");
}

#[test]
fn model_binding_refuses_a_forged_expression() {
    let e = documents().into_iter().find(|e| e["name"] == "pf_cantilever").unwrap();
    let mut model = e["model"].clone();
    for c in model["combinations"].as_array_mut().unwrap() {
        if c["id"] == "comb:double" {
            let minuend = c["minuend_id"].clone();
            c["minuend_id"] = c["subtrahend_id"].clone();
            c["subtrahend_id"] = minuend;
        }
    }
    let verdict = reader_verdict::validate("preview-physics", &e["document"], Some(&model), None);
    assert_eq!(verdict["ok"], false, "{verdict}");
}

/// Transport is metadata-only (no rows, diagnostics or status): it admits the member on
/// its closed shape and leaves the reading against the rows to the source check. The
/// producer's documents are admitted, also without their results; a malformed member is
/// refused by name (the Python mirror alike).
#[test]
fn transport_reads_the_member_shape_only() {
    let e = documents().into_iter().find(|e| e["name"] == "pf_cantilever").unwrap();
    assert_eq!(reader_verdict::transport("preview-physics", &e["document"])["ok"], true);
    let mut metadata = e["document"].clone();
    for key in ["results", "diagnostics", "status"] {
        metadata.as_object_mut().unwrap().remove(key);
    }
    assert_eq!(reader_verdict::transport("preview-physics", &metadata)["ok"], true, "{}", reader_verdict::transport("preview-physics", &metadata));
    for forged in [serde_json::json!([]), serde_json::json!({"combinations": []}), serde_json::json!({"combinations": [], "operand_cases": {}})] {
        let mut document = e["document"].clone();
        document["contract_evidence"]["combination_certificate"] = forged;
        let verdict = reader_verdict::transport("preview-physics", &document);
        assert!(verdict["ok"] == false && verdict.to_string().contains("transport combination certificate shape"), "{verdict}");
    }
}
