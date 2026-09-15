//! Canonical-hash parity corpus (H1 / verification F-5a; RFC 8785 since H5).
//!
//! `fixtures/canonical_hash/cases.json` pins the engine's canonical JSON
//! form — RFC 8785 (JCS): UTF-16-sorted keys, ECMAScript number rendering —
//! and its SHA-256 for invented inputs. This native test is the blessing
//! authority: run `CANONICAL_HASH_BLESS=1 cargo test` in this crate to
//! regenerate the expectations, then review the diff. The desktop Vitest
//! suite replays the same cases through the wasm exports
//! (`canonical_json_string` / `canonical_sha256_hex`), proving the
//! native and wasm lanes agree byte-for-byte on the same input text.
//!
//! Cases carry their input as a raw JSON *string* so both lanes parse the
//! identical text — number rendering (`1e9` → `1000000000`, `200.0` →
//! `200`) is part of the pinned behavior, so feeding pre-parsed values
//! would erase what the corpus exists to pin.

use open_pipe_stress_operation_applier::{canonical_json, sha256_hex};
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/canonical_hash/cases.json")
}

#[test]
fn checked_ijson_corpus_matches_native_engine() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../fixtures/canonical_hash/cases.json");
    let corpus: Value = serde_json::from_str(&fs::read_to_string(path).expect("checked corpus must exist"))
        .expect("checked corpus must be valid JSON");
    let supported = ["scalar-string-unicode", "number-negative-zero-renders-zero", "number-ecma-notation-boundaries", "number-beyond-2-53-integer-kept-exact"];
    for case in corpus["cases"].as_array().expect("cases").iter().filter(|case| supported.contains(&case["case_id"].as_str().unwrap_or(""))) {
        let case_id = case["case_id"].as_str().expect("case_id");
        let input = case["input_json"].as_str().expect("input_json");
        let actual = canonical_json_checked_v1_text(input);
        if ["number-beyond-2-53-integer-kept-exact", "number-ecma-notation-boundaries"].contains(&case_id) {
            assert!(actual.unwrap_err().contains("UNSAFE"), "case {case_id}");
        } else {
            let canonical = actual.unwrap_or_else(|error| panic!("case {case_id}: {error}"));
            assert_eq!(canonical, case["expected_canonical"].as_str().expect("expected_canonical"), "case {case_id}");
            assert_eq!(sha256_hex(&canonical), case["expected_sha256"].as_str().expect("expected_sha256"), "case {case_id}");
        }
    }
    let canonical = canonical_json_checked_v1_text("[1,1.0]").expect("inline 1/1.0 case");
    assert_eq!(canonical, "[1,1]");
    assert_eq!(sha256_hex(&canonical), "e61b9f584dbe27741cef6e9ee440831d7d94470c0871b0871541f0308916efea");
    let canonical = canonical_json_checked_v1_text("[1e-6,1e-7]").expect("inline safe exponent case");
    assert_eq!(canonical, "[0.000001,1e-7]");
    assert_eq!(sha256_hex(&canonical), "19ca01c5d07894d9ce68294ad32b64d9c2a851c244ae8010e0a2b8a26f3734a0");
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
        cases.len() >= 20,
        "coverage floor violated: the parity corpus must keep at least 20 cases"
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
