//! Direct PHYS-R4 publisher and extrema regression; no public solve is run.
//! Original source inputs and request wiring are retained from the historic
//! pressure_membrane_range public test. Zero mechanical/thermal actions follow
//! from the fixed-zero-displacement, no-primitive-load constitutive reference;
//! they are explicitly supplied here, not obtained from a bypassed solver.
//!
//! Independent reference: PHYSICS_MANAGER/INDEPENDENT_REVIEW/
//! FREEZE_03_BACKCHECK/independent_arithmetic_probe.json, SHA-256
//! 23db606adf7fe61f4df6031ac6b11b8285b88b12d6dfe381a04faaabd13c349c.
//! For represented OD, wall, nu and p, Fraction algebra evaluates
//! 2*nu*p*(OD/2-wall)^2 / (wall*(OD-wall)). Pi cancels.
//! This reference was independently backchecked in MEMBRANE_BACKCHECK.
//! No NUM guard, historical mode, exact-boundary solver or tolerance is changed.

use super::*;
use serde_json::{json, Value};

const CASE: &str = "case:source-section";
const REGION: &str = "region:source-section";
const PIPE: &str = "pipe:source-section";
const A: &str = "node:section-a";
const B: &str = "node:section-b";

const EXPECTED_MEMBRANE_PA: f64 = 3.1333333333333337e-171;

fn anchor(id: &str, node: &str) -> Value {
    json!({"id":id,"node":node,"family":"anchor",
        "restraints":["UX","UY","UZ","RX","RY","RZ"],
        "provenance":"independent_section_geometry_control"})
}

fn fixture(inputs: &Value, fixed: bool, pressurized: bool) -> Value {
    let mut supports = vec![anchor("support:section-a", A)];
    if fixed {
        supports.push(anchor("support:section-b", B));
    }
    let mut loads = Vec::new();
    for (id, direction, dimension, unit, magnitude) in [
        (
            "load:tip-y",
            "global_y",
            "force",
            "N",
            inputs["tip_Fy_N"].as_f64().unwrap_or(0.0),
        ),
        (
            "load:tip-torque",
            "rotation_x",
            "moment",
            "N*m",
            inputs["tip_Mx_Nm"].as_f64().unwrap_or(0.0),
        ),
    ] {
        if magnitude != 0.0 {
            loads.push(json!({"id":id,"category":if dimension=="moment" {"concentrated_moment"} else {"concentrated_force"},
                "target":{"type":"node","node":B},"direction":direction,
                "dimension":dimension,"magnitude":{"value":magnitude,"unit":unit},
                "provenance":"independent_section_geometry_reference"}));
        }
    }
    let regions = if pressurized {
        vec![json!({"id":REGION,"member_pipe_ids":[PIPE],
            "pressure_basis":"internal_differential_zero_external_v1",
            "pressure":{"value":inputs["p_Pa"],"unit":"Pa"},
            "terminals":[
                {"node_ref":A,"closure_transfer":"transfers_to_wall","provenance":"explicit_test_closure"},
                {"node_ref":B,"closure_transfer":"transfers_to_wall","provenance":"explicit_test_closure"}],
            "provenance":"independent_geometry_pressure_reference"})]
    } else {
        vec![]
    };
    json!({"model":{
        "schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model",
        "pressure_contract":{"version":"3.0.0","mode":"exact_pressure_v3","shear_model":"euler_bernoulli"},
        "project":{"id":"project:section-oracle","units":{"length":"m","force":"N","angle":"rad",
            "pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics",
            "rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[{"id":A,"position":{"x":0.0,"y":0.0,"z":0.0},"provenance":"synthetic"},
            {"id":B,"position":{"x":inputs["L_m"],"y":0.0,"z":0.0},"provenance":"synthetic"}],
        "pipe_segments":[{"id":PIPE,"from":A,"to":B,"section":{
            "outside_diameter":{"value":inputs["OD_m"],"unit":"m"},
            "wall_thickness":{"value":inputs["wall_m"],"unit":"m"},
            "corrosion_allowance":{"value":0.0,"unit":"m"}},
            "material":"material:section","y_reference":{"x":0.0,"y":1.0,"z":0.0},
            "provenance":"arithmetic_geometry_control_not_manufactured_pipe"}],
        "materials":[{"id":"material:section","constitutive_basis":"homogeneous_isotropic_E_nu_v1",
            "elastic_modulus":{"value":inputs["E_Pa"],"unit":"Pa"},
            "poisson_ratio":{"value":inputs["nu"],"unit":"1"},
            "provenance":"synthetic_isotropic_input"}],
        "supports":supports,"components":[],
        "load_cases":[{"id":CASE,"primitive_loads":loads,"pressure_regions":regions,
            "provenance":"independent_reference"}],"combinations":[]
    },"materials":[]})
}

fn prepared_original_fixture() -> (
    PreviewModel,
    BuiltModel,
    pressure_runtime::ExactPressureCase,
) {
    prepared(json!({"OD_m":4e-77,"wall_m":1e-77,"L_m":1.0,
        "E_Pa":1.0,"nu":0.1,"p_Pa":4.7e-170}))
}

fn assert_original_reference(actual: f64, label: &str) {
    assert!(actual.is_finite(), "{label}: {actual:?}");
    let tolerance = 1e-9 * EXPECTED_MEMBRANE_PA.abs();
    assert!((actual - EXPECTED_MEMBRANE_PA).abs() <= tolerance,
        "{label}: actual={actual:.17e}; independent reference={EXPECTED_MEMBRANE_PA:.17e}; tolerance={tolerance:.17e}");
}

#[test]
fn original_subnormal_wall_force_direct_publisher_preserves_membrane() {
    let (model, built, pressure) = prepared_original_fixture();
    let state = pressure.pipe_states.get(&0).unwrap();
    let case = &model.load_cases[0];
    let mechanical_actions = [0.0; ELEMENT_DOF];
    // Independent Fraction witness rounds the tension-positive wall force to
    // binary64 bits 1. End actions have opposite node-on-element signs.
    let mut wall_actions = [0.0; ELEMENT_DOF];
    wall_actions[UX] = -f64::from_bits(1);
    wall_actions[DOF_PER_NODE + UX] = f64::from_bits(1);
    let mut rows = Vec::new();
    let mut diagnostics = Vec::new();
    append_exact_pressure_results(
        &mut rows, &mut diagnostics, case, PIPE, state, &wall_actions,
        Some(&mechanical_actions), &built.pipes[0], &[], None, None,
    );
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    assert!(rows.iter().all(|r| r.value.is_finite()));
    for location in ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"] {
        for (kind, component, unit) in [
            ("pipe_wall_axial_force_v2", "wall_axial_force", "N"),
            ("pipe_axial_membrane_stress_v2", "axial_membrane_stress", "Pa"),
        ] {
            let matching: Vec<_> = rows.iter().filter(|r| {
                r.kind == kind && r.entity_ref == PIPE
                    && r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case" && b.ref_id == CASE)
                    && r.metadata.as_ref().is_some_and(|m| m.component == component && m.location == location)
            }).collect();
            if unit == "N" {
                assert_eq!(matching.len(), 1, "{kind} {location}: {rows:?}");
                assert_eq!(matching[0].unit, unit);
                assert_eq!(matching[0].value.to_bits(), 1, "{location}: independent projected force");
            } else {
                // D-5 (DEL-05-03 Design §2.1): the membrane stress is a stress
                // row, published on the nominal section and then on the reduced
                // one; this section has no mill tolerance or allowance, so both
                // divide the one wall force by the same area.
                assert_eq!(matching.len(), 2, "{kind} {location}: {rows:?}");
                for (row, basis) in matching.iter().zip(crate::pressure_exact::SectionBasis::BOTH) {
                    assert_eq!(row.unit, unit);
                    assert!(row.metadata.as_ref().unwrap().basis.ends_with(basis.basis_suffix()), "{location}: {row:?}");
                    assert_original_reference(row.value, location);
                }
                assert_eq!(matching[0].value.to_bits(), matching[1].value.to_bits(), "{location}: coincident bases");
            }
        }
    }
    // The old redivision is deliberately outside the unchanged tolerance;
    // this establishes that the assertion is sensitive to the original defect.
    let redivided = f64::from_bits(1) / state.sections.nominal().wall_area_m2();
    assert!((redivided / EXPECTED_MEMBRANE_PA - 1.0).abs() > 0.67);
}

#[test]
fn original_subnormal_wall_force_direct_extrema_preserves_membrane() {
    let (_, built, pressure) = prepared_original_fixture();
    let state = pressure.pipe_states.get(&0).unwrap();
    let mechanical_actions = [0.0; ELEMENT_DOF];
    let maximum = exact_straight_summary_extrema(
        &built.pipes[0], &mechanical_actions, &[], built.sections.get(PIPE).unwrap(),
        Some((state, pressure_exact::SectionBasis::Nominal)),
    ).unwrap();
    // Invoke the actual corrected extrema coefficient path, not just the
    // constitutive helper. Constant pressure and zero bending make every
    // station carry the independent membrane maximum.
    //
    // K-D5 (DEL-04-01 kd5-member-relative-bound.md §3.2, "Fallback arithmetic
    // transport"): this noncertified maximum now replays the binary64 recipe
    // with an absolute quantum per rounding (u*(magnitude+error) +
    // min_subnormal), propagated through the stored areas. At this subnormal
    // wall force each quantum is about 5e-171 Pa, so the truthful enclosure is
    // [0, 4.2e-169] Pa, about 135 times the reference, and its midpoint no
    // longer equals the membrane stress. The unchanged absolute gap (1e-12 Pa)
    // is met, so the maximum stays available. The pin is now containment of
    // the independent reference; the membrane rows themselves stay pinned
    // exactly by the publisher test above, which keeps the sensitivity to the
    // redivision defect, and the extrema path's sensitivity is kept by the
    // unclamped control (`unclamped_subnormal_wall_force_extrema_*`), whose
    // enclosure stays clear of zero so its public midpoint decides.
    assert!(maximum.value_lower <= EXPECTED_MEMBRANE_PA, "{maximum:?}");
    assert!(EXPECTED_MEMBRANE_PA <= maximum.value_upper, "{maximum:?}");
    assert_eq!(maximum.value_upper, maximum.upper_bound, "{maximum:?}");
    assert!(maximum.certified_gap <= 1e-12 + 1e-12 * maximum.value_lower, "{maximum:?}");
    assert!(maximum.value_lower <= maximum.value_upper);
    assert!(maximum.value_upper <= maximum.upper_bound);
    assert!((0.0..=1.0).contains(&maximum.station));
    // No envelope, load-case summary, public admission, solution error or
    // complete solver-to-publisher wiring claim is made by this direct test.
}

/// The fixed, pressurized fixture prepared to the lower-level subject. This
/// preparation reaches the subject directly: it does not invoke
/// solve_load_case, select a solver, or produce a solved envelope.
fn prepared(inputs: Value) -> (PreviewModel, BuiltModel, pressure_runtime::ExactPressureCase) {
    let request: LinearStaticPreviewRequest = serde_json::from_value(fixture(&inputs, true, true)).unwrap();
    let mut model = request.model;
    let mut materials = model.materials.clone();
    let mut diagnostics = Vec::new();
    pressure_runtime::validate_profile(&model, &mut diagnostics);
    validate_model_inputs(&model, &materials, &mut diagnostics);
    validate_support_family_tokens(&model, &mut diagnostics);
    resolve_shared_sections(&mut model, &mut diagnostics);
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    let built = build_model(&model, &materials, &mut diagnostics).unwrap();
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    assert_eq!(built.pipes.len(), 1);
    let pressure = pressure_runtime::build_pressure_case(&model, &built, &materials, &model.load_cases[0], &mut diagnostics).unwrap();
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    assert_eq!(pressure.pipe_states.len(), 1);
    (model, built, pressure)
}

/// Independent reference of the unclamped control: Fraction algebra of
/// 2*nu*p*(OD/2-wall)^2 / (wall*(OD-wall)) for OD = 4e-77 m, wall = 1e-77 m,
/// nu = 0.1, p = 4.7e-164 Pa (binary64 inputs), rounded to nearest.
const UNCLAMPED_MEMBRANE_PA: f64 = 3.133333333333333e-165;

/// Review finding F7(b): a normal-stress control on the same extrema path,
/// where the redivision defect is still visible. At p = 4.7e-164 Pa the
/// pressure wall force is subnormal (about 3e-318 N, some 20 significant
/// bits), so redividing its rounding by the wall area misses the membrane
/// stress by about 8e-7 relative, while the stress itself is normal and the
/// fallback enclosure (about 3e-4 relative) stays clear of zero. The public
/// row value, the enclosure midpoint (kd5-member-relative-bound.md §3.2),
/// is then the membrane stress to within the unchanged 1e-9, which a
/// redivided coefficient would miss by about 800 times that.
#[test]
fn unclamped_subnormal_wall_force_extrema_midpoint_is_the_membrane() {
    let (_, built, pressure) = prepared(json!({"OD_m":4e-77,"wall_m":1e-77,"L_m":1.0,
        "E_Pa":1.0,"nu":0.1,"p_Pa":4.7e-164}));
    let state = pressure.pipe_states.get(&0).unwrap();
    let mechanical_actions = [0.0; ELEMENT_DOF];
    let maximum = exact_straight_summary_extrema(
        &built.pipes[0], &mechanical_actions, &[], built.sections.get(PIPE).unwrap(),
        Some((state, pressure_exact::SectionBasis::Nominal)),
    ).unwrap();
    let reference = UNCLAMPED_MEMBRANE_PA;
    // Preconditions (paths differ): the wall force is subnormal and its
    // redivision misses the reference by far more than the tolerance; the
    // enclosure is unclamped and narrow enough for its midpoint to decide.
    let wall = state
        .sections
        .recover_wall_effective(0.0, state.material, state.pressure)
        .unwrap()
        .wall_n;
    assert!(wall > 0.0 && wall < f64::MIN_POSITIVE, "{wall:e}");
    let redivided = wall / state.sections.nominal().wall_area_m2();
    assert!((redivided / reference - 1.0).abs() > 1e-7, "{redivided:e}");
    assert!(maximum.value_lower > 0.0, "{maximum:?}");
    assert!((maximum.upper_bound - maximum.value_lower) < 1e-3 * reference, "{maximum:?}");
    // The pins: containment, the public midpoint, the current reader rules.
    assert!(maximum.value_lower <= reference && reference <= maximum.value_upper, "{maximum:?}");
    assert_eq!(maximum.value_upper, maximum.upper_bound, "{maximum:?}");
    let midpoint = maximum.value_lower + 0.5 * (maximum.upper_bound - maximum.value_lower);
    assert!(
        (midpoint - reference).abs() <= 1e-9 * reference,
        "midpoint={midpoint:.17e}; independent reference={reference:.17e}"
    );
    assert!(maximum.certified_gap <= 1e-12 + 1e-12 * maximum.value_lower, "{maximum:?}");
    assert!((0.0..=1.0).contains(&maximum.station));
}

