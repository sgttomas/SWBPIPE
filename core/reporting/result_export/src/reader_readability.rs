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
/// The ordinary standing's finding for a case that is not eligible: a case that
/// is merely not checks passed, or an inconsistent case record. Which of the two
/// is decided here by [`case_records_consistent`].
const CASE_EVIDENCE_INCOMPLETE: &str = "NUMERICAL_CASE_EVIDENCE_INCOMPLETE";
const MECHANICS_NOT_SOLVED: &str = "MECHANICS_NOT_SOLVED";
/// Findings that state a readable result's numerical standing and never make it
/// unreadable.
const STANDING_FINDINGS: [&str; 7] = [
    "NUMERICAL_INTEGRITY_NOT_QUALIFIED",
    CASE_EVIDENCE_INCOMPLETE,
    "SOURCE_BLOCKS_CASE_UNQUALIFIED",
    "SOURCE_BLOCKS_PHYSICAL_ROW_INSPECTION_ONLY",
    "SOURCE_BLOCKS_ENVELOPE_UNQUALIFIED",
    crate::reader_verdict::LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE,
    crate::reader_verdict::RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE,
];
/// The producer's integrity diagnostic id of a load case
/// (product_physics `integrity_diagnostic_id`).
const INTEGRITY_ID_PREFIX: &str = "diagnostic:numerical-integrity:";

/// `{readable, solved, unreadable}` for `source`, given `standing`, the output
/// of `reader_verdict::standing` for the same source, model and invocation.
/// `unreadable` lists, in `standing`'s order, every finding that makes the
/// result unreadable (a route refusal's finding may be the reader's free text,
/// which callers quote and never read as a code), then
/// [`NUMERICAL_CASE_RECORD_INCONSISTENT`] where a case record is inconsistent.
/// `readable` is no unreadable finding; `solved` is the envelope's mechanics
/// status. Neither changes `standing`.
pub fn readability(source: &Value, standing: &Value) -> Value {
    let findings: Vec<&str> = standing["findings"].as_array().map(Vec::as_slice).unwrap_or(&[]).iter().filter_map(Value::as_str).collect();
    let mut unreadable: Vec<&str> = Vec::new();
    for finding in &findings {
        if *finding != MECHANICS_NOT_SOLVED && !STANDING_FINDINGS.contains(finding) && !unreadable.contains(finding) {
            unreadable.push(finding);
        }
    }
    if findings.contains(&CASE_EVIDENCE_INCOMPLETE) && !case_records_consistent(source) {
        unreadable.push(NUMERICAL_CASE_RECORD_INCONSISTENT);
    }
    json!({
        "readable": unreadable.is_empty(),
        "solved": source["status"]["mechanics"] == "MECHANICS_SOLVED",
        "unreadable": unreadable,
    })
}

/// Every `numerical_quality` case record that the ordinary standing does not
/// find eligible is the producer's record: its evidence names exactly the
/// case's integrity diagnostic, which the envelope carries once, and its
/// (structural status, solve quality, matrix fidelity, accuracy evidence) is
/// the producer's closed record for that diagnostic's code
/// (`nonlinear_support_evidence::standing_quality`); or, for a case the
/// producer did not assess, its evidence is empty and its record is the
/// not-assessed record. A code the producer does not publish is inconsistent.
/// An eligible case is left as the ordinary standing found it.
pub fn case_records_consistent(source: &Value) -> bool {
    let mut codes: HashMap<&str, Vec<Option<&str>>> = HashMap::new();
    for diagnostic in source["diagnostics"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        if let Some(id) = diagnostic["id"].as_str() {
            codes.entry(id).or_default().push(diagnostic["code"].as_str());
        }
    }
    let cases = source["numerical_quality"]["cases"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    cases.iter().all(|case| case_record_consistent(case, &codes))
}

fn case_record_consistent(case: &Value, codes: &HashMap<&str, Vec<Option<&str>>>) -> bool {
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
    let expected = match refs.as_slice() {
        [] => Some(["numerically_unresolved", "not_assessed", "not_assessed", "not_claimed"]),
        [only] => {
            let wanted = format!("{INTEGRITY_ID_PREFIX}{case_id}");
            if only.as_str() != Some(wanted.as_str()) {
                return false;
            }
            match codes.get(wanted.as_str()).map(Vec::as_slice) {
                Some([Some(code)]) => crate::nonlinear_support_evidence::standing_quality(code),
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
