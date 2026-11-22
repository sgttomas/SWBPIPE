//! DEL-08-01 AC-016 and VER-016: every tag of the numerical vocabulary has
//! exactly one reviewed statement, and every statement keys a tag.
//!
//! - The renderer maps each tag to its statement with a match that has no
//!   wildcard arm (`report_renderer::standing`), so a tag without a statement
//!   does not compile.
//! - The generated catalogue (`fixtures/reports/numerical_vocabulary.json`) is
//!   written from the vocabulary crate's lists, never by scanning source, and
//!   the generated statement table (`fixtures/reports/standing_statements.json`)
//!   from the renderer's statements. This test compares the two in both
//!   directions and holds each committed file current; regenerate both with
//!   `OPENPIPESTRESS_WRITE_STANDING_STATEMENTS=1`.
use open_pipe_stress_numerical_vocabulary::catalogue;
use open_pipe_stress_report_renderer::standing::statement_table;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::path::PathBuf;

const WRITE: &str = "OPENPIPESTRESS_WRITE_STANDING_STATEMENTS";

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/reports").join(name)
}

fn catalogue_document() -> Value {
    json!({
        "document_kind": "swbpipe.numerical_vocabulary_catalogue",
        "version": 1,
        "generator": "core/reporting/report_renderer/tests/standing_statements.rs",
        "tags": catalogue().iter().map(|e| json!({"vocabulary": e.vocabulary, "tag": e.tag})).collect::<Vec<_>>(),
    })
}

fn statement_document() -> Value {
    json!({
        "document_kind": "swbpipe.standing_statements",
        "version": 1,
        "generator": "core/reporting/report_renderer/tests/standing_statements.rs",
        "statements": statement_table().into_iter().map(|(vocabulary, tag, statement)| json!({"vocabulary": vocabulary, "tag": tag, "statement": statement})).collect::<Vec<_>>(),
    })
}

fn text(value: &Value) -> String {
    format!("{}\n", serde_json::to_string_pretty(value).unwrap())
}

#[test]
fn every_tag_has_exactly_one_statement_and_every_statement_keys_a_tag() {
    let tags: Vec<(&str, &str)> = catalogue().iter().map(|e| (e.vocabulary, e.tag)).collect();
    let keyed: Vec<(&str, &str)> = statement_table().iter().map(|(v, t, _)| (*v, *t)).collect();
    let tag_set: BTreeSet<_> = tags.iter().copied().collect();
    let keyed_set: BTreeSet<_> = keyed.iter().copied().collect();
    assert_eq!(tag_set.len(), tags.len(), "a tag is listed twice");
    assert_eq!(keyed_set.len(), keyed.len(), "a tag has more than one statement");
    let missing: Vec<_> = tag_set.difference(&keyed_set).collect();
    let orphaned: Vec<_> = keyed_set.difference(&tag_set).collect();
    assert!(missing.is_empty(), "tags without a statement: {missing:?}");
    assert!(orphaned.is_empty(), "statements keying no tag: {orphaned:?}");
    for (vocabulary, tag, statement) in statement_table() {
        assert!(!statement.trim().is_empty(), "{vocabulary}.{tag}: empty statement");
    }
}

#[test]
fn the_generated_catalogue_and_statement_table_are_current() {
    let files = [("numerical_vocabulary.json", catalogue_document()), ("standing_statements.json", statement_document())];
    if std::env::var(WRITE).is_ok() {
        for (name, document) in &files {
            std::fs::write(fixture(name), text(document)).unwrap();
        }
        return;
    }
    for (name, document) in &files {
        let committed = std::fs::read_to_string(fixture(name)).unwrap_or_default();
        assert!(committed == text(document), "fixtures/reports/{name} is stale; regenerate with {WRITE}=1");
    }
}

/// The committed files agree with each other in both directions, as a reader
/// of the files alone would check them.
#[test]
fn the_committed_catalogue_and_statement_table_agree() {
    let read = |name: &str| -> Value { serde_json::from_str(&std::fs::read_to_string(fixture(name)).unwrap()).unwrap() };
    let pairs = |document: &Value, member: &str| -> BTreeSet<(String, String)> {
        document[member]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| (e["vocabulary"].as_str().unwrap().to_string(), e["tag"].as_str().unwrap().to_string()))
            .collect()
    };
    assert_eq!(pairs(&read("numerical_vocabulary.json"), "tags"), pairs(&read("standing_statements.json"), "statements"));
}

/// VER-016: a standing record carrying a tag outside the vocabulary is refused
/// when the renderer input is decoded, by name, and never read as a known tag.
#[test]
fn a_record_carrying_a_tag_outside_the_vocabulary_is_refused_by_name() {
    use open_pipe_stress_report_renderer::standing::NumericalStanding;
    let record = json!({
        "record_version": "historical_1",
        "evidence_member": "absent",
        "figures": {"status": "unavailable", "reason": "older_document"},
        "standing_findings": [],
        "counts": [{"health": "checks_passed", "cases": 1}],
        "cases": [{
            "case_id": "case:a", "health": "checks_passed", "integrity_code": "NUMERICAL_INTEGRITY_CHECKS_PASSED",
            "integrity_diagnostic_id": null, "causes": [],
            "confirmation": {"outcome": "not_applicable", "detail": null, "status": null, "premises": []},
            "supports": []
        }]
    });
    assert!(serde_json::from_value::<NumericalStanding>(record.clone()).is_ok());
    for (pointer, value, vocabulary) in [
        ("/cases/0/health", "checks_passed_enough", "case_health"),
        ("/cases/0/integrity_code", "NUMERICAL_INTEGRITY_SENSITIVE_ENOUGH", "historical_integrity_code"),
        ("/cases/0/confirmation/outcome", "confirmed", "historical_confirmation"),
        ("/standing_findings", "x", "standing_finding"),
    ] {
        let mut altered = record.clone();
        if pointer == "/standing_findings" {
            altered["standing_findings"] = json!([value]);
        } else {
            *altered.pointer_mut(pointer).unwrap() = json!(value);
        }
        let error = serde_json::from_value::<NumericalStanding>(altered).unwrap_err().to_string();
        assert!(error.contains(&format!("NUMERICAL_TAG_UNSUPPORTED: {vocabulary}")), "{pointer}: {error}");
    }
}

/// DEL-08-01 REQ-024, "Package and user release": the frame's two statements, each its
/// own line. SUPPORT-STATES-FRAME is absent without a nonlinear support, and, where supports
/// are not confirmed, states each one's reviewed reason (its support row's wording) once, in
/// the order first met: the gap family fact for a gap support, else the support row's
/// statement of the case's outcome. No case-table sentence appears in it, so nothing promises
/// a quotation the frame does not carry. KEPT-FOR-INSPECTION-FRAME
/// follows only where a load case carries the solver's recovery-basis mark, by its cause or
/// by its integrity code; a gap support without the mark does not bring it.
#[test]
fn the_frame_states_support_states_and_kept_for_inspection_as_separate_lines() {
    use open_pipe_stress_numerical_vocabulary::HistoricalCause;
    // The support rows' reviewed reasons, as `support_reason` gives them for a non-gap support.
    const ESTABLISHED: &str = "Not confirmed: the solver's confirmation reports this state established, conditional on the solver's inverse-norm step. This result does not publish the deciding quantity or its interval.";
    const NO_RECORD: &str = "Not confirmed: the result records no support-state confirmation for this load case.";
    const NOT_ESTABLISHED: &str = "Not confirmed: the solver's confirmation did not establish the support states of this load case.";
    use open_pipe_stress_report_renderer::standing::{
        cause_statement, frame_statements, kept_for_inspection_frame_statement, support_states_frame_statement, NumericalStanding,
        GAP_NOT_CONFIRMED, KEPT_FOR_INSPECTION, SUPPORT_STATES_NOT_CONFIRMED,
    };
    let case = |id: &str, outcome: &str, behaviors: &[&str], code: Option<&str>, causes: &[&str]| {
        serde_json::json!({
            "case_id": id, "health": "sensitive", "integrity_code": code, "integrity_diagnostic_id": null,
            "causes": causes.iter().map(|c| serde_json::json!({"cause": c, "diagnostic_id": null, "locations": []})).collect::<Vec<_>>(),
            "confirmation": {"outcome": outcome, "detail": null, "status": null, "premises": []},
            "supports": behaviors.iter().enumerate().map(|(i, b)| serde_json::json!({
                "support_id": format!("support:{i}"), "behavior": b, "friction_normal": null, "state": "active", "state_row_id": null
            })).collect::<Vec<_>>(),
        })
    };
    let record = |cases: Vec<Value>| -> NumericalStanding {
        serde_json::from_value(serde_json::json!({
            "record_version": "historical_1", "evidence_member": "absent",
            "figures": {"status": "unavailable", "reason": "older_document"},
            "standing_findings": [], "counts": [], "cases": cases,
        }))
        .unwrap()
    };
    let kept = format!("{KEPT_FOR_INSPECTION} {}", cause_statement(HistoricalCause::RecoveryBasisUnqualified));
    let linear = record(vec![case("load:A", "not_applicable", &[], Some("NUMERICAL_INTEGRITY_CHECKS_PASSED"), &[])]);
    assert_eq!(frame_statements(&linear), Vec::<String>::new());

    // The gap witness's form: the mark by cause and by code on each case; both lines, separately.
    // A case with a gap and a one-way support gives both supports' reasons, each once.
    let marked = record(vec![
        case("load:A", "no_confirmation_record", &["gap", "one_way"], Some("NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED"), &["recovery_basis_unqualified"]),
        case("load:B", "no_confirmation_record", &["gap"], Some("NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED"), &["recovery_basis_unqualified"]),
    ]);
    assert_eq!(frame_statements(&marked), vec![format!("{SUPPORT_STATES_NOT_CONFIRMED} {GAP_NOT_CONFIRMED} {NO_RECORD}"), kept.clone()]);

    // Nonlinear supports without the mark: the first line only.
    let unmarked = record(vec![
        case("load:A", "solver_reported_established", &["friction"], Some("NUMERICAL_INTEGRITY_SENSITIVE"), &["formation_recovery"]),
        case("load:B", "not_applicable", &[], None, &[]),
        case("load:C", "no_confirmation_record", &["one_way"], None, &[]),
        case("load:D", "solver_reported_established", &["one_way"], None, &[]),
    ]);
    assert_eq!(frame_statements(&unmarked), vec![format!("{SUPPORT_STATES_NOT_CONFIRMED} {ESTABLISHED} {NO_RECORD}")]);

    // The detailed non-establishment outcome: the support row's statement, which promises no
    // quotation; the case table's sentence, which does, never appears in a frame line.
    let detailed = record(vec![case("load:A", "not_established_detail_quoted", &["one_way", "lift_off"], None, &[])]);
    assert_eq!(frame_statements(&detailed), vec![format!("{SUPPORT_STATES_NOT_CONFIRMED} {NOT_ESTABLISHED}")]);
    // Supports under a not-applicable outcome are not confirmed, with the support rows' words.
    let not_applicable = record(vec![case("load:A", "not_applicable", &["one_way"], None, &[])]);
    assert_eq!(frame_statements(&not_applicable), vec![format!("{SUPPORT_STATES_NOT_CONFIRMED} Not confirmed.")]);
    for line in [&marked, &unmarked, &detailed, &not_applicable].into_iter().flat_map(|r| frame_statements(r)) {
        assert!(!line.contains("quoted below") && !line.contains("No nonlinear supports"), "{line}");
    }

    // A gap support is not the mark; either form of the mark alone brings the line.
    assert_eq!(kept_for_inspection_frame_statement(&record(vec![case("load:A", "no_confirmation_record", &["gap"], None, &[])])), None);
    assert_eq!(kept_for_inspection_frame_statement(&record(vec![case("load:A", "no_confirmation_record", &[], None, &["recovery_basis_unqualified"])])), Some(kept.clone()));
    assert_eq!(
        kept_for_inspection_frame_statement(&record(vec![case("load:A", "no_confirmation_record", &[], Some("NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED"), &[])])),
        Some(kept)
    );
    assert_eq!(support_states_frame_statement(&linear), None);
}
