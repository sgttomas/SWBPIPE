//! RT-A3 S6 (DEL-04-07 Design `route-unification-r4.md` §3.7): the PP
//! consumers of the independent references in `validation/references/rt_a3_s6/`
//! (exact rational, generated without product code), in both solver modes.
//!
//! - LS-1: W-16 stage 2's A-1, D-3 and C-5 as exact-route load/reference-state
//!   twins (Euler–Bernoulli, and Timoshenko for D-3 and C-5). A resolved case's
//!   rigid `boundary_motion` (a mixed record's rigid restraint included), an
//!   active nonlinear support's own-DOF `boundary_motion` (its moved ground)
//!   and an active spring's `base_motion` reach the active-set loop, its K-D5
//!   confirmation and the ledger with W-16 stage 2's laws. Every strict
//!   dropped-motion mutant is run too: it matches its own exact reference and
//!   the reference case is outside its criterion.
//! - C-1: a pressurized objective joint between anchors with a one-way guide;
//!   the no-nonlinear, tie-rod, Timoshenko, gap and load/reference-state
//!   (moved anchor) controls.
//! - The readers: each envelope passes the pressure reader with the model; the
//!   moved-ground and spring-base evidence and the relative rows are bound,
//!   and named forgeries are refused.
//!
//! Criterion (the packet's): each published value within
//! 1e-9·|expected|, or 1e-9 of the case's family zero scale when the expected
//! value is 0. The exact route forms its section constants by D-5's sequence,
//! a last-place difference from the packet's leaves.
use super::*;
use serde_json::{json, Value};

const REFERENCES: &str = include_str!("../../../validation/references/rt_a3_s6/references.json");
const SUMS: &str = include_str!("../../../validation/references/rt_a3_s6/SHA256SUMS");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const DOF_TOKENS: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
const COMPONENTS: [&str; 6] = ["ux", "uy", "uz", "rx", "ry", "rz"];
const SLOTS: [&str; 6] = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];

const REQUESTS: [(&str, &str); 11] = [
    ("ls1_a1", include_str!("../../../validation/references/rt_a3_s6/requests/ls1_a1.json")),
    ("ls1_d3", include_str!("../../../validation/references/rt_a3_s6/requests/ls1_d3.json")),
    ("ls1_c5", include_str!("../../../validation/references/rt_a3_s6/requests/ls1_c5.json")),
    ("ls1_d3_timoshenko", include_str!("../../../validation/references/rt_a3_s6/requests/ls1_d3_timoshenko.json")),
    ("ls1_c5_timoshenko", include_str!("../../../validation/references/rt_a3_s6/requests/ls1_c5_timoshenko.json")),
    ("c1", include_str!("../../../validation/references/rt_a3_s6/requests/c1.json")),
    ("c1_linear", include_str!("../../../validation/references/rt_a3_s6/requests/c1_linear.json")),
    ("c1_tied", include_str!("../../../validation/references/rt_a3_s6/requests/c1_tied.json")),
    ("c1_timoshenko", include_str!("../../../validation/references/rt_a3_s6/requests/c1_timoshenko.json")),
    ("c1_gap", include_str!("../../../validation/references/rt_a3_s6/requests/c1_gap.json")),
    ("c1_ls", include_str!("../../../validation/references/rt_a3_s6/requests/c1_ls.json")),
];

fn references() -> Value {
    serde_json::from_str(REFERENCES).unwrap()
}

fn sha256(text: &str) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(text.as_bytes()))
}

/// The request, bound to the oracle and to SHA256SUMS by its bytes.
fn bound(name: &str) -> (Value, Value) {
    let text = REQUESTS.iter().find(|(n, _)| *n == name).unwrap().1;
    let oracle = references()["requests"][name].clone();
    let digest = sha256(text);
    assert_eq!(digest, oracle["request_sha256"].as_str().unwrap(), "{name}: the oracle binds the request bytes");
    let path = format!("  {}", oracle["request"].as_str().unwrap());
    let line = SUMS.lines().find(|l| l.ends_with(&path)).unwrap_or_else(|| panic!("SHA256SUMS lists {path}"));
    assert_eq!(&line[..64], digest);
    (serde_json::from_str(text).unwrap(), oracle)
}

fn single_case(request: &Value, case: &str) -> Value {
    let mut wire = request.clone();
    let cases = wire["model"]["load_cases"].as_array_mut().unwrap();
    cases.retain(|c| c["id"] == case);
    assert_eq!(cases.len(), 1, "{case}");
    wire
}

fn exact(v: &Value) -> f64 {
    v["value"].as_f64().unwrap()
}

fn solve(request: &Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()
}

/// The pressure reader with the model, `None` when it admits the envelope.
fn reader(envelope: &Value, request: &Value) -> Option<String> {
    let found = result_export::reader_verdict::validate("pressure", envelope, Some(&request["model"]), None);
    (found["ok"] != true).then(|| found["error"].to_string())
}

fn within(observed: f64, expected: f64, zero_scale: f64) -> bool {
    observed.is_finite() && (observed - expected).abs() <= 1e-9 * if expected == 0.0 { zero_scale } else { expected.abs() }
}

/// Every published nodal component and every referenced support component
/// against the record: the number of failures (0 = the record matches) and
/// the worst relative error.
fn compare(output: &MechanicsEnvelope, record: &Value, at: &str, report: bool) -> (usize, f64) {
    let scales = &record["scales"];
    let zero = [exact(&scales["translation_m"]), exact(&scales["rotation_rad"]), exact(&scales["force_n"]), exact(&scales["moment_nm"])];
    let (mut failures, mut worst) = (0, 0.0_f64);
    let mut judge = |observed: f64, expected: f64, family: usize, what: String| {
        let ok = within(observed, expected, zero[family]);
        let denominator = if expected == 0.0 { zero[family] } else { expected.abs() };
        if denominator > 0.0 {
            worst = worst.max((observed - expected).abs() / denominator);
        }
        if !ok {
            failures += 1;
            if report {
                eprintln!("{at}: {what} observed {observed:e} expected {expected:e}");
            }
        }
    };
    for (node, comps) in record["displacements"].as_object().unwrap() {
        for (k, c) in comps.as_array().unwrap().iter().enumerate() {
            let id = format!("result:disp:{}:{}", stable_suffix(node), COMPONENTS[k]);
            let row = output.results.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("{at}: {id}"));
            let observed = row.value / if k < 3 { 1000.0 } else { 1.0 };
            judge(observed, exact(c), if k < 3 { 0 } else { 1 }, id);
        }
    }
    for (support, entry) in record["supports"].as_object().unwrap() {
        for (k, c) in entry["components"].as_array().unwrap().iter().enumerate() {
            let row = output.results.iter().find(|r| {
                r.kind == "support_reaction_component_v2" && &r.entity_ref == support && r.metadata.as_ref().is_some_and(|m| m.component == SLOTS[k])
            });
            let row = row.unwrap_or_else(|| panic!("{at}: {support} {}", SLOTS[k]));
            judge(row.value, exact(c), if k < 3 { 2 } else { 3 }, format!("{support} {}", SLOTS[k]));
        }
    }
    (failures, worst)
}

fn integrity<'a>(output: &'a MechanicsEnvelope, case: &str) -> &'a Diagnostic {
    output.diagnostics.iter().find(|d| d.id == integrity_diagnostic_id(case)).unwrap_or_else(|| panic!("{case}: integrity"))
}

fn state_code(output: &MechanicsEnvelope, support: &str) -> f64 {
    let rows = output.results.iter().filter(|r| r.kind == "nonlinear_support_active_set_state_code" && r.entity_ref == support).collect::<Vec<_>>();
    assert_eq!(rows.len(), 1, "{support}");
    rows[0].value
}

fn code_of(state: &str) -> f64 {
    match state {
        "inactive" => 0.0,
        "active" => 1.0,
        other => panic!("{other}"),
    }
}

/// The request with one motion entry `support|field|DOF` removed (the
/// dropped-motion mutant); an emptied field is removed whole.
fn drop_motion(request: &Value, case: &str, key: &str) -> Value {
    let parts = key.split('|').collect::<Vec<_>>();
    let (support, field, dof) = (parts[0], parts[1], parts[2]);
    let mut mutant = request.clone();
    let state = mutant["model"]["load_cases"].as_array_mut().unwrap().iter_mut().find(|c| c["id"] == case).unwrap();
    let entry = state["analysis_state"]["support_states"].as_array_mut().unwrap().iter_mut().find(|s| s["support_ref"] == support).unwrap();
    let list = entry[field].as_array_mut().unwrap();
    let before = list.len();
    list.retain(|m| m["dof"] != dof);
    assert_eq!(list.len() + 1, before, "{key}");
    if list.is_empty() {
        entry.as_object_mut().unwrap().remove(field);
    }
    mutant
}

/// One LS-1 case in one mode: standing, states, displacements and support
/// actions, the relative rows, the motion evidence and the reader.
fn run_ls(name: &str, request: &Value, record: &Value, case: &str, mode: PreviewSolverMode) -> MechanicsEnvelope {
    let at = format!("{name} {case} {mode:?}");
    let wire = single_case(request, case);
    let output = solve(&wire, mode);
    assert_eq!(output.status.mechanics, "MECHANICS_SOLVED", "{at}: {:#?}", output.diagnostics);
    let found = integrity(&output, case);
    assert_eq!(found.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{at}: {}", found.message);
    assert!(found.message.contains(NONLINEAR_CONFIRMED_EVIDENCE_LINE), "{at}: {}", found.message);
    for (support, state) in record["states"].as_object().unwrap() {
        assert_eq!(state_code(&output, support), code_of(state.as_str().unwrap()), "{at}: {support}");
    }
    let (failures, worst) = compare(&output, record, &at, true);
    assert_eq!(failures, 0, "{at}");
    // R-b: one relative row per moved ground, fl(U - g_s), near its exact value.
    let relative = output.results.iter().filter(|r| r.kind == "nonlinear_support_final_relative_displacement").collect::<Vec<_>>();
    let grounds = ground_components(&output, case);
    assert_eq!(relative.len(), grounds.len(), "{at}");
    for (support, dof, g) in &grounds {
        let row = relative.iter().find(|r| &r.entity_ref == support).unwrap();
        let k = DOF_TOKENS.iter().position(|d| d == dof).unwrap();
        let expected = exact(&record["relative_displacement"][support.as_str()]);
        let observed = row.value / if k < 3 { 1000.0 } else { 1.0 };
        let zero = exact(&record["scales"][if k < 3 { "translation_m" } else { "rotation_rad" }]);
        assert!(within(observed, expected, zero), "{at}: {support} relative {observed:e} exact {expected:e} (g {g:e})");
    }
    assert_eq!(found.message.contains(NONLINEAR_RELATIVE_DISPLACEMENT_CLAUSE), !grounds.is_empty(), "{at}");
    let envelope = serde_json::to_value(&output).unwrap();
    assert_eq!(reader(&envelope, &wire), None, "{at}");
    println!("RT-A3-S6 {at}: max relative error {worst:.3e}");
    output
}

/// The case's moved-ground evidence: (support, DOF, g) from its load/reference-state record.
fn ground_components(output: &MechanicsEnvelope, case: &str) -> Vec<(String, String, f64)> {
    let records = output.contract_evidence.as_ref().unwrap()["load_reference_states"].as_array().unwrap();
    let record = records.iter().find(|r| r["load_case_id"] == case).unwrap();
    record["support_components"].as_array().unwrap().iter()
        .filter(|c| c["law_kind"] == "nonlinear_ground")
        .map(|c| (c["support_id"].as_str().unwrap().to_string(), c["dof"].as_str().unwrap().to_string(), c["prescribed_value"].as_f64().unwrap()))
        .collect()
}

#[test]
fn the_references_are_the_frozen_bytes_and_every_cross_check_agrees() {
    let line = SUMS.lines().find(|l| l.ends_with("  references.json")).unwrap();
    assert_eq!(&line[..64], sha256(REFERENCES));
    let checks = references()["crosscheck"].as_array().unwrap().clone();
    assert!(checks.len() > 800 && checks.iter().all(|c| c["agrees"] == true));
    for (name, _) in REQUESTS {
        bound(name);
    }
}

/// LS-1, both modes: every twin case matches its exact reference.
#[test]
fn ls1_twins_match_in_both_modes() {
    for name in ["ls1_a1", "ls1_d3", "ls1_c5", "ls1_d3_timoshenko", "ls1_c5_timoshenko"] {
        let (request, oracle) = bound(name);
        for (case, record) in oracle["cases"].as_object().unwrap() {
            for mode in MODES {
                run_ls(name, &request, record, case, mode);
            }
        }
    }
}

/// LS-1: each strict dropped-motion mutant matches its own exact reference,
/// and the reference case lies outside the mutant's criterion (and the mutant
/// outside the reference's): every rigid, nonlinear-ground and spring-base
/// motion is numerically consumed. A weak-boundary mutant is not run.
#[test]
fn ls1_dropped_motion_mutants_are_numerically_distinguished() {
    let mut run = 0;
    for name in ["ls1_a1", "ls1_d3", "ls1_c5", "ls1_d3_timoshenko", "ls1_c5_timoshenko"] {
        let (request, oracle) = bound(name);
        for (case, record) in oracle["cases"].as_object().unwrap() {
            let mode = PreviewSolverMode::SparseInteractive;
            let reference = solve(&single_case(&request, case), mode);
            for (key, mutant) in record["dropped_motion_mutants"].as_object().unwrap() {
                let at = format!("{name} {case} drop {key}");
                if mutant["weak_boundary"] == true {
                    continue;
                }
                let (failures, _) = compare(&reference, mutant, &at, false);
                assert!(failures > 0, "{at}: the reference survives the mutant's criterion");
                for mode in MODES {
                    let wire = single_case(&drop_motion(&request, case, key), case);
                    let output = solve(&wire, mode);
                    assert_eq!(output.status.mechanics, "MECHANICS_SOLVED", "{at}: {:#?}", output.diagnostics);
                    let (failures, worst) = compare(&output, mutant, &at, true);
                    assert_eq!(failures, 0, "{at} {mode:?}");
                    let (against_reference, _) = compare(&output, record, &at, false);
                    assert!(against_reference > 0, "{at} {mode:?}: the mutant survives the reference's criterion");
                    for (support, state) in mutant["states"].as_object().unwrap() {
                        assert_eq!(state_code(&output, support), code_of(state.as_str().unwrap()), "{at} {mode:?}: {support}");
                    }
                    println!("RT-A3-S6 {at} {mode:?}: mutant matches its reference (max relative error {worst:.3e})");
                }
                run += 1;
            }
        }
    }
    assert!(run >= 20, "{run}");
}

/// The load/reference-state evidence names each motion by its law and source.
#[test]
fn ls1_evidence_names_each_motion_law() {
    let (request, _) = bound("ls1_c5");
    let output = solve(&single_case(&request, "load:UNEQUAL"), PreviewSolverMode::DenseScrutiny);
    let records = output.contract_evidence.as_ref().unwrap()["load_reference_states"].as_array().unwrap().clone();
    let components = records[0]["support_components"].as_array().unwrap();
    let laws = components.iter().map(|c| (c["support_id"].as_str().unwrap(), c["law_kind"].as_str().unwrap(), c["physical_state_source"].as_str().unwrap())).collect::<Vec<_>>();
    assert_eq!(laws, [("support:LIFT", "nonlinear_ground", "support_state.boundary_motion"), ("support:K", "spring_base", "support_state.base_motion")]);
    let contributions = records[0]["contributions"].as_array().unwrap();
    for (id, class) in [("support_state:support:LIFT:UZ", "nonlinear_ground_motion"), ("support_state:support:K:UZ", "spring_base_motion")] {
        assert!(contributions.iter().any(|c| c["source_id"] == id && c["classification"] == class), "{id}");
    }
    let (request, _) = bound("ls1_d3");
    let output = solve(&single_case(&request, "load:ACTIVE"), PreviewSolverMode::DenseScrutiny);
    let records = output.contract_evidence.as_ref().unwrap()["load_reference_states"].as_array().unwrap().clone();
    let laws = records[0]["support_components"].as_array().unwrap().iter().map(|c| (c["dof"].as_str().unwrap(), c["law_kind"].as_str().unwrap())).collect::<Vec<_>>();
    // D-3's record: its own RY ground (R-b) and its rigid UZ (R-d).
    assert_eq!(laws, [("RY", "nonlinear_ground"), ("UZ", "rigid_prescribed")]);
}

/// Reader negatives on a moved-ground and moved-spring-base envelope.
#[test]
fn the_reader_refuses_forged_or_missing_motion_evidence() {
    let (request, _) = bound("ls1_c5");
    let wire = single_case(&request, "load:UNEQUAL");
    let envelope = serde_json::to_value(solve(&wire, PreviewSolverMode::SparseInteractive)).unwrap();
    assert_eq!(reader(&envelope, &wire), None);
    let components = "/contract_evidence/load_reference_states/0/support_components";
    let ground = envelope.pointer(components).unwrap().as_array().unwrap().iter().position(|c| c["law_kind"] == "nonlinear_ground").unwrap();
    let spring = envelope.pointer(components).unwrap().as_array().unwrap().iter().position(|c| c["law_kind"] == "spring_base").unwrap();
    let relative = envelope["results"].as_array().unwrap().iter().position(|r| r["kind"] == "nonlinear_support_final_relative_displacement").unwrap();
    let mutate = |edit: &dyn Fn(&mut Value)| {
        let mut forged = envelope.clone();
        edit(&mut forged);
        reader(&forged, &wire).expect("the reader refuses the forgery")
    };
    // A moved ground altered in the evidence (and its contribution with it).
    let found = mutate(&|e| {
        let g = e.pointer(&format!("{components}/{ground}/prescribed_value")).unwrap().as_f64().unwrap();
        *e.pointer_mut(&format!("{components}/{ground}/prescribed_value")).unwrap() = json!(g * 2.0);
        for c in e.pointer_mut("/contract_evidence/load_reference_states/0/contributions").unwrap().as_array_mut().unwrap() {
            if c["source_id"] == "support_state:support:LIFT:UZ" {
                c["value"] = json!(g * 2.0);
            }
        }
    });
    assert!(found.contains("NONLINEAR_RELATIVE_DISPLACEMENT"), "{found}");
    // A moved ground restated as a rigid prescribed value.
    let found = mutate(&|e| *e.pointer_mut(&format!("{components}/{ground}/law_kind")).unwrap() = json!("rigid_prescribed"));
    assert!(found.contains("SUPPORT_CONTRIBUTION_BINDING"), "{found}");
    // A spring base claimed from `boundary_motion`.
    let found = mutate(&|e| *e.pointer_mut(&format!("{components}/{spring}/physical_state_source")).unwrap() = json!("support_state.boundary_motion"));
    assert!(found.contains("SUPPORT_COMPONENT_LAW"), "{found}");
    // The relative row removed, altered, or attributed to another case's evidence.
    let found = mutate(&|e| {
        e["results"].as_array_mut().unwrap().remove(relative);
    });
    assert!(found.contains("NONLINEAR_RELATIVE_DISPLACEMENT"), "{found}");
    let found = mutate(&|e| {
        let v = e["results"][relative]["value"].as_f64().unwrap();
        e["results"][relative]["value"] = json!(v + 0.5);
    });
    assert!(found.contains("NONLINEAR_RELATIVE_DISPLACEMENT"), "{found}");
    let found = mutate(&|e| e["contract_evidence"]["load_reference_states"][0]["load_case_id"] = json!("load:OTHER"));
    assert!(found.contains("LOAD_REFERENCE"), "{found}");
    // The ground evidence removed (with its contribution): the relative row is unbound.
    let found = mutate(&|e| {
        e.pointer_mut(components).unwrap().as_array_mut().unwrap().remove(ground);
        e.pointer_mut("/contract_evidence/load_reference_states/0/contributions").unwrap().as_array_mut().unwrap()
            .retain(|c| c["source_id"] != "support_state:support:LIFT:UZ");
    });
    assert!(found.contains("NONLINEAR_RELATIVE_DISPLACEMENT"), "{found}");
}

/// One C-1 case in one mode.
fn run_c1(name: &str, request: &Value, record: &Value, case: &str, mode: PreviewSolverMode) -> MechanicsEnvelope {
    let at = format!("{name} {case} {mode:?}");
    let wire = single_case(request, case);
    let output = solve(&wire, mode);
    assert_eq!(output.status.mechanics, "MECHANICS_SOLVED", "{at}: {:#?}", output.diagnostics);
    let found = integrity(&output, case);
    let gap = name == "c1_gap";
    if gap {
        // R-3 unbuilt: a gap with a connector is unqualified, never confirmed.
        assert_eq!(found.code, "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", "{at}: {}", found.message);
        assert!(!found.message.contains(NONLINEAR_CONFIRMED_EVIDENCE_LINE), "{at}");
    } else {
        assert_eq!(found.code, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{at}: {}", found.message);
        assert_eq!(found.message.contains(NONLINEAR_CONFIRMED_EVIDENCE_LINE), name != "c1_linear", "{at}: {}", found.message);
    }
    if let Some(state) = record["state"].as_str() {
        assert_eq!(state_code(&output, "support:GUIDE"), code_of(state), "{at}");
    }
    let (failures, worst) = compare(&output, record, &at, true);
    assert_eq!(failures, 0, "{at}");
    for (rod, force) in record["tie_rod_force_n"].as_object().unwrap() {
        let id = joint::tie_rod_row_id("component:JOINT", rod);
        let row = output.results.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("{at}: {id}"));
        assert_eq!(row.kind, joint::TIE_ROD_FORCE_KIND);
        assert!(within(row.value, exact(force), exact(&record["scales"]["force_n"])), "{at}: {rod} {} exact {}", row.value, exact(force));
    }
    // The joint's own rows: its elastic axial generalized force k_j*(u_C - u_B),
    // its pressure force p*Ae (the effective-area summand, once) and the
    // effective axial force (elastic less pressure).
    let joint_row = |kind: &str, component: &str| {
        output.results.iter().find(|r| r.kind == kind && r.entity_ref == "component:JOINT" && r.metadata.as_ref().is_some_and(|m| m.component == component))
            .map(|r| r.value)
    };
    let force_zero = exact(&record["scales"]["force_n"]);
    let elastic = exact(&record["joint"]["axial_force_n"]);
    let pressure = exact(&record["joint"]["joint_pressure_force_n"]);
    if let Some(gt) = joint_row("connector_generalized_force_v1", "gt_x") {
        assert!(within(gt, elastic, force_zero), "{at}: gt_x {gt:e} exact {elastic:e}");
    }
    let pe = joint_row("connector_pressure_force_v1", "pe_x").unwrap_or_else(|| panic!("{at}: pe_x"));
    assert!(within(pe, pressure, force_zero), "{at}: pe_x {pe:e} exact {pressure:e}");
    if record["tie_rod_force_n"].as_object().unwrap().is_empty() {
        let s_x = joint_row("connector_effective_axial_force_v1", "s_x").unwrap_or_else(|| panic!("{at}: s_x"));
        assert!(within(s_x, elastic - pressure, force_zero), "{at}: s_x {s_x:e}");
    }
    let envelope = serde_json::to_value(&output).unwrap();
    assert_eq!(reader(&envelope, &wire), None, "{at}");
    println!("RT-A3-S6 {at}: {} max relative error {worst:.3e}", found.code);
    output
}

/// C-1 and its controls, both modes.
#[test]
fn c1_joint_thrust_opens_and_closes_the_guide_in_both_modes() {
    for name in ["c1", "c1_linear", "c1_tied", "c1_timoshenko", "c1_gap", "c1_ls"] {
        let (request, oracle) = bound(name);
        for (case, record) in oracle["cases"].as_object().unwrap() {
            for mode in MODES {
                run_c1(name, &request, record, case, mode);
            }
        }
    }
}

/// C-1's load/reference-state twin: dropping the anchor's motion reopens the
/// guide under pressure; the mutant matches its own reference and not the
/// reference case's.
#[test]
fn c1_ls_dropped_anchor_motion_is_numerically_distinguished() {
    let (request, oracle) = bound("c1_ls");
    for (case, record) in oracle["cases"].as_object().unwrap() {
        let key = "support:D|boundary_motion|UX";
        let mutant = &record["dropped_motion_mutants"][key];
        for mode in MODES {
            let wire = single_case(&drop_motion(&request, case, key), case);
            let output = solve(&wire, mode);
            assert_eq!(output.status.mechanics, "MECHANICS_SOLVED", "{case} {mode:?}");
            assert_eq!(state_code(&output, "support:GUIDE"), code_of(mutant["state"].as_str().unwrap()), "{case} {mode:?}");
            assert_eq!(compare(&output, mutant, case, true).0, 0, "{case} {mode:?}");
            assert!(compare(&output, record, case, false).0 > 0, "{case} {mode:?}");
        }
    }
}

/// Ordinary EX disclosure names the current S6 contract on both the new
/// nonlinear contexts and the no-nonlinear control; mechanics are separately
/// bound to independent reference values above.
#[test]
fn the_formulation_basis_states_current_s6_admission_in_both_contexts() {
    let s6_identity = "beside objective connectors and on load/reference-state documents included, are solved in each case by the active-set loop";
    let s5_identity = "on a document without the load/reference-state group and without an objective connector, gap, one-way, lift-off and friction supports";
    for (name, case) in [("ls1_c5", "load:ACTIVE"), ("c1", "load:PRESSURE"), ("c1_ls", "load:PRESSURE")] {
        let (request, _) = bound(name);
        let output = solve(&single_case(&request, case), PreviewSolverMode::DenseScrutiny);
        let text = output.formulation_basis.limitations.join("\n");
        assert!(text.contains(s6_identity) && !text.contains(s5_identity), "{name}");
        if name != "c1" {
            assert!(text.contains("Boundary motion on an active nonlinear support's own DOF moves its ground"), "{name}");
            assert!(text.contains("Base motion on an active spring's stiffness axis adds the formed load k*g"), "{name}");
            assert!(text.contains("Device preload/reference, inactive or locked supports are not provided."), "{name}");
            assert!(text.contains("contact and friction are static and history-free (DEC-067)"), "{name}");
        }
    }
    // The no-nonlinear control names the same current contract.
    let (request, _) = bound("c1_linear");
    let output = solve(&single_case(&request, "load:PRESSURE"), PreviewSolverMode::DenseScrutiny);
    let text = output.formulation_basis.limitations.join("\n");
    assert!(text.contains(s6_identity) && !text.contains(s5_identity));
}

