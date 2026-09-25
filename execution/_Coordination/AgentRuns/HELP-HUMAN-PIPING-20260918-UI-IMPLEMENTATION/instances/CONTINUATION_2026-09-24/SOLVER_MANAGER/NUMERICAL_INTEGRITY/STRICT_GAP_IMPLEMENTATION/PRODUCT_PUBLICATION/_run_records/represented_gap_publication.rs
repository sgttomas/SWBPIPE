//! Public-entrypoint containment controls for NGR02 mixed recovery.
//!
//! A converged represented-exact gap response is not a qualified ordinary
//! binary64 recovery basis. These tests specify the first solver-first cut's
//! nonpassing publication; they do not qualify a precision-2 method or Current.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, AccuracyEvidence, LinearStaticPreviewRequest,
    MechanicsEnvelope, NumericalQualityStatus, PreviewSolverMode, ResultItem, StructuralStatus,
};
use serde_json::{json, Value};

const CASE: &str = "load-case:axial-publication";
const TIP: &str = "node:axial-tip";
const GAP_SUPPORT: &str = "support:axial-gap";
const UNQUALIFIED: &str = "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED";
const E_PA: f64 = 200_000_000_000.0;
const G_PA: f64 = 77_000_000_000.0;
const OD_M: f64 = 0.1;
const WALL_M: f64 = 0.005;
const LENGTH_M: f64 = 1.0;
const GAP_MM: f64 = 0.1;

fn axial_request(force_n: f64, gap: Option<(&str, &str)>) -> LinearStaticPreviewRequest {
    // Actual public fixture version/status/project metadata remain unchanged.
    // Only the explicitly declared invented model entities below are replaced;
    // no inherited pressure, temperature, component or combination path remains.
    let mut model: Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/invented_preview_model.json"
    )).unwrap();
    assert_eq!(model["schema_version"], "0.1.0");
    model["nodes"] = json!([
        {"id":"node:axial-root","position":{"x":0.0,"y":0.0,"z":0.0},
         "provenance":"invented axial publication control; SI coordinates"},
        {"id":TIP,"position":{"x":LENGTH_M,"y":0.0,"z":0.0},
         "provenance":"invented axial publication control; SI coordinates"}
    ]);
    model["pipe_segments"] = json!([{
        "id":"pipe:axial-publication","from":"node:axial-root","to":TIP,
        "section":{"outside_diameter":{"value":OD_M,"unit":"m"},
                   "wall_thickness":{"value":WALL_M,"unit":"m"}},
        "material":"material:axial-publication", "y_reference":{"x":0.0,"y":1.0,"z":0.0},
        "provenance":"invented straight circular elastic pipe; no pressure or thermal load"
    }]);
    model["materials"] = json!([{
        "id":"material:axial-publication",
        "elastic_modulus":{"value":E_PA,"unit":"Pa"},
        "shear_modulus":{"value":G_PA,"unit":"Pa"},
        "provenance":"explicit invented E and G; no catalog or engineering acceptance"
    }]);
    // The adapter preserves explicitly authored anchor DOF subsets. Its guide
    // family permits translations only, so these five kinematic constraints
    // deliberately use anchor with UX omitted, not a rotational guide claim.
    model["supports"] = json!([
        {"id":"support:axial-root","node":"node:axial-root","family":"anchor",
         "restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":"invented fixed root"},
        {"id":"support:axial-kinematic","node":TIP,"family":"anchor",
         "restraints":["UY","UZ","RX","RY","RZ"],"provenance":"explicit invented five-DOF kinematic restraint; UX alone free; not a standard support product"}
    ]);
    if let Some((sense, seed)) = gap {
        model["supports"].as_array_mut().unwrap().push(json!({
            "id":GAP_SUPPORT,"node":TIP,"family":"nonlinear","restraints":[],
            "nonlinear":{"behavior":"gap","dof":"UX","initial_state":seed,
                         "closes_when":sense,"gap":{"value":GAP_MM,"unit":"mm"}},
            "provenance":"invented unilateral axial stop; explicit sense and seed"
        }));
    }
    model["sections"] = json!([]);
    model["components"] = json!([]);
    model["combinations"] = json!([]);
    model["load_cases"] = json!([{
        "id":CASE,"primitive_loads":[{
            "id":"load:axial-publication","category":"concentrated_force",
            "target":{"type":"node","node":TIP},"direction":"UX",
            "magnitude":{"value":force_n,"unit":"N"},"dimension":"force",
            "provenance":"invented signed axial end force; no distributed/eigenloads"
        }],"provenance":"invented small-strain axial load case"
    }]);
    LinearStaticPreviewRequest {
        model: serde_json::from_value(model).unwrap(),
        // The actual public fixture's embedded-material route, without fallback
        // substitution into a separately authored descriptor.
        materials: Vec::new(),
    }
}

fn context(result: &MechanicsEnvelope, label: &str) -> String {
    format!("{label}; mechanics={}; quality={:#?}; diagnostics={:#?}; row_count={}",
        result.status.mechanics, result.numerical_quality, result.diagnostics, result.results.len())
}

fn row<'a>(result: &'a MechanicsEnvelope, kind: &str, entity: &str, ctx: &str) -> &'a ResultItem {
    let matches: Vec<_> = result.results.iter().filter(|r|
        r.kind == kind && r.entity_ref == entity && r.basis_ref.as_ref().is_some_and(|basis|
            basis.ref_type == "load_case" && basis.ref_id == CASE)
    ).collect();
    assert_eq!(matches.len(), 1, "expected one {kind} for {entity}; {ctx}");
    matches[0]
}

fn gap_is_nonpassing(mode: PreviewSolverMode, sense: &str, seed: &str, magnitude_n: f64) {
    let sign = if sense == "positive_displacement" { 1.0 } else { -1.0 };
    let result = run_linear_static_preview_with_mode(
        axial_request(sign * magnitude_n, Some((sense, seed))), mode,
    );
    let label = format!("mode={} sense={sense} seed={seed} load={}N", mode.as_str(), sign * magnitude_n);
    let ctx = context(&result, &label);
    // Establish actual completed nonlinear execution before checking the expected
    // qualification defect. A malformed/unsupported-model return cannot satisfy it.
    assert!(result.diagnostics.iter().any(|d| d.code == "NONLINEAR_SUPPORT_LOOP_CONVERGED"
        && d.affected_refs.iter().any(|r| r == CASE)), "no completed gap loop; {ctx}");
    assert_eq!(row(&result, "nonlinear_support_active_set_converged_flag", "nonlinear_supports", &ctx).value, 1.0, "{ctx}");
    assert_eq!(row(&result, "nonlinear_support_active_set_final_residual_count", "nonlinear_supports", &ctx).value, 0.0, "{ctx}");
    // Independent axial law puts 25 kN below and 100 kN above the 0.1 mm stop.
    let contact = magnitude_n == 100_000.0;
    assert_eq!(row(&result, "nonlinear_support_active_set_state_code", GAP_SUPPORT, &ctx).value,
        if contact { 1.0 } else { 0.0 }, "wrong final contact state; {ctx}");
    let diagnostic = result.diagnostics.iter().find(|d| d.code == UNQUALIFIED
        && d.affected_refs.iter().any(|r| r == CASE))
        .unwrap_or_else(|| panic!("missing specific mixed-recovery containment (not an input or numerical fallback error); {ctx}"));
    assert!(!result.diagnostics.iter().any(|d|
        d.affected_refs.iter().any(|r| r == CASE)
        && matches!(d.code.as_str(), "NUMERICAL_INTEGRITY_CHECKS_PASSED" | "NUMERICAL_INTEGRITY_SENSITIVE")
    ), "ordinary preprojection report must not qualify selected mixed recovery; {ctx}");
    assert_eq!(result.numerical_quality.status, NumericalQualityStatus::Unresolved, "{ctx}");
    assert_eq!(result.numerical_quality.cases.len(), 1, "{ctx}");
    let case = &result.numerical_quality.cases[0];
    assert_eq!(case.basis_ref.ref_id, CASE, "{ctx}");
    assert_eq!(case.solve_quality, NumericalQualityStatus::Unresolved, "{ctx}");
    assert_eq!(case.structural_status, StructuralStatus::NumericallyUnresolved, "{ctx}");
    assert_eq!(case.accuracy_evidence, AccuracyEvidence::Unresolved, "{ctx}");
    assert!(case.evidence_refs.contains(&diagnostic.id), "case must bind its unresolved diagnostic; {ctx}");
}

macro_rules! gap_case {
    ($name:ident, $mode:ident, $sense:literal, $seed:literal, $load:expr) => {
        #[test]
        fn $name() { gap_is_nonpassing(PreviewSolverMode::$mode, $sense, $seed, $load); }
    };
}

gap_case!(sparse_positive_inactive_open, SparseInteractive, "positive_displacement", "inactive", 25_000.0);
gap_case!(sparse_positive_active_open, SparseInteractive, "positive_displacement", "active", 25_000.0);
gap_case!(sparse_positive_inactive_contact, SparseInteractive, "positive_displacement", "inactive", 100_000.0);
gap_case!(sparse_positive_active_contact, SparseInteractive, "positive_displacement", "active", 100_000.0);
gap_case!(sparse_negative_inactive_open, SparseInteractive, "negative_displacement", "inactive", 25_000.0);
gap_case!(sparse_negative_active_open, SparseInteractive, "negative_displacement", "active", 25_000.0);
gap_case!(sparse_negative_inactive_contact, SparseInteractive, "negative_displacement", "inactive", 100_000.0);
gap_case!(sparse_negative_active_contact, SparseInteractive, "negative_displacement", "active", 100_000.0);
gap_case!(dense_positive_inactive_open, DenseScrutiny, "positive_displacement", "inactive", 25_000.0);
gap_case!(dense_positive_active_open, DenseScrutiny, "positive_displacement", "active", 25_000.0);
gap_case!(dense_positive_inactive_contact, DenseScrutiny, "positive_displacement", "inactive", 100_000.0);
gap_case!(dense_positive_active_contact, DenseScrutiny, "positive_displacement", "active", 100_000.0);
gap_case!(dense_negative_inactive_open, DenseScrutiny, "negative_displacement", "inactive", 25_000.0);
gap_case!(dense_negative_active_open, DenseScrutiny, "negative_displacement", "active", 25_000.0);
gap_case!(dense_negative_inactive_contact, DenseScrutiny, "negative_displacement", "inactive", 100_000.0);
gap_case!(dense_negative_active_contact, DenseScrutiny, "negative_displacement", "active", 100_000.0);

#[test]
fn ordinary_no_gap_companion_remains_qualified_and_matches_small_strain_axial_law() {
    // Uniform circular annulus, homogeneous linear elasticity, small axial
    // strain, no pressure/thermal/eigenload, fixed root, no axial tip restraint:
    // N=F, epsilon=F/(EA), u_tip=FL/(EA). OD,t,L in m; E in Pa; F in N.
    // A=pi*t*(OD-t), independently expressed without the solver's section API.
    let area_m2 = std::f64::consts::PI * WALL_M * (OD_M - WALL_M);
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        for force_n in [-100_000.0, 100_000.0] {
            let result = run_linear_static_preview_with_mode(axial_request(force_n, None), mode);
            let ctx = context(&result, &format!("ordinary mode={} load={force_n}N", mode.as_str()));
            assert!(!result.diagnostics.iter().any(|d| d.severity == "blocking" || d.code == UNQUALIFIED), "{ctx}");
            assert_eq!(result.numerical_quality.status, NumericalQualityStatus::ChecksPassed, "{ctx}");
            assert_eq!(result.numerical_quality.cases.len(), 1, "{ctx}");
            assert_eq!(result.numerical_quality.cases[0].solve_quality, NumericalQualityStatus::ChecksPassed, "{ctx}");
            let displacement = row(&result, "global_nodal_displacement_x", TIP, &ctx);
            assert_eq!(displacement.unit, "mm", "{ctx}");
            let expected_mm = force_n * LENGTH_M / (E_PA * area_m2) * 1000.0;
            assert!((displacement.value - expected_mm).abs() / expected_mm.abs() <= 1.0e-9,
                "independent FL/EA mismatch: observed={}mm expected={expected_mm}mm; {ctx}", displacement.value);
        }
    }
}
