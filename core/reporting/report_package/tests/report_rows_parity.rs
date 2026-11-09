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

/// The composer's bound members (DEL-08-01 Design `report-section-composer.md`
/// §2.6): an absent `entity_ref` is null, and `value_text` is the canonical
/// text before the display rule (a Pa value keeps its stored text and unit).
#[test]
fn rust_report_rows_carry_the_bound_members() {
    let rows = expected_report_result_rows(&serde_json::json!({"results": [
        {"id": "p", "kind": "pipe_lame_hoop_stress_v2", "value": 1234567.5, "unit": "Pa"},
        {"id": "z", "kind": "global_nodal_displacement_x", "value": -0.0, "unit": "mm", "entity_ref": "node:N-1"}
    ]}))
    .unwrap();
    assert_eq!(rows[0]["quantity_display"], "1.23457 MPa");
    assert_eq!(rows[0]["value_text"], "1234567.5");
    assert_eq!(rows[0]["unit"], "Pa");
    assert_eq!(rows[0]["entity_ref"], Value::Null);
    assert_eq!(rows[1]["value_text"], "0");
    assert_eq!(rows[1]["entity_ref"], "node:N-1");
    assert_eq!(rows[1]["kind"], "global_nodal_displacement_x");
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
        // The composer's bound entity is a string or absent.
        serde_json::json!({"results": [{"id": "a", "kind": "k", "value": 1, "unit": "u", "entity_ref": 7}]}),
    ] {
        let error = expected_report_result_rows(&source).unwrap_err();
        assert!(error.starts_with("REPORT-PACKAGE-REPORT-ROWS-MISMATCH"), "{error}");
    }
}

/// W-11 (DEL-08-01 Design `exact-route-report-sections.md` §3.4): the
/// section-basis suffix on exact-route rows, and none on a W-7 global
/// end-force row, a row without metadata or metadata without a basis.
#[test]
fn rust_report_rows_add_the_section_basis_suffix() {
    let vectors: Value = serde_json::from_str(VECTORS).unwrap();
    let rows = expected_report_result_rows(&vectors["exact_route_source_results"]).unwrap();
    assert_eq!(Value::Array(rows), vectors["exact_route_expected_rows"]);
    // The pressure-free vectors carry no section_basis token and keep their rows.
    assert!(!vectors["source_results"].to_string().contains("section_basis="));
}

/// Any other `section_basis=` value, or more than one such token, refuses.
#[test]
fn rust_report_rows_refuse_an_unreadable_section_basis() {
    let vectors: Value = serde_json::from_str(VECTORS).unwrap();
    let refused = vectors["section_basis_refused_sources"].as_array().unwrap();
    assert_eq!(refused.len(), 2);
    for source in refused {
        let error = expected_report_result_rows(source).unwrap_err();
        let id = source["results"][0]["id"].as_str().unwrap();
        assert_eq!(error, format!("REPORT-PACKAGE-REPORT-ROWS-MISMATCH: source row {id} has an unreadable section basis"));
    }
}
