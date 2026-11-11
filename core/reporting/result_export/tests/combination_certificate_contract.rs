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

/// Any certificate claim a document still makes: the member, or a diagnostic in the
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
/// is admitted as no certificate claim: no member, no certificate notice under any
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
/// and they make no certificate claim.
#[test]
fn legacy_w_cb3_successors_are_accepted() {
    for text in [
        // The registered pre-certificate bytes, kept when §8.53 regenerated the live successors with the member.
        include_str!("../../../../fixtures/results/retained_precision_combination_successor_sparse_interactive_pre_certificate.json"),
        include_str!("../../../../fixtures/results/retained_precision_combination_successor_dense_scrutiny_pre_certificate.json"),
    ] {
        let w: Value = serde_json::from_str(text).unwrap();
        assert!(claims_survive(&w["source"]).is_empty());
        let bound = rp::validate(&w["source"], Some(&w["invocation"])).unwrap();
        assert!(bound.invocation_bound && bound.numerical_eligible);
        let unbound = rp::validate(&w["source"], None).unwrap();
        assert!(!unbound.invocation_bound && !unbound.numerical_eligible);
    }
}

/// on the registered build the
/// W1 freeze admits the member, so the live W-CB3 successors carry the ordinary run's certificate. The full retained
/// reader accepts them with and without their invocation (G7's base read checks the member against the projected
/// rows), the projected base reads true from an ordinary entry too (member, records, notices and roster), and the
/// certificate's notices are exactly the member's, one per record.
#[test]
fn registered_w_cb3_successors_carry_the_certificate_and_it_reads_true() {
    for text in [
        include_str!("../../../../fixtures/results/retained_precision_combination_successor_sparse_interactive.json"),
        include_str!("../../../../fixtures/results/retained_precision_combination_successor_dense_scrutiny.json"),
    ] {
        let w: Value = serde_json::from_str(text).unwrap();
        let source = &w["source"];
        let member = &source["contract_evidence"]["combination_certificate"];
        assert!(member.is_object() && claims_survive(source).iter().any(|c| c == "member"));
        let bound = rp::validate(source, Some(&w["invocation"])).unwrap();
        assert!(bound.invocation_bound && bound.numerical_eligible);
        let unbound = rp::validate(source, None).unwrap();
        assert!(!unbound.invocation_bound && !unbound.numerical_eligible);
        let mut base = source.clone();
        base.as_object_mut().unwrap().remove("retained_precision");
        base["producer"]["semantic_contract_id"] = serde_json::json!("openpipestress.result_semantics/0.3.0/preview-physics-2");
        base["formulation_basis"]["profile_id"] = serde_json::json!("product_preview_mechanics_v1");
        let rows = base["results"].as_array_mut().unwrap();
        rows.retain(|r| r["kind"] != rp::REQUESTED_MODE_KIND);
        rows.iter_mut().for_each(|r| {
            r.as_object_mut().unwrap().remove("recovery_method");
        });
        assert!(!open_pipe_stress_result_export::in_retained_base_read());
        assert!(open_pipe_stress_result_export::semantic_contract::for_source(&base).is_ok());
        let notices: Vec<&Value> = source["diagnostics"].as_array().unwrap().iter()
            .filter(|d| d["id"].as_str().unwrap().starts_with("diagnostic:combination-certificate:")).collect();
        let records = member["combinations"].as_array().unwrap();
        assert_eq!(notices.len(), records.len());
        for (notice, record) in notices.iter().zip(records) {
            assert_eq!(notice["affected_refs"], serde_json::json!([record["combination_id"]]));
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

/// The retained base-read context (Agent 0's ruling after review cert2) is
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

// ------------------------------------------------------------------ the held gate notices (RT-A5)

const HELD_CORPUS: &str = include_str!("../../../../fixtures/results/preview_physics_held_status_corpus.json");
const HELD_GATE: &str = "diagnostic:preview-physics:combination-gate:13:comb:o-s-held";
/// Claim-like refusals: no reader polices R's language (Agent 0, 2026-11-05: R is bound to its
/// record, not verified as true, and supplies no row bound, eligibility, certificate notice or
/// standing). The Python mirror's `HELD_CLAIMS`.
const HELD_CLAIMS: [&str; 4] = [
    "its rows are established within their stated bound",
    "every row is bounded by 1e-300 within its stated bound",
    "ESTABLISHED Within Its Stated BOUND",
    "les lignes sont établies \u{2014} within their stated bound \u{2264} 2\u{207b}\u{b9}\u{2070}\u{2070}\u{2070} \u{2713}",
];
const G1: &str = "EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING";
const G1_TEXT: &str = "Combination withheld: it declares an operating minus sustained difference, and the subtrahend case's primitive loads, as a multiset by content (category, target, direction, dimension, value and unit after normalization; ids ignored), are not contained in the minuend case's.";

fn held_entry(name: &str) -> Value {
    let corpus: Value = serde_json::from_str(HELD_CORPUS).unwrap();
    corpus["documents"].as_array().unwrap().iter().find(|e| e["name"] == name).unwrap().clone()
}
fn held_notice(d: &mut Value) -> &mut Value {
    d["diagnostics"].as_array_mut().unwrap().iter_mut().find(|n| n["id"] == HELD_GATE).unwrap()
}
fn held_refusal(d: &mut Value) -> &mut Value {
    &mut d["contract_evidence"]["held_status_derivation"][0]["standing"]["reason"]
}
fn gate<'a>(d: &'a mut Value, id: &str) -> &'a mut Value {
    d["contract_evidence"]["combination_gates"].as_array_mut().unwrap().iter_mut().find(|g| g["combination_id"] == id).unwrap()
}

/// a held gate's notice is set-bound, required
/// once, and exactly "Combination withheld: " + R + ".", R the refusal of the document's
/// held-status record for that combination, bound to the record and never verified as true.
/// The same list as the Python mirror's `HELD_GATE_CASES` (`tests/test_combination_certificate.py`),
/// with and without the model.
#[test]
fn held_gate_notices_are_the_records_refusal_in_their_frame() {
    type Edit = fn(&mut Value);
    let cases: [(&str, bool, &str, Option<Edit>, Option<&str>); 10] = [
        ("canonical source-unavailable notice", true, "source_unavailable", None, None),
        ("canonical sliding-ground notice", true, "sliding_ground_differs", None, None),
        // The roster compares the notice whole with "Combination withheld: " + R + ".".
        ("inner text other than the record's refusal", true, "source_unavailable",
         Some(|d| held_notice(d)["message"] = "Combination withheld: the minuend case case:O lent a state.".into()), Some("SOURCE_COMBINATION_CERTIFICATE_ROSTER_CONTRADICTION")),
        ("frame without its prefix", true, "source_unavailable",
         Some(|d| { let m = held_notice(d)["message"].as_str().unwrap().trim_start_matches("Combination withheld: ").to_string(); held_notice(d)["message"] = m.into(); }), Some("SOURCE_COMBINATION_CERTIFICATE_ROSTER_CONTRADICTION")),
        ("frame without its period", true, "source_unavailable",
         Some(|d| { let m = held_notice(d)["message"].as_str().unwrap().trim_end_matches('.').to_string(); held_notice(d)["message"] = m.into(); }), Some("SOURCE_COMBINATION_CERTIFICATE_ROSTER_CONTRADICTION")),
        // The held-status reader binds the record's kind and a non-empty refusal, not its text (R is
        // bound, not verified), so a refusal changed in the evidence alone is refused first by the
        // roster, at this full public reader.
        ("refusal changed in the evidence only", true, "source_unavailable",
         Some(|d| { let r = format!("{} (edited)", held_refusal(d).as_str().unwrap()); *held_refusal(d) = r.into(); }), Some("SOURCE_COMBINATION_CERTIFICATE_ROSTER_CONTRADICTION")),
        // The gates reader (`preview_combination_gates`), which precedes the held-status reader and
        // the roster, refuses a held code on a combination with no held record, and another code on
        // a held record's gate.
        ("a held code on a non-held gate", false, "pf_held",
         Some(|d| {
             gate(d, "comb:own")["reason"] = "HELD_STATUS_SOURCE_UNAVAILABLE".into();
             let n = d["diagnostics"].as_array_mut().unwrap().iter_mut()
                 .find(|n| n["id"].as_str().unwrap().starts_with("diagnostic:preview-physics:combination-gate:") && n["affected_refs"] == serde_json::json!(["comb:own"])).unwrap();
             n["code"] = "HELD_STATUS_SOURCE_UNAVAILABLE".into();
             n["message"] = "Combination withheld: the minuend case case:O2 has no converged nonlinear support state to lend.".into();
         }), Some("SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_DERIVATION")),
        ("a non-held code on the held gate", true, "source_unavailable",
         Some(|d| { gate(d, "comb:o-s-held")["reason"] = G1.into(); held_notice(d)["code"] = G1.into(); held_notice(d)["message"] = G1_TEXT.into(); }),
         Some("SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_DERIVATION")),
        ("duplicated", true, "source_unavailable", Some(|d| { let n = held_notice(d).clone(); d["diagnostics"].as_array_mut().unwrap().push(n); }), Some("SOURCE_COMBINATION_CERTIFICATE_ROSTER_EXTRA")),
        ("dropped", true, "source_unavailable", Some(|d| d["diagnostics"].as_array_mut().unwrap().retain(|n| n["id"] != HELD_GATE)), Some("SOURCE_COMBINATION_CERTIFICATE_ROSTER_MISSING")),
    ];
    for (what, held, name, edit, code) in cases {
        let mut e = if held { held_entry(name) } else { documents().into_iter().find(|e| e["name"] == name).unwrap() };
        if let Some(edit) = edit {
            edit(&mut e["document"]);
        }
        for model in [None, Some(&e["model"])] {
            let verdict = reader_verdict::validate("preview-physics", &e["document"], model, None);
            let label = if model.is_some() { "with model" } else { "without model" };
            match code {
                None => assert_eq!(verdict["ok"], true, "{what} ({label}): {verdict}"),
                Some(code) => assert_eq!((&verdict["ok"], verdict["error"]["code"].as_str()), (&Value::Bool(false), Some(code)), "{what} ({label}): {verdict}"),
            }
        }
    }
}

/// What a reading could take as standing: the certificate member, numerical quality, the rows and
/// their bounds, and the held-status record less its refusal text.
fn standing(d: &Value) -> Value {
    let mut record = d["contract_evidence"]["held_status_derivation"][0].clone();
    record["standing"].as_object_mut().unwrap().remove("reason");
    serde_json::json!({"certificate": d["contract_evidence"]["combination_certificate"], "quality": d["numerical_quality"],
                       "results": d["results"], "status": d["status"], "held": record})
}

/// A well-bound record carrying claim-like text is admitted, and it changes no row bound,
/// eligibility or certificate standing against the benign refusal; forged on one side only, the
/// roster refuses it (its notice is no longer the canonical entry built from the record). The held
/// member stays not assessed, with no group and no bound.
#[test]
fn held_gate_claim_like_refusals_are_bound_never_policed_and_claim_nothing() {
    let benign = held_entry("source_unavailable");
    for claim in HELD_CLAIMS {
        for model in [None, Some(&benign["model"])] {
            assert_eq!(reader_verdict::validate("preview-physics", &benign["document"], model, None)["ok"], true);
            let mut bound = benign["document"].clone();
            *held_refusal(&mut bound) = claim.into();
            held_notice(&mut bound)["message"] = format!("Combination withheld: {claim}.").into();
            let verdict = reader_verdict::validate("preview-physics", &bound, model, None);
            assert_eq!(verdict["ok"], true, "{claim}: {verdict}");
            assert_eq!(standing(&bound), standing(&benign["document"]), "{claim}");
            let held = bound["contract_evidence"]["combination_certificate"]["combinations"].as_array().unwrap().iter()
                .find(|r| r["combination_id"] == "comb:o-s-held").unwrap();
            assert_eq!((&held["assessment"], &held["groups"], &held["uncovered_row_count"]), (&Value::from("not_assessed"), &serde_json::json!([]), &Value::from(0)));
            // Forged on one side only, refused at the full public reader. In the notice only, it is no
            // longer the bound entry (its whole message is not the record's canonical one), so the
            // certificate's own notice reader (`check`'s DIAGNOSTICS, before the roster) owns a notice in
            // its reserved wording (read case-sensitively: the mixed-case claim is the roster's), and the
            // roster any other; in the record only, the roster refuses first.
            let notice_owner = if claim == HELD_CLAIMS[2] { "SOURCE_COMBINATION_CERTIFICATE_ROSTER_CONTRADICTION" } else { "SOURCE_COMBINATION_CERTIFICATE_DIAGNOSTICS" };
            for (notice_side, code) in [(true, notice_owner), (false, "SOURCE_COMBINATION_CERTIFICATE_ROSTER_CONTRADICTION")] {
                let mut forged = benign["document"].clone();
                if notice_side {
                    held_notice(&mut forged)["message"] = format!("Combination withheld: {claim}.").into();
                } else {
                    *held_refusal(&mut forged) = claim.into();
                }
                let verdict = reader_verdict::validate("preview-physics", &forged, model, None);
                assert_eq!(verdict["error"]["code"], code, "{claim} (notice side {notice_side}): {verdict}");
            }
            // A well-bound claim-like refusal lifts only the vocabulary test: a forged binding or state
            // is still refused by the mandatory readers before the certificate (gates, then held status).
            let forgeries: [(&str, fn(&mut Value), &str); 4] = [
                ("its record's outcome", |d| d["contract_evidence"]["held_status_derivation"][0]["outcome"] = "HELD_STATUS_BINDING".into(), "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_DERIVATION"),
                ("its record's source case", |d| d["contract_evidence"]["held_status_derivation"][0]["source"]["load_case_id"] = "case:S".into(), "SOURCE_PREVIEW_PHYSICS_HELD_STATUS_BINDING"),
                ("its record's state", |d| d["contract_evidence"]["held_status_derivation"][0]["standing"]["kind"] = "established".into(), "SOURCE_PREVIEW_PHYSICS_HELD_STATUS_EVIDENCE"),
                ("its gate", |d| gate(d, "comb:o-s-held")["withheld"] = false.into(), "SOURCE_PREVIEW_PHYSICS_GATE_REASON"),
            ];
            for (what, forge, code) in forgeries {
                let mut forged = bound.clone();
                forge(&mut forged);
                let verdict = reader_verdict::validate("preview-physics", &forged, model, None);
                assert_eq!((&verdict["ok"], verdict["error"]["code"].as_str()), (&Value::Bool(false), Some(code)), "{claim}: {what}: {verdict}");
            }
        }
    }
}

// ------------------------------------------------------------------ HELD_STATUS_RELEASED (RT-A5, P1)

const HELD_RELEASED: &str = "diagnostic:preview-physics:held-status-released:13:comb:o-s-held";
fn released_notice(d: &mut Value) -> &mut Value {
    d["diagnostics"].as_array_mut().unwrap().iter_mut().find(|n| n["id"] == HELD_RELEASED).unwrap()
}
fn replace_in_released(d: &mut Value, old: &str, new: &str) {
    let notice = released_notice(d);
    let message = notice["message"].as_str().unwrap().to_string();
    assert!(message.contains(old), "{old}: {message}");
    notice["message"] = message.replace(old, new).into();
}

/// one set-bound HELD_STATUS_RELEASED entry per held
/// combination whose record releases a support, its message rebuilt from the record's recorded fields
/// only, at the full reader, with and without the model; the Python mirror's `HELD_RELEASED_CASES`.
/// h2 is an ordinary release with a negative bit token, h7g a strict-gap one, h5 a displacement and h13
/// a branch.
#[test]
fn held_status_released_notices_are_rebuilt_from_the_record() {
    type Edit = fn(&mut Value);
    const CONTRADICTION: &str = "SOURCE_COMBINATION_CERTIFICATE_ROSTER_CONTRADICTION";
    const DERIVATION: &str = "SOURCE_PREVIEW_PHYSICS_HELD_STATUS_DERIVATION";
    let cases: [(&str, &str, Option<Edit>, Option<&str>); 12] = [
        ("canonical ordinary (negative bit token)", "h2_released", None, None),
        ("canonical strict gap", "h7g_gap", None, None),
        ("canonical displacement", "h5_slip_sense", None, None),
        ("canonical branch", "h13_branch", None, None),
        // The roster rebuilds the whole message from the validated record and compares it bit for bit.
        ("a changed bit token", "h2_released", Some(|d| replace_in_released(d, "c080000000000000", "c080000000000001")), Some(CONTRADICTION)),
        ("a changed sign", "h7g_gap", Some(|d| replace_in_released(d, "reaction_sign positive", "reaction_sign negative")), Some(CONTRADICTION)),
        ("a changed state", "h2_released", Some(|d| replace_in_released(d, "classified inactive", "classified active")), Some(CONTRADICTION)),
        ("a changed reason", "h2_released", Some(|d| replace_in_released(d, "(changed state:", "(branch:")), Some(CONTRADICTION)),
        // The held-status reader, which precedes the roster, refuses first what it binds: the notice's
        // named supports in record order, exactly one notice for a released record, and none for a
        // record that releases nothing (the roster's MISSING and EXTRA stand behind it).
        ("its supports out of order", "h2_released", Some(|d| { let n = released_notice(d); let mut refs = n["affected_refs"].as_array().unwrap().clone(); refs.reverse(); n["affected_refs"] = refs.into(); }), Some(DERIVATION)),
        ("duplicated", "h2_released", Some(|d| { let n = released_notice(d).clone(); d["diagnostics"].as_array_mut().unwrap().push(n); }), Some(DERIVATION)),
        ("dropped", "h2_released", Some(|d| d["diagnostics"].as_array_mut().unwrap().retain(|n| n["id"] != HELD_RELEASED)), Some(DERIVATION)),
        ("on a combination that releases nothing", "h4_kept_explicit",
         Some(|d| { let n = released_notice(&mut held_entry("h2_released")["document"]).clone(); d["diagnostics"].as_array_mut().unwrap().push(n); }), Some(DERIVATION)),
    ];
    for (what, name, edit, code) in cases {
        let mut e = held_entry(name);
        if let Some(edit) = edit {
            edit(&mut e["document"]);
        }
        for model in [None, Some(&e["model"])] {
            let verdict = reader_verdict::validate("preview-physics", &e["document"], model, None);
            let label = if model.is_some() { "with model" } else { "without model" };
            match code {
                None => assert_eq!(verdict["ok"], true, "{what} ({label}): {verdict}"),
                Some(code) => assert_eq!((&verdict["ok"], verdict["error"]["code"].as_str()), (&Value::Bool(false), Some(code)), "{what} ({label}): {verdict}"),
            }
        }
    }
}
