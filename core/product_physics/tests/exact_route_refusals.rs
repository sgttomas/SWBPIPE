//! T4-U0 (exact-route hardening) public-entry controls.
//!
//! Before B7b these pinned the `2.0.0/exact_straight_pressure_v2` route's
//! composition, combination and negative-pressure refusals. B7b deleted that
//! contract, so the checks that protected only its codes are gone; the live
//! behaviours are kept on the current contract: the seam's emission order (now
//! `EXACT_PRESSURE_FAMILY_NOT_ADMITTED`), the pressure-free connector refusal
//! and the signed-zero pressure publication. Every input is a committed
//! invented fixture patched inline: X0 =
//! `fixtures/exact_pressure_connected_request.json` (without the
//! load/reference-state group), Y0 =
//! `fixtures/product_preview/pressure/load_reference/pressure.request.json`
//! (with it), both current model 0.8.0 documents under
//! `3.0.0/exact_pressure_v3`. "Both entries" are the captured value entry and
//! the typed entry; "both modes" are sparse-interactive and dense-scrutiny.

use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    PreviewSolverMode,
};
use open_pipe_stress_result_export::semantic_contract;
use serde_json::{json, Value};

const X0: &str = include_str!("fixtures/exact_pressure_connected_request.json");
const Y0: &str =
    include_str!("../../../fixtures/product_preview/pressure/load_reference/pressure.request.json");
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
const SOURCE: &str = "core/product_physics/src/pressure_runtime.rs";
const FAMILY: &str = "EXACT_PRESSURE_FAMILY_NOT_ADMITTED";
const NEGATIVE: &str = "PRESSURE_REGION_PRESSURE_NEGATIVE";
const LOAD_STATE_EQUIVALENT_STATIC: &str = "EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED";
const PRESSURE_ID: &str = "openpipestress.result_semantics/0.3.0/pressure-2";
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

/// Cases with the load/reference-state group declare every support's
/// participation; the added support gets one so that no load-state code is
/// added.
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

/// A still-refused component family (W-15 admits the valve and RT-A3 the tee,
/// the earlier samples).
fn other(base: Base) -> Value {
    json!({"id":"component:other","kind":"other","node":base.tip(),"provenance":PROVENANCE})
}

fn gap(base: Base) -> Value {
    json!({"id":"support:gap","node":base.tip(),"family":"nonlinear","restraints":[],
        "nonlinear":{"behavior":"gap","dof":"UZ","initial_state":"inactive",
            "closes_when":"positive_displacement","gap":{"value":1000.0,"unit":"mm"}},
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

fn with_other(base: Base) -> Value {
    let mut document = base.document();
    push(&mut document, "components", other(base));
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

fn set_region_pressure(document: &mut Value, case: &str, pressure: Value) {
    case_mut(document, case)["pressure_regions"][0]["pressure"] = pressure;
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
    assert_eq!(envelope["producer"]["semantic_contract_id"], PRESSURE_ID);
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

/// A1.9: on the pressure-free route (0.8.0, no pressure contract) the connector
/// is refused as a contract-version mismatch, and no exact family code
/// applies. The exact document's allowances leave with its contract.
#[test]
fn a1_objective_connector_outside_the_exact_route_is_a_version_mismatch() {
    let mut document = Base::X0.document();
    let model = &mut document["model"];
    assert_eq!(model["schema_version"], "0.8.0");
    model.as_object_mut().unwrap().remove("pressure_contract");
    for case in model["load_cases"].as_array_mut().unwrap() {
        case.as_object_mut().unwrap().remove("pressure_regions");
    }
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"].as_object_mut().unwrap().remove("corrosion_allowance");
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
    assert!(!has_code(&envelope, FAMILY));
}

/// The seam's refusals keep the route's emission order (component, support,
/// then the per-case equivalent_static code: since W-16 slice c, the
/// load/reference-state refusal; since W-16 slice b a combination is admitted
/// and never named), under the current contract's codes; with the load/reference-state group, the
/// support's family code precedes the load-state code that `validate_document`
/// adds for the undeclared support, in the same batch.
#[test]
fn a1_precedence_follows_emission_order() {
    for base in [Base::X0, Base::Y0] {
        // A component is refused; W-16 slice b admits the combination beside it.
        let mut document = with_other(base);
        document["model"]["combinations"] = json!([combination(base)]);
        let envelope = run_all(&document);
        assert_blocked(&envelope, base);
        position(&envelope, FAMILY, &["component:other"]);
        assert!(!envelope["diagnostics"].as_array().unwrap().iter()
            .any(|d| d["code"] == FAMILY && d["affected_refs"] == json!(["combination:sum"])));

        // Component before support.
        let mut document = with_support(base, gap(base), true);
        push(&mut document, "components", other(base));
        let envelope = run_all(&document);
        assert_blocked(&envelope, base);
        assert!(
            position(&envelope, FAMILY, &["component:other"])
                < position(&envelope, FAMILY, &["support:gap"])
        );

        // A combination beside the per-case equivalent_static code. W-16 slice
        // c admits generation at the seam and slice b the combination; the
        // per-case code is now the load/reference-state refusal, which only Y0
        // (with the group) takes. On X0 nothing refuses at the seam any more
        // (the incomplete generation input is reported past it), so only Y0
        // is a seam case here.
        if matches!(base, Base::Y0) {
            let mut document = with_equivalent_static(base);
            document["model"]["combinations"] = json!([combination(base)]);
            let envelope = run_all(&document);
            let case_refs = [base.equivalent_static_case(), "equivalent_static"];
            assert_blocked(&envelope, base);
            assert!(!has_code(&envelope, FAMILY));
            position(&envelope, LOAD_STATE_EQUIVALENT_STATIC, &case_refs);
        }
    }

    let envelope = run_all(&with_support(Base::Y0, gap(Base::Y0), false));
    assert_blocked(&envelope, Base::Y0);
    assert!(
        position(&envelope, FAMILY, &["support:gap"])
            < index_of(&envelope, "LOAD_STATE_SUPPORT_STATE_MISSING")
    );
}

/// A2.4 and A2.5: both signed zeros are admitted. An authored Pa value is
/// published unchanged, so -0.0 Pa publishes p_pa = -0.0; a scaled unit goes
/// through the affine unit transform (offset +0.0), so -0.0 kPa publishes
/// +0.0, as it did before T4-U0. Byte equality with the base is the SP-1
/// probe's, not this test's.
#[test]
fn a2_signed_zero_pressure_is_admitted() {
    for (value, unit, negative_zero) in [
        (json!(-0.0), "Pa", true),
        (json!(-0.0), "kPa", false),
        (json!(0.0), "kPa", false),
    ] {
        let mut document = Base::X0.document();
        set_region_pressure(
            &mut document,
            "case:closed-pressure",
            json!({"value":value,"unit":unit}),
        );
        let envelope = run_all(&document);
        assert_eq!(
            envelope["status"]["mechanics"], "MECHANICS_SOLVED",
            "{:#?}",
            envelope["diagnostics"]
        );
        assert!(!has_code(&envelope, NEGATIVE));
        let region = envelope["contract_evidence"]["pressure"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["region_id"] == "region:fixture-pressure")
            .expect("region evidence");
        let p_pa = region["p_pa"].as_f64().unwrap();
        assert_eq!(p_pa, 0.0);
        assert_eq!(p_pa.is_sign_negative(), negative_zero, "{value} {unit}");
        // The accepted Rust reader admits the envelope.
        semantic_contract::for_source(&envelope).expect("the Rust reader admits p_pa = +-0");
    }
}
