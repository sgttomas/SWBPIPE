//! The shared object-form comparison (W-11, DEL-08-01 Design
//! `exact-route-report-sections.md` §6.3; Codex review round 3 of #1648): a
//! struct written positionally at any depth is found, with its fixed path;
//! collections and tuples pass; nothing in the path echoes input text.
//! Invented data only. Standard claim fence applies (claims taxonomy per
//! DEC-081).

use open_pipe_stress_report_renderer::decode_renderable_report_input;
use open_pipe_stress_report_renderer::object_form::{positional_struct_path, report_positional_struct_path};
use serde_json::{json, Value};

const STRAIGHT: &str = include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_straight.json");
const PROVENANCE: [&str; 8] = [
    "source_name", "source_location", "source_license", "contributor", "contributor_certification", "redistribution_status", "review_status",
    "privacy_classification",
];

fn report() -> Value {
    serde_json::from_str::<Value>(STRAIGHT).unwrap()["request"]["report"].clone()
}

fn positional_at(report: &mut Value, pointer: &str) {
    let record = report.pointer_mut(pointer).unwrap();
    *record = Value::Array(PROVENANCE.iter().map(|name| record[*name].clone()).collect());
}

#[test]
fn a_genuine_report_with_every_array_field_passes() {
    assert_eq!(report_positional_struct_path(&report()), None);
}

#[test]
fn a_positional_struct_is_found_at_every_depth_with_its_fixed_path() {
    for (pointer, path) in [
        ("/calculation_report/provenance", "calculation_report.provenance"),
        ("/calculation_report/model_input_summary/provenance", "calculation_report.model_input_summary.provenance"),
        ("/calculation_report/load_case_summary/1/provenance", "calculation_report.load_case_summary[1].provenance"),
        ("/calculation_report/result_export_refs/0/provenance", "calculation_report.result_export_refs[0].provenance"),
        ("/report_sections/provenance_notes/1", "report_sections.provenance_notes[1]"),
        ("/report_sections/limitations/1/provenance", "report_sections.limitations[1].provenance"),
    ] {
        let mut input = report();
        input.pointer_mut(pointer).unwrap()["review_status"] = json!("quarantined");
        positional_at(&mut input, pointer);
        // The renderer's decode reads it.
        assert!(decode_renderable_report_input(input.clone()).is_ok(), "{pointer}");
        let found = report_positional_struct_path(&input);
        assert_eq!(found.as_deref(), Some(path), "{pointer}");
        assert!(!found.unwrap().contains("quarantined"));
    }
    // The whole input positional.
    let input = report();
    let members = ["report_title", "calculation_report", "report_sections", "result_rows", "report_context", "user_rule_checks", "exact_route", "library_references"];
    let positional = Value::Array(members.iter().map(|name| input.get(*name).cloned().unwrap_or(Value::Null)).collect());
    assert!(decode_renderable_report_input(positional.clone()).is_ok());
    assert_eq!(report_positional_struct_path(&positional).as_deref(), Some("the input"));
}

#[test]
fn an_undecodable_input_is_left_to_the_decode() {
    assert_eq!(report_positional_struct_path(&json!({"report_title": "x"})), None);
    assert_eq!(report_positional_struct_path(&json!([1, 2])), None);
}

#[test]
fn collections_tuples_and_maps_pass_and_map_keys_are_not_echoed() {
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Inner {
        a: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Outer {
        list: Vec<u32>,
        pair: (u32, String),
        map: std::collections::BTreeMap<String, Inner>,
        value: Value,
    }
    let object = json!({"list": [1, 2], "pair": [3, "x"], "map": {"private-key-text": {"a": 1}}, "value": [[1], {"b": [2]}]});
    let decoded: Outer = serde_json::from_value(object.clone()).unwrap();
    assert_eq!(positional_struct_path(&decoded, &object), None);
    let positional = json!({"list": [1, 2], "pair": [3, "x"], "map": {"private-key-text": [1]}, "value": []});
    let decoded: Outer = serde_json::from_value(positional.clone()).unwrap();
    assert_eq!(positional_struct_path(&decoded, &positional).as_deref(), Some("map.<key>"));
}
