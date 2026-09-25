//! Independent public-entrypoint pressure assembly oracles.
//!
//! Expectations were frozen before reading the new pressure implementation:
//! PHYSICS_MANAGER/ASSEMBLY_ORACLE/FROZEN_RUNTIME_EXPECTATIONS.json.
//! No production pressure, section, assembly or stress helper is an oracle here.

use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, MechanicsEnvelope,
    PreviewSolverMode, ResultItem,
};
use serde_json::{json, Value};
use std::f64::consts::PI;

const CASE: &str = "case:pressure-oracle";
const REGION: &str = "region:pressure-oracle";
const PIPE: &str = "pipe:A-B";
const ROOT: &str = "node:A";
const TIP: &str = "node:B";
const ROOT_SUPPORT: &str = "support:A";
const TIP_SUPPORT: &str = "support:B";
const AS: f64 = 0.0011 * PI;
const EA: f64 = 220_000_000.0 * PI;
const P: f64 = 5000.0 * PI;
const LENGTH: f64 = 6.0;
const STATIONS: [(&str, f64); 5] = [
    ("end_i", 0.0),
    ("quarter_1", 0.25),
    ("midspan", 0.5),
    ("quarter_3", 0.75),
    ("end_j", 1.0),
];
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];

fn close(actual: f64, expected: f64, zero_scale: f64, context: &str) {
    assert!(actual.is_finite(), "{context}: nonfinite {actual}");
    let tolerance = 1.0e-9
        * if expected == 0.0 {
            zero_scale.abs()
        } else {
            expected.abs()
        };
    assert!(
        (actual - expected).abs() <= tolerance,
        "{context}: actual={actual:.17e}; expected={expected:.17e}; tolerance={tolerance:.17e}"
    );
}

fn node(id: &str, position: [f64; 3]) -> Value {
    json!({"id":id,"position":{"x":position[0],"y":position[1],"z":position[2]},
        "provenance":"independent_pressure_runtime_oracle"})
}

fn anchor(id: &str, node_id: &str) -> Value {
    json!({"id":id,"node":node_id,"family":"anchor",
        "restraints":["UX","UY","UZ","RX","RY","RZ"],
        "provenance":"independent_pressure_runtime_oracle"})
}

fn terminal(node_id: &str, transfer: bool) -> Value {
    json!({"node_ref":node_id,"closure_transfer":if transfer {"transfers_to_wall"}
        else {"separately_supported_or_compensated"},
        "provenance":"explicit_independent_pressure_boundary"})
}

fn material(id: &str, e: f64, nu: f64) -> Value {
    json!({"id":id,"constitutive_basis":"homogeneous_isotropic_E_nu_v1",
        "elastic_modulus":{"value":e,"unit":"Pa"},
        "poisson_ratio":{"value":nu,"unit":"1"},
        "thermal_expansion_coefficient":{"value":0.000012,"unit":"1/degC"},
        "temperature_points":[],"provenance":"synthetic_isotropic_reference_no_catalog"})
}

fn pipe(id: &str, from: &str, to: &str, outside: f64, wall: f64, material_id: &str) -> Value {
    json!({"id":id,"from":from,"to":to,
        "section":{"outside_diameter":{"value":outside,"unit":"m"},
            "wall_thickness":{"value":wall,"unit":"m"}},
        "material":material_id,"y_reference":{"x":0.0,"y":0.0,"z":1.0},
        "provenance":"synthetic_straight_annulus"})
}

fn model(fixed: bool, transfer: bool, thermal: f64) -> Value {
    let mut supports = vec![anchor(ROOT_SUPPORT, ROOT)];
    if fixed {
        supports.push(anchor(TIP_SUPPORT, TIP));
    }
    let loads = if thermal == 0.0 {
        vec![]
    } else {
        vec![json!({"id":"load:thermal","category":"thermal",
            "target":{"type":"element","pipe":PIPE},"direction":"global_x",
            "magnitude":{"value":thermal/0.000012,"unit":"degC"},
            "dimension":"temperature_interval","provenance":"independent_thermal_strain"})]
    };
    json!({"model":{
        "schema_version":"0.3.0","document_kind":"openpipestress.product_preview.model",
        "pressure_contract":{"version":"2.0.0","mode":"exact_straight_pressure_v2"},
        "project":{"id":"project:pressure-oracle","units":{"length":"m","force":"N",
            "angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics",
            "rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[node(ROOT,[0.0,0.0,0.0]),node(TIP,[LENGTH,0.0,0.0])],
        "pipe_segments":[pipe(PIPE,ROOT,TIP,0.12,0.01,"material:one")],
        "supports":supports,"components":[],
        "materials":[material("material:one",200e9,0.3)],
        "load_cases":[{"id":CASE,"primitive_loads":loads,"pressure_regions":[{
            "id":REGION,"member_pipe_ids":[PIPE],
            "pressure_basis":"internal_differential_zero_external_v1",
            "pressure":{"value":2e6,"unit":"Pa"},
            "terminals":[terminal(ROOT,transfer),terminal(TIP,transfer)],
            "provenance":"independent_pressure_region"}],
            "provenance":"synthetic_load_case"}],"combinations":[]
    },"materials":[]})
}

fn solve(input: Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    let request: LinearStaticPreviewRequest = serde_json::from_value(input)
        .expect("the independently authored public DTO must deserialize");
    run_linear_static_preview_with_mode(request, mode)
}

fn solved(result: &MechanicsEnvelope) {
    assert_eq!(
        result.status.mechanics, "MECHANICS_SOLVED",
        "public entrypoint must solve the supported reference: {:?}",
        result.diagnostics
    );
    assert!(!result.accepted_model_state_mutated);
    assert!(result.results.iter().all(|r| r.value.is_finite()));
    assert!(
        !result
            .diagnostics
            .iter()
            .any(|d| d.severity == "blocking" || d.severity == "failure"),
        "successful reference carries blocking diagnostics: {:?}",
        result.diagnostics
    );
}

fn row<'a>(
    result: &'a MechanicsEnvelope,
    kind: &str,
    entity: &str,
    component: &str,
    location: Option<&str>,
) -> &'a ResultItem {
    let matching: Vec<_> = result
        .results
        .iter()
        .filter(|r| {
            r.kind == kind
                && r.entity_ref == entity
                && r.metadata.as_ref().is_some_and(|m| {
                    m.component == component && location.is_none_or(|loc| m.location == loc)
                })
        })
        .collect();
    assert_eq!(
        matching.len(),
        1,
        "expected one real row kind={kind}, entity={entity}, component={component}, location={location:?}; candidates={:?}",
        result.results.iter().filter(|r| r.entity_ref == entity).collect::<Vec<_>>()
    );
    matching[0]
}

fn displacement(result: &MechanicsEnvelope, node_id: &str, vector: [f64; 3], scale_m: f64) {
    for (axis, expected) in ["x", "y", "z"].iter().zip(vector) {
        let r = row(
            result,
            &format!("global_nodal_displacement_{axis}"),
            node_id,
            &format!("nodal_displacement_{axis}"),
            None,
        );
        assert_eq!(r.unit, "mm");
        close(
            r.value / 1000.0,
            expected,
            scale_m,
            &format!("{node_id} displacement {axis}"),
        );
        let rotation = row(
            result,
            &format!("global_nodal_rotation_{axis}"),
            node_id,
            &format!("nodal_rotation_{axis}"),
            None,
        );
        assert_eq!(rotation.unit, "rad");
        close(
            rotation.value,
            0.0,
            scale_m / LENGTH,
            &format!("{node_id} rotation {axis}"),
        );
    }
}

fn support(result: &MechanicsEnvelope, id: &str, force: [f64; 3], force_scale: f64) {
    for (component, expected) in ["Fx", "Fy", "Fz"].into_iter().zip(force) {
        let r = row(
            result,
            "support_reaction_component_v2",
            id,
            component,
            Some("node"),
        );
        assert_eq!(r.unit, "N");
        assert_eq!(r.metadata.as_ref().unwrap().coordinate_system, "global");
        close(r.value, expected, force_scale, &format!("{id} {component}"));
    }
    for component in ["Mx", "My", "Mz"] {
        let r = row(
            result,
            "support_reaction_component_v2",
            id,
            component,
            Some("node"),
        );
        assert_eq!(r.unit, "N*m");
        assert_eq!(r.metadata.as_ref().unwrap().coordinate_system, "global");
        close(
            r.value,
            0.0,
            force_scale * LENGTH,
            &format!("{id} {component}"),
        );
    }
}

fn pressure_row(
    result: &MechanicsEnvelope,
    kind: &str,
    pipe_id: &str,
    component: &str,
    location: &str,
    expected: f64,
    scale: f64,
) {
    let r = row(result, kind, pipe_id, component, Some(location));
    let stress = kind.contains("stress");
    assert_eq!(r.unit, if stress { "Pa" } else { "N" });
    assert_eq!(
        r.metadata.as_ref().unwrap().coordinate_system,
        if stress {
            "pipe_section"
        } else {
            "element_local"
        }
    );
    let basis = r
        .basis_ref
        .as_ref()
        .expect("new physical rows retain their actual case identity");
    assert_eq!(basis.ref_type, "load_case");
    assert_eq!(basis.ref_id, CASE);
    close(
        r.value,
        expected,
        scale,
        &format!("{pipe_id} {component} {location}"),
    );
}

fn pressure_fields(
    result: &MechanicsEnvelope,
    pipe_id: &str,
    wall_at_fraction: impl Fn(f64) -> f64,
    p: f64,
    ri: f64,
    ro: f64,
    force_scale: f64,
) {
    let area = PI * (ro * ro - ri * ri);
    // Frozen common bore is exactly 0.05 m: keep the analytical area coefficient
    // identical to P's independent definition, so exact zero S is not replaced
    // by cancellation noise from a different floating expression.
    assert_eq!(ri, 0.05);
    let cap = p * 0.0025 * PI;
    let hoop_inner = p * (ro * ro + ri * ri) / (ro * ro - ri * ri);
    let hoop_outer = p * 2.0 * ri * ri / (ro * ro - ri * ri);
    for (location, fraction) in STATIONS {
        let wall = wall_at_fraction(fraction);
        for (kind, component, expected, scale) in [
            (
                "pipe_wall_axial_force_v2",
                "wall_axial_force",
                wall,
                force_scale,
            ),
            (
                "pipe_effective_axial_force_v2",
                "effective_axial_force",
                wall - cap,
                force_scale,
            ),
            (
                "pipe_axial_membrane_stress_v2",
                "axial_membrane_stress",
                wall / area,
                force_scale / area,
            ),
            (
                "pipe_lame_radial_stress_v2",
                "lame_inner_radial_stress",
                -p,
                force_scale / area,
            ),
            (
                "pipe_lame_radial_stress_v2",
                "lame_outer_radial_stress",
                0.0,
                force_scale / area,
            ),
            (
                "pipe_lame_hoop_stress_v2",
                "lame_inner_hoop_stress",
                hoop_inner,
                force_scale / area,
            ),
            (
                "pipe_lame_hoop_stress_v2",
                "lame_outer_hoop_stress",
                hoop_outer,
                force_scale / area,
            ),
        ] {
            pressure_row(result, kind, pipe_id, component, location, expected, scale);
        }
    }
    pressure_row(
        result,
        "pipe_wall_endpoint_action_v2",
        pipe_id,
        "wall_axial_end_action",
        "end_i",
        -wall_at_fraction(0.0),
        force_scale,
    );
    pressure_row(
        result,
        "pipe_wall_endpoint_action_v2",
        pipe_id,
        "wall_axial_end_action",
        "end_j",
        wall_at_fraction(1.0),
        force_scale,
    );
    for r in result.results.iter().filter(|r| r.entity_ref == pipe_id) {
        assert!(
            r.kind != "element_local_axial_force"
                && r.kind != "element_local_axial_normal_stress"
                && !r.kind.contains("pressure_longitudinal")
                && !r.kind.contains("pressure_hoop"),
            "exact member leaked competing legacy pressure/axial semantics: {r:?}"
        );
    }
}

fn region_evidence(result: &MechanicsEnvelope, members: &[&str]) {
    let pressure = result
        .contract_evidence
        .as_ref()
        .expect("bound normalized pressure evidence")["pressure"]
        .as_array()
        .expect("pressure evidence array");
    let entry = pressure
        .iter()
        .find(|p| p["region_id"] == REGION && p["load_case_id"] == CASE)
        .expect("region and load case jointly identify actual normalized inputs");
    for member in members {
        assert!(entry["member_pipe_ids"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v == member));
    }
    let ids = entry["result_ids"]
        .as_array()
        .expect("region evidence binds actual pressure rows");
    for r in result.results.iter().filter(|r| pressure_kind(&r.kind)) {
        assert!(
            ids.iter().any(|id| id == &r.id),
            "pressure row absent from region mapping: {}",
            r.id
        );
    }
}

fn pressure_kind(kind: &str) -> bool {
    matches!(
        kind,
        "pipe_wall_endpoint_action_v2"
            | "pipe_wall_axial_force_v2"
            | "pipe_effective_axial_force_v2"
            | "pipe_axial_membrane_stress_v2"
            | "pipe_lame_radial_stress_v2"
            | "pipe_lame_hoop_stress_v2"
    )
}

fn case_row<'a>(
    result: &'a MechanicsEnvelope,
    case_id: &str,
    kind: &str,
    entity: &str,
    component: &str,
    location: Option<&str>,
) -> &'a ResultItem {
    let matching: Vec<_> = result
        .results
        .iter()
        .filter(|r| {
            r.kind == kind
                && r.entity_ref == entity
                && r.basis_ref
                    .as_ref()
                    .is_some_and(|b| b.ref_type == "load_case" && b.ref_id == case_id)
                && r.metadata.as_ref().is_some_and(|m| {
                    m.component == component && location.map_or(true, |l| m.location == l)
                })
        })
        .collect();
    assert_eq!(matching.len(),1,"case-qualified real row missing/ambiguous: {case_id} {kind} {entity} {component} {location:?}");
    matching[0]
}

#[test]
fn six_si_pressure_states_through_both_public_solver_modes() {
    for mode in MODES {
        for (fixed, transfer, thermal, extension, wall, effective, reaction) in [
            (false, true, 0.0, 3.0 / 55000.0, P, 0.0, 0.0),
            (true, true, 0.0, 0.0, 3000.0 * PI, -2000.0 * PI, 2000.0 * PI),
            (false, false, 0.0, -9.0 / 110000.0, 0.0, -P, 0.0),
            (
                true,
                false,
                0.0,
                0.0,
                3000.0 * PI,
                -2000.0 * PI,
                -3000.0 * PI,
            ),
            (false, true, 0.0012, 399.0 / 55000.0, P, 0.0, 0.0),
            (
                true,
                true,
                0.0012,
                0.0,
                -261000.0 * PI,
                -266000.0 * PI,
                266000.0 * PI,
            ),
        ] {
            let result = solve(model(fixed, transfer, thermal), mode);
            solved(&result);
            let force_scale = P.abs().max((EA * thermal).abs());
            let displacement_scale = LENGTH * thermal.abs().max(P / EA);
            displacement(&result, TIP, [extension, 0.0, 0.0], displacement_scale);
            pressure_fields(&result, PIPE, |_| wall, 2e6, 0.05, 0.06, force_scale);
            close(
                wall - P,
                effective,
                force_scale,
                "frozen effective-force algebra",
            );
            support(&result, ROOT_SUPPORT, [reaction, 0.0, 0.0], force_scale);
            if fixed {
                support(&result, TIP_SUPPORT, [-reaction, 0.0, 0.0], force_scale);
            }
            region_evidence(&result, &[PIPE]);
        }
    }
}

#[test]
fn exact_pressure_rotates_as_vectors_and_normalizes_mm_mpa_inputs() {
    let t = [1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0];
    for mode in MODES {
        for fixed in [false, true] {
            for (millimetres, pressure_value, pressure_unit) in [
                (false, 2.0e6, "Pa"),
                (false, 2000.0, "kPa"),
                (true, 2.0, "MPa"),
            ] {
                let mut input = model(fixed, true, 0.0);
                let length_scale = if millimetres { 1000.0 } else { 1.0 };
                input["model"]["nodes"][1]["position"] = json!({"x":LENGTH*t[0]*length_scale,"y":LENGTH*t[1]*length_scale,"z":LENGTH*t[2]*length_scale});
                if millimetres {
                    input["model"]["project"]["units"]["length"] = json!("mm");
                    input["model"]["project"]["units"]["pressure"] = json!("MPa");
                    input["model"]["project"]["units"]["stress"] = json!("MPa");
                    input["model"]["pipe_segments"][0]["section"] = json!({
                        "outside_diameter":{"value":120.0,"unit":"mm"},
                        "wall_thickness":{"value":10.0,"unit":"mm"}});
                    input["model"]["materials"][0]["elastic_modulus"] =
                        json!({"value":200000.0,"unit":"MPa"});
                    input["model"]["load_cases"][0]["pressure_regions"][0]["pressure"] =
                        json!({"value":2.0,"unit":"MPa"});
                }
                input["model"]["load_cases"][0]["pressure_regions"][0]["pressure"] =
                    json!({"value":pressure_value,"unit":pressure_unit});
                let result = solve(input, mode);
                solved(&result);
                let extension = if fixed { 0.0 } else { 3.0 / 55000.0 };
                displacement(&result, TIP, t.map(|v| v * extension), LENGTH * P / EA);
                let wall = if fixed { 3000.0 * PI } else { P };
                pressure_fields(&result, PIPE, |_| wall, 2e6, 0.05, 0.06, P);
                let reaction = if fixed { 2000.0 * PI } else { 0.0 };
                support(&result, ROOT_SUPPORT, t.map(|v| v * reaction), P);
                if fixed {
                    support(&result, TIP_SUPPORT, t.map(|v| -v * reaction), P);
                }
            }
        }
    }
}

fn two_span(
    fixed: bool,
    reverse_first: bool,
    reverse_second: bool,
    reverse_terminals: bool,
) -> Value {
    let mut input = model(false, true, 0.0);
    input["model"]["nodes"] = json!([
        node(ROOT, [0.0, 0.0, 0.0]),
        node(TIP, [3.0, 0.0, 0.0]),
        node("node:C", [6.0, 0.0, 0.0])
    ]);
    input["model"]["pipe_segments"] = json!([
        pipe(
            PIPE,
            if reverse_first { TIP } else { ROOT },
            if reverse_first { ROOT } else { TIP },
            0.12,
            0.01,
            "material:one"
        ),
        pipe(
            "pipe:B-C",
            if reverse_second { "node:C" } else { TIP },
            if reverse_second { TIP } else { "node:C" },
            0.13,
            0.015,
            "material:two"
        )
    ]);
    input["model"]["materials"] = json!([
        material("material:one", 200e9, 0.3),
        material("material:two", 100e9, 0.25)
    ]);
    if fixed {
        input["model"]["supports"]
            .as_array_mut()
            .unwrap()
            .push(anchor("support:C", "node:C"));
    }
    let region = &mut input["model"]["load_cases"][0]["pressure_regions"][0];
    region["member_pipe_ids"] = json!(["pipe:B-C", PIPE]);
    region["terminals"] = if reverse_terminals {
        json!([terminal("node:C", true), terminal(ROOT, true)])
    } else {
        json!([terminal(ROOT, true), terminal("node:C", true)])
    };
    input
}

#[test]
fn equal_bore_unequal_wall_and_e_chain_is_orientation_independent() {
    for mode in MODES {
        for fixed in [false, true] {
            for first in [false, true] {
                for second in [false, true] {
                    for terminals in [false, true] {
                        let result = solve(two_span(fixed, first, second, terminals), mode);
                        solved(&result);
                        let wall = if fixed { (427000.0 / 157.0) * PI } else { P };
                        let middle = if fixed {
                            -3.0 / 785000.0
                        } else {
                            3.0 / 110000.0
                        };
                        let tip = if fixed { 0.0 } else { 179.0 / 2530000.0 };
                        displacement(&result, TIP, [middle, 0.0, 0.0], 179.0 / 2530000.0);
                        displacement(&result, "node:C", [tip, 0.0, 0.0], 179.0 / 2530000.0);
                        pressure_fields(&result, PIPE, |_| wall, 2e6, 0.05, 0.06, P);
                        pressure_fields(&result, "pipe:B-C", |_| wall, 2e6, 0.05, 0.065, P);
                        let reaction = if fixed { (358000.0 / 157.0) * PI } else { 0.0 };
                        support(&result, ROOT_SUPPORT, [reaction, 0.0, 0.0], P);
                        if fixed {
                            support(&result, "support:C", [-reaction, 0.0, 0.0], P);
                        }
                        region_evidence(&result, &[PIPE, "pipe:B-C"]);
                    }
                }
            }
        }
    }
}

#[test]
fn mixed_closures_use_the_actual_cap_load_path_for_support_reactions() {
    for mode in MODES {
        for left_transfer in [false, true] {
            let mut input = model(false, true, 0.0);
            input["model"]["load_cases"][0]["pressure_regions"][0]["terminals"] =
                json!([terminal(ROOT, left_transfer), terminal(TIP, !left_transfer)]);
            let result = solve(input, mode);
            solved(&result);
            let (wall, extension, reaction) = if left_transfer {
                (0.0, -9.0 / 110000.0, P)
            } else {
                (P, 3.0 / 55000.0, -P)
            };
            pressure_fields(&result, PIPE, |_| wall, 2e6, 0.05, 0.06, P);
            displacement(&result, TIP, [extension, 0.0, 0.0], LENGTH * P / EA);
            support(&result, ROOT_SUPPORT, [reaction, 0.0, 0.0], P);
        }
    }
}

#[test]
fn distributed_axial_load_is_subtracted_separately_from_pressure_eigenload() {
    for mode in MODES {
        let mut input = model(false, true, 0.0);
        input["model"]["load_cases"][0]["primitive_loads"] = json!([{
            "id":"load:axial-distributed","category":"weight",
            "target":{"type":"element","pipe":PIPE},"direction":"global_x",
            "magnitude":{"value":100.0,"unit":"N/m"},"dimension":"force_per_length",
            "provenance":"independent_uniform_axial_mechanical_load"}]);
        let result = solve(input, mode);
        solved(&result);
        pressure_fields(&result, PIPE, |s| P + 600.0 * (1.0 - s), 2e6, 0.05, 0.06, P);
        displacement(
            &result,
            TIP,
            [3.0 / 55000.0 + 1800.0 / EA, 0.0, 0.0],
            LENGTH * P / EA,
        );
        support(&result, ROOT_SUPPORT, [-600.0, 0.0, 0.0], P);
    }
}

#[test]
fn signed_pressure_zero_poisson_and_thermal_reversal_preserve_the_selected_equations() {
    for mode in MODES {
        for (pressure, nu, fixed, thermal) in [
            (0.0, 0.3, false, 0.0),
            (-2e6, 0.3, false, 0.0),
            (-2e6, 0.3, true, 0.0),
            (2e6, 0.0, false, 0.0),
            (2e6, 0.0, true, 0.0),
            (2e6, 0.3, true, -0.0012),
        ] {
            let mut input = model(fixed, true, thermal);
            input["model"]["materials"][0]["poisson_ratio"]["value"] = json!(nu);
            input["model"]["load_cases"][0]["pressure_regions"][0]["pressure"]["value"] =
                json!(pressure);
            let result = solve(input, mode);
            solved(&result);
            let cap = pressure * 0.0025 * PI;
            let wall = if fixed {
                -EA * thermal + 2.0 * nu * cap
            } else {
                cap
            };
            let extension = if fixed {
                0.0
            } else {
                LENGTH * (thermal + (1.0 - 2.0 * nu) * cap / EA)
            };
            let force_scale = cap.abs().max((EA * thermal).abs());
            let displacement_scale = LENGTH * thermal.abs().max((cap / EA).abs());
            pressure_fields(&result, PIPE, |_| wall, pressure, 0.05, 0.06, force_scale);
            displacement(&result, TIP, [extension, 0.0, 0.0], displacement_scale);
            let reaction = if fixed { cap - wall } else { 0.0 };
            support(&result, ROOT_SUPPORT, [reaction, 0.0, 0.0], force_scale);
            if fixed {
                support(&result, TIP_SUPPORT, [-reaction, 0.0, 0.0], force_scale);
            }
        }
    }
}

fn rejected(input: Value, code: &str) {
    for mode in MODES {
        let result = solve(input.clone(), mode);
        assert_ne!(
            result.status.mechanics, "MECHANICS_SOLVED",
            "invalid contract was accepted"
        );
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.code == code && (d.severity == "blocking" || d.severity == "failure")),
            "expected targeted {code}; actual={:?}",
            result.diagnostics
        );
        assert!(
            !result
                .results
                .iter()
                .any(|r| r.kind.starts_with("pipe_") && r.kind.ends_with("_v2")),
            "invalid exact request published exact pressure rows"
        );
    }
}

#[test]
fn exact_material_and_region_contracts_are_required_without_inference() {
    let mut input = model(false, true, 0.0);
    input["model"]["materials"][0]
        .as_object_mut()
        .unwrap()
        .remove("poisson_ratio");
    rejected(input, "EXACT_PRESSURE_POISSON_RATIO_REQUIRED");
    let mut input = model(false, true, 0.0);
    input["model"]["materials"][0]
        .as_object_mut()
        .unwrap()
        .remove("constitutive_basis");
    rejected(input, "EXACT_PRESSURE_MATERIAL_BASIS_REQUIRED");
    let mut input = model(false, true, 0.0);
    input["model"]["load_cases"][0]
        .as_object_mut()
        .unwrap()
        .remove("pressure_regions");
    rejected(input, "EXACT_PRESSURE_REGIONS_REQUIRED");
    let mut input = model(false, true, 0.0);
    input["model"]["load_cases"][0]["pressure_regions"][0]["terminals"] = json!([]);
    rejected(input, "PRESSURE_TERMINALS_INVALID");
    let mut input = model(false, true, 0.0);
    input["model"]["load_cases"][0]["pressure_regions"][0]["terminals"][0]["closure_transfer"] =
        json!("guess_closed");
    rejected(input, "PRESSURE_TERMINAL_CLOSURE_INVALID");
    let mut input = model(false, true, 0.0);
    input["model"]["load_cases"][0]["pressure_regions"][0]["pressure_basis"] =
        json!("arbitrary_external_pressure");
    rejected(input, "PRESSURE_REGION_BASIS_UNSUPPORTED");
}

#[test]
fn invalid_region_membership_geometry_and_overlap_block() {
    let mut input = model(false, true, 0.0);
    input["model"]["load_cases"][0]["pressure_regions"][0]["member_pipe_ids"] = json!([PIPE, PIPE]);
    rejected(input, "PRESSURE_REGION_MEMBERS_INVALID");
    let mut input = model(false, true, 0.0);
    input["model"]["load_cases"][0]["pressure_regions"][0]["member_pipe_ids"] =
        json!(["pipe:missing"]);
    rejected(input, "PRESSURE_REGION_MEMBER_UNKNOWN");
    let mut input = model(false, true, 0.0);
    let mut second = input["model"]["load_cases"][0]["pressure_regions"][0].clone();
    second["id"] = json!("region:overlap");
    input["model"]["load_cases"][0]["pressure_regions"]
        .as_array_mut()
        .unwrap()
        .push(second);
    rejected(input, "PRESSURE_REGION_PIPE_OVERLAP");
    let mut input = two_span(false, false, false, false);
    input["model"]["pipe_segments"][1]["section"]["wall_thickness"]["value"] = json!(0.01);
    rejected(input, "PRESSURE_REGION_BORE_MISMATCH");
    let mut input = two_span(false, false, false, false);
    input["model"]["nodes"][2]["position"]["y"] = json!(1.0);
    rejected(input, "PRESSURE_REGION_NONCOLLINEAR");
    let mut input = two_span(false, false, false, false);
    input["model"]["nodes"]
        .as_array_mut()
        .unwrap()
        .push(node("node:D", [3.0, 1.0, 0.0]));
    input["model"]["pipe_segments"]
        .as_array_mut()
        .unwrap()
        .push(pipe(
            "pipe:outside-branch",
            TIP,
            "node:D",
            0.12,
            0.01,
            "material:one",
        ));
    rejected(input, "PRESSURE_REGION_INTERNAL_BRANCH");
}

#[test]
fn namespaces_and_nonzero_legacy_pressure_cannot_silently_fallback() {
    let mut exact_primitive = model(false, true, 0.0);
    exact_primitive["model"]["load_cases"][0]["primitive_loads"] = json!([{
        "id":"load:old-pressure","category":"pressure","target":{"type":"element","pipe":PIPE},
        "direction":"global_x","dimension":"pressure","magnitude":{"value":2e6,"unit":"Pa"},
        "provenance":"explicit_invalid_legacy_pressure_in_exact_model"}]);
    rejected(exact_primitive.clone(), "EXACT_PRESSURE_REQUIRES_REGION");
    let mut legacy = exact_primitive;
    legacy["model"]["schema_version"] = json!("0.2.0");
    legacy["model"]
        .as_object_mut()
        .unwrap()
        .remove("pressure_contract");
    legacy["model"]["load_cases"][0]
        .as_object_mut()
        .unwrap()
        .remove("pressure_regions");
    legacy["model"]["materials"][0]["shear_modulus"] = json!({"value":77e9,"unit":"Pa"});
    rejected(legacy, "PRESSURE_MODEL_REAUTHOR_REQUIRED");
    let mut mismatch = model(false, true, 0.0);
    mismatch["model"]["schema_version"] = json!("0.2.0");
    rejected(mismatch, "PREVIEW_CONTRACT_VERSION_MISMATCH");
    let mut unknown = model(false, true, 0.0);
    unknown["model"]["pressure_contract"]["version"] = json!("99.0.0");
    rejected(unknown, "PRESSURE_CONTRACT_UNSUPPORTED");
}

#[test]
fn exact_override_materials_replace_base_list_and_redundant_g_is_not_authoritative() {
    let mut missing_nu = model(false, true, 0.0);
    let mut override_material = material("material:one", 200e9, 0.3);
    override_material
        .as_object_mut()
        .unwrap()
        .remove("poisson_ratio");
    missing_nu["materials"] = json!([override_material]);
    rejected(missing_nu, "EXACT_PRESSURE_POISSON_RATIO_REQUIRED");
    let mut missing_material = two_span(false, false, false, false);
    missing_material["materials"] = json!([material("material:one", 200e9, 0.3)]);
    rejected(missing_material, "MATERIAL_INPUT_MISSING");
    for mode in MODES {
        let mut input = model(false, true, 0.0);
        input["model"]["materials"][0]["shear_modulus"] = json!({"value":1e3,"unit":"Pa"});
        add_torque(&mut input);
        let result = solve(input, mode);
        solved(&result);
        check_axial_and_torsion(&result, 3.0 / 55000.0, 200e9 / (2.0 * 1.3));
        pressure_fields(&result, PIPE, |_| P, 2e6, 0.05, 0.06, P);
    }
}

fn add_torque(input: &mut Value) {
    input["model"]["load_cases"][0]["primitive_loads"]
        .as_array_mut()
        .unwrap()
        .push(json!({
        "id":"load:torsion","category":"occasional",
        "target":{"type":"node","node":TIP},"direction":"rotation_x",
        "magnitude":{"value":1.0,"unit":"N*m"},"dimension":"moment",
        "provenance":"independent_circular_torsion_reference"}));
}

fn check_axial_and_torsion(result: &MechanicsEnvelope, extension: f64, g: f64) {
    let ux = row(
        result,
        "global_nodal_displacement_x",
        TIP,
        "nodal_displacement_x",
        None,
    );
    assert_eq!(ux.unit, "mm");
    close(
        ux.value / 1000.0,
        extension,
        LENGTH * P / EA,
        "selected-pair axial extension",
    );
    let rx = row(
        result,
        "global_nodal_rotation_x",
        TIP,
        "nodal_rotation_x",
        None,
    );
    assert_eq!(rx.unit, "rad");
    close(
        rx.value,
        LENGTH / (g * (3.355e-6 * PI)),
        0.0,
        "selected common E/nu derived G torsion",
    );
}

fn point(id: &str, temperature: f64, e: f64, nu: f64) -> Value {
    json!({"id":id,"temperature":{"value":temperature,"unit":"degC"},
        "elastic_modulus":{"value":e,"unit":"Pa"},"poisson_ratio":{"value":nu,"unit":"1"},
        "shear_modulus":{"value":123456.0,"unit":"Pa"},
        "provenance":"independent_selected_material_pair"})
}

#[test]
fn selected_point_and_interpolated_pair_determine_g_after_selection() {
    for mode in MODES {
        for interpolated in [false, true] {
            let mut input = model(false, true, 0.0);
            input["model"]["materials"][0]["temperature_points"] = json!([
                point("point:cold", 0.0, 100e9, 0.1),
                point("point:hot", 100.0, 300e9, 0.4)
            ]);
            if interpolated {
                input["model"]["load_cases"][0]["modulus_basis_temperature"] =
                    json!({"value":50.0,"unit":"degC"});
            } else {
                input["model"]["load_cases"][0]["modulus_basis_ref"] = json!("point:cold");
            }
            add_torque(&mut input);
            let result = solve(input, mode);
            solved(&result);
            let (extension, g) = if interpolated {
                (3.0 / 44000.0, 80e9)
            } else {
                (3.0 / 13750.0, 100e9 / (2.0 * 1.1))
            };
            check_axial_and_torsion(&result, extension, g);
            pressure_fields(&result, PIPE, |_| P, 2e6, 0.05, 0.06, P);
        }
    }
    for temperature in [0.0, 100.0, -1.0, 101.0] {
        let mut input = model(false, true, 0.0);
        input["model"]["materials"][0]["temperature_points"] = json!([
            point("point:cold", 0.0, 100e9, 0.1),
            point("point:hot", 100.0, 300e9, 0.4)
        ]);
        input["model"]["load_cases"][0]["modulus_basis_temperature"] =
            json!({"value":temperature,"unit":"degC"});
        rejected(input, "MODULUS_BASIS_UNRESOLVED");
    }
}

#[test]
fn thermal_alpha_is_required_on_the_selected_basis_not_unused_base() {
    for mode in MODES {
        let mut input = model(false, true, 0.0012);
        input["model"]["materials"][0]
            .as_object_mut()
            .unwrap()
            .remove("thermal_expansion_coefficient");
        let mut selected = point("point:thermal", 100.0, 100e9, 0.2);
        selected["thermal_expansion_coefficient"] = json!({"value":20e-6,"unit":"1/degC"});
        input["model"]["materials"][0]["temperature_points"] = json!([selected]);
        input["model"]["load_cases"][0]["modulus_basis_ref"] = json!("point:thermal");
        let result = solve(input, mode);
        solved(&result);
        displacement(
            &result,
            TIP,
            [0.012 + 9.0 / 55000.0, 0.0, 0.0],
            0.012 + 9.0 / 55000.0,
        );
        pressure_fields(&result, PIPE, |_| P, 2e6, 0.05, 0.06, P);
    }
    let mut missing = model(false, true, 0.0012);
    missing["model"]["materials"][0]["temperature_points"] =
        json!([point("point:missing-alpha", 100.0, 100e9, 0.2)]);
    missing["model"]["load_cases"][0]["modulus_basis_ref"] = json!("point:missing-alpha");
    rejected(missing, "THERMAL_EXPANSION_INPUT_MISSING");
}

#[test]
fn legacy_missing_g_remains_an_input_error() {
    let mut input = model(false, true, 0.0);
    input["model"]["schema_version"] = json!("0.2.0");
    input["model"]
        .as_object_mut()
        .unwrap()
        .remove("pressure_contract");
    input["model"]["load_cases"][0]
        .as_object_mut()
        .unwrap()
        .remove("pressure_regions");
    rejected(input, "MATERIAL_INPUT_MISSING");
}

#[test]
fn same_region_id_in_distinct_cases_has_unique_binding_and_order_independent_maximum() {
    for mode in MODES {
        for tied in [false, true] {
            for reverse in [false, true] {
                let mut input = model(false, true, 0.0);
                let mut first = input["model"]["load_cases"][0].clone();
                first["id"] = json!("case:z-positive");
                let mut second = first.clone();
                second["id"] = json!("case:a-negative");
                second["pressure_regions"][0]["pressure"]["value"] =
                    json!(if tied { -2e6 } else { -4e6 });
                input["model"]["load_cases"] = if reverse {
                    json!([second, first])
                } else {
                    json!([first, second])
                };
                let result = solve(input, mode);
                solved(&result);
                let negative_factor = if tied { -1.0 } else { -2.0 };
                for (case_id, factor) in [
                    ("case:z-positive", 1.0),
                    ("case:a-negative", negative_factor),
                ] {
                    let r = case_row(
                        &result,
                        case_id,
                        "global_nodal_displacement_x",
                        TIP,
                        "nodal_displacement_x",
                        None,
                    );
                    close(
                        r.value / 1000.0,
                        factor * 3.0 / 55000.0,
                        LENGTH * P / EA,
                        "case-qualified displacement",
                    );
                    for (location, _) in STATIONS {
                        let wall = case_row(
                            &result,
                            case_id,
                            "pipe_wall_axial_force_v2",
                            PIPE,
                            "wall_axial_force",
                            Some(location),
                        );
                        close(
                            wall.value,
                            factor * P,
                            P,
                            "case-qualified signed wall force",
                        );
                        let stress = case_row(
                            &result,
                            case_id,
                            "pipe_axial_membrane_stress_v2",
                            PIPE,
                            "axial_membrane_stress",
                            Some(location),
                        );
                        close(
                            stress.value,
                            factor * P / AS,
                            P / AS,
                            "case-qualified membrane stress",
                        );
                    }
                }
                let entries = result.contract_evidence.as_ref().unwrap()["pressure"]
                    .as_array()
                    .unwrap();
                assert_eq!(entries.len(), 2);
                for r in result.results.iter().filter(|r| pressure_kind(&r.kind)) {
                    let occurrences: Vec<_> = entries
                        .iter()
                        .filter(|e| {
                            e["result_ids"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .any(|id| id == &r.id)
                        })
                        .collect();
                    assert_eq!(
                        occurrences.len(),
                        1,
                        "pressure row must belong to exactly one case-region mapping"
                    );
                    assert_eq!(occurrences[0]["region_id"], REGION);
                    assert_eq!(
                        occurrences[0]["load_case_id"],
                        r.basis_ref.as_ref().unwrap().ref_id
                    );
                }
                let maximum = result
                    .summary
                    .max_displacement
                    .as_ref()
                    .expect("actual-case displacement maximum");
                assert_eq!(maximum.unit, "mm");
                assert_eq!(maximum.location_ref, TIP);
                close(
                    maximum.value,
                    (-negative_factor) * 3.0 / 55000.0 * 1000.0,
                    0.0,
                    "all-case displacement maximum",
                );
                let source = result
                    .results
                    .iter()
                    .find(|r| r.id == maximum.result_ref)
                    .expect("maximum refers to an actual returned source row");
                assert_eq!(source.entity_ref, TIP);
                assert_eq!(source.basis_ref.as_ref().unwrap().ref_id, "case:a-negative");
                assert_eq!(source.unit, maximum.unit);
                assert_eq!(source.value, maximum.value);
            }
        }
    }
    let mut input = model(false, true, 0.0);
    let duplicate = input["model"]["load_cases"][0]["pressure_regions"][0].clone();
    input["model"]["load_cases"][0]["pressure_regions"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    rejected(input, "PRESSURE_REGION_ID_DUPLICATE");
}
