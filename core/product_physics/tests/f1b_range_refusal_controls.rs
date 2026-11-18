//! Controls for the refusal of a static load case whose ordinary attempt
//! meets a binary64 range failure (DEL-04-07 scope, the range-failure bullet
//! under "Method choices that define the solve"; release limitation
//! LIM-030), observed through the product's public entries:
//!
//! - a load term that is not an authored nodal load, the seventh family in
//!   `force_scaling_admission`'s order, which `f1b_w2_runtime.rs` does not
//!   observe at product level (the admission unit tests in `src/f1b_tests.rs`
//!   name it from the admission function directly);
//! - a model with a nonlinear support, where force scaling is never attempted;
//! - a model with a skewed restraint, refused separately;
//! - the evaluation trigger on the simplest model.
//!
//! Each expectation comes from the source's order (trigger, then the skew and
//! nonlinear exclusions, then admission) and from DEL-04-07's blocked-document
//! definition (mechanics status `MODEL_INCOMPLETE`, no result rows), with the
//! refusal templates of `append_force_scaling_refusal`, `append_integrity_failure`
//! and `skew::force_scaled_refusal`; no force-scale exponent is asserted.
//!
//! Every input is invented.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    LinearStaticPreviewRequest, MechanicsEnvelope, PreviewSolverMode,
};
use serde_json::{json, Value};

const PROV: &str = "invented_range_refusal_control_input_no_library_data";
const ALL: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];

fn anchor(node: &str, restraints: &[&str]) -> Value {
    json!({"id": format!("anchor:{node}"), "node": node, "family": "anchor", "restraints": restraints,
           "provenance": PROV})
}

/// The product's result for `request` through both public entries.
fn envelopes(request: &Value, mode: PreviewSolverMode) -> Vec<(&'static str, MechanicsEnvelope)> {
    let typed: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    vec![
        (
            "captured",
            run_linear_static_preview_value_with_mode(request.clone(), mode)
                .expect("the captured entry admits the request"),
        ),
        ("typed", run_linear_static_preview_with_mode(typed, mode)),
    ]
}

/// The whole document is the blocked one with zero result rows, and its one
/// blocking diagnostic is `case`'s integrity record refusing `family` under
/// force scaling.
fn assert_family_refusal(request: &Value, case: &str, family: &str) {
    for mode in MODES {
        for (entry, envelope) in envelopes(request, mode) {
            let ctx = format!("{family}: {entry} {mode:?}");
            assert_eq!(
                envelope.status.mechanics, "MODEL_INCOMPLETE",
                "{ctx}: {:?}",
                envelope.diagnostics
            );
            assert!(
                envelope.results.is_empty(),
                "{ctx}: {} rows",
                envelope.results.len()
            );
            let blocking: Vec<_> = envelope
                .diagnostics
                .iter()
                .filter(|d| d.severity == "blocking")
                .collect();
            assert_eq!(blocking.len(), 1, "{ctx}: {:?}", envelope.diagnostics);
            let record = blocking[0];
            assert_eq!(
                record.id,
                format!("diagnostic:numerical-integrity:{case}"),
                "{ctx}: {}",
                record.message
            );
            assert_eq!(record.code, "NUMERICAL_INTEGRITY_UNRESOLVED", "{ctx}");
            assert_eq!(record.affected_refs, vec![case.to_string()], "{ctx}");
            let prefix = format!(
                "Load case {case}: range: family not admitted under force scaling: {family}; range_scaling: attempted; step1_trigger="
            );
            assert!(
                record.message.starts_with(&prefix),
                "{ctx}: {}",
                record.message
            );
            assert!(
                record.message.contains("; force_scale_exponent="),
                "{ctx}: {}",
                record.message
            );
        }
    }
}

/// K2a's exact-zero member (`f1b_w2_runtime.rs` `exact_zero`: N0 anchored, N1
/// free in UY only; its 12EIy/L^3 formation leaves the normal range) with a
/// ground spring at N1's UY whose base the case moves by 1 mm: the case's only
/// action is the spring's base motion, a formed load k*g whose source is the
/// spring support, not an authored nodal load.
fn spring_base_range_request() -> Value {
    let restrained: Vec<&str> = ALL.iter().copied().filter(|d| *d != "UY").collect();
    json!({"model": {
        "schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "project": {"id": "invented:range-refusal:spring-base",
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa",
                              "temperature": "degC", "stress": "Pa"}},
        "nodes": [
            {"id": "N0", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV},
            {"id": "N1", "position": {"x": 1.8189894035458565e-12, "y": 0.0, "z": 0.0}, "provenance": PROV}
        ],
        "pipe_segments": [{"id": "M1", "from": "N0", "to": "N1", "material": "mat:R",
            "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
            "section": {"outside_diameter": {"value": 1.0e-11, "unit": "m"},
                        "wall_thickness": {"value": 1.0e-12, "unit": "m"}},
            "provenance": PROV}],
        "materials": [{"id": "mat:R", "elastic_modulus": {"value": 6.4e-280, "unit": "Pa"},
                       "shear_modulus": {"value": 1.0e-100, "unit": "Pa"}, "provenance": PROV}],
        "supports": [
            anchor("N0", &ALL),
            anchor("N1", &restrained),
            {"id": "spring:N1:UY", "node": "N1", "family": "spring", "restraints": ["UY"],
             "stiffness": {"dof": "UY", "value": {"value": 1.0e-280, "unit": "N/m"}}, "provenance": PROV}
        ],
        "load_cases": [{"id": "case", "label": "spring base motion", "kind": "primitive_user_load",
            "primitive_loads": [{"id": "load:base", "category": "imposed_displacement",
                "target": {"type": "support", "support": "spring:N1:UY", "dof": "UY"},
                "direction": "UY", "magnitude": {"value": 0.001, "unit": "m"},
                "dimension": "displacement", "provenance": PROV}],
            "provenance": PROV}],
        "combinations": []
    }, "materials": []})
}

/// Admission family 7 of 8: a range-triggered case whose only load term is a
/// moved spring base's formed load (no element load, no connector, bend,
/// thermal, pressure or constant-effort support before it) is refused naming
/// the non-nodal load term, and the whole document is blocked with no rows.
#[test]
fn range_refusal_names_a_non_nodal_load_term() {
    assert_family_refusal(&spring_base_range_request(), "case", "non_nodal_load_term");
}

fn pow2(exponent: i32) -> f64 {
    assert!((-1022..=1023).contains(&exponent));
    f64::from_bits(((exponent + 1023) as u64) << 52)
}

/// The whole document is the blocked one with zero result rows; its blocking
/// diagnostics are exactly `expected` (code, affected case), and no force
/// scaling was attempted (no `range_scaling` text anywhere).
fn assert_blocked_without_force_scaling(request: &Value, expected: &[(&str, &str)]) {
    for mode in MODES {
        for (entry, envelope) in envelopes(request, mode) {
            let ctx = format!("{entry} {mode:?}");
            assert_eq!(
                envelope.status.mechanics, "MODEL_INCOMPLETE",
                "{ctx}: {:?}",
                envelope.diagnostics
            );
            assert!(
                envelope.results.is_empty(),
                "{ctx}: {} rows",
                envelope.results.len()
            );
            let blocking: Vec<_> = envelope
                .diagnostics
                .iter()
                .filter(|d| d.severity == "blocking")
                .map(|d| (d.code.as_str(), d.affected_refs.clone()))
                .collect();
            let wanted: Vec<_> = expected
                .iter()
                .map(|(code, case)| (*code, vec![case.to_string()]))
                .collect();
            assert_eq!(blocking, wanted, "{ctx}: {:?}", envelope.diagnostics);
            assert!(
                envelope
                    .diagnostics
                    .iter()
                    .all(|d| !d.message.contains("range_scaling")),
                "{ctx}: {:?}",
                envelope.diagnostics
            );
        }
    }
}

/// A 1 m cantilever a-b (OD 0.12 m, wall 0.01 m) with a 100 N tip force in Y
/// and moduli times `modulus_scale` (the `f1b_w2_runtime.rs`
/// `cantilever_family` construction: at 2^-1000 the ordinary evaluation's
/// products leave binary64's normal range while every input is normal), with
/// `extra` supports at b and `loads` added to the case.
fn cantilever(modulus_scale: f64, extra: Vec<Value>, loads: Vec<Value>) -> Value {
    let mut supports = vec![anchor("node:a", &ALL)];
    supports.extend(extra);
    let mut primitive = vec![json!({"id": "f", "category": "concentrated_force",
        "target": {"type": "node", "node": "node:b"}, "direction": "global_y",
        "magnitude": {"value": 100.0, "unit": "N"}, "dimension": "force", "provenance": PROV})];
    primitive.extend(loads);
    json!({"model": {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "invented:range-refusal:cantilever", "units": {"length": "m", "force": "N",
            "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
            "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "node:a", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV},
                  {"id": "node:b", "position": {"x": 1.0, "y": 0.0, "z": 0.0}, "provenance": PROV}],
        "pipe_segments": [{"id": "pipe:a-b", "from": "node:a", "to": "node:b",
            "section": {"outside_diameter": {"value": 0.12, "unit": "m"},
                        "wall_thickness": {"value": 0.01, "unit": "m"}},
            "material": "material:m", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV}],
        "supports": supports, "components": [],
        "materials": [{"id": "material:m", "elastic_modulus": {"value": 200e9 * modulus_scale, "unit": "Pa"},
            "shear_modulus": {"value": 76.923076923e9 * modulus_scale, "unit": "Pa"}, "provenance": PROV}],
        "load_cases": [{"id": "case:f", "primitive_loads": primitive, "provenance": PROV}],
        "combinations": []}, "materials": []})
}

/// A model with a nonlinear support never reaches force scaling: on a range
/// failure of the case's ordinary attempt in the certified evaluation, the
/// trigger is dropped (`built.nonlinear_supports` is not empty), the failure
/// is not a contact-seed failure, and the case's integrity diagnostic is the
/// ordinary one, `NUMERICAL_INTEGRITY_UNRESOLVED` (a range error has no other
/// integrity code). The document is the blocked one with zero rows. (The
/// formation arm, `SOLVER_SYSTEM_BLOCKED` before any case, is observed by
/// `k2a_formation_range_runtime.rs`.)
#[test]
fn range_failure_on_a_nonlinear_model_is_not_force_scaled() {
    let gap = json!({"id": "support:gap", "node": "node:b", "family": "nonlinear", "restraints": [],
        "nonlinear": {"behavior": "gap", "dof": "UZ", "initial_state": "inactive",
                      "closes_when": "positive_displacement", "gap": {"value": 1000.0, "unit": "mm"}},
        "provenance": PROV});
    let request = cantilever(pow2(-1000), vec![gap], vec![]);
    assert_blocked_without_force_scaling(&request, &[("NUMERICAL_INTEGRITY_UNRESOLVED", "case:f")]);
    for mode in MODES {
        for (entry, envelope) in envelopes(&request, mode) {
            let record = envelope
                .diagnostics
                .iter()
                .find(|d| d.id == "diagnostic:numerical-integrity:case:f")
                .unwrap_or_else(|| panic!("{entry} {mode:?}: {:?}", envelope.diagnostics));
            assert_eq!(record.severity, "blocking");
        }
    }
}

/// A linear model with a skewed restraint is refused separately on a range
/// failure: `SUPPORT_DIRECTION_SKEW_FORCE_SCALED_UNSUPPORTED` for the case,
/// before any force scaling, and the document is the blocked one with zero
/// rows. The model is `skew_restraints_references.rs`'s (a line stop along
/// (3, 4, 0) at the tip of a 4 m member) with its tip load of 1e-307 N.
#[test]
fn range_failure_on_a_skew_model_is_refused_without_force_scaling() {
    let qv = |value: f64, unit: &str| json!({"value": value, "unit": unit});
    let request = json!({"model": {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "invented:range-refusal:skew", "units": {"length": "m", "force": "N", "angle": "rad",
            "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
            "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "A", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV},
                  {"id": "B", "position": {"x": 4.0, "y": 0.0, "z": 0.0}, "provenance": PROV}],
        "pipe_segments": [{"id": "pipe:AB", "from": "A", "to": "B",
            "section": {"outside_diameter": qv(0.206, "m"), "wall_thickness": qv(0.011, "m")},
            "material": "material:skew", "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": PROV}],
        "supports": [
            anchor("A", &ALL),
            {"id": "rot_B", "node": "B", "family": "anchor", "restraints": ["RX", "RY", "RZ"], "provenance": PROV},
            {"id": "stop_B", "node": "B", "family": "line_stop", "restraints": ["UY"],
             "direction": {"x": 3.0, "y": 4.0, "z": 0.0}, "provenance": PROV}
        ],
        "components": [],
        "materials": [{"id": "material:skew", "elastic_modulus": qv(171_798_691_840.0, "Pa"),
            "shear_modulus": qv(68_719_476_736.0, "Pa"), "provenance": PROV}],
        "load_cases": [{"id": "case:skew", "primitive_loads": [{"id": "load:y", "category": "concentrated_force",
            "target": {"type": "node", "node": "B"}, "direction": "global_y", "magnitude": qv(-1.0e-307, "N"),
            "dimension": "force", "provenance": PROV}], "provenance": PROV}],
        "combinations": []}, "materials": []});
    assert_blocked_without_force_scaling(
        &request,
        &[(
            "SUPPORT_DIRECTION_SKEW_FORCE_SCALED_UNSUPPORTED",
            "case:skew",
        )],
    );
}

/// The evaluation trigger on the simplest model: the 1 m cantilever (OD 0.12
/// m, wall 0.01 m, 100 N tip force) with a 50 N/m distributed load, and its
/// moduli times 2^-996 (E about 3.0e-289 Pa, every input normal binary64).
/// The ordinary attempt's certified evaluation leaves the normal range, force
/// scaling is attempted, and admission names the distributed load
/// (`uniform_element_load`, the first excluded family the case carries): the
/// whole document is blocked with no rows. (`f1b_w2_runtime.rs`
/// `cantilever_family("uniform_element_load", 2^-1000)` is the same
/// construction; its formation-trigger counterpart there is the exact-zero
/// member of the `zero_nodal_load_term` row.)
#[test]
fn range_evaluation_witness_on_a_cantilever_with_a_distributed_load() {
    let distributed = json!({"id": "w", "category": "distributed_force",
        "target": {"type": "element", "pipe": "pipe:a-b"}, "direction": "global_z",
        "magnitude": {"value": -50.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": PROV});
    let request = cantilever(pow2(-996), vec![], vec![distributed]);
    assert_family_refusal(&request, "case:f", "uniform_element_load");
    for mode in MODES {
        for (entry, envelope) in envelopes(&request, mode) {
            let record = envelope
                .diagnostics
                .iter()
                .find(|d| d.id == "diagnostic:numerical-integrity:case:f")
                .unwrap();
            assert!(
                record.message.contains("step1_trigger=Evaluation("),
                "{entry} {mode:?}: {}",
                record.message
            );
        }
    }
}
