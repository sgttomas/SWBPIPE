//! T4-U2a: the exact route's admission seam and the `3.0.0/exact_pressure_v3`
//! identity, through PP's public entries.
//!
//! Inputs are the committed invented fixtures patched inline: X0 =
//! `fixtures/exact_pressure_connected_request.json` (without the
//! load/reference-state group), Y0 =
//! `fixtures/product_preview/pressure/load_reference/pressure.request.json`
//! (with the group). B7b deleted the v2 contract: both are current model
//! 0.9.0 documents, re-authored once from their v2 originals to
//! `3.0.0/exact_pressure_v3` with `shear_model` Euler-Bernoulli and an
//! explicit zero corrosion allowance. "Both entries" are the captured value
//! entry and the typed entry; "both modes" are sparse-interactive and
//! dense-scrutiny.

use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    PreviewSolverMode,
};
use open_pipe_stress_result_export::semantic_contract;
use serde_json::{json, Value};

#[path = "common/current_model.rs"]
mod current_model;

const X0: &str = include_str!("fixtures/exact_pressure_connected_request.json");
const Y0: &str =
    include_str!("../../../fixtures/product_preview/pressure/load_reference/pressure.request.json");
const CORPUS: &str =
    include_str!("../../../fixtures/results/pressure_v3_straight_reader_corpus.json");
const CORPUS_PATH: &str = "../../fixtures/results/pressure_v3_straight_reader_corpus.json";
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
const SOURCE: &str = "core/product_physics/src/pressure_runtime.rs";
const FAMILY: &str = "EXACT_PRESSURE_FAMILY_NOT_ADMITTED";
const COMPOSITION: &str = "EXACT_PRESSURE_COMPOSITION_UNSUPPORTED";
const COMBINATION: &str = "EXACT_PRESSURE_COMBINATION_UNSUPPORTED";
const NEGATIVE: &str = "PRESSURE_REGION_PRESSURE_NEGATIVE";
const PRESSURE_ID: &str = "openpipestress.result_semantics/0.3.0/pressure-2";
const PROVENANCE: &str = "invented_u2a_control";

#[derive(Clone, Copy, Debug)]
enum Base {
    X0,
    Y0,
}

impl Base {
    /// The committed current document (0.9.0, v3, CA 0, Euler-Bernoulli).
    fn current(self) -> Value {
        let document: Value = serde_json::from_str(match self {
            Base::X0 => X0,
            Base::Y0 => Y0,
        })
        .unwrap();
        current_model::assert_current_exact_request(&document, &format!("{self:?}"));
        document
    }
    fn tip(self) -> &'static str {
        match self {
            Base::X0 => "node:fixture-tip",
            Base::Y0 => "node:middle",
        }
    }
    /// The pipe every committed region of the fixture contains.
    fn region_pipe(self) -> &'static str {
        match self {
            Base::X0 => "pipe:fixture-span",
            Base::Y0 => "pipe:first",
        }
    }
    fn cases(self) -> [&'static str; 2] {
        match self {
            Base::X0 => ["case:closed-pressure", "case:six-component-load"],
            Base::Y0 => ["case:cold-pressure", "case:hot-pressure"],
        }
    }
    fn pressurized_case(self) -> &'static str {
        match self {
            Base::X0 => "case:closed-pressure",
            Base::Y0 => "case:cold-pressure",
        }
    }
    fn negative_pressure(self) -> Value {
        match self {
            Base::X0 => json!({"value":-2000,"unit":"kPa"}),
            Base::Y0 => json!({"value":-2,"unit":"MPa"}),
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

/// Cases of a document with the load/reference-state group declare every
/// support's participation.
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

fn bend(base: Base, realized: bool) -> Value {
    let consumption = if realized { "curved_bend_macro_element" } else { "mechanics_geometry_only" };
    json!({"id":"component:bend","kind":"bend","node":base.tip(),
        "geometry":{"bend_pipe_ref":base.region_pipe(),"bend_radius":{"value":1.0,"unit":"m"},
            "bend_plane_orientation":"invented","bend_geometry_source_reference":"invented"},
        "modifiers":{"flexibility_factor_user_value":{"value":1.0,"unit":"none"},"source_reference":"invented"},
        "mechanics_interface":{"solver_consumption":consumption},
        "provenance":PROVENANCE})
}

fn simple(base: Base, id: &str, kind: &str) -> Value {
    json!({"id":id,"kind":kind,"node":base.tip(),"provenance":PROVENANCE})
}

fn annotation_joint(base: Base) -> Value {
    json!({"id":"component:joint-annotation","kind":"expansion_joint","node":base.tip(),
        "mechanics_interface":{"solver_consumption":"not_solver_consumed"},"provenance":PROVENANCE})
}

fn legacy_joint(base: Base) -> Value {
    json!({"id":"component:joint-legacy","kind":"expansion_joint","node":base.tip(),
        "geometry":{"expansion_joint_pipe_ref":base.region_pipe()},
        "modifiers":{"axial_stiffness_user_value":{"value":1000.0,"unit":"N/mm"}},
        "provenance":PROVENANCE})
}

fn connector(base: Base) -> Value {
    json!({"id":"component:connector","kind":"expansion_joint","node":base.tip(),
        "objective_connector":{"version":"1.0.0"},"provenance":PROVENANCE})
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

/// Both entries in both modes. All four share a status; blocked outputs share
/// their diagnostics. Returns [sparse captured, sparse typed, dense captured,
/// dense typed].
fn run_all(document: &Value) -> Vec<Value> {
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
            assert_eq!(output["diagnostics"], outputs[0]["diagnostics"], "entries or modes disagree");
        }
    }
    outputs
}

fn diagnostics(envelope: &Value) -> &Vec<Value> {
    envelope["diagnostics"].as_array().unwrap()
}

fn has_code(envelope: &Value, code: &str) -> bool {
    diagnostics(envelope).iter().any(|d| d["code"] == code)
}

/// The one seam finding with `code` and exactly `refs`.
fn finding<'a>(envelope: &'a Value, code: &str, refs: &[&str]) -> &'a Value {
    let found = diagnostics(envelope)
        .iter()
        .filter(|d| d["code"] == code && d["affected_refs"] == json!(refs))
        .collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{code} {refs:?} in {:#?}", envelope["diagnostics"]);
    let finding = found[0];
    assert_eq!(finding["severity"], "blocking");
    assert_eq!(finding["source"], SOURCE);
    assert_eq!(
        finding["id"],
        format!("diagnostic:pressure-runtime:{}:{code}", refs.join(":").replace(':', "-"))
    );
    finding
}

fn assert_v3_blocked(envelope: &Value, base: Base) {
    assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE");
    assert_eq!(envelope["results"], json!([]));
    assert_eq!(envelope["contract_evidence"], base.empty_evidence());
    assert_eq!(envelope["producer"]["semantic_contract_id"], PRESSURE_ID);
    assert_eq!(envelope["formulation_basis"]["profile_id"], "exact_pressure_v3");
    assert!(!has_code(envelope, COMPOSITION) && !has_code(envelope, COMBINATION));
}

fn assert_v3_identity(envelope: &Value) {
    assert_eq!(envelope["producer"]["semantic_contract_id"], PRESSURE_ID);
    assert_eq!(envelope["formulation_basis"]["profile_id"], "exact_pressure_v3");
    let limitations = envelope["formulation_basis"]["limitations"].to_string();
    for clause in [
        "3.0.0/exact_pressure_v3 under pressure-2 semantics",
        "external-pressure stability and collapse are not assessed",
        "steady-flow momentum and transient pressure loads",
        "Bourdon",
        "pressure stiffening of flexibility factors and stress intensification",
        "ovalization",
        "EXACT_PRESSURE_FAMILY_NOT_ADMITTED",
    ] {
        assert!(limitations.contains(clause), "{clause}: {limitations}");
    }
    let evidence = &envelope["contract_evidence"];
    for case in evidence["exact_cases"].as_array().unwrap() {
        assert_eq!(case["profile_mode"], "exact_pressure_v3");
    }
    assert!(!evidence["pressure"].as_array().unwrap().is_empty());
    for region in evidence["pressure"].as_array().unwrap() {
        assert_eq!(region["profile_version"], "3.0.0");
        assert_eq!(region["profile_mode"], "exact_pressure_v3");
    }
}

/// The current pressure-2 raw reader admits the envelope and its header
/// dispatches to the resident pressure table.
fn assert_readers_dispatch_v3(envelope: &Value) {
    semantic_contract::validate_pressure_evidence(envelope).expect("pressure-2 reader");
    let (table, _) = semantic_contract::for_source(envelope).expect("pressure-2 dispatch");
    assert!(std::ptr::eq(table, semantic_contract::pressure_rows_contract()));
}

/// The retired v2 identity the header of each base's result named before B7b.
fn retired_identity(base: Base) -> &'static str {
    match base {
        Base::X0 => "openpipestress.result_semantics/0.3.0/physics-1",
        Base::Y0 => "openpipestress.result_semantics/0.3.0/load-reference-1",
    }
}

#[test]
fn v3_straight_documents_publish_under_pressure_1_and_dispatch_in_the_reader() {
    for base in [Base::X0, Base::Y0] {
        for envelope in run_all(&base.current()) {
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{base:?}");
            assert!(!has_code(&envelope, FAMILY));
            assert!(envelope.get("source_block_recovery").is_none());
            assert_v3_identity(&envelope);
            assert_readers_dispatch_v3(&envelope);
        }
        // B7b: a current envelope relabelled with its retired v2 identity is
        // refused by name at header dispatch, never read.
        let mut relabelled = run_all(&base.current()).swap_remove(0);
        relabelled["producer"]["semantic_contract_id"] = json!(retired_identity(base));
        let refusal = semantic_contract::for_source(&relabelled).unwrap_err();
        assert!(
            refusal.starts_with(semantic_contract::SOURCE_EXACT_PRESSURE_V2_RETIRED),
            "{refusal}"
        );
    }
}

/// The source controls of both families (without and with the group).
const PHYSICS_SOURCE_N05: &str = include_str!(
    "../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json"
);
const LOAD_REFERENCE_SOURCE_N05: &str = include_str!(
    "../../../fixtures/product_preview/pressure_source/load_reference_source/n05.request.json"
);
const PHYSICS_SOURCE_N06: &str = include_str!(
    "../../../fixtures/product_preview/pressure_source/physics_source/n06.request.json"
);
const LOAD_REFERENCE_SOURCE_N06: &str = include_str!(
    "../../../fixtures/product_preview/pressure_source/load_reference_source/n06.request.json"
);
/// The retired ruling-3 diagnostic (T4-I31). The join deleted its producer; no
/// envelope may carry it.
const NOT_JOINED: &str = "EXACT_PRESSURE_V3_RETAINED_SOURCE_NOT_JOINED";

/// The cases a captured run routed to retained-source recovery: those it
/// selected (in the receipt) or attempted without a selection.
fn recovery_cases(envelope: &Value) -> Vec<String> {
    let mut cases: Vec<String> = envelope["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| {
            d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"
                || d["code"] == "SOURCE_BLOCK_RECOVERY_SELECTED"
        })
        .map(|d| d["affected_refs"][0].as_str().unwrap().to_string())
        .collect();
    cases.dedup();
    assert!(!cases.is_empty(), "the fixture must exercise retained-source recovery");
    cases
}

fn selected_cases(envelope: &Value) -> Vec<String> {
    envelope["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_SELECTED")
        .map(|d| d["affected_refs"][0].as_str().unwrap().to_string())
        .collect()
}

/// A current source control whose captured run attempts retained-source
/// recovery, in one mode. The captured run publishes `pressure-source-1` with
/// a valid receipt, at least one selected case and no missing-join
/// diagnostic. The typed entry attempts no recovery. Returns the captured
/// envelope. (Before B7b this compared each v3 twin with its v2 original.)
fn assert_joined(name: &str, document: &Value, mode: PreviewSolverMode) -> Value {
    use open_pipe_stress_result_export::physics_source as ps;
    let label = format!("{name} {mode:?}");
    current_model::assert_current_exact_request(document, &label);
    let captured = serde_json::to_value(
        run_linear_static_preview_value_with_mode(document.clone(), mode).unwrap(),
    )
    .unwrap();
    let typed = serde_json::to_value(run_linear_static_preview_with_mode(
        serde_json::from_value(document.clone()).unwrap(),
        mode,
    ))
    .unwrap();
    assert_eq!(captured["producer"]["semantic_contract_id"], ps::PRESSURE_CONTRACT_ID, "{label}");
    assert_eq!(captured["formulation_basis"]["profile_id"], "exact_pressure_v3", "{label}");
    assert_eq!(captured["source_block_recovery"]["body"]["policy"], "PRESSURE-SOURCE-1", "{label}");
    assert_eq!(captured["source_block_recovery"]["body"]["status"], "qualified", "{label}");
    assert!(!has_code(&captured, NOT_JOINED), "{label}");
    assert!(!recovery_cases(&captured).is_empty(), "{label}");
    assert!(!selected_cases(&captured).is_empty(), "{label}");
    let invocation = json!({"request":document,"solver_mode":mode.as_str()});
    let eligible = captured["contract_evidence"].get("load_reference_states").is_none();
    assert_eq!(ps::validate_in(&captured, Some(&invocation), ps::SourceContract::PressureV3), Ok(eligible), "{label}");
    assert_eq!(ps::validate_transport_metadata_in(&captured, ps::SourceContract::PressureV3), Ok(()), "{label}");
    assert!(!has_code(&typed, NOT_JOINED), "{label}");
    assert!(typed.get("source_block_recovery").is_none(), "{label}");
    assert_ne!(typed["producer"]["semantic_contract_id"], ps::PRESSURE_CONTRACT_ID, "{label}");
    captured
}

/// The join's flip of T4-RV13 S-1 (HELP_HUMAN's ruling 3, which lapsed by its
/// own condition when the join landed; b7-plan D5): a current document whose
/// captured run selects retained-source recovery publishes `pressure-source-1`
/// with a valid receipt and no missing-join diagnostic.
#[test]
fn v3_twins_of_retained_source_documents_publish_pressure_source_1() {
    for (name, text) in [
        ("physics_source/n05", PHYSICS_SOURCE_N05),
        ("load_reference_source/n05", LOAD_REFERENCE_SOURCE_N05),
    ] {
        let document: Value = serde_json::from_str(text).unwrap();
        for mode in MODES {
            let captured = assert_joined(name, &document, mode);
            assert_eq!(captured["status"]["mechanics"], "MECHANICS_SOLVED", "{name} {mode:?}");
        }
    }
}

/// The join's flip of ruling 3 (c): n06 (without and with the group) solves
/// only through retained-source recovery. It is rescued in both modes on the
/// captured entry: solved and published as `pressure-source-1`. The typed entry
/// attempts no retained-source recovery.
#[test]
fn v3_twins_that_v2_rescues_are_rescued_under_pressure_source_1() {
    for (name, text) in [
        ("physics_source/n06", PHYSICS_SOURCE_N06),
        ("load_reference_source/n06", LOAD_REFERENCE_SOURCE_N06),
    ] {
        let document: Value = serde_json::from_str(text).unwrap();
        for mode in MODES {
            let captured = assert_joined(name, &document, mode);
            assert_eq!(captured["status"]["mechanics"], "MECHANICS_SOLVED", "{name} {mode:?}");
            assert!(has_code(&captured, "SOURCE_BLOCK_RECOVERY_SELECTED"), "{name} {mode:?}");
        }
    }
}

/// Every family not yet admitted is refused by name on v3, without and with
/// the group, in both entries and both modes.
#[test]
fn each_family_not_yet_admitted_is_refused_by_name_on_v3() {
    for base in [Base::X0, Base::Y0] {
        let components = [
            // T4-U2 admits the realized bend (see the A3 test below).
            // W-15 admits valve, flange, reducer, rigid and specialty (see the
            // W-15 tests below); RT-A3 the tee (`rt_a3_tee_references.rs`).
            (bend(base, false), "geometry-only bend"),
            (simple(base, "component:other", "other"), "component"),
            (annotation_joint(base), "expansion joint (annotation only"),
        ];
        let mut cases = components
            .into_iter()
            .map(|(component, name)| {
                let id = component["id"].as_str().unwrap().to_string();
                let mut document = base.current();
                push(&mut document, "components", component);
                (document, vec![id], name)
            })
            .collect::<Vec<_>>();
        for (support, name) in [(gap(base), "nonlinear support"), (constant_effort(base), "constant-effort support")] {
            let id = support["id"].as_str().unwrap().to_string();
            let mut document = base.current();
            push(&mut document, "supports", support);
            declare_support_state(&mut document, &id);
            cases.push((document, vec![id], name));
        }
        // W-16 slice b admits load combinations (see the test below); W-16 slice c
        // admits equivalent-static generation (see the test after it).
        for (document, refs, name) in cases {
            let refs = refs.iter().map(String::as_str).collect::<Vec<_>>();
            for envelope in run_all(&document) {
                assert_v3_blocked(&envelope, base);
                let message = finding(&envelope, FAMILY, &refs)["message"].as_str().unwrap().to_string();
                assert!(message.contains(name), "{name}: {message}");
                if name == "geometry-only bend" {
                    // D-2: never admitted; the text asks for the realization.
                    assert!(message.contains("is not admitted under 3.0.0/exact_pressure_v3"), "{message}");
                    assert!(message.contains("realize this bend"), "{message}");
                } else {
                    assert!(message.contains("is not yet admitted under 3.0.0/exact_pressure_v3"), "{message}");
                }
            }
        }
    }
}

/// W-16 slice b (DEL-05-02 Design `exact-combinations.md`): the seam admits a
/// model-authored combination, so the family refusal never names it. On both
/// bases the document solves and states the combination's evidence record.
#[test]
fn a_load_combination_is_admitted_at_the_seam() {
    for base in [Base::X0, Base::Y0] {
        let [first, second] = base.cases();
        let mut document = base.current();
        document["model"]["combinations"] = json!([{"id":"combination:sum","basis":"mechanics",
            "terms":[{"load_case":first,"factor":1.0},{"load_case":second,"factor":1.0}],"provenance":PROVENANCE}]);
        for envelope in run_all(&document) {
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED");
            assert!(!has_code(&envelope, FAMILY), "{:#?}", envelope["diagnostics"]);
            let records = envelope["contract_evidence"]["exact_combinations"].as_array().unwrap();
            assert_eq!(records.len(), 1);
            assert_eq!(records[0]["combination_id"], "combination:sum");
        }
    }
}

/// W-16 slice c (DEL-05-01 Design `exact-equivalent-static.md`): the seam
/// admits a case's `equivalent_static`, so the family refusal never names it.
/// On the plain document a wind generation solves and publishes its record;
/// on the load/reference-state document the route refuses generation by name,
/// before the effective cases are selected.
#[test]
fn equivalent_static_is_admitted_at_the_seam_and_refused_on_load_state_documents() {
    const LOAD_STATE: &str = "EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED";
    for base in [Base::X0, Base::Y0] {
        let [_, second] = base.cases();
        let mut equivalent = base.current();
        case_mut(&mut equivalent, second)["equivalent_static"] = json!({"wind": {"pressure": {"value": 800.0, "unit": "Pa"},
            "shape_factor": {"value": 0.7, "unit": "1"}, "direction": "global_z", "exposed_pipe_refs": [base.region_pipe()]},
            "provenance": PROVENANCE});
        for envelope in run_all(&equivalent) {
            assert!(!has_code(&envelope, FAMILY), "{:#?}", envelope["diagnostics"]);
            match base {
                Base::X0 => {
                    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{:#?}", envelope["diagnostics"]);
                    assert!(!has_code(&envelope, LOAD_STATE));
                    let case = envelope["contract_evidence"]["exact_cases"].as_array().unwrap().iter()
                        .find(|c| c["load_case_id"] == second).unwrap();
                    assert_eq!(case["equivalent_static"]["generated_load_ids"].as_array().unwrap().len(), 1);
                }
                Base::Y0 => {
                    assert_v3_blocked(&envelope, base);
                    let message = finding(&envelope, LOAD_STATE, &[second, "equivalent_static"])["message"].as_str().unwrap().to_string();
                    assert!(message.contains("declared source ledger"), "{message}");
                }
            }
        }
    }
}

/// T4-U3 (D-4): a legacy joint takes `LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED`
/// (refs [component, pipe]) first and alone: the seam skips it, so the v3
/// family code never names it. An explicit annotation is exempt from the
/// legacy code even when it carries a pipe reference and takes the seam's
/// refusal. (B7b deleted the v2 composition side of this test with v2.)
#[test]
fn legacy_joints_take_the_legacy_code_and_annotations_the_route_refusal() {
    const LEGACY: &str = "LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED";
    for base in [Base::X0, Base::Y0] {
        let mut document = base.current();
        push(&mut document, "components", legacy_joint(base));
        for envelope in run_all(&document) {
            assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE");
            let blocking = diagnostics(&envelope)
                .iter()
                .filter(|d| d["severity"] == "blocking")
                .collect::<Vec<_>>();
            assert_eq!(blocking.len(), 1, "{blocking:#?}");
            assert_eq!(blocking[0]["code"], LEGACY);
            assert_eq!(blocking[0]["affected_refs"], json!(["component:joint-legacy", base.region_pipe()]));
        }
        let mut annotation_with_pipe = annotation_joint(base);
        annotation_with_pipe["geometry"] = json!({"expansion_joint_pipe_ref": base.region_pipe()});
        for annotation in [annotation_joint(base), annotation_with_pipe] {
            let mut v3 = base.current();
            push(&mut v3, "components", annotation);
            for envelope in run_all(&v3) {
                assert_v3_blocked(&envelope, base);
                let message = finding(&envelope, FAMILY, &["component:joint-annotation"])["message"].to_string();
                assert!(message.contains("annotation-only joints are analysed as pipe on the pressure-free route only"), "{message}");
                assert!(!has_code(&envelope, LEGACY) && !has_code(&envelope, "EXPANSION_JOINT_ANNOTATION_ONLY"));
            }
        }
    }
}

/// T4-U3: on v3 the objective connector is admitted at the seam and goes to
/// its own classifier: an incomplete record, or one with a contradictory
/// `solver_consumption` (N-6), is `OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE`,
/// never the seam's family refusal.
#[test]
fn v3_connectors_reach_their_classifier_not_the_family_refusal() {
    const INCOMPLETE: &str = "OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE";
    for base in [Base::X0, Base::Y0] {
        let mut annotated = connector(base);
        annotated["mechanics_interface"] = json!({"solver_consumption": "not_solver_consumed"});
        for component in [connector(base), annotated] {
            let mut v3 = base.current();
            push(&mut v3, "components", component);
            for envelope in run_all(&v3) {
                assert_v3_blocked(&envelope, base);
                assert!(!has_code(&envelope, FAMILY), "{:#?}", envelope["diagnostics"]);
                let found = diagnostics(&envelope)
                    .iter()
                    .filter(|d| d["code"] == INCOMPLETE)
                    .collect::<Vec<_>>();
                assert_eq!(found.len(), 1, "{:#?}", envelope["diagnostics"]);
                assert_eq!(found[0]["affected_refs"], json!(["component:connector", "objective_connector"]));
            }
        }
    }
}

/// T4-U0's A3 end to end, since T4-U2: a realized bend on the member of a v3
/// exact pressure region is admitted at the seam and recovered on the arc,
/// never on its chord: it publishes the pressure family's rows and withholds
/// the Lamé rows and the straight-statics maximum (no panic), in both entries
/// and both modes. T4-U2's references (`t4_u2_pressure_references.rs`) check
/// the values.
#[test]
fn a3_realized_bend_in_a_v3_exact_region_is_admitted_and_recovered_on_the_arc() {
    for base in [Base::X0, Base::Y0] {
        let mut document = base.current();
        let region = &case_mut(&mut document, base.pressurized_case())["pressure_regions"][0];
        assert!(region["member_pipe_ids"].as_array().unwrap().contains(&json!(base.region_pipe())));
        push(&mut document, "components", bend(base, true));
        for envelope in run_all(&document) {
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{base:?}: {:#?}", envelope["diagnostics"]);
            assert!(!has_code(&envelope, FAMILY));
            assert!(!has_code(&envelope, "EXACT_PRESSURE_REGION_MEMBER_NOT_STRAIGHT"));
            assert_v3_identity(&envelope);
            let kinds = envelope["results"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|row| {
                    row["entity_ref"] == base.region_pipe()
                        && row["basis_ref"]["ref_id"] == base.pressurized_case()
                })
                .map(|row| row["kind"].as_str().unwrap().to_string())
                .collect::<std::collections::BTreeSet<_>>();
            for kind in ["pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2", "pipe_wall_endpoint_action_v2"] {
                assert!(kinds.contains(kind), "{base:?}: {kind} missing: {kinds:?}");
            }
            for kind in ["pipe_lame_hoop_stress_v2", "pipe_lame_radial_stress_v2", "element_local_axial_force"] {
                assert!(!kinds.contains(kind), "{base:?}: {kind} published on an arc");
            }
            // T4-U4b (§2.1, §2.3): the arc publishes its certified maximum.
            assert!(kinds.contains("pipe_elastic_normal_stress_maximum_v2"), "{base:?}: no arc maximum");
            assert!(!has_code(&envelope, "EXACT_STRESS_GOVERNING_MAXIMUM_UNAVAILABLE"), "{base:?}");
            // Every diagnostic id stays unique (X0 has two cases).
            let ids = diagnostics(&envelope).iter().map(|d| d["id"].as_str().unwrap()).collect::<Vec<_>>();
            let unique = ids.iter().collect::<std::collections::BTreeSet<_>>();
            assert_eq!(unique.len(), ids.len(), "{base:?}: {ids:#?}");
        }
    }
}

/// Unknown contract identities are refused as unsupported, and the retired v2
/// contract (B7b) by name with the re-author code, on a current document
/// without and with the group; nothing falls back to another identity.
#[test]
fn unknown_contract_identities_are_refused_on_both_document_versions() {
    for base in [Base::X0, Base::Y0] {
        for (contract, code) in [
            (json!({"version":"2.0.0","mode":"exact_straight_pressure_v2"}), "PRESSURE_MODEL_REAUTHOR_REQUIRED"),
            (json!({"version":"2.0.0","mode":"exact_straight_pressure_v2","shear_model":"euler_bernoulli"}), "PRESSURE_MODEL_REAUTHOR_REQUIRED"),
            (json!({"version":"3.0.0","mode":"exact_straight_pressure_v2"}), "PRESSURE_CONTRACT_UNSUPPORTED"),
            (json!({"version":"2.0.0","mode":"exact_pressure_v3"}), "PRESSURE_CONTRACT_UNSUPPORTED"),
            (json!({"version":"3.0.0","mode":"exact_pressure_v4"}), "PRESSURE_CONTRACT_UNSUPPORTED"),
            (json!({"version":"3.0.0"}), "PRESSURE_CONTRACT_UNSUPPORTED"),
        ] {
            let mut document = base.current();
            document["model"]["pressure_contract"] = contract.clone();
            for envelope in run_all(&document) {
                assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE", "{contract}");
                assert!(has_code(&envelope, code), "{contract}: {code}");
                let other = if code == "PRESSURE_CONTRACT_UNSUPPORTED" { "PRESSURE_MODEL_REAUTHOR_REQUIRED" } else { "PRESSURE_CONTRACT_UNSUPPORTED" };
                assert!(!has_code(&envelope, other), "{contract}: not {other}");
                assert_ne!(envelope["producer"]["semantic_contract_id"], PRESSURE_ID);
            }
        }
    }
}

/// p < 0 is admitted on v3 and accepted by its reader. (B7b deleted v2's
/// named refusal of it.)
#[test]
fn negative_pressure_is_admitted_on_v3() {
    for base in [Base::X0, Base::Y0] {
        let mut v3 = base.current();
        case_mut(&mut v3, base.pressurized_case())["pressure_regions"][0]["pressure"] =
            base.negative_pressure();
        for envelope in run_all(&v3) {
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{base:?}");
            assert!(!has_code(&envelope, NEGATIVE));
            let regions = envelope["contract_evidence"]["pressure"].as_array().unwrap();
            assert!(regions.iter().any(|r| r["p_pa"].as_f64().unwrap() < 0.0));
            assert_v3_identity(&envelope);
            assert_readers_dispatch_v3(&envelope);
        }
    }
}

/// The shared reader corpus (RE, Python): X0 under v3 with p < 0
/// and Y0, both modes, captured entry. Regenerate with
/// `T4_U2A_WRITE_CORPUS=1`; otherwise the committed bytes must be current.
#[test]
fn shared_pressure_v3_reader_corpus_is_current() {
    let mut cases = Vec::new();
    for base in [Base::X0, Base::Y0] {
        let mut document = base.current();
        if matches!(base, Base::X0) {
            case_mut(&mut document, base.pressurized_case())["pressure_regions"][0]["pressure"] =
                base.negative_pressure();
        }
        for (mode, name) in MODES.into_iter().zip(["sparse_interactive", "dense_scrutiny"]) {
            let envelope = serde_json::to_value(
                run_linear_static_preview_value_with_mode(document.clone(), mode).unwrap(),
            )
            .unwrap();
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED");
            semantic_contract::validate_pressure_evidence(&envelope).unwrap();
            cases.push(json!({"label":format!("{} v3 {name}", match base { Base::X0 => "X0 p<0", Base::Y0 => "Y0" }),
                "document_schema_version":document["model"]["schema_version"],"solver_mode":name,"envelope":envelope}));
        }
    }
    let corpus = json!({
        "corpus":"T4-U2a pressure-1 straight reader corpus",
        "generator":"core/product_physics/tests/exact_admission_seam.rs::shared_pressure_v3_reader_corpus_is_current",
        "inputs":["core/product_physics/tests/fixtures/exact_pressure_connected_request.json (X0, model 0.9.0, pressure_contract 3.0.0/exact_pressure_v3, case:closed-pressure p = -2000 kPa)",
            "fixtures/product_preview/pressure/load_reference/pressure.request.json (Y0, model 0.9.0 with the load/reference-state group, pressure_contract 3.0.0/exact_pressure_v3)"],
        "cases":cases,
    });
    let bytes = serde_json::to_string_pretty(&corpus).unwrap() + "\n";
    if std::env::var_os("T4_U2A_WRITE_CORPUS").is_some() {
        std::fs::write(CORPUS_PATH, &bytes).unwrap();
        return;
    }
    assert!(CORPUS == bytes, "the shared pressure-2 reader corpus is stale; regenerate it");
}

/// B7b (O-10, HELP_HUMAN's technical ruling of 2026-10-13; b7-plan "D2"): the
/// ordinary pressure-2 table is a resident reviewed input, beside the
/// pressure-source table and the pressure result schema; the deleted physics-1
/// table is not. Before B7b this test asserted the opposite (RV130).
#[test]
fn the_pressure_1_table_is_a_reviewed_input() {
    let identity = include_str!("../src/build_identity.rs");
    assert!(!identity.contains("\"../../fixtures/results/semantic_contract_v0_3_physics_1.json\""));
    assert!(identity.contains("\"../../schemas/results_pressure_2.v0.3.schema.json\""));
    assert!(identity.contains("\"../../fixtures/results/semantic_contract_v0_3_pressure_2.json\""));
    assert!(identity.contains("\"../../fixtures/results/semantic_contract_v0_3_pressure_source_1.json\""));
}

/// W-15 (DEL-03-05 Design `rigid-component-stiffness-scaling.md` §2.7): a
/// rigid-family component, `kind` at `node`, mapping `pipe` with scale `s`.
fn rigid(id: &str, kind: &str, node: &str, pipe: &str, s: f64) -> Value {
    json!({"id":id,"kind":kind,"node":node,
        "geometry":{"rigid_pipe_ref":pipe},
        "modifiers":{"stiffness_scaling_user_value":{"value":s,"unit":"1"},"source_reference":"invented W-15 scale"},
        "mechanics_interface":{"solver_consumption":"mechanics_geometry_only"},
        "provenance":PROVENANCE})
}

const REDUCER: &str = "EXACT_PRESSURE_REDUCER_IN_REGION";
const EXTRA_PIPE: &str = "pipe:w15-extra";
const EXTRA_NODE: &str = "node:w15-end";

/// The base with a second straight member, tip to a new end node 1 m on, of
/// the same section and material, outside every region unless `in_region`
/// (then the pressurized region takes it and ends at the new node).
fn extended(base: Base, in_region: bool) -> Value {
    let mut document = base.current();
    let mut node = document["model"]["nodes"][0].clone();
    node["id"] = json!(EXTRA_NODE);
    node["position"] = json!({"x":2,"y":0,"z":0});
    push(&mut document, "nodes", node);
    let mut pipe = document["model"]["pipe_segments"][0].clone();
    pipe["id"] = json!(EXTRA_PIPE);
    pipe["from"] = json!(base.tip());
    pipe["to"] = json!(EXTRA_NODE);
    push(&mut document, "pipe_segments", pipe);
    if let Some(references) = document["model"].get_mut("reference_configurations").and_then(|r| r[0]["member_references"].as_array_mut()) {
        let mut reference = references[0].clone();
        reference["pipe_ref"] = json!(EXTRA_PIPE);
        references.push(reference);
    }
    for case in document["model"]["load_cases"].as_array_mut().unwrap() {
        if let Some(states) = case.get_mut("analysis_state").and_then(|s| s["element_states"].as_array_mut()) {
            let mut state = states[0].clone();
            state["pipe_ref"] = json!(EXTRA_PIPE);
            states.push(state);
        }
    }
    // Y0's far anchor moves to the new end so the chain stays supported as authored.
    for support in document["model"]["supports"].as_array_mut().unwrap() {
        if support["node"] == base.tip() {
            support["node"] = json!(EXTRA_NODE);
        }
    }
    if in_region {
        let region = &mut case_mut(&mut document, base.pressurized_case())["pressure_regions"][0];
        region["member_pipe_ids"].as_array_mut().unwrap().push(json!(EXTRA_PIPE));
        region["terminals"][1]["node_ref"] = json!(EXTRA_NODE);
    }
    document
}

/// W-15: valve, flange, rigid and specialty are admitted on v3 with their
/// scale, inside or outside a pressure region; an unmapped record is
/// admitted and solves as pipe with its named warning.
#[test]
fn w15_rigid_family_is_admitted_on_v3() {
    for base in [Base::X0, Base::Y0] {
        for kind in ["valve", "flange", "rigid", "specialty"] {
            let mut document = base.current();
            push(&mut document, "components", rigid("component:w15", kind, base.tip(), base.region_pipe(), 15.0));
            for envelope in run_all(&document) {
                assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{base:?} {kind}: {:#?}", envelope["diagnostics"]);
                assert!(!has_code(&envelope, FAMILY) && !has_code(&envelope, REDUCER), "{base:?} {kind}");
                assert!(has_code(&envelope, "RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED"), "{base:?} {kind}");
                assert_v3_identity(&envelope);
                assert_readers_dispatch_v3(&envelope);
            }
        }
        for kind in ["valve", "flange", "reducer", "rigid", "specialty"] {
            let mut document = base.current();
            push(&mut document, "components", simple(base, "component:w15", kind));
            for envelope in run_all(&document) {
                assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{base:?} {kind}: {:#?}", envelope["diagnostics"]);
                assert!(!has_code(&envelope, FAMILY), "{base:?} {kind}");
                assert!(!has_code(&envelope, "RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED"), "{base:?} {kind}");
            }
        }
    }
}

/// W-15 §2.1, §2.7: on a closed-end cantilever region (X0), a scaled valve
/// body is an equivalent pipe of its mapped section: the wall axial force is
/// unchanged, and the elastic pressure strain, so the tip's axial motion, is
/// s times smaller.
#[test]
fn w15_scaled_body_in_a_region_keeps_its_wall_force() {
    let base = Base::X0;
    let plain = run_all(&base.current());
    let s = 15.0;
    let mut document = base.current();
    push(&mut document, "components", rigid("component:w15", "valve", base.tip(), base.region_pipe(), s));
    let row = |envelope: &Value, kind: &str, entity: &str| -> f64 {
        envelope["results"].as_array().unwrap().iter()
            .find(|r| r["kind"] == kind && r["entity_ref"] == entity && r["basis_ref"]["ref_id"] == base.pressurized_case())
            .unwrap_or_else(|| panic!("{kind} {entity}"))["value"].as_f64().unwrap()
    };
    for (scaled, plain) in run_all(&document).iter().zip(&plain) {
        let (wall, wall_plain) = (row(scaled, "pipe_wall_axial_force_v2", base.region_pipe()), row(plain, "pipe_wall_axial_force_v2", base.region_pipe()));
        assert!((wall - wall_plain).abs() <= 1e-12 * wall_plain.abs(), "{wall} vs {wall_plain}");
        let (tip, tip_plain) = (row(scaled, "global_nodal_displacement_x", base.tip()), row(plain, "global_nodal_displacement_x", base.tip()));
        assert!(tip_plain != 0.0);
        assert!((tip * s - tip_plain).abs() <= 1e-9 * tip_plain.abs(), "{tip} * {s} vs {tip_plain}");
    }
}

/// W-15 §2.7: a reducer whose mapped member belongs to a pressure region is
/// refused case by case by name, in a single-member region and an equal-bore
/// region; one mapped at a region's boundary node with its own member
/// outside, or outside every region, is admitted like a valve.
#[test]
fn w15_reducer_is_refused_only_where_its_member_is_in_a_region() {
    for base in [Base::X0, Base::Y0] {
        let case = base.pressurized_case();
        let region = "region:fixture-pressure";
        let region = if matches!(base, Base::X0) { region } else { "region:closed" };
        // Inside the single-member region.
        let mut single = base.current();
        push(&mut single, "components", rigid("component:reducer", "reducer", base.tip(), base.region_pipe(), 4.0));
        // Inside an equal-bore two-member region (the guard compares the two equal bores).
        let mut equal = extended(base, true);
        push(&mut equal, "components", rigid("component:reducer", "reducer", base.tip(), EXTRA_PIPE, 4.0));
        for (label, document, pipe) in [("single-member region", single, base.region_pipe()), ("equal-bore region", equal, EXTRA_PIPE)] {
            for envelope in run_all(&document) {
                assert_v3_blocked(&envelope, base);
                assert!(!has_code(&envelope, FAMILY), "{label}");
                assert!(!has_code(&envelope, "PRESSURE_REGION_BORE_MISMATCH"), "{label}");
                let refusal = finding(&envelope, REDUCER, &[case, region, "component:reducer", pipe]);
                assert!(refusal["message"].as_str().unwrap().starts_with("reducer maps a member of this pressure region"), "{label}");
                // Only the pressurized case names it (Y0's hot case has the same region).
                let count = diagnostics(&envelope).iter().filter(|d| d["code"] == REDUCER).count();
                let regions = document["model"]["load_cases"].as_array().unwrap().iter()
                    .filter(|c| c["pressure_regions"].as_array().is_some_and(|r| r.iter().any(|r| r["member_pipe_ids"].as_array().unwrap().contains(&json!(pipe))))).count();
                assert_eq!(count, regions, "{label}: one refusal per case whose region holds the member");
            }
        }
        // At the region's boundary node (the tip), its own member outside the region; and outside every region.
        for (label, node) in [("boundary node", base.tip()), ("outside every region", EXTRA_NODE)] {
            let mut document = extended(base, false);
            push(&mut document, "components", rigid("component:reducer", "reducer", node, EXTRA_PIPE, 4.0));
            for envelope in run_all(&document) {
                assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{base:?} {label}: {:#?}", envelope["diagnostics"]);
                assert!(!has_code(&envelope, REDUCER) && !has_code(&envelope, FAMILY), "{base:?} {label}");
                assert!(has_code(&envelope, "RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED"), "{base:?} {label}");
            }
        }
    }
}

/// W-15 §2.3: a body mapped onto a span an objective connector replaces is
/// refused by name (blocking) on v3; no member is solved for it.
#[test]
fn w15_body_on_a_replaced_span_is_a_mapping_conflict() {
    let mut document: Value =
        serde_json::from_str(include_str!("fixtures/silent_zero/stress_free_connector_empty.request.json")).unwrap();
    let span = document["model"]["components"][0]["objective_connector"]["topology"]["span_ref"].as_str().unwrap().to_string();
    let node = document["model"]["pipe_segments"].as_array().unwrap().iter()
        .find(|pipe| pipe["id"] == span.as_str()).unwrap()["to"].as_str().unwrap().to_string();
    push(&mut document, "components", rigid("component:w15", "valve", &node, &span, 15.0));
    for envelope in run_all(&document) {
        assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE");
        let conflicts = diagnostics(&envelope).iter().filter(|d| d["code"] == "RIGID_COMPONENT_MAPPING_CONFLICT").collect::<Vec<_>>();
        assert_eq!(conflicts.len(), 1, "{:#?}", envelope["diagnostics"]);
        assert_eq!(conflicts[0]["id"], "diagnostic:component:component-w15:rigid-mapping-conflict:replaced-span");
        assert_eq!(conflicts[0]["severity"], "blocking");
        assert_eq!(conflicts[0]["affected_refs"], json!(["component:w15", span]));
        assert_eq!(envelope["results"], json!([]));
    }
}
