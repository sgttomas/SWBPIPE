//! K-D5 on the converged active set (DEL-04-01 kd5-nonlinear-active-set.md
//! §6): the PP consumers of the independent references in
//! `validation/references/kd5_nonlinear/`, the committed demo models and the
//! named fail-closed controls.
use super::*;

const SUPPORT_SLOT: [&str; 6] = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];

struct Reference {
    request: &'static str,
    oracle: &'static str,
    /// Each load case's expected standing: `None` is Passed, `Some(reason)`
    /// the named §2.4 reason in the integrity sentence.
    expected: &'static [(&'static str, Option<&'static str>)],
}

const REFERENCES: [Reference; 7] = [
    Reference {
        request: include_str!("../../../validation/references/kd5_nonlinear/f1_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/f1_oracle.json"),
        expected: &[
            ("load:SLIDE-POS", None),
            ("load:SLIDE-NEG", None),
            ("load:STICK", None),
            // Exactly at capacity |r_t| = mu*N (R3): weakly active.
            ("load:BOUNDARY", Some("support support:NL-F: stick capacity not established")),
        ],
    },
    Reference {
        request: include_str!("../../../validation/references/kd5_nonlinear/f1_explicit_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/f1_explicit_oracle.json"),
        expected: &[("load:SLIDE", None), ("load:STICK", None)],
    },
    Reference {
        request: include_str!("../../../validation/references/kd5_nonlinear/f1_mu0_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/f1_mu0_oracle.json"),
        expected: &[("load:FREE", None)],
    },
    Reference {
        request: include_str!("../../../validation/references/kd5_nonlinear/f2_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/f2_oracle.json"),
        expected: &[("load:SLIDE", None)],
    },
    Reference {
        request: include_str!("../../../validation/references/kd5_nonlinear/u1_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/u1_oracle.json"),
        expected: &[
            ("load:PRESSED", None),
            ("load:LIFTED", None),
            // Zero reaction and zero separation (R3): weakly active.
            ("load:IN-PLANE", Some("support support:NL-U: separation not established")),
        ],
    },
    Reference {
        request: include_str!("../../../validation/references/kd5_nonlinear/u1_spring_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/u1_spring_oracle.json"),
        expected: &[("load:PRESSED", None), ("load:LIFTED", None)],
    },
    // U-1's rotational twin (review M1): the one-way support on RX, in the
    // moment family. PRESSED: Mx = 1400/9 N*m exactly; LIFTED: Inactive with
    // theta_x = 700/EI rad.
    Reference {
        request: include_str!("../../../validation/references/kd5_nonlinear/u1_rx_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/u1_rx_oracle.json"),
        expected: &[("load:PRESSED", None), ("load:LIFTED", None)],
    },
];

/// The oracle binds the immutable 0.1.0 reference bytes. Exercise those same
/// physical inputs on B7a's current public route by explicitly re-authoring only
/// the test invocation's document version; the reference files stay unchanged.
fn current_reference_request(reference: &Reference) -> serde_json::Value {
    use sha2::{Digest, Sha256};
    let oracle: serde_json::Value = serde_json::from_str(reference.oracle).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(reference.request.as_bytes())),
        oracle["request_sha256"].as_str().unwrap(),
        "the independent oracle binds the complete immutable request bytes"
    );
    let mut request: serde_json::Value = serde_json::from_str(reference.request).unwrap();
    assert_eq!(request["model"]["schema_version"], serde_json::json!("0.1.0"));
    request["model"]["schema_version"] = serde_json::json!(crate::MODEL_SCHEMA_VERSION);
    request
}

fn bound_pair(value: &serde_json::Value) -> (f64, f64) {
    (value[0].as_f64().unwrap(), value[1].as_f64().unwrap())
}

/// |v - x| <= b for some exact x in [lo, hi], decided exactly: v <= hi + b and
/// v >= lo - b, with every operand binary64 and the sums exact.
fn within_exact(v: f64, (lo, hi): (f64, f64), b: f64) -> bool {
    use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
    let sign = |a: f64, c: f64, d: f64| {
        let mut sum = ExactAccumulator::new();
        sum.add(a).and_then(|()| sum.add(c)).and_then(|()| sum.add(d)).map(|()| sum.signum())
    };
    // hi + b - v >= 0 and v - lo + b >= 0.
    matches!(sign(hi, b, -v), Ok(s) if s >= 0) && matches!(sign(v, -lo, b), Ok(s) if s >= 0)
}

fn single_case(request: &serde_json::Value, case: &str) -> serde_json::Value {
    let mut wire = request.clone();
    let cases = wire["model"]["load_cases"].as_array_mut().unwrap();
    cases.retain(|c| c["id"] == case);
    assert_eq!(cases.len(), 1);
    wire
}

/// §6: the published state, every certified member and support group within
/// its B of the exact reference (‖v‖ against the exact norm, with one unit in
/// the last place of q for the group norm's own rounding), the nonlinear slot
/// rows within B of the exact signed value, and the named reasons.
#[test]
fn kd5_nonlinear_references_bind_and_match_in_both_modes() {
    for reference in &REFERENCES {
        let oracle: serde_json::Value = serde_json::from_str(reference.oracle).unwrap();
        let request = current_reference_request(reference);
        // The actual normalized leaves the generator was given.
        let mut model: PreviewModel = serde_json::from_value(request["model"].clone()).unwrap();
        let mut materials = model.materials.clone();
        let mut diagnostics = Vec::new();
        normalize_model_units(&mut model, &mut materials, &mut diagnostics);
        let built = build_model(&model, &materials, &mut diagnostics).unwrap();
        assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
        for (pipe, frame) in built.pipes.iter().zip(&built.frame_elements) {
            let expected = &oracle["normalized_sections"][&pipe.element_id];
            for (actual, key) in [
                (frame.section.area, "As_m2"),
                (frame.section.second_moment_y, "I_m4"),
                (frame.section.second_moment_z, "I_m4"),
                (frame.section.torsion_constant, "J_m4"),
            ] {
                assert_eq!(actual.to_bits(), expected[key].as_f64().unwrap().to_bits(), "{} {key}: {actual:?}", pipe.element_id);
            }
            assert_eq!(frame.section.elastic_modulus.to_bits(), oracle["elastic_modulus_pa"].as_f64().unwrap().to_bits());
            assert_eq!(frame.section.shear_modulus.to_bits(), oracle["shear_modulus_pa"].as_f64().unwrap().to_bits());
        }
        for &(case, reason) in reference.expected {
            let exact = &oracle["cases"][case];
            for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
                let at = format!("{} {case} {mode:?}", oracle["request"]);
                tests::KD5_RECOVERY_PROBE.with(|probe| *probe.borrow_mut() = Some(Vec::new()));
                let output = run_linear_static_preview_value_with_mode(single_case(&request, case), mode).unwrap();
                let records = tests::KD5_RECOVERY_PROBE.with(|probe| probe.borrow_mut().take().unwrap());
                assert_eq!(output.status.mechanics, "MECHANICS_SOLVED", "{at}: {:?}", output.diagnostics);
                let integrity = output.diagnostics.iter().find(|d| d.id == integrity_diagnostic_id(case)).unwrap();
                if let Some(reason) = reason {
                    assert_eq!(integrity.code, "NUMERICAL_INTEGRITY_SENSITIVE", "{at}: {}", integrity.message);
                    assert!(integrity.message.contains(&format!("the converged support state is not established at 128-bit precision ({reason}); member rows are binary64 recoveries")), "{at}: {}", integrity.message);
                    assert!(records.is_empty(), "{at}: a fail-closed case publishes no certified rows");
                    // The exact reference agrees: no state qualifies strictly.
                    assert!(exact["strict_states"].as_array().unwrap().is_empty(), "{at}");
                } else {
                    assert_eq!(integrity.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{at}: {}", integrity.message);
                    assert!(integrity.message.contains(NONLINEAR_CONFIRMED_EVIDENCE_LINE), "{at}");
                    let strict = exact["strict_states"].as_array().unwrap();
                    assert_eq!(strict.len(), 1, "{at}: the reference has one strict state");
                    let state = &strict[0];
                    for (support, expected_state) in state["states"].as_object().unwrap() {
                        let code = output.results.iter().find(|r| r.kind == "nonlinear_support_active_set_state_code" && &r.entity_ref == support).unwrap().value;
                        let expected_code = match expected_state.as_str().unwrap() { "inactive" => 0.0, "active" => 1.0, "sticking" => 2.0, "sliding" => 3.0, other => panic!("{other}") };
                        assert_eq!(code, expected_code, "{at}: {support}");
                    }
                    // Every certified group within B of the exact norm.
                    let mut judged = 0;
                    for record in &records {
                        for group in &record.groups {
                            let bound = group.bound.clone().unwrap_or_else(|e| panic!("{at}: {} {e}", record.member));
                            let range = bound_pair(&state["groups"][&record.member][&group.location][&group.kind]);
                            let slack = group.q.next_up() - group.q;
                            assert!(
                                within_exact(group.q, range, bound + slack),
                                "{at}: {} {} {} q={:e} B={:e} exact={range:?}",
                                record.member, group.location, group.kind, group.q, bound
                            );
                            judged += 1;
                        }
                    }
                    assert!(judged > 0, "{at}");
                    // Nonlinear slots: signed, within B of the exact value, and the
                    // scalar row equals the published component.
                    for (support, signed) in state["signed"].as_object().unwrap() {
                        let Some(slot) = signed.get("slot") else { continue };
                        let slot = slot.as_str().unwrap();
                        let index = ["UX", "UY", "UZ", "RX", "RY", "RZ"].iter().position(|d| *d == slot).unwrap();
                        let component = output.results.iter().find(|r| r.kind == "support_reaction_component_v2" && &r.entity_ref == support && r.metadata.as_ref().is_some_and(|m| m.component == SUPPORT_SLOT[index])).unwrap().value;
                        let scalar = output.results.iter().find(|r| r.kind == "nonlinear_support_final_reaction" && &r.entity_ref == support).unwrap().value;
                        assert_eq!(component.to_bits(), scalar.to_bits(), "{at}: {support}");
                        let record = records.iter().find(|r| &r.member == support).unwrap();
                        let family = if index < 3 { "support_force" } else { "support_moment" };
                        let bound = record.groups.iter().find(|g| g.kind == family).unwrap().bound.clone().unwrap();
                        assert!(within_exact(component, bound_pair(&signed["value"]), bound), "{at}: {support} {component:e} B={bound:e} exact={:?}", signed["value"]);
                        if signed["value"][0] == 0.0 && signed["value"][1] == 0.0 {
                            assert_eq!(component.to_bits(), 0.0_f64.to_bits(), "{at}: an inactive support publishes exact +0");
                        }
                    }
                }
            }
        }
    }
}

/// §5: the 8 committed case-modes publish Passed with the evidence line, and
/// the retired nonlinear-route sentence is gone.
#[test]
fn kd5_eight_committed_nonlinear_case_modes_pass_with_the_confirmation() {
    let models = [
        include_str!("../../../fixtures/product_preview/invented_demo_model.json"),
        include_str!("../tests/fixtures/preview_physics_invented_model.json"),
    ];
    let mut checked = 0;
    for model in models {
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let model: serde_json::Value = serde_json::from_str(model).unwrap();
            let output = run_linear_static_preview_value_with_mode(serde_json::json!({"model": model}), mode).unwrap();
            assert_eq!(output.status.mechanics, "MECHANICS_SOLVED", "{:?}", output.diagnostics);
            for case in ["load:L-100", "load:L-200"] {
                let diagnostic = output.diagnostics.iter().find(|d| d.id == integrity_diagnostic_id(case)).unwrap();
                assert_eq!(diagnostic.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{}", diagnostic.message);
                assert!(diagnostic.message.contains(NONLINEAR_CONFIRMED_EVIDENCE_LINE));
                assert!(!diagnostic.message.contains("on the nonlinear-support route"));
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 8);
}

/// The manager's ruling of 2026-10-12 (kd5-nonlinear-active-set.md, "The
/// rulings"): a rotational friction support is refused by name at
/// validation; a translational one on the same request is admitted.
#[test]
fn rotational_friction_is_refused_by_name_at_validation() {
    for (dof, refused) in [("RX", true), ("RZ", true), ("UX", false)] {
        let mut request = current_reference_request(&REFERENCES[1]);
        let support = request["model"]["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "support:NL-F").unwrap();
        support["nonlinear"]["dof"] = serde_json::json!(dof);
        let output = run_linear_static_preview_value_with_mode(request, PreviewSolverMode::DenseScrutiny).unwrap();
        let refusal = output.diagnostics.iter().find(|d| d.code == "NONLINEAR_ROTATIONAL_FRICTION_UNSUPPORTED");
        assert_eq!(refusal.is_some(), refused, "{dof}: {:?}", output.diagnostics);
        if let Some(refusal) = refusal {
            assert_eq!(refusal.severity, "blocking");
            assert_eq!(refusal.id, "diagnostic:support:support:NL-F:rotational-friction");
            assert_ne!(output.status.mechanics, "MECHANICS_SOLVED");
        } else {
            assert_eq!(output.status.mechanics, "MECHANICS_SOLVED");
        }
    }
}

/// G-1, the R1 containment control (kd5-nonlinear-active-set.md §6): a
/// gap-bearing case, closed or open, keeps its recovery-basis branch in both
/// modes; no confirmation runs and no certified row is published.
#[test]
fn gap_cases_keep_their_recovery_basis_branch() {
    let mut request = current_reference_request(&REFERENCES[4]);
    let support = request["model"]["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "support:NL-U").unwrap();
    support["nonlinear"] = serde_json::json!({"behavior": "gap", "dof": "UZ", "initial_state": "inactive", "closes_when": "negative_displacement", "gap": {"value": 1e-5, "unit": "m"}});
    for case in ["load:PRESSED", "load:LIFTED"] {
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            tests::KD5_RECOVERY_PROBE.with(|probe| *probe.borrow_mut() = Some(Vec::new()));
            let output = run_linear_static_preview_value_with_mode(single_case(&request, case), mode).unwrap();
            let records = tests::KD5_RECOVERY_PROBE.with(|probe| probe.borrow_mut().take().unwrap());
            assert_eq!(output.status.mechanics, "MECHANICS_SOLVED", "{case} {mode:?}: {:?}", output.diagnostics);
            let integrity = output.diagnostics.iter().find(|d| d.id == integrity_diagnostic_id(case)).unwrap();
            assert_eq!(integrity.code, "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", "{case} {mode:?}");
            assert!(!integrity.message.contains(NONLINEAR_CONFIRMED_EVIDENCE_LINE));
            assert!(!integrity.message.contains("converged support state is not established"));
            assert!(records.is_empty());
        }
    }
}

/// T4-U5b item 6 (Codex scrutiny 4 (b)): a nonlinear case whose converged
/// support state is not established (F-1 BOUNDARY, exactly at capacity) has
/// no bound, so every nodal row is the solve's own u bit for bit; its
/// confirmed siblings (STICK, SLIDE-POS) publish the corrected displacement.
#[test]
fn a_refused_confirmation_publishes_the_solve_displacement() {
    use crate::corrected_displacement_tests::{assert_nodal_rows, corrected, with_publication_probe};
    let request = current_reference_request(&REFERENCES[0]);
    let nodes: Vec<String> = request["model"]["nodes"].as_array().unwrap().iter().map(|n| n["id"].as_str().unwrap().to_string()).collect();
    let nodes: Vec<&str> = nodes.iter().map(String::as_str).collect();
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        for (case, refused) in [("load:BOUNDARY", true), ("load:STICK", false), ("load:SLIDE-POS", false)] {
            let at = format!("{case} {mode:?}");
            let (output, records) = with_publication_probe(|| run_linear_static_preview_value_with_mode(single_case(&request, case), mode).unwrap());
            assert_eq!(output.status.mechanics, "MECHANICS_SOLVED", "{at}");
            let integrity = output.diagnostics.iter().find(|d| d.id == integrity_diagnostic_id(case)).unwrap();
            assert_eq!(integrity.message.contains("converged support state is not established"), refused, "{at}: {}", integrity.message);
            assert_eq!(records.len(), 1, "{at}");
            let record = &records[0];
            assert_eq!(record.bound.is_none(), refused, "{at}");
            let expected = if refused { record.solve.clone() } else { corrected(record) };
            assert_eq!(record.published, expected, "{at}");
            assert_nodal_rows(&output.results, &nodes, &expected, &at);
        }
    }
}
