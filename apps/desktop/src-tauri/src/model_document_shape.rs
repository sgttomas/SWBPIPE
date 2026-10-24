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
        shape.need(project.get("units"), "project.units", Kind::Object);
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
                }
                "supports" => {
                    if shape.need(record.get("restraints"), &format!("{at}.restraints"), Kind::Array) {
                        for (item, value) in record["restraints"].as_array().into_iter().flatten().enumerate() {
                            shape.need(Some(value), &format!("{at}.restraints[{item}]"), Kind::String);
                        }
                    }
                }
                "combinations" => {
                    if shape.need(record.get("terms"), &format!("{at}.terms"), Kind::Array) {
                        for (item, term) in record["terms"].as_array().into_iter().flatten().enumerate() {
                            let term_at = format!("{at}.terms[{item}]");
                            if !shape.need(Some(term), &term_at, Kind::Object) {
                                continue;
                            }
                            shape.need(term.get("load_case"), &format!("{term_at}.load_case"), Kind::String);
                            shape.need(term.get("factor"), &format!("{term_at}.factor"), Kind::Number);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    shape.0
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
        for case in cases {
            let id = case["case_id"].as_str().unwrap();
            let seen = sorted(
                model_document_shape_diagnostics(&case["document"])
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
