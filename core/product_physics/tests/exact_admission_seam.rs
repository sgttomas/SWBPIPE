//! T4-U2a: the exact route's admission seam and the `3.0.0/exact_pressure_v3`
//! identity, through PP's public entries.
//!
//! Inputs are the committed invented fixtures patched inline: X0 =
//! `fixtures/exact_pressure_connected_request.json` (0.3.0), Y0 =
//! `fixtures/product_preview/load_reference/pressure.request.json` (0.4.0). A v3
//! twin changes only `pressure_contract`. "Both entries" are the captured
//! value entry and the typed entry; "both modes" are sparse-interactive and
//! dense-scrutiny.

use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    PreviewSolverMode,
};
use open_pipe_stress_result_export::semantic_contract;
use serde_json::{json, Value};

const X0: &str = include_str!("fixtures/exact_pressure_connected_request.json");
const Y0: &str =
    include_str!("../../../fixtures/product_preview/load_reference/pressure.request.json");
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
const PRESSURE_ID: &str = "openpipestress.result_semantics/0.3.0/pressure-1";
const PROVENANCE: &str = "invented_u2a_control";

#[derive(Clone, Copy, Debug)]
enum Base {
    X0,
    Y0,
}

impl Base {
    fn v2(self) -> Value {
        serde_json::from_str(match self {
            Base::X0 => X0,
            Base::Y0 => Y0,
        })
        .unwrap()
    }
    /// The v3 twin: only the contract identity changes.
    fn v3(self) -> Value {
        let mut document = self.v2();
        document["model"]["pressure_contract"] = json!({"version":"3.0.0","mode":"exact_pressure_v3"});
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

/// 0.4.0 cases declare every support's participation.
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
        "3.0.0/exact_pressure_v3 under pressure-1 semantics",
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

/// The v3 raw reader admits the envelope; the v2 readers refuse it by name.
fn assert_readers_dispatch_v3(envelope: &Value, base: Base) {
    semantic_contract::validate_pressure_evidence(envelope).expect("pressure-1 reader");
    let (table, _) = semantic_contract::for_source(envelope).expect("pressure-1 dispatch");
    assert_eq!(table["semantic_contract_id"], PRESSURE_ID);
    let as_v2 = match base {
        Base::X0 => semantic_contract::validate_physics_evidence(envelope).unwrap_err(),
        Base::Y0 => semantic_contract::validate_load_reference_evidence(envelope).unwrap_err(),
    };
    assert!(as_v2.ends_with(semantic_contract::V3_READ_AS_V2), "{as_v2}");
}

#[test]
fn v3_straight_documents_publish_under_pressure_1_and_dispatch_in_the_reader() {
    for base in [Base::X0, Base::Y0] {
        for envelope in run_all(&base.v3()) {
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{base:?}");
            assert!(!has_code(&envelope, FAMILY));
            assert!(envelope.get("source_block_recovery").is_none());
            assert_v3_identity(&envelope);
            assert_readers_dispatch_v3(&envelope, base);
        }
        // A v2 result is not read as v3.
        let v2 = &run_all(&base.v2())[0];
        let as_v3 = semantic_contract::validate_pressure_evidence(v2).unwrap_err();
        assert!(as_v3.ends_with(semantic_contract::V2_READ_AS_V3), "{as_v3}");
        // A v3 envelope relabelled with a v2 identity is refused by the v2 dispatch.
        let mut relabelled = run_all(&base.v3()).swap_remove(0);
        relabelled["producer"]["semantic_contract_id"] = match base {
            Base::X0 => json!(semantic_contract::PHYSICS_ID),
            Base::Y0 => json!(semantic_contract::LOAD_REFERENCE_ID),
        };
        assert!(semantic_contract::for_source(&relabelled).is_err());
    }
}

/// Every published number of a v3 twin is bit-equal to its v2 original
/// (SP-1's twin clause), apart from the closed exclusion list.
#[test]
fn v3_twin_numbers_are_bit_equal_to_v2() {
    for base in [Base::X0, Base::Y0] {
        let (v2, v3) = (run_all(&base.v2()), run_all(&base.v3()));
        for (original, twin) in v2.iter().zip(&v3) {
            let mut differences = Vec::new();
            compare("", original, twin, &mut differences);
            assert!(differences.is_empty(), "{base:?}: {differences:#?}");
        }
    }
}

/// The closed exclusion list (T4-I8 /sp1_pair_scope E1-E4): the identity and
/// semantics strings, the approximation text and the evidence strings naming
/// the contract. No numeric leaf is excluded.
fn excluded(path: &str) -> bool {
    path == "/producer/semantic_contract_id"
        || path == "/formulation_basis/profile_id"
        || path.starts_with("/formulation_basis/limitations")
        || (path.starts_with("/contract_evidence/exact_cases/") && path.ends_with("/profile_mode"))
        || (path.starts_with("/contract_evidence/pressure/")
            && (path.ends_with("/profile_mode") || path.ends_with("/profile_version")))
}

fn compare(path: &str, a: &Value, b: &Value, differences: &mut Vec<String>) {
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            for key in x.keys().chain(y.keys().filter(|k| !x.contains_key(*k))) {
                let child = format!("{path}/{key}");
                match (x.get(key), y.get(key)) {
                    (Some(p), Some(q)) => compare(&child, p, q, differences),
                    _ if excluded(&child) => {}
                    _ => differences.push(format!("{child}: key present on one side")),
                }
            }
        }
        (Value::Array(x), Value::Array(y)) if x.len() == y.len() || excluded(path) => {
            if !excluded(path) {
                for (index, (p, q)) in x.iter().zip(y).enumerate() {
                    compare(&format!("{path}/{index}"), p, q, differences);
                }
            }
        }
        (Value::Number(x), Value::Number(y)) => {
            let (p, q) = (x.as_f64().unwrap(), y.as_f64().unwrap());
            if p.to_bits() != q.to_bits() || x.to_string() != y.to_string() {
                differences.push(format!("{path}: {x} vs {y}"));
            }
        }
        _ if a == b || excluded(path) => {}
        _ => differences.push(format!("{path}: {a} vs {b}")),
    }
}

/// Every family not yet admitted is refused by name on v3, on 0.3.0 and 0.4.0,
/// in both entries and both modes; v2 keeps its codes (T4-U0's tests).
#[test]
fn each_family_not_yet_admitted_is_refused_by_name_on_v3() {
    for base in [Base::X0, Base::Y0] {
        let components = [
            (bend(base, true), "realized curved bend"),
            (bend(base, false), "geometry-only bend"),
            (simple(base, "component:valve", "valve"), "valve"),
            (simple(base, "component:flange", "flange"), "flange"),
            (simple(base, "component:reducer", "reducer"), "reducer"),
            (simple(base, "component:tee", "tee"), "branch connection"),
            (simple(base, "component:rigid", "rigid"), "rigid or specialty"),
            (simple(base, "component:specialty", "specialty"), "rigid or specialty"),
            (simple(base, "component:other", "other"), "component"),
            (annotation_joint(base), "expansion joint (annotation only"),
        ];
        let mut cases = components
            .into_iter()
            .map(|(component, name)| {
                let id = component["id"].as_str().unwrap().to_string();
                let mut document = base.v3();
                push(&mut document, "components", component);
                (document, vec![id], name)
            })
            .collect::<Vec<_>>();
        for (support, name) in [(gap(base), "nonlinear support"), (constant_effort(base), "constant-effort support")] {
            let id = support["id"].as_str().unwrap().to_string();
            let mut document = base.v3();
            push(&mut document, "supports", support);
            declare_support_state(&mut document, &id);
            cases.push((document, vec![id], name));
        }
        let [first, second] = base.cases();
        let mut combination = base.v3();
        combination["model"]["combinations"] = json!([{"id":"combination:sum","basis":"mechanics",
            "terms":[{"load_case":first,"factor":1.0},{"load_case":second,"factor":1.0}],"provenance":PROVENANCE}]);
        cases.push((combination, vec!["combination:sum".into()], "load combination"));
        let mut equivalent = base.v3();
        case_mut(&mut equivalent, second)["equivalent_static"] = json!({"provenance":PROVENANCE});
        cases.push((equivalent, vec![second.into(), "equivalent_static".into()], "equivalent-static generation"));
        for (document, refs, name) in cases {
            let refs = refs.iter().map(String::as_str).collect::<Vec<_>>();
            for envelope in run_all(&document) {
                assert_v3_blocked(&envelope, base);
                let message = finding(&envelope, FAMILY, &refs)["message"].as_str().unwrap().to_string();
                assert!(message.contains(name), "{name}: {message}");
                assert!(message.contains("is not yet admitted under 3.0.0/exact_pressure_v3"), "{message}");
            }
        }
    }
}

/// T4-U3 (D-4): a legacy joint takes `LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED`
/// (refs [component, pipe]) on v2 and v3 alike, first and alone: the seam skips
/// it, so neither the v2 composition code nor the v3 family code names it. An
/// explicit annotation is exempt from the legacy code even when it carries a
/// pipe reference: v2 gives the composition refusal, v3 the seam's.
#[test]
fn legacy_joints_take_the_legacy_code_and_annotations_the_route_refusal() {
    const LEGACY: &str = "LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED";
    for base in [Base::X0, Base::Y0] {
        for document in [base.v2(), base.v3()] {
            let mut document = document;
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
        }
        let mut annotation_with_pipe = annotation_joint(base);
        annotation_with_pipe["geometry"] = json!({"expansion_joint_pipe_ref": base.region_pipe()});
        for annotation in [annotation_joint(base), annotation_with_pipe] {
            let mut v2 = base.v2();
            push(&mut v2, "components", annotation.clone());
            for envelope in run_all(&v2) {
                assert!(has_code(&envelope, COMPOSITION));
                assert!(!has_code(&envelope, LEGACY) && !has_code(&envelope, "EXPANSION_JOINT_ANNOTATION_ONLY"));
            }
            let mut v3 = base.v3();
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
/// never the seam's family refusal. v2 keeps its connector codes.
#[test]
fn v3_connectors_reach_their_classifier_not_the_family_refusal() {
    const INCOMPLETE: &str = "OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE";
    for base in [Base::X0, Base::Y0] {
        let mut annotated = connector(base);
        annotated["mechanics_interface"] = json!({"solver_consumption": "not_solver_consumed"});
        for component in [connector(base), annotated] {
            let mut v3 = base.v3();
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
        let mut v2 = base.v2();
        push(&mut v2, "components", connector(base));
        for envelope in run_all(&v2) {
            assert!(has_code(&envelope, "OBJECTIVE_CONNECTOR_NOT_IMPLEMENTED") && has_code(&envelope, COMPOSITION));
            assert!(!has_code(&envelope, INCOMPLETE));
        }
    }
}

/// T4-U0's A3 end to end: a realized bend on a member of a v3 exact pressure
/// region reaches the seam's named refusal (no panic, no maximum row) in both
/// entries and both modes.
#[test]
fn a3_realized_bend_in_a_v3_exact_region_reaches_the_seam_refusal() {
    for base in [Base::X0, Base::Y0] {
        let mut document = base.v3();
        let region = &case_mut(&mut document, base.pressurized_case())["pressure_regions"][0];
        assert!(region["member_pipe_ids"].as_array().unwrap().contains(&json!(base.region_pipe())));
        push(&mut document, "components", bend(base, true));
        for envelope in run_all(&document) {
            assert_v3_blocked(&envelope, base);
            finding(&envelope, FAMILY, &["component:bend"]);
            assert!(!envelope.to_string().contains("pipe_elastic_normal_stress_maximum_v2"));
            assert!(!has_code(&envelope, "EXACT_PRESSURE_REGION_MEMBER_NOT_STRAIGHT"));
        }
    }
}

#[test]
fn unknown_contract_identities_are_refused_on_both_document_versions() {
    for base in [Base::X0, Base::Y0] {
        for contract in [
            json!({"version":"3.0.0","mode":"exact_straight_pressure_v2"}),
            json!({"version":"2.0.0","mode":"exact_pressure_v3"}),
            json!({"version":"3.0.0","mode":"exact_pressure_v4"}),
            json!({"version":"3.0.0"}),
        ] {
            let mut document = base.v2();
            document["model"]["pressure_contract"] = contract.clone();
            for envelope in run_all(&document) {
                assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE", "{contract}");
                assert!(has_code(&envelope, "PRESSURE_CONTRACT_UNSUPPORTED"), "{contract}");
                assert_ne!(envelope["producer"]["semantic_contract_id"], PRESSURE_ID);
            }
        }
    }
}

/// p < 0 is admitted on v3 (and accepted by its reader) and refused by name on v2.
#[test]
fn negative_pressure_is_admitted_on_v3_and_refused_on_v2() {
    for base in [Base::X0, Base::Y0] {
        let mut v3 = base.v3();
        let mut v2 = base.v2();
        for document in [&mut v3, &mut v2] {
            case_mut(document, base.pressurized_case())["pressure_regions"][0]["pressure"] =
                base.negative_pressure();
        }
        for envelope in run_all(&v3) {
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{base:?}");
            assert!(!has_code(&envelope, NEGATIVE));
            let regions = envelope["contract_evidence"]["pressure"].as_array().unwrap();
            assert!(regions.iter().any(|r| r["p_pa"].as_f64().unwrap() < 0.0));
            assert_v3_identity(&envelope);
            assert_readers_dispatch_v3(&envelope, base);
        }
        for envelope in run_all(&v2) {
            assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE");
            finding(&envelope, NEGATIVE, &[base.pressurized_case(), region_id(base), "pressure"]);
        }
    }
}

fn region_id(base: Base) -> &'static str {
    match base {
        Base::X0 => "region:fixture-pressure",
        Base::Y0 => "region:closed",
    }
}

/// The shared reader corpus (RE, Python, TypeScript): X0 under v3 with p < 0
/// and Y0's v3 twin, both modes, captured entry. Regenerate with
/// `T4_U2A_WRITE_CORPUS=1`; otherwise the committed bytes must be current.
#[test]
fn shared_pressure_v3_reader_corpus_is_current() {
    let mut cases = Vec::new();
    for base in [Base::X0, Base::Y0] {
        let mut document = base.v3();
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
            cases.push(json!({"label":format!("{} v3 {name}", match base { Base::X0 => "X0 p<0", Base::Y0 => "Y0 twin" }),
                "document_schema_version":document["model"]["schema_version"],"solver_mode":name,"envelope":envelope}));
        }
    }
    let corpus = json!({
        "corpus":"T4-U2a pressure-1 straight reader corpus",
        "generator":"core/product_physics/tests/exact_admission_seam.rs::shared_pressure_v3_reader_corpus_is_current",
        "inputs":["core/product_physics/tests/fixtures/exact_pressure_connected_request.json (X0, pressure_contract 3.0.0/exact_pressure_v3, case:closed-pressure p = -2000 kPa)",
            "fixtures/product_preview/load_reference/pressure.request.json (Y0, pressure_contract 3.0.0/exact_pressure_v3)"],
        "cases":cases,
    });
    let bytes = serde_json::to_string_pretty(&corpus).unwrap() + "\n";
    if std::env::var_os("T4_U2A_WRITE_CORPUS").is_some() {
        std::fs::write(CORPUS_PATH, &bytes).unwrap();
        return;
    }
    assert!(CORPUS == bytes, "the shared pressure-1 reader corpus is stale; regenerate it");
}

/// The pressure-1 table skeleton stays outside the reviewed inputs (T3 item;
/// RV130), so no registered profile or build identity depends on it.
#[test]
fn the_pressure_1_table_is_not_a_reviewed_input() {
    let identity = include_str!("../src/build_identity.rs");
    assert!(identity.contains("semantic_contract_v0_3_physics_1.json"));
    assert!(!identity.contains("pressure_1"));
}
