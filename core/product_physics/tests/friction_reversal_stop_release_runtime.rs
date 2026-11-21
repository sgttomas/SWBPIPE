//! Three-way friction reversal with stop release on the bundled demo model
//! (`fixtures/product_preview/invented_demo_model.json`: joint-free and
//! pressure-free), through the captured value entry the desktop uses, in both
//! solver modes. Each load case is solved as authored (`Original`), with its
//! line weight reversed (`ReverseZ`) and with every primitive reversed
//! (`ReverseAll`: the Y load lifts N-140 off the one-way stop, which releases,
//! and the derived friction normal grows from the frame share to the whole Y load).
//!
//! Two models, two tables (DEL-04-11 REQ-014, the variable spring preload law):
//! - the frozen table `EXPECTED` is the stiffness-only result: it holds on a twin
//!   of the bundled demo whose hanger SH-140 is a plain spring of the same id,
//!   node, DOF and rate, made in memory and asserted to differ from the bundled
//!   demo as loaded in that one support record only. On it, `ReverseZ` slides the
//!   rack shoe NL-130-FRIC the other way and the stop NL-140 stays active;
//! - the bundled demo itself solves under the law (SH-140's 430 N cold load acts
//!   along +UZ in every case and variant), against the successor package
//!   `validation/references/demo_friction_reversal_preload` (its README), whose
//!   six scenarios have exactly the fields of `EXPECTED`. Under the law the shoe
//!   slides +UZ in all six, and SH-140 publishes its cold load as its applied force.
//!
//! The frozen expectations are an independent reference, not product output: an exact
//! rational (Python `fractions`) small-displacement 3D Euler-Bernoulli frame of the
//! demo (annular A, I, J = 2I; consistent line loads; thermal initial strain;
//! SH-140 a 42000 N/m UZ spring at zero reference; anchor, guide and S-130 rigid;
//! components and CE-120 not consumed), solved for every NL-140 state x friction
//! state (stick, slide +UZ, slide -UZ) x sign of the S-130 action, with static
//! zero-reference Coulomb friction mu = 0.01 on N = |S-130 UY action| of the same
//! state. Exactly one branch is admissible in each scenario (stop active iff its
//! action is negative, released iff N-140 UY <= 0; stick iff |F| <= mu N; slide iff
//! F = -sign(UZ) mu N). The reference also checks, exactly, that the anchor carries
//! no Y force (the loop lies in z = 0 and the Y load enters above S-130), so a
//! released stop leaves N = |applied Y| and F = mu N by statics, and that an active
//! stop's normal is P-130's propped shear 3 EI theta / L^2. The authored L-100 normal
//! agrees with the earlier independent frame record (52.37328198733 N) to 6e-13.
//!
//! Criteria. Frame quantities use the analytic criterion |obs - exp| <= 1e-9 |exp|.
//! An exactly-zero expectation uses 1e-9 times the case's authored Y-force scale.
//! The friction force is mu N, solved together with the reaction that defines N
//! (the derived-normal Coulomb coupling), so its accuracy class is a force at the
//! scale of N: |obs - exp| <= 1e-9 N.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const DEMO: &str = include_str!("../../../fixtures/product_preview/invented_demo_model.json");
/// The scenarios under the preload law, and the package's sums that bind them.
const UNDER_THE_LAW: &str = include_str!("../../../validation/references/demo_friction_reversal_preload/demo_friction_reversal_preload.json");
const UNDER_THE_LAW_SUMS: &str = include_str!("../../../validation/references/demo_friction_reversal_preload/SHA256SUMS");
const HANGER: &str = "support:SH-140";
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const MU: f64 = 0.01;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Variant {
    Original,
    ReverseZ,
    ReverseAll,
}

/// Reference values (N, mm; support actions act on the structure).
struct Expected {
    case: &'static str,
    variant: Variant,
    applied_y_n: f64,
    stop_active: bool,
    stop_uy_mm: f64,
    stop_reaction_n: f64,
    slip_mm: f64,
    friction_n: f64,
    normal_n: f64,
    s130_fy_n: f64,
    sh140_fz_n: f64,
}

#[rustfmt::skip]
const EXPECTED: [Expected; 6] = [
    Expected { case: "load:L-100", variant: Variant::Original, applied_y_n: 350.0, stop_active: true, stop_uy_mm: 0.0, stop_reaction_n: -297.6267180126413, slip_mm: -6.643926797396845, friction_n: 0.5237328198735871, normal_n: 52.373281987358716, s130_fy_n: -52.373281987358716, sh140_fz_n: 279.00851842729384 },
    Expected { case: "load:L-100", variant: Variant::ReverseZ, applied_y_n: 350.0, stop_active: true, stop_uy_mm: 0.0, stop_reaction_n: -402.3732819873587, slip_mm: 6.643926797396845, friction_n: -0.5237328198735871, normal_n: 52.373281987358716, s130_fy_n: 52.373281987358716, sh140_fz_n: -279.00851842729384 },
    Expected { case: "load:L-100", variant: Variant::ReverseAll, applied_y_n: -350.0, stop_active: false, stop_uy_mm: -5.356128551796324, stop_reaction_n: 0.0, slip_mm: 5.999143262955726, friction_n: -3.5, normal_n: 350.0, s130_fy_n: 350.0, sh140_fz_n: -251.93114323387397 },
    Expected { case: "load:L-200", variant: Variant::Original, applied_y_n: 125.0, stop_active: true, stop_uy_mm: 0.0, stop_reaction_n: -98.81335900632064, slip_mm: -3.3219633986984225, friction_n: 0.26186640993679355, normal_n: 26.186640993679358, s130_fy_n: -26.186640993679358, sh140_fz_n: 139.50425921364692 },
    Expected { case: "load:L-200", variant: Variant::ReverseZ, applied_y_n: 125.0, stop_active: true, stop_uy_mm: 0.0, stop_reaction_n: -151.18664099367936, slip_mm: 3.3219633986984225, friction_n: -0.26186640993679355, normal_n: 26.186640993679358, s130_fy_n: 26.186640993679358, sh140_fz_n: -139.50425921364692 },
    Expected { case: "load:L-200", variant: Variant::ReverseAll, applied_y_n: -125.0, stop_active: false, stop_uy_mm: -1.778257869477212, stop_reaction_n: 0.0, slip_mm: 3.107892473562634, friction_n: -1.25, normal_n: 125.0, s130_fy_n: 125.0, sh140_fz_n: -130.5144534132732 },
];

/// The bundled demo as loaded.
fn demo() -> Value {
    serde_json::from_str(DEMO).unwrap()
}

/// The stiffness-only twin of the demo: SH-140 a plain spring of the hanger's
/// id, node, stiffness DOF and rate, with no hanger member and no cold load.
fn plain_spring_twin() -> Value {
    let mut model = demo();
    let supports = model["supports"].as_array_mut().unwrap();
    let at = supports.iter().position(|s| s["id"] == HANGER).expect("the demo's hanger SH-140");
    let hanger = supports[at].clone();
    supports[at] = json!({"id": HANGER, "node": hanger["node"], "family": "spring",
        "restraints": [hanger["hanger"]["stiffness"]["dof"]], "stiffness": hanger["hanger"]["stiffness"],
        "provenance": hanger["provenance"]});
    model
}

/// `model` with the variant's primitive loads negated; nothing else changes.
fn demo_request(mut model: Value, variant: Variant) -> Value {
    // The reference models C-130 as unconsumed ("components and CE-120 not
    // consumed"); W-15 (DEL-03-05 Design `rigid-component-stiffness-scaling.md`)
    // now consumes the demo's valve, scaling P-130 by its user scale. This
    // test-local copy declares C-130 not_solver_consumed so the input matches
    // the reference's stated basis.
    let valve = model["components"].as_array_mut().unwrap().iter_mut()
        .find(|component| component["id"] == "component:C-130").expect("the demo's valve C-130");
    valve["mechanics_interface"]["solver_consumption"] = json!("not_solver_consumed");
    for case in model["load_cases"].as_array_mut().unwrap() {
        for load in case["primitive_loads"].as_array_mut().unwrap() {
            let reverse = match variant {
                Variant::Original => false,
                Variant::ReverseZ => load["direction"] == "global_z" && load["dimension"] == "force_per_length",
                Variant::ReverseAll => true,
            };
            if reverse {
                let value = load["magnitude"]["value"].as_f64().unwrap();
                load["magnitude"]["value"] = json!(-value);
            }
        }
    }
    json!({"model": model, "materials": []})
}

fn solve(model: &Value, variant: Variant, mode: PreviewSolverMode) -> Value {
    let envelope = run_linear_static_preview_value_with_mode(demo_request(model.clone(), variant), mode).expect("captured entry");
    let v = serde_json::to_value(envelope).unwrap();
    assert_eq!(v["status"]["mechanics"], "MECHANICS_SOLVED", "{variant:?} {mode:?}: {}", v["diagnostics"]);
    assert_eq!(v["accepted_model_state_mutated"], false);
    v
}

fn row<'a>(v: &'a Value, kind: &str, entity: Option<&str>, location: Option<&str>, case: &str) -> &'a Value {
    v["results"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| {
            r["kind"] == kind
                && entity.is_none_or(|e| r["entity_ref"] == e)
                && location.is_none_or(|l| r["metadata"]["location"] == l)
                && r["basis_ref"]["ref_type"] == "load_case"
                && r["basis_ref"]["ref_id"] == case
        })
        .unwrap_or_else(|| panic!("missing {kind} {entity:?} {location:?} for {case}"))
}

fn value(v: &Value, kind: &str, entity: Option<&str>, location: Option<&str>, case: &str) -> f64 {
    row(v, kind, entity, location, case)["value"].as_f64().unwrap()
}

fn action(v: &Value, support: &str, component: &str, case: &str) -> f64 {
    let found = v["results"].as_array().unwrap().iter().find(|r| {
        r["kind"] == "support_reaction_component_v2"
            && r["entity_ref"] == support
            && r["metadata"]["component"] == component
            && r["basis_ref"]["ref_id"] == case
    });
    found.unwrap_or_else(|| panic!("missing {support} {component} for {case}"))["value"].as_f64().unwrap()
}

/// Analytic criterion for a nonzero expectation.
fn close(observed: f64, expected: f64, what: &str) {
    assert_ne!(expected, 0.0, "{what}");
    assert!(
        observed.is_finite() && (observed - expected).abs() <= 1e-9 * expected.abs(),
        "{what}: observed {observed:.17e} expected {expected:.17e}"
    );
}

/// An exactly-zero expectation, at 1e-9 of the stated scale.
fn zero(observed: f64, scale: f64, what: &str) {
    assert!(observed.is_finite() && observed.abs() <= 1e-9 * scale.abs(), "{what}: observed {observed:.17e} (scale {scale})");
}

/// Every scenario of `table` on `model`, in both modes and all three variants.
fn friction_reverses_and_the_stop_releases(model: &Value, table: &[Expected], label: &str) {
    for mode in MODES {
        for variant in [Variant::Original, Variant::ReverseZ, Variant::ReverseAll] {
            let v = solve(model, variant, mode);
            assert!(!v["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "NONLINEAR_SUPPORT_LOOP_BLOCKED"));
            for e in table.iter().filter(|e| e.variant == variant) {
                let (case, at) = (e.case, format!("{label} {mode:?} {variant:?} {}", e.case));
                let nl = |entity: &str, kind: &str, location: Option<&str>| value(&v, kind, Some(entity), location, case);
                assert_eq!(value(&v, "nonlinear_support_active_set_converged_flag", None, None, case), 1.0, "{at}");
                assert_eq!(value(&v, "nonlinear_support_active_set_final_residual_count", None, None, case), 0.0, "{at}");

                // NL-140: active (code 1) under the authored and Z-reversed loads,
                // released (code 0) once the Y load is reversed.
                let stop = "support:NL-140";
                let (code, state) = if e.stop_active { (1.0, "active") } else { (0.0, "inactive") };
                assert_eq!(nl(stop, "nonlinear_support_active_set_state_code", None), code, "{at}");
                let reaction_row = row(&v, "nonlinear_support_final_reaction", Some(stop), Some("uy"), case);
                assert!(reaction_row["metadata"]["basis"].as_str().unwrap().ends_with(&format!("final_state={state}")), "{at}");
                let stop_uy = nl(stop, "nonlinear_support_final_displacement", Some("uy"));
                let stop_reaction = reaction_row["value"].as_f64().unwrap();
                if e.stop_active {
                    assert_eq!(stop_uy, 0.0, "{at}: an active stop holds UY");
                    close(stop_reaction, e.stop_reaction_n, &format!("{at} stop reaction"));
                    assert!(stop_reaction < 0.0, "{at}: the stop can only push in -Y");
                } else {
                    close(stop_uy, e.stop_uy_mm, &format!("{at} stop UY"));
                    assert!(stop_uy < 0.0, "{at}: a released stop lifts off in -Y");
                    zero(stop_reaction, e.applied_y_n, &format!("{at} released stop reaction"));
                }

                // NL-130-FRIC slides (code 3), its force opposing the slip.
                let shoe = "support:NL-130-FRIC";
                assert_eq!(nl(shoe, "nonlinear_support_active_set_state_code", None), 3.0, "{at}");
                let slip = nl(shoe, "nonlinear_support_final_displacement", Some("uz"));
                let friction = nl(shoe, "nonlinear_support_final_reaction", Some("uz"));
                close(slip, e.slip_mm, &format!("{at} slip"));
                assert!(friction * slip < 0.0, "{at}: friction opposes the slip");
                assert!((friction - e.friction_n).abs() <= 1e-9 * e.normal_n, "{at} friction: observed {friction:.17e} expected {:.17e}", e.friction_n);

                // The derived normal is |S-130 UY| of the same state: the frame share
                // while the stop holds, the whole applied Y load once it releases.
                let normal_row = row(&v, "nonlinear_support_friction_normal_reaction_derived", Some(shoe), None, case);
                let normal = normal_row["value"].as_f64().unwrap();
                assert_eq!(normal_row["unit"], "N");
                let basis = normal_row["metadata"]["basis"].as_str().unwrap();
                for part in ["derived_support_reaction", "source_ref=support:S-130", "source_dof=uy"] {
                    assert!(basis.contains(part), "{at}: {basis}");
                }
                close(normal, e.normal_n, &format!("{at} normal"));
                assert!((friction + slip.signum() * MU * normal).abs() <= 1e-9 * normal, "{at}: Coulomb F = -sign(slip) mu N");
                let s130_fy = action(&v, "support:S-130", "Fy", case);
                close(s130_fy, e.s130_fy_n, &format!("{at} S-130 Fy"));
                assert_eq!(normal, value(&v, "support_reaction_force_magnitude_v2", Some("support:S-130"), None, case), "{at}");

                // Y balance: the anchor carries no Y force; S-130 and the stop carry the load.
                zero(action(&v, "support:S-100", "Fy", case), e.applied_y_n, &format!("{at} anchor Fy"));
                if !e.stop_active {
                    close(normal, e.applied_y_n.abs(), &format!("{at} released normal"));
                }
                close(action(&v, HANGER, "Fz", case), e.sh140_fz_n, &format!("{at} SH-140 Fz"));
            }
        }
    }
}

/// The frozen table, on the plain-spring twin; the twin is the bundled demo but
/// for SH-140's record, which keeps the hanger's id, node, DOF and rate.
#[test]
fn demo_friction_reverses_three_ways_and_the_stop_releases() {
    let (bundled, twin) = (demo(), plain_spring_twin());
    let keys = |m: &Value| m.as_object().unwrap().keys().cloned().collect::<Vec<_>>();
    assert_eq!(keys(&bundled), keys(&twin));
    for key in keys(&bundled).iter().filter(|k| *k != "supports") {
        assert_eq!(bundled[key], twin[key], "{key}");
    }
    let (a, b) = (bundled["supports"].as_array().unwrap(), twin["supports"].as_array().unwrap());
    assert_eq!(a.len(), b.len());
    let differing: Vec<&Value> = a.iter().zip(b).filter(|(x, y)| x != y).map(|(x, _)| &x["id"]).collect();
    assert_eq!(differing, [HANGER], "the twin differs in SH-140's record only");
    let (hanger, spring) = (a.iter().find(|s| s["id"] == HANGER).unwrap(), b.iter().find(|s| s["id"] == HANGER).unwrap());
    assert_eq!(hanger["family"], "variable_spring_hanger");
    assert_eq!((&spring["node"], &spring["stiffness"]), (&hanger["node"], &hanger["hanger"]["stiffness"]));
    assert!(spring.get("hanger").is_none() && hanger["hanger"]["cold_load"]["value"].as_f64().unwrap() > 0.0);
    friction_reverses_and_the_stop_releases(&twin, &EXPECTED, "plain-spring twin");
}

/// The successor package's six scenarios, bound by its sums, with exactly the
/// fields of `EXPECTED` (N, mm, support actions on the structure).
fn under_the_law() -> (Vec<Expected>, Value) {
    use sha2::{Digest, Sha256};
    let digest = format!("{:x}", Sha256::digest(UNDER_THE_LAW.as_bytes()));
    let line = UNDER_THE_LAW_SUMS.lines().find(|l| l.ends_with("  demo_friction_reversal_preload.json")).unwrap();
    assert_eq!(&line[..64], digest, "the package's sums bind its scenarios");
    let package: Value = serde_json::from_str(UNDER_THE_LAW).unwrap();
    assert!(package["crosscheck"].as_array().unwrap().iter().all(|c| c["agrees"] == true));
    let number = |row: &Value, key: &str| row[key].as_f64().unwrap_or_else(|| panic!("{key}"));
    let rows = package["rows"].as_array().unwrap().iter().map(|row| Expected {
        case: match row["case"].as_str().unwrap() {
            "load:L-100" => "load:L-100",
            "load:L-200" => "load:L-200",
            other => panic!("{other}"),
        },
        variant: match row["variant"].as_str().unwrap() {
            "Original" => Variant::Original,
            "ReverseZ" => Variant::ReverseZ,
            "ReverseAll" => Variant::ReverseAll,
            other => panic!("{other}"),
        },
        applied_y_n: number(row, "applied_y_n"),
        stop_active: row["stop_active"].as_bool().unwrap(),
        stop_uy_mm: number(row, "stop_uy_mm"),
        stop_reaction_n: number(row, "stop_reaction_n"),
        slip_mm: number(row, "slip_mm"),
        friction_n: number(row, "friction_n"),
        normal_n: number(row, "normal_n"),
        s130_fy_n: number(row, "s130_fy_n"),
        sh140_fz_n: number(row, "sh140_fz_n"),
    }).collect::<Vec<_>>();
    assert_eq!(rows.len(), 6);
    (rows, package)
}

/// The bundled demo under the law: the successor package's scenarios, and in
/// each SH-140's applied force equal to the package's (its entered cold load).
#[test]
fn the_bundled_demo_reverses_under_the_preload_law() {
    let (rows, package) = under_the_law();
    let model = demo();
    friction_reverses_and_the_stop_releases(&model, &rows, "bundled demo");
    for mode in MODES {
        for variant in [Variant::Original, Variant::ReverseZ, Variant::ReverseAll] {
            let v = solve(&model, variant, mode);
            for note in package["notes"].as_array().unwrap().iter().filter(|n| n["variant"] == format!("{variant:?}")) {
                let case = note["case"].as_str().unwrap();
                let applied = value(&v, "variable_spring_hanger_applied_load", Some(HANGER), None, case);
                assert_eq!(applied, note["sh140_applied_force_n"].as_f64().unwrap(), "{mode:?} {variant:?} {case}");
            }
        }
    }
}
