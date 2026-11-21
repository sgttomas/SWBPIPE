//! Unit tests of the origin test of a generated load on a load/reference-state
//! document (DEL-05-01 REQ-008; issue 452 decision 10): the test is judged on
//! the effective case, whose stored primitives are exactly those the declared
//! ledger lists, and not on the model's authored case. Inputs are invented.
use super::*;
use serde_json::{json, Value};

const GENERATED: &str = "case:join:generated:seismic:global-y:element-0";

/// The maintained load/reference-state witness with seismic generation on its
/// one case and a stored primitive under `id`, listed in the ledger or not.
fn model(id: &str, listed: bool) -> PreviewModel {
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
    if listed {
        case["analysis_state"]["load_sources"].as_array_mut().unwrap().push(json!({"source_ref": id, "factor": 1.0}));
    }
    let request: LinearStaticPreviewRequest = serde_json::from_value(request).unwrap();
    assert!(case_state::is_load_state(&request.model));
    request.model
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
