//! Canonical-hash parity corpus (H1 / verification F-5a).
//!
//! `fixtures/canonical_hash/cases.json` pins the engine's canonical JSON
//! form and SHA-256 for invented inputs. This native test is the blessing
//! authority: run `CANONICAL_HASH_BLESS=1 cargo test` in this crate to
//! regenerate the expectations, then review the diff. The desktop Vitest
//! suite replays the same cases through the wasm exports
//! (`canonical_json_string` / `canonical_sha256_hex`), proving the
//! native and wasm lanes agree byte-for-byte on the same input text.
//!
//! Cases carry their input as a raw JSON *string* so both lanes parse the
//! identical text — number normalization (`1e9` → `1000000000.0`) is part
//! of the pinned behavior, so feeding pre-parsed values would erase what
//! the corpus exists to pin.

use open_pipe_stress_operation_applier::{canonical_json, sha256_hex};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/canonical_hash/cases.json")
}

#[test]
fn canonical_hash_corpus_matches_native_engine() {
    let path = fixture_path();
    let raw = fs::read_to_string(&path).expect("canonical-hash corpus must exist");
    let mut corpus: Value = serde_json::from_str(&raw).expect("corpus must be valid JSON");
    let bless = std::env::var("CANONICAL_HASH_BLESS").is_ok();

    let cases = corpus["cases"]
        .as_array()
        .expect("corpus.cases must be an array")
        .clone();
    assert!(
        cases.len() >= 12,
        "coverage floor violated: the parity corpus must keep at least 12 cases"
    );

    let mut blessed = Vec::with_capacity(cases.len());
    for case in cases {
        let case_id = case["case_id"].as_str().expect("case_id").to_string();
        let input_json = case["input_json"].as_str().expect("input_json");
        let value: Value = serde_json::from_str(input_json)
            .unwrap_or_else(|error| panic!("case {case_id}: input_json must parse: {error}"));
        let canonical = canonical_json(&value);
        let hash = sha256_hex(&canonical);

        if bless {
            let mut updated = case.clone();
            updated["expected_canonical"] = Value::String(canonical);
            updated["expected_sha256"] = Value::String(hash);
            blessed.push(updated);
        } else {
            assert_eq!(
                case["expected_canonical"].as_str().unwrap_or("TBD"),
                canonical,
                "case {case_id}: canonical form drifted — regenerate with CANONICAL_HASH_BLESS=1 cargo test and review the diff"
            );
            assert_eq!(
                case["expected_sha256"].as_str().unwrap_or("TBD"),
                hash,
                "case {case_id}: canonical sha256 drifted — regenerate with CANONICAL_HASH_BLESS=1 cargo test and review the diff"
            );
        }
    }

    if bless {
        corpus["cases"] = Value::Array(blessed);
        let pretty = serde_json::to_string_pretty(&corpus).expect("corpus re-encodes");
        fs::write(&path, format!("{pretty}\n")).expect("blessed corpus must write");
    }
}
