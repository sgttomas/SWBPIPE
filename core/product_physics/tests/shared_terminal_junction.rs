//! §8.30 correction 1 (RT-A3; R-4 design C-1): a shared-terminal pressure
//! junction on the exact route.
//!
//! Three pipes meet at the anchored node T with `components: []`; each is its
//! own pressure region ending at T. Each `transfers_to_wall` cap at T is
//! p*Ai along the member's outward axis -s_k, so with every terminal there
//! transferring, the caps sum to the junction load -sum(p*Ai*s_k) and the anchor
//! reaction of this closed tee is 0. A `separately_supported_or_compensated`
//! terminal at T dropped its member's share without a junction diagnostic
//! (R_T = -20,250*pi y N, about -63.6 kN, before this correction). The exact
//! route has no junction representation until R-4 stage S1, so such a node is
//! refused by name. Configurations the per-region caps already represent stay
//! admitted: every terminal at the node transferring, a two-member straight
//! continuation, and one region's closure beside unpressurized pipes.
//!
//! The request is the minimal one from the R-4 review
//! (`fixtures/shared_terminal_junction.request.json`, invented data).

use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use std::f64::consts::PI;

const REQUEST: &str = include_str!("fixtures/shared_terminal_junction.request.json");
const CODE: &str = "EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED";
const CASE: &str = "case:pressure";
const COMPENSATED: &str = "separately_supported_or_compensated";
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
/// The branch's p*Ai = 1e7 * pi * 0.045^2 = 20,250*pi N: the dropped share.
const BRANCH_CAP: f64 = 20_250.0 * PI;

fn request() -> Value {
    serde_json::from_str(REQUEST).unwrap()
}

fn regions(document: &mut Value) -> &mut Vec<Value> {
    document["model"]["load_cases"][0]["pressure_regions"]
        .as_array_mut()
        .unwrap()
}

/// Sets the closure transfer of `region`'s terminal at node:T.
fn set_transfer_at_t(document: &mut Value, region: &str, transfer: &str) {
    let region = regions(document)
        .iter_mut()
        .find(|item| item["id"] == region)
        .unwrap();
    let terminal = region["terminals"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|terminal| terminal["node_ref"] == "node:T")
        .unwrap();
    terminal["closure_transfer"] = json!(transfer);
}

/// Removes `pipe`, its far node and its region from the model.
fn remove_leg(document: &mut Value, pipe: &str, far_node: &str, region: &str) {
    document["model"]["pipe_segments"]
        .as_array_mut()
        .unwrap()
        .retain(|item| item["id"] != pipe);
    document["model"]["nodes"]
        .as_array_mut()
        .unwrap()
        .retain(|item| item["id"] != far_node);
    regions(document).retain(|item| item["id"] != region);
}

/// Keeps `pipe` in the model but takes its region out of the case (a dry leg).
fn unregion(document: &mut Value, region: &str) {
    regions(document).retain(|item| item["id"] != region);
}

fn solve(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(
        run_linear_static_preview_value_with_mode(document.clone(), mode)
            .expect("the captured entry accepts the document"),
    )
    .unwrap()
}

fn codes(envelope: &Value) -> Vec<String> {
    envelope["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| d["code"].as_str().unwrap().to_string())
        .collect()
}

fn refused(document: &Value, refs: &[&str], message: &str) {
    for mode in MODES {
        let envelope = solve(document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}");
        assert_eq!(envelope["results"], json!([]), "{mode:?}: no row is published");
        let blocking = envelope["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|d| d["severity"] == "blocking")
            .collect::<Vec<_>>();
        assert_eq!(blocking.len(), 1, "{mode:?}: {blocking:#?}");
        let finding = blocking[0];
        assert_eq!(finding["code"], CODE);
        assert_eq!(finding["affected_refs"], json!(refs));
        assert_eq!(finding["source"], "core/product_physics/src/pressure_runtime.rs");
        assert_eq!(finding["message"], message);
    }
}

/// The global force of `support`'s reaction rows, in N.
fn reaction(envelope: &Value, support: &str) -> [f64; 3] {
    ["Fx", "Fy", "Fz"].map(|component| {
        let rows = envelope["results"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| {
                row["kind"] == "support_reaction_component_v2"
                    && row["entity_ref"] == support
                    && row["metadata"]["component"] == component
            })
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), 1, "{support} {component}");
        rows[0]["value"].as_f64().unwrap()
    })
}

fn solved_with_reaction(document: &Value, expected: [f64; 3]) {
    for mode in MODES {
        let envelope = solve(document, mode);
        assert_eq!(
            envelope["status"]["mechanics"], "MECHANICS_SOLVED",
            "{mode:?}: {:#?}",
            envelope["diagnostics"]
        );
        assert!(!codes(&envelope).iter().any(|code| code == CODE), "{mode:?}");
        let actual = reaction(&envelope, "support:T");
        for axis in 0..3 {
            assert!(
                (actual[axis] - expected[axis]).abs() <= 1e-9 * BRANCH_CAP,
                "{mode:?} axis {axis}: actual {actual:?}, expected {expected:?}"
            );
        }
    }
}

#[test]
fn a_compensated_terminal_at_a_shared_tee_is_refused_by_name() {
    refused(
        &request(),
        &[CASE, "node:T", "region:C"],
        "pressure regions region:A, region:B, region:C each end at node node:T, so the node is a shared-terminal pressure junction, and separately_supported_or_compensated is declared there by region:C; 3.0.0/exact_pressure_v3 has no junction representation yet, and a compensated terminal at a junction can omit its member's share (p*Ai along the member's outward axis) of the junction's pressure load -sum(p*Ai*s) from the pipe solve, so the exact route refuses the junction rather than analyse it without that load; this temporary refusal also applies where the omitted share or the resultant is zero or cancels, such as a zero-pressure compensated branch or equal opposed compensated caps; where the members share one fluid space, transfers_to_wall at every terminal of the node carries that load",
    );
}

#[test]
fn every_compensated_terminal_at_the_junction_is_named() {
    let mut document = request();
    set_transfer_at_t(&mut document, "region:A", COMPENSATED);
    refused(
        &document,
        &[CASE, "node:T", "region:A", "region:C"],
        "pressure regions region:A, region:B, region:C each end at node node:T, so the node is a shared-terminal pressure junction, and separately_supported_or_compensated is declared there by region:A, region:C; 3.0.0/exact_pressure_v3 has no junction representation yet, and a compensated terminal at a junction can omit its member's share (p*Ai along the member's outward axis) of the junction's pressure load -sum(p*Ai*s) from the pipe solve, so the exact route refuses the junction rather than analyse it without that load; this temporary refusal also applies where the omitted share or the resultant is zero or cancels, such as a zero-pressure compensated branch or equal opposed compensated caps; where the members share one fluid space, transfers_to_wall at every terminal of the node carries that load",
    );
}

#[test]
fn a_compensated_terminal_at_a_two_region_continuation_is_refused() {
    let mut document = request();
    remove_leg(&mut document, "pipe:C", "node:C", "region:C");
    set_transfer_at_t(&mut document, "region:B", COMPENSATED);
    refused(
        &document,
        &[CASE, "node:T", "region:B"],
        "pressure regions region:A, region:B each end at node node:T, so the node is a shared-terminal pressure junction, and separately_supported_or_compensated is declared there by region:B; 3.0.0/exact_pressure_v3 has no junction representation yet, and a compensated terminal at a junction can omit its member's share (p*Ai along the member's outward axis) of the junction's pressure load -sum(p*Ai*s) from the pipe solve, so the exact route refuses the junction rather than analyse it without that load; this temporary refusal also applies where the omitted share or the resultant is zero or cancels, such as a zero-pressure compensated branch or equal opposed compensated caps; where the members share one fluid space, transfers_to_wall at every terminal of the node carries that load",
    );
}

/// Negative: the ordinary closed tee, every terminal at T transferring. The
/// caps at T sum to the junction load, so the anchor reaction is 0 (R-4 T-1).
#[test]
fn a_tee_whose_terminals_all_transfer_still_solves_with_zero_anchor_reaction() {
    let mut document = request();
    set_transfer_at_t(&mut document, "region:C", "transfers_to_wall");
    solved_with_reaction(&document, [0.0; 3]);
}

/// Negative: a two-member straight continuation of two regions, both
/// transferring at the shared node, is unchanged.
#[test]
fn a_two_region_straight_continuation_that_transfers_still_solves() {
    let mut document = request();
    remove_leg(&mut document, "pipe:C", "node:C", "region:C");
    solved_with_reaction(&document, [0.0; 3]);
}

/// Negative: one region's compensated closure at T beside two unpressurized
/// pipes is an ordinary closure, not a shared terminal, and is unchanged. Its
/// cap at T is excluded and its far cap at C pushes along +y, so the anchor
/// takes -20,250*pi y N.
#[test]
fn one_regions_closure_beside_dry_pipes_is_not_a_shared_terminal() {
    let mut document = request();
    unregion(&mut document, "region:A");
    unregion(&mut document, "region:B");
    solved_with_reaction(&document, [0.0, -BRANCH_CAP, 0.0]);
}
