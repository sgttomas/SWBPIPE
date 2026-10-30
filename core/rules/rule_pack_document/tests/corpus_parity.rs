//! Codec parity against the blessed conformance corpus (TP-C1-GRAMMAR-001):
//! every corpus expression must decode through the production codec and
//! survive an encode -> decode round trip unchanged. Evaluation semantics
//! are pinned by the evaluator crate's own corpus runner, not re-tested
//! here.

use std::fs;
use std::path::PathBuf;

use open_pipe_stress_rule_pack_document::{decode_expression, encode_expression};

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../fixtures/rule_expressions/conformance_corpus")
}

#[test]
fn every_corpus_expression_round_trips_through_the_production_codec() {
    let mut case_paths: Vec<PathBuf> = fs::read_dir(corpus_dir())
        .expect("conformance corpus directory must exist")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("case_") && name.ends_with(".json"))
        })
        .collect();
    case_paths.sort();
    assert!(
        case_paths.len() >= 60,
        "corpus shrank below its enforced floor: {}",
        case_paths.len()
    );

    for path in case_paths {
        let raw = fs::read_to_string(&path).expect("corpus case readable");
        let case: serde_json::Value = serde_json::from_str(&raw).expect("corpus case is JSON");
        let expression_value = case
            .get("expression")
            .unwrap_or_else(|| panic!("{} has no expression member", path.display()));
        let decoded = decode_expression(expression_value).unwrap_or_else(|error| {
            panic!("{} failed to decode: {error}", path.display());
        });
        let encoded = encode_expression(&decoded);
        let round_trip = decode_expression(&encoded).unwrap_or_else(|error| {
            panic!("{} failed round-trip decode: {error}", path.display());
        });
        assert_eq!(
            decoded,
            round_trip,
            "{} round trip diverged",
            path.display()
        );
    }
}
