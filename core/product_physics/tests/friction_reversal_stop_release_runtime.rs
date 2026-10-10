//! Three-way friction reversal with stop release on the bundled demo model
//! (`fixtures/product_preview/invented_demo_model.json`: joint-free and
//! pressure-free), through the captured value entry the desktop uses, in both
//! solver modes. Each load case is solved as authored (`Original`), with its
//! line weight reversed (`ReverseZ`: the rack shoe NL-130-FRIC slides the other
//! way, the stop NL-140 stays active) and with every primitive reversed
//! (`ReverseAll`: the Y load lifts N-140 off the one-way stop, which releases,
//! and the derived friction normal grows from the frame share to the whole Y load).
//!
//! The expectations are an independent reference, not product output: an exact
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

/// The demo with the variant's primitive loads negated; nothing else changes.
fn demo_request(variant: Variant) -> Value {
    let mut model: Value = serde_json::from_str(DEMO).unwrap();
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

fn solve(variant: Variant, mode: PreviewSolverMode) -> Value {
    let envelope = run_linear_static_preview_value_with_mode(demo_request(variant), mode).expect("captured entry");
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

#[test]
fn demo_friction_reverses_three_ways_and_the_stop_releases() {
    for mode in MODES {
        for variant in [Variant::Original, Variant::ReverseZ, Variant::ReverseAll] {
            let v = solve(variant, mode);
            assert!(!v["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "NONLINEAR_SUPPORT_LOOP_BLOCKED"));
            for e in EXPECTED.iter().filter(|e| e.variant == variant) {
                let (case, at) = (e.case, format!("{mode:?} {variant:?} {}", e.case));
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

                // NL-130-FRIC slides (code 3); slip and force reverse with the Z load.
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
                close(action(&v, "support:SH-140", "Fz", case), e.sh140_fz_n, &format!("{at} SH-140 Fz"));
            }
        }
    }
}
