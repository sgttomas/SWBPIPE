//! Model document 0.7.0 (DEL-02-01 Design `model-document-0.7.0.md` §2, §3): the
//! annotation and record slots are model data that no solve reads. The invented
//! annotated document (`fixtures/model_document/invented_annotated_model.json`,
//! which carries every new member) solves end to end through the public value
//! entry the desktop and the runner use, in both solver modes, and its envelope
//! is byte-identical to that of the same document with the new members removed.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};

const ANNOTATED: &str = include_str!("../../../fixtures/model_document/invented_annotated_model.json");

/// The top-level members 0.7.0 adds.
const DOCUMENT_MEMBERS: [&str; 5] = ["lines", "unresolved_assumptions", "rule_pack_refs", "constraints", "equipment_interfaces"];
/// Collections whose records may carry `tags` and `notes`.
const ANNOTATED_COLLECTIONS: [&str; 8] =
    ["nodes", "pipe_segments", "sections", "materials", "supports", "components", "load_cases", "combinations"];
/// Collections whose records may carry `provenance_record`.
const PROVENANCE_COLLECTIONS: [&str; 4] = ["materials", "sections", "components", "supports"];

/// The same document without any 0.7.0 member: the new top-level members, the
/// `model_settings` record (§3.8; its node-proximity tolerance finds no pair in this
/// fixture with or without it), every record's `tags` and
/// `notes`, the project's, and every `provenance_record`.
fn without_new_members(document: &Value) -> (Value, usize) {
    let mut stripped = document.clone();
    let mut removed = 0;
    let root = stripped.as_object_mut().expect("model document object");
    for member in DOCUMENT_MEMBERS {
        removed += usize::from(root.remove(member).is_some());
    }
    removed += usize::from(root.remove("model_settings").is_some());
    let project = root.get_mut("project").and_then(Value::as_object_mut).expect("project object");
    for member in ["tags", "notes"] {
        removed += usize::from(project.remove(member).is_some());
    }
    for collection in ANNOTATED_COLLECTIONS {
        for record in root.get_mut(collection).and_then(Value::as_array_mut).into_iter().flatten() {
            let record = record.as_object_mut().expect("record object");
            for member in ["tags", "notes"] {
                removed += usize::from(record.remove(member).is_some());
            }
            if PROVENANCE_COLLECTIONS.contains(&collection) {
                removed += usize::from(record.remove("provenance_record").is_some());
            }
        }
    }
    (stripped, removed)
}

#[test]
fn the_annotated_document_solves_exactly_as_the_document_without_its_new_members() {
    let annotated: Value = serde_json::from_str(ANNOTATED).expect("fixture parses");
    assert_eq!(annotated["schema_version"], json!(MODEL_SCHEMA_VERSION));
    for member in DOCUMENT_MEMBERS {
        assert!(annotated[member].as_array().is_some_and(|items| !items.is_empty()), "the fixture carries {member}");
    }
    assert!(annotated["model_settings"]["node_proximity_tolerance"].is_object(), "the fixture carries model_settings");
    let (plain, removed) = without_new_members(&annotated);
    assert!(removed >= DOCUMENT_MEMBERS.len() + 1 + 2 + 2 * ANNOTATED_COLLECTIONS.len() + PROVENANCE_COLLECTIONS.len(),
        "every slot kind is exercised ({removed} members removed)");
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let solved = serde_json::to_value(
            run_linear_static_preview_value_with_mode(json!({"model": annotated, "materials": []}), mode)
                .expect("the annotated document is admitted"),
        )
        .unwrap();
        let reference = serde_json::to_value(
            run_linear_static_preview_value_with_mode(json!({"model": plain, "materials": []}), mode)
                .expect("the document without the new members is admitted"),
        )
        .unwrap();
        assert_eq!(solved["status"]["mechanics"], json!("MECHANICS_SOLVED"), "{mode:?}: {}", solved["diagnostics"]);
        assert!(solved["results"].as_array().is_some_and(|rows| !rows.is_empty()), "{mode:?}: result rows");
        assert_eq!(
            serde_json::to_string(&solved).unwrap(),
            serde_json::to_string(&reference).unwrap(),
            "{mode:?}: the new members change no published byte"
        );
    }
}
