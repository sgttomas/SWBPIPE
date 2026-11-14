//! DEL-08-01 AC-016 and VER-016: every tag of the numerical vocabulary has
//! exactly one reviewed statement, and every statement keys a tag.
//!
//! - The renderer maps each tag to its statement with a match that has no
//!   wildcard arm (`report_renderer::standing`), so a tag without a statement
//!   does not compile.
//! - The generated catalogue (`fixtures/reports/numerical_vocabulary.json`) is
//!   written from the vocabulary crate's lists, never by scanning source, and
//!   the generated statement table (`fixtures/reports/standing_statements.json`)
//!   from the renderer's statements. This test compares the two in both
//!   directions and holds each committed file current; regenerate both with
//!   `OPENPIPESTRESS_WRITE_STANDING_STATEMENTS=1`.
use open_pipe_stress_numerical_vocabulary::catalogue;
use open_pipe_stress_report_renderer::standing::statement_table;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::path::PathBuf;

const WRITE: &str = "OPENPIPESTRESS_WRITE_STANDING_STATEMENTS";

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/reports").join(name)
}

fn catalogue_document() -> Value {
    json!({
        "document_kind": "swbpipe.numerical_vocabulary_catalogue",
        "version": 1,
        "generator": "core/reporting/report_renderer/tests/standing_statements.rs",
        "tags": catalogue().iter().map(|e| json!({"vocabulary": e.vocabulary, "tag": e.tag})).collect::<Vec<_>>(),
    })
}

fn statement_document() -> Value {
    json!({
        "document_kind": "swbpipe.standing_statements",
        "version": 1,
        "generator": "core/reporting/report_renderer/tests/standing_statements.rs",
        "statements": statement_table().into_iter().map(|(vocabulary, tag, statement)| json!({"vocabulary": vocabulary, "tag": tag, "statement": statement})).collect::<Vec<_>>(),
    })
}

fn text(value: &Value) -> String {
    format!("{}\n", serde_json::to_string_pretty(value).unwrap())
}

#[test]
fn every_tag_has_exactly_one_statement_and_every_statement_keys_a_tag() {
    let tags: Vec<(&str, &str)> = catalogue().iter().map(|e| (e.vocabulary, e.tag)).collect();
    let keyed: Vec<(&str, &str)> = statement_table().iter().map(|(v, t, _)| (*v, *t)).collect();
    let tag_set: BTreeSet<_> = tags.iter().copied().collect();
    let keyed_set: BTreeSet<_> = keyed.iter().copied().collect();
    assert_eq!(tag_set.len(), tags.len(), "a tag is listed twice");
    assert_eq!(keyed_set.len(), keyed.len(), "a tag has more than one statement");
    let missing: Vec<_> = tag_set.difference(&keyed_set).collect();
    let orphaned: Vec<_> = keyed_set.difference(&tag_set).collect();
    assert!(missing.is_empty(), "tags without a statement: {missing:?}");
    assert!(orphaned.is_empty(), "statements keying no tag: {orphaned:?}");
    for (vocabulary, tag, statement) in statement_table() {
        assert!(!statement.trim().is_empty(), "{vocabulary}.{tag}: empty statement");
    }
}

#[test]
fn the_generated_catalogue_and_statement_table_are_current() {
    let files = [("numerical_vocabulary.json", catalogue_document()), ("standing_statements.json", statement_document())];
    if std::env::var(WRITE).is_ok() {
        for (name, document) in &files {
            std::fs::write(fixture(name), text(document)).unwrap();
        }
        return;
    }
    for (name, document) in &files {
        let committed = std::fs::read_to_string(fixture(name)).unwrap_or_default();
        assert!(committed == text(document), "fixtures/reports/{name} is stale; regenerate with {WRITE}=1");
    }
}

/// The committed files agree with each other in both directions, as a reader
/// of the files alone would check them.
#[test]
fn the_committed_catalogue_and_statement_table_agree() {
    let read = |name: &str| -> Value { serde_json::from_str(&std::fs::read_to_string(fixture(name)).unwrap()).unwrap() };
    let pairs = |document: &Value, member: &str| -> BTreeSet<(String, String)> {
        document[member]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| (e["vocabulary"].as_str().unwrap().to_string(), e["tag"].as_str().unwrap().to_string()))
            .collect()
    };
    assert_eq!(pairs(&read("numerical_vocabulary.json"), "tags"), pairs(&read("standing_statements.json"), "statements"));
}
