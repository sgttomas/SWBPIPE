//! Unit tests of the origin test of a generated load on a load/reference-state
//! document (DEL-05-01 REQ-008; issue 452 decision 10): the test is judged on
//! the effective case, whose stored primitives are exactly those the declared
//! ledger lists, and not on the model's authored case. Inputs are invented.
use super::*;
use serde_json::{json, Value};

const GENERATED: &str = "case:join:generated:seismic:global-y:element-0";

/// The maintained load/reference-state witness with seismic generation on its
/// one case and a stored primitive under `id`, listed in the ledger at
/// `factor` or not listed (`None`).
fn request(id: &str, factor: Option<f64>) -> Value {
    let mut request: Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/pressure_source/load_reference_source/eigen_motion.request.json"
    ))
    .unwrap();
    let case = &mut request["model"]["load_cases"][0];
    case["equivalent_static"] = json!({"seismic": {"gravity_acceleration": {"value": 9.8, "unit": "m/s^2"},
        "g_factor_y": {"value": 0.3, "unit": "1"}}, "provenance": "invented"});
    let mut stored = case["primitive_loads"][0].clone();
    stored["id"] = json!(id);
    case["primitive_loads"].as_array_mut().unwrap().push(stored);
    if let Some(factor) = factor {
        case["analysis_state"]["load_sources"].as_array_mut().unwrap().push(json!({"source_ref": id, "factor": factor}));
    }
    request
}

fn model(id: &str, listed: bool) -> PreviewModel {
    to_model(request(id, listed.then_some(1.0)))
}

fn to_model(request: Value) -> PreviewModel {
    let request: LinearStaticPreviewRequest = serde_json::from_value(request).unwrap();
    assert!(case_state::is_load_state(&request.model));
    request.model
}

/// The witness with a valve at the tip that replaces its member, its centre
/// of gravity off the member's axis so that the y generation forms a moment.
fn with_valve(mut request: Value) -> Value {
    request["model"]["components"] = json!([{"id": "component:valve", "kind": "valve", "node": "tip",
        "geometry": {"rigid_pipe_ref": "member", "weight": {"value": 2350.0, "unit": "N"},
            "center_of_gravity": {"x": 0.0, "y": 0.0, "z": 0.125, "unit": "m"}},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": "invented"}]);
    request
}

/// A stored primitive that the ledger lists, a lowering-appended one listed
/// in the same way included, is part of the effective case: a generated-looking
/// id it carries is authored, never generated. One that the ledger does not
/// list is excluded from the effective case, so the same id is judged
/// generated there (and the collision itself is refused by
/// `refuse_load_state_generated_collisions`). The authored case's own test
/// (`is_generated_equivalent_static_load`) would call both authored.
#[test]
fn the_origin_test_reads_the_effective_case() {
    let listed = model(GENERATED, true);
    let case = &listed.load_cases[0];
    assert!(!is_generated_in_effective_case(&listed, case, GENERATED, Some("seismic")));
    assert!(!is_generated_in_effective_case(&listed, case, GENERATED, None));
    assert!(!is_generated_equivalent_static_load(case, GENERATED, None));

    let unlisted = model(GENERATED, false);
    let case = &unlisted.load_cases[0];
    assert!(is_generated_in_effective_case(&unlisted, case, GENERATED, Some("seismic")));
    assert!(!is_generated_in_effective_case(&unlisted, case, GENERATED, Some("wind")));
    assert!(!is_generated_equivalent_static_load(case, GENERATED, Some("seismic")));

    // A primitive appended after the authored ones (as a lowering appends
    // its records) and listed is part of the effective case.
    let mut lowered = model("case:join:authored", true);
    let mut appended = lowered.load_cases[0].primitive_loads[0].clone();
    appended.id = GENERATED.to_string();
    lowered.load_cases[0].primitive_loads.push(appended);
    if let case_state::input::Authored::Value(state) = &mut lowered.load_cases[0].analysis_state {
        state.load_sources.push(case_state::input::LoadSourceInput { source_ref: GENERATED.to_string(), factor: 1.0 });
    }
    assert!(!is_generated_in_effective_case(&lowered, &lowered.load_cases[0], GENERATED, Some("seismic")));

    // Without generation inputs nothing is generated.
    let mut none = model(GENERATED, false);
    none.load_cases[0].equivalent_static = None;
    assert!(!is_generated_in_effective_case(&none, &none.load_cases[0], GENERATED, None));
}

/// The certified nodal ledger (`certified_loads::nodal`) on a load and
/// reference-state document (issue 452 decision 10), on the witness with a
/// valve at the tip:
/// - a stored primitive under the generated component force's identity that
///   the ledger lists is authored: it takes its ledger factor through
///   `source_factor`, as a formed product, and the component rule is not
///   consulted;
/// - one the ledger does not list is outside the effective case, and the
///   solve refuses the collision by name before assembly;
/// - a truly generated component force and moment are pushed as the stored
///   values the rule gives, at factor 1, and pass the binding: the document
///   solves in both modes with no value mismatch.
#[test]
fn the_nodal_ledger_reads_the_effective_case() {
    use crate::seismic_component::{load_id, rule_value, Part, VALUE_MISMATCH};
    use open_pipe_stress_primitive_loads::NodalLoadContribution;
    let force = load_id("case:join", "component:valve", 1, Part::Force);
    let moment = load_id("case:join", "component:valve", 1, Part::Moment(0));
    let ledger_of = |model: &PreviewModel, loads: Vec<NodalLoadContribution>| {
        let application = LoadApplication { nodal_loads: loads, ..prepare_loads(model.nodes.len(), model.pipe_segments.len(), &[]) };
        let mut ledger = LoadLedger::new();
        let mut diagnostics = Vec::new();
        certified_loads::nodal(&mut ledger, model, &application, "case:join", model.nodes.len(), &mut diagnostics);
        (ledger.finish(model.nodes.len() * 6).unwrap(), diagnostics)
    };

    // Listed at 1.25: the authored lookalike is a formed product of its stored
    // magnitude and its ledger factor.
    let listed = to_model(with_valve(request(&force, Some(1.25))));
    let magnitude = listed.load_cases[0].primitive_loads.iter().find(|load| load.id == force).unwrap().magnitude.value;
    assert!(rule_value(&listed, "case:join", &force).is_none(), "a listed lookalike is not generated");
    let lookalike = NodalLoadContribution { load_id: force.clone(), node_index: 1, global_dof: 9, value: magnitude * 1.25 };
    let (ledger, diagnostics) = ledger_of(&listed, vec![lookalike]);
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
    assert!(ledger.closure_records()[0].formed);
    assert!(matches!(ledger.load_primitives()[0].formula, LoadFormula::NodalProduct { global_dof: 9, magnitude: m, factor: 1.25 } if m == magnitude));

    // Unlisted: outside the effective case, so the identity is the
    // generator's, and the solve refuses the collision before assembly.
    let unlisted = with_valve(request(&force, None));
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let out = serde_json::to_value(run_linear_static_preview_value_with_mode(unlisted.clone(), mode).unwrap()).unwrap();
        assert_eq!(out["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}");
        assert!(out["results"].as_array().unwrap().is_empty(), "{mode:?}");
        let found: Vec<&Value> = out["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == "EQUIVALENT_STATIC_GENERATED_ID_COLLISION").collect();
        assert_eq!(found.len(), 1, "{mode:?}");
        assert!(found[0]["message"].as_str().unwrap().contains("not listed in analysis_state.load_sources"), "{mode:?}");
    }

    // Truly generated: the generator's own loads, pushed as the rule's values.
    let plain = with_valve(request("case:join:authored", Some(1.0)));
    let model = to_model(plain.clone());
    let mut diagnostics = Vec::new();
    let loads = build_load_case_primitive_loads(&model, &model.load_cases[0], &mut diagnostics);
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    let application = prepare_loads(model.nodes.len(), model.pipe_segments.len(), &loads);
    let generated: Vec<NodalLoadContribution> =
        application.nodal_loads.iter().filter(|load| load.load_id == force || load.load_id == moment).cloned().collect();
    assert_eq!(generated.len(), 2, "the force and the moment about x");
    for load in &generated {
        let rule = rule_value(&model, "case:join", &load.load_id).expect("a generated component load");
        assert_eq!(load.value.to_bits(), rule.0.to_bits(), "{} is the rule's stored value", load.load_id);
    }
    let (ledger, diagnostics) = ledger_of(&model, generated.clone());
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
    assert!(ledger.closure_records().iter().all(|record| !record.formed), "input terms at factor 1");
    assert!(ledger.load_primitives().is_empty());
    for load in &generated {
        assert_eq!(ledger.values()[load.global_dof].to_bits(), load.value.to_bits());
    }
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let out = serde_json::to_value(run_linear_static_preview_value_with_mode(plain.clone(), mode).unwrap()).unwrap();
        assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
        assert!(out["diagnostics"].as_array().unwrap().iter().all(|d| d["code"] != VALUE_MISMATCH), "{mode:?}");
    }
}

/// The certified intensity producer (`certified_loads::intensity`) and the
/// operand bound it feeds (`add_uniform_element_loads`) judge the origin on
/// the same effective case: a generated element load takes the generator's
/// seismic operands, while a stored primitive under the same identity that
/// the ledger lists, a lowering-appended one included, is authored. Unlisted,
/// the primitive is outside the effective case and the identity is the
/// generator's (the collision itself is refused by
/// `refuse_load_state_generated_collisions`).
#[test]
fn the_intensity_producer_reads_the_effective_case() {
    use open_pipe_stress_frame_kernel::structural::IntensityPrimitive;
    use open_pipe_stress_primitive_loads::{ElementUniformLoadContribution, LoadDirection, LoadQuantity};
    let load = ElementUniformLoadContribution {
        load_id: GENERATED.to_string(),
        element_index: 0,
        direction: LoadDirection::GlobalY,
        magnitude: LoadQuantity::new(12.5, LoadDimension::ForcePerLength).unwrap(),
        extent: None,
    };
    let seismic = |model: &PreviewModel| matches!(certified_loads::intensity(model, &load, "case:join"),
        IntensityPrimitive::Seismic { coefficient, gravity, direction, .. } if coefficient == 0.3 && gravity == 9.8 && direction == [0.0, 1.0, 0.0]);

    assert!(seismic(&model("case:join:authored", true)), "the generated element load takes the generator's operands");
    assert!(!seismic(&model(GENERATED, true)), "a listed authored lookalike is authored");

    let mut lowered = model("case:join:authored", true);
    let mut appended = lowered.load_cases[0].primitive_loads[0].clone();
    appended.id = GENERATED.to_string();
    lowered.load_cases[0].primitive_loads.push(appended);
    if let case_state::input::Authored::Value(state) = &mut lowered.load_cases[0].analysis_state {
        state.load_sources.push(case_state::input::LoadSourceInput { source_ref: GENERATED.to_string(), factor: 1.0 });
    }
    assert!(!seismic(&lowered), "a lowering-appended listed lookalike is authored");

    assert!(seismic(&model(GENERATED, false)), "unlisted, the identity is the generator's on the effective case");
}
