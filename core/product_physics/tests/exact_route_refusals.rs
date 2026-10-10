//! T4-U0 (exact-route hardening) public-entry controls.
//!
//! Pins the exact route's previously untested refusals (A1.1-A1.9), the
//! emission order where two apply. Every input is a committed invented fixture
//! patched inline: X0 = `fixtures/exact_pressure_connected_request.json`
//! (0.3.0), Y0 = `fixtures/product_preview/load_reference/pressure.request.json`
//! (0.4.0). "Both entries" are the captured value entry and the typed entry;
//! "both modes" are sparse-interactive and dense-scrutiny.

use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    PreviewSolverMode,
};
use serde_json::{json, Value};

const X0: &str = include_str!("fixtures/exact_pressure_connected_request.json");
const Y0: &str =
    include_str!("../../../fixtures/product_preview/load_reference/pressure.request.json");
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
const SOURCE: &str = "core/product_physics/src/pressure_runtime.rs";
const COMPOSITION: &str = "EXACT_PRESSURE_COMPOSITION_UNSUPPORTED";
const COMBINATION: &str = "EXACT_PRESSURE_COMBINATION_UNSUPPORTED";
const PROVENANCE: &str = "invented_u0_control";

#[derive(Clone, Copy)]
enum Base {
    X0,
    Y0,
}

impl Base {
    fn document(self) -> Value {
        serde_json::from_str(match self {
            Base::X0 => X0,
            Base::Y0 => Y0,
        })
        .unwrap()
    }
    fn tip(self) -> &'static str {
        match self {
            Base::X0 => "node:fixture-tip",
            Base::Y0 => "node:middle",
        }
    }
    fn pipe(self) -> &'static str {
        match self {
            Base::X0 => "pipe:fixture-span",
            Base::Y0 => "pipe:first",
        }
    }
    /// The two cases a combination sums, and the case that carries
    /// `equivalent_static`.
    fn cases(self) -> [&'static str; 2] {
        match self {
            Base::X0 => ["case:closed-pressure", "case:six-component-load"],
            Base::Y0 => ["case:cold-pressure", "case:hot-pressure"],
        }
    }
    fn equivalent_static_case(self) -> &'static str {
        match self {
            Base::X0 => "case:six-component-load",
            Base::Y0 => "case:cold-pressure",
        }
    }
    fn producer(self) -> &'static str {
        match self {
            Base::X0 => "openpipestress.result_semantics/0.3.0/physics-1",
            Base::Y0 => "openpipestress.result_semantics/0.3.0/load-reference-1",
        }
    }
    fn empty_evidence(self) -> Value {
        match self {
            Base::X0 => json!({"pressure":[],"connector":[],"exact_cases":[]}),
            Base::Y0 => {
                json!({"pressure":[],"connector":[],"exact_cases":[],"load_reference_states":[]})
            }
        }
    }
}

fn push(document: &mut Value, array: &str, item: Value) {
    document["model"][array].as_array_mut().unwrap().push(item);
}

fn case_mut<'a>(document: &'a mut Value, id: &str) -> &'a mut Value {
    document["model"]["load_cases"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|case| case["id"] == id)
        .unwrap()
}

/// 0.4.0 cases declare every support's participation; the added support gets
/// one so that no load-state code is added.
fn declare_support_state(document: &mut Value, support: &str) {
    for case in document["model"]["load_cases"].as_array_mut().unwrap() {
        if let Some(states) = case
            .get_mut("analysis_state")
            .and_then(|state| state.get_mut("support_states"))
            .and_then(Value::as_array_mut)
        {
            states.push(json!({"support_ref":support,"participation":{"kind":"active_model_device"}}));
        }
    }
}

fn valve(base: Base) -> Value {
    json!({"id":"component:valve","kind":"valve","node":base.tip(),"provenance":PROVENANCE})
}

fn bend(base: Base) -> Value {
    json!({"id":"component:bend","kind":"bend","node":base.tip(),
        "geometry":{"bend_pipe_ref":base.pipe(),"bend_radius":{"value":1.0,"unit":"m"},
            "bend_plane_orientation":"invented","bend_geometry_source_reference":"invented"},
        "modifiers":{"flexibility_factor_user_value":{"value":1.0,"unit":"none"},"source_reference":"invented"},
        "mechanics_interface":{"solver_consumption":"curved_bend_macro_element"},
        "provenance":PROVENANCE})
}

fn gap(base: Base) -> Value {
    json!({"id":"support:gap","node":base.tip(),"family":"nonlinear","restraints":[],
        "nonlinear":{"behavior":"gap","dof":"UZ","initial_state":"inactive",
            "closes_when":"positive_displacement","gap":{"value":1000.0,"unit":"mm"}},
        "provenance":PROVENANCE})
}

fn constant_effort(base: Base) -> Value {
    json!({"id":"support:ce","node":base.tip(),"family":"constant_effort_support","restraints":["UY"],
        "hanger":{"hanger_type":"constant_effort_support","constant_load":{"value":375.0,"unit":"N"},
            "travel_range":{"value":0.05,"unit":"m"},"source_reference":"invented"},
        "provenance":PROVENANCE})
}

fn connector(base: Base, version: &str) -> Value {
    json!({"id":"component:connector","kind":"expansion_joint","node":base.tip(),
        "objective_connector":{"version":version},"provenance":PROVENANCE})
}

fn combination(base: Base) -> Value {
    let [first, second] = base.cases();
    json!({"id":"combination:sum","basis":"mechanics","terms":[
        {"load_case":first,"factor":1.0},{"load_case":second,"factor":1.0}],
        "provenance":PROVENANCE})
}

fn with_valve(base: Base) -> Value {
    let mut document = base.document();
    push(&mut document, "components", valve(base));
    document
}

fn with_combination(base: Base) -> Value {
    let mut document = base.document();
    document["model"]["combinations"] = json!([combination(base)]);
    document
}

fn with_equivalent_static(base: Base) -> Value {
    let mut document = base.document();
    case_mut(&mut document, base.equivalent_static_case())["equivalent_static"] =
        json!({"provenance":PROVENANCE});
    document
}

fn with_support(base: Base, support: Value, declare: bool) -> Value {
    let mut document = base.document();
    let id = support["id"].as_str().unwrap().to_string();
    push(&mut document, "supports", support);
    if declare {
        declare_support_state(&mut document, &id);
    }
    document
}

/// Runs both entries in both modes and requires the same status from all four,
/// and identical diagnostics from all four when blocked (a solved envelope's
/// integrity report is mode-specific); returns the captured sparse-interactive
/// envelope.
fn run_all(document: &Value) -> Value {
    let mut outputs = Vec::new();
    for mode in MODES {
        let captured = run_linear_static_preview_value_with_mode(document.clone(), mode)
            .expect("the captured entry accepts the document");
        let typed = run_linear_static_preview_with_mode(
            serde_json::from_value(document.clone()).expect("the typed DTO deserializes"),
            mode,
        );
        outputs.push(serde_json::to_value(captured).unwrap());
        outputs.push(serde_json::to_value(typed).unwrap());
    }
    for output in &outputs[1..] {
        assert_eq!(output["status"], outputs[0]["status"]);
        if output["status"]["mechanics"] != "MECHANICS_SOLVED" {
            assert_eq!(
                output["diagnostics"], outputs[0]["diagnostics"],
                "entries or modes disagree"
            );
        }
    }
    outputs.swap_remove(0)
}

fn assert_blocked(envelope: &Value, base: Base) {
    assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE");
    assert_eq!(envelope["results"], json!([]));
    assert_eq!(envelope["contract_evidence"], base.empty_evidence());
    assert_eq!(envelope["producer"]["semantic_contract_id"], base.producer());
}

/// Index of the one diagnostic with `code` and exactly `refs`; checks its
/// pressure-runtime id, severity and source.
fn position(envelope: &Value, code: &str, refs: &[&str]) -> usize {
    let diagnostics = envelope["diagnostics"].as_array().unwrap();
    let matches = diagnostics
        .iter()
        .enumerate()
        .filter(|(_, d)| d["code"] == code && d["affected_refs"] == json!(refs))
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "{code} {refs:?} in {diagnostics:#?}");
    let (index, finding) = matches[0];
    assert_eq!(finding["severity"], "blocking");
    assert_eq!(finding["source"], SOURCE);
    assert_eq!(
        finding["id"],
        format!(
            "diagnostic:pressure-runtime:{}:{code}",
            refs.join(":").replace(':', "-")
        )
    );
    index
}

fn index_of(envelope: &Value, code: &str) -> usize {
    envelope["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .position(|d| d["code"] == code)
        .unwrap_or_else(|| panic!("{code} absent: {:#?}", envelope["diagnostics"]))
}

fn has_code(envelope: &Value, code: &str) -> bool {
    envelope["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .any(|d| d["code"] == code)
}

#[test]
fn a1_metadata_only_component_is_refused_by_composition() {
    for base in [Base::X0, Base::Y0] {
        let envelope = run_all(&with_valve(base));
        assert_blocked(&envelope, base);
        position(&envelope, COMPOSITION, &["component:valve"]);
    }
}

/// A1.2, the plan's negative control: a realized bend in an exact model is
/// refused by composition in both entries and both modes, not a panic. T4-U2a
/// lifts this refusal and owns the end-to-end form of the A3 wiring.
#[test]
fn a1_realized_bend_in_an_exact_model_is_refused_not_a_panic() {
    for base in [Base::X0, Base::Y0] {
        let mut document = base.document();
        push(&mut document, "components", bend(base));
        let envelope = run_all(&document);
        assert_blocked(&envelope, base);
        position(&envelope, COMPOSITION, &["component:bend"]);
    }
}

#[test]
fn a1_nonlinear_and_constant_effort_supports_are_refused_by_composition() {
    for base in [Base::X0, Base::Y0] {
        for (support, id) in [(gap(base), "support:gap"), (constant_effort(base), "support:ce")] {
            let envelope = run_all(&with_support(base, support, true));
            assert_blocked(&envelope, base);
            position(&envelope, COMPOSITION, &[id]);
            assert!(!has_code(&envelope, "LOAD_STATE_SUPPORT_STATE_MISSING"));
        }
    }
}

#[test]
fn a1_equivalent_static_is_refused_by_composition() {
    for base in [Base::X0, Base::Y0] {
        let envelope = run_all(&with_equivalent_static(base));
        assert_blocked(&envelope, base);
        position(
            &envelope,
            COMPOSITION,
            &[base.equivalent_static_case(), "equivalent_static"],
        );
    }
}

#[test]
fn a1_combination_is_refused_by_name() {
    for base in [Base::X0, Base::Y0] {
        let envelope = run_all(&with_combination(base));
        assert_blocked(&envelope, base);
        position(&envelope, COMBINATION, &["combination:sum"]);
    }
}

/// A1.7 and A1.8 pin today's objective-connector codes and their order before
/// the composition code. These are connector-code pins that T4-U3 re-agrees
/// (T4-RV2 N-7): T4-U3 changes the connector route and orders its legacy code
/// first.
#[test]
fn a1_objective_connector_codes_precede_the_composition_code() {
    for base in [Base::X0, Base::Y0] {
        for (version, code) in [
            ("1.0.0", "OBJECTIVE_CONNECTOR_NOT_IMPLEMENTED"),
            ("2.0.0", "OBJECTIVE_CONNECTOR_VERSION_UNSUPPORTED"),
        ] {
            let mut document = base.document();
            push(&mut document, "components", connector(base, version));
            let envelope = run_all(&document);
            assert_blocked(&envelope, base);
            let connector_code = position(
                &envelope,
                code,
                &["component:connector", "objective_connector"],
            );
            let composition = position(&envelope, COMPOSITION, &["component:connector"]);
            assert!(connector_code < composition, "{version}: connector code first");
            assert!(envelope["diagnostics"].as_array().unwrap().iter().any(|d| {
                d["code"] == "EXPANSION_JOINT_MECHANICS_INTERFACE_UNSUPPORTED"
                    && d["severity"] == "warning"
            }));
        }
    }
}

/// A1.9: on the pressure-free route (0.2.0, no pressure contract) the connector
/// is refused as a contract-version mismatch, and no exact composition code
/// applies.
#[test]
fn a1_objective_connector_outside_the_exact_route_is_a_version_mismatch() {
    let mut document = Base::X0.document();
    let model = &mut document["model"];
    model["schema_version"] = json!("0.2.0");
    model.as_object_mut().unwrap().remove("pressure_contract");
    for case in model["load_cases"].as_array_mut().unwrap() {
        case.as_object_mut().unwrap().remove("pressure_regions");
    }
    push(&mut document, "components", connector(Base::X0, "1.0.0"));
    let envelope = run_all(&document);
    assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE");
    assert_eq!(envelope["results"], json!([]));
    position(
        &envelope,
        "PREVIEW_CONTRACT_VERSION_MISMATCH",
        &["component:connector", "objective_connector"],
    );
    assert!(!has_code(&envelope, COMPOSITION));
}

#[test]
fn a1_precedence_follows_emission_order() {
    for base in [Base::X0, Base::Y0] {
        // Component composition before combination.
        let mut document = with_valve(base);
        document["model"]["combinations"] = json!([combination(base)]);
        let envelope = run_all(&document);
        assert_blocked(&envelope, base);
        assert!(
            position(&envelope, COMPOSITION, &["component:valve"])
                < position(&envelope, COMBINATION, &["combination:sum"])
        );

        // Component before support.
        let mut document = with_support(base, gap(base), true);
        push(&mut document, "components", valve(base));
        let envelope = run_all(&document);
        assert_blocked(&envelope, base);
        assert!(
            position(&envelope, COMPOSITION, &["component:valve"])
                < position(&envelope, COMPOSITION, &["support:gap"])
        );

        // Combination before the per-case equivalent_static code.
        let mut document = with_equivalent_static(base);
        document["model"]["combinations"] = json!([combination(base)]);
        let envelope = run_all(&document);
        assert_blocked(&envelope, base);
        assert!(
            position(&envelope, COMBINATION, &["combination:sum"])
                < position(
                    &envelope,
                    COMPOSITION,
                    &[base.equivalent_static_case(), "equivalent_static"],
                )
        );
    }

    // 0.4.0: the support's composition code precedes the load-state code that
    // `validate_document` adds for the undeclared support, in the same batch.
    let envelope = run_all(&with_support(Base::Y0, gap(Base::Y0), false));
    assert_blocked(&envelope, Base::Y0);
    assert!(
        position(&envelope, COMPOSITION, &["support:gap"])
            < index_of(&envelope, "LOAD_STATE_SUPPORT_STATE_MISSING")
    );
}
