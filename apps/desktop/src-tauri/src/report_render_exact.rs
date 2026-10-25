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

/// The exact-route identities step 0 recognizes wherever the input names one.
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

/// Step 0 (§3.6). An input is an exact-route input when it carries a non-null
/// `exact_route`, names an exact identity in any string, or carries an
/// exact-route limitation entry (an id `limitation:…:exact:…`, the renderer's
/// predicate). Such an input passes only when its `exact_route` names an
/// identity in `admitted` and every exact identity it names is in `admitted`.
/// The walk reads the decoded value in place and allocates nothing; only a
/// refusal formats its message.
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

/// The first exact identity that `value` names, as a string or an object key,
/// for which `counts` holds. The decoded value's depth is bounded by the
/// ingress parse's recursion limit.
fn first_named<'a>(value: &'a Value, counts: &dyn Fn(&str) -> bool) -> Option<&'a str> {
    let named = |text: &'a str| EXACT_IDENTITIES.iter().any(|id| *id == text).then_some(text).filter(|id| counts(id));
    match value {
        Value::String(text) => named(text),
        Value::Array(items) => items.iter().find_map(|item| first_named(item, counts)),
        Value::Object(map) => map.iter().find_map(|(key, item)| named(key).or_else(|| first_named(item, counts))),
        _ => None,
    }
}

/// The first exact-route limitation entry id in `report_sections.limitations`.
fn first_exact_limitation(input: &Value) -> Option<&str> {
    // `get`, not `pointer`: a pointer lookup allocates its unescaped tokens.
    input
        .get("report_sections")
        .and_then(|sections| sections.get("limitations"))
        .and_then(Value::as_array)?
        .iter()
        .filter_map(|limitation| limitation.get("limitation_id").and_then(Value::as_str))
        .find(|id| id.starts_with("limitation:") && id.contains(":exact:"))
}
