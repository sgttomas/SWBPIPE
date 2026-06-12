//! Cross-engine operation contract corpus runner (TP-SEAM-CORPUS-001).
//!
//! Executes every case file in `fixtures/model_operations/contract_corpus/`
//! through this crate's `validate_operation` / `apply_operation` seam and
//! asserts the recorded expected outcome. The same case files are executed by
//! the browser-mirror engine in
//! `apps/desktop/src/services/operationContractCorpus.test.ts` under the same
//! comparison rules, so the two engines are held to identical semantic
//! outcomes and to one shared canonical hash of the applied model document.
//!
//! Comparison rules (mirrored exactly in the Vitest runner; documented in the
//! corpus README):
//! - exact equality on semantic fields (validation states, diff rows,
//!   diagnostics as code/severity/blocking/affected_refs records compared
//!   order-insensitively, acceptance, audit/professional boundary, model-basis
//!   claim echo fields, identity echo fields);
//! - engine-identity fields excluded only via the documented allowlist
//!   (`application_route`, `diagnostics[].id`, `diagnostics[].source`,
//!   `diagnostics[].message`, `diagnostics[].remediation`). Tightened at
//!   TP-H1-HASHUNIFY-001 (`backend_canonicalization` label compared, engine
//!   self-consistency asserted) and again at H5: the engine canonicalization
//!   is RFC 8785 (JCS), so `model_basis.backend_model_hash` and
//!   `applied_model_backend_hash` VALUES are now full byte-equality
//!   comparisons across every lane — JS transport (`200.0` → `200`) no
//!   longer shifts the canonical bytes, which is what kept those values
//!   excluded between TP-H1-HASHUNIFY-001 and H5;
//! - applied model documents compared BOTH as parsed-JSON deep equality
//!   (numbers compared by f64 value) AND by the engine's own canonical hash
//!   (`canonical_json` + `sha256_hex`, pinned independently by
//!   `fixtures/canonical_hash/`). The ECMA harness renderers that previously
//!   approximated a transport-invariant hash (Rust + TS twins) are retired —
//!   the engine rendering IS transport-invariant now.
//!
//! Regeneration: expected outcomes are generated from THIS crate (the
//! contract reference) by running `CORPUS_BLESS=1 cargo test` in this crate
//! directory. Blessed output requires human review before being relied on.
//!
//! Boundary: all corpus data is invented (PUBLIC_DOMAIN_OR_ORIGINAL); no
//! protected standards content; no compliance/certification/approval claims.

use open_pipe_stress_operation_applier::{
    apply_operation, canonical_json, sha256_hex, validate_operation, OperationOutcome,
};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const REQUIRED_ACCEPTED_KINDS: [&str; 18] = [
    "set_field",
    "update_load",
    "update_support",
    "create_node",
    "connect_pipe_run",
    "delete_pipe_run",
    "create_section",
    "create_material",
    "create_support",
    "create_load_case",
    "delete_load_case",
    "create_primitive_load",
    "delete_primitive_load",
    "create_combination",
    "delete_combination",
    "create_combination_term",
    "delete_support",
    "delete_combination_term",
];

const REQUIRED_BLOCK_CODES: [&str; 13] = [
    "OP-TARGET-ALREADY-EXISTS",                // duplicate id
    "OP-TARGET-NOT-FOUND",                     // missing target
    "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE", // unit mismatch
    "OP-UNIT-DIMENSION-UNKNOWN",               // invalid dimension
    "OP-STALE-BEFORE-VALUE",                   // stale before-value
    "OP-FIELD-EDIT-DEFERRED",                  // deferred field
    "OP-FIELD-PATH-UNSUPPORTED",               // unsupported field path
    "OP-VALUE-NOT-NUMERIC",                    // non-finite magnitude
    "OP-VALUE-EMPTY",                          // empty required text
    "OP-UNIT-DIMENSION-MISMATCH",              // dynamic terms.N.factor block
    "OP-SUPPORT-DELETE-REFERENCED",            // support still referenced by primitive load
    "OP-LOAD-CASE-DELETE-REFERENCED",          // load case still referenced by combination
    "OP-PIPE-DELETE-REFERENCED",               // pipe still referenced by primitive load
];

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/model_operations/contract_corpus")
}

fn case_paths() -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = fs::read_dir(corpus_dir())
        .expect("contract corpus directory must exist")
        .filter_map(|entry| entry.ok().map(|item| item.path()))
        .filter(|path| {
            path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("json")
        })
        .collect();
    paths.sort();
    assert!(
        !paths.is_empty(),
        "contract corpus must contain at least one case file"
    );
    paths
}

fn load_case(path: &Path) -> Value {
    let raw = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("case file {} must be readable: {error}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|error| panic!("case file {} must be valid JSON: {error}", path.display()))
}

/// The engine's own canonical hash of a document (`canonical_json` +
/// `sha256_hex`). Since H5 the engine canonicalization is RFC 8785 (JCS),
/// which is transport-invariant — identical values hash identically whether
/// they arrived as raw case-file text or through a JS `JSON.stringify` lane —
/// so this single formula serves both the blessed-value comparison and the
/// engine self-consistency assertion. The previous harness-only ECMA
/// renderer pair (Rust + TS twins, with their plain-decimal range
/// constraint) is retired; the engine rendering replaced it.
fn engine_canonical_hash(value: &Value) -> String {
    format!("sha256:{}", sha256_hex(&canonical_json(value)))
}

/// Parsed-JSON deep equality with numbers compared by f64 value, so that an
/// integer literal and the same value as a float literal compare equal (the
/// canonical hash already renders them identically).
fn json_semantically_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => match (l.as_f64(), r.as_f64()) {
            (Some(lv), Some(rv)) => lv == rv,
            _ => l == r,
        },
        (Value::Array(l), Value::Array(r)) => {
            l.len() == r.len()
                && l.iter()
                    .zip(r.iter())
                    .all(|(li, ri)| json_semantically_equal(li, ri))
        }
        (Value::Object(l), Value::Object(r)) => {
            l.len() == r.len()
                && l.iter()
                    .all(|(key, lv)| r.get(key).is_some_and(|rv| json_semantically_equal(lv, rv)))
        }
        _ => left == right,
    }
}

fn diagnostic_sort_key(item: &Value) -> String {
    let refs = item["affected_refs"]
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_default();
    format!(
        "{}\u{0}{}\u{0}{}",
        item["code"].as_str().unwrap_or_default(),
        refs,
        item["severity"].as_str().unwrap_or_default()
    )
}

/// Projects the engine outcome onto the cross-engine semantic comparison
/// surface. Mirrored exactly by `semanticOutcome` in the Vitest runner.
fn semantic_outcome(outcome: &OperationOutcome) -> Value {
    let raw = serde_json::to_value(outcome).expect("outcome must encode as JSON");
    let mut diagnostics: Vec<Value> = raw["diagnostics"]
        .as_array()
        .expect("outcome diagnostics must be an array")
        .iter()
        .map(|item| {
            json!({
                "code": item["code"],
                "severity": item["severity"],
                "blocking": item["severity"] == "blocking",
                "affected_refs": item["affected_refs"],
            })
        })
        .collect();
    diagnostics.sort_by_key(diagnostic_sort_key);
    json!({
        "mode": raw["mode"],
        "operation_id": raw["operation_id"],
        "change_id": raw["change_id"],
        "operation_kind": raw["operation_kind"],
        "change_kind": raw["change_kind"],
        "target_object_type": raw["target_object_type"],
        "target_ref": raw["target_ref"],
        "schema_version": raw["schema_version"],
        "document_kind": raw["document_kind"],
        "deliverable_refs": raw["deliverable_refs"],
        "validation": raw["validation"],
        "diff_preview": raw["diff_preview"],
        "diagnostics": diagnostics,
        "model_basis": {
            "claimed_model_hash": raw["model_basis"]["claimed_model_hash"],
            "claimed_hash_canonicalization": raw["model_basis"]["claimed_hash_canonicalization"],
            "binding_status": raw["model_basis"]["binding_status"],
            // Label compared since TP-H1-HASHUNIFY-001; the VALUE joined the
            // compared surface at H5 — RFC 8785 rendering made input-text
            // hashes transport-invariant, retiring the measured exclusion.
            "backend_canonicalization": raw["model_basis"]["backend_canonicalization"],
            "backend_model_hash": raw["model_basis"]["backend_model_hash"],
        },
        "input_model_unchanged": raw["input_model_unchanged"],
        "acceptance": raw["acceptance"],
        "audit_boundary": raw["audit_boundary"],
        "professional_boundary": raw["professional_boundary"],
        "applied": !raw["applied_model"].is_null(),
    })
}

fn run_case(case: &Value, case_id: &str) -> OperationOutcome {
    let base_model = &case["base_model"];
    let intent = &case["intent"];
    let claimed = &case["claimed_model_hash"];
    let claimed_ref = if claimed.is_null() {
        None
    } else {
        Some(claimed)
    };
    let snapshot = base_model.clone();
    let outcome = match case["mode"].as_str() {
        Some("validate_only") => validate_operation(base_model, intent, claimed_ref),
        Some("apply") => apply_operation(base_model, intent, claimed_ref),
        other => panic!("case {case_id}: unsupported mode {other:?}"),
    };
    assert_eq!(
        *base_model, snapshot,
        "case {case_id}: the input model document must never be mutated"
    );
    outcome
}

#[test]
fn contract_corpus_cases_match_the_rust_contract_reference() {
    let bless = std::env::var("CORPUS_BLESS").is_ok();
    for path in case_paths() {
        let mut case = load_case(&path);
        let case_id = case["case_id"]
            .as_str()
            .unwrap_or_else(|| panic!("case file {} must carry case_id", path.display()))
            .to_string();
        let outcome = run_case(&case, &case_id);
        let projected = semantic_outcome(&outcome);
        let applied_hash = outcome.applied_model.as_ref().map(engine_canonical_hash);

        if bless {
            case["expected"] = json!({
                "outcome": projected,
                "applied_model": outcome.applied_model.clone(),
                "applied_model_canonical_sha256": applied_hash,
            });
            let serialized =
                serde_json::to_string_pretty(&case).expect("case must re-encode as JSON");
            fs::write(&path, serialized + "\n").unwrap_or_else(|error| {
                panic!(
                    "case file {} must be writable in bless mode: {error}",
                    path.display()
                )
            });
            continue;
        }

        let expected = &case["expected"];
        assert!(
            !expected.is_null(),
            "case {case_id}: expected outcome missing — regenerate with CORPUS_BLESS=1 cargo test and review the diff"
        );
        assert_eq!(
            projected, expected["outcome"],
            "case {case_id}: semantic outcome does not match the recorded contract expectation"
        );
        match &outcome.applied_model {
            Some(applied) => {
                assert!(
                    json_semantically_equal(applied, &expected["applied_model"]),
                    "case {case_id}: applied model document does not deep-equal the expected applied document"
                );
                assert_eq!(
                    applied_hash.as_deref(),
                    expected["applied_model_canonical_sha256"].as_str(),
                    "case {case_id}: engine canonical hash of the applied model does not match the recorded hash"
                );
                assert_eq!(
                    outcome.applied_model_backend_hash.as_deref(),
                    expected["applied_model_canonical_sha256"].as_str(),
                    "case {case_id}: the engine's applied_model_backend_hash must byte-equal the blessed canonical hash in every lane (H5 full byte-equality)"
                );
            }
            None => {
                assert!(
                    expected["applied_model"].is_null(),
                    "case {case_id}: engine blocked the apply but the expectation records an applied model"
                );
                assert!(
                    expected["applied_model_canonical_sha256"].is_null(),
                    "case {case_id}: blocked applies must not record an applied-model hash"
                );
                assert!(
                    outcome.applied_model_backend_hash.is_none(),
                    "case {case_id}: blocked applies must not carry an applied-model backend hash"
                );
            }
        }
    }
}

#[test]
fn contract_corpus_covers_the_required_kinds_and_block_classes() {
    if std::env::var("CORPUS_BLESS").is_ok() {
        // Bless mode rewrites the case files in the sibling test; coverage is
        // evaluated on the committed expectations in a normal run.
        return;
    }
    let mut accepted_kinds: BTreeSet<String> = BTreeSet::new();
    let mut blocked_codes: BTreeSet<String> = BTreeSet::new();
    let mut accepted_dynamic_paths: BTreeSet<&'static str> = BTreeSet::new();
    let mut blocked_dynamic_paths: BTreeSet<&'static str> = BTreeSet::new();

    for path in case_paths() {
        let case = load_case(&path);
        let case_id = case["case_id"].as_str().unwrap_or_default().to_string();
        let expected = &case["expected"];
        assert!(
            !expected.is_null(),
            "case {case_id}: expected outcome missing — coverage cannot be evaluated"
        );
        let change_kind = case["intent"]["change"]["change_kind"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let field_path = case["intent"]["change"]["field_path"]
            .as_str()
            .unwrap_or_default();
        let accepted =
            expected["outcome"]["validation"]["application_status"] == "applied_to_session_model";
        let is_primitive_magnitude = field_path
            .strip_prefix("primitive_loads.")
            .and_then(|rest| rest.split_once('.'))
            .is_some_and(|(index, tail)| {
                !index.is_empty()
                    && index.chars().all(|ch| ch.is_ascii_digit())
                    && tail == "magnitude.value"
            });
        let is_term_factor = field_path
            .strip_prefix("terms.")
            .and_then(|rest| rest.split_once('.'))
            .is_some_and(|(index, tail)| {
                !index.is_empty() && index.chars().all(|ch| ch.is_ascii_digit()) && tail == "factor"
            });
        if accepted {
            accepted_kinds.insert(change_kind);
            if is_primitive_magnitude {
                accepted_dynamic_paths.insert("primitive_loads.N.magnitude.value");
            }
            if is_term_factor {
                accepted_dynamic_paths.insert("terms.N.factor");
            }
        } else {
            if is_primitive_magnitude {
                blocked_dynamic_paths.insert("primitive_loads.N.magnitude.value");
            }
            if is_term_factor {
                blocked_dynamic_paths.insert("terms.N.factor");
            }
        }
        if let Some(diagnostics) = expected["outcome"]["diagnostics"].as_array() {
            for item in diagnostics {
                if item["blocking"] == true {
                    blocked_codes.insert(item["code"].as_str().unwrap_or_default().to_string());
                }
            }
        }
    }

    for kind in REQUIRED_ACCEPTED_KINDS {
        assert!(
            accepted_kinds.contains(kind),
            "coverage floor violated: no accepted-apply corpus case for operation kind `{kind}`"
        );
    }
    for code in REQUIRED_BLOCK_CODES {
        assert!(
            blocked_codes.contains(code),
            "coverage floor violated: no corpus case produces blocking diagnostic `{code}`"
        );
    }
    for dynamic_path in ["primitive_loads.N.magnitude.value", "terms.N.factor"] {
        assert!(
            accepted_dynamic_paths.contains(dynamic_path),
            "coverage floor violated: no accepted-apply corpus case edits dynamic path `{dynamic_path}`"
        );
        assert!(
            blocked_dynamic_paths.contains(dynamic_path),
            "coverage floor violated: no blocked corpus case targets dynamic path `{dynamic_path}`"
        );
    }
}
