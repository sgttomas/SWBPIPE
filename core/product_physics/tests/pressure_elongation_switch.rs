//! RT-L2 controls (DEL-04-07 `pressure-elongation.md` §4.3, §7 item 2): the
//! pressure-elongation switch on the exact route, through the public entry in
//! both solver modes.
//!
//! - Anchored–anchored closed straight, uniform p, no thermal: not included,
//!   reactions 0, N_el = S = 0, N_w = p·A_i; included, N_el = −(1 − 2ν)·p·A_i,
//!   N_w = 2ν·p·A_i, reactions ∓(1 − 2ν)·p·A_i. The published maxima equal
//!   |N_w|/A_s in both settings (assembled, certified and published agree; the
//!   case's checks pass).
//! - Non-transferring terminals: each support feels p·A_i when not included.
//! - A mixed operating/sustained pair on a cantilever: S does not move, and
//!   O − S is the free elongation (1 − 2ν)·p·A_i·L/(E·A_s) at the tip.
//! - The not-included straight and arc solves against the independent
//!   unbalanced-thrust model (`common/pressure_elongation.rs`); the arc's free
//!   strain evidence is exactly 0 and no beam theory is assumed.
//! - The expansion joint's assembled attachment load is p·A_e.
//! - The formulation limitations carry exactly one of the two fixed statements,
//!   immediately after the D-3 sentence.

#[path = "common/pressure_elongation.rs"]
mod pressure_elongation;
#[path = "common/current_model.rs"]
mod current_model;

use pressure_elongation::{assert_case_token, assert_off_is_the_unbalanced_thrust, solve, with_cases};
use open_pipe_stress_product_physics::PreviewSolverMode;
use serde_json::{json, Value};
use std::f64::consts::PI;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const PIPE: &str = "pipe:A-B";
const P: f64 = 2.0e6;
const E: f64 = 200e9;
const NU: f64 = 0.3;
const LENGTH: f64 = 6.0;
// OD 0.12 m, wall 0.01 m, no allowance: r_i = 0.05 m.
const AI: f64 = PI * 0.0025;
const AS: f64 = PI * 0.0011;
const D3: &str = "Not modelled: steady-flow momentum and transient pressure loads; bend opening under pressure (Bourdon effect); pressure stiffening of flexibility factors and stress intensification; ovalization.";
const NOT_INCLUDED: &str = "Pressure elongation is not included in any load case: the closed-end free pressure strain is omitted; pressure thrust at unbalanced locations is applied.";
const INCLUDED: &str = "Pressure elongation is included only in the operating load cases the model settings name, as the closed-end free strain (1 − 2ν)·p·A_i/(E·A_s), with A_i on the pressure bore and A_s the nominal section area; it is not a thin-wall mean-diameter form.";

fn terminal(node: &str, transfer: bool) -> Value {
    json!({"node_ref": node, "closure_transfer": if transfer { "transfers_to_wall" } else { "separately_supported_or_compensated" },
        "provenance": "rt_l2_control"})
}

fn anchor(node: &str) -> Value {
    json!({"id": format!("support:{}", node.trim_start_matches("node:")), "node": node, "family": "anchor",
        "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": "rt_l2_control"})
}

/// One straight A→B along x, anchored at A and (when `both`) at B, with one
/// pressurised case per id at the same p.
fn straight(both: bool, transfer: bool, cases: &[&str]) -> Value {
    let mut supports = vec![anchor("node:A")];
    if both {
        supports.push(anchor("node:B"));
    }
    let load_cases = cases.iter().map(|id| json!({"id": id, "primitive_loads": [], "pressure_regions": [{
        "id": format!("region:{}", id.trim_start_matches("case:")), "member_pipe_ids": [PIPE],
        "pressure_basis": "internal_differential_zero_external_v1", "pressure": {"value": P, "unit": "Pa"},
        "terminals": [terminal("node:A", transfer), terminal("node:B", transfer)], "provenance": "rt_l2_control"}],
        "provenance": "rt_l2_control"})).collect::<Vec<_>>();
    json!({"model": {
        "schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
        "project": {"id": "project:rt-l2", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "node:A", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": "rt_l2_control"},
            {"id": "node:B", "position": {"x": LENGTH, "y": 0.0, "z": 0.0}, "provenance": "rt_l2_control"}],
        "pipe_segments": [{"id": PIPE, "from": "node:A", "to": "node:B",
            "section": {"outside_diameter": {"value": 0.12, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"},
                "corrosion_allowance": {"value": 0.0, "unit": "m"}},
            "material": "material:one", "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": "rt_l2_control"}],
        "supports": supports, "components": [],
        "materials": [{"id": "material:one", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": {"value": E, "unit": "Pa"}, "poisson_ratio": {"value": NU, "unit": "1"},
            "provenance": "synthetic_isotropic_reference_no_catalog"}],
        "load_cases": load_cases, "combinations": []}, "materials": []})
}

fn rows<'a>(envelope: &'a Value, kind: &str, entity: &str, case: &str) -> Vec<&'a Value> {
    envelope["results"].as_array().unwrap().iter()
        .filter(|r| r["kind"] == kind && r["entity_ref"] == entity
            && (r["basis_ref"]["ref_id"] == case || r["id"].as_str().unwrap().contains(&format!(":{case}:"))))
        .collect()
}

fn close(actual: f64, expected: f64, scale: f64, what: &str) {
    assert!((actual - expected).abs() <= 1e-9 * scale, "{what}: {actual:.17e} != {expected:.17e}");
}

fn reactions(envelope: &Value, support: &str, case: &str) -> [f64; 6] {
    let mut out = [f64::NAN; 6];
    for (i, c) in ["Fx", "Fy", "Fz", "Mx", "My", "Mz"].iter().enumerate() {
        let found = rows(envelope, "support_reaction_component_v2", support, case).into_iter()
            .filter(|r| r["metadata"]["component"] == *c).collect::<Vec<_>>();
        assert_eq!(found.len(), 1, "{support} {c}");
        out[i] = found[0]["value"].as_f64().unwrap();
    }
    out
}

fn solved(envelope: &Value) {
    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{:?}", envelope["diagnostics"]);
    for case in envelope["numerical_quality"]["cases"].as_array().unwrap() {
        assert_eq!(case["solve_quality"], "checks_passed", "{case}");
    }
}

fn limitations(envelope: &Value) -> Vec<String> {
    serde_json::from_value(envelope["formulation_basis"]["limitations"].clone()).unwrap()
}

fn assert_statement(envelope: &Value, included: bool) {
    let list = limitations(envelope);
    let at = list.iter().position(|s| s == D3).expect("the D-3 sentence");
    assert_eq!(list[at + 1], if included { INCLUDED } else { NOT_INCLUDED });
    assert_eq!(list.iter().filter(|s| s.as_str() == INCLUDED || s.as_str() == NOT_INCLUDED).count(), 1);
}

/// The wall, effective and membrane rows of the member at every station, and
/// both bases' published maxima, for one case.
fn assert_member(envelope: &Value, case: &str, wall: f64, effective: f64) {
    let scale = P * AI;
    for (kind, expected) in [("pipe_wall_axial_force_v2", wall), ("pipe_effective_axial_force_v2", effective)] {
        let found = rows(envelope, kind, PIPE, case);
        assert_eq!(found.len(), 5, "{kind}");
        for r in found {
            close(r["value"].as_f64().unwrap(), expected, scale, &format!("{case} {kind} {}", r["id"]));
        }
    }
    let membrane = rows(envelope, "pipe_axial_membrane_stress_v2", PIPE, case);
    assert_eq!(membrane.len(), 10, "five stations on two bases");
    for r in membrane {
        close(r["value"].as_f64().unwrap(), wall / AS, scale / AS, &format!("{case} membrane {}", r["id"]));
    }
    let maxima = rows(envelope, "pipe_elastic_normal_stress_maximum_v2", PIPE, case);
    assert_eq!(maxima.len(), 2, "one maximum per basis");
    for r in maxima {
        close(r["value"].as_f64().unwrap(), (wall / AS).abs(), scale / AS, &format!("{case} maximum {}", r["id"]));
    }
}

#[test]
fn anchored_closed_straight_closed_forms_in_both_settings() {
    for mode in MODES {
        let off = solve(&straight(true, true, &["case:P"]), mode);
        solved(&off);
        assert_case_token(&off, "case:P", "not_included");
        assert_statement(&off, false);
        for support in ["support:A", "support:B"] {
            for value in reactions(&off, support, "case:P") {
                close(value, 0.0, P * AI * LENGTH, &format!("off {support}"));
            }
        }
        assert_member(&off, "case:P", P * AI, 0.0);

        let on = solve(&with_cases(&straight(true, true, &["case:P"]), &["case:P".into()]), mode);
        solved(&on);
        assert_case_token(&on, "case:P", "included");
        assert_statement(&on, true);
        let nel = -(1.0 - 2.0 * NU) * P * AI;
        assert_member(&on, "case:P", 2.0 * NU * P * AI, nel);
        let (a, b) = (reactions(&on, "support:A", "case:P"), reactions(&on, "support:B", "case:P"));
        close(a[0].abs(), (1.0 - 2.0 * NU) * P * AI, P * AI, "on |R_A,x|");
        close(a[0], -b[0], P * AI, "on R_A,x = -R_B,x");
        for i in 1..6 {
            close(a[i], 0.0, P * AI * LENGTH, "on R_A");
            close(b[i], 0.0, P * AI * LENGTH, "on R_B");
        }
    }
}

#[test]
fn a_non_transferring_terminal_feels_p_ai_when_not_included() {
    for mode in MODES {
        let document = straight(true, false, &["case:P"]);
        let off = assert_off_is_the_unbalanced_thrust(&document, mode, "non-transferring");
        solved(&off);
        let (a, b) = (reactions(&off, "support:A", "case:P"), reactions(&off, "support:B", "case:P"));
        close(a[0].abs(), P * AI, P * AI, "|R_A,x| = pAi");
        close(b[0], -a[0], P * AI, "R_B,x = -R_A,x");
        assert_member(&off, "case:P", P * AI, 0.0);
    }
}

#[test]
fn the_closed_straight_matches_the_unbalanced_thrust_model() {
    for mode in MODES {
        for both in [true, false] {
            assert_off_is_the_unbalanced_thrust(&straight(both, true, &["case:P"]), mode, &format!("closed, both anchored {both}"));
        }
    }
}

#[test]
fn operating_minus_sustained_is_the_elongation_response() {
    let elongation = (1.0 - 2.0 * NU) * P * AI / (E * AS) * LENGTH;
    for mode in MODES {
        let document = with_cases(&straight(false, true, &["case:O", "case:S"]), &["case:O".into()]);
        let envelope = solve(&document, mode);
        solved(&envelope);
        assert_case_token(&envelope, "case:O", "included");
        assert_case_token(&envelope, "case:S", "not_included");
        assert_statement(&envelope, true);
        for (axis, expected) in ["x", "y", "z"].into_iter().zip([elongation, 0.0, 0.0]) {
            let kind = format!("global_nodal_displacement_{axis}");
            let tip = |case: &str| {
                let found = rows(&envelope, &kind, "node:B", case);
                assert_eq!(found.len(), 1);
                assert_eq!(found[0]["unit"], "mm");
                found[0]["value"].as_f64().unwrap() / 1000.0
            };
            let (o, s) = (tip("case:O"), tip("case:S"));
            assert_eq!(s, 0.0, "the sustained case does not move: {kind}");
            close(o - s, expected, elongation, &format!("O - S {kind}"));
        }
        // A free closed pipe carries the cap in its wall in both cases.
        assert_member(&envelope, "case:O", P * AI, 0.0);
        assert_member(&envelope, "case:S", P * AI, 0.0);
    }
}

fn arc_documents() -> Vec<(String, Value)> {
    let corpus: Value = serde_json::from_str(&std::fs::read_to_string(
        current_model::project_root().join("fixtures/results/pressure_v3_arc_reader_corpus.json")).unwrap()).unwrap();
    corpus["cases"].as_array().unwrap().iter()
        .filter(|c| !pressure_elongation::pressurised_cases(&c["document"]).is_empty())
        // The corpus names its pressurised cases (today's mechanics); "not included" is the document without it.
        .map(|c| (c["label"].as_str().unwrap().to_string(), pressure_elongation::without_elongation(&c["document"])))
        .collect()
}

#[test]
fn a_realized_arc_has_no_free_pressure_strain_when_not_included() {
    let mut checked = 0;
    for (label, document) in arc_documents() {
        if label.contains("KINK") {
            continue; // its kink remainder is a thrust the p = 0 model does not carry
        }
        for mode in MODES {
            let off = assert_off_is_the_unbalanced_thrust(&document, mode, &label);
            let on = solve(&pressure_elongation::with_elongation(&document), mode);
            for case in pressure_elongation::pressurised_cases(&document) {
                assert_case_token(&off, &case, "not_included");
                assert_case_token(&on, &case, "included");
            }
            let strains = |envelope: &Value| {
                envelope["contract_evidence"]["pressure"].as_array().unwrap().iter()
                    .flat_map(|r| r["applied_loads"].as_array().unwrap().iter())
                    .filter_map(|l| l.get("arc_pressure_strain").map(|s| s.as_f64().unwrap()))
                    .collect::<Vec<_>>()
            };
            let (s_off, s_on) = (strains(&off), strains(&on));
            assert!(!s_off.is_empty() && s_off.len() == s_on.len(), "{label}");
            assert!(s_off.iter().all(|s| s.to_bits() == 0), "{label}: arc strain evidence is exactly 0");
            assert!(s_on.iter().all(|s| *s != 0.0), "{label}: the included arc carries eps_p");
            // The switch is discriminating: the included solve moves.
            let moves = |envelope: &Value| envelope["results"].as_array().unwrap().iter()
                .filter(|r| r["kind"].as_str().unwrap().starts_with("global_nodal_displacement_"))
                .map(|r| r["value"].as_f64().unwrap().abs()).fold(0.0_f64, f64::max);
            assert!(moves(&on) > 0.0, "{label}");
            solved(&off);
            solved(&on);
        }
        checked += 1;
    }
    assert_eq!(checked, 3);
}

#[test]
fn an_expansion_joint_attachment_feels_p_ae_when_not_included() {
    let request: Value = serde_json::from_str(&std::fs::read_to_string(current_model::project_root()
        .join("fixtures/reports/invented/exact_report_witness_inputs/pressure_2_connector.request.json")).unwrap()).unwrap();
    // The witness request names its pressurised case; "not included" is the request without it.
    let request = pressure_elongation::without_elongation(&request);
    let joint = &request["model"]["components"][0];
    assert_eq!(joint["kind"], "expansion_joint");
    let ae = joint["objective_connector"]["pressure_model"]["effective_area"]["value"].as_f64().unwrap();
    let p = request["model"]["load_cases"][0]["pressure_regions"][0]["pressure"]["value"].as_f64().unwrap();
    for mode in MODES {
        let off = solve(&request, mode);
        solved(&off);
        assert_case_token(&off, "load:U5A", "not_included");
        let assembly = &off["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"];
        let nodes = assembly["node_order"].as_array().unwrap();
        let rhs = assembly["assembled_pressure_rhs_global"].as_array().unwrap();
        for (index, node) in nodes.iter().enumerate() {
            let f = (0..6).map(|k| rhs[index * 6 + k].as_f64().unwrap()).collect::<Vec<_>>();
            let expected = match node.as_str().unwrap() {
                // The joint pushes its two attachments apart along its axis.
                "node:B" => -p * ae,
                "node:C" => p * ae,
                _ => 0.0,
            };
            close(f[0], expected, p * ae, &format!("{node} Fx"));
            for k in 1..6 {
                close(f[k], 0.0, p * ae, &format!("{node} component {k}"));
            }
        }
    }
}
