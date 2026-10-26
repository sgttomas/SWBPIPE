//! The native render's identity boundary (W-11, DEL-08-01 Design
//! `exact-route-report-sections.md` §3.6 step 0 and §5.5).
//!
//! Owner ruling 10A keeps exact-route report outputs refused until the owner
//! opens them. The report package enforces this at its Rust identity boundary
//! (item 0); the native render command enforces it here. Step 0 runs first for
//! every render input, on the argument value that `command_ingress::admit`
//! already priced and decoded, before any further parse, custody query, lease
//! growth or render, and it allocates nothing before it decides.
//!
//! Step 0 recognizes an exact-route input only by structured markers at
//! defined paths, never by free text such as a load-case label.
//!
//! A [`RenderAdmittedSet`] names the exact identities the render admits. Its
//! field is private to this module, so no other code can construct one. In a
//! non-test build [`PRODUCTION`], which wraps the empty
//! [`RENDER_ADMITTED_EXACT_CONTRACTS`], is the only value there is. The widened
//! `EXACT_RENDER_WITNESS_ADMITTED` exists only in this crate's test build, for
//! `report_render_exact_tests.rs`. The set reaches step 0 only along the graph
//! that `report_render_exact_tests::render_admitted_set_forwarding_graph_is_closed`
//! verifies. Slice (a) adds no custody binding, resubmitted source or
//! re-derivation: an input step 0 admits (only the witness set admits any)
//! goes straight to the renderer. Standard claim fence applies (claims
//! taxonomy per DEC-081).

use crate::command_ingress::{Args, CommandError};
use open_pipe_stress_result_export::semantic_contract::{PHYSICS_RETAINED_ID, PRESSURE_ID, PRESSURE_SOURCE_ID};
use serde_json::Value;
use std::fmt::Write as _;

/// The exact identities whose `exact_route` the native render admits: none
/// until the owner opens exact-route report outputs (ruling 10A). Opening adds
/// `PRESSURE_ID` and `PRESSURE_SOURCE_ID` (§5.1).
pub(crate) const RENDER_ADMITTED_EXACT_CONTRACTS: [&str; 0] = [];

/// The exact-route identities step 0 recognizes at the input's identity
/// fields.
pub(crate) const EXACT_IDENTITIES: [&str; 3] = [PRESSURE_ID, PRESSURE_SOURCE_ID, PHYSICS_RETAINED_ID];

pub(crate) const RENDER_ROUTE_NOT_ADMITTED: &str = "REPORT-RENDER-ROUTE-NOT-ADMITTED";

/// The exact identities step 0 admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RenderAdmittedSet(&'static [&'static str]);

impl RenderAdmittedSet {
    fn contains(&self, id: &str) -> bool {
        self.0.contains(&id)
    }
}

/// The production set: every build's only render set outside this crate's
/// tests.
pub(crate) const PRODUCTION: RenderAdmittedSet = RenderAdmittedSet(&RENDER_ADMITTED_EXACT_CONTRACTS);

/// The two identities the direct-native render tests admit (test build only).
#[cfg(test)]
pub(crate) const EXACT_RENDER_WITNESS_ADMITTED: RenderAdmittedSet = RenderAdmittedSet(&[PRESSURE_ID, PRESSURE_SOURCE_ID]);

/// The render command's path after ingress: step 0 against `admitted`, then
/// the renderer.
pub(crate) fn render_calculation_report_args_in(args: &mut Args, admitted: RenderAdmittedSet) -> Result<Value, CommandError> {
    let input = args.value("input")?;
    check_render_route_admitted(&input, admitted).map_err(CommandError::Command)?;
    crate::render_calculation_report_value(input).map_err(CommandError::Command)
}

/// Step 0 (§3.6). It reads only structured markers at the input's defined
/// paths, never free text (labels, notes, statements, descriptions,
/// provenance or any other user-supplied value). An input is an exact-route
/// input when it carries:
/// - a non-null `exact_route`;
/// - an exact identity at an identity field: `exact_route.semantic_contract_id`,
///   `exact_route.admission.semantic_contract_id`, or the `ref_id` of a
///   `report_sections.limitations[].source` whose `ref_type` is
///   `semantic_contract`;
/// - an exact limitation entry: a `report_sections.limitations[].limitation_id`
///   `limitation:…:exact:…` (the renderer's predicate).
///
/// Such an input passes only when its `exact_route` names an identity in
/// `admitted` and every exact identity at its identity fields is in
/// `admitted`. An input with no marker renders as before: it has no
/// `exact_route`, so the renderer emits no exact block, standing line or
/// headlines, and it has no exact limitation entry. Free text equal to an
/// exact identity is therefore not route evidence. The walk reads the decoded
/// value in place and allocates nothing; only a refusal formats its message.
pub(crate) fn check_render_route_admitted(input: &Value, admitted: RenderAdmittedSet) -> Result<(), String> {
    let route = input.get("exact_route").filter(|route| !route.is_null());
    let Some(route) = route else {
        if let Some(id) = first_named(input, &|_| true) {
            return Err(refusal(admitted, &format_args!("{id}")));
        }
        if let Some(id) = first_exact_limitation(input) {
            return Err(refusal(admitted, &format_args!("the exact-route limitation entry {id} without an exact_route section")));
        }
        return Ok(());
    };
    match route.get("semantic_contract_id").and_then(Value::as_str) {
        Some(id) if admitted.contains(id) => {}
        Some(id) => return Err(refusal(admitted, &format_args!("{id}"))),
        None => return Err(refusal(admitted, &format_args!("an exact_route section that names no identity"))),
    }
    match first_named(input, &|id| !admitted.contains(id)) {
        Some(id) => Err(refusal(admitted, &format_args!("{id}"))),
        None => Ok(()),
    }
}

fn refusal(admitted: RenderAdmittedSet, found: &dyn std::fmt::Display) -> String {
    let mut message = format!("{RENDER_ROUTE_NOT_ADMITTED}: the rendered report admits ");
    if admitted.0.is_empty() {
        message.push_str("no exact-route identity");
    } else {
        message.push_str("only the exact-route identities ");
        message.push_str(&admitted.0.join(", "));
    }
    let _ = write!(message, "; found {found}.");
    message
}

/// The first exact identity at one of `input`'s identity fields, in the order
/// `exact_route.semantic_contract_id`, `exact_route.admission.semantic_contract_id`,
/// then each `semantic_contract` limitation source, for which `counts` holds.
fn first_named<'a>(input: &'a Value, counts: &dyn Fn(&str) -> bool) -> Option<&'a str> {
    let route = input.get("exact_route");
    let route_identities = [
        route.and_then(|route| route.get("semantic_contract_id")),
        route.and_then(|route| route.get("admission")).and_then(|admission| admission.get("semantic_contract_id")),
    ];
    let limitation_sources = limitations(input)
        .filter_map(|limitation| limitation.get("source"))
        .filter(|source| source.get("ref_type").and_then(Value::as_str) == Some("semantic_contract"))
        .map(|source| source.get("ref_id"));
    route_identities
        .into_iter()
        .chain(limitation_sources)
        .filter_map(|field| field.and_then(Value::as_str))
        .find(|id| EXACT_IDENTITIES.contains(id) && counts(id))
}

/// The entries of `report_sections.limitations`.
fn limitations(input: &Value) -> impl Iterator<Item = &Value> {
    // `get`, not `pointer`: a pointer lookup allocates its unescaped tokens.
    input
        .get("report_sections")
        .and_then(|sections| sections.get("limitations"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
}

/// The first exact-route limitation entry id in `report_sections.limitations`.
fn first_exact_limitation(input: &Value) -> Option<&str> {
    limitations(input)
        .filter_map(|limitation| limitation.get("limitation_id").and_then(Value::as_str))
        .find(|id| id.starts_with("limitation:") && id.contains(":exact:"))
}
