//! The static unit check at rule-pack validation (W-17, FR-RULE-002;
//! DEL-06-02 `Design/rule-pack-unit-check.md` §2 and §4).
//!
//! Every pack here is built inline from invented ids, units and values; no
//! protected standards content. Nothing here is a professional, certification,
//! sealing, authentication, or code-compliance claim.

use open_pipe_stress_rule_pack_document::{
    check_rule_pack_units, validate_rule_pack_document, CheckUnitCheck, UnitCheck,
    UnitCheckFinding,
};
use serde_json::{json, Value};

fn intent(dimension: &str, unit_ref: &str) -> Value {
    json!({
        "dimension": dimension,
        "unit_ref": unit_ref,
        "unit_required": true,
        "dimension_check_required": true
    })
}

fn input(id: &str, dimension: &str, unit_ref: &str) -> Value {
    json!({
        "input_id": id,
        "source_kind": "user_supplied_rule_value",
        "quantity_intent": intent(dimension, unit_ref)
    })
}

fn slot(id: &str, dimension: &str, unit_ref: &str) -> Value {
    json!({"slot_id": id, "quantity_intent": intent(dimension, unit_ref)})
}

fn reference(id: &str) -> Value {
    json!({"ref_id": id, "ref_type": "required_input"})
}

fn variable(id: &str) -> Value {
    json!({"node": "variable_ref", "variable_id": id})
}

fn formula(id: &str, input_refs: &[&str], ast: Value, output: Value) -> Value {
    json!({
        "formula_id": id,
        "declaration_form": "declarative_ast",
        "declaration_payload": {
            "payload_kind": "declarative_ast",
            "payload_summary": "Invented test formula.",
            "expression_ast": ast
        },
        "input_refs": input_refs.iter().map(|id| reference(id)).collect::<Vec<_>>(),
        "output_dimension": output
    })
}

fn check(id: &str, formula_id: &str, input_refs: &[&str], slot_refs: &[&str]) -> Value {
    json!({
        "check_id": id,
        "category_ref": {"ref_id": "invented_category", "ref_type": "stress_category"},
        "required_input_refs": input_refs.iter().map(|id| reference(id)).collect::<Vec<_>>(),
        "value_slot_refs": slot_refs
            .iter()
            .map(|id| json!({"ref_id": id, "ref_type": "value_slot"}))
            .collect::<Vec<_>>(),
        "formula_ref": {"ref_id": formula_id, "ref_type": "formula"}
    })
}

fn pack(inputs: Vec<Value>, slots: Vec<Value>, formulas: Vec<Value>, checks: Vec<Value>) -> Value {
    json!({
        "schema_version": "0.5.0",
        "rule_pack_kind": "open_pipe_stress_rule_pack",
        "grammar_version": "1.0.0",
        "metadata": {"rule_pack_id": "invented_unit_check_pack", "schema_version": "0.5.0"},
        "required_inputs": inputs,
        "value_slots": slots,
        "required_load_cases": [{
            "required_load_case_id": "invented_case",
            "name": "Invented case",
            "basis_ref": {"ref_type": "load_case", "ref_id": "load:invented"}
        }],
        "stress_categories": [{"category_id": "invented_category", "name": "Invented category A"}],
        "load_case_mappings": [{
            "mapping_id": "invented_mapping",
            "category_ref": {"ref_id": "invented_category", "ref_type": "stress_category"},
            "required_load_case_refs": [{"ref_id": "invented_case", "ref_type": "required_load_case"}]
        }],
        "formula_declarations": formulas,
        "check_definitions": checks
    })
}

/// A ratio of two catalog stress inputs (`MPa`), checked against a
/// dimensionless slot in the evaluator's `ratio` token.
fn ratio_formula(output: Value) -> Value {
    formula(
        "invented_ratio",
        &["invented_actual", "invented_limit"],
        json!({
            "node": "binary",
            "operator": "divide",
            "left": variable("invented_actual"),
            "right": variable("invented_limit")
        }),
        output,
    )
}

fn stress_inputs() -> Vec<Value> {
    vec![
        input("invented_actual", "stress", "MPa"),
        input("invented_limit", "stress", "MPa"),
    ]
}

fn only_check(unit_check: &UnitCheck) -> &CheckUnitCheck {
    assert_eq!(unit_check.checks.len(), 1, "{unit_check:?}");
    &unit_check.checks[0]
}

fn codes(findings: &[UnitCheckFinding]) -> Vec<(&str, &str, &str)> {
    findings
        .iter()
        .map(|finding| {
            (
                finding.code.as_str(),
                finding.severity.as_str(),
                finding.subject.as_str(),
            )
        })
        .collect()
}

#[test]
fn a_consistent_pack_reads_consistent() {
    let document = pack(
        stress_inputs(),
        vec![slot("invented_slot", "dimensionless", "1")],
        vec![ratio_formula(intent("dimensionless", "ratio"))],
        vec![check(
            "invented_check",
            "invented_ratio",
            &["invented_actual", "invented_limit"],
            &["invented_slot"],
        )],
    );
    let unit_check = check_rule_pack_units(&document);
    // `1` is the catalog's dimensionless symbol, but the evaluator names a
    // derived ratio `ratio`: the slot never matches the formula's unit.
    let check = only_check(&unit_check);
    assert_eq!(
        codes(&check.findings),
        vec![(
            "RULE_PACK_LIMIT_UNIT_MISMATCH",
            "blocking",
            "invented_slot"
        )]
    );

    let mut consistent = document.clone();
    consistent["value_slots"][0] = slot("invented_slot", "dimensionless", "ratio");
    let unit_check = check_rule_pack_units(&consistent);
    // `ratio` is not a catalog symbol: a warning on the declaration only.
    assert_eq!(
        codes(&unit_check.declaration_findings),
        vec![("RULE_PACK_UNIT_NOT_IN_CATALOG", "warning", "invented_slot")]
    );
    assert_eq!(unit_check.declaration_findings[0].path, "value_slots[0].quantity_intent.unit_ref");
    let check = only_check(&unit_check);
    assert_eq!(check.status, "consistent", "{check:?}");
    assert!(check.findings.is_empty());
    assert_eq!(unit_check.status, "findings");
}

#[test]
fn an_expression_unit_mismatch_blocks_its_check_with_the_runtime_code() {
    let document = pack(
        vec![
            input("invented_actual", "stress", "MPa"),
            input("invented_limit", "stress", "Pa"),
        ],
        vec![slot("invented_slot", "dimensionless", "ratio")],
        vec![ratio_formula(intent("dimensionless", "ratio"))],
        vec![check(
            "invented_check",
            "invented_ratio",
            &["invented_actual", "invented_limit"],
            &["invented_slot"],
        )],
    );
    let unit_check = check_rule_pack_units(&document);
    let check = only_check(&unit_check);
    assert_eq!(check.status, "findings");
    assert_eq!(check.formula_id, "invented_ratio");
    assert_eq!(
        codes(&check.findings),
        vec![("UnitMismatch", "blocking", "divide")]
    );
    let finding = &check.findings[0];
    assert_eq!(
        finding.path,
        "formula_declarations[0].declaration_payload.expression_ast"
    );
    let units: Vec<Option<&str>> = finding
        .operands
        .iter()
        .map(|operand| operand.unit_ref.as_deref())
        .collect();
    assert_eq!(units, vec![Some("MPa"), Some("Pa")]);
}

#[test]
fn an_undeclared_variable_blocks_its_check() {
    // `invented_other` is in the AST but in no input_refs list; and
    // `invented_limit` is in the formula's input_refs but not the check's.
    let document = pack(
        stress_inputs(),
        vec![],
        vec![formula(
            "invented_predicate",
            &["invented_actual", "invented_limit"],
            json!({
                "node": "logical",
                "operator": "and",
                "left": {
                    "node": "compare",
                    "operator": "less_than",
                    "left": variable("invented_actual"),
                    "right": variable("invented_limit")
                },
                "right": {
                    "node": "compare",
                    "operator": "less_than",
                    "left": variable("invented_other"),
                    "right": variable("invented_actual")
                }
            }),
            intent("dimensionless", "ratio"),
        )],
        vec![check("invented_check", "invented_predicate", &["invented_actual"], &[])],
    );
    let unit_check = check_rule_pack_units(&document);
    let check = only_check(&unit_check);
    assert_eq!(
        codes(&check.findings),
        vec![
            ("RULE_PACK_UNDECLARED_VARIABLE", "blocking", "invented_limit"),
            ("RULE_PACK_UNDECLARED_VARIABLE", "blocking", "invented_other"),
        ]
    );
    assert_eq!(
        check.findings[1].path,
        "formula_declarations[0].declaration_payload.expression_ast.right.left"
    );
}

fn limit_document(slots: Vec<Value>, slot_refs: &[&str]) -> Value {
    pack(
        stress_inputs(),
        slots,
        vec![ratio_formula(intent("dimensionless", "ratio"))],
        vec![check(
            "invented_check",
            "invented_ratio",
            &["invented_actual", "invented_limit"],
            slot_refs,
        )],
    )
}

#[test]
fn a_boolean_predicate_never_reads_a_limit() {
    let document = pack(
        stress_inputs(),
        vec![slot("invented_slot", "length", "mm")],
        vec![formula(
            "invented_predicate",
            &["invented_actual", "invented_limit"],
            json!({
                "node": "compare",
                "operator": "less_than_or_equal",
                "left": variable("invented_actual"),
                "right": variable("invented_limit")
            }),
            intent("dimensionless", "ratio"),
        )],
        vec![check(
            "invented_check",
            "invented_predicate",
            &["invented_actual", "invented_limit"],
            &["invented_slot"],
        )],
    );
    let unit_check = check_rule_pack_units(&document);
    let check = only_check(&unit_check);
    assert_eq!(check.status, "consistent", "{check:?}");
}

#[test]
fn a_single_incompatible_declared_slot_is_an_unconditional_limit_mismatch() {
    let unit_check = check_rule_pack_units(&limit_document(
        vec![slot("invented_slot", "stress", "MPa")],
        &["invented_slot"],
    ));
    let check = only_check(&unit_check);
    assert_eq!(
        codes(&check.findings),
        vec![("RULE_PACK_LIMIT_UNIT_MISMATCH", "blocking", "invented_slot")]
    );
    assert_eq!(
        check.findings[0].path,
        "check_definitions[0].value_slot_refs[0]"
    );
}

#[test]
fn one_compatible_and_one_incompatible_slot_give_a_candidate_finding_only() {
    let unit_check = check_rule_pack_units(&limit_document(
        vec![
            slot("invented_good_slot", "dimensionless", "ratio"),
            slot("invented_bad_slot", "stress", "MPa"),
        ],
        &["invented_good_slot", "invented_bad_slot"],
    ));
    let check = only_check(&unit_check);
    assert_eq!(
        codes(&check.findings),
        vec![(
            "RULE_PACK_LIMIT_CANDIDATE_MISMATCH",
            "conditional",
            "invented_bad_slot"
        )]
    );
}

#[test]
fn an_undeclared_slot_is_a_warning_only() {
    let unit_check =
        check_rule_pack_units(&limit_document(vec![], &["invented_undeclared_slot"]));
    let check = only_check(&unit_check);
    assert_eq!(
        codes(&check.findings),
        vec![(
            "RULE_PACK_LIMIT_SLOT_UNDECLARED",
            "warning",
            "invented_undeclared_slot"
        )]
    );
}

#[test]
fn an_incomplete_declaration_is_reported_and_its_parts_are_not_typed() {
    // The TBD input makes the quotient unknown: no expression finding, no
    // limit finding against the incompatible slot, and no output warning.
    let unit_check = check_rule_pack_units(&pack(
        vec![
            input("invented_actual", "stress", "MPa"),
            input("invented_limit", "TBD", "TBD"),
        ],
        vec![slot("invented_slot", "length", "mm")],
        vec![ratio_formula(intent("length", "mm"))],
        vec![check(
            "invented_check",
            "invented_ratio",
            &["invented_actual", "invented_limit"],
            &["invented_slot"],
        )],
    ));
    let check = only_check(&unit_check);
    assert_eq!(
        codes(&check.findings),
        vec![(
            "RULE_PACK_UNIT_DECLARATION_INCOMPLETE",
            "warning",
            "invented_limit"
        )]
    );
    assert_eq!(check.findings[0].path, "required_inputs[1].quantity_intent");
}

#[test]
fn an_output_dimension_difference_is_a_warning() {
    let unit_check = check_rule_pack_units(&limit_document(
        vec![slot("invented_slot", "dimensionless", "ratio")],
        &["invented_slot"],
    ));
    assert_eq!(only_check(&unit_check).status, "consistent");

    let mut document = limit_document(
        vec![slot("invented_slot", "dimensionless", "ratio")],
        &["invented_slot"],
    );
    document["formula_declarations"][0]["output_dimension"] = intent("stress", "MPa");
    let unit_check = check_rule_pack_units(&document);
    let check = only_check(&unit_check);
    assert_eq!(
        codes(&check.findings),
        vec![(
            "RULE_PACK_OUTPUT_DIMENSION_DIFFERS",
            "warning",
            "invented_ratio"
        )]
    );
}

#[test]
fn a_non_catalog_unit_is_a_warning_not_an_error() {
    let unit_check = check_rule_pack_units(&pack(
        vec![
            input("invented_actual", "stress", "invented_stress_unit"),
            input("invented_limit", "stress", "invented_stress_unit"),
        ],
        vec![slot("invented_slot", "dimensionless", "ratio")],
        vec![ratio_formula(intent("dimensionless", "ratio"))],
        vec![check(
            "invented_check",
            "invented_ratio",
            &["invented_actual", "invented_limit"],
            &["invented_slot"],
        )],
    ));
    assert_eq!(
        codes(&unit_check.declaration_findings),
        vec![
            ("RULE_PACK_UNIT_NOT_IN_CATALOG", "warning", "invented_actual"),
            ("RULE_PACK_UNIT_NOT_IN_CATALOG", "warning", "invented_limit"),
            ("RULE_PACK_UNIT_NOT_IN_CATALOG", "warning", "invented_slot"),
        ]
    );
    assert!(unit_check.declaration_findings[0]
        .message
        .contains("refused at run time"));
    assert_eq!(only_check(&unit_check).status, "consistent");
}

#[test]
fn a_refusal_marker_or_undecodable_formula_is_not_checkable() {
    let mut document = limit_document(
        vec![slot("invented_slot", "dimensionless", "ratio")],
        &["invented_slot"],
    );
    document["formula_declarations"][0]["declaration_payload"]["expression_ast"] =
        json!({"node": "unsafe_host_access", "request": "filesystem"});
    let unit_check = check_rule_pack_units(&document);
    assert_eq!(only_check(&unit_check).status, "not_checkable");
    assert!(only_check(&unit_check).findings.is_empty());

    document["formula_declarations"][0]["declaration_payload"]["expression_ast"] =
        json!({"node": "no_such_node"});
    assert_eq!(
        only_check(&check_rule_pack_units(&document)).status,
        "not_checkable"
    );
}

#[test]
fn unit_check_findings_never_enter_has_blocking_findings() {
    let consistent = limit_document(
        vec![slot("invented_slot", "dimensionless", "ratio")],
        &["invented_slot"],
    );
    let inconsistent = limit_document(
        vec![slot("invented_slot", "stress", "MPa")],
        &["invented_slot"],
    );
    let consistent = validate_rule_pack_document(&consistent, false);
    let inconsistent = validate_rule_pack_document(&inconsistent, false);
    assert_eq!(inconsistent.unit_check.status, "findings");
    assert_eq!(
        inconsistent.has_blocking_findings,
        consistent.has_blocking_findings
    );
    assert_eq!(inconsistent.document_findings, consistent.document_findings);
    assert_eq!(inconsistent.formulas, consistent.formulas);
}

#[test]
fn unit_check_diagnostics_never_echo_magnitudes_table_rows_or_payload_text() {
    // Sentinel magnitude, table row and payload summary: the serialized
    // unit_check carries findings for this formula but none of them.
    let ast = json!({
        "node": "binary",
        "operator": "add",
        "left": {
            "node": "interpolate",
            "table": {
                "table_id": "invented_table",
                "argument_dimension": "stress",
                "argument_unit_ref": "MPa",
                "result_dimension": "stress",
                "result_unit_ref": "MPa",
                "rows": [
                    {"argument": 24681.13579, "result": 97531.08642},
                    {"argument": 35792.46813, "result": 86420.97531}
                ]
            },
            "argument": {
                "node": "literal",
                "quantity": {"value": 31415.92653, "dimension": "length", "unit_ref": "mm"}
            }
        },
        "right": {
            "node": "literal",
            "quantity": {"value": 27182.81828, "dimension": "stress", "unit_ref": "Pa"}
        }
    });
    let mut document = pack(
        stress_inputs(),
        vec![],
        vec![formula(
            "invented_sentinel_formula",
            &["invented_actual"],
            ast,
            intent("stress", "MPa"),
        )],
        vec![check(
            "invented_check",
            "invented_sentinel_formula",
            &["invented_actual"],
            &[],
        )],
    );
    document["formula_declarations"][0]["declaration_payload"]["payload_summary"] =
        json!("SENTINEL-PAYLOAD-SUMMARY-TEXT");
    let unit_check = check_rule_pack_units(&document);
    let check = only_check(&unit_check);
    assert_eq!(
        codes(&check.findings),
        vec![("DimensionMismatch", "blocking", "invented_table")]
    );
    let serialized = serde_json::to_string(&unit_check).expect("unit_check serializes");
    for sentinel in [
        "24681", "97531", "35792", "86420", "31415", "27182", "SENTINEL-PAYLOAD",
    ] {
        assert!(
            !serialized.contains(sentinel),
            "unit_check echoes {sentinel}: {serialized}"
        );
    }
}
