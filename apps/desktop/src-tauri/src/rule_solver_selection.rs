//! Rule-pack format 0.5.0 run path: required load cases against the model and
//! the solved envelope, and each `solver_result` input's row on each basis
//! (W-17).
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
//!   absent member matching only a row without it. A tie-rod row's rod is the
//!   exact id its connector record holds ([`row_tie_rod`]), never decoded
//!   from the row id. No row:
//!   `RULE_SOLVER_RESULT_ROW_MISSING`; more than one:
//!   `RULE_SOLVER_RESULT_ROW_AMBIGUOUS`. Neither binds; neither is a pass.
//!   Rows without `basis_ref` are never selected, nor is a held-operand row
//!   (`metadata.basis` `held_status_operand`; DEL-05-02 `operating-case.md`
//!   §7.6): it is the held system's operand under the combination, never the
//!   combination's result, which is its difference row.
//! - **Eligibility gates (§2.5, unchanged).** The run's mechanics
//!   qualification (`qualify_rule_mechanics_with_context`) is run once per run
//!   by the command before this resolution, exactly as before; it covers every
//!   row of the envelope. Each matched row then passes
//!   `semantic_contract::rule_binding_refusal`, as a row bound by id did; a
//!   refused row is recorded for that input on that basis with its reason and
//!   never bound.
//!
//! The rows are indexed once per run by (basis, kind, entity).

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

/// The exact rod id a `connector_tie_rod_force_v1` row on `basis` belongs to:
/// the one `tie_rods[].id` of the row's connector record
/// (`contract_evidence.connector[]` whose `component_id` is the row's
/// `entity_ref`) whose encoded row id the row carries, as the reader binds it
/// (`connector_evidence.rs` `validate`). The row id encodes a rod id lossily
/// (`stable_suffix` maps `:` to `-`), so the rod is taken from the record,
/// never from the row id: `None` when the envelope has no record or more than
/// one for the connector, or when no rod or more than one of the record
/// encodes to the row's id. Such a row matches no `tie_rod` selector.
pub(crate) fn row_tie_rod<'a>(
    envelope: &'a Value,
    row: &Value,
    basis: &BasisRef,
) -> Option<&'a str> {
    let entity = row.get("entity_ref").and_then(Value::as_str)?;
    let id = row.get("id").and_then(Value::as_str)?;
    let records: Vec<&'a Value> = envelope
        .pointer("/contract_evidence/connector")
        .and_then(Value::as_array)?
        .iter()
        .filter(|record| record.get("component_id").and_then(Value::as_str) == Some(entity))
        .collect();
    let [record] = records.as_slice() else {
        return None;
    };
    let bound: Vec<&'a str> = record
        .get("tie_rods")
        .and_then(Value::as_array)?
        .iter()
        .filter_map(|rod| rod.get("id").and_then(Value::as_str))
        .filter(|rod| is_tie_rod_row_id(id, &basis.ref_id, &tie_rod_row_tail(entity, rod)))
        .collect();
    match bound.as_slice() {
        [rod] => Some(*rod),
        _ => None,
    }
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
    /// such a row matches no selector.
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

/// Whether `row` is the selector's row on `basis`. Equality is
/// exact string equality on every member; an absent optional member matches
/// only a row without it. `row_rod` is the rod the row's connector record
/// binds it to ([`row_tie_rod`]); a selector's `tie_rod` must equal it
/// exactly, so an alias that merely encodes to the same row id, or a rod the
/// record no longer holds, matches nothing.
pub(crate) fn row_matches(
    selector: &SolverResultSelector,
    basis: &BasisRef,
    row: &Value,
    row_rod: Option<&str>,
) -> bool {
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
        (Some(rod), true) => row_rod == Some(rod.as_str()),
    }
}

/// RE `preview_held_status::OPERAND_BASIS`: a held-operand row's typed basis.
const HELD_STATUS_OPERAND_BASIS: &str = "held_status_operand";

/// A held-operand row: never selected.
pub(crate) fn is_held_operand_row(row: &Value) -> bool {
    row.pointer("/metadata/basis").and_then(Value::as_str) == Some(HELD_STATUS_OPERAND_BASIS)
}

/// The envelope's basis-bearing rows indexed once by (basis, kind, entity),
/// each tie-rod row with the rod its connector record binds it to.
pub(crate) struct RowIndex<'a> {
    rows: HashMap<(BasisRef, &'a str, &'a str), Vec<(&'a Value, Option<&'a str>)>>,
    solved: HashSet<BasisRef>,
}

impl<'a> RowIndex<'a> {
    pub(crate) fn new(envelope: &'a Value) -> Self {
        let mut rows: HashMap<(BasisRef, &'a str, &'a str), Vec<(&'a Value, Option<&'a str>)>> =
            HashMap::new();
        let mut solved = HashSet::new();
        for row in envelope
            .get("results")
            .and_then(Value::as_array)
            .map(Vec::as_slice)
            .unwrap_or(&[])
        {
            // Rows without a well-formed basis_ref are never selected.
            let Some(basis) = row.get("basis_ref").and_then(BasisRef::from_value) else {
                continue;
            };
            if is_held_operand_row(row) {
                continue;
            }
            solved.insert(basis.clone());
            let kind = row.get("kind").and_then(Value::as_str).unwrap_or_default();
            let entity = row
                .get("entity_ref")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let rod = if kind == TIE_ROD_KIND {
                row_tie_rod(envelope, row, &basis)
            } else {
                None
            };
            rows.entry((basis, kind, entity))
                .or_default()
                .push((row, rod));
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
                    .filter(|(row, rod)| row_matches(selector, basis, row, *rod))
                    .map(|(row, _)| *row)
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// What the run binds and refuses, per input and basis.
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
/// against the current model and its solved envelope.
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

    // each required load case against the model, then the envelope.
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

    // each solver input's one row on each available basis.
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

/// The display facts of one matched row: what the rule-check panel shows of it.
fn preview_row(envelope: &Value, row: &Value) -> Value {
    #[cfg(test)]
    PREVIEW_ROWS_BUILT.with(|count| count.set(count.get() + 1));
    let mut shown = serde_json::json!({
        "id": row.get("id").cloned().unwrap_or(Value::Null),
        "value": row.get("value").cloned().unwrap_or(Value::Null),
        "unit": row.get("unit").cloned().unwrap_or(Value::Null),
        "dimension": row.get("dimension").cloned().unwrap_or(Value::Null),
        "binding_refusal": open_pipe_stress_result_export::semantic_contract::rule_binding_refusal(envelope, row),
    });
    if let Some(body) = row.get("component_body") {
        shown["component_body"] = serde_json::json!({
            "component_ref": body.get("component_ref").cloned().unwrap_or(Value::Null),
            "stiffness_scale": body.get("stiffness_scale").cloned().unwrap_or(Value::Null),
        });
    }
    shown
}

#[cfg(test)]
thread_local! {
    /// Test support: how many matched rows the preview has built on this thread.
    pub(crate) static PREVIEW_ROWS_BUILT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The most queries one selector preview may carry: one per solver input of a pack.
pub(crate) const MAX_PREVIEW_QUERIES: usize = 128;
/// The most bases one query may name: one per load case the input is bound on.
pub(crate) const MAX_PREVIEW_BASES_PER_QUERY: usize = 128;
/// The longest identifier a query may carry, in UTF-8 bytes: its `input_id`,
/// each selector member and each basis's `ref_id`.
pub(crate) const MAX_PREVIEW_IDENTIFIER_BYTES: usize = 256;
/// The largest arguments body the preview admits, before any prescan or parse:
/// the compact JSON of a request at every bound above, with plain identifiers
/// ([`preview_args_bound`], about 4.8 MiB). An identifier written with escapes
/// can make an otherwise admissible request longer; such a body is refused by
/// the cap (`INGRESS-ARGS-TOO-LARGE`) rather than parsed.
pub(crate) const MAX_PREVIEW_ARGS_BYTES: usize = preview_args_bound();
/// The largest selector-preview reply.
pub(crate) const MAX_PREVIEW_REPLY_BYTES: u64 = 8 * 1024 * 1024;
/// A selector-preview request outside its bounds, refused before the result is read.
pub(crate) const RULE_SOLVER_PREVIEW_REQUEST_INVALID: &str = "RULE_SOLVER_PREVIEW_REQUEST_INVALID";
/// A selector-preview reply above [`MAX_PREVIEW_REPLY_BYTES`].
pub(crate) const RULE_SOLVER_PREVIEW_RESPONSE_LIMIT: &str = "RULE_SOLVER_PREVIEW_RESPONSE_LIMIT";

/// The compact JSON length of the largest admissible request: `{"jobId":<id>,
/// "queries":[q,…]}` with 128 queries, each `{"input_id":<id>,"selector":{…},
/// "bases":[b,…]}` with every selector member present and 128 bases
/// `{"ref_type":"combination","ref_id":<id>}`, every identifier (the handle
/// included) at [`MAX_PREVIEW_IDENTIFIER_BYTES`].
const fn preview_args_bound() -> usize {
    let id = MAX_PREVIEW_IDENTIFIER_BYTES + 2; // quoted
    let basis = r#"{"ref_type":"combination","ref_id":}"#.len() + id;
    let selector = r#"{"result_kind":,"entity_ref":,"location":,"component":,"section_basis":"reduced_section","tie_rod":}"#.len() + 5 * id;
    let query = r#"{"input_id":,"selector":,"bases":[]}"#.len() + id + selector + MAX_PREVIEW_BASES_PER_QUERY * (basis + 1);
    r#"{"jobId":,"queries":[]}"#.len() + id + MAX_PREVIEW_QUERIES * (query + 1)
}

/// One selector-preview query, read and bounded: its input, its selector
/// (`None` when it does not parse: reported unreadable, matching nothing), and
/// its bases, each once.
#[derive(Debug)]
pub(crate) struct PreviewQuery {
    input_id: String,
    selector: Option<SolverResultSelector>,
    bases: Vec<BasisRef>,
}

/// Reads the preview's `queries` argument, moved, before the result is read.
/// Refused by name: not an array, more than [`MAX_PREVIEW_QUERIES`] queries, a
/// query that is not an object, an `input_id` that is not a non-empty string of
/// at most [`MAX_PREVIEW_IDENTIFIER_BYTES`], an input queried twice, a selector
/// member longer than that, `bases` not an array or longer than
/// [`MAX_PREVIEW_BASES_PER_QUERY`], a basis that [`BasisRef::from_value`] does
/// not accept or whose `ref_id` is longer than the bound, a basis that names one
/// already named in the query (by its parsed identity), and a basis object
/// carrying any member other than `ref_type` and `ref_id`. A refusal names the
/// query and basis by position and echoes no identifier.
pub(crate) fn parse_preview_queries(queries: Value) -> Result<Vec<PreviewQuery>, String> {
    let refuse = |reason: String| Err(format!("{RULE_SOLVER_PREVIEW_REQUEST_INVALID}: {reason}"));
    let Value::Array(queries) = queries else { return refuse("queries must be an array".into()) };
    if queries.len() > MAX_PREVIEW_QUERIES {
        return refuse(format!("{} queries, more than the {MAX_PREVIEW_QUERIES} one preview may carry", queries.len()));
    }
    let bounded = |text: &str| !text.is_empty() && text.len() <= MAX_PREVIEW_IDENTIFIER_BYTES;
    let mut inputs = HashSet::new();
    let mut read = Vec::with_capacity(queries.len());
    for (at, query) in queries.into_iter().enumerate() {
        let Value::Object(mut query) = query else { return refuse(format!("query {at} must be an object")) };
        let input_id = match query.remove("input_id") {
            Some(Value::String(text)) if bounded(&text) => text,
            _ => return refuse(format!("query {at}'s input_id must be a non-empty string of at most {MAX_PREVIEW_IDENTIFIER_BYTES} bytes")),
        };
        if !inputs.insert(input_id.clone()) {
            return refuse(format!("query {at} names an input an earlier query names"));
        }
        let selector_value = query.remove("selector").unwrap_or(Value::Null);
        if let Value::Object(members) = &selector_value {
            if members.values().any(|member| member.as_str().is_some_and(|text| text.len() > MAX_PREVIEW_IDENTIFIER_BYTES)) {
                return refuse(format!("query {at}'s selector has a member longer than {MAX_PREVIEW_IDENTIFIER_BYTES} bytes"));
            }
        }
        let selector = parse_solver_result_ref(&selector_value).ok();
        let Some(Value::Array(bases)) = query.remove("bases") else { return refuse(format!("query {at}'s bases must be an array")) };
        if bases.len() > MAX_PREVIEW_BASES_PER_QUERY {
            return refuse(format!("query {at} names {} bases, more than the {MAX_PREVIEW_BASES_PER_QUERY} one query may name", bases.len()));
        }
        let mut named = HashSet::new();
        let mut parsed = Vec::with_capacity(bases.len());
        for (index, basis) in bases.iter().enumerate() {
            let Some(reference) = BasisRef::from_value(basis).filter(|reference| reference.ref_id.len() <= MAX_PREVIEW_IDENTIFIER_BYTES) else {
                return refuse(format!("query {at}'s basis {index} must be {{ref_type: load_case or combination, ref_id: a non-empty string of at most {MAX_PREVIEW_IDENTIFIER_BYTES} bytes}}"));
            };
            if !named.insert(reference.clone()) {
                return refuse(format!("query {at}'s basis {index} names a basis the query already names"));
            }
            if basis.as_object().is_some_and(|members| members.len() != 2) {
                return refuse(format!("query {at}'s basis {index} must carry only ref_type and ref_id"));
            }
            parsed.push(reference);
        }
        read.push(PreviewQuery { input_id, selector, bases: parsed });
    }
    Ok(read)
}

/// Writes the preview reply's compact JSON through `sink`, piece by piece, one
/// basis entry at a time: a sink that refuses stops it there, before any further
/// row is built. Display only: for each query, which rows the selector names on
/// each basis, with the run's own index ([`RowIndex`]) and row gate
/// (`rule_binding_refusal`), and whether the result publishes any row on the
/// basis. The display distinguishes no row, one row and more than one, so
/// `matched` is 0, 1 or 2 (two or more) and `rows` holds the row only when it is
/// the one the run would bind. Nothing here binds a value.
fn emit_preview(envelope: &Value, queries: &[PreviewQuery], job_id: &str, sink: &mut dyn FnMut(&str) -> Result<(), String>) -> Result<(), String> {
    let text = |value: &Value| serde_json::to_string(value).expect("a JSON value serializes");
    let index = RowIndex::new(envelope);
    let row_count = envelope.get("results").and_then(Value::as_array).map_or(0, Vec::len);
    sink(&format!(
        r#"{{"document_kind":"swbpipe.desktop.rule_solver_rows_preview","version":1,"row_count":{row_count},"job_id":{},"inputs":["#,
        text(&Value::String(job_id.to_string()))
    ))?;
    for (at, query) in queries.iter().enumerate() {
        sink(if at == 0 { "" } else { "," })?;
        let Some(selector) = &query.selector else {
            sink(&format!(r#"{{"input_id":{},"selector_readable":false,"bases":[]}}"#, text(&Value::String(query.input_id.clone()))))?;
            continue;
        };
        sink(&format!(r#"{{"input_id":{},"selector_readable":true,"bases":["#, text(&Value::String(query.input_id.clone()))))?;
        for (number, basis) in query.bases.iter().enumerate() {
            let selected = index.select(selector, basis);
            let rows: Vec<Value> = match selected.as_slice() {
                [row] => vec![preview_row(envelope, row)],
                _ => Vec::new(),
            };
            let entry = serde_json::json!({
                "basis_ref": {"ref_type": basis.ref_type, "ref_id": basis.ref_id},
                "solved": index.is_solved(basis),
                "matched": selected.len().min(2),
                "rows": rows,
            });
            sink(if number == 0 { "" } else { "," })?;
            sink(&text(&entry))?;
        }
        sink("]}")?;
    }
    sink("]}")
}

/// The exact length of the preview reply, counted as it would be written and
/// refused by name as soon as the running count passes `limit`.
pub(crate) fn preview_reply_length(envelope: &Value, queries: &[PreviewQuery], job_id: &str, limit: u64) -> Result<u64, String> {
    let mut length = 0u64;
    emit_preview(envelope, queries, job_id, &mut |piece| {
        length += piece.len() as u64;
        if length > limit {
            return Err(format!(
                "{RULE_SOLVER_PREVIEW_RESPONSE_LIMIT}: the preview's reply passes its {limit}-byte limit; preview fewer inputs or load cases"
            ));
        }
        Ok(())
    })?;
    Ok(length)
}

/// The preview reply's text ([`emit_preview`]), written once its length is leased.
pub(crate) fn write_preview_reply(envelope: &Value, queries: &[PreviewQuery], job_id: &str) -> String {
    let mut text = String::new();
    emit_preview(envelope, queries, job_id, &mut |piece| {
        text.push_str(piece);
        Ok(())
    })
    .expect("writing to a string does not refuse");
    text
}

/// The selector that names `row` (test support): its kind, entity, location,
/// component, section basis and, for a rod row, the rod of its connector's
/// `tie_rods` record that the reader binds it to ([`row_tie_rod`]). `None`
/// for a row without a basis, a malformed section-basis token, or a rod row
/// no single record rod binds.
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
        Some(row_tie_rod(envelope, row, &basis)?.to_string())
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
