//! The bounded JSON Schema evaluator against its committed behavioural vectors
//! (`fixtures/results/schema_eval_vectors.json`) and the closure's keyword
//! census (DEL-08-01 Design `report-package-current-results.md` §2.3 item 1).
//! The same vectors are checked against the independent Python Draft 2020-12
//! oracle by `tests/test_schema_eval_parity.py`. Parity is regression
//! evidence, not a claim of general-validator equivalence.

use open_pipe_stress_report_package::schema_eval::{
    keyword_census, Catalog, SchemaFailure, ANNOTATION_KEYWORDS, COMMENT_KEYWORDS, CORE_KEYWORDS,
    EVALUATED_KEYWORDS, RESULTS_V03_CLOSURE,
};
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::Path;

const VECTORS: &str = include_str!("../../../../fixtures/results/schema_eval_vectors.json");
/// The real documents the vectors read at run time, each named as one literal
/// so CI's fixture scan selects this test when one changes
/// (`docs/CI_STRATEGY.md`).
const INSTANCE_FILES: [&str; 3] = [
    "validation/witness/inputs/del1005_export_results_success_input.json",
    "fixtures/results/pressure_plain_connected_sparse.document.json",
    "fixtures/results/retained_precision_successor_derivative_sparse_interactive.json",
];

fn project(path: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(path)
}

fn pointer<'a>(value: &'a Value, pointer: &str) -> &'a Value {
    if pointer.is_empty() { value } else { value.pointer(pointer).unwrap_or_else(|| panic!("{pointer}")) }
}

fn patch(mut document: Value, operations: &[Value]) -> Value {
    for operation in operations {
        let path = operation["path"].as_str().unwrap();
        let (parent, last) = path.rsplit_once('/').unwrap();
        let target = if parent.is_empty() { &mut document } else { document.pointer_mut(parent).unwrap() };
        match (operation["op"].as_str().unwrap(), target) {
            ("remove", Value::Object(map)) => {
                map.remove(last).unwrap();
            }
            ("remove", Value::Array(items)) => {
                items.remove(last.parse().unwrap());
            }
            (_, Value::Array(items)) if last == "-" => items.push(operation["value"].clone()),
            (_, Value::Array(items)) => items[last.parse::<usize>().unwrap()] = operation["value"].clone(),
            (_, Value::Object(map)) => {
                map.insert(last.to_string(), operation["value"].clone());
            }
            _ => panic!("unsupported patch {operation}"),
        }
    }
    document
}

fn instance(case: &Value) -> Value {
    let base = match case["instance_file"].as_str() {
        Some(file) => {
            assert!(INSTANCE_FILES.contains(&file), "{file} is not a declared instance file");
            let text = std::fs::read_to_string(project(file)).unwrap();
            let value: Value = serde_json::from_str(&text).unwrap();
            pointer(&value, case["instance_pointer"].as_str().unwrap_or("")).clone()
        }
        None => case["instance"].clone(),
    };
    patch(base, case["patch"].as_array().map(Vec::as_slice).unwrap_or(&[]))
}

/// The closure plus the case's schema as one more resource.
fn catalog_with(schema: &Value) -> Catalog {
    let mut resources: Vec<(String, Value)> = RESULTS_V03_CLOSURE
        .iter()
        .map(|(name, text)| (name.to_string(), serde_json::from_str(text).unwrap()))
        .collect();
    resources.push(("schema-eval-vector.schema.json".to_string(), schema.clone()));
    Catalog::new(resources)
}

#[test]
fn every_vector_has_its_expected_verdict() {
    let vectors: Value = serde_json::from_str(VECTORS).unwrap();
    let cases = vectors["cases"].as_array().unwrap();
    assert!(cases.len() >= 100);
    let mut ids = BTreeSet::new();
    for case in cases {
        let id = case["id"].as_str().unwrap();
        assert!(ids.insert(id), "duplicate case id {id}");
        let instance = instance(case);
        // The one enumerated oracle-disagreement class: the evaluator gives the
        // ECMA-262 verdict (invalid) for a `pattern` string ending in a line
        // feed, which Python's `re` accepts (DEL-08-01 Design §2.3 item 1).
        // The case is in the class only if its sole failure is the anchored
        // `pattern` at the recorded string, and removing the final line feed
        // makes it valid.
        if let Some(divergence) = case.get("oracle_divergence") {
            assert_eq!(divergence["class"], "ecma_dollar_before_final_line_feed", "{id}");
            assert_eq!(case["valid"], false, "{id}");
            let at = divergence["string_pointer"].as_str().unwrap();
            let text = pointer(&instance, at).as_str().expect("the recorded value is a string").to_string();
            assert!(text.ends_with('\n'), "{id}");
            let catalog = catalog_with(&case["schema"]);
            let failure = catalog.validate("schema-eval-vector.schema.json", &instance);
            let path = if at.is_empty() { "/" } else { at };
            assert_eq!(
                failure,
                Err(SchemaFailure::Invalid { instance_path: path.to_string(), keyword: "pattern".to_string() }),
                "{id}: the only failure is the pattern at the recorded string"
            );
            let mut twin = instance.clone();
            let slot = if at.is_empty() { &mut twin } else { twin.pointer_mut(at).unwrap() };
            *slot = Value::String(text[..text.len() - 1].to_string());
            assert_eq!(catalog.validate("schema-eval-vector.schema.json", &twin), Ok(()), "{id}: without the line feed it is valid");
        }
        let outcome = catalog_with(&case["schema"]).validate("schema-eval-vector.schema.json", &instance);
        match outcome {
            Ok(()) => assert!(case["valid"].as_bool().unwrap(), "{id}: expected invalid, evaluated valid"),
            Err(SchemaFailure::Invalid { .. }) => {
                assert!(!case["valid"].as_bool().unwrap(), "{id}: expected valid, evaluated invalid ({outcome:?})")
            }
            Err(failure @ SchemaFailure::Unsupported { .. }) => panic!("{id}: {failure}"),
        }
    }
}

/// Each evaluated keyword has at least one valid and one invalid vector, and
/// every recorded form of the closure is covered (round-4 review S1).
#[test]
fn the_vectors_cover_every_evaluated_keyword_and_form() {
    let vectors: Value = serde_json::from_str(VECTORS).unwrap();
    let mut seen = BTreeSet::new();
    for case in vectors["cases"].as_array().unwrap() {
        for keyword in case["covers"].as_array().unwrap() {
            seen.insert((keyword.as_str().unwrap().to_string(), case["valid"].as_bool().unwrap()));
        }
    }
    for keyword in EVALUATED_KEYWORDS.iter().chain(["$ref"].iter()) {
        for verdict in [true, false] {
            assert!(seen.contains(&(keyword.to_string(), verdict)), "{keyword} has no {verdict} case");
        }
    }
    for form in [
        "integer", "nullable", "six-type-union", "numeric-equality", "boolean-number-distinct", "structured-equality",
        "boundary", "hex16", "hex64", "final-line-feed", "applicability", "required-without-properties",
        "empty-array", "fixed-array", "bounded-array", "unconstrained-object", "unconstrained-array", "closed-object",
        "parent-constraints", "local-pointer", "whole-resource", "external-fragment", "resource-base", "boolean-schema",
        "annotation", "comment", "unicode", "document", "header-only",
    ] {
        assert!(seen.iter().any(|(name, _)| name == form), "form {form} has no case");
    }
}

/// The census of the five embedded resources: exactly the 32 names the
/// round-4 review counted, each classified, and nothing else.
#[test]
fn the_closure_keyword_census_is_classified_and_complete() {
    let mut names = BTreeSet::new();
    for (_, text) in RESULTS_V03_CLOSURE {
        keyword_census(&serde_json::from_str(text).unwrap(), &mut names);
    }
    let classified: BTreeSet<String> = CORE_KEYWORDS
        .iter()
        .chain(ANNOTATION_KEYWORDS.iter())
        .chain(COMMENT_KEYWORDS.iter())
        .chain(EVALUATED_KEYWORDS.iter())
        .map(|name| name.to_string())
        .collect();
    assert_eq!(classified.len(), 32);
    assert_eq!(names, classified);
    let names: Vec<&str> = RESULTS_V03_CLOSURE.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names,
        [
            "results.v0.3.schema.yaml",
            "results_pressure_2.v0.3.schema.json",
            "physics_source_recovery.schema.json",
            "retained_precision_mp_v2.schema.json",
            "load_reference_state.schema.json"
        ]
    );
}

/// Every reference in the closure resolves within the catalog.
#[test]
fn every_closure_reference_resolves() {
    fn refs(value: &Value, out: &mut Vec<String>) {
        match value {
            Value::Object(map) => {
                for (key, item) in map {
                    if key == "$ref" {
                        out.push(item.as_str().unwrap().to_string());
                    }
                    refs(item, out);
                }
            }
            Value::Array(items) => items.iter().for_each(|item| refs(item, out)),
            _ => {}
        }
    }
    let catalog = Catalog::results_v03();
    for (name, schema) in catalog.resources() {
        let mut found = Vec::new();
        refs(schema, &mut found);
        for reference in found {
            let (resource, fragment) = reference.split_once('#').unwrap_or((reference.as_str(), ""));
            let resource = if resource.is_empty() { name } else { resource.rsplit('/').next().unwrap() };
            let target = catalog.resources().find(|(own, _)| *own == resource).map(|(_, value)| value);
            assert!(target.and_then(|value| value.pointer(fragment)).is_some(), "{name}: {reference}");
        }
    }
}

#[test]
fn numeric_equality_does_not_round_an_integer_before_comparison() {
    use open_pipe_stress_report_package::schema_eval::json_equal;
    let integer: Value = serde_json::from_str("9007199254740993").unwrap();
    let rounded: Value = serde_json::from_str("9007199254740992.0").unwrap();
    assert!(!json_equal(&integer, &rounded));
}
