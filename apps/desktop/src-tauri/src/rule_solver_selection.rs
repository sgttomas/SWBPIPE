//! Rule-pack format 0.5.0 run path: required load cases against the model and
//! the solved envelope, and each `solver_result` input's row on each basis
//! (W-17; DEL-06-01 Design `rule-pack-0.5.0.md` §2.5, §2.6, §2.8).
//!
//! Pure functions over the pack, the current model and the solved envelope,
//! so they are unit-tested against committed JSON fixtures. What they decide:
//!
//! - **Required load cases (§2.6).** A required load case absent from the
//!   model's `load_cases` / `combinations` is `RULE_REQUIRED_LOAD_CASE_MISSING`;
//!   present but with no envelope row on its basis (for example a withheld
//!   combination) `RULE_REQUIRED_LOAD_CASE_UNSOLVED`. Either is reported to the
//!   runner as an [`UnavailableBasis`]: every check mapped to it is
//!   `RULE_INPUTS_INCOMPLETE` there and never evaluated there.
//! - **Row selection (§2.5).** On each available basis an input binds the one
//!   row whose `basis_ref` equals the basis, whose `kind` and `entity_ref`
//!   equal the selector's, and whose `metadata.location`, `metadata.component`,
//!   section basis and tie rod equal the selector's optional members, an
//!   absent member matching only a row without it. No row:
//!   `RULE_SOLVER_RESULT_ROW_MISSING`; more than one:
//!   `RULE_SOLVER_RESULT_ROW_AMBIGUOUS`. Neither binds; neither is a pass.
//!   Rows without `basis_ref` are never selected.
//! - **Eligibility gates (§2.5, unchanged).** The run's mechanics
//!   qualification (`qualify_rule_mechanics_with_context`) is run once per run
//!   by the command before this resolution, exactly as before; it covers every
//!   row of the envelope. Each matched row then passes
//!   `semantic_contract::rule_binding_refusal`, as a row bound by id did; a
//!   refused row is recorded for that input on that basis with its reason and
//!   never bound.
//!
//! The rows are indexed once per run by (basis, kind, entity) (§2.8 "Cost").

use std::collections::{HashMap, HashSet};

use open_pipe_stress_result_export::section_basis;
use open_pipe_stress_rule_check_runner::{
    BasisRef, RefusedSolverResult, SolverResultBinding, UnavailableBasis,
};
use open_pipe_stress_rule_pack_document::{
    parse_solver_result_ref, SolverResultSelector, RULE_REQUIRED_LOAD_CASE_MISSING,
    RULE_REQUIRED_LOAD_CASE_UNSOLVED, RULE_SOLVER_RESULT_ROW_AMBIGUOUS,
    RULE_SOLVER_RESULT_ROW_MISSING,
};
use serde_json::Value;

// --- Tie rods ----------------------------------------------------------------
//
// A `connector_tie_rod_force_v1` row names its rod only in its row id. The
// result reader binds a rod row to its rod by that id
// (`core/reporting/result_export/src/connector_evidence.rs` `tie_rod_row_tail`
// and `is_tie_rod_row_id`), from the producer's convention
// (`core/product_physics/src/joint.rs` `tie_rod_row_id`). Both are private to
// their crates and their files are priced, so the convention is restated
// here, unchanged; `tie_rod_matcher_agrees_with_the_reader_binding_on_the_corpus`
// cross-checks it against every rod row of the committed connector reader
// corpus, so a drift of the producer or the reader fails that test.

/// `connector_evidence.rs` `TIE_ROD_KIND` / `joint.rs` `TIE_ROD_FORCE_KIND`.
pub(crate) const TIE_ROD_KIND: &str = "connector_tie_rod_force_v1";
/// `connector_evidence.rs` / `joint.rs` `TIE_ROD_LOCATION`.
const TIE_ROD_LOCATION: &str = "tie_rod";

/// `connector_evidence.rs` `stable_suffix`.
fn stable_suffix(id: &str) -> String {
    id.replace(':', "-")
}

/// `connector_evidence.rs` `tie_rod_row_tail`: the base row id of a rod's
/// force; a row of any load case other than the first carries it under
/// `result:loadcase:{case}:`.
fn tie_rod_row_tail(component: &str, rod: &str) -> String {
    format!(
        "connector:{}:tie-rod-force:{TIE_ROD_LOCATION}:{}:t_r",
        stable_suffix(component),
        stable_suffix(rod)
    )
}

/// `connector_evidence.rs` `is_tie_rod_row_id`.
fn is_tie_rod_row_id(id: &str, case: &str, tail: &str) -> bool {
    id.strip_prefix("result:") == Some(tail)
        || id.strip_prefix(&format!("result:loadcase:{}:", stable_suffix(case))) == Some(tail)
}

// --- Section basis -----------------------------------------------------------

/// A row's section basis, read from the `section_basis=` token of
/// `metadata.basis` with the reader's one parser
/// (`result_export::section_basis::split_token`, the producer's convention).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RowSectionBasis {
    /// `metadata.basis` names no section basis.
    Absent,
    Named(&'static str),
    /// `metadata.basis` carries a `section_basis=` token the parser refuses;
    /// such a row matches no selector (§2.5).
    Malformed,
}

pub(crate) fn row_section_basis(row: &Value) -> RowSectionBasis {
    let Some(token) = row.pointer("/metadata/basis").and_then(Value::as_str) else {
        return RowSectionBasis::Absent;
    };
    if !token.contains("section_basis=") {
        return RowSectionBasis::Absent;
    }
    match section_basis::split_token(token) {
        Some((_, basis)) => RowSectionBasis::Named(basis),
        None => RowSectionBasis::Malformed,
    }
}

fn metadata_str<'a>(row: &'a Value, key: &str) -> Option<&'a str> {
    row.get("metadata")
        .and_then(|metadata| metadata.get(key))
        .and_then(Value::as_str)
}

/// Whether `row` is the selector's row on `basis` (design §2.5). Equality is
/// exact string equality on every member; an absent optional member matches
/// only a row without it.
pub(crate) fn row_matches(selector: &SolverResultSelector, basis: &BasisRef, row: &Value) -> bool {
    if !basis.names_row(row) {
        return false;
    }
    let kind = row.get("kind").and_then(Value::as_str);
    if kind != Some(selector.result_kind.as_str())
        || row.get("entity_ref").and_then(Value::as_str) != Some(selector.entity_ref.as_str())
        || metadata_str(row, "location") != selector.location.as_deref()
        || metadata_str(row, "component") != selector.component.as_deref()
    {
        return false;
    }
    let section_matches = match row_section_basis(row) {
        RowSectionBasis::Malformed => false,
        RowSectionBasis::Absent => selector.section_basis.is_none(),
        RowSectionBasis::Named(named) => selector.section_basis.as_deref() == Some(named),
    };
    if !section_matches {
        return false;
    }
    // A tie-rod force row always carries its rod; any other row carries none.
    match (&selector.tie_rod, kind == Some(TIE_ROD_KIND)) {
        (None, rod_row) => !rod_row,
        (Some(_), false) => false,
        (Some(rod), true) => is_tie_rod_row_id(
            row.get("id").and_then(Value::as_str).unwrap_or_default(),
            &basis.ref_id,
            &tie_rod_row_tail(&selector.entity_ref, rod),
        ),
    }
}

/// The envelope's basis-bearing rows indexed once by (basis, kind, entity).
pub(crate) struct RowIndex<'a> {
    rows: HashMap<(BasisRef, &'a str, &'a str), Vec<&'a Value>>,
    solved: HashSet<BasisRef>,
}

impl<'a> RowIndex<'a> {
    pub(crate) fn new(envelope: &'a Value) -> Self {
        let mut rows: HashMap<(BasisRef, &'a str, &'a str), Vec<&'a Value>> = HashMap::new();
        let mut solved = HashSet::new();
        for row in envelope
            .get("results")
            .and_then(Value::as_array)
            .map(Vec::as_slice)
            .unwrap_or(&[])
        {
            // Rows without a well-formed basis_ref are never selected (§2.5).
            let Some(basis) = row.get("basis_ref").and_then(BasisRef::from_value) else {
                continue;
            };
            solved.insert(basis.clone());
            let kind = row.get("kind").and_then(Value::as_str).unwrap_or_default();
            let entity = row
                .get("entity_ref")
                .and_then(Value::as_str)
                .unwrap_or_default();
            rows.entry((basis, kind, entity)).or_default().push(row);
        }
        Self { rows, solved }
    }

    /// Whether the envelope publishes any row on `basis`.
    pub(crate) fn is_solved(&self, basis: &BasisRef) -> bool {
        self.solved.contains(basis)
    }

    /// Every row the selector matches on `basis`, in envelope order.
    pub(crate) fn select(
        &self,
        selector: &SolverResultSelector,
        basis: &BasisRef,
    ) -> Vec<&'a Value> {
        self.rows
            .get(&(
                basis.clone(),
                selector.result_kind.as_str(),
                selector.entity_ref.as_str(),
            ))
            .map(|rows| {
                rows.iter()
                    .copied()
                    .filter(|row| row_matches(selector, basis, row))
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// What the run binds and refuses, per input and basis (design §2.5, §2.6).
#[derive(Debug, Default)]
pub(crate) struct SolverInputResolution {
    pub(crate) solver_results: Vec<SolverResultBinding>,
    pub(crate) refused_solver_results: Vec<RefusedSolverResult>,
    pub(crate) unavailable_bases: Vec<UnavailableBasis>,
}

fn model_ids<'a>(model: &'a Value, key: &str) -> HashSet<&'a str> {
    model
        .get(key)
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.get("id").and_then(Value::as_str))
                .collect()
        })
        .unwrap_or_default()
}

fn row_id(row: &Value) -> String {
    row.get("id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

/// Resolve a validated 0.5.0 pack's required load cases and solver inputs
/// against the current model and its solved envelope (design §2.5, §2.6).
/// Inputs are resolved on every available basis of the pack; the runner uses,
/// for each outcome, only the entries of that outcome's basis.
pub(crate) fn resolve_rule_pack_solver_inputs(
    document: &Value,
    model: &Value,
    envelope: &Value,
) -> SolverInputResolution {
    let index = RowIndex::new(envelope);
    let load_cases = model_ids(model, "load_cases");
    let combinations = model_ids(model, "combinations");
    let mut resolution = SolverInputResolution::default();

    // §2.6: each required load case against the model, then the envelope.
    let mut available: Vec<BasisRef> = Vec::new();
    let mut seen: HashSet<BasisRef> = HashSet::new();
    for item in document
        .get("required_load_cases")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[])
    {
        let Some(basis) = item.get("basis_ref").and_then(BasisRef::from_value) else {
            continue;
        };
        if !seen.insert(basis.clone()) {
            continue;
        }
        let id = item
            .get("required_load_case_id")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let in_model = match basis.ref_type.as_str() {
            "load_case" => load_cases.contains(basis.ref_id.as_str()),
            _ => combinations.contains(basis.ref_id.as_str()),
        };
        let (code, message) = if !in_model {
            (
                RULE_REQUIRED_LOAD_CASE_MISSING,
                format!(
                    "required load case '{id}' names the {} '{}', which the current model does \
                     not define; the check is not evaluated on it",
                    basis.ref_type, basis.ref_id
                ),
            )
        } else if !index.is_solved(&basis) {
            (
                RULE_REQUIRED_LOAD_CASE_UNSOLVED,
                format!(
                    "required load case '{id}' names the {} '{}', which the solved result \
                     publishes no row for; the check is not evaluated on it",
                    basis.ref_type, basis.ref_id
                ),
            )
        } else {
            available.push(basis);
            continue;
        };
        resolution.unavailable_bases.push(UnavailableBasis {
            basis_ref: basis,
            required_load_case_id: id,
            code: code.to_string(),
            message,
        });
    }

    // §2.5: each solver input's one row on each available basis.
    for input in document
        .get("required_inputs")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[])
    {
        if input.get("source_kind").and_then(Value::as_str) != Some("solver_result") {
            continue;
        }
        let Some(input_id) = input
            .get("input_id")
            .and_then(Value::as_str)
            .filter(|id| !id.is_empty())
        else {
            continue;
        };
        // Validation blocks a pack whose selector is absent or malformed
        // (RULE_SOLVER_RESULT_REF_MISSING); such an input stays unsupplied.
        let Some(selector) = input
            .get("solver_result_ref")
            .and_then(|reference| parse_solver_result_ref(reference).ok())
        else {
            continue;
        };
        for basis in &available {
            let refuse = |reason: &str, message: String| RefusedSolverResult {
                input_id: input_id.to_string(),
                basis_ref: basis.clone(),
                reason: reason.to_string(),
                message: Some(message),
            };
            let rows = index.select(&selector, basis);
            let row = match rows.as_slice() {
                [row] => *row,
                [] => {
                    resolution.refused_solver_results.push(refuse(
                        RULE_SOLVER_RESULT_ROW_MISSING,
                        format!(
                            "no solved row of kind '{}' for '{}' matches this input's selector on \
                             {} '{}'; the input is treated as unsupplied",
                            selector.result_kind, selector.entity_ref, basis.ref_type, basis.ref_id
                        ),
                    ));
                    continue;
                }
                many => {
                    resolution.refused_solver_results.push(refuse(
                        RULE_SOLVER_RESULT_ROW_AMBIGUOUS,
                        format!(
                            "{} solved rows match this input's selector on {} '{}' ({}); a \
                             selector must name exactly one row, so the input is treated as \
                             unsupplied",
                            many.len(),
                            basis.ref_type,
                            basis.ref_id,
                            many.iter()
                                .map(|row| row_id(row))
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    ));
                    continue;
                }
            };
            // The per-row gate a row bound by id passed before (T0R).
            if let Some(reason) =
                open_pipe_stress_result_export::semantic_contract::rule_binding_refusal(
                    envelope, row,
                )
            {
                resolution.refused_solver_results.push(RefusedSolverResult {
                    input_id: input_id.to_string(),
                    basis_ref: basis.clone(),
                    reason: reason.to_string(),
                    message: None,
                });
                continue;
            }
            // As before: a matched row without a numeric value or a unit
            // binds nothing, so completeness blocks the check (never a pass).
            let (Some(value), Some(unit)) = (
                row.get("value").and_then(Value::as_f64),
                row.get("unit").and_then(Value::as_str),
            ) else {
                continue;
            };
            resolution.solver_results.push(SolverResultBinding {
                input_id: input_id.to_string(),
                basis_ref: basis.clone(),
                result_id: row_id(row),
                value,
                unit: unit.to_string(),
            });
        }
    }
    resolution
}

/// The selector that names `row` (test support): its kind, entity, location,
/// component, section basis and, for a rod row, the rod of its connector's
/// `tie_rods` record that the reader binds it to. `None` for a row without a
/// basis, a malformed section-basis token, or a rod row no record rod binds.
#[cfg(test)]
pub(crate) fn selector_for_row(envelope: &Value, row: &Value) -> Option<SolverResultSelector> {
    let basis = row.get("basis_ref").and_then(BasisRef::from_value)?;
    let kind = row.get("kind").and_then(Value::as_str)?.to_string();
    let entity = row.get("entity_ref").and_then(Value::as_str)?.to_string();
    let section_basis = match row_section_basis(row) {
        RowSectionBasis::Absent => None,
        RowSectionBasis::Named(named) => Some(named.to_string()),
        RowSectionBasis::Malformed => return None,
    };
    let tie_rod = if kind == TIE_ROD_KIND {
        let id = row.get("id").and_then(Value::as_str)?;
        let rod = envelope
            .pointer("/contract_evidence/connector")
            .and_then(Value::as_array)?
            .iter()
            .filter(|record| {
                record.get("component_id").and_then(Value::as_str) == Some(entity.as_str())
            })
            .flat_map(|record| {
                record
                    .get("tie_rods")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
            })
            .filter_map(|rod| rod.get("id").and_then(Value::as_str))
            .find(|rod| is_tie_rod_row_id(id, &basis.ref_id, &tie_rod_row_tail(&entity, rod)))?;
        Some(rod.to_string())
    } else {
        None
    };
    Some(SolverResultSelector {
        result_kind: kind,
        entity_ref: entity,
        location: metadata_str(row, "location").map(str::to_string),
        component: metadata_str(row, "component").map(str::to_string),
        section_basis,
        tie_rod,
    })
}

#[cfg(test)]
#[path = "rule_solver_selection_tests.rs"]
mod tests;
