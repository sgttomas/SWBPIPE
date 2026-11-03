//! I2a unit tests (DEL-04-09): the request, the state case's post-lowering
//! binding, the preparation's parity with the static pre-formation checks, the
//! mass against the frozen `validation/references/f1_modal/mass_discrete`
//! references, the weight-basis identity and its D-TOL measurement, the W/a
//! range premise and the self-weight inspection seam. All models are invented.
use super::context::{primitive_load_difference, BoundCase, ModalCaseContext, WindPresence};
use super::mass::{assemble_mass, mass_bearing_members, ModalMass};
use super::prepare::{prepare_modal, Prepared};
use super::request::ModalPreviewRequest;
use super::weight_basis::{check_weight_basis, IdentityReport, IdentityTolerance};
use super::*;
use crate::self_weight::{
    generate_self_weight_operations, inspect_generated_self_weight, SelfWeightGravity, SelfWeightRequest,
};
use crate::{has_blocking, operating_conditions, Diagnostic, LinearStaticPreviewRequest, PreviewModel};
use serde_json::{json, Value};

#[path = "../../tests/common/condition_twin.rs"]
mod twin;
#[path = "../../tests/common/current_model.rs"]
mod current_model;

/// **TEST-LOCAL, NON-PRODUCTION bound.** D-TOL (the weight-basis identity
/// tolerance, Design §2.5) is experimental and unpinned: this pair exists only
/// for these tests' assertions. It is not an admission tolerance, it pins
/// nothing, and no production path reads it.
pub(super) const TEST_LOCAL_IDENTITY_BOUND: IdentityTolerance = IdentityTolerance { relative: 1e-12, absolute: 0.0 };
/// **TEST-LOCAL, NON-PRODUCTION bound** for the product mass against the
/// frozen mass references (normwise, relative to the largest reference entry).
const TEST_LOCAL_MASS_BOUND: f64 = 1e-13;

pub(super) const PROV: &str = "invented DEL-04-09 I2a test input";
const MASS_REFERENCES: &str = include_str!("../../../../validation/references/f1_modal/mass_discrete/mass_references.json");
const U3_REFERENCES: &str = include_str!("../../../../validation/references/t4_i12/u3_reference_cases.json");
const PF_DEMO: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");
const EX_REQUEST: &str = include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json");
const LOAD_STATE_REQUEST: &str = include_str!("../../../../fixtures/product_preview/pressure/load_reference/connected.request.json");
const INTERMEDIATE_STOP: &str = include_str!("../../tests/fixtures/model_checks/intermediate_stop_witness.json");
const HASH: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

// ---------------------------------------------------------------------------
// Builders
// ---------------------------------------------------------------------------

pub(super) fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

pub(super) fn codes(found: &[Diagnostic]) -> Vec<&str> {
    found.iter().map(|d| d.code.as_str()).collect()
}

pub(super) fn modal(value: Value) -> ModalPreviewRequest {
    ModalPreviewRequest::parse_modal_request(value).unwrap_or_else(|refusal| panic!("{refusal}"))
}

/// `request` with the modal members `members` added.
pub(super) fn with(mut request: Value, members: Value) -> Value {
    for (key, value) in members.as_object().unwrap() {
        request[key] = value.clone();
    }
    request
}

pub(super) fn section() -> Value {
    json!({"outside_diameter": q(0.2, "m"), "wall_thickness": q(0.01, "m"), "material_density": q(7850.0, "kg/m^3"),
        "contents_density": q(1000.0, "kg/m^3")})
}

/// a(0) – b(1) – c(2) along X, anchored at a, a load on c; with `valve`, a
/// valve at c maps body:v (W = 500 N, r = (−0.5, 0.25, 0) m).
pub(super) fn pf_model(valve: bool) -> Value {
    let node = |id: &str, x: f64| json!({"id": id, "position": {"x": x, "y": 0, "z": 0}, "provenance": PROV});
    let pipe = |id: &str, from: &str, to: &str| json!({"id": id, "from": from, "to": to, "section": section(), "material": "m",
        "y_reference": {"x": 0, "y": 1, "z": 0}, "provenance": PROV});
    let components = if valve {
        json!([{"id": "component:valve", "kind": "valve", "node": "c",
            "geometry": {"rigid_pipe_ref": "body:v", "weight": q(500.0, "N"), "center_of_gravity": {"x": -0.5, "y": 0.25, "z": 0, "unit": "m"}},
            "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROV}])
    } else {
        json!([])
    };
    json!({"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:i2a", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [node("a", 0.0), node("b", 1.0), node("c", 2.0)],
        "pipe_segments": [pipe("pipe:p1", "a", "b"), pipe("body:v", "b", "c")],
        "materials": [{"id": "m", "elastic_modulus": q(200e9, "Pa"), "shear_modulus": q(80e9, "Pa"), "provenance": PROV}],
        "supports": [{"id": "s", "node": "a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}],
        "components": components,
        "load_cases": [{"id": "case:f", "primitive_loads": [{"id": "load:f", "category": "concentrated_force", "target": {"type": "node", "node": "c"},
            "direction": "global_y", "magnitude": q(1000.0, "N"), "dimension": "force", "provenance": PROV}], "provenance": PROV}],
        "combinations": []})
}

pub(super) fn pf_request(valve: bool) -> Value {
    json!({"model": pf_model(valve), "materials": []})
}

/// K5's arc model shape: a – b straight, b – c a realized 90° bend of radius
/// 0.25 m, c – d straight; densities added; `gap` adds an open gap support.
pub(super) fn arc_request(gap: bool) -> Value {
    let node = |id: &str, x: f64, y: f64| json!({"id": id, "position": {"x": x, "y": y, "z": 0.0}, "provenance": PROV});
    let pipe = |id: &str, from: &str, to: &str, y: [f64; 3]| json!({"id": id, "from": from, "to": to, "section": section(),
        "material": "m", "y_reference": {"x": y[0], "y": y[1], "z": y[2]}, "provenance": PROV});
    let mut supports = vec![
        json!({"id": "support:a", "node": "a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}),
        json!({"id": "support:d", "node": "d", "family": "anchor", "restraints": ["UX", "UY", "UZ"], "provenance": PROV}),
    ];
    if gap {
        supports.push(json!({"id": "support:gap", "node": "b", "family": "nonlinear", "restraints": [],
            "nonlinear": {"behavior": "gap", "dof": "UZ", "initial_state": "inactive", "closes_when": "positive_displacement", "gap": q(1000.0, "mm")},
            "provenance": PROV}));
    }
    let h = std::f64::consts::FRAC_1_SQRT_2;
    json!({"model": {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:i2a-arc", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [node("a", 0.0, 0.0), node("b", 1.0, 0.0), node("c", 1.25, 0.25), node("d", 1.25, 1.25)],
        "pipe_segments": [pipe("pipe:a-b", "a", "b", [0.0, 1.0, 0.0]), pipe("pipe:b-c", "b", "c", [h, -h, 0.0]), pipe("pipe:c-d", "c", "d", [1.0, 0.0, 0.0])],
        "supports": supports,
        "components": [{"id": "component:bend", "kind": "bend", "node": "c",
            "geometry": {"bend_pipe_ref": "pipe:b-c", "bend_radius": q(0.25, "m"), "bend_geometry_source_reference": PROV},
            "modifiers": {"flexibility_factor_user_value": q(2.0, "none"), "source_reference": PROV},
            "mechanics_interface": {"solver_consumption": "curved_bend_macro_element", "rule_check_consumption": "user_rule_pack_inputs_only"},
            "provenance": PROV}],
        "materials": [{"id": "m", "elastic_modulus": q(200e9, "Pa"), "shear_modulus": q(76.923076923e9, "Pa"), "provenance": PROV}],
        "load_cases": [{"id": "case:z", "primitive_loads": [{"id": "z", "category": "concentrated_force", "target": {"type": "node", "node": "b"},
            "direction": "global_z", "magnitude": q(1000.0, "N"), "dimension": "force", "provenance": PROV}], "provenance": PROV}],
        "combinations": []},
        "materials": []})
}

fn prepared(request: &ModalPreviewRequest) -> Prepared<'_> {
    prepare_modal(request).unwrap_or_else(|found| panic!("prepare refused: {found:#?}"))
}

/// Prepare, form the mass and run the identity with the test-local bound.
fn identity(request: &ModalPreviewRequest) -> (ModalMass, IdentityReport) {
    let prepared = prepared(request);
    let mass = assemble_mass(&prepared, prepared.inputs.weight_to_mass_acceleration)
        .unwrap_or_else(|found| panic!("mass refused: {found:#?}"));
    let report = check_weight_basis(&prepared, &mass, TEST_LOCAL_IDENTITY_BOUND).unwrap_or_else(|found| panic!("identity refused: {found:#?}"));
    (mass, report)
}

// ---------------------------------------------------------------------------
// The state case's binding (context items 1–4)
// ---------------------------------------------------------------------------

fn twin_model() -> PreviewModel {
    serde_json::from_value(twin::document(crate::MODEL_SCHEMA_VERSION)).unwrap()
}

fn lowered(model: &PreviewModel) -> PreviewModel {
    let mut lowered = model.clone();
    let mut found = Vec::new();
    operating_conditions::lower(&mut lowered, &mut found);
    assert!(!has_blocking(&found), "{found:#?}");
    lowered
}

fn bind(model: &PreviewModel, case: &str) -> Result<BoundCase, Diagnostic> {
    ModalCaseContext::capture_state_case(model, case).unwrap().bind_state_case(&lowered(model))
}

fn index(model: &PreviewModel, case: &str) -> usize {
    model.load_cases.iter().position(|c| c.id == case).unwrap()
}

fn refused(result: Result<BoundCase, Diagnostic>, needle: &str) {
    let refusal = result.expect_err("the binding must refuse");
    assert_eq!(refusal.code, MODAL_STATE_CASE_UNAVAILABLE);
    assert!(refusal.message.contains(needle), "{needle:?} not in: {}", refusal.message);
}

#[test]
fn context_binds_a_plain_case_with_no_condition_use() {
    let model = twin_model();
    let bound = bind(&model, "PLAIN").unwrap();
    assert_eq!(bound.case_index, 3);
    assert_eq!((bound.authored, bound.generated_temperatures, bound.generated_added_weights), (0..2, 2..2, 2..2));
    assert_eq!(bound.wind, WindPresence::Absent);
    assert!(!bound.condition_use_authored);
}

#[test]
fn context_binds_temperature_then_added_weight_in_pipe_order() {
    let model = twin_model();
    let bound = bind(&model, "OPE").unwrap();
    // Three authored weights; a temperature record per pipe; added weight on
    // the pipes whose sets carry one (cs:hot on P2, cs:cold on P3).
    assert_eq!((bound.authored, bound.generated_temperatures, bound.generated_added_weights), (0..3, 3..6, 6..8));
    assert!(bound.condition_use_authored);
    let sus = bind(&model, "SUS").unwrap();
    assert_eq!((sus.generated_temperatures, sus.generated_added_weights), (0..0, 0..2));
}

#[test]
fn context_distinguishes_every_wind_presence() {
    let model = twin_model();
    // Resolved with refs: cs:hot (P2) and cs:cold (P3) mark wind; cs:warm does not.
    assert_eq!(bind(&model, "WIND").unwrap().wind, WindPresence::Resolved(vec!["P2".into(), "P3".into()]));
    // Resolved, empty: no set marks wind.
    let mut document = twin::document(crate::MODEL_SCHEMA_VERSION);
    for set in document["condition_sets"].as_array_mut().unwrap() {
        set["exposure"] = json!({"wind": false});
    }
    let none: PreviewModel = serde_json::from_value(document.clone()).unwrap();
    assert_eq!(bind(&none, "WIND").unwrap().wind, WindPresence::Resolved(vec![]));
    // Authored (a list, and an empty list), and unstated, with no condition use.
    let wind = |refs: Option<Value>| {
        let mut document = document.clone();
        let mut wind = json!({"pressure": q(1200.0, "Pa"), "shape_factor": q(0.7, "1"), "direction": "global_y"});
        if let Some(refs) = refs {
            wind["exposed_pipe_refs"] = refs;
        }
        document["load_cases"][3]["equivalent_static"] = json!({"wind": wind, "provenance": PROV});
        let model: PreviewModel = serde_json::from_value(document).unwrap();
        bind(&model, "PLAIN").unwrap().wind
    };
    assert_eq!(wind(Some(json!(["P1"]))), WindPresence::Authored(vec!["P1".into()]));
    assert_eq!(wind(Some(json!([]))), WindPresence::Authored(vec![]));
    assert_eq!(wind(None), WindPresence::Unstated);
    // Absent.
    assert_eq!(bind(&model, "OPE").unwrap().wind, WindPresence::Absent);
}

#[test]
fn context_refuses_a_tampered_authored_prefix() {
    let model = twin_model();
    let context = ModalCaseContext::capture_state_case(&model, "OPE").unwrap();
    let ope = index(&model, "OPE");
    // One bit of an authored magnitude.
    let mut tampered = lowered(&model);
    let value = &mut tampered.load_cases[ope].primitive_loads[2].magnitude.value;
    *value = f64::from_bits(value.to_bits() + 1);
    refused(context.bind_state_case(&tampered), "differs in its magnitude.value");
    // An authored provenance.
    let mut tampered = lowered(&model);
    tampered.load_cases[ope].primitive_loads[0].provenance = Some("edited".into());
    refused(context.bind_state_case(&tampered), "differs in its provenance");
    // Another member of the case.
    let mut tampered = lowered(&model);
    tampered.load_cases[ope].provenance = Some("edited".into());
    refused(context.bind_state_case(&tampered), "other than its primitive loads");
    // The field-wise comparison itself.
    let load = &model.load_cases[ope].primitive_loads[0];
    let mut other = load.clone();
    assert_eq!(primitive_load_difference(load, &other), None);
    other.magnitude.value = -0.0 * load.magnitude.value.signum();
    assert_eq!(primitive_load_difference(load, &other), Some("magnitude.value"));
}

#[test]
fn context_refuses_any_generated_suffix_but_s2s_exact_output() {
    let model = twin_model();
    let context = ModalCaseContext::capture_state_case(&model, "OPE").unwrap();
    let ope = index(&model, "OPE");
    let tamper = |edit: &dyn Fn(&mut Vec<crate::PreviewPrimitiveLoad>)| {
        let mut tampered = lowered(&model);
        edit(&mut tampered.load_cases[ope].primitive_loads);
        context.bind_state_case(&tampered)
    };
    refused(tamper(&|loads| loads.push(loads[7].clone())), "appends 6 records");
    refused(tamper(&|loads| { loads.pop(); }), "appends 4 records");
    refused(tamper(&|loads| loads.swap(3, 4)), "generated record 3");
    refused(tamper(&|loads| loads.swap(5, 6)), "generated record 5");
    refused(tamper(&|loads| loads[4].id.push('x')), "its id differs");
    refused(tamper(&|loads| loads[6].provenance = Some(crate::operating_conditions::resolution::added_weight_provenance("cs:cold"))), "its provenance differs");
    refused(tamper(&|loads| loads[3].direction = "global_x".into()), "its direction differs");
    refused(tamper(&|loads| loads[7].magnitude.value = 1.0), "S2 copies the set's added_weight verbatim");
    // A temperature magnitude is S2's output: it is bound by identity and
    // provenance, never recomputed here.
    assert!(tamper(&|loads| loads[3].magnitude.value = 1.0).is_ok());
}

#[test]
fn context_refuses_missing_or_contradictory_evidence() {
    let model = twin_model();
    let context = ModalCaseContext::capture_state_case(&model, "OPE").unwrap();
    // An unlowered model is no evidence that the lowering ran.
    refused(context.bind_state_case(&model), "the lowering has not run");
    // A model with another case at the state case's place.
    let mut moved = lowered(&model);
    moved.load_cases.swap(0, 1);
    refused(context.bind_state_case(&moved), "not the state case");
    let mut fewer = lowered(&model);
    fewer.load_cases.pop();
    refused(context.bind_state_case(&fewer), "load cases");
    // Reordered pipes.
    let mut pipes = lowered(&model);
    pipes.pipe_segments.swap(0, 1);
    refused(context.bind_state_case(&pipes), "pipe segments");
    // A wind exposure other than the resolved one.
    let wind = ModalCaseContext::capture_state_case(&model, "WIND").unwrap();
    let mut tampered = lowered(&model);
    let at = index(&model, "WIND");
    tampered.load_cases[at].equivalent_static.as_mut().unwrap().wind.as_mut().unwrap().exposed_pipe_refs.reverse();
    refused(wind.bind_state_case(&tampered), "wind exposure");
    // No such case.
    assert_eq!(ModalCaseContext::capture_state_case(&model, "NONE").unwrap_err().code, MODAL_STATE_CASE_UNAVAILABLE);
}

// ---------------------------------------------------------------------------
// The request (positive bounds, state requirements, modulus conflict)
// ---------------------------------------------------------------------------

fn request_codes(members: Value) -> Vec<String> {
    match modal(with(pf_request(false), members)).validate_modal_request() {
        Ok(_) => vec![],
        Err(found) => found.into_iter().map(|d| d.code).collect(),
    }
}

#[test]
fn request_requires_a_positive_extraction_bound() {
    assert_eq!(request_codes(json!({})), [MODAL_EXTRACTION_BOUND_REQUIRED]);
    assert_eq!(request_codes(json!({"mode_count": 0})), [MODAL_EXTRACTION_BOUND_INVALID]);
    assert_eq!(request_codes(json!({"frequency_cutoff": q(0.0, "Hz")})), [MODAL_EXTRACTION_BOUND_INVALID]);
    assert_eq!(request_codes(json!({"frequency_cutoff": q(-5.0, "Hz")})), [MODAL_EXTRACTION_BOUND_INVALID]);
    assert_eq!(request_codes(json!({"frequency_cutoff": q(5.0, "m")})), [MODAL_EXTRACTION_BOUND_INVALID]);
    // Angular frequency is another dimension of the catalog.
    assert_eq!(request_codes(json!({"frequency_cutoff": q(5.0, "rad/s")})), [MODAL_EXTRACTION_BOUND_INVALID]);
    let found = modal(with(pf_request(false), json!({"frequency_cutoff": q(5.0, "m")}))).validate_modal_request().unwrap_err();
    assert!(found[0].message.contains("`m`") && found[0].message.contains("frequency"), "{}", found[0].message);
    // A negative count is not a count: the closed request refuses it.
    assert!(ModalPreviewRequest::parse_modal_request(with(pf_request(false), json!({"mode_count": -1}))).is_err());
    assert!(ModalPreviewRequest::parse_modal_request(with(pf_request(false), json!({"mode_count": 1, "solver": "x"}))).is_err());
    let inputs = modal(with(pf_request(false), json!({"mode_count": 3, "frequency_cutoff": q(40.0, "Hz")}))).validate_modal_request().unwrap();
    assert_eq!((inputs.mode_count, inputs.frequency_cutoff_hz), (Some(3), Some(40.0)));
}

#[test]
fn request_weight_to_mass_acceleration_is_a_positive_acceleration() {
    for bad in [q(0.0, "m/s^2"), q(-9.8125, "m/s^2"), q(9.8125, "m")] {
        assert_eq!(request_codes(json!({"mode_count": 1, "weight_to_mass_acceleration": bad})), [MODAL_WEIGHT_ACCELERATION_INVALID]);
    }
    let inputs = modal(with(pf_request(false), json!({"mode_count": 1, "weight_to_mass_acceleration": q(9.8125, "m/s^2")}))).validate_modal_request().unwrap();
    assert_eq!(inputs.weight_to_mass_acceleration, Some(9.8125));
}

#[test]
fn request_state_case_is_required_where_section_1_4_requires_it() {
    // A load/reference-state document.
    let load_state: Value = serde_json::from_str(LOAD_STATE_REQUEST).unwrap();
    let found = modal(with(load_state.clone(), json!({"mode_count": 1}))).validate_modal_request().unwrap_err();
    assert_eq!(codes(&found), [MODAL_STATE_CASE_REQUIRED]);
    assert!(found[0].message.contains("load/reference-state document"), "{}", found[0].message);
    assert!(modal(with(load_state.clone(), json!({"mode_count": 1, "state_case": "case:cold"}))).validate_modal_request().is_ok());
    // A model with a nonlinear support.
    let found = modal(with(arc_request(true), json!({"mode_count": 1}))).validate_modal_request().unwrap_err();
    assert_eq!(codes(&found), [MODAL_STATE_CASE_REQUIRED]);
    assert!(found[0].message.contains("nonlinear supports (support:gap)"), "{}", found[0].message);
    // Optional on a plain document; a name that is not a case is unavailable.
    assert!(request_codes(json!({"mode_count": 1})).is_empty());
    assert!(request_codes(json!({"mode_count": 1, "state_case": "case:f"})).is_empty());
    assert_eq!(request_codes(json!({"mode_count": 1, "state_case": "case:none"})), [MODAL_STATE_CASE_UNAVAILABLE]);
}

#[test]
fn request_modulus_basis_conflicts_with_a_state_case() {
    assert_eq!(request_codes(json!({"mode_count": 1, "state_case": "case:f", "modulus_basis_ref": "point:hot"})), [MODAL_MODULUS_BASIS_CONFLICT]);
    assert_eq!(
        request_codes(json!({"mode_count": 1, "state_case": "case:f", "modulus_basis_temperature": q(300.0, "degC")})),
        [MODAL_MODULUS_BASIS_CONFLICT]
    );
    // Without a state case the request's basis is resolved by the static resolver.
    let found = prepare_modal(&modal(with(pf_request(false), json!({"mode_count": 1, "modulus_basis_ref": "point:none"})))).err().unwrap();
    assert!(codes(&found).iter().any(|c| c.starts_with("MODULUS_BASIS")), "{found:#?}");
    // The internal case that carries the request's basis is named in no diagnostic.
    let text = serde_json::to_string(&found).unwrap();
    assert!(!text.contains("request-modulus-basis"), "{text}");
    assert!(!text.contains(&crate::stable_suffix("modal:request-modulus-basis")), "{text}");
}

// ---------------------------------------------------------------------------
// Preparation parity with the static pre-formation checks
// ---------------------------------------------------------------------------

/// Two comparisons, neither a proof that the sequences are identical:
/// (1) every diagnostic against `pre_formation_checks`, which is itself a
/// mirror of the static entry's pre-formation steps (not the entry); and
/// (2) the decision against the actual static entry
/// (`run_linear_static_preview_value_with_mode`) on the same request: it is
/// blocked exactly when preparation refuses, with the same blocking
/// diagnostics. They hold only on the fixtures given.
fn parity(request: Value, members: Value) {
    let static_found = LinearStaticPreviewRequest::pre_formation_checks_value(request.clone()).unwrap();
    let entry = crate::run_linear_static_preview_value_with_mode(request.clone(), crate::PreviewSolverMode::DenseScrutiny)
        .unwrap_or_else(|refusal| panic!("the static entry refused the request: {refusal}"));
    let modal_request = modal(with(request, members));
    let prepared = prepare_modal(&modal_request);
    let modal_found = match &prepared {
        Ok(prepared) => prepared.diagnostics.clone(),
        Err(found) => found.clone(),
    };
    let key = |d: &Diagnostic| (d.id.clone(), d.code.clone(), d.severity.clone(), d.affected_refs.clone());
    let static_keys: Vec<_> = static_found.iter().map(key).collect();
    let modal_keys: Vec<_> = modal_found.iter().filter(|d| !d.code.starts_with("MODAL_")).map(key).collect();
    assert_eq!(modal_keys, static_keys, "against the mirrored pre-formation checks");
    let blocking = |found: &[Diagnostic]| -> Vec<_> {
        found.iter().filter(|d| d.severity == "blocking" && !d.code.starts_with("MODAL_")).map(key).collect()
    };
    assert_eq!(prepared.is_err(), has_blocking(&entry.diagnostics), "admitted or refused as the static entry decides: {:#?}", entry.diagnostics);
    assert_eq!(blocking(&modal_found), blocking(&entry.diagnostics), "the static entry's blocking diagnostics");
}

#[test]
fn prepare_agrees_with_the_mirrored_checks_and_the_static_entrys_decision() {
    // PF: the bundled demo, and the intermediate-stop witness made invalid as
    // the model-checks corpus makes it.
    // The demo's nonlinear supports would need a state case and stop at the
    // interim refusal; without them the whole pre-formation sequence runs.
    let mut demo: Value = serde_json::from_str(PF_DEMO).unwrap();
    demo["supports"].as_array_mut().unwrap().retain(|s| s.get("nonlinear").is_none());
    parity(json!({"model": demo, "materials": []}), json!({"mode_count": 1}));
    let mut stop: Value = serde_json::from_str(INTERMEDIATE_STOP).unwrap();
    stop["model"]["supports"][0]["family"] = json!("review_unknown");
    stop["model"]["pipe_segments"][0]["section"]["material_density"]["value"] = json!(15700.0);
    parity(stop, json!({"mode_count": 1}));
    parity(pf_request(true), json!({"mode_count": 1}));
    // EX.
    parity(serde_json::from_str(EX_REQUEST).unwrap(), json!({"mode_count": 1}));
    // A load/reference-state document, with its state case.
    let load_state: Value = serde_json::from_str(LOAD_STATE_REQUEST).unwrap();
    parity(load_state.clone(), json!({"mode_count": 1, "state_case": "case:hot"}));
    let prepared_state = modal(with(load_state, json!({"mode_count": 1, "state_case": "case:hot"})));
    let bound = prepared(&prepared_state).bound_case.unwrap();
    assert_eq!(bound.case_id, "case:hot");
}

#[test]
fn a_static_refusal_of_the_state_case_refuses_it_by_name() {
    // A load-state document whose state case does not resolve: the static
    // resolver's refusal is carried.
    let mut load_state: Value = serde_json::from_str(LOAD_STATE_REQUEST).unwrap();
    for state in load_state["model"]["load_cases"][1]["analysis_state"]["element_states"].as_array_mut().unwrap() {
        state["material_selection"]["point_ref"] = json!("point:none");
    }
    let found = prepare_modal(&modal(with(load_state, json!({"mode_count": 1, "state_case": "case:hot"})))).err().unwrap();
    let last = found.last().unwrap();
    assert_eq!(last.code, MODAL_STATE_CASE_UNAVAILABLE, "{found:#?}");
    assert!(last.message.contains("static path's own admission"), "{}", last.message);
    let carried = &found[found.len() - 2];
    assert!(carried.severity == "blocking" && last.message.contains(&carried.code), "{found:#?}");
}

#[test]
fn a_contents_density_state_case_is_refused_upstream() {
    // A condition-contents use on the state case: refused by the resolution's
    // interim refusal (no modal bypass), and carried by name.
    let mut document = twin::document(crate::MODEL_SCHEMA_VERSION);
    document["condition_sets"][0]["conditions"][0]["contents_density"] = q(1000.0, "kg/m^3");
    document["load_cases"][1]["condition_use"] = json!({"contents_density": "OPE1"});
    let request = json!({"model": document, "materials": [], "mode_count": 1, "state_case": "SUS"});
    let found = prepare_modal(&modal(request)).err().unwrap();
    assert!(codes(&found).contains(&"LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED"), "{found:#?}");
    assert_eq!(found.last().unwrap().code, MODAL_STATE_CASE_UNAVAILABLE);
    assert!(found.last().unwrap().message.contains("LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED"));
}

#[test]
fn interim_refusals_are_by_name() {
    let found = prepare_modal(&modal(with(arc_request(true), json!({"mode_count": 1, "state_case": "case:z"})))).err().unwrap();
    assert_eq!(found.last().unwrap().code, MODAL_NONLINEAR_SUPPORT_INTERIM_UNSUPPORTED, "{found:#?}");
    let mut timoshenko = pf_request(false);
    timoshenko["model"]["model_settings"] = json!({"shear_model": "timoshenko_annulus_energy_v1"});
    timoshenko["model"]["materials"][0]["poisson_ratio"] = q(0.25, "1");
    let found = prepare_modal(&modal(with(timoshenko, json!({"mode_count": 1})))).err().unwrap();
    assert_eq!(found.last().unwrap().code, MODAL_TIMOSHENKO_MASS_UNSUPPORTED, "{found:#?}");
}

// ---------------------------------------------------------------------------
// The frozen mass references and the D-TOL measurement
// ---------------------------------------------------------------------------

fn references() -> Value {
    serde_json::from_str(MASS_REFERENCES).unwrap()
}

fn reference_case(id: &str) -> Value {
    references()["cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap().clone()
}

pub(super) fn number(value: &Value) -> f64 {
    value.as_f64().unwrap_or_else(|| value.as_str().unwrap().parse().unwrap())
}

/// A frozen neutral mass-reference model as a product request on `basis`
/// (`PF` or `EX`), with the request's acceleration; `None` when it holds a
/// family this converter does not author (connectors, tie rods).
pub(super) fn reference_request(case: &Value, basis: &str) -> Option<Value> {
    let m = &case["model"];
    let exact = basis == "EX";
    let material = m["materials"].as_object().unwrap().iter().next().unwrap();
    let (e, g, rho) = (number(&material.1["elastic_modulus_Pa"]), number(&material.1["shear_modulus_Pa"]), number(&material.1["density_kg_per_m3"]));
    let nodes: Vec<Value> = m["nodes"].as_array().unwrap().iter()
        .map(|n| json!({"id": n["id"], "position": {"x": n["xyz"][0], "y": n["xyz"][1], "z": n["xyz"][2]}, "provenance": PROV}))
        .collect();
    let mut pipes = Vec::new();
    let mut components = Vec::new();
    for member in m["members"].as_array().unwrap() {
        let kind = member["kind"].as_str().unwrap();
        if kind != "straight" && kind != "arc" {
            return None;
        }
        let s = &m["sections"][member["section"].as_str().unwrap()];
        let mut section = json!({"outside_diameter": q(number(&s["outside_diameter_m"]), "m"), "wall_thickness": q(number(&s["wall_thickness_m"]), "m"),
            "mill_tolerance": q(number(&s["mill_tolerance_m"]), "m"), "material_density": q(rho, "kg/m^3")});
        if exact {
            section["corrosion_allowance"] = q(number(&s["corrosion_allowance_m"]), "m");
        }
        if !s["contents_density_kg_per_m3"].is_null() {
            section["contents_density"] = q(number(&s["contents_density_kg_per_m3"]), "kg/m^3");
        }
        if !s["insulation_thickness_m"].is_null() {
            section["insulation_thickness"] = q(number(&s["insulation_thickness_m"]), "m");
            section["insulation_density"] = q(number(&s["insulation_density_kg_per_m3"]), "kg/m^3");
        }
        let y = &member["y_reference"];
        pipes.push(json!({"id": member["id"], "from": member["i"], "to": member["j"], "section": section, "material": material.0,
            "y_reference": {"x": y[0], "y": y[1], "z": y[2]}, "provenance": PROV}));
        if kind == "arc" {
            components.push(json!({"id": format!("bend:{}", member["id"].as_str().unwrap()), "kind": "bend", "node": member["j"],
                "geometry": {"bend_pipe_ref": member["id"], "bend_radius": q(number(&member["radius_m"]), "m"), "bend_geometry_source_reference": PROV},
                "modifiers": {"flexibility_factor_user_value": q(number(&member["flexibility_factor"]), "none"), "source_reference": PROV},
                "mechanics_interface": {"solver_consumption": "curved_bend_macro_element", "rule_check_consumption": "user_rule_pack_inputs_only"},
                "provenance": PROV}));
        }
    }
    for c in m["components"].as_array().unwrap() {
        let mut geometry = json!({"rigid_pipe_ref": c["maps_member"], "weight": q(number(&c["weight_N"]), "N")});
        if !c["center_of_gravity_m"].is_null() {
            let r = &c["center_of_gravity_m"];
            geometry["center_of_gravity"] = json!({"x": r[0], "y": r[1], "z": r[2], "unit": "m"});
        }
        let mut component = json!({"id": c["id"], "kind": c["kind"], "node": c["node"], "geometry": geometry, "provenance": PROV});
        if !c["solver_consumption"].is_null() {
            component["mechanics_interface"] = json!({"solver_consumption": c["solver_consumption"]});
        }
        if !c["stiffness_scaling"].is_null() {
            component["modifiers"] = json!({"stiffness_scaling_user_value": q(number(&c["stiffness_scaling"]), "1"), "source_reference": c["stiffness_source_reference"]});
        }
        components.push(component);
    }
    let mut supported = std::collections::HashSet::new();
    let supports: Vec<Value> = m["supports"].as_array().unwrap().iter().enumerate().map(|(i, s)| {
        supported.insert(s["node"].as_str().unwrap().to_string());
        let restraints: Vec<String> = s["restrained"].as_array().unwrap().iter().map(|r| r.as_str().unwrap().to_uppercase()).collect();
        json!({"id": format!("support:{i}"), "node": s["node"], "family": "anchor", "restraints": restraints, "provenance": PROV})
    }).collect();
    let free = m["nodes"].as_array().unwrap().iter().map(|n| n["id"].as_str().unwrap()).find(|n| !supported.contains(*n)).or_else(|| m["nodes"].as_array().unwrap().last().and_then(|n| n["id"].as_str())).unwrap();
    let mut load_case = json!({"id": "case:f", "primitive_loads": [{"id": "load:f", "category": "concentrated_force", "target": {"type": "node", "node": free},
        "direction": "global_y", "magnitude": q(1000.0, "N"), "dimension": "force", "provenance": PROV}], "provenance": PROV});
    let materials = if exact {
        load_case["pressure_regions"] = json!([]);
        json!([{"id": material.0, "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "elastic_modulus": q(e, "Pa"), "poisson_ratio": q(e / (2.0 * g) - 1.0, "1"), "provenance": PROV}])
    } else {
        json!([{"id": material.0, "elastic_modulus": q(e, "Pa"), "shear_modulus": q(g, "Pa"), "provenance": PROV}])
    };
    let mut model = json!({"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": format!("project:{}", case["id"].as_str().unwrap()), "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": components, "materials": materials,
        "load_cases": [load_case], "combinations": []});
    if exact {
        model["pressure_contract"] = current_model::current_contract();
    }
    let mut request = json!({"model": model, "materials": [], "mode_count": 1});
    if let Some(a) = m["weight_to_mass_acceleration_m_per_s2"].as_f64() {
        request["weight_to_mass_acceleration"] = q(a, "m/s^2");
    }
    Some(request)
}

/// The product mass against one frozen basis: (normwise relative error of
/// M, relative error of each T_k).
fn compare_with_reference(case: &Value, basis: &str, mass: &ModalMass, model: &PreviewModel) -> (f64, f64) {
    let reference = &case["bases"][basis];
    let node_index = |id: &str| model.nodes.iter().position(|n| n.id == id).unwrap();
    let dofs: Vec<usize> = reference["assembled_dof_order"].as_array().unwrap().iter().map(|pair| {
        let dof = ["ux", "uy", "uz", "rx", "ry", "rz"].iter().position(|d| *d == pair[1]).unwrap();
        6 * node_index(pair[0].as_str().unwrap()) + dof
    }).collect();
    let n = dofs.len();
    let mut expected = vec![0.0; n * n];
    for entry in reference["assembled_full_mass_upper_nonzero"].as_array().unwrap() {
        let (r, c) = (entry[0].as_u64().unwrap() as usize, entry[1].as_u64().unwrap() as usize);
        // [row, col, exact, decimal], or [row, col, decimal] where no exact form exists.
        let value: f64 = entry.as_array().unwrap().last().unwrap().as_str().unwrap().parse().unwrap();
        expected[r * n + c] = value;
        expected[c * n + r] = value;
    }
    let scale = expected.iter().fold(0.0f64, |m, v| m.max(v.abs()));
    let mut error = 0.0f64;
    for r in 0..n {
        for c in 0..n {
            error = error.max((mass.mass_entry(dofs[r], dofs[c]) - expected[r * n + c]).abs());
        }
    }
    let mut total_error = 0.0f64;
    for (k, axis) in ["X", "Y", "Z"].iter().enumerate() {
        let t: f64 = reference["per_direction_kg"][axis]["full_model"]["decimal"].as_str().unwrap().parse().unwrap();
        total_error = total_error.max((mass.summary.total_by_direction[k] - t).abs() / t);
    }
    (error / scale, total_error)
}

/// One row of the D-TOL table.
struct Row {
    model: String,
    mass: Option<(f64, f64)>,
    nodal: (f64, f64),
    resultant: (f64, f64),
}

fn row(model: &str, mass: Option<(f64, f64)>, report: &IdentityReport) -> Row {
    Row { model: model.to_string(), mass, nodal: report.largest_discrepancy("nodal"), resultant: report.largest_discrepancy("resultant") }
}

/// The reference cases this converter authors, on each basis the product
/// admits for them; every refusal is reported with its codes.
fn reference_rows(rows: &mut Vec<Row>, refused: &mut Vec<String>) {
    for case in references()["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        for basis in ["PF", "EX"] {
            let Some(request) = reference_request(case, basis) else {
                refused.push(format!("{id}/{basis}: not authored by this converter (connector, tie rod)"));
                continue;
            };
            let request = modal(request);
            let prepared = match prepare_modal(&request) {
                Ok(prepared) => prepared,
                Err(found) => {
                    refused.push(format!("{id}/{basis}: prepare refused {:?}", codes(&found)));
                    continue;
                }
            };
            let mass = assemble_mass(&prepared, prepared.inputs.weight_to_mass_acceleration).unwrap_or_else(|f| panic!("{id}/{basis}: {f:#?}"));
            // The ineligible components keep their pipe mass, with self-weight's warning.
            for status in case["component_status"].as_array().unwrap() {
                if let Some(code) = status["expected_warning"].as_str() {
                    assert!(mass.warnings.iter().any(|w| w.code == code), "{id}/{basis}: {:?}", mass.warnings);
                }
            }
            let compared = compare_with_reference(case, basis, &mass, &prepared.model);
            let report = check_weight_basis(&prepared, &mass, TEST_LOCAL_IDENTITY_BOUND).unwrap_or_else(|f| panic!("{id}/{basis}: {f:#?}"));
            rows.push(row(&format!("{id}/{basis}"), Some(compared), &report));
        }
    }
}

#[test]
fn mass_agrees_with_the_frozen_mass_references() {
    let (mut rows, mut refused) = (Vec::new(), Vec::new());
    reference_rows(&mut rows, &mut refused);
    assert!(rows.iter().any(|r| r.model.ends_with("/PF")) && rows.iter().any(|r| r.model.ends_with("/EX")), "{refused:#?}");
    for row in &rows {
        let (matrix, totals) = row.mass.unwrap();
        assert!(matrix <= TEST_LOCAL_MASS_BOUND && totals <= TEST_LOCAL_MASS_BOUND, "{}: {matrix:e} {totals:e}", row.model);
    }
    eprintln!("not compared: {refused:#?}");
}

#[test]
fn d_tol_measurement_table() {
    let (mut rows, mut refused) = (Vec::new(), Vec::new());
    reference_rows(&mut rows, &mut refused);
    for (name, request) in [
        ("pipe-only PF", with(pf_request(false), json!({"mode_count": 1}))),
        ("component PF", with(pf_request(true), json!({"mode_count": 1, "weight_to_mass_acceleration": q(9.8125, "m/s^2")}))),
        ("arc PF", with(arc_request(false), json!({"mode_count": 1}))),
        ("EX n05", with(ex_request_with_density(), json!({"mode_count": 1}))),
        ("connector EX", with(connector_request(), json!({"mode_count": 1}))),
    ] {
        let (_, report) = identity(&modal(request));
        rows.push(row(name, None, &report));
    }
    println!("D-TOL measurement (experimental, unpinned; test-local bound {TEST_LOCAL_IDENTITY_BOUND:?})");
    println!("| model | mass rel (M) | mass rel (T_k) | nodal abs | nodal rel | resultant abs | resultant rel |");
    println!("|---|---|---|---|---|---|---|");
    let mut largest = [0.0f64; 4];
    for r in &rows {
        let (m, t) = r.mass.map_or(("-".to_string(), "-".to_string()), |(m, t)| (format!("{m:.2e}"), format!("{t:.2e}")));
        println!("| {} | {m} | {t} | {:.2e} | {:.2e} | {:.2e} | {:.2e} |", r.model, r.nodal.0, r.nodal.1, r.resultant.0, r.resultant.1);
        largest = [largest[0].max(r.nodal.0), largest[1].max(r.nodal.1), largest[2].max(r.resultant.0), largest[3].max(r.resultant.1)];
    }
    println!("| **largest** | | | {:.2e} | {:.2e} | {:.2e} | {:.2e} |", largest[0], largest[1], largest[2], largest[3]);
    println!("not compared: {refused:?}");
    assert!(largest[1] <= TEST_LOCAL_IDENTITY_BOUND.relative && largest[3] <= TEST_LOCAL_IDENTITY_BOUND.relative);
}

// ---------------------------------------------------------------------------
// The weight-basis identity
// ---------------------------------------------------------------------------

pub(super) fn ex_request_with_density() -> Value {
    let mut request: Value = serde_json::from_str(EX_REQUEST).unwrap();
    for pipe in request["model"]["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["material_density"] = q(7850.0, "kg/m^3");
        pipe["section"]["contents_density"] = q(1000.0, "kg/m^3");
    }
    request
}

/// T4-I12's frozen connector demo (re-authored to the current document), with
/// densities on every pipe section.
pub(super) fn connector_request() -> Value {
    let references: Value = serde_json::from_str(U3_REFERENCES).unwrap();
    let mut request = current_model::reauthor_request(references["cases"]["U3-SYS-DEMO-CONNECTOR-001"]["inputs"]["document_v3_0.3.0"].clone());
    for pipe in request["model"]["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["material_density"] = q(7850.0, "kg/m^3");
    }
    request
}

#[test]
fn identity_holds_on_pf_and_ex_straights() {
    let (mass, report) = identity(&modal(with(pf_request(false), json!({"mode_count": 1}))));
    assert_eq!(report.acceleration, 1.0);
    assert_eq!(mass.members.len(), 2);
    assert!(report.discrepancies.iter().any(|d| d.kind == "nodal"));
    let prepared_ex = modal(with(ex_request_with_density(), json!({"mode_count": 1})));
    assert!(prepared(&prepared_ex).exact);
    let (_, report) = identity(&prepared_ex);
    assert!(report.discrepancies.iter().filter(|d| d.kind == "nodal").count() >= 3);
    // A corrupted formed mass is refused: m̄ scaled as if the density differed,
    // with the model and its self-weight basis unchanged.
    let request = modal(with(pf_request(false), json!({"mode_count": 1})));
    let prepared = prepared(&request);
    let mut corrupted = prepared.model.clone();
    corrupted.pipe_segments[0].section.material_density.as_mut().unwrap().value *= 1.0 + 1e-9;
    let scaled = super::prepare::tests_support::with_model(prepared, corrupted);
    let mass = assemble_mass(&scaled, None).unwrap();
    let found = check_weight_basis(&scaled, &mass, TEST_LOCAL_IDENTITY_BOUND).unwrap_err();
    assert!(codes(&found).iter().all(|c| *c == MODAL_MASS_WEIGHT_BASIS_MISMATCH), "{found:#?}");
    assert!(found.iter().any(|d| d.affected_refs.contains(&"pipe:p1".to_string())), "{found:#?}");
}

#[test]
fn identity_holds_on_an_arc_on_the_resultant() {
    let (mass, report) = identity(&modal(with(arc_request(false), json!({"mode_count": 1}))));
    assert!(mass.members.iter().any(|m| matches!(m.kind, super::mass::MemberKind::Arc { .. })));
    assert!(report.discrepancies.iter().any(|d| d.subject == "pipe:b-c" && d.kind == "resultant"));
}

#[test]
fn identity_holds_on_a_component_body() {
    let request = modal(with(pf_request(true), json!({"mode_count": 1, "weight_to_mass_acceleration": q(9.8125, "m/s^2")})));
    let (mass, report) = identity(&request);
    assert_eq!(report.acceleration, 9.8125);
    assert_eq!(mass.bodies.len(), 1);
    assert_eq!(mass.bodies[0].mass, 500.0 / 9.8125);
    // The body replaces the member's distributed mass.
    assert!(mass.members.iter().all(|m| m.pipe != "body:v"));
    assert!(report.discrepancies.iter().any(|d| d.subject == "component:valve"));
    // The centre of gravity and the total: pipe p1 (m̄·1 at x = 0.5) and the body at c + r.
    let summary = &mass.summary;
    let pipe = mass.members[0].mass_per_length;
    let body = 500.0 / 9.8125;
    let total = pipe + body;
    assert!((summary.total_by_direction[0] - total).abs() <= 1e-12 * total);
    let cog = summary.center_of_gravity.unwrap();
    assert!((cog[0] - (pipe * 0.5 + body * 1.5) / total).abs() <= 1e-12);
    assert!((cog[1] - body * 0.25 / total).abs() <= 1e-12);
    // Without a, the body cannot enter.
    let found = assemble_mass(&prepared(&modal(with(pf_request(true), json!({"mode_count": 1})))), None).unwrap_err();
    assert_eq!(codes(&found), [MODAL_WEIGHT_ACCELERATION_REQUIRED]);
    assert!(found[0].message.contains("component:valve"));
    // An ineligible component keeps the pipe mass, with self-weight's warning.
    let mut ineligible = pf_request(true);
    ineligible["model"]["components"][0]["mechanics_interface"]["solver_consumption"] = json!("not_a_mode");
    let (mass, _) = identity(&modal(with(ineligible, json!({"mode_count": 1}))));
    assert!(mass.bodies.is_empty() && mass.members.len() == 2);
}

#[test]
fn a_distant_model_does_not_hide_a_component_moment_error() {
    // Review finding 1 (I2a): a component is compared about its own node, with
    // its own size, so a model extent of 1e13 m cannot inflate the moment scale.
    let mut far = pf_request(true);
    for (node, x) in far["model"]["nodes"].as_array_mut().unwrap().iter_mut().zip([0.0, 1e13, 1e13 + 1.0]) {
        node["position"]["x"] = json!(x);
    }
    let request = modal(with(far, json!({"mode_count": 1, "weight_to_mass_acceleration": q(10.0, "m/s^2")})));
    let prepared = prepared(&request);
    let mut mass = assemble_mass(&prepared, prepared.inputs.weight_to_mass_acceleration).unwrap();
    check_weight_basis(&prepared, &mass, TEST_LOCAL_IDENTITY_BOUND).unwrap_or_else(|f| panic!("the intact far model refused: {f:#?}"));
    // Only the mass side's offset changes: (-0.5, 0.25, 0) to (-0.5, 0, 0), a 125 N·m moment error.
    let body = &mut mass.bodies[0];
    body.block = open_pipe_stress_modal_eigen::reformation::mass::component_mass(&open_pipe_stress_modal_eigen::reformation::mass::ComponentBody {
        node: body.node,
        weight: body.weight,
        acceleration: 10.0,
        center_of_gravity: [-0.5, 0.0, 0.0],
    })
    .unwrap();
    let found = check_weight_basis(&prepared, &mass, TEST_LOCAL_IDENTITY_BOUND).unwrap_err();
    assert!(codes(&found).iter().all(|c| *c == MODAL_MASS_WEIGHT_BASIS_MISMATCH), "{found:#?}");
    assert!(found.iter().any(|d| d.message.contains("component:valve") && d.message.contains("its own node centroid")), "{found:#?}");
}

#[test]
fn a_replaced_span_carries_no_mass_and_no_weight() {
    let request = modal(with(connector_request(), json!({"mode_count": 1})));
    let prepared = prepared(&request);
    let replaced = crate::joint::replaced_span_ids(&prepared.model);
    assert_eq!(replaced.len(), 1);
    let span = replaced.iter().next().unwrap();
    assert!(!mass_bearing_members(&prepared.model).contains(span));
    let mass = assemble_mass(&prepared, None).unwrap();
    assert!(mass.members.iter().all(|m| &m.pipe != span));
    let report = check_weight_basis(&prepared, &mass, TEST_LOCAL_IDENTITY_BOUND).unwrap();
    assert!(report.discrepancies.iter().all(|d| &d.subject != span));
}

/// The valve model with an applied self-weight case generated at g along −Z.
fn applied_request(g: f64) -> Value {
    let mut request = pf_request(true);
    let model: PreviewModel = serde_json::from_value(request["model"].clone()).unwrap();
    let plan = generate_self_weight_operations(&model, &SelfWeightRequest {
        case_id: "case:w".into(),
        label: "invented self-weight".into(),
        pipe_refs: vec!["pipe:p1".into(), "body:v".into()],
        gravity: SelfWeightGravity { value: g, unit: "m/s^2".into(), axis: "global_z".into() },
        provenance: PROV.into(),
        source_model_hash: HASH.into(),
    })
    .unwrap();
    let loads: Vec<Value> = plan.changes[1..].iter().map(|c| serde_json::from_str(&c.after).unwrap()).collect();
    request["model"]["load_cases"].as_array_mut().unwrap().push(json!({"id": "case:w", "primitive_loads": loads, "provenance": PROV}));
    request
}

#[test]
fn acceleration_agreement_with_the_applied_weight_case() {
    let request = modal(with(applied_request(-9.8125), json!({"mode_count": 1, "weight_to_mass_acceleration": q(9.8125, "m/s^2")})));
    let (_, report) = identity(&request);
    // The full-scope applied case's non-blocking row.
    assert_eq!(report.applied_rows.len(), 1);
    let applied = &report.applied_rows[0];
    assert_eq!((applied.case_id.as_str(), applied.axis), ("case:w", 2));
    assert!(applied.force_relative <= TEST_LOCAL_IDENTITY_BOUND.relative && applied.moment_relative <= TEST_LOCAL_IDENTITY_BOUND.relative, "{applied:?}");
    // A different a is refused, naming the case and both values.
    let request = modal(with(applied_request(-9.8125), json!({"mode_count": 1, "weight_to_mass_acceleration": q(9.8, "m/s^2")})));
    let prepared = prepared(&request);
    let mass = assemble_mass(&prepared, prepared.inputs.weight_to_mass_acceleration).unwrap();
    let found = check_weight_basis(&prepared, &mass, TEST_LOCAL_IDENTITY_BOUND).unwrap_err();
    assert!(codes(&found).iter().all(|c| *c == MODAL_WEIGHT_ACCELERATION_MISMATCH), "{found:#?}");
    assert!(found[0].message.contains("case:w") && found[0].message.contains("9.8125e0") && found[0].message.contains("9.8e0"), "{}", found[0].message);
    // A partial-scope applied case is disclosed, not compared.
    let mut partial = applied_request(-9.8125);
    let loads = partial["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap();
    loads.retain(|l| l["target"]["type"] != "element");
    let (_, report) = identity(&modal(with(partial, json!({"mode_count": 1, "weight_to_mass_acceleration": q(9.8125, "m/s^2")}))));
    assert!(report.applied_rows.is_empty());
    assert!(report.disclosures.iter().any(|d| d.contains("partial-scope")), "{:?}", report.disclosures);
}

#[test]
fn a_manual_override_is_disclosed_and_takes_no_part() {
    // One component primitive preserved as a manual override (W-15's form).
    let mut request = applied_request(-9.8125);
    let loads = request["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap();
    let at = loads.iter().position(|l| l["target"]["type"] == "node").unwrap();
    let original: Value = serde_json::from_str(loads[at]["provenance"].as_str().unwrap()).unwrap();
    loads[at]["magnitude"]["value"] = json!(-123.0);
    loads[at]["provenance"] = json!(json!({"method": crate::self_weight::MANUAL_OVERRIDE_METHOD, "original_generation_provenance": original,
        "decision": "preserve_modified_generated_load", "source_model_hash": HASH}).to_string());
    let (_, report) = identity(&modal(with(request, json!({"mode_count": 1, "weight_to_mass_acceleration": q(9.8125, "m/s^2")}))));
    assert!(report.disclosures.iter().any(|d| d.contains("ManualOverride")), "{:?}", report.disclosures);
    // The case is not current as a whole, so it gives no applied row.
    assert!(report.applied_rows.is_empty());
}

#[test]
fn range_premise_witnesses_refuse_by_operand() {
    for witness in references()["range_premise_witnesses"].as_array().unwrap() {
        let mut request = pf_request(true);
        let component = &mut request["model"]["components"][0]["geometry"];
        component["weight"] = q(number(&witness["W_N"]), "N");
        if let Some(r) = witness.get("center_of_gravity_m") {
            component["center_of_gravity"] = json!({"x": r[0], "y": r[1], "z": r[2], "unit": "m"});
        } else {
            component["center_of_gravity"] = json!({"x": 0, "y": 0, "z": 0, "unit": "m"});
        }
        let a = number(&witness["a_m_per_s2"]);
        let request = modal(with(request, json!({"mode_count": 1, "weight_to_mass_acceleration": q(a, "m/s^2")})));
        let prepared = match prepare_modal(&request) {
            Ok(prepared) => prepared,
            Err(found) => panic!("{witness}: prepare refused {found:#?}"),
        };
        let found = assemble_mass(&prepared, prepared.inputs.weight_to_mass_acceleration).unwrap_err();
        assert_eq!(codes(&found), [MODAL_COMPONENT_MASS_OUT_OF_RANGE], "{witness}");
        let operand = witness["fails"].as_str().unwrap();
        assert!(found[0].affected_refs == ["component:valve".to_string(), operand.to_string()], "{witness}: {:?}", found[0].affected_refs);
    }
}

// ---------------------------------------------------------------------------
// The self-weight inspection seam (D-SW review conditions)
// ---------------------------------------------------------------------------

fn gravity(axis: &str) -> SelfWeightGravity {
    SelfWeightGravity { value: 1.0, unit: "m/s^2".into(), axis: axis.into() }
}

#[test]
fn inspection_is_pure_and_its_scratch_id_is_unique() {
    let mut model: PreviewModel = serde_json::from_value(pf_model(true)).unwrap();
    let before = format!("{model:?}");
    let members = vec!["pipe:p1".to_string(), "body:v".to_string()];
    let plan = inspect_generated_self_weight(&model, &members, &gravity("global_x")).unwrap();
    assert_eq!(format!("{model:?}"), before, "the inspection reads the model only");
    assert_eq!(plan.changes[0].target_ref, "modal:weight-basis");
    // Taken by a case, then by a primitive identity in the generated namespace.
    model.load_cases[0].id = "modal:weight-basis".into();
    model.load_cases[0].primitive_loads[0].id = "load:self-weight:19:modal:weight-basis~:7:pipe:p1".into();
    let plan = inspect_generated_self_weight(&model, &members, &gravity("global_x")).unwrap();
    assert_eq!(plan.changes[0].target_ref, "modal:weight-basis~~");
    assert!(model.load_cases.iter().all(|c| c.id != plan.changes[0].target_ref));
    // Review finding 2 (I2a): a case id in the generated namespace is taken too.
    let mut model: PreviewModel = serde_json::from_value(pf_model(false)).unwrap();
    model.load_cases[0].id = format!("load:self-weight:{}:modal:weight-basis:7:pipe:p1", "modal:weight-basis".len());
    let plan = inspect_generated_self_weight(&model, &["pipe:p1".to_string()], &gravity("global_x"))
        .unwrap_or_else(|found| panic!("a valid model refused by the scratch identity: {found:#?}"));
    assert_eq!(plan.changes[0].target_ref, "modal:weight-basis~");
    // End to end: the valid model prepares, forms its mass and passes the identity.
    let mut renamed = pf_request(false);
    renamed["model"]["load_cases"][0]["id"] = json!(model.load_cases[0].id);
    identity(&modal(with(renamed, json!({"mode_count": 1}))));
}

#[test]
fn inspection_is_unreachable_before_preparation_passes() {
    // The one call site is the weight-basis check, which takes a `Prepared`;
    // a `Prepared` has no constructor outside `prepare_modal`.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut sites = Vec::new();
    let mut stack = vec![root.clone()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") && !path.ends_with("modal/tests.rs") {
                let text = std::fs::read_to_string(&path).unwrap();
                for (n, line) in text.lines().enumerate() {
                    if line.contains("inspect_generated_self_weight(") && !line.contains("fn inspect_generated_self_weight(") {
                        sites.push(format!("{}:{}", path.strip_prefix(&root).unwrap().display(), n + 1));
                    }
                }
            }
        }
    }
    assert_eq!(sites.len(), 1, "{sites:?}");
    assert!(sites[0].starts_with("modal/weight_basis.rs"), "{sites:?}");
    let prepare_text = include_str!("prepare.rs");
    assert_eq!(prepare_text.matches("Admitted(())").count(), 2, "constructed only in `prepare_modal`");
    assert!(include_str!("weight_basis.rs").contains("pub(crate) fn check_weight_basis(prepared: &Prepared"));
    // A request that fails validation never yields a `Prepared`.
    let mut invalid = pf_request(false);
    invalid["model"]["pipe_segments"][0]["material"] = json!("missing");
    assert!(prepare_modal(&modal(with(invalid, json!({"mode_count": 1})))).is_err());
}

#[test]
fn no_scratch_identity_or_empty_hash_reaches_a_modal_output() {
    let (mass, report) = identity(&modal(with(applied_request(-9.8125), json!({"mode_count": 1, "weight_to_mass_acceleration": q(9.8125, "m/s^2")}))));
    let text = format!("{mass:?}{report:?}");
    assert!(!text.contains("modal:weight-basis") && !text.contains("source_model_hash"));
}
