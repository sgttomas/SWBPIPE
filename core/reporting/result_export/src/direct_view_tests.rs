//! The pressure reader's one direct-case view (DEL-05-02): `exact_combination_evidence::direct_view`,
//! built in one pass from the received document, is the value the reader formerly made with two
//! whole copies (the document without the combination certificate, then that without the
//! combination record and rows), errors included; and the reader's verdict through the one view
//! is its verdict through the two, refusal for refusal. The former functions are kept below as the
//! reference. The documents are every document of the certificate corpus, the pressure-2
//! combination corpus and the RT-A3 withheld constant-effort envelope, every certificate forgery,
//! and structural mutations of each pressure document that reach each branch of the view.
use super::validate_direct_pressure_evidence;
use crate::exact_combination_evidence as ec;
use serde_json::{json, Value};

type Check = Result<(), String>;

// ------------------------------------------------------------------ the reference

/// The former `combination_certificate::without_member`.
fn reference_without_member(source: &Value) -> Option<Value> {
    source["contract_evidence"].get("combination_certificate")?;
    let mut view = source.clone();
    if let Some(evidence) = view["contract_evidence"].as_object_mut() {
        evidence.remove("combination_certificate");
    }
    Some(view)
}

/// The former `exact_combination_evidence::direct_view`.
fn reference_direct_view(source: &Value) -> Result<Value, String> {
    let records = source["contract_evidence"]
        .get("exact_combinations")
        .ok_or_else(|| "SOURCE_PHYSICS_COMBINATION_EVIDENCE_MISSING".to_string())?;
    if !records.as_array().is_some_and(|r| !r.is_empty()) {
        return Err("SOURCE_PHYSICS_COMBINATION_EVIDENCE_SHAPE".into());
    }
    let mut view = source.clone();
    if let Some(evidence) = view["contract_evidence"].as_object_mut() {
        evidence.remove("exact_combinations");
    }
    if let Some(rows) = view["results"].as_array_mut() {
        rows.retain(|row| row["basis_ref"]["ref_type"] != "combination");
    }
    Ok(view)
}

/// The former composition: `direct_view(without_member(source))`.
fn reference_view(source: &Value) -> Result<Value, String> {
    match reference_without_member(source) {
        Some(view) => reference_direct_view(&view),
        None => reference_direct_view(source),
    }
}

/// The former `validate_pressure_evidence_without_certificate` and
/// `validate_pressure_evidence_uncertified`.
fn reference_without_certificate(source: &Value) -> Check {
    match reference_without_member(source) {
        Some(view) => reference_uncertified(&view),
        None => reference_uncertified(source),
    }
}
fn reference_uncertified(source: &Value) -> Check {
    if ec::present(source) {
        let view = reference_direct_view(source)?;
        validate_direct_pressure_evidence(&view)?;
        return ec::validate(source);
    }
    validate_direct_pressure_evidence(source)
}

// ------------------------------------------------------------------ the documents

const CERTIFICATE_CORPUS: &str = include_str!("../../../../fixtures/results/combination_certificate_corpus.json");
const FORGERIES: &str = include_str!("../../../../fixtures/results/combination_certificate_forgeries.json");
const COMBINATION_CORPUS: &str = include_str!("../../../../fixtures/results/pressure_v3_combination_reader_corpus.json");
const WITHHELD: &str = include_str!("../../../../fixtures/results/rt_a3_ce2_withheld_sparse.json");

/// A forgery's JSON-pointer edit, as `tests/combination_certificate_contract.rs` applies it.
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

/// Structural mutations of a document, each reaching a branch of the view or of the
/// reader's dispatch.
fn mutations(document: &Value) -> Vec<(&'static str, Value)> {
    let edit = |f: &dyn Fn(&mut Value)| {
        let mut d = document.clone();
        f(&mut d);
        d
    };
    let is_combination = |r: &Value| r["basis_ref"]["ref_type"] == "combination";
    vec![
        ("record absent", edit(&|d| {
            d["contract_evidence"].as_object_mut().unwrap().remove("exact_combinations");
        })),
        ("record empty", edit(&|d| d["contract_evidence"]["exact_combinations"] = json!([]))),
        ("record not an array", edit(&|d| d["contract_evidence"]["exact_combinations"] = json!({}))),
        ("record null", edit(&|d| d["contract_evidence"]["exact_combinations"] = Value::Null)),
        ("certificate absent", edit(&|d| {
            d["contract_evidence"].as_object_mut().unwrap().remove("combination_certificate");
        })),
        ("certificate null", edit(&|d| d["contract_evidence"]["combination_certificate"] = Value::Null)),
        ("no combination (record and rows absent, certificate kept)", edit(&|d| {
            d["contract_evidence"].as_object_mut().unwrap().remove("exact_combinations");
            if let Some(rows) = d["results"].as_array_mut() {
                rows.retain(|r| !is_combination(r));
            }
        })),
        ("no combination and no certificate", edit(&|d| {
            let e = d["contract_evidence"].as_object_mut().unwrap();
            e.remove("exact_combinations");
            e.remove("combination_certificate");
            if let Some(rows) = d["results"].as_array_mut() {
                rows.retain(|r| !is_combination(r));
            }
        })),
        ("rows absent", edit(&|d| {
            d.as_object_mut().unwrap().remove("results");
        })),
        ("rows not an array", edit(&|d| d["results"] = json!({"a": 1}))),
        ("rows reversed", edit(&|d| {
            if let Some(rows) = d["results"].as_array_mut() {
                rows.reverse();
            }
        })),
        ("a direct row's basis names a combination", edit(&|d| {
            if let Some(row) = d["results"].as_array_mut().and_then(|rows| rows.iter_mut().find(|r| !is_combination(r))) {
                row["basis_ref"]["ref_type"] = json!("combination");
            }
        })),
        ("an unknown evidence member", edit(&|d| d["contract_evidence"]["unknown"] = json!(1))),
        ("an unknown top-level member", edit(&|d| d["unknown"] = json!(1))),
        ("evidence not an object", edit(&|d| d["contract_evidence"] = json!([1]))),
        ("evidence absent", edit(&|d| {
            d.as_object_mut().unwrap().remove("contract_evidence");
        })),
        ("document null", Value::Null),
        ("document an array", json!([document.clone()])),
    ]
}

/// Every document: (label, document, read by the pressure reader).
fn documents() -> Vec<(String, Value, bool)> {
    let mut out = Vec::new();
    let certificate: Value = serde_json::from_str(CERTIFICATE_CORPUS).unwrap();
    let corpus = certificate["documents"].as_array().unwrap();
    for e in corpus {
        out.push((format!("certificate/{}", e["name"].as_str().unwrap()), e["document"].clone(), e["reader"] == "pressure"));
    }
    let forgeries: Value = serde_json::from_str(FORGERIES).unwrap();
    for forgery in forgeries["forgeries"].as_array().unwrap() {
        let e = corpus.iter().find(|e| e["name"] == forgery["document"]).unwrap();
        let mut document = e["document"].clone();
        for edit in forgery["edits"].as_array().unwrap() {
            apply(&mut document, edit);
        }
        out.push((format!("forgery/{}: {}", forgery["document"].as_str().unwrap(), forgery["what"].as_str().unwrap()), document, e["reader"] == "pressure"));
    }
    let combination: Value = serde_json::from_str(COMBINATION_CORPUS).unwrap();
    for case in combination["cases"].as_array().unwrap() {
        out.push((format!("combination/{}", case["label"].as_str().unwrap()), case["envelope"].clone(), true));
    }
    let withheld: Value = serde_json::from_str(WITHHELD).unwrap();
    out.push(("rt_a3_ce2_withheld_sparse".into(), withheld["envelope"].clone(), true));
    let pressure: Vec<(String, Value)> = out
        .iter()
        .filter(|(label, _, pressure)| *pressure && !label.starts_with("forgery/"))
        .map(|(label, document, _)| (label.clone(), document.clone()))
        .collect();
    for (label, document) in pressure {
        for (what, mutated) in mutations(&document) {
            out.push((format!("{label} / {what}"), mutated, true));
        }
    }
    out
}

// ------------------------------------------------------------------ the proof

#[test]
fn the_one_view_is_the_former_two_copies_view_on_every_document() {
    let documents = documents();
    // 7 + 5 + 1 corpus documents, the forgeries, and 18 mutations of each of the 8 pressure documents.
    let forgeries = documents.iter().filter(|(l, _, _)| l.starts_with("forgery/")).count();
    assert!(forgeries >= 140, "{forgeries}");
    assert_eq!(documents.len(), 13 + forgeries + 8 * 18);
    let (mut ok, mut missing, mut shape) = (0, 0, 0);
    for (label, document, _) in &documents {
        let view = ec::direct_view(document);
        assert_eq!(view, reference_view(document), "{label}");
        match view {
            Ok(_) => ok += 1,
            Err(e) if e == "SOURCE_PHYSICS_COMBINATION_EVIDENCE_MISSING" => missing += 1,
            Err(e) if e == "SOURCE_PHYSICS_COMBINATION_EVIDENCE_SHAPE" => shape += 1,
            Err(e) => panic!("{label}: {e}"),
        }
    }
    // Each outcome is reached.
    assert!(ok > 100 && missing > 10 && shape >= 24, "{ok} {missing} {shape}");
}

#[test]
fn the_pressure_reader_through_the_one_view_refuses_as_through_the_two() {
    let (mut admitted, mut refused) = (0, 0);
    for (label, document, pressure) in documents() {
        if !pressure {
            continue;
        }
        let verdict = super::validate_pressure_evidence_without_certificate(&document);
        assert_eq!(verdict, reference_without_certificate(&document), "{label}");
        if verdict.is_ok() {
            admitted += 1;
        } else {
            refused += 1;
        }
    }
    assert!(admitted >= 8 && refused >= 50, "{admitted} {refused}");
}
