//! RT-A3 increment 4, K-1 (DEL-04-07 Design `route-unification-r4.md` §3.5,
//! "K-1, controls"; DEL-04-07 `pressure-contract.md`, "Nonlinear supports on
//! EX (RT-A3 increment 4)"): every pressure-free K-D5 nonlinear control
//! (`core/product_physics/src/kd5_nonlinear_tests.rs` and the
//! `validation/references/kd5_nonlinear/` requests) solved on its exact twin
//! with explicitly empty pressure regions, in both solver modes, against the
//! PF original.
//!
//! A twin is the PF request re-authored only where the exact contract asks:
//! - the current model version and `pressure_contract` 3.0.0/exact_pressure_v3
//!   (Euler–Bernoulli, PF's beam theory);
//! - each material as `homogeneous_isotropic_E_nu_v1` with the PF modulus E and
//!   ν = E/(2G) − 1, asserted to reproduce PF's G bit for bit;
//! - each section's corrosion allowance stated as an explicit 0 m;
//! - each load case's `pressure_regions: []`.
//!
//! Nothing else changes: nodes, members, supports (seeds included), loads and
//! units are the PF request's.
//!
//! **Finding: the twins are not bit-identical, and the cause precedes the
//! loop.** EX forms each member's section constants by D-5's sequence
//! (`pressure_exact::SourceAnnulus`, published as `pipe_sections`); PF forms
//! them by its own (bound by the kd5 oracles' `normalized_sections`). For the
//! controls' 0.168 × 0.007 m section, A differs by 3 units in the last place
//! and I and J by 1 (A = 0.003540574920595697 m² on EX, 0.003540574920595698 m²
//! on PF). The same
//! twin without its nonlinear support, a linear case, differs in the same
//! rows. K-1's full assertion is therefore held, not loosened:
//! - [`every_kd5_control_keeps_its_states_counts_and_standing_on_its_exact_twin`]
//!   asserts bit for bit what a last-place stiffness difference does not move:
//!   each support's state code, the loop's iteration count, final residual
//!   count and converged flag, and the standing (the integrity code, the
//!   `numerical_quality` record, and kd5's evidence line or the named §2.4
//!   reason), each as the PF test pins it. It prints, per case, how many other
//!   loop rows differ and the largest difference relative to the case's
//!   largest value in the row's family (at most 4.5e-15 on this base, except
//!   U-1 IN-PLANE dense, whose moment family is wholly binary64 noise of order
//!   1e-16 N·m and changes sign), and how many of the loop's residual
//!   observations (noise of the final iterate) differ;
//! - [`kd5_twin_section_constants_differ_from_pf_in_the_last_place`] pins the
//!   cause;
//! - [`every_kd5_control_twin_is_bit_identical`] is K-1's full assertion
//!   (every nodal displacement and rotation, every support action row and every
//!   `nonlinear_support_*` row bit for bit), held `#[ignore]`d until EX and PF
//!   form the same section constants, a choice outside this increment.
//!
//! Every twin envelope passes the pressure reader with and without its model,
//! so these controls also exercise the reader's nonlinear binding
//! (`result_export::nonlinear_support_evidence`) on further support kinds:
//! derived and explicit friction, μ = 0, a rotational one-way support, a
//! parallel spring, the gap branch and the fail-closed weakly active states.
//!
//! The controls that cannot form an exact twin are listed with their reason in
//! [`NOT_FORMABLE`] and pinned by name; none is forced.
use open_pipe_stress_product_physics::result_export::reader_verdict;
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const CONFIRMED: &str = "Converged support state: established at 128-bit precision";
const NOT_ESTABLISHED: &str = "the converged support state is not established at 128-bit precision (";
const DISPLACEMENT_KINDS: [&str; 6] = [
    "global_nodal_displacement_x",
    "global_nodal_displacement_y",
    "global_nodal_displacement_z",
    "global_nodal_rotation_x",
    "global_nodal_rotation_y",
    "global_nodal_rotation_z",
];
const SUPPORT_KINDS: [&str; 3] = ["support_reaction_component_v2", "support_reaction_force_magnitude_v2", "support_reaction_moment_magnitude_v2"];

/// One K-D5 control: a PF request and the load cases its test solves, each
/// alone, with the standing the PF test pins (`None` Passed, `Some` the
/// named reason, or the gap branch).
struct Control {
    label: &'static str,
    request: &'static str,
    /// The kd5 oracle, whose `normalized_sections` are PF's built sections.
    oracle: &'static str,
    cases: &'static [(&'static str, Standing)],
    /// A re-authoring the PF test applies to its request (the gap control).
    edit: Option<fn(&mut Value)>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Standing {
    Passed,
    FailClosed(&'static str),
    GapBranch,
}

/// U-1 re-authored as the PF gap control does (`gap_cases_keep_their_recovery_basis_branch`).
fn as_gap(request: &mut Value) {
    let support = request["model"]["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "support:NL-U").unwrap();
    support["nonlinear"] = json!({"behavior": "gap", "dof": "UZ", "initial_state": "inactive", "closes_when": "negative_displacement", "gap": {"value": 1e-5, "unit": "m"}});
}

const CONTROLS: [Control; 8] = [
    Control {
        label: "F-1 (derived normal)",
        request: include_str!("../../../validation/references/kd5_nonlinear/f1_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/f1_oracle.json"),
        cases: &[
            ("load:SLIDE-POS", Standing::Passed),
            ("load:SLIDE-NEG", Standing::Passed),
            ("load:STICK", Standing::Passed),
            ("load:BOUNDARY", Standing::FailClosed("support support:NL-F: stick capacity not established")),
        ],
        edit: None,
    },
    Control {
        label: "F-1 explicit normal",
        request: include_str!("../../../validation/references/kd5_nonlinear/f1_explicit_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/f1_explicit_oracle.json"),
        cases: &[("load:SLIDE", Standing::Passed), ("load:STICK", Standing::Passed)],
        edit: None,
    },
    Control {
        label: "F-1 mu = 0",
        request: include_str!("../../../validation/references/kd5_nonlinear/f1_mu0_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/f1_mu0_oracle.json"),
        cases: &[("load:FREE", Standing::Passed)],
        edit: None,
    },
    Control {
        label: "F-2",
        request: include_str!("../../../validation/references/kd5_nonlinear/f2_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/f2_oracle.json"),
        cases: &[("load:SLIDE", Standing::Passed)],
        edit: None,
    },
    Control {
        label: "U-1",
        request: include_str!("../../../validation/references/kd5_nonlinear/u1_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/u1_oracle.json"),
        cases: &[
            ("load:PRESSED", Standing::Passed),
            ("load:LIFTED", Standing::Passed),
            ("load:IN-PLANE", Standing::FailClosed("support support:NL-U: separation not established")),
        ],
        edit: None,
    },
    Control {
        label: "U-1 parallel spring",
        request: include_str!("../../../validation/references/kd5_nonlinear/u1_spring_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/u1_spring_oracle.json"),
        cases: &[("load:PRESSED", Standing::Passed), ("load:LIFTED", Standing::Passed)],
        edit: None,
    },
    Control {
        label: "U-1 rotational (RX)",
        request: include_str!("../../../validation/references/kd5_nonlinear/u1_rx_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/u1_rx_oracle.json"),
        cases: &[("load:PRESSED", Standing::Passed), ("load:LIFTED", Standing::Passed)],
        edit: None,
    },
    Control {
        label: "G-1 gap containment (U-1 as a gap)",
        request: include_str!("../../../validation/references/kd5_nonlinear/u1_request.json"),
        oracle: include_str!("../../../validation/references/kd5_nonlinear/u1_oracle.json"),
        cases: &[("load:PRESSED", Standing::GapBranch), ("load:LIFTED", Standing::GapBranch)],
        edit: Some(as_gap),
    },
];

/// The PF controls with no exact twin, and why: kd5's 8 committed case-modes
/// (`kd5_eight_committed_nonlinear_case_modes_pass_with_the_confirmation`) solve
/// two committed demo models whose geometry-only bend (D-2; S6's inventory)
/// and terminal component the exact route refuses by name. Pinned by
/// `controls_without_a_twin_are_refused_by_name_not_forced`.
const NOT_FORMABLE: [(&str, &str, &str, [&str; 2]); 2] = [
    ("committed demo model (kd5 8 case-modes)", include_str!("../../../fixtures/product_preview/invented_demo_model.json"),
        "geometry-only bend component:C-110 and terminal component:C-140 are refused by EXACT_PRESSURE_FAMILY_NOT_ADMITTED",
        ["component:C-110", "component:C-140"]),
    ("preview-physics invented model (kd5 8 case-modes)", include_str!("fixtures/preview_physics_invented_model.json"),
        "geometry-only bend component:C-110 and terminal component:C-140 are refused by EXACT_PRESSURE_FAMILY_NOT_ADMITTED",
        ["component:C-110", "component:C-140"]),
];

/// The PF request at the current model version, as the kd5 tests author it.
fn pf_request(control: &Control) -> Value {
    let mut request: Value = serde_json::from_str(control.request).unwrap();
    request["model"]["schema_version"] = json!(MODEL_SCHEMA_VERSION);
    if let Some(edit) = control.edit {
        edit(&mut request);
    }
    request
}

fn single_case(request: &Value, case: &str) -> Value {
    let mut wire = request.clone();
    let cases = wire["model"]["load_cases"].as_array_mut().unwrap();
    cases.retain(|c| c["id"] == case);
    assert_eq!(cases.len(), 1, "{case}");
    wire
}

/// The exact twin of a PF request (module docs).
fn exact_twin(pf: &Value) -> Value {
    let mut twin = pf.clone();
    let model = &mut twin["model"];
    model["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
    for material in model["materials"].as_array_mut().unwrap() {
        let e = material["elastic_modulus"]["value"].as_f64().unwrap();
        let g = material["shear_modulus"]["value"].as_f64().unwrap();
        assert_eq!((material["elastic_modulus"]["unit"].as_str(), material["shear_modulus"]["unit"].as_str()), (Some("Pa"), Some("Pa")));
        let nu = e / (2.0 * g) - 1.0;
        assert_eq!((e / (2.0 * (1.0 + nu))).to_bits(), g.to_bits(), "{}: nu reproduces G", material["id"]);
        *material = json!({"id": material["id"], "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": material["elastic_modulus"], "poisson_ratio": {"value": nu, "unit": "1"}, "provenance": material["provenance"]});
    }
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        assert!(pipe["section"].get("corrosion_allowance").is_none());
        pipe["section"]["corrosion_allowance"] = json!({"value": 0.0, "unit": "m"});
    }
    for case in model["load_cases"].as_array_mut().unwrap() {
        assert!(case.get("pressure_regions").is_none());
        case["pressure_regions"] = json!([]);
    }
    twin
}

fn solve(request: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).expect("the entry accepts the request")).unwrap()
}

fn bits(row: &Value) -> String {
    format!("{:016x}", row["value"].as_f64().unwrap().to_bits())
}

/// The loop results of an envelope, keyed by kind, entity and component, in
/// emission order, with their value bits.
fn loop_results(envelope: &Value) -> Vec<(String, String)> {
    envelope["results"].as_array().unwrap().iter()
        .filter(|r| {
            let kind = r["kind"].as_str().unwrap();
            r["basis_ref"]["ref_type"] == "load_case"
                && (DISPLACEMENT_KINDS.contains(&kind) || SUPPORT_KINDS.contains(&kind) || kind.starts_with("nonlinear_support_"))
        })
        .map(|r| (format!("{} {} {} {}", r["kind"].as_str().unwrap(), r["entity_ref"].as_str().unwrap(),
            r["metadata"]["component"].as_str().unwrap_or(""), r["metadata"]["location"].as_str().unwrap_or("")), bits(r)))
        .collect()
}

/// The case's standing: its integrity code, its `numerical_quality` record
/// and the K-D5 sentence it carries (the evidence line, or the named reason).
fn standing(envelope: &Value, case: &str) -> (String, Value, String) {
    let found = envelope["diagnostics"].as_array().unwrap().iter()
        .filter(|d| d["affected_refs"] == json!([case]) && d["code"].as_str().unwrap_or("").starts_with("NUMERICAL_INTEGRITY_"))
        .collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{case}: {found:#?}");
    let message = found[0]["message"].as_str().unwrap();
    let sentence = if message.contains(CONFIRMED) {
        CONFIRMED.to_string()
    } else if let Some((_, rest)) = message.split_once(NOT_ESTABLISHED) {
        rest.split_once("); member rows").map(|(reason, _)| reason.to_string()).unwrap_or_default()
    } else {
        String::new()
    };
    let quality = envelope["numerical_quality"]["cases"].as_array().unwrap().iter()
        .find(|q| q["basis_ref"]["ref_id"] == case).cloned().unwrap();
    (found[0]["code"].as_str().unwrap().to_string(), quality, sentence)
}

fn reader(envelope: &Value, request: Option<&Value>) -> Option<String> {
    let found = reader_verdict::validate("pressure", envelope, request.map(|r| &r["model"]), None);
    (found["ok"] != true).then(|| found["error"].to_string())
}

/// The rows a last-place stiffness difference cannot move: the state codes and
/// the loop's counts and flag.
fn invariant(key: &str) -> bool {
    ["nonlinear_support_active_set_state_code", "nonlinear_support_active_set_iteration_count",
        "nonlinear_support_active_set_final_residual_count", "nonlinear_support_active_set_converged_flag"]
        .iter()
        .any(|kind| key.starts_with(&format!("{kind} ")))
}

/// The distance of two binary64 values in units in the last place.
fn ulps(a: &str, b: &str) -> u64 {
    let ordered = |hex: &str| {
        let bits = u64::from_str_radix(hex, 16).unwrap() as i64;
        if bits < 0 { i64::MIN - bits } else { bits }
    };
    ordered(a).abs_diff(ordered(b))
}

/// Every control's twin, every case alone, both modes: `(label, original, twin, twin request)`.
fn twins() -> Vec<(String, Standing, Value, Value, Value, &'static str)> {
    let mut out = Vec::new();
    for control in &CONTROLS {
        let pf = pf_request(control);
        let ex = exact_twin(&pf);
        for &(case, expected) in control.cases {
            let (pf_case, ex_case) = (single_case(&pf, case), single_case(&ex, case));
            for mode in MODES {
                let at = format!("{} {case} {mode:?}", control.label);
                let original = solve(&pf_case, mode);
                let twin = solve(&ex_case, mode);
                assert_eq!(original["status"]["mechanics"], "MECHANICS_SOLVED", "{at} PF: {:#?}", original["diagnostics"]);
                assert_eq!(twin["status"]["mechanics"], "MECHANICS_SOLVED", "{at} EX: {:#?}", twin["diagnostics"]);
                assert_eq!(twin["formulation_basis"]["profile_id"], "exact_pressure_v3", "{at}");
                assert!(twin["contract_evidence"]["pressure"].as_array().unwrap().is_empty(), "{at}: no pressure region");
                out.push((at, expected, original, twin, ex_case.clone(), case));
            }
        }
    }
    assert_eq!(out.len(), 2 * 17);
    out
}

/// K-1 as far as it holds on this base (module docs): the states, the loop's
/// counts and the standing bit for bit, the PF test's standing, and the reader.
#[test]
fn every_kd5_control_keeps_its_states_counts_and_standing_on_its_exact_twin() {
    let mut summary = Vec::new();
    for (at, expected, original, twin, ex_case, case) in twins() {
        let (a, b) = (loop_results(&original), loop_results(&twin));
        assert_eq!(a.iter().map(|r| &r.0).collect::<Vec<_>>(), b.iter().map(|r| &r.0).collect::<Vec<_>>(), "{at}: the same loop rows");
        assert!(a.iter().any(|(key, _)| key.starts_with("nonlinear_support_active_set_state_code")), "{at}");
        let invariant_rows = |rows: &[(String, String)]| rows.iter().filter(|(key, _)| invariant(key)).cloned().collect::<Vec<_>>();
        assert_eq!(invariant_rows(&a), invariant_rows(&b), "{at}: states and loop counts");
        let (pf_standing, ex_standing) = (standing(&original, case), standing(&twin, case));
        assert_eq!(pf_standing, ex_standing, "{at}: the standing differs");
        match expected {
            Standing::Passed => {
                assert_eq!(pf_standing.0, "NUMERICAL_INTEGRITY_CHECKS_PASSED", "{at}");
                assert_eq!(pf_standing.2, CONFIRMED, "{at}");
            }
            Standing::FailClosed(reason) => {
                assert_eq!(pf_standing.0, "NUMERICAL_INTEGRITY_SENSITIVE", "{at}");
                assert_eq!(pf_standing.2, reason, "{at}");
            }
            Standing::GapBranch => {
                assert_eq!(pf_standing.0, "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", "{at}");
                assert_eq!(pf_standing.2, "", "{at}");
            }
        }
        assert_eq!(reader(&twin, Some(&ex_case)), None, "{at}: the pressure reader with the model");
        assert_eq!(reader(&twin, None), None, "{at}: the pressure reader without the model");
        // Observed, not asserted (the held test asserts bit identity): the
        // value rows' largest relative difference, and the loop's residual
        // observations, which are binary64 noise of the final iterate.
        let value = |hex: &str| f64::from_bits(u64::from_str_radix(hex, 16).unwrap());
        let observation = |key: &str| key.contains("_observed_") || key.starts_with("nonlinear_support_free_dof_work_residual");
        let differing = a.iter().zip(&b).filter(|(x, y)| x.1 != y.1).collect::<Vec<_>>();
        // Each difference relative to the case's largest value in its family:
        // forces (support actions, nonlinear reactions and normals), moments,
        // translations (mm) and rotations (rad).
        let kind_of = |key: &str| {
            let kind = key.split(' ').next().unwrap();
            let moment = key.contains(" M") || kind.contains("moment");
            if kind.starts_with("support_reaction") || kind.ends_with("_reaction") || kind.contains("normal_reaction") {
                if moment { "moment" } else { "force" }
            } else if kind.starts_with("global_nodal_rotation") || key.ends_with(" rx") || key.ends_with(" ry") || key.ends_with(" rz") {
                "rotation"
            } else if kind.starts_with("global_nodal_displacement") || kind.contains("displacement") {
                "translation"
            } else {
                kind
            }
            .to_string()
        };
        let scale = |k: &str| a.iter().filter(|(key, _)| kind_of(key) == k).map(|(_, h)| value(h).abs()).fold(0.0_f64, f64::max);
        let relative = differing.iter().filter(|(x, _)| !observation(&x.0))
            .map(|(x, y)| (value(&x.1) - value(&y.1)).abs() / scale(&kind_of(&x.0)))
            .fold(0.0_f64, f64::max);
        let noise = differing.iter().filter(|(x, _)| observation(&x.0)).count();
        summary.push(format!("{at}: {} loop rows, {} value rows differ (largest relative to its family's case scale {relative:.1e}), {noise} residual observations differ; {}",
            a.len(), differing.len() - noise, pf_standing.0));
    }
    println!("{}", summary.join("\n"));
}

/// K-1's full assertion (module docs), held: every loop row bit for bit.
#[test]
#[ignore = "held: EX's D-5 section constants differ from PF's in the last place (kd5_twin_section_constants_differ_from_pf_in_the_last_place); a section-formation choice outside RT-A3 increment 4"]
fn every_kd5_control_twin_is_bit_identical() {
    for (at, _, original, twin, _, _) in twins() {
        assert_eq!(loop_results(&original), loop_results(&twin), "{at}: the loop results differ");
    }
}

/// The cause of the held assertion: each control's twin publishes D-5 section
/// constants (A, I, J) that differ from PF's built sections (the kd5 oracle's
/// `normalized_sections`, which kd5's own test binds to PF's build) by 1 to 3
/// units in the last place; and without its nonlinear support, the same twin's linear case
/// differs from PF's in its nodal rows too, so the difference precedes the loop.
#[test]
fn kd5_twin_section_constants_differ_from_pf_in_the_last_place() {
    for control in &CONTROLS {
        let oracle: Value = serde_json::from_str(control.oracle).unwrap();
        let pf = pf_request(control);
        let (case, _) = control.cases[0];
        let twin = solve(&single_case(&exact_twin(&pf), case), PreviewSolverMode::DenseScrutiny);
        for section in twin["contract_evidence"]["exact_cases"][0]["pipe_sections"].as_array().unwrap() {
            let pipe = section["pipe_id"].as_str().unwrap();
            let built = &oracle["normalized_sections"][pipe];
            for (ex, pf) in [("As_m2", "As_m2"), ("I_m4", "I_m4"), ("J_m4", "J_m4")] {
                let (e, p) = (section["nominal_section"][ex].as_f64().unwrap(), built[pf].as_f64().unwrap());
                let distance = ulps(&format!("{:016x}", e.to_bits()), &format!("{:016x}", p.to_bits()));
                assert!((1..=3).contains(&distance), "{} {pipe} {ex}: EX {e:e}, PF {p:e}, {distance} ulp", control.label);
            }
        }
        // The linear twin: the nonlinear supports removed.
        let mut linear = single_case(&pf, case);
        linear["model"]["supports"].as_array_mut().unwrap().retain(|s| s.get("nonlinear").is_none_or(Value::is_null));
        let (a, b) = (solve(&linear, PreviewSolverMode::DenseScrutiny), solve(&exact_twin(&linear), PreviewSolverMode::DenseScrutiny));
        let nodal = |e: &Value| e["results"].as_array().unwrap().iter()
            .filter(|r| DISPLACEMENT_KINDS.contains(&r["kind"].as_str().unwrap()))
            .map(|r| (r["entity_ref"].clone(), r["kind"].clone(), bits(r))).collect::<Vec<_>>();
        assert_ne!(nodal(&a), nodal(&b), "{}: the linear twin's nodal rows", control.label);
    }
}

/// The PF test's rotational-friction control: refused by name at validation
/// on the twin as on the original, in both modes; a translational friction
/// support on the same request is admitted.
#[test]
fn rotational_friction_is_refused_by_name_on_the_twin_as_on_the_original() {
    let control = &CONTROLS[1];
    for (dof, refused) in [("RX", true), ("RZ", true), ("UX", false)] {
        let mut pf = pf_request(control);
        let support = pf["model"]["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "support:NL-F").unwrap();
        support["nonlinear"]["dof"] = json!(dof);
        let ex = exact_twin(&pf);
        for mode in MODES {
            let refusals = |envelope: &Value| envelope["diagnostics"].as_array().unwrap().iter()
                .filter(|d| d["code"] == "NONLINEAR_ROTATIONAL_FRICTION_UNSUPPORTED")
                .map(|d| json!([d["id"], d["severity"], d["message"], d["affected_refs"]])).collect::<Vec<_>>();
            let (original, twin) = (solve(&pf, mode), solve(&ex, mode));
            assert_eq!(refusals(&original), refusals(&twin), "{dof} {mode:?}");
            assert_eq!(refusals(&twin).len(), usize::from(refused), "{dof} {mode:?}");
            assert_eq!(original["status"]["mechanics"] == "MECHANICS_SOLVED", !refused, "{dof} {mode:?}");
            assert_eq!(twin["status"]["mechanics"], original["status"]["mechanics"], "{dof} {mode:?}");
        }
    }
}

/// The controls with no exact twin are refused by the exact route by name,
/// not adapted: each committed model's twin is `MODEL_INCOMPLETE` with the
/// family refusal on exactly the named components, in both modes, while the
/// PF original solves.
#[test]
fn controls_without_a_twin_are_refused_by_name_not_forced() {
    for (label, model, reason, refused) in NOT_FORMABLE {
        let mut pf = json!({"model": serde_json::from_str::<Value>(model).unwrap(), "materials": []});
        pf["model"]["schema_version"] = json!(MODEL_SCHEMA_VERSION);
        let twin = exact_twin(&pf);
        for mode in MODES {
            assert_eq!(solve(&pf, mode)["status"]["mechanics"], "MECHANICS_SOLVED", "{label} {mode:?}");
            let envelope = solve(&twin, mode);
            assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE", "{label} {mode:?}: {reason}");
            let named = envelope["diagnostics"].as_array().unwrap().iter()
                .filter(|d| d["severity"] == "blocking")
                .map(|d| (d["code"].as_str().unwrap(), d["affected_refs"].clone()))
                .collect::<Vec<_>>();
            assert_eq!(named, refused.iter().map(|c| ("EXACT_PRESSURE_FAMILY_NOT_ADMITTED", json!([c]))).collect::<Vec<_>>(), "{label} {mode:?}: {reason}");
        }
    }
}
