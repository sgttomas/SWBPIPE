//! The report's display rule (DEL-08-01 Design
//! `report-package-current-results.md` §2.3 item 6 and its display
//! resolution) against the shared parity vectors
//! `fixtures/reports/invented/report_quantity_display_parity.json`. The
//! desktop's rule is checked against the same file by
//! `apps/desktop/src/features/report/reportQuantityDisplay.parity.test.ts`.

use open_pipe_stress_report_package::quantity_display::{display_number, display_quantity, source_displays_scaled_value};
use serde_json::{json, Value};

const VECTORS: &str = include_str!("../../../../fixtures/reports/invented/report_quantity_display_parity.json");

#[test]
fn rust_display_rule_matches_the_shared_parity_vectors() {
    let vectors: Value = serde_json::from_str(VECTORS).unwrap();
    let vectors = vectors["vectors"].as_array().unwrap();
    assert!(vectors.len() >= 40);
    for vector in vectors {
        let Value::Number(value) = &vector["value"] else { panic!("a numeric value: {vector}") };
        let unit = vector["unit"].as_str().unwrap();
        assert!(!vector["reason"].as_str().unwrap().is_empty(), "every vector states its reason");
        assert_eq!(display_quantity(value, unit), vector["display"].as_str().unwrap(), "{vector}");
    }
}

/// The edge cases the display rule must cover are present in the vectors.
#[test]
fn the_vectors_cover_the_stated_edge_cases() {
    let vectors: Value = serde_json::from_str(VECTORS).unwrap();
    let displays: Vec<(String, String)> = vectors["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|vector| (vector["unit"].as_str().unwrap().to_string(), vector["display"].as_str().unwrap().to_string()))
        .collect();
    let has = |unit: &str, display: &str| displays.iter().any(|(u, d)| u == unit && d == display);
    assert!(has("Pa", "0 MPa"), "zero");
    assert!(has("rad", "0 rad"), "negative zero");
    assert!(has("Pa", "1 MPa") && has("N", "1000000 N"), "rounding carries in Pa and in another unit");
    assert!(has("N", "12.3457 N"), "half away from zero on an even digit");
    assert!(has("Pa", "1e-13 MPa") && has("N", "1e-7 N"), "small exponent forms");
    assert!(has("N", "1e+21 N") && has("Pa", "1e+21 MPa"), "large exponent forms");
    assert!(has("Pa", "5e-330 MPa") && has("N", "1.79769e+308 N"), "extreme exponents");
    assert!(has("N", "9007200000000000 N") && has("N*m", "0.3 N*m"), "safe integer and binary noise");
}

/// The rule works on the text: no binary64 rounding enters, whatever the
/// exponent (a shifted value outside binary64's range stays exact).
#[test]
fn the_rule_shifts_and_rounds_the_text() {
    assert_eq!(display_number("5e-324", 6), "5e-330");
    assert_eq!(display_number("1.7976931348623157e+308", 6), "1.79769e+302");
    assert_eq!(display_number("-0.0000999999500", 0), "-0.0001");
    assert_eq!(display_number("0", 6), "0");
    assert_eq!(display_number("-0", 0), "0");
    assert_eq!(display_number("999999.5", 6), "1");
}

/// `conversion_performed` (§2.3 item 5): true only when a displayed value is
/// stored in Pa: a result row, or an exact-route headline that is not withheld.
#[test]
fn a_scaled_value_is_displayed_only_for_pa() {
    let row = |unit: &str| json!({"id": "r", "kind": "k", "value": 1, "unit": unit});
    assert!(!source_displays_scaled_value(&json!({"results": [row("MPa"), row("N")]})));
    assert!(source_displays_scaled_value(&json!({"results": [row("MPa"), row("Pa")]})));
    assert!(!source_displays_scaled_value(&json!({"results": []})));
    let headline = |unit: &str| json!({"value": 1, "unit": unit, "location_ref": "l", "result_ref": "r"});
    let exact = |nominal: Value, reduced: Value, id: &str| {
        json!({
            "producer": {"semantic_contract_id": id},
            "results": [row("MPa")],
            "summary": {"max_open_formula_stress_nominal_section": nominal, "max_open_formula_stress_reduced_section": reduced}
        })
    };
    let pressure = "openpipestress.result_semantics/0.3.0/pressure-2";
    let source = "openpipestress.result_semantics/0.3.0/pressure-source-1";
    let preview = "openpipestress.result_semantics/0.3.0/preview-physics-2";
    assert!(source_displays_scaled_value(&exact(headline("Pa"), headline("Pa"), pressure)));
    assert!(source_displays_scaled_value(&exact(Value::Null, headline("Pa"), source)));
    assert!(!source_displays_scaled_value(&exact(Value::Null, Value::Null, pressure)), "withheld headlines display no value");
    assert!(!source_displays_scaled_value(&exact(headline("MPa"), headline("MPa"), pressure)));
    assert!(!source_displays_scaled_value(&exact(headline("Pa"), headline("Pa"), preview)), "a pressure-free report has no headline");
}
