//! RT-A3 §4 (DEL-05-02 Work `combination-certificate.md` §3): the reader of
//! `contract_evidence.combination_certificate` over the shared corpus
//! (`fixtures/results/combination_certificate_corpus.json`, written by PP
//! `tests/combination_certificate.rs`). The producer documents are admitted with and
//! without their models; every forgery of `fixtures/results/combination_certificate_forgeries.json`
//! (the list the Python mirror applies, `tests/test_combination_certificate.py`) is
//! refused with the same code, without and with its model (`model_code` where the model
//! contradicts the forged record first). The absence rule's matrix (the certificate
//! review's threat cases on both actual envelopes) is part of that list; the
//! inspection-only meaning of absence and the legacy W-CB3 successors are tested below.
use open_pipe_stress_result_export::{reader_verdict, retained_precision as rp};
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
        let document = forged(&e["document"], forgery);
        let model = forged_model(&e["model"], forgery);
        for model in [None, Some(&model)] {
            let verdict = reader_verdict::validate(e["reader"].as_str().unwrap(), &document, model, None);
            let code = match model {
                Some(_) if forgery.get("model_code").is_some() => &forgery["model_code"],
                _ => &forgery["code"],
            };
            let label = if model.is_some() { "with model" } else { "without model" };
            match code.as_str() {
                None => assert_eq!(verdict["ok"], true, "{} ({label}): {verdict}", forgery["what"]),
                Some(code) => {
                    let code = format!("SOURCE_COMBINATION_CERTIFICATE_{code}");
                    assert!(verdict["ok"] == false && verdict.to_string().contains(&code), "{} ({label}): {verdict}", forgery["what"]);
                }
            }
        }
    }
}

fn forged(document: &Value, forgery: &Value) -> Value {
    let mut document = document.clone();
    for edit in forgery["edits"].as_array().unwrap() {
        apply(&mut document, edit);
    }
    document
}

/// The model the forgery's with-model reading uses: its `model_edits` applied, if any (review
/// cert3 finding 2's model-only id collision).
fn forged_model(model: &Value, forgery: &Value) -> Value {
    let mut model = model.clone();
    for edit in forgery.get("model_edits").and_then(Value::as_array).into_iter().flatten() {
        apply(&mut model, edit);
    }
    model
}

/// Any §4 certificate claim a document still makes: the member, or a diagnostic in the
/// certificate's vocabulary, restated independently of the reader (the Python test's
/// `_claims_survive` alike). W-16b's derived-rows notice is conditional on the member.
fn claims_survive(document: &Value) -> Vec<String> {
    let mut found = Vec::new();
    if document["contract_evidence"].get("combination_certificate").is_some() {
        found.push("member".to_string());
    }
    for d in document["diagnostics"].as_array().unwrap() {
        let (id, code, message) = (d["id"].as_str().unwrap(), d["code"].as_str().unwrap(), d["message"].as_str().unwrap_or_default());
        if id.starts_with("diagnostic:combination-certificate:")
            || code.starts_with("COMBINATION_ROWS_")
            || ["established within", "within their stated bound", "within its stated bound", "against a stated bound"].iter().any(|t| message.contains(t))
        {
            found.push(id.to_string());
        }
    }
    found
}

/// Absence of the member (member and notices both dropped, the current identity kept)
/// is admitted as no §4 certificate claim: no member, no certificate notice under any
/// id, and no eligibility effect (decision (b)): numerical_quality and the standing are
/// the certified document's.
#[test]
fn absence_is_inspection_only_with_no_certificate_claim() {
    let forgeries: Value = serde_json::from_str(FORGERIES).unwrap();
    let documents = documents();
    for name in ["ex_cantilever", "pf_cantilever"] {
        let e = documents.iter().find(|e| e["name"] == name).unwrap();
        let forgery = forgeries["forgeries"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["document"] == name && f["what"] == "member and its notices absent (no certificate claim)")
            .unwrap();
        let stripped = forged(&e["document"], forgery);
        for model in [None, Some(&e["model"])] {
            let verdict = reader_verdict::validate(e["reader"].as_str().unwrap(), &stripped, model, None);
            assert_eq!(verdict, serde_json::json!({"ok": true}), "{name}");
            assert_eq!(reader_verdict::standing(&stripped, model, None), reader_verdict::standing(&e["document"], model, None), "{name}");
        }
        assert!(claims_survive(&stripped).is_empty(), "{name}: {:?}", claims_survive(&stripped));
        assert!(!claims_survive(&e["document"]).is_empty(), "{name}");
        assert_eq!(stripped["numerical_quality"], e["document"]["numerical_quality"], "{name}");
        assert_eq!(stripped["formulation_basis"], e["document"]["formulation_basis"], "{name}");
    }
}

/// The registered W-CB3 retained combination successors, recorded before the certificate
/// and byte-untouched (the Python test pins their SHA-256), are accepted with and
/// without their invocation; their standing is their own retained-precision receipt's,
/// and they make no §4 certificate claim.
#[test]
fn legacy_w_cb3_successors_are_accepted() {
    for text in [
        include_str!("../../../../fixtures/results/retained_precision_combination_successor_sparse_interactive.json"),
        include_str!("../../../../fixtures/results/retained_precision_combination_successor_dense_scrutiny.json"),
    ] {
        let w: Value = serde_json::from_str(text).unwrap();
        assert!(claims_survive(&w["source"]).is_empty());
        let bound = rp::validate(&w["source"], Some(&w["invocation"])).unwrap();
        assert!(bound.invocation_bound && bound.numerical_eligible);
        let unbound = rp::validate(&w["source"], None).unwrap();
        assert!(!unbound.invocation_bound && !unbound.numerical_eligible);
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
        ("add", Value::Array(items)) if last == "-" => items.push(edit["value"].clone()),
        ("add", Value::Array(items)) => items.insert(last.parse::<usize>().unwrap(), edit["value"].clone()),
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

/// The retained base-read context (Work `combination-certificate.md` §2, Agent 0's ruling after review cert2) is
/// entered only by the retained reader's G7 base read: `retained_base_read` is crate-private and its one caller is
/// `retained_precision.rs` (the unit tests in `combination_certificate.rs` scan for it and read under it). From an
/// ordinary entry the context is never set: prose under a combination ref on a W-CB3 successor's ordinary base is
/// refused by the roster, the unmodified successor reads through its own reader, and the context is clear after each.
#[test]
fn the_retained_base_read_context_is_not_reachable_from_ordinary_reads() {
    let w: Value = serde_json::from_str(include_str!("../../../../fixtures/results/retained_precision_combination_successor_sparse_interactive.json")).unwrap();
    let base = |s: &Value| {
        let mut b = s.clone();
        b.as_object_mut().unwrap().remove("retained_precision");
        b["producer"]["semantic_contract_id"] = serde_json::json!("openpipestress.result_semantics/0.3.0/preview-physics-2");
        b["formulation_basis"]["profile_id"] = serde_json::json!("product_preview_mechanics_v1");
        let rows = b["results"].as_array_mut().unwrap();
        rows.retain(|r| r["kind"] != rp::REQUESTED_MODE_KIND);
        rows.iter_mut().for_each(|r| {
            r.as_object_mut().unwrap().remove("recovery_method");
        });
        b
    };
    let mut noted = base(&w["source"]);
    noted["diagnostics"].as_array_mut().unwrap().push(serde_json::json!({"id": "diagnostic:review:claim", "code": "REVIEW_NOTE", "severity": "info",
        "source": "core/product_physics", "affected_refs": ["combination:ab"], "message": "combination combination:ab: all derived row groups are established."}));
    assert!(!open_pipe_stress_result_export::in_retained_base_read());
    assert!(open_pipe_stress_result_export::semantic_contract::for_source(&base(&w["source"])).is_ok());
    assert_eq!(open_pipe_stress_result_export::semantic_contract::for_source(&noted).err().as_deref(), Some("SOURCE_COMBINATION_CERTIFICATE_ROSTER_EXTRA"));
    assert_eq!(reader_verdict::validate("preview-physics", &noted, None, None)["error"]["code"], "SOURCE_COMBINATION_CERTIFICATE_ROSTER_EXTRA");
    assert!(rp::validate(&w["source"], Some(&w["invocation"])).is_ok());
    assert!(!open_pipe_stress_result_export::in_retained_base_read());
}
