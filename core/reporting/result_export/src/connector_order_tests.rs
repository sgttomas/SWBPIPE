//! RT-U4 (order, review of #105): `validate`'s coverage loop with two
//! connector records. Cases run in `exact_cases` order and, within a case,
//! records in the source `connector` order. The envelope is W16B MO-F's (one
//! tie-rod connector `component:C-J`, cases `load:U5A` then `case:second`)
//! with a second record `component:C-K` cloned from it, replacing `pipe:K`,
//! with its rows. `validate` is called directly, on the direct-case view
//! (`exact_combination_evidence::direct_view`, as `physics_evidence` does):
//! the model-level checks of `physics_evidence` are not under test here.
use super::validate;
use serde_json::{json, Value};
use std::collections::HashSet;

const ROD: &str = "tie-rod-force:tie_rod:rod-yp:t_r";
const END_FX: &str = "connector-endpoint-force:end_i:fx";

fn two_connectors() -> Value {
    let corpus: Value =
        serde_json::from_str(include_str!("../../../../fixtures/results/pressure_v3_combination_reader_corpus.json")).unwrap();
    let mut envelope = corpus["cases"].as_array().unwrap().iter()
        .find(|c| c["label"] == "W16B MO-F: two tie rods withheld by name").unwrap()["envelope"].clone();
    let evidence = &mut envelope["contract_evidence"];
    let mut second = evidence["connector"][0].clone();
    second["component_id"] = json!("component:C-K");
    second["replaced_pipe_id"] = json!("pipe:K");
    evidence["connector"].as_array_mut().unwrap().push(second);
    let results = envelope["results"].as_array_mut().unwrap();
    let cloned: Vec<Value> = results.iter().filter(|r| r["entity_ref"] == "component:C-J").map(|r| {
        let mut row = r.clone();
        row["entity_ref"] = json!("component:C-K");
        row["id"] = json!(r["id"].as_str().unwrap().replace("component-C-J", "component-C-K"));
        row
    }).collect();
    assert!(cloned.len() > 24);
    results.extend(cloned);
    envelope
}

fn check(envelope: &Value) -> Result<(), String> {
    let view = crate::exact_combination_evidence::direct_view(envelope).unwrap();
    let envelope = &view;
    let evidence = &envelope["contract_evidence"];
    let exact_cases = evidence["exact_cases"].as_array().unwrap();
    let cases: HashSet<&str> = exact_cases.iter().map(|c| c["load_case_id"].as_str().unwrap()).collect();
    let result = validate(&evidence["connector"], &evidence["pressure"], envelope["results"].as_array().unwrap(),
        &HashSet::new(), &cases, exact_cases, true, true);
    for _ in 0..32 {
        assert_eq!(validate(&evidence["connector"], &evidence["pressure"], envelope["results"].as_array().unwrap(),
            &HashSet::new(), &cases, exact_cases, true, true), result);
    }
    result
}

/// Removes the row `result:[loadcase:<case>:]connector:<component>:<tail>`.
fn remove(envelope: &mut Value, case: &str, component: &str, tail: &str) {
    let qualifier = if case == "load:U5A" { String::new() } else { format!("loadcase:{}:", case.replace(':', "-")) };
    let id = format!("result:{qualifier}connector:{component}:{tail}");
    let results = envelope["results"].as_array_mut().unwrap();
    let at = results.iter().position(|r| r["id"] == id.as_str()).unwrap_or_else(|| panic!("no row {id}"));
    results.remove(at);
}

fn reverse_records(envelope: &mut Value) {
    envelope["contract_evidence"]["connector"].as_array_mut().unwrap().reverse();
}

#[test]
fn the_two_connector_control_admits_in_both_record_orders() {
    let mut envelope = two_connectors();
    assert_eq!(check(&envelope), Ok(()));
    reverse_records(&mut envelope);
    assert_eq!(check(&envelope), Ok(()));
}

/// One case, two records: a rod row missing on C-J (TIE_ROD_ROWS) and an end
/// row missing on C-K (ROW_COVERAGE). The record first in `connector` is
/// reported, in both arrangements.
#[test]
fn within_a_case_the_first_record_in_connector_order_is_reported() {
    let mut envelope = two_connectors();
    remove(&mut envelope, "load:U5A", "component-C-J", ROD);
    remove(&mut envelope, "load:U5A", "component-C-K", END_FX);
    assert_eq!(check(&envelope), Err("SOURCE_PHYSICS_CONNECTOR_TIE_ROD_ROWS".into()));
    reverse_records(&mut envelope);
    assert_eq!(check(&envelope), Err("SOURCE_PHYSICS_CONNECTOR_ROW_COVERAGE".into()));
}

/// Cases before records: the earlier case's defect on the later record
/// (C-K, `load:U5A`) is reported before the later case's defect on the
/// earlier record (C-J, `case:second`). Records-before-cases would report the
/// second.
#[test]
fn cases_precede_records() {
    let mut envelope = two_connectors();
    remove(&mut envelope, "load:U5A", "component-C-K", END_FX);
    remove(&mut envelope, "case:second", "component-C-J", ROD);
    assert_eq!(check(&envelope), Err("SOURCE_PHYSICS_CONNECTOR_ROW_COVERAGE".into()));
    reverse_records(&mut envelope);
    assert_eq!(check(&envelope), Err("SOURCE_PHYSICS_CONNECTOR_ROW_COVERAGE".into()));
}
