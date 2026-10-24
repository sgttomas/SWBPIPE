//! The report-row contract's Rust helper against the shared parity vectors
//! (`fixtures/reports/invented/report_result_rows_parity.json`; DEL-08-01
//! Design `report-package-current-results.md` §2.3 item 6). The desktop helper
//! is checked against the same file by
//! `apps/desktop/src/features/report/reportResultRows.parity.test.ts`.

use open_pipe_stress_report_package::bindings::expected_report_result_rows;
use serde_json::Value;

const VECTORS: &str = include_str!("../../../../fixtures/reports/invented/report_result_rows_parity.json");

#[test]
fn rust_report_rows_match_the_shared_parity_vectors() {
    let vectors: Value = serde_json::from_str(VECTORS).unwrap();
    let rows = expected_report_result_rows(&vectors["source_results"]).unwrap();
    assert_eq!(Value::Array(rows), vectors["expected_rows"]);
}

/// Magnitudes in Rust's large exponent range are integer-valued above 2^53 - 1
/// and fail `guard_json` before a row is formed.
#[test]
fn rust_report_rows_refuse_guard_unsafe_sources() {
    let vectors: Value = serde_json::from_str(VECTORS).unwrap();
    let refused = vectors["guard_refused_sources"].as_array().unwrap();
    assert!(!refused.is_empty());
    for source in refused {
        let error = expected_report_result_rows(source).unwrap_err();
        assert!(error.contains("UNSAFE_JSON_NUMBER"), "{error}");
    }
}

#[test]
fn rust_report_rows_refuse_malformed_sources() {
    for source in [
        serde_json::json!({}),
        serde_json::json!({"results": [{"id": "a", "kind": "k", "unit": "u"}]}),
        serde_json::json!({"results": [{"id": "a", "kind": "k", "value": 1, "unit": "u", "metadata": {}}]}),
        serde_json::json!({"results": [{"id": "a", "kind": "k", "value": 1, "unit": "u", "basis_ref": {"ref_type": "t"}}]}),
    ] {
        let error = expected_report_result_rows(&source).unwrap_err();
        assert!(error.starts_with("REPORT-PACKAGE-REPORT-ROWS-MISMATCH"), "{error}");
    }
}
