//! The report's projection of a result's diagnostics (DEL-08-01 Design
//! `exact-route-report-sections.md` §3.6 and §6.3 item 4) against the shared
//! parity vectors `fixtures/reports/invented/report_diagnostics_parity.json`.
//! The desktop's projection (`diagnosticsForSections`) is checked against the
//! same file by `apps/desktop/src/features/report/reportDiagnostics.parity.test.ts`.

use open_pipe_stress_report_package::bindings::{check_report_diagnostics, expected_report_diagnostics, BINDING_MISMATCH};
use serde_json::{json, Value};

const VECTORS: &str = include_str!("../../../../fixtures/reports/invented/report_diagnostics_parity.json");

fn vectors() -> Vec<Value> {
    let file: Value = serde_json::from_str(VECTORS).unwrap();
    file["vectors"].as_array().unwrap().clone()
}

/// A report whose Diagnostics are `entries`, each with a session provenance.
fn report_with(entries: &[Value]) -> Value {
    let entries: Vec<Value> = entries
        .iter()
        .map(|entry| {
            let mut entry = entry.clone();
            entry["provenance"] = json!({"source_name": "invented session", "review_status": "pending"});
            entry
        })
        .collect();
    json!({"report_sections": {"diagnostics": entries}})
}

#[test]
fn rust_projection_matches_the_shared_parity_vectors() {
    let vectors = vectors();
    assert!(vectors.len() >= 20);
    for vector in &vectors {
        let reason = vector["reason"].as_str().unwrap();
        assert!(!reason.is_empty(), "every vector states its reason");
        let source = json!({"diagnostics": vector["diagnostics"]});
        let projected = expected_report_diagnostics(&source).unwrap_or_else(|error| panic!("{reason}: {error}"));
        assert_eq!(Value::Array(projected), vector["expected"], "{reason}");
        // A report carrying the projection, with session provenance, passes the check.
        let expected = vector["expected"].as_array().unwrap();
        assert_eq!(check_report_diagnostics(&report_with(expected), &source), Ok(()), "{reason}");
    }
}

/// The edge cases the projection must cover are present in the vectors.
#[test]
fn the_vectors_cover_the_stated_edge_cases() {
    let vectors = vectors();
    let has = |test: &dyn Fn(&Value, &Value) -> bool| {
        vectors.iter().any(|vector| {
            let pairs = vector["diagnostics"].as_array().unwrap().iter().zip(vector["expected"].as_array().unwrap());
            pairs.into_iter().any(|(source, expected)| test(source, expected))
        })
    };
    let affected = |expected: &Value| expected["affected_object"]["ref_id"].as_str().unwrap().to_string();
    assert!(has(&|s, e| s.get("source").is_none() && e["source"]["ref_id"] == "core/product_physics"), "missing source");
    assert!(has(&|s, e| s["source"].is_null() && s.get("source").is_some() && e["source"]["ref_id"] == "core/product_physics"), "null source");
    assert!(has(&|s, e| s["source"] == "" && e["source"]["ref_id"] == ""), "empty source kept");
    assert!(has(&|s, e| s["affected_refs"] == json!([]) && affected(e) == s["id"]), "empty affected_refs falls back to the id");
    assert!(has(&|s, e| s.get("affected_refs").is_none() && affected(e) == s["id"]), "absent affected_refs falls back to the id");
    assert!(has(&|s, e| s["id"] == "" && affected(e).starts_with("diagnostic-") && affected(e) != "diagnostic-0"), "empty id falls back to a non-zero index");
    assert!(has(&|s, e| s.get("id").is_none() && affected(e).starts_with("diagnostic-")), "absent id falls back to the index");
    assert!(has(&|s, e| s["severity"] == "error" && e["severity"] == "blocking" && e["class"] == "SOLVE_BLOCKING"), "error");
    assert!(has(&|s, e| s["severity"] == "blocking" && e["severity"] == "blocking"), "blocking");
    assert!(has(&|s, e| s["severity"] == "warning" && e["severity"] == "warning" && e["class"] == "ASSUMPTION_WARNING"), "warning");
    assert!(has(&|s, e| s["severity"] == "info" && e["severity"] == "info" && e["class"] == "ASSUMPTION_WARNING"), "info");
    assert!(has(&|s, e| s["affected_refs"].as_array().is_some_and(|refs| refs.len() == 2) && affected(e).contains(", ")), "joined refs");
}

/// A source diagnostic not in the producer's form refuses: the desktop's
/// projection of it would not be a report entry the renderer reads.
#[test]
fn an_unreadable_source_diagnostic_refuses() {
    let good = json!({"id": "d", "code": "C", "severity": "warning", "message": "m", "source": "s", "affected_refs": ["r"]});
    let edits: Vec<(&str, Box<dyn Fn(&mut Value)>)> = vec![
        ("code", Box::new(|d: &mut Value| d["code"] = json!(1))),
        ("severity", Box::new(|d: &mut Value| { d.as_object_mut().unwrap().remove("severity"); })),
        ("message", Box::new(|d: &mut Value| d["message"] = Value::Null)),
        ("source", Box::new(|d: &mut Value| d["source"] = json!(["s"]))),
        ("affected_refs", Box::new(|d: &mut Value| d["affected_refs"] = json!("r"))),
        ("affected_refs", Box::new(|d: &mut Value| d["affected_refs"] = json!([1]))),
        ("id", Box::new(|d: &mut Value| { d["id"] = json!(7); d.as_object_mut().unwrap().remove("affected_refs"); })),
    ];
    for (field, edit) in edits {
        let mut diagnostic = good.clone();
        edit(&mut diagnostic);
        let error = expected_report_diagnostics(&json!({"diagnostics": [diagnostic]})).expect_err(field);
        assert_eq!(error, format!("{BINDING_MISMATCH}: report diagnostics: source diagnostic 0 has an unreadable {field}"));
    }
    let error = expected_report_diagnostics(&json!({"results": []})).expect_err("no diagnostics");
    assert_eq!(error, format!("{BINDING_MISMATCH}: report diagnostics: the source has no diagnostics"));
}

/// The check over invented entries: every bound field, the order, the count,
/// and an appended integrity entry. Appended entries of other codes pass, and
/// the provenance is not bound.
#[test]
fn the_check_binds_each_field_the_order_and_the_appended_entries() {
    let source = json!({"diagnostics": [
        {"id": "diagnostic:numerical-integrity:case", "code": "NUMERICAL_INTEGRITY_SENSITIVE", "severity": "warning", "message": "invented evidence", "source": "core/product_physics", "affected_refs": ["case"]},
        {"id": "diagnostic:invented", "code": "INVENTED", "severity": "info", "message": "invented"}
    ]});
    let expected = expected_report_diagnostics(&source).unwrap();
    let appended = json!({"code": "COMPONENT_PROVENANCE_MISSING", "class": "ASSUMPTION_WARNING", "severity": "warning",
        "source": {"ref_type": "component", "ref_id": "c"}, "affected_object": {"ref_type": "component", "ref_id": "c"},
        "message": "invented", "remediation": "invented"});
    let genuine = report_with(&[expected[0].clone(), expected[1].clone(), appended]);
    assert_eq!(check_report_diagnostics(&genuine, &source), Ok(()));
    let mut other_provenance = genuine.clone();
    other_provenance["report_sections"]["diagnostics"][0]["provenance"] = json!({"source_name": "another session"});
    assert_eq!(check_report_diagnostics(&other_provenance, &source), Ok(()), "the provenance is not bound");
    let refusal = |what: &str| Err(format!("{BINDING_MISMATCH}: {what}"));
    let edits: Vec<(&str, Box<dyn Fn(&mut Vec<Value>)>, Result<(), String>)> = vec![
        ("deleted", Box::new(|d| { d.remove(0); }), refusal("report diagnostic 0 code")),
        ("renamed", Box::new(|d| d[0]["code"] = json!("INVENTED_RENAMED")), refusal("report diagnostic 0 code")),
        ("passed at info", Box::new(|d| { d[0]["code"] = json!("NUMERICAL_INTEGRITY_CHECKS_PASSED"); d[0]["severity"] = json!("info"); }), refusal("report diagnostic 0 code")),
        ("downgraded", Box::new(|d| d[0]["severity"] = json!("info")), refusal("report diagnostic 0 severity")),
        ("class", Box::new(|d| d[0]["class"] = json!("SOLVE_BLOCKING")), refusal("report diagnostic 0 class")),
        ("source", Box::new(|d| d[0]["source"]["ref_id"] = json!("invented")), refusal("report diagnostic 0 source")),
        ("message", Box::new(|d| d[0]["message"] = json!("invented edit")), refusal("report diagnostic 0 message")),
        ("affected object", Box::new(|d| d[0]["affected_object"]["ref_id"] = json!("invented")), refusal("report diagnostic 0 affected_object")),
        ("remediation", Box::new(|d| d[0]["remediation"] = json!("invented")), refusal("report diagnostic 0 remediation")),
        ("reordered", Box::new(|d| d.swap(0, 1)), refusal("report diagnostic 0 code")),
        ("truncated", Box::new(|d| d.truncate(1)), refusal("report diagnostics cover the source diagnostics")),
        ("emptied", Box::new(|d| d.clear()), refusal("report diagnostics cover the source diagnostics")),
        (
            "an appended integrity entry",
            Box::new(|d| { let mut forged = d[0].clone(); forged["code"] = json!("NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED"); d.push(forged); }),
            refusal("report diagnostic 3 is an integrity code the source does not carry"),
        ),
        (
            "an appended passed check",
            Box::new(|d| { let mut forged = d[1].clone(); forged["code"] = json!("NUMERICAL_INTEGRITY_CHECKS_PASSED"); d.push(forged); }),
            refusal("report diagnostic 3 is an integrity code the source does not carry"),
        ),
        ("an appended other entry", Box::new(|d| { let copy = d[2].clone(); d.push(copy); }), Ok(())),
    ];
    for (label, edit, outcome) in edits {
        let mut report = genuine.clone();
        edit(report["report_sections"]["diagnostics"].as_array_mut().unwrap());
        assert_eq!(check_report_diagnostics(&report, &source), outcome, "{label}");
    }
    // A report without Diagnostics refuses a source that has some, and passes one that has none.
    assert_eq!(check_report_diagnostics(&json!({}), &source), refusal("report diagnostics cover the source diagnostics"));
    assert_eq!(check_report_diagnostics(&json!({}), &json!({"diagnostics": []})), Ok(()));
}
