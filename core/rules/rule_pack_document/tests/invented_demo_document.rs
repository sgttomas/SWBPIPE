//! End-to-end checks over the repository's invented example rule pack
//! (`examples/rule_packs/invented_demo.yaml`, DEL-06-05): the production
//! codec decodes its frozen-grammar AST, the sandboxed evaluator computes
//! the invented demonstration ratio, the stamped JCS checksum matches a
//! fresh computation (golden cross-engine pin with the Python suite), and
//! document validation reports no blocking findings.
//!
//! All values are invented non-engineering demonstration content. Nothing
//! here is a professional, certification, sealing, authentication, or
//! code-compliance claim.

use std::fs;
use std::path::PathBuf;

use open_pipe_stress_expression_evaluator::{
    evaluate, AnalysisStatus, BindingSource, Dimension, EvaluationInput, EvaluationValue, Quantity,
    VariableBinding,
};
use open_pipe_stress_rule_pack_document::{
    compute_rule_pack_checksum, decode_expression, validate_rule_pack_document,
};

fn example_document() -> serde_json::Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../examples/rule_packs/invented_demo.yaml");
    let raw = fs::read_to_string(path).expect("example rule pack readable");
    serde_json::from_str(&raw).expect("example rule pack is strict JSON")
}

#[test]
fn example_pack_expression_evaluates_the_invented_ratio() {
    let document = example_document();
    let ast = document
        .pointer("/formula_declarations/0/declaration_payload/expression_ast")
        .expect("example formula carries a declarative AST");
    let expression = decode_expression(ast).expect("decode example AST");
    let declared_grammar_version = document
        .pointer("/grammar_version")
        .and_then(serde_json::Value::as_str)
        .expect("example declares grammar_version")
        .to_string();

    let result = evaluate(&EvaluationInput {
        expression,
        bindings: vec![
            VariableBinding::new(
                "demo_actual_quantity",
                BindingSource::SolverResultField,
                Quantity::new(50.0, Dimension::Stress, "demo_unit").expect("invented quantity"),
            ),
            VariableBinding::new(
                "demo_limit_quantity",
                BindingSource::UserSuppliedValue,
                Quantity::new(100.0, Dimension::Stress, "demo_unit").expect("invented quantity"),
            ),
        ],
        required_variable_ids: vec![
            "demo_actual_quantity".to_string(),
            "demo_limit_quantity".to_string(),
        ],
        statuses: vec![AnalysisStatus::MechanicsSolved],
        declared_grammar_version,
    });

    assert!(
        result.findings.is_empty(),
        "unexpected findings: {:?}",
        result.findings
    );
    match result.value {
        Some(EvaluationValue::Quantity(quantity)) => {
            assert_eq!(quantity.value, 0.5);
            assert_eq!(quantity.dimension, Dimension::Dimensionless);
            assert_eq!(quantity.unit_ref, "ratio");
        }
        other => panic!("expected a dimensionless ratio quantity, got {other:?}"),
    }
}

#[test]
fn example_pack_stamped_checksum_matches_recomputation() {
    let document = example_document();
    let computed = compute_rule_pack_checksum(&document).expect("checksum computable");
    let stamped = document
        .pointer("/checksums/rule_pack_checksum/value")
        .and_then(serde_json::Value::as_str)
        .expect("example carries a stamped checksum value");
    assert_eq!(
        computed.value, stamped,
        "stamped checksum is stale; recompute via compute_rule_pack_checksum and restamp \
         (payload = document minus the checksums member, RFC 8785 canonical bytes)"
    );
    assert!(computed.grammar_version_bound);
    assert_eq!(computed.grammar_version, "1.0.0");
}

#[test]
fn example_pack_document_validation_has_no_blocking_findings() {
    let document = example_document();
    let outcome = validate_rule_pack_document(&document, true);
    assert!(
        !outcome.has_blocking_findings,
        "document findings: {:?}, lifecycle findings: {:?}, formulas: {:?}",
        outcome.document_findings, outcome.lifecycle_findings, outcome.formulas
    );
    assert!(!outcome.lifecycle_blocked);
    assert!(outcome.grammar_version_supported);
    assert!(outcome.publicly_exportable);
    assert_eq!(outcome.checksum.match_status, "match");
    assert_eq!(outcome.formulas.len(), 1);
    assert_eq!(outcome.formulas[0].decode_status, "decoded");
}
