//! DEL-05-02 Standing: whether the reader can read a result at all, a second
//! verdict beside `reader_verdict::standing`'s unchanged `eligible`.
//!
//! A result that is not numerically eligible can still be read. Its numerical
//! standing (sensitive, quality unresolved, not assessed, or the pressure-source
//! qualification findings) is stated by whatever presents it and is never a
//! ground for refusing it. A result is unreadable where the reader cannot read
//! it with its model: an unsupported or retired route, the route's evidence or
//! model checks refusing, ambiguous ids, a requested basis unavailable,
//! incomplete case coverage, an aggregate contradicting its cases, or a case
//! record inconsistent with the producer's closed record for its integrity
//! code. Whether the mechanics are solved is reported separately (`solved`).
//!
//! The verdict is computed from `standing`'s own output and the source, so the
//! standing is read once and `standing` itself is unchanged.
use serde_json::{json, Value};
use std::collections::HashMap;

/// A case record that is not the producer's closed record for the integrity
/// code its evidence names, or whose evidence does not name exactly the case's
/// integrity diagnostic.
pub const NUMERICAL_CASE_RECORD_INCONSISTENT: &str = "NUMERICAL_CASE_RECORD_INCONSISTENT";
/// A load case's state of one nonlinear support published in more than one row,
/// with equal or different row ids: `NUMERICAL_SUPPORT_STATE_DUPLICATED: <load
/// case> <support>`. The producer publishes one row per load case and support,
/// so the document is malformed: it names no one state, and its states are not
/// read as not published.
pub const NUMERICAL_SUPPORT_STATE_DUPLICATED: &str = "NUMERICAL_SUPPORT_STATE_DUPLICATED";
/// A diagnostic id published more than once, identical or not:
/// `NUMERICAL_DIAGNOSTIC_ID_DUPLICATED: <id>`. The document names no one
/// diagnostic for that id, which is not the same as recording none.
pub const NUMERICAL_DIAGNOSTIC_ID_DUPLICATED: &str = "NUMERICAL_DIAGNOSTIC_ID_DUPLICATED";
/// The result row kind of a nonlinear support's computed state.
pub(crate) const STATE_CODE_KIND: &str = "nonlinear_support_active_set_state_code";
/// The ordinary standing's finding for a case that is not eligible: a case that
/// is merely not checks passed, or an inconsistent case record. Which of the two
/// is decided here by [`case_records_consistent`].
const CASE_EVIDENCE_INCOMPLETE: &str = "NUMERICAL_CASE_EVIDENCE_INCOMPLETE";
const MECHANICS_NOT_SOLVED: &str = "MECHANICS_NOT_SOLVED";
/// Findings that state a readable result's numerical standing and never make it
/// unreadable: the closed `standing_finding` vocabulary.
fn standing_finding(finding: &str) -> bool {
    open_pipe_stress_numerical_vocabulary::StandingFinding::parse(finding).is_some()
}
/// The producer's integrity diagnostic id of a load case
/// (product_physics `integrity_diagnostic_id`).
const INTEGRITY_ID_PREFIX: &str = "diagnostic:numerical-integrity:";
/// A blocked document's retained integrity evaluation (product_physics
/// `INSPECTION_ONLY_CODE`, DEL-04-07 *Blocked document*).
const INSPECTION_ONLY: &str = "NUMERICAL_INTEGRITY_INSPECTION_ONLY";
/// The producer's record for a case it did not assess, or whose retained
/// evaluation is inspection-only.
const NOT_ASSESSED_RECORD: [&str; 4] = ["numerically_unresolved", "not_assessed", "not_assessed", "not_claimed"];

/// The reader's notion of a document's mechanics status, shared by
/// [`readability`]'s `solved` and [`case_records_consistent`] so that the two
/// always agree: `Some(true)` solved, `Some(false)` positively blocked, `None`
/// a status that is missing or malformed, which is neither.
fn mechanics_solved(source: &Value) -> Option<bool> {
    match source["status"]["mechanics"].as_str() {
        Some("MECHANICS_SOLVED") => Some(true),
        Some("MODEL_INCOMPLETE") => Some(false),
        _ => None,
    }
}

/// `{readable, solved, unreadable}` for `source`, given `standing`, the output
/// of `reader_verdict::standing` for the same source, model and invocation.
/// `unreadable` lists, in `standing`'s order, every finding that makes the
/// result unreadable (a route refusal's finding may be the reader's free text,
/// which callers quote and never read as a code), then
/// [`NUMERICAL_CASE_RECORD_INCONSISTENT`] where a case record is inconsistent,
/// then each named duplicate ([`duplicated_evidence`]).
/// `readable` is no unreadable finding; `solved` is the envelope's mechanics
/// status. Neither changes `standing`.
pub fn readability(source: &Value, standing: &Value) -> Value {
    let findings: Vec<&str> = standing["findings"].as_array().map(Vec::as_slice).unwrap_or(&[]).iter().filter_map(Value::as_str).collect();
    let mut unreadable: Vec<String> = Vec::new();
    for finding in &findings {
        if *finding != MECHANICS_NOT_SOLVED && !standing_finding(finding) && !unreadable.iter().any(|f| f == finding) {
            unreadable.push(finding.to_string());
        }
    }
    if findings.contains(&CASE_EVIDENCE_INCOMPLETE) && !case_records_consistent(source) {
        unreadable.push(NUMERICAL_CASE_RECORD_INCONSISTENT.to_string());
    }
    unreadable.extend(duplicated_evidence(source));
    json!({
        "readable": unreadable.is_empty(),
        "solved": mechanics_solved(source) == Some(true),
        "unreadable": unreadable,
    })
}

/// Every duplicate the historical reader cannot read, each once, in document
/// order: each diagnostic id published more than once
/// ([`NUMERICAL_DIAGNOSTIC_ID_DUPLICATED`]), then each (load case, support)
/// whose state is published in more than one row
/// ([`NUMERICAL_SUPPORT_STATE_DUPLICATED`]). Empty for a document without one;
/// genuinely absent evidence is not a duplicate and keeps its own reading.
pub fn duplicated_evidence(source: &Value) -> Vec<String> {
    let mut out = Vec::new();
    let mut ids = std::collections::HashSet::new();
    // An empty or absent id is the ordinary reader's NUMERICAL_EVIDENCE_ID_AMBIGUOUS.
    for id in source["diagnostics"].as_array().map(Vec::as_slice).unwrap_or(&[]).iter().filter_map(|d| d["id"].as_str().filter(|id| !id.is_empty())) {
        let finding = format!("{NUMERICAL_DIAGNOSTIC_ID_DUPLICATED}: {id}");
        if !ids.insert(id) && !out.contains(&finding) {
            out.push(finding);
        }
    }
    let mut states = std::collections::HashSet::new();
    for row in source["results"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        if row["kind"] != STATE_CODE_KIND || row["basis_ref"]["ref_type"] != "load_case" {
            continue;
        }
        let key = (row["basis_ref"]["ref_id"].as_str().unwrap_or(""), row["entity_ref"].as_str().unwrap_or(""));
        let finding = format!("{NUMERICAL_SUPPORT_STATE_DUPLICATED}: {} {}", key.0, key.1);
        if !states.insert(key) && !out.contains(&finding) {
            out.push(finding);
        }
    }
    out
}

/// Every `numerical_quality` case record that the ordinary standing does not
/// find eligible is the producer's record: its evidence names exactly the
/// case's integrity diagnostic, which the envelope carries once, and its
/// (structural status, solve quality, matrix fidelity, accuracy evidence) is
/// the producer's closed record for that diagnostic's code
/// ([`standing_quality`]); or, for a case the
/// producer did not assess, its evidence is empty and its record is the
/// not-assessed record. The inspection-only code's meaning depends on the
/// document's state (DEL-04-07 *Blocked document*): in a positively blocked
/// document (`MODEL_INCOMPLETE`) it is a retained evaluation that qualifies no
/// result, and its case's record is the not-assessed record with that
/// diagnostic as its one evidence reference; in a solved document, or one whose
/// mechanics status is missing or malformed, it is inconsistent. A code the
/// producer does not publish is inconsistent. An eligible case is left as the
/// ordinary standing found it.
pub fn case_records_consistent(source: &Value) -> bool {
    let blocked = mechanics_solved(source) == Some(false);
    let mut codes: HashMap<&str, Vec<Option<&str>>> = HashMap::new();
    for diagnostic in source["diagnostics"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        if let Some(id) = diagnostic["id"].as_str() {
            codes.entry(id).or_default().push(diagnostic["code"].as_str());
        }
    }
    let cases = source["numerical_quality"]["cases"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    cases.iter().all(|case| case_record_consistent(case, &codes, blocked))
}

fn case_record_consistent(case: &Value, codes: &HashMap<&str, Vec<Option<&str>>>, blocked: bool) -> bool {
    let record = [&case["structural_status"], &case["solve_quality"], &case["model_matrix_fidelity"], &case["accuracy_evidence"]];
    let eligible = record[0] == "passive_model_basis"
        && record[1] == "checks_passed"
        && record[2] == "represented_equations_retained"
        && matches!(record[3].as_str(), Some("not_claimed" | "reference_verified"));
    if eligible {
        return true;
    }
    let Some(case_id) = case["basis_ref"]["ref_id"].as_str().filter(|_| case["basis_ref"]["ref_type"] == "load_case") else {
        return false;
    };
    let Some(refs) = case["evidence_refs"].as_array() else {
        return false;
    };
    let wanted = format!("{INTEGRITY_ID_PREFIX}{case_id}");
    let expected = match refs.as_slice() {
        // The producer gives the not-assessed record only to a case with no
        // integrity diagnostic (product_physics `assessed_numerical_quality`).
        [] if codes.contains_key(wanted.as_str()) => None,
        [] => Some(NOT_ASSESSED_RECORD),
        [only] => {
            if only.as_str() != Some(wanted.as_str()) {
                return false;
            }
            match codes.get(wanted.as_str()).map(Vec::as_slice) {
                // Supported only in a positively blocked document, where it
                // names a retained evaluation and no verdict; it has no solved
                // record, so it is not in STANDING_QUALITY.
                Some([Some(INSPECTION_ONLY)]) if blocked => Some(NOT_ASSESSED_RECORD),
                Some([Some(code)]) => standing_quality(code),
                _ => None,
            }
        }
        _ => None,
    };
    match expected {
        Some(expected) => record.iter().zip(expected).all(|(field, want)| *field == want),
        None => false,
    }
}

/// The producer's closed record for each integrity code it publishes in a
/// solved document: (structural_status, solve_quality, model_matrix_fidelity,
/// accuracy_evidence), as product_physics `assessed_numerical_quality`
/// assigns them and as `result_export::nonlinear_support_evidence` reads them
/// for a nonlinear case (`NUMERICAL_INTEGRITY_UNRESOLVED` is the producer's
/// default arm). The inspection-only code, which the producer publishes only in
/// a blocked document, has no record here; `case_record_consistent` reads it.
/// `None`: a code with no solved record. Held here, not
/// borrowed from result_export, so that the priced reader is unchanged; the
/// tests compare it, entry for entry, with the priced reader's table.
pub const STANDING_QUALITY: [(&str, [&str; 4]); 8] = [
    ("NUMERICAL_INTEGRITY_CHECKS_PASSED", ["passive_model_basis", "checks_passed", "represented_equations_retained", "not_claimed"]),
    ("NUMERICAL_INTEGRITY_SENSITIVE", ["passive_model_basis", "sensitive", "represented_equations_retained", "not_claimed"]),
    ("NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM", ["physical_mechanism_witnessed", "failed", "not_assessed", "not_claimed"]),
    ("NUMERICAL_INTEGRITY_NEGATIVE_ENERGY", ["negative_energy_witnessed", "failed", "not_assessed", "not_claimed"]),
    ("NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", ["numerically_unresolved", "unresolved", "not_assessed", "unresolved"]),
    ("NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED", ["numerically_unresolved", "unresolved", "assembly_loss_detected", "unresolved"]),
    ("NUMERICAL_INTEGRITY_FAILED", ["numerically_unresolved", "failed", "assembly_uncertainty", "unresolved"]),
    ("NUMERICAL_INTEGRITY_UNRESOLVED", ["numerically_unresolved", "unresolved", "assembly_uncertainty", "unresolved"]),
];

/// The record [`STANDING_QUALITY`] holds for `code`; `None`: a code with no
/// solved record.
pub fn standing_quality(code: &str) -> Option<[&'static str; 4]> {
    STANDING_QUALITY.iter().find(|(known, _)| *known == code).map(|(_, record)| *record)
}
