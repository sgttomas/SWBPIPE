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
//! rows. K-1's bit-identical first wording is therefore held as superseded:
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
//! - [`every_kd5_control_twin_is_bit_identical`] is K-1's assertion as first
//!   worded (every nodal displacement and rotation, every support action row
//!   and every `nonlinear_support_*` row bit for bit). Agent 0 chose option (a),
//!   keeping each route's established section formation, so it stays held as
//!   the superseded wording;
//! - [`every_kd5_control_twin_is_within_the_k1_bound`] is K-1 as amended
//!   (DEL-04-07 `k1-twin-bound.md`, basis `k1_twin_bound.py`). On this named
//!   corpus the states, the loop's counts, convergence and the standing are
//!   exact. Every value row satisfies |x_EX − x_PF| ≤ a_f + r_f·S_f in its
//!   family's canonical unit, with S_f formed from the case's inputs and every
//!   value required finite. The amendment was adopted after independent
//!   review, and the assertion runs;
//! - [`the_k1_bound_refuses_its_controls`] runs: the comparison refuses the U-1
//!   lifted twin without its parallel spring, a 1e-6 relative EX-only section
//!   perturbation that keeps the discrete evidence, a one-ulp offset at zero
//!   scale, and NaN and ±infinity.
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

/// Every control's twin, every case alone, both modes:
/// `(label, standing, original, twin, PF request, twin request, case)`.
#[allow(clippy::type_complexity)]
fn twins() -> Vec<(String, Standing, Value, Value, Value, Value, &'static str)> {
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
                out.push((at, expected, original, twin, pf_case.clone(), ex_case.clone(), case));
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
    for (at, expected, original, twin, pf_case, ex_case, case) in twins() {
        k1_history_premises(&at, &original, &twin, &pf_case, &ex_case, case);
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

/// K-1 as first worded (module docs), superseded by option (a): every loop row bit for bit.
#[test]
#[ignore = "superseded: K-1 as first worded; EX's D-5 section constants differ from PF's in the last place (kd5_twin_section_constants_differ_from_pf_in_the_last_place), and option (a) keeps both formations (every_kd5_control_twin_is_within_the_k1_bound)"]
fn every_kd5_control_twin_is_bit_identical() {
    for (at, _, original, twin, _, _, _) in twins() {
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

// ------------------------------------------------------------------ K-1 as amended (option (a))
//
// R-4 §3.5 K-1, amended (DEL-04-07 `k1-twin-bound.md`; basis `k1_twin_bound.py`): on the named corpus the states,
// the loop's counts, convergence and the standing are exact, and every value row satisfies
// |x_EX − x_PF| ≤ a_f + r_f·S_f in its family's canonical unit, with S_f formed from the case's inputs only.

/// One published loop row: its key, value and unit.
type Row = (String, f64, String);

/// The amended K-1 constants, per family: (family, canonical unit, a_f, log2 r_f). a_f = 0 throughout: a family's
/// scale is zero only when the case has no load, no applied friction and no gap, and then both routes publish
/// exact zeros. r_f is the next power of two at or above twice the basis' largest enclosure / S_f: the final states'
/// for value and residual rows, and for an iterate-delta row the final iterate's and its derived preceding iterate's
/// (the note's §3, "Iterate deltas", under [`k1_history_premises`]).
const K1_FAMILIES: [(&str, &str, f64, i32); 11] = [
    ("translation", "m", 0.0, -32),
    ("rotation", "rad", 0.0, -31),
    ("force", "N", 0.0, -26),
    ("moment", "N*m", 0.0, -29),
    ("residual_force", "N", 0.0, -36),
    ("residual_moment", "N*m", 0.0, -38),
    ("work", "N*m", 0.0, -40),
    ("translation_delta", "m", 0.0, -32),
    ("rotation_delta", "rad", 0.0, -31),
    ("force_delta", "N", 0.0, -27),
    ("moment_delta", "N*m", 0.0, -29),
];

/// The loop's iteration count per case (`k1_twin_bound.py`'s `ITERATIONS`), a discrete premise of the iterate-delta
/// basis: with one iterate no delta row is published; with two, the preceding iterate is the first; with three, it is
/// the second, whose input the basis derives from the first.
const K1_ITERATIONS: [(&str, &str, f64); 17] = [
    ("F-1 (derived normal)", "load:SLIDE-POS", 2.0),
    ("F-1 (derived normal)", "load:SLIDE-NEG", 2.0),
    ("F-1 (derived normal)", "load:STICK", 3.0),
    ("F-1 (derived normal)", "load:BOUNDARY", 3.0),
    ("F-1 explicit normal", "load:SLIDE", 2.0),
    ("F-1 explicit normal", "load:STICK", 3.0),
    ("F-1 mu = 0", "load:FREE", 2.0),
    ("F-2", "load:SLIDE", 2.0),
    ("U-1", "load:PRESSED", 1.0),
    ("U-1", "load:LIFTED", 2.0),
    ("U-1", "load:IN-PLANE", 2.0),
    ("U-1 parallel spring", "load:PRESSED", 1.0),
    ("U-1 parallel spring", "load:LIFTED", 2.0),
    ("U-1 rotational (RX)", "load:PRESSED", 1.0),
    ("U-1 rotational (RX)", "load:LIFTED", 2.0),
    ("G-1 gap containment (U-1 as a gap)", "load:PRESSED", 2.0),
    ("G-1 gap containment (U-1 as a gap)", "load:LIFTED", 1.0),
];

/// The iterate-history premises of the K-1 delta basis that the product publishes (the note's §3, "Iterate
/// deltas"). The envelope publishes no per-iteration state, slip direction, derived-normal branch or applied
/// friction force; it publishes the iteration count, the final states and the loop's solver diagnostics. On both
/// routes:
/// - the iteration count is the basis' (`K1_ITERATIONS`);
/// - the first iterate's states are the authored seeds: the twin keeps the PF request's supports, seeds included,
///   and neither route reports the first-iterate all-active seed recovery (`SOLVER_SINGULAR_SYSTEM`, "recovered by
///   one all-active trial"). A sliding seed has no previous iterate, so it carries no applied force;
/// - from these, the basis derives the second iterate's input (kept sliding, the slip direction and the derived-
///   normal branch, each the sign of a first-iterate quantity whose margin exceeds its enclosure on both routes).
///   That derivation, not this test, establishes the matched preceding iterate of a three-iterate case.
fn k1_history_premises(at: &str, original: &Value, twin: &Value, pf_case: &Value, ex_case: &Value, case: &str) {
    let label = at.split(" load:").next().unwrap();
    let expected = K1_ITERATIONS.iter().find(|(l, c, _)| *l == label && *c == case).unwrap_or_else(|| panic!("{at}: no premise")).2;
    for (route, envelope) in [("PF", original), ("EX", twin)] {
        let count = envelope["results"].as_array().unwrap().iter()
            .filter(|r| r["kind"] == "nonlinear_support_active_set_iteration_count").map(|r| r["value"].as_f64().unwrap()).collect::<Vec<_>>();
        assert_eq!(count, [expected], "{at} {route}: the iteration count premise");
        let recovered = envelope["diagnostics"].as_array().unwrap().iter()
            .any(|d| d["code"] == "SOLVER_SINGULAR_SYSTEM" && d["message"].as_str().unwrap_or("").contains("recovered by one all-active trial"));
        assert!(!recovered, "{at} {route}: the first iterate used the authored seeds");
    }
    assert_eq!(pf_case["model"]["supports"], ex_case["model"]["supports"], "{at}: the same supports and seeds");
}

/// The rows whose bits K-1 requires exactly: the state codes, the loop's counts and its converged flag.
const K1_EXACT: [&str; 4] = [
    "nonlinear_support_active_set_state_code",
    "nonlinear_support_active_set_iteration_count",
    "nonlinear_support_active_set_final_residual_count",
    "nonlinear_support_active_set_converged_flag",
];

fn loop_values(envelope: &Value) -> Vec<Row> {
    envelope["results"].as_array().unwrap().iter()
        .filter(|r| {
            let kind = r["kind"].as_str().unwrap();
            r["basis_ref"]["ref_type"] == "load_case"
                && (DISPLACEMENT_KINDS.contains(&kind) || SUPPORT_KINDS.contains(&kind) || kind.starts_with("nonlinear_support_"))
        })
        .map(|r| (format!("{} {} {} {}", r["kind"].as_str().unwrap(), r["entity_ref"].as_str().unwrap(),
            r["metadata"]["component"].as_str().unwrap_or(""), r["metadata"]["location"].as_str().unwrap_or("")),
            r["value"].as_f64().unwrap(), r["unit"].as_str().unwrap().to_string()))
        .collect()
}

/// The authored annulus' A, I and J in binary64 (an input scale, formed as `k1_twin_bound.py`'s `section`).
fn k1_section(od: f64, t: f64) -> (f64, f64, f64) {
    let d = od - 2.0 * t;
    let (od2, d2) = (od * od, d * d);
    let area = std::f64::consts::PI / 4.0 * (od2 - d2);
    let inertia = std::f64::consts::PI / 64.0 * (od2 * od2 - d2 * d2);
    (area, inertia, 2.0 * inertia)
}

/// S_f of one case from its PF request's inputs only (`k1_twin_bound.py`'s `scales`, by the same operations):
/// the load and explicit friction force F, the members' total and shortest lengths, the authored section, E, G,
/// the spring stiffness and the gap clearance.
fn k1_scales(request: &Value, case: &str) -> std::collections::BTreeMap<&'static str, f64> {
    let m = &request["model"];
    let position = |id: &Value| {
        let node = m["nodes"].as_array().unwrap().iter().find(|n| n["id"] == *id).unwrap();
        ["x", "y", "z"].map(|a| node["position"][a].as_f64().unwrap())
    };
    let (mut lengths, mut areas, mut inertias, mut polars, mut e_mod, mut g_mod) = (vec![], vec![], vec![], vec![], 0.0, 0.0);
    for pipe in m["pipe_segments"].as_array().unwrap() {
        let (a, b) = (position(&pipe["from"]), position(&pipe["to"]));
        let (dx, dy, dz) = (b[0] - a[0], b[1] - a[1], b[2] - a[2]);
        lengths.push((dx * dx + dy * dy + dz * dz).sqrt());
        let (area, inertia, polar) = k1_section(pipe["section"]["outside_diameter"]["value"].as_f64().unwrap(),
            pipe["section"]["wall_thickness"]["value"].as_f64().unwrap());
        areas.push(area);
        inertias.push(inertia);
        polars.push(polar);
        let material = m["materials"].as_array().unwrap().iter().find(|x| x["id"] == pipe["material"]).unwrap();
        e_mod = material["elastic_modulus"]["value"].as_f64().unwrap();
        g_mod = material["shear_modulus"]["value"].as_f64().unwrap();
    }
    let min = |v: &[f64]| v.iter().copied().fold(f64::INFINITY, f64::min);
    let max = |v: &[f64]| v.iter().copied().fold(0.0, f64::max);
    let total = lengths.iter().sum::<f64>();
    let shortest = min(&lengths);
    let load_case = m["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == case).unwrap();
    let mut force = load_case["primitive_loads"].as_array().unwrap().iter().map(|l| l["magnitude"]["value"].as_f64().unwrap().abs()).sum::<f64>();
    let (mut clearance, mut springs) = (0.0, 0.0);
    for s in m["supports"].as_array().unwrap() {
        let nl = &s["nonlinear"];
        if nl["behavior"] == "friction" && nl.get("normal_reaction").is_some() {
            force += nl["friction_coefficient"]["value"].as_f64().unwrap().abs() * nl["normal_reaction"]["value"].as_f64().unwrap().abs();
        }
        if nl["behavior"] == "gap" {
            clearance += nl["gap"]["value"].as_f64().unwrap().abs();
        }
        if let Some(k) = s["hanger"]["stiffness"]["value"]["value"].as_f64() {
            springs += k.abs();
        }
    }
    let (cube, square) = (total * total * total, total * total);
    let compliance_t = total / (e_mod * min(&areas)) + cube / (e_mod * min(&inertias)) + cube / (g_mod * min(&polars));
    let compliance_r = square / (e_mod * min(&inertias)) + square / (g_mod * min(&polars));
    let stiffness = e_mod * max(&areas) / shortest + 12.0 * e_mod * max(&inertias) / (shortest * shortest * shortest) + springs;
    let s_force = force + clearance * stiffness;
    let s_moment = s_force * total;
    let s_translation = s_force * compliance_t + clearance;
    let s_rotation = s_force * compliance_r + clearance / shortest;
    [
        ("translation", s_translation), ("rotation", s_rotation), ("force", s_force), ("moment", s_moment),
        ("residual_force", s_force), ("residual_moment", s_moment), ("work", s_force * s_translation + s_moment * s_rotation),
        ("translation_delta", s_translation), ("rotation_delta", s_rotation), ("force_delta", s_force), ("moment_delta", s_moment),
    ]
    .into_iter()
    .collect()
}

/// A loop row's K-1 family and the factor from the family's canonical unit to the row's unit (mm rows: 1000).
fn k1_family(key: &str, unit: &str) -> Option<(&'static str, f64)> {
    let kind = key.split(' ').next().unwrap();
    let component = key.split(' ').nth(2).unwrap_or("");
    let family = match kind {
        k if k.starts_with("global_nodal_displacement_") => "translation",
        k if k.starts_with("global_nodal_rotation_") => "rotation",
        "support_reaction_component_v2" if component.starts_with('F') => "force",
        "support_reaction_component_v2" if component.starts_with('M') => "moment",
        "support_reaction_force_magnitude_v2" => "force",
        "support_reaction_moment_magnitude_v2" => "moment",
        "nonlinear_support_final_displacement" if unit == "mm" => "translation",
        "nonlinear_support_final_displacement" if unit == "rad" => "rotation",
        "nonlinear_support_final_reaction" if unit == "N" => "force",
        "nonlinear_support_final_reaction" if unit == "N*m" => "moment",
        "nonlinear_support_friction_normal_reaction_input" | "nonlinear_support_friction_normal_reaction_derived" => "force",
        "nonlinear_support_observed_free_dof_force_residual" => "residual_force",
        "nonlinear_support_observed_free_dof_moment_residual" => "residual_moment",
        "nonlinear_support_free_dof_work_residual" => "work",
        "nonlinear_support_observed_max_translation_delta" => "translation_delta",
        "nonlinear_support_observed_max_rotation_delta" => "rotation_delta",
        "nonlinear_support_observed_max_force_reaction_delta" => "force_delta",
        "nonlinear_support_observed_max_moment_reaction_delta" => "moment_delta",
        _ => return None,
    };
    let canonical = K1_FAMILIES.iter().find(|f| f.0 == family).unwrap().1;
    let factor = match (canonical, unit) {
        ("m", "mm") => 1000.0,
        (c, u) if c == u => 1.0,
        _ => return None,
    };
    Some((family, factor))
}

/// The amended K-1 value comparison of one case-mode: every violation, empty when it holds. The routes must publish
/// the same rows (key and unit, in order); the exact rows must agree bit for bit; every value must be finite on both
/// routes; and each value row must satisfy |x_EX − x_PF| ≤ (a_f + r_f·S_f)·(unit factor). A row with no family fails
/// closed. Rows present on both routes are compared even when the row sets differ.
fn k1_violations(pf: &[Row], ex: &[Row], scales: &std::collections::BTreeMap<&'static str, f64>) -> Vec<String> {
    let mut out = Vec::new();
    if pf.iter().map(|r| (&r.0, &r.2)).collect::<Vec<_>>() != ex.iter().map(|r| (&r.0, &r.2)).collect::<Vec<_>>() {
        out.push("the routes publish different loop rows".into());
    }
    let paired = pf.iter().filter_map(|a| {
        let found = ex.iter().filter(|b| b.0 == a.0 && b.2 == a.2).collect::<Vec<_>>();
        (found.len() == 1).then(|| (a, found[0]))
    });
    for ((key, a, unit), (_, b, _)) in paired {
        if !a.is_finite() || !b.is_finite() {
            out.push(format!("{key}: not finite (PF {a}, EX {b})"));
            continue;
        }
        if K1_EXACT.iter().any(|k| key.starts_with(&format!("{k} "))) {
            if a.to_bits() != b.to_bits() {
                out.push(format!("{key}: exact row differs (PF {a}, EX {b})"));
            }
            continue;
        }
        let Some((family, factor)) = k1_family(key, unit) else {
            out.push(format!("{key} [{unit}]: no K-1 family"));
            continue;
        };
        let (_, _, a_f, log2_r) = K1_FAMILIES.iter().find(|f| f.0 == family).unwrap();
        let scale = scales[family];
        let bound = (a_f + 2f64.powi(*log2_r) * scale) * factor;
        if !scale.is_finite() || !bound.is_finite() || (a - b).abs() > bound {
            out.push(format!("{key}: |EX − PF| = {:e} {unit} above {bound:e} ({family}, S_f {scale:e})", (a - b).abs()));
        }
    }
    out
}

/// The discrete K-1 evidence of a pair: the states, counts and flag (bit for bit) and the standing.
fn discrete(original: &Value, twin: &Value, case: &str) -> (Vec<(String, String)>, Vec<(String, String)>, (String, Value, String), (String, Value, String)) {
    let rows = |e: &Value| loop_results(e).into_iter().filter(|(k, _)| invariant(k)).collect::<Vec<_>>();
    (rows(original), rows(twin), standing(original, case), standing(twin, case))
}

/// K-1 as amended (option (a), adopted after independent review): on the named corpus the
/// discrete evidence is exact and every value row is within its family's bound.
#[test]
fn every_kd5_control_twin_is_within_the_k1_bound() {
    for (at, _, original, twin, pf_case, ex_case, case) in twins() {
        k1_history_premises(&at, &original, &twin, &pf_case, &ex_case, case);
        let (a, b, pf_standing, ex_standing) = discrete(&original, &twin, case);
        assert_eq!(a, b, "{at}: states and loop counts");
        assert_eq!(pf_standing, ex_standing, "{at}: the standing");
        let violations = k1_violations(&loop_values(&original), &loop_values(&twin), &k1_scales(&pf_case, case));
        assert!(violations.is_empty(), "{at}: {violations:#?}");
    }
}

/// The amended K-1's controls: each must fail the bound. A real route discrepancy with the same support state
/// (the U-1 lifted twin without its parallel spring's stiffness); a 1e-6 relative EX-only section perturbation that
/// keeps the discrete evidence; a zero-scale family with a finite offset above a_f; and NaN and ±infinity. These
/// test the comparison itself, independently of the corpus assertion.
#[test]
fn the_k1_bound_refuses_its_controls() {
    let solved_pair = |control: usize, case: &str, edit: &dyn Fn(&mut Value), mode: PreviewSolverMode| {
        let pf = single_case(&pf_request(&CONTROLS[control]), case);
        let mut ex = exact_twin(&pf);
        edit(&mut ex);
        let (original, twin) = (solve(&pf, mode), solve(&ex, mode));
        assert_eq!(twin["status"]["mechanics"], "MECHANICS_SOLVED", "{case}: {:#?}", twin["diagnostics"]);
        (pf, original, twin)
    };
    for mode in MODES {
        // 1. U-1 parallel spring, lifted: the EX twin without the spring's stiffness.
        let (pf, original, twin) = solved_pair(5, "load:LIFTED", &|ex| {
            ex["model"]["supports"].as_array_mut().unwrap().retain(|s| s["id"] != "support:SH-C");
        }, mode);
        let (a, b, pf_standing, ex_standing) = discrete(&original, &twin, "load:LIFTED");
        assert_eq!((&a, &pf_standing.0), (&b, &ex_standing.0), "{mode:?}: the spring control keeps the state, counts and standing");
        let v = k1_violations(&loop_values(&original), &loop_values(&twin), &k1_scales(&pf, "load:LIFTED"));
        assert!(v.iter().any(|x| x == "the routes publish different loop rows"), "{mode:?}: the spring's own rows are gone");
        assert!(v.iter().any(|x| x.contains("(translation,")), "{mode:?}: the spring omitted must fail the value bound: {v:#?}");

        // 2. A 1e-6 relative EX-only section perturbation (every wall), on U-1 lifted and F-2's coupled slide.
        //    A near-uniform section change moves displacements by about 1e-6·|u| and leaves statically fixed
        //    forces alone, so it is visible where |u| is large against S_f (the note's resolution table); on
        //    U-1 pressed every displacement row is zero and it is not.
        for (control, case) in [(4, "load:LIFTED"), (3, "load:SLIDE")] {
            let (pf, original, twin) = solved_pair(control, case, &|ex| {
                for pipe in ex["model"]["pipe_segments"].as_array_mut().unwrap() {
                    let t = pipe["section"]["wall_thickness"]["value"].as_f64().unwrap();
                    pipe["section"]["wall_thickness"]["value"] = json!(t * (1.0 + 1e-6));
                }
            }, mode);
            let (a, b, pf_standing, ex_standing) = discrete(&original, &twin, case);
            assert_eq!((&a, &pf_standing), (&b, &ex_standing), "{case} {mode:?}: the perturbation keeps the discrete evidence");
            let v = k1_violations(&loop_values(&original), &loop_values(&twin), &k1_scales(&pf, case));
            assert!(v.iter().any(|x| x.contains("(translation,")), "{case} {mode:?}: a 1e-6 section perturbation must fail the bound: {v:#?}");
        }
    }

    // 2b. A 1e-6 relative EX-only response perturbation (every value row), on every twin case-mode.
    for (at, _, original, twin, pf_case, _, case) in twins() {
        let mut perturbed = loop_values(&twin);
        for row in perturbed.iter_mut().filter(|r| !K1_EXACT.iter().any(|k| r.0.starts_with(&format!("{k} ")))) {
            row.1 *= 1.0 + 1e-6;
        }
        let v = k1_violations(&loop_values(&original), &perturbed, &k1_scales(&pf_case, case));
        assert!(!v.is_empty() && v.iter().all(|x| x.contains("|EX − PF|")), "{at}: a 1e-6 response perturbation must fail the value bound: {v:#?}");
    }

    // 3. Zero scale: U-1 pressed under a 0 N load has every S_f = 0 (no load, applied friction, gap or
    //    spring). Both routes publish exact zeros, which hold at a_f = 0; one unit in the last place above zero,
    //    a finite offset above a_f, fails.
    let mut unloaded = single_case(&pf_request(&CONTROLS[4]), "load:PRESSED");
    unloaded["model"]["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(0.0);
    let zero = k1_scales(&unloaded, "load:PRESSED");
    assert!(zero.values().all(|s| *s == 0.0), "{zero:?}");
    let (pf_zero, ex_zero) = (loop_values(&solve(&unloaded, PreviewSolverMode::DenseScrutiny)),
        loop_values(&solve(&exact_twin(&unloaded), PreviewSolverMode::DenseScrutiny)));
    assert!(k1_violations(&pf_zero, &ex_zero, &zero).is_empty(), "the unloaded twin holds at zero scale");
    let at = ex_zero.iter().position(|r| r.0.starts_with("global_nodal_rotation_y node:C")).unwrap();
    assert_eq!(ex_zero[at].1, 0.0);
    let mut offset = ex_zero.clone();
    offset[at].1 = f64::from_bits(1);
    let v = k1_violations(&pf_zero, &offset, &zero);
    assert!(v.len() == 1 && v[0].contains("(rotation, S_f 0e0)"), "{v:#?}");

    // 4. NaN and ±infinity are refused on either route.
    let pf = single_case(&pf_request(&CONTROLS[4]), "load:PRESSED");
    let rows = loop_values(&solve(&pf, PreviewSolverMode::DenseScrutiny));
    let rotation = rows.iter().position(|r| r.0.starts_with("global_nodal_rotation_y node:C")).unwrap();
    let scales = k1_scales(&pf, "load:PRESSED");
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        for side in 0..2 {
            let mut broken = rows.clone();
            broken[rotation].1 = bad;
            let v = if side == 0 { k1_violations(&broken, &rows, &scales) } else { k1_violations(&rows, &broken, &scales) };
            assert!(v.len() == 1 && v[0].contains("not finite"), "{bad} on side {side}: {v:#?}");
        }
    }
    assert!(k1_violations(&rows, &rows, &scales).is_empty(), "the unbroken rows hold");
}
