//! Rust side of the shared interval-mode parity cases (T3 D2 revision 5b.3
//! §4.11.5): every case in `fixtures/rule_interval/rule_interval_cases.json`
//! is decoded with the production formula decoder and evaluated by the
//! evaluator's interval mode; the outcome, the enclosure bits and the notes
//! must equal the file's. `tests/test_rule_interval.py` runs the same file
//! through the Python reference evaluator. All values are invented.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use open_pipe_stress_expression_evaluator::{
    enclosure_from_bound, evaluate_interval, AnalysisStatus, BindingSource, Enclosure,
    EvaluationInput, Expression, IntervalBinding, IntervalValue, Quantity, Truth, VariableBinding,
    GRAMMAR_VERSION,
};
use open_pipe_stress_rule_pack_document::{decode_dimension, decode_expression, encode_dimension};
use serde_json::Value;

fn case_file() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../fixtures/rule_interval/rule_interval_cases.json");
    let raw = fs::read_to_string(path).expect("case file readable");
    serde_json::from_str(&raw).expect("case file is JSON")
}

fn from_bits(value: &Value) -> f64 {
    let text = value.as_str().expect("bit string");
    f64::from_bits(u64::from_str_radix(text.trim_start_matches("0x"), 16).expect("hex bits"))
}

fn bits(value: f64) -> String {
    format!("0x{:016x}", value.to_bits())
}

fn truth_token(truth: Truth) -> &'static str {
    match truth {
        Truth::True => "T",
        Truth::False => "F",
        Truth::Indeterminate => "U",
    }
}

fn features(expression: &Expression, out: &mut BTreeSet<&'static str>) {
    use open_pipe_stress_expression_evaluator::{
        AggregateFunction, BinaryOperator, ComparisonOperator, LogicalOperator, LookupMode,
        UnaryOperator,
    };
    match expression {
        Expression::Literal(_) => {
            out.insert("literal");
        }
        Expression::VariableRef(_) => {
            out.insert("variable_ref");
        }
        Expression::Unary { operator, operand } => {
            out.insert(match operator {
                UnaryOperator::Negate => "unary:negate",
                UnaryOperator::Abs => "unary:abs",
                UnaryOperator::Not => "unary:not",
            });
            features(operand, out);
        }
        Expression::Binary {
            operator,
            left,
            right,
        } => {
            out.insert(match operator {
                BinaryOperator::Add => "binary:add",
                BinaryOperator::Subtract => "binary:subtract",
                BinaryOperator::Multiply => "binary:multiply",
                BinaryOperator::Divide => "binary:divide",
            });
            features(left, out);
            features(right, out);
        }
        Expression::Compare {
            operator,
            left,
            right,
        } => {
            out.insert(match operator {
                ComparisonOperator::LessThan => "compare:less_than",
                ComparisonOperator::LessThanOrEqual => "compare:less_than_or_equal",
                ComparisonOperator::GreaterThan => "compare:greater_than",
                ComparisonOperator::GreaterThanOrEqual => "compare:greater_than_or_equal",
                ComparisonOperator::Equal => "compare:equal",
                ComparisonOperator::NotEqual => "compare:not_equal",
            });
            features(left, out);
            features(right, out);
        }
        Expression::Logical {
            operator,
            left,
            right,
        } => {
            out.insert(match operator {
                LogicalOperator::And => "logical:and",
                LogicalOperator::Or => "logical:or",
            });
            features(left, out);
            features(right, out);
        }
        Expression::Select {
            condition,
            then_branch,
            else_branch,
        } => {
            out.insert("select");
            features(condition, out);
            features(then_branch, out);
            features(else_branch, out);
        }
        Expression::Aggregate { function, operands } => {
            out.insert(match function {
                AggregateFunction::Min => "aggregate:min",
                AggregateFunction::Max => "aggregate:max",
            });
            for operand in operands {
                features(operand, out);
            }
        }
        Expression::Interpolate { argument, .. } => {
            out.insert("interpolate");
            features(argument, out);
        }
        Expression::Lookup { mode, argument, .. } => {
            out.insert(match mode {
                LookupMode::Exact => "lookup:exact",
                LookupMode::Step => "lookup:step",
            });
            features(argument, out);
        }
        Expression::UnsupportedForm { .. } => {
            out.insert("unsupported_form");
        }
        Expression::UnsafeHostAccess { .. } => {
            out.insert("unsafe_host_access");
        }
    }
}

const EVERY_FEATURE: &[&str] = &[
    "literal",
    "variable_ref",
    "unary:negate",
    "unary:abs",
    "unary:not",
    "binary:add",
    "binary:subtract",
    "binary:multiply",
    "binary:divide",
    "compare:less_than",
    "compare:less_than_or_equal",
    "compare:greater_than",
    "compare:greater_than_or_equal",
    "compare:equal",
    "compare:not_equal",
    "logical:and",
    "logical:or",
    "select",
    "aggregate:min",
    "aggregate:max",
    "interpolate",
    "lookup:exact",
    "lookup:step",
    "unsupported_form",
    "unsafe_host_access",
];

#[test]
fn rust_interval_mode_matches_every_shared_case() {
    let document = case_file();
    assert_eq!(
        document["document_kind"],
        "openpipestress.rule_interval_cases"
    );
    let cases = document["cases"].as_array().expect("cases");
    assert!(cases.len() >= 60, "the shared case file must not shrink");
    let mut covered = BTreeSet::new();
    let mut ids = BTreeSet::new();
    let mut negative_controls = 0;
    for case in cases {
        let case_id = case["case_id"].as_str().expect("case_id");
        assert!(ids.insert(case_id.to_string()), "duplicate {case_id}");
        let expression = decode_expression(&case["formula"]).expect("decodable formula");
        features(&expression, &mut covered);

        let mut bindings = Vec::new();
        let mut intervals = Vec::new();
        for input in case["inputs"].as_array().expect("inputs") {
            let id = input["variable_id"].as_str().unwrap();
            let q = from_bits(&input["value_bits"]);
            let b = from_bits(&input["bound_bits"]);
            let dimension = decode_dimension(input["dimension"].as_str().unwrap()).unwrap();
            bindings.push(VariableBinding::new(
                id,
                BindingSource::SolverResultField,
                Quantity::new(q, dimension, input["unit_ref"].as_str().unwrap()).unwrap(),
            ));
            // An explicit enclosure (or null) replaces the bound; any b other
            // than zero binds enclosure_from_bound(q, b), which has no
            // finite enclosure for a negative, NaN or infinite b.
            if let Some(explicit) = input.get("enclosure_bits") {
                intervals.push(IntervalBinding {
                    variable_id: id.to_string(),
                    enclosure: explicit.as_array().map(|ends| Enclosure {
                        lo: from_bits(&ends[0]),
                        hi: from_bits(&ends[1]),
                    }),
                });
            } else if b != 0.0 {
                intervals.push(IntervalBinding {
                    variable_id: id.to_string(),
                    enclosure: enclosure_from_bound(q, b),
                });
            }
        }
        let result = evaluate_interval(
            &EvaluationInput {
                expression,
                bindings,
                required_variable_ids: vec![],
                statuses: vec![AnalysisStatus::MechanicsSolved],
                declared_grammar_version: GRAMMAR_VERSION.to_string(),
            },
            &intervals,
        );

        let expected = &case["expected"];
        match expected["kind"].as_str().unwrap() {
            "blocked" => {
                let actual: Vec<Value> = result
                    .findings
                    .iter()
                    .map(|f| serde_json::json!([format!("{:?}", f.code), f.subject_id]))
                    .collect();
                assert_eq!(Value::Array(actual), expected["findings"], "{case_id}");
                assert!(result.value.is_none(), "{case_id}");
                continue;
            }
            "truth" => {
                assert!(
                    result.findings.is_empty(),
                    "{case_id}: {:?}",
                    result.findings
                );
                let Some(IntervalValue::Boolean(truth)) = result.value else {
                    panic!("{case_id}: expected a truth, got {:?}", result.value);
                };
                assert_eq!(truth_token(truth), expected["truth"], "{case_id}");
                if case["negative_control"] == true {
                    negative_controls += 1;
                    assert_ne!(truth, Truth::True, "{case_id}: a negative control read T");
                }
            }
            "quantity" => {
                assert!(
                    result.findings.is_empty(),
                    "{case_id}: {:?}",
                    result.findings
                );
                let Some(IntervalValue::Quantity(quantity)) = &result.value else {
                    panic!("{case_id}: expected a quantity, got {:?}", result.value);
                };
                let enclosure = match quantity.enclosure {
                    Some(e) => serde_json::json!([bits(e.lo), bits(e.hi)]),
                    None => Value::Null,
                };
                assert_eq!(enclosure, expected["enclosure"], "{case_id}");
                assert_eq!(
                    encode_dimension(quantity.dimension),
                    expected["dimension"],
                    "{case_id}"
                );
                assert_eq!(quantity.unit_ref, expected["unit_ref"], "{case_id}");
            }
            other => panic!("{case_id}: unknown expected kind {other}"),
        }
        let notes: Vec<Value> = result
            .notes
            .iter()
            .map(|n| serde_json::json!([n.code.as_str(), n.subject_id]))
            .collect();
        assert_eq!(Value::Array(notes), case["notes"], "{case_id}");
    }
    for feature in EVERY_FEATURE {
        assert!(covered.contains(feature), "no case covers {feature}");
    }
    assert!(
        negative_controls >= 8,
        "negative controls: {negative_controls}"
    );
}
