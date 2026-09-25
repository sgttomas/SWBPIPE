//! Public composite controls use separately authored explicit E/nu companions.
//! Original NUM requests/references remain unchanged. No expected result is read
//! from a produced raw fixture; analytical fields and whole-body balance apply.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, PreviewSolverMode,
};
use serde_json::{json, Value};

fn request(name: &str) -> Value {
    let text = match name {
        "n05" => include_str!("../../../fixtures/product_preview/physics_source/n05.request.json"),
        "n06" => include_str!("../../../fixtures/product_preview/physics_source/n06.request.json"),
        "fields" => {
            include_str!("../../../fixtures/product_preview/physics_source/fields.request.json")
        }
        "mixed" => {
            include_str!("../../../fixtures/product_preview/physics_source/mixed.request.json")
        }
        _ => unreachable!(),
    };
    serde_json::from_str(text).unwrap()
}
fn relative(actual: f64, expected: f64) {
    if expected == 0.0 {
        assert_eq!(actual, 0.0);
    } else {
        assert!(
            ((actual - expected) / expected).abs() <= 1e-9,
            "{actual:e} vs {expected:e}"
        );
    }
}
fn solve(input: Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(input, mode).unwrap()).unwrap()
}
fn row<'a>(
    raw: &'a Value,
    entity: &str,
    kind: &str,
    component: Option<&str>,
    case: &str,
) -> &'a Value {
    let rows: Vec<_> = raw["results"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| {
            r["entity_ref"] == entity
                && r["kind"] == kind
                && r["basis_ref"]["ref_id"] == case
                && component.map_or(true, |c| r["metadata"]["component"] == c)
        })
        .collect();
    assert_eq!(rows.len(), 1, "{entity}/{kind}/{component:?}/{case}");
    rows[0]
}
fn qualified(raw: &Value) {
    assert_eq!(
        raw["status"]["mechanics"], "MECHANICS_SOLVED",
        "{}",
        raw["diagnostics"]
    );
    assert_eq!(
        raw["producer"]["semantic_contract_id"],
        "openpipestress.result_semantics/0.3.0/physics-source-1"
    );
    assert_eq!(
        raw["source_block_recovery"]["body"]["policy"],
        "PHYSICS-SOURCE-1"
    );
    assert_eq!(
        raw["source_block_recovery"]["body"]["status"], "qualified",
        "{}",
        raw["diagnostics"]
    );
}
#[test]
fn actual_explicit_material_source_companions_preserve_torsion_actions_and_constant_normal_field() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        for name in ["n05", "n06"] {
            let input = request(name);
            let torque = input["model"]["load_cases"][0]["primitive_loads"][0]["magnitude"]
                ["value"]
                .as_f64()
                .unwrap();
            let spring = input["model"]["supports"][1]["stiffness"]["value"]["value"]
                .as_f64()
                .unwrap();
            let raw = solve(input, mode);
            qualified(&raw);
            let j = std::f64::consts::PI * (0.1_f64.powi(4) - 0.09_f64.powi(4)) / 2.0;
            relative(
                row(
                    &raw,
                    "independent-root",
                    "global_nodal_rotation_x",
                    None,
                    "case",
                )["value"]
                    .as_f64()
                    .unwrap(),
                torque / spring,
            );
            relative(
                row(
                    &raw,
                    "independent-tip",
                    "global_nodal_rotation_x",
                    None,
                    "case",
                )["value"]
                    .as_f64()
                    .unwrap(),
                torque / spring + torque * 2.0 / (80e9 * j),
            );
            relative(
                row(
                    &raw,
                    "independent-spring",
                    "support_reaction_component_v2",
                    Some("Mx"),
                    "case",
                )["value"]
                    .as_f64()
                    .unwrap(),
                -torque,
            );
            relative(
                row(
                    &raw,
                    "independent-spring",
                    "support_reaction_moment_magnitude_v2",
                    Some("moment_magnitude"),
                    "case",
                )["value"]
                    .as_f64()
                    .unwrap(),
                torque.abs(),
            );
            let maximum = row(
                &raw,
                "independent-member",
                "pipe_elastic_normal_stress_maximum_v2",
                None,
                "case",
            );
            assert_eq!(
                maximum["metadata"]["basis"],
                "retained_source_endpoint_normal_max_v1"
            );
            assert_eq!(maximum["value"], 0.0);
            let material = &raw["contract_evidence"]["exact_cases"][0]["pipe_materials"][0];
            assert_eq!(material["nu"], 0.25);
            assert_eq!(material["G_pa"], 80e9);
            assert_eq!(
                raw["contract_evidence"]["exact_cases"][0]["recovery_method"],
                "retained_source_blocks_exact_v1"
            );
        }
    }
}
#[test]
fn actual_source_fields_six_signed_actions_and_circular_maximum_match_independent_equilibrium() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let raw = solve(request("fields"), mode);
        qualified(&raw);
        for (component, expected) in [
            ("Fx", -0.001),
            ("Fy", -0.002),
            ("Fz", 0.003),
            ("Mx", 0.0),
            ("My", -0.0055),
            ("Mz", -0.0046),
        ] {
            relative(
                row(
                    &raw,
                    "anchor",
                    "support_reaction_component_v2",
                    Some(component),
                    "case",
                )["value"]
                    .as_f64()
                    .unwrap(),
                expected,
            );
        }
        let area = std::f64::consts::PI * (0.1_f64.powi(2) - 0.09_f64.powi(2));
        let inertia = std::f64::consts::PI * (0.1_f64.powi(4) - 0.09_f64.powi(4)) / 4.0;
        let expected =
            0.001 / area + (0.0055_f64.powi(2) + 0.0046_f64.powi(2)).sqrt() / (inertia / 0.1);
        relative(
            row(
                &raw,
                "independent-member",
                "pipe_elastic_normal_stress_maximum_v2",
                None,
                "case",
            )["value"]
                .as_f64()
                .unwrap(),
            expected,
        );
        relative(
            row(
                &raw,
                "independent-tip",
                "global_nodal_displacement_x",
                None,
                "case",
            )["value"]
                .as_f64()
                .unwrap(),
            1000.0 * 0.001 * 2.0 / (200e9 * area),
        );
        relative(
            row(
                &raw,
                "independent-tip",
                "global_nodal_displacement_y",
                None,
                "case",
            )["value"]
                .as_f64()
                .unwrap(),
            1000.0 * (0.002 * 8.0 / 3.0 + 0.0006 * 4.0 / 2.0) / (200e9 * inertia),
        );
        relative(
            row(
                &raw,
                "independent-tip",
                "global_nodal_displacement_z",
                None,
                "case",
            )["value"]
                .as_f64()
                .unwrap(),
            1000.0 * (-0.003 * 8.0 / 3.0 + 0.0005 * 4.0 / 2.0) / (200e9 * inertia),
        );
    }
}
#[test]
fn source_selected_and_ordinary_physics_cases_keep_distinct_actual_warrants() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let raw = solve(request("mixed"), mode);
        qualified(&raw);
        let cases = raw["contract_evidence"]["exact_cases"].as_array().unwrap();
        assert_eq!(cases.len(), 2);
        assert_eq!(
            cases[0]["recovery_method"],
            "retained_source_blocks_exact_v1"
        );
        assert_eq!(
            cases[1]["recovery_method"],
            if mode == PreviewSolverMode::DenseScrutiny {
                "ordinary_dense_structural_v1"
            } else {
                "ordinary_sparse_structural_v1"
            }
        );
        assert_eq!(cases[1]["pipe_materials"][0]["E_pa"], 2000.0);
        assert_eq!(cases[1]["pipe_materials"][0]["G_pa"], 800.0);
        assert_eq!(
            raw["contract_evidence"]["pressure"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            raw["source_block_recovery"]["body"]["cases"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
    }
}
#[test]
fn nonempty_region_presence_never_enters_empty_region_source_method_even_at_zero() {
    for pressure in [0.0, 1e-5] {
        for mode in [
            PreviewSolverMode::SparseInteractive,
            PreviewSolverMode::DenseScrutiny,
        ] {
            let mut input = request("n05");
            let mut region =
                request("mixed")["model"]["load_cases"][1]["pressure_regions"][0].clone();
            region["pressure"]["value"] = json!(pressure);
            input["model"]["load_cases"][0]["pressure_regions"] = json!([region]);
            let raw = solve(input, mode);
            assert_ne!(
                raw["producer"]["semantic_contract_id"],
                "openpipestress.result_semantics/0.3.0/physics-source-1"
            );
            assert!(raw.get("source_block_recovery").is_none());
        }
    }
}

#[test]
fn recorded_g_is_not_the_source_selected_material_basis() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let mut altered = request("n05");
        altered["model"]["materials"][0]["shear_modulus"] = json!({"value":77e9,"unit":"Pa"});
        let baseline = solve(request("n05"), mode);
        let changed = solve(altered, mode);
        qualified(&baseline);
        qualified(&changed);
        assert_ne!(
            baseline["source_block_recovery"]["body"]["invocation"],
            changed["source_block_recovery"]["body"]["invocation"]
        );
        assert_eq!(baseline["results"], changed["results"]);
        assert_eq!(
            changed["contract_evidence"]["exact_cases"][0]["pipe_materials"][0]["G_pa"],
            80e9
        );
    }
}
