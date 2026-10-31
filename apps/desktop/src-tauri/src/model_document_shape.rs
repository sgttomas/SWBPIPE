//! The desktop model document shape, natively (DEL-15-01 W-6a repair 1).
//!
//! A port of the webview's open-time shape check
//! (`src/features/model-file/modelDocumentShape.ts`,
//! `modelDocumentShapeDiagnostics`): the members the desktop's `PreviewModel`
//! requires. File > Save Model Document… runs it natively on the exact document
//! it admits, so a native caller cannot save a file the webview's open path
//! would refuse. Codes, paths and messages are the webview's; the shared
//! invented corpus `fixtures/model_document/desktop_shape_cases.json` is read
//! by this module's tests and by `modelDocumentShape.parity.test.ts`, so the
//! two verdicts stay equal. Members are visited in `serde_json`'s key order,
//! so the order of several findings inside one object may differ from the
//! webview's; the set is the same.

use serde_json::{Map, Value};

pub const MODEL_FILE_SHAPE: &str = "MODEL-FILE-SHAPE";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapeDiagnostic {
    pub path: String,
    pub message: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    String,
    Object,
    Array,
    Number,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::String => "string",
            Kind::Object => "object",
            Kind::Array => "array",
            Kind::Number => "number",
        }
    }
}

/// `kindOf` of the webview: JSON numbers are always finite.
fn kind_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Array(_) => "array",
        Value::String(_) => "string",
        Value::Number(_) => "number",
        Value::Object(_) => "object",
        Value::Bool(_) => "other",
    }
}

fn matches(value: &Value, kind: Kind) -> bool {
    matches!(
        (value, kind),
        (Value::String(_), Kind::String) | (Value::Object(_), Kind::Object) | (Value::Array(_), Kind::Array) | (Value::Number(_), Kind::Number)
    )
}

/// `[member, required string members, optional collection]`, as the webview's `ENTITY_STRINGS`.
const ENTITY_STRINGS: [(&str, &[&str], bool); 9] = [
    ("materials", &["id", "label", "provenance"], true),
    ("sections", &["id", "name", "section_type"], true),
    ("nodes", &["id", "label", "provenance"], false),
    ("pipe_segments", &["id", "label", "from", "to", "material", "provenance"], false),
    ("supports", &["id", "label", "node", "provenance"], false),
    ("components", &["id", "label", "kind", "node", "provenance"], false),
    ("load_cases", &["id", "label", "kind", "status", "provenance"], false),
    ("combinations", &["id", "label", "basis", "provenance"], true),
    ("diagnostics", &["code", "severity", "message"], false),
];

/// The combination bases whose operands are not terms, as the webview's `TERMLESS_BASES`.
const TERMLESS_BASES: [&str; 2] = ["result_state_subtraction", "range_envelope"];

#[derive(Clone, Copy)]
enum Member {
    String,
    Quantity,
    Point,
}

/// The webview's `COMPONENT_DESCRIPTORS`: the component members the workspace
/// formats or trims on adoption, typed as `PreviewComponent` declares them.
const COMPONENT_DESCRIPTORS: [(&str, &[(&str, Member)]); 3] = [
    ("geometry", &[("bend_radius", Member::Quantity), ("bend_angle", Member::Quantity), ("bend_plane_orientation", Member::String), ("bend_pipe_ref", Member::String), ("bend_geometry_source_reference", Member::String), ("branch_header_pipe_ref", Member::String), ("branch_branch_pipe_ref", Member::String), ("branch_run_size", Member::Quantity), ("branch_header_size", Member::Quantity), ("branch_connection_angle", Member::Quantity), ("branch_connection_type", Member::String), ("branch_reinforcement_area", Member::Quantity), ("branch_reinforcement_reference", Member::String), ("branch_geometry_source_reference", Member::String), ("rigid_pipe_ref", Member::String), ("rigid_body_length", Member::Quantity), ("end_a_size", Member::Quantity), ("end_b_size", Member::Quantity), ("weight", Member::Quantity), ("center_of_gravity", Member::Point), ("eccentric_offset", Member::Point), ("connection_end_a_reference", Member::String), ("connection_end_b_reference", Member::String), ("stiffness_behavior_reference", Member::String), ("rigid_component_source_reference", Member::String), ("expansion_joint_pipe_ref", Member::String), ("effective_area", Member::Quantity), ("movement_limit", Member::Quantity), ("hardware_reference", Member::String), ("manufacturer_reference", Member::String), ("pressure_thrust_reference", Member::String), ("expansion_joint_source_reference", Member::String)]),
    ("modifiers", &[("sif_user_value", Member::Quantity), ("branch_header_sif_user_value", Member::Quantity), ("branch_branch_sif_user_value", Member::Quantity), ("flexibility_factor_user_value", Member::Quantity), ("sif_in_plane_user_value", Member::Quantity), ("sif_out_of_plane_user_value", Member::Quantity), ("stiffness_scaling_user_value", Member::Quantity), ("linear_stiffness_user_value", Member::Quantity), ("rotational_stiffness_user_value", Member::Quantity), ("axial_stiffness_user_value", Member::Quantity), ("lateral_stiffness_user_value", Member::Quantity), ("angular_stiffness_user_value", Member::Quantity), ("torsional_stiffness_user_value", Member::Quantity), ("source_reference", Member::String)]),
    ("mechanics_interface", &[("solver_consumption", Member::String), ("rule_check_consumption", Member::String)]),
];

struct Shape(Vec<ShapeDiagnostic>);

impl Shape {
    /// `need`: true when `value` is of `kind`; otherwise records the finding.
    fn need(&mut self, value: Option<&Value>, path: &str, kind: Kind) -> bool {
        match value {
            Some(value) if matches(value, kind) => true,
            Some(value) => {
                self.0.push(ShapeDiagnostic {
                    path: path.to_string(),
                    message: format!("{path} is {}; {} required.", kind_name(value), kind.name()),
                });
                false
            }
            None => {
                self.0.push(ShapeDiagnostic {
                    path: path.to_string(),
                    message: format!("{path} is missing ({} required).", kind.name()),
                });
                false
            }
        }
    }

    fn quantity(&mut self, value: Option<&Value>, path: &str) {
        if !self.need(value, path, Kind::Object) {
            return;
        }
        let object = value.and_then(Value::as_object);
        self.need(object.and_then(|item| item.get("value")), &format!("{path}.value"), Kind::Number);
        self.need(object.and_then(|item| item.get("unit")), &format!("{path}.unit"), Kind::String);
    }

    fn object<'a>(&mut self, record: &'a Map<String, Value>, key: &str, path: &str) -> Option<&'a Map<String, Value>> {
        if self.need(record.get(key), path, Kind::Object) {
            record.get(key).and_then(Value::as_object)
        } else {
            None
        }
    }
}

/// Named shape findings for `document`; empty means the desktop can open it.
pub fn model_document_shape_diagnostics(document: &Value) -> Vec<ShapeDiagnostic> {
    let mut shape = Shape(Vec::new());
    if !shape.need(Some(document), "$", Kind::Object) {
        return shape.0;
    }
    let doc = document.as_object().expect("checked object");
    shape.need(doc.get("schema_version"), "schema_version", Kind::String);
    shape.need(doc.get("document_kind"), "document_kind", Kind::String);
    if let Some(boundary) = shape.object(doc, "data_boundary", "data_boundary") {
        for (key, value) in boundary {
            shape.need(Some(value), &format!("data_boundary.{key}"), Kind::String);
        }
    }
    if let Some(project) = shape.object(doc, "project", "project") {
        for key in ["id", "name", "description"] {
            shape.need(project.get(key), &format!("project.{key}"), Kind::String);
        }
        if let Some(units) = shape.object(project, "units", "project.units") {
            for (key, value) in units {
                shape.need(Some(value), &format!("project.units.{key}"), Kind::String);
            }
        }
    }
    if let Some(status) = shape.object(doc, "analysis_status", "analysis_status") {
        for key in ["mechanics", "rule_check", "professional_acceptance"] {
            shape.need(status.get(key), &format!("analysis_status.{key}"), Kind::String);
        }
    }
    for (member, required, optional) in ENTITY_STRINGS {
        let list = doc.get(member);
        if optional && list.is_none() {
            continue;
        }
        if !shape.need(list, member, Kind::Array) {
            continue;
        }
        for (index, entry) in list.and_then(Value::as_array).into_iter().flatten().enumerate() {
            let at = format!("{member}[{index}]");
            if !shape.need(Some(entry), &at, Kind::Object) {
                continue;
            }
            let record = entry.as_object().expect("checked object");
            for key in required {
                shape.need(record.get(*key), &format!("{at}.{key}"), Kind::String);
            }
            match member {
                "nodes" => {
                    if let Some(position) = shape.object(record, "position", &format!("{at}.position")) {
                        for axis in ["x", "y", "z"] {
                            shape.need(position.get(axis), &format!("{at}.position.{axis}"), Kind::Number);
                        }
                    }
                }
                "materials" => shape.quantity(record.get("elastic_modulus"), &format!("{at}.elastic_modulus")),
                "sections" => {
                    if let Some(properties) = shape.object(record, "properties", &format!("{at}.properties")) {
                        for (key, value) in properties {
                            shape.quantity(Some(value), &format!("{at}.properties.{key}"));
                        }
                    }
                    if !matches!(record.get("provenance"), Some(Value::String(_) | Value::Object(_))) {
                        shape.need(record.get("provenance"), &format!("{at}.provenance"), Kind::String);
                    }
                }
                "pipe_segments" => {
                    if let Some(section) = shape.object(record, "section", &format!("{at}.section")) {
                        for (key, value) in section {
                            shape.quantity(Some(value), &format!("{at}.section.{key}"));
                        }
                    }
                    // The open's model index trims a present shared-section reference.
                    if let Some(section_ref) = record.get("section_ref").filter(|value| !value.is_null()) {
                        shape.need(Some(section_ref), &format!("{at}.section_ref"), Kind::String);
                    }
                }
                "components" => {
                    for (group, members) in COMPONENT_DESCRIPTORS {
                        let Some(descriptor) = record.get(group).and_then(Value::as_object) else { continue };
                        for (key, member) in members {
                            let Some(value) = descriptor.get(*key).filter(|value| !value.is_null()) else { continue };
                            let path = format!("{at}.{group}.{key}");
                            match member {
                                Member::String => {
                                    shape.need(Some(value), &path, Kind::String);
                                }
                                Member::Quantity => shape.quantity(Some(value), &path),
                                Member::Point => {
                                    if shape.need(Some(value), &path, Kind::Object) {
                                        for axis in ["x", "y", "z"] {
                                            shape.need(value.get(axis), &format!("{path}.{axis}"), Kind::Number);
                                        }
                                        shape.need(value.get("unit"), &format!("{path}.unit"), Kind::String);
                                    }
                                }
                            }
                        }
                    }
                    if let Some(completeness) = record.get("completeness").filter(|value| !value.is_null()) {
                        if shape.need(Some(completeness), &format!("{at}.completeness"), Kind::Array) {
                            for (item, finding) in completeness.as_array().into_iter().flatten().enumerate() {
                                let finding_at = format!("{at}.completeness[{item}]");
                                if !shape.need(Some(finding), &finding_at, Kind::Object) {
                                    continue;
                                }
                                shape.need(finding.get("diagnostic_code"), &format!("{finding_at}.diagnostic_code"), Kind::String);
                                shape.need(finding.get("status"), &format!("{finding_at}.status"), Kind::String);
                            }
                        }
                    }
                }
                "diagnostics" => {
                    if let Some(id) = record.get("id").filter(|value| !value.is_null()) {
                        shape.need(Some(id), &format!("{at}.id"), Kind::String);
                    }
                }
                "supports" => {
                    if shape.need(record.get("restraints"), &format!("{at}.restraints"), Kind::Array) {
                        for (item, value) in record["restraints"].as_array().into_iter().flatten().enumerate() {
                            shape.need(Some(value), &format!("{at}.restraints[{item}]"), Kind::String);
                        }
                    }
                }
                "combinations" => {
                    // A term names exactly one operand, a load case or (model
                    // document 0.9.0) a combination, as a string. A subtraction or
                    // range envelope may omit `terms`; every other basis carries it.
                    let terms_optional = record.get("terms").is_none() && record.get("basis").and_then(Value::as_str).is_some_and(|basis| TERMLESS_BASES.contains(&basis));
                    if !terms_optional && shape.need(record.get("terms"), &format!("{at}.terms"), Kind::Array) {
                        for (item, term) in record["terms"].as_array().into_iter().flatten().enumerate() {
                            let term_at = format!("{at}.terms[{item}]");
                            if !shape.need(Some(term), &term_at, Kind::Object) {
                                continue;
                            }
                            let named: Vec<&str> = ["load_case", "combination"].into_iter().filter(|key| term.get(*key).is_some()).collect();
                            match named.len() {
                                0 => shape.0.push(ShapeDiagnostic {
                                    path: term_at.clone(),
                                    message: format!("{term_at} names neither load_case nor combination (exactly one string required)."),
                                }),
                                2 => shape.0.push(ShapeDiagnostic {
                                    path: term_at.clone(),
                                    message: format!("{term_at} names both load_case and combination (exactly one required)."),
                                }),
                                _ => {}
                            }
                            for key in named {
                                shape.need(term.get(key), &format!("{term_at}.{key}"), Kind::String);
                            }
                            shape.need(term.get("factor"), &format!("{term_at}.factor"), Kind::Number);
                        }
                    }
                    // The subtraction and envelope operands: a present, non-null
                    // member is a string, or an array of them.
                    for key in ["minuend_id", "subtrahend_id", "mode"] {
                        if let Some(value) = record.get(key).filter(|value| !value.is_null()) {
                            shape.need(Some(value), &format!("{at}.{key}"), Kind::String);
                        }
                    }
                    if let Some(operands) = record.get("operand_ids").filter(|value| !value.is_null()) {
                        if shape.need(Some(operands), &format!("{at}.operand_ids"), Kind::Array) {
                            for (item, value) in operands.as_array().into_iter().flatten().enumerate() {
                                shape.need(Some(value), &format!("{at}.operand_ids[{item}]"), Kind::String);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    shape.0
}

/// A case of the shared shape corpus as a document: a copy of the corpus's
/// schema-valid `base` with the case's `edits` applied (`value` set at `path`,
/// or the member removed when `remove` is true), or the case's `replace`.
#[cfg(test)]
pub(crate) fn corpus_case_document(corpus: &Value, case: &Value) -> Value {
    if let Some(replace) = case.get("replace") {
        return replace.clone();
    }
    let mut document = corpus["base"].clone();
    for edit in case["edits"].as_array().expect("edits") {
        let path = edit["path"].as_array().expect("path");
        let (last, parents) = path.split_last().expect("non-empty path");
        let mut target = &mut document;
        for step in parents {
            target = match step {
                Value::String(key) => &mut target[key.as_str()],
                Value::Number(index) => &mut target[index.as_u64().expect("index") as usize],
                other => panic!("path step {other}"),
            };
        }
        match (last, edit.get("remove").and_then(Value::as_bool).unwrap_or(false)) {
            (Value::String(key), true) => {
                target.as_object_mut().expect("object").remove(key);
            }
            (Value::String(key), false) => target[key.as_str()] = edit["value"].clone(),
            (Value::Number(index), false) => target[index.as_u64().expect("index") as usize] = edit["value"].clone(),
            (other, _) => panic!("path end {other}"),
        }
    }
    document
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &str = include_str!("../../../../fixtures/model_document/desktop_shape_cases.json");
    const DEMO: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");
    const EVERY_SLOT: &str = include_str!("../../../../fixtures/model_document/invented_every_slot_model.json");

    fn sorted(mut findings: Vec<(String, String, String)>) -> Vec<(String, String, String)> {
        findings.sort();
        findings
    }

    /// The shared corpus: the same verdict, finding for finding, as the webview.
    #[test]
    fn the_shared_corpus_gives_the_webviews_findings() {
        let corpus: Value = serde_json::from_str(CASES).unwrap();
        let cases = corpus["cases"].as_array().unwrap();
        assert!(cases.len() >= 10);
        assert!(corpus.get("base").is_some_and(Value::is_object));
        for case in cases {
            let id = case["case_id"].as_str().unwrap();
            let seen = sorted(
                model_document_shape_diagnostics(&corpus_case_document(&corpus, case))
                    .into_iter()
                    .map(|item| (MODEL_FILE_SHAPE.to_string(), item.path, item.message))
                    .collect(),
            );
            let want = sorted(
                case["expected"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|item| {
                        (item["code"].as_str().unwrap().to_string(), item["path"].as_str().unwrap().to_string(), item["message"].as_str().unwrap().to_string())
                    })
                    .collect(),
            );
            assert_eq!(seen, want, "{id}");
        }
    }

    #[test]
    fn the_documents_the_desktop_bundles_pass() {
        for text in [DEMO, EVERY_SLOT] {
            assert_eq!(model_document_shape_diagnostics(&serde_json::from_str(text).unwrap()), vec![]);
        }
    }
}
