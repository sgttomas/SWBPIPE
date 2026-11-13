//! The report's projection of a result's diagnostics (DEL-08-01 Design
//! `exact-route-report-sections.md` §3.6 and §6.3 item 4) against the shared
//! parity vectors `fixtures/reports/invented/report_diagnostics_parity.json`.
//! The desktop's projection (`diagnosticsForSections`) is checked against the
//! same file by `apps/desktop/src/features/report/reportDiagnostics.parity.test.ts`.

use open_pipe_stress_report_package::bindings::{check_report_diagnostics, expected_report_diagnostics, BINDING_MISMATCH, SUPPLEMENTAL_DIAGNOSTIC_LABEL};
use serde_json::{json, Value};

mod common;

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
        // RT-A3 §4: a vector may state the result's combination evidence, rows and numerical cases.
        let mut source = json!({"diagnostics": vector["diagnostics"]});
        for member in ["contract_evidence", "results", "numerical_quality"] {
            if let Some(value) = vector.get(member) {
                source[member] = value.clone();
            }
        }
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
    // RT-A3 §4: supplied text quoted on a result with combinations, a roster entry copied, and prose copied on a
    // result without one.
    let quoted = |s: &Value, e: &Value| e["message"] == format!("{SUPPLEMENTAL_DIAGNOSTIC_LABEL}\"{}\"", s["message"].as_str().unwrap());
    assert!(has(&|s, e| s["code"] == "REVIEW_NOTE" && quoted(s, e)), "supplied text quoted");
    assert!(has(&|s, e| s["code"] == "COMBINATION_STRESS_MAXIMUM_UNAVAILABLE" && e["message"] == s["message"]), "roster entry copied");
    assert!(has(&|s, e| s["code"] == "REVIEW_NOTE" && e["message"] == s["message"]), "no combination: copied");
}

/// RT-A3 §4 (DEL-05-02 Work `combination-certificate.md` §2, the residual text boundary) on an actual
/// certificate document, whose combination identities carry no `comb:` prefix: each diagnostic naming a
/// combination (an entry of the reader's closed roster, the certificate's notice among them) is copied, and
/// every other message, the certificate review's claim under a load-case ref among them, is quoted after the
/// supplemental label. The reader admits the added note as supplemental text.
#[test]
fn a_certificate_document_copies_roster_entries_and_quotes_supplied_text() {
    let mut source: Value = serde_json::from_str(include_str!("../../../../fixtures/results/preview_physics_range_envelope_sparse.json")).unwrap();
    let claim = "combination combination:U4B-RANGE-MAX: all derived row groups are established within their stated\u{a0}bound.";
    source["diagnostics"].as_array_mut().unwrap().push(json!({"id": "diagnostic:review:claim", "code": "REVIEW_NOTE", "severity": "info",
        "source": "core/product_physics", "affected_refs": ["load:L-100"], "message": claim}));
    assert_eq!(open_pipe_stress_result_export::reader_verdict::validate("preview-physics", &source, None, None), json!({"ok": true}));
    let combinations = ["combination:C-OPER-ALT", "combination:U4B-RANGE-MAX"];
    let projected = expected_report_diagnostics(&source).unwrap();
    let diagnostics = source["diagnostics"].as_array().unwrap();
    assert_eq!(projected.len(), diagnostics.len());
    for (d, entry) in diagnostics.iter().zip(&projected) {
        let names = d["affected_refs"].as_array().is_some_and(|refs| refs.iter().any(|r| combinations.iter().any(|c| r == c)));
        let message = d["message"].as_str().unwrap();
        let expected = if names { message.to_string() } else { format!("{SUPPLEMENTAL_DIAGNOSTIC_LABEL}\"{message}\"") };
        assert_eq!(entry["message"], expected.as_str(), "{}", d["id"]);
    }
    let notice = projected.iter().find(|e| e["code"] == "COMBINATION_ROWS_NOT_ASSESSED").unwrap();
    assert!(!notice["message"].as_str().unwrap().starts_with(SUPPLEMENTAL_DIAGNOSTIC_LABEL));
    assert_eq!(projected.last().unwrap()["message"], format!("{SUPPLEMENTAL_DIAGNOSTIC_LABEL}\"{claim}\""));
}

/// Review cert3 finding 2 (Agent 0's no-masking rule): an unused model material sharing a combination's
/// id does not take it out of the reader's roster. The report's source-only population names
/// `comb:cancel` as both readings do: the canonical document with the colliding model is admitted and its
/// roster entries are copied, and a claim under that id is refused with and without the model, so the
/// projection never copies a message a reader left unchecked.
#[test]
fn a_model_only_id_collision_neither_unrosters_a_claim_nor_splits_the_classification() {
    let corpus: Value = serde_json::from_str(include_str!("../../../../fixtures/results/combination_certificate_corpus.json")).unwrap();
    let e = corpus["documents"].as_array().unwrap().iter().find(|e| e["name"] == "ex_cantilever").unwrap();
    let (mut source, mut model) = (e["document"].clone(), e["model"].clone());
    let mut material = model["materials"][0].clone();
    material["id"] = json!("comb:cancel");
    model["materials"].as_array_mut().unwrap().push(material);
    let validate = |source: &Value, model: Option<&Value>| open_pipe_stress_result_export::reader_verdict::validate("pressure", source, model, None);
    assert_eq!(validate(&source, Some(&model)), json!({"ok": true}));
    assert!(open_pipe_stress_result_export::document_combination_identities(&source).contains("comb:cancel"));
    let canonical = expected_report_diagnostics(&source).unwrap();
    for (d, entry) in source["diagnostics"].as_array().unwrap().iter().zip(&canonical) {
        if d["affected_refs"].as_array().is_some_and(|refs| refs.iter().any(|r| r == "comb:cancel")) {
            assert_eq!(entry["message"], d["message"], "{}", d["id"]);
        }
    }
    let claim = "combination comb:cancel: all derived rows are bounded by 1e-30.";
    source["diagnostics"].as_array_mut().unwrap().push(json!({"id": "diagnostic:review:collision", "code": "REVIEW_NOTE", "severity": "info",
        "source": "core/product_physics", "affected_refs": ["comb:cancel"], "message": claim}));
    for model in [None, Some(&model)] {
        let verdict = validate(&source, model);
        assert!(verdict["ok"] == false && verdict.to_string().contains("SOURCE_COMBINATION_CERTIFICATE_ROSTER_EXTRA"), "{verdict}");
    }
    assert_eq!(expected_report_diagnostics(&source).unwrap().last().unwrap()["message"], claim);
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

/// RT-A5 (DEL-05-02 `operating-case.md` §7.5; Work `combination-certificate.md` §2): a held gate
/// notice's refusal text is bound to its held-status record, not verified as true, so the report
/// quotes a diagnostic whose typed code is a held gate code whole as supplied text, though the
/// roster names its combination and admitted it, whatever its text, and the source's bytes are
/// unchanged (desktop `heldStatusPresentation.test.tsx` alike).
#[test]
fn a_held_gate_notice_is_quoted_as_supplied_text_whatever_its_refusal() {
    const HELD: &str = include_str!("../../../../fixtures/results/preview_physics_held_status_corpus.json");
    const GATE: &str = "diagnostic:preview-physics:combination-gate:13:comb:o-s-held";
    let corpus: Value = serde_json::from_str(HELD).unwrap();
    let base = corpus["documents"].as_array().unwrap().iter().find(|e| e["name"] == "source_unavailable").unwrap()["document"].clone();
    let claims = ["its rows are established within their stated bound", "ESTABLISHED Within Its Stated BOUND", "les lignes sont établies \u{2014} within their stated bound \u{2713}"];
    let mut sources = vec![base.clone()];
    for claim in claims {
        let mut source = base.clone();
        source["contract_evidence"]["held_status_derivation"][0]["standing"]["reason"] = json!(claim);
        let at = source["diagnostics"].as_array().unwrap().iter().position(|d| d["id"] == GATE).unwrap();
        source["diagnostics"][at]["message"] = json!(format!("Combination withheld: {claim}."));
        sources.push(source);
    }
    let benign = expected_report_diagnostics(&base).unwrap();
    for source in sources {
        let before = serde_json::to_vec(&source).unwrap();
        let entries = expected_report_diagnostics(&source).unwrap();
        assert_eq!(serde_json::to_vec(&source).unwrap(), before, "raw bytes");
        for (index, diagnostic) in source["diagnostics"].as_array().unwrap().iter().enumerate() {
            if diagnostic["id"] == GATE {
                assert_eq!(entries[index]["message"], json!(format!("{SUPPLEMENTAL_DIAGNOSTIC_LABEL}\"{}\"", diagnostic["message"].as_str().unwrap())));
            } else {
                // No other entry changes with the refusal's text.
                assert_eq!(entries[index], benign[index], "{}", diagnostic["id"]);
            }
        }
        assert!(check_report_diagnostics(&report_with(&entries), &source).is_ok());
    }
}

/// RT-A5: `HELD_STATUS_RELEASED` is the sixth quoted held-status code (`HELD_STATUS_QUOTED_CODES`):
/// the report quotes it whole as supplied text on every released held document, bytes unchanged.
#[test]
fn a_held_status_released_notice_is_quoted_as_supplied_text() {
    const HELD: &str = include_str!("../../../../fixtures/results/preview_physics_held_status_corpus.json");
    let corpus: Value = serde_json::from_str(HELD).unwrap();
    for name in ["h2_released", "h5_slip_sense", "h13_branch", "h7g_gap"] {
        let source = corpus["documents"].as_array().unwrap().iter().find(|e| e["name"] == name).unwrap()["document"].clone();
        let before = serde_json::to_vec(&source).unwrap();
        let entries = expected_report_diagnostics(&source).unwrap();
        assert_eq!(serde_json::to_vec(&source).unwrap(), before, "{name}: raw bytes");
        let released: Vec<(usize, &Value)> = source["diagnostics"].as_array().unwrap().iter().enumerate().filter(|(_, d)| d["code"] == "HELD_STATUS_RELEASED").collect();
        assert_eq!(released.len(), 1, "{name}");
        let (index, diagnostic) = released[0];
        assert_eq!(entries[index]["message"], json!(format!("{SUPPLEMENTAL_DIAGNOSTIC_LABEL}\"{}\"", diagnostic["message"].as_str().unwrap())), "{name}");
    }
}

/// The Message cell of the one row whose code is `code` in the rendered
/// report's Diagnostics table (the table after the "Diagnostics" heading, up
/// to its end), as escaped HTML. Panics unless exactly one such row exists
/// and it has the table's four cells.
fn diagnostics_message_cell<'a>(html: &'a str, code: &str) -> &'a str {
    let start = html.find("<h3>Diagnostics</h3>\n<table>\n").expect("the Diagnostics table");
    let table = &html[start..start + html[start..].find("</table>").expect("its end")];
    let opening = format!("<tr><td>{code}</td>");
    assert_eq!(table.matches(&opening).count(), 1, "one {code} row");
    let row = &table[table.find(&opening).unwrap()..];
    let row = &row[..row.find("</tr>").unwrap()];
    let cells = row.strip_prefix("<tr><td>").unwrap().strip_suffix("</td>").unwrap().split("</td><td>").collect::<Vec<_>>();
    assert_eq!(cells.len(), 4, "code, severity, message, remediation");
    cells[2]
}

/// The report for a source carrying the warning for `case` and `node` at the
/// force `force`, rendered from the package's projection.
fn rendered_warning(case: &str, node: &str, force: f64) -> (String, String) {
    use open_pipe_stress_report_renderer::render_calculation_report;
    let message = open_pipe_stress_result_export::cap_remainder_warning_message(case, node, force);
    let source = json!({
        "contract_evidence": {"combination_gates": [{"combination_id": "comb:invented", "withheld": false, "reason": null}]},
        "diagnostics": [{"id": "diagnostic:pressure-runtime:invented:PRESSURE_REGION_CAP_REMAINDER_RETAINED",
            "code": open_pipe_stress_result_export::CAP_REMAINDER_WARNING_CODE, "severity": "warning", "message": message,
            "source": "core/product_physics/src/pressure_runtime.rs", "affected_refs": [case, "region:P", node]}]
    });
    let entries = expected_report_diagnostics(&source).unwrap();
    assert_eq!(entries[0]["message"], message.as_str(), "copied, not quoted as supplied text");
    let mut input = common::fixture_report_input();
    let provenance = serde_json::to_value(common::invented_section_provenance()).unwrap();
    input.report_sections.diagnostics = entries.into_iter().map(|mut entry| {
        entry["provenance"] = provenance.clone();
        serde_json::from_value(entry).unwrap()
    }).collect();
    let outcome = render_calculation_report(&input);
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    (outcome.html, message)
}

/// DEL-04-07: a person reading only the rendered report's Diagnostics sees,
/// for a case admitted under the cancellation screen's exception, the case,
/// the node at which the retained remainder is largest relative to the
/// pressure force on the bore, that remainder as a force with its unit, and
/// that the force is not a bound on the resulting displacement, all in that
/// warning's Message cell of the Diagnostics table. The warning's message is
/// the producer's form (`result_export::cap_remainder_warning_message`; the
/// admitting reader reads its force and binds the rest to the record), copied
/// on a result with a combination; the Diagnostics table shows a message and
/// not its affected references, so the case and the node reach the reader
/// through the sentence.
#[test]
fn a_report_reader_sees_the_case_the_node_the_force_and_that_it_is_no_bound() {
    use open_pipe_stress_report_renderer::{escape_html, DIAGNOSTIC_EVIDENCE_SUBHEADING};
    let (html, message) = rendered_warning("case:P", "node:C1", 0.2197357159078235);
    let cell = diagnostics_message_cell(&html, open_pipe_stress_result_export::CAP_REMAINDER_WARNING_CODE);
    assert_eq!(cell, escape_html(&message), "the whole message, in the cell");
    assert_eq!(html.matches(&escape_html(&message)).count(), 1, "once");
    for shown in ["In case case:P,", "at node node:C1,", "where it is 2.197357159078235e-1 N; that force is not a bound on the resulting displacement."] {
        assert!(cell.contains(&escape_html(shown)), "{shown}");
    }
    assert!(!html.contains(DIAGNOSTIC_EVIDENCE_SUBHEADING), "no evidence block");
    assert!(!html.contains(&format!("{SUPPLEMENTAL_DIAGNOSTIC_LABEL}")), "not as supplied text");
    assert!(!html.contains("case:P, region:P, node:C1"), "the table shows no affected references for this message");
}

/// DEL-04-07 (the fourth numerical scrutiny's finding 2): with long case and
/// node identifiers the warning passes the renderer's long-text limit, and it
/// still stays whole in its Diagnostics cell: the force with its unit and the
/// not-a-bound clause, the case and the node are in that cell, and no
/// diagnostic evidence entry is created for it.
#[test]
fn a_long_warning_stays_whole_in_its_diagnostics_cell() {
    use open_pipe_stress_report_renderer::{escape_html, DIAGNOSTIC_EVIDENCE_SUBHEADING, DIAGNOSTIC_MESSAGE_TABLE_LIMIT};
    let case = format!("case:{}", "long-case-".repeat(25));
    let node = format!("node:{}", "long-node-".repeat(25));
    let force = 2.98023223876953125e-8;
    let (html, message) = rendered_warning(&case, &node, force);
    assert!(message.chars().count() > DIAGNOSTIC_MESSAGE_TABLE_LIMIT, "{} characters", message.chars().count());
    let cell = diagnostics_message_cell(&html, open_pipe_stress_result_export::CAP_REMAINDER_WARNING_CODE);
    let sentence = format!("where it is {force:e} N; that force is not a bound on the resulting displacement.");
    assert!(cell.contains(&escape_html(&sentence)), "the force and the not-a-bound clause in the cell: {cell}");
    assert!(cell.contains(&escape_html(&format!("In case {case},"))) && cell.contains(&escape_html(&format!("at node {node},"))), "the case and the node in the cell");
    assert_eq!(cell, escape_html(&message), "the whole message, in the cell");
    assert!(!html.contains(DIAGNOSTIC_EVIDENCE_SUBHEADING), "no evidence block");
    assert!(!html.contains("full text: Diagnostic evidence"), "no evidence entry");
}

/// The renderer keeps this warning's code in its table-message exemption by
/// the result export's name for it.
#[test]
fn the_renderer_s_table_exemption_names_the_warning_s_code() {
    assert_eq!(open_pipe_stress_report_renderer::DIAGNOSTIC_MESSAGE_IN_TABLE_CODES, [open_pipe_stress_result_export::CAP_REMAINDER_WARNING_CODE]);
}
