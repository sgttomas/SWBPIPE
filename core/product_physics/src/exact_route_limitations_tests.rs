//! W-11 (DEL-08-01 Design `exact-route-report-sections.md` §2.2): the exact
//! routes' limitation coverage fixture (`fixtures/reports/exact_route_limitations.json`),
//! written from the producer's own composing seam (`exact_formulation_basis`)
//! over its whole domain, and the certified-recovery statements entering each
//! route's list exactly once. The report's statement table
//! (`apps/desktop/src/features/report/exactReportLimitations.ts`) is checked
//! against this fixture. With `OPENPIPESTRESS_WRITE_EXACT_ROUTE_LIMITATIONS=<path>`
//! the fixture test writes the fixture there instead of comparing (the
//! maintained writer). Invented models only. Standard claim fence applies
//! (claims taxonomy per DEC-081).

use super::*;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeSet;

const FIXTURE: &str = include_str!("../../../fixtures/reports/exact_route_limitations.json");
const WRITE_ENV: &str = "OPENPIPESTRESS_WRITE_EXACT_ROUTE_LIMITATIONS";
const GENERATOR: &str =
    "core/product_physics/src/exact_route_limitations_tests.rs::exact_route_limitations_fixture_is_current";

/// The committed exact corpora (§1.3), each with its published limitation
/// lists found wherever an object carries a producer and a formulation basis.
const EXACT_CORPORA: [(&str, &str); 15] = [
    ("pressure_v3_straight_reader_corpus", include_str!("../../../fixtures/results/pressure_v3_straight_reader_corpus.json")),
    ("pressure_v3_arc_reader_corpus", include_str!("../../../fixtures/results/pressure_v3_arc_reader_corpus.json")),
    ("pressure_v3_connector_reader_corpus", include_str!("../../../fixtures/results/pressure_v3_connector_reader_corpus.json")),
    ("pressure_v3_joint_reader_control", include_str!("../../../fixtures/results/pressure_v3_joint_reader_control.json")),
    ("pressure_source_load_state_eigen_motion_dense", include_str!("../../../fixtures/results/pressure_source_load_state_eigen_motion_dense.document.json")),
    ("pressure_source_load_state_eigen_motion_sparse", include_str!("../../../fixtures/results/pressure_source_load_state_eigen_motion_sparse.document.json")),
    ("pressure_source_load_state_fields_dense", include_str!("../../../fixtures/results/pressure_source_load_state_fields_dense.document.json")),
    ("pressure_source_load_state_fields_sparse", include_str!("../../../fixtures/results/pressure_source_load_state_fields_sparse.document.json")),
    ("pressure_source_load_state_mixed_dense", include_str!("../../../fixtures/results/pressure_source_load_state_mixed_dense.document.json")),
    ("pressure_source_load_state_mixed_sparse", include_str!("../../../fixtures/results/pressure_source_load_state_mixed_sparse.document.json")),
    ("pressure_source_load_state_n05_dense", include_str!("../../../fixtures/results/pressure_source_load_state_n05_dense.document.json")),
    ("pressure_source_load_state_n05_sparse", include_str!("../../../fixtures/results/pressure_source_load_state_n05_sparse.document.json")),
    ("pressure_source_load_state_n06_dense", include_str!("../../../fixtures/results/pressure_source_load_state_n06_dense.document.json")),
    ("pressure_source_load_state_n06_sparse", include_str!("../../../fixtures/results/pressure_source_load_state_n06_sparse.document.json")),
    ("physics_source_n05_sparse_raw", include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json")),
];
/// A preview-physics-2 envelope: the recovery statements end its list too.
const PREVIEW_ENVELOPE: &str = include_str!("../../../fixtures/results/preview_physics_connected_sparse.json");
/// Straight exact models without and with the load/reference-state group.
const STRAIGHT_REQUEST: &str = include_str!("../tests/fixtures/exact_pressure_connected_request.json");
const LOAD_STATE_REQUEST: &str = include_str!("../../../fixtures/product_preview/pressure/load_reference/pressure.request.json");

#[derive(Serialize)]
struct CoverageFixture {
    schema_version: &'static str,
    generator: &'static str,
    lists: Vec<CoverageList>,
    strings: Vec<String>,
}

#[derive(Serialize)]
struct CoverageList {
    semantic_contract_id: &'static str,
    load_reference_states: bool,
    objective_connectors: bool,
    realized_bends: bool,
    reachable: bool,
    limitations: Vec<String>,
}

/// The seam's whole domain, in fixture order: `selected` (pressure-2, then
/// pressure-source-1), then the load/reference-state group, connectors and
/// realized bends, each absent before present.
fn domain() -> Vec<(bool, bool, bool, bool)> {
    let mut shapes = Vec::new();
    for selected in [false, true] {
        for load_state in [false, true] {
            for connectors in [false, true] {
                for bends in [false, true] {
                    shapes.push((selected, load_state, connectors, bends));
                }
            }
        }
    }
    shapes
}

fn identity(selected: bool) -> &'static str {
    if selected {
        exact_admission::PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID
    } else {
        PRESSURE_SEMANTIC_CONTRACT_ID
    }
}

fn coverage_fixture() -> CoverageFixture {
    let mut lists = Vec::new();
    let mut strings = BTreeSet::new();
    for (selected, load_state, connectors, bends) in domain() {
        let basis = exact_formulation_basis(load_state, connectors, bends, selected);
        assert_eq!(basis.profile_id, exact_admission::PRESSURE_PROFILE_ID);
        strings.extend(basis.limitations.iter().cloned());
        lists.push(CoverageList {
            semantic_contract_id: identity(selected),
            load_reference_states: load_state,
            objective_connectors: connectors,
            realized_bends: bends,
            // The source family admits no components, so the producer never
            // selects a model with a connector or a realized bend.
            reachable: !(selected && (connectors || bends)),
            limitations: basis.limitations,
        });
    }
    // BTreeSet<String> orders by bytes, which for UTF-8 is code-point order.
    CoverageFixture { schema_version: "1.0.0", generator: GENERATOR, lists, strings: strings.into_iter().collect() }
}

/// Every published (identity, limitations) pair in `document`.
fn published_lists(document: &Value, out: &mut Vec<(String, Vec<String>)>) {
    match document {
        Value::Object(object) => {
            let id = object.get("producer").and_then(|producer| producer["semantic_contract_id"].as_str());
            let limitations = object.get("formulation_basis").and_then(|basis| basis["limitations"].as_array());
            if let (Some(id), Some(limitations)) = (id, limitations) {
                let texts = limitations.iter().map(|text| text.as_str().unwrap().to_string()).collect();
                out.push((id.to_string(), texts));
            }
            object.values().for_each(|value| published_lists(value, out));
        }
        Value::Array(items) => items.iter().for_each(|value| published_lists(value, out)),
        _ => {}
    }
}

fn corpus_lists() -> Vec<(String, String, Vec<String>)> {
    let mut found = Vec::new();
    for (name, text) in EXACT_CORPORA {
        let mut lists = Vec::new();
        published_lists(&serde_json::from_str(text).unwrap(), &mut lists);
        assert!(!lists.is_empty(), "{name} publishes no limitation list");
        found.extend(lists.into_iter().map(|(id, list)| (name.to_string(), id, list)));
    }
    found
}

#[test]
fn exact_route_limitations_fixture_is_current() {
    let fixture = coverage_fixture();
    assert_eq!(fixture.lists.len(), 16);
    assert_eq!(fixture.strings.len(), 19, "§1.3: 2 + 6 + 2 + 5 + 1 + 1 + 2 distinct strings");
    assert_eq!(fixture.lists.iter().filter(|list| !list.reachable).count(), 6);
    for list in &fixture.lists {
        assert!((10..=17).contains(&list.limitations.len()), "{} entries", list.limitations.len());
    }
    // Against real envelopes: every published exact list in the committed
    // corpora equals one fixture list of its own identity.
    let mut shapes = BTreeSet::new();
    for (name, id, published) in corpus_lists() {
        let matched = fixture
            .lists
            .iter()
            .find(|list| list.semantic_contract_id == id && list.limitations == published)
            .unwrap_or_else(|| panic!("{name}: a published {id} list equals no fixture list"));
        assert!(matched.reachable, "{name}: an unreachable shape was published");
        shapes.insert((id, matched.load_reference_states, matched.objective_connectors, matched.realized_bends));
    }
    let p2 = PRESSURE_SEMANTIC_CONTRACT_ID.to_string();
    let ps1 = exact_admission::PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID.to_string();
    let expected: BTreeSet<_> = [
        (p2.clone(), false, false, false),
        (p2.clone(), true, false, false),
        (p2.clone(), false, false, true),
        (p2.clone(), true, false, true),
        (p2.clone(), false, true, true),
        (p2.clone(), false, true, false),
        (p2.clone(), true, true, false),
        (ps1.clone(), true, false, false),
        (ps1, false, false, false),
    ]
    .into_iter()
    .collect();
    assert_eq!(shapes, expected, "the committed corpora's published shapes");

    let text = format!("{}\n", serde_json::to_string_pretty(&fixture).unwrap());
    if let Ok(target) = std::env::var(WRITE_ENV) {
        std::fs::write(target, &text).unwrap();
        return;
    }
    assert!(text == FIXTURE, "the committed coverage fixture is stale; regenerate it with {WRITE_ENV}=<path>");
}

fn assert_recovery_once(list: &[String], what: &str) {
    for statement in CERTIFIED_RECOVERY_LIMITATIONS {
        assert_eq!(list.iter().filter(|text| *text == statement).count(), 1, "{what}: {statement:.40}");
    }
    assert!(list.len() >= 2, "{what}");
    assert_eq!(&list[list.len() - 2..], CERTIFIED_RECOVERY_LIMITATIONS.map(str::to_string).as_slice(), "{what}: the pair is last");
}

/// An exact model of one shape: the straight request with or without the
/// load/reference-state group, with an objective connector and a realized
/// bend added as the shape requires. The basis reads only the shape.
fn exact_model(load_state: bool, connectors: bool, bends: bool) -> PreviewModel {
    let request: Value = serde_json::from_str(if load_state { LOAD_STATE_REQUEST } else { STRAIGHT_REQUEST }).unwrap();
    let mut raw = request["model"].clone();
    let node = raw["nodes"][0]["id"].as_str().unwrap().to_string();
    let components = raw["components"].as_array_mut().unwrap();
    if connectors {
        components.push(json!({"id": "component:invented-joint", "kind": "expansion_joint", "node": node, "objective_connector": {}}));
    }
    if bends {
        components.push(json!({
            "id": "component:invented-bend",
            "kind": "bend",
            "node": node,
            "mechanics_interface": {"solver_consumption": DEC_070_CURVED_BEND_SOLVER_CONSUMPTION}
        }));
    }
    let model: PreviewModel = serde_json::from_value(raw).unwrap();
    assert_eq!(pressure_runtime::exact_contract(&model), Some(pressure_runtime::ExactContract::PressureV3));
    assert_eq!(case_state::is_load_state(&model), load_state);
    assert_eq!(model.components.iter().any(|c| c.objective_connector.is_some()), connectors);
    assert_eq!(model.components.iter().any(is_curved_bend_macro_component), bends);
    model
}

#[test]
fn certified_recovery_statements_enter_once() {
    for (selected, load_state, connectors, bends) in domain() {
        let basis = exact_formulation_basis(load_state, connectors, bends, selected);
        assert_recovery_once(&basis.limitations, &format!("seam {selected} {load_state} {connectors} {bends}"));
    }
    for (name, id, list) in corpus_lists() {
        assert_recovery_once(&list, &format!("{name} ({id})"));
    }
    let mut preview = Vec::new();
    published_lists(&serde_json::from_str(PREVIEW_ENVELOPE).unwrap(), &mut preview);
    assert_eq!(preview.len(), 1);
    assert_eq!(preview[0].0, preview_physics::ID);
    assert_recovery_once(&preview[0].1, "preview-physics-2 envelope");
    // The other arm appends the same constant to the preview basis.
    let mut expected_other = preview_formulation_basis().limitations;
    expected_other.extend(CERTIFIED_RECOVERY_LIMITATIONS.map(str::to_string));
    let pressure_free: PreviewModel = serde_json::from_str(include_str!("../../../fixtures/product_preview/invented_preview_model.json")).unwrap();
    assert!(pressure_runtime::exact_contract(&pressure_free).is_none());
    assert_eq!(formulation_basis_for_model(&pressure_free).limitations, expected_other);
    assert_recovery_once(&expected_other, "the other arm");

    // The producer returns the seam's basis unchanged on the exact arm, so the
    // pair is never appended a second time.
    for (selected, load_state, connectors, bends) in domain() {
        if selected && (connectors || bends) {
            continue;
        }
        let model = exact_model(load_state, connectors, bends);
        let produced = formulation_basis_for_model_with_source(&model, selected);
        assert_eq!(produced, exact_formulation_basis(load_state, connectors, bends, selected));
        assert_recovery_once(&produced.limitations, "producer");
        if !selected {
            assert_eq!(formulation_basis_for_model(&model), produced);
        }
    }
}
