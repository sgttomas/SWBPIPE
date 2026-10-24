//! Golden conformance corpus runner for the frozen rule-pack expression
//! grammar (DEC-022 / D-02 Option A, tranche TP-C1-GRAMMAR-001).
//!
//! Executes every `case_*.json` file under
//! `fixtures/rule_expressions/conformance_corpus/` through `evaluate` and
//! asserts the recorded expected outcome exactly: value (exact `f64`
//! equality on canonical outputs), ordered findings as
//! `(code, subject_id)` records, resulting statuses, and sorted source
//! variable ids. The corpus is the freeze artifact for grammar
//! v`GRAMMAR_VERSION`: a grammar change without a corresponding corpus
//! extension fails review.
//!
//! The JSON reader below is part of this test harness only. The evaluator
//! crate itself still does not parse text (rule-pack expressions reach it as
//! structured ASTs); corpus files are repo-controlled fixtures, so the
//! reader is a strict, panic-on-malformed ASCII JSON subset with no
//! dependencies.
//!
//! Boundary: all corpus data is invented (PUBLIC_DOMAIN_OR_ORIGINAL); no
//! protected standards content; no compliance/certification/approval claims.

use open_pipe_stress_expression_evaluator::{
    check_expression_types, evaluate, AggregateFunction, AnalysisStatus, BinaryOperator,
    BindingSource, ComparisonOperator, Dimension, EvaluationInput, EvaluationValue, Expression,
    FindingCode, LogicalOperator, LookupMode, Quantity, TableRow, TypeEnv, UnaryOperator,
    UserTable, VariableBinding, GRAMMAR_VERSION,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Coverage floor (enforced, not just documented)
// ---------------------------------------------------------------------------

/// Every grammar feature must appear in at least one accepting
/// (finding-free) case.
const REQUIRED_ACCEPT_FEATURES: &[&str] = &[
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
];

/// Every relevant finding code must appear in at least one blocking case.
/// `NonFiniteInput` is excluded because finite JSON cannot encode NaN/inf;
/// it is pinned by in-crate unit tests instead.
const REQUIRED_BLOCK_CODES: &[FindingCode] = &[
    FindingCode::UnsafeConstruct,
    FindingCode::UnsupportedExpressionForm,
    FindingCode::MissingVariable,
    FindingCode::DuplicateBinding,
    FindingCode::InvalidReference,
    FindingCode::MissingRequiredValue,
    FindingCode::DivisionByZero,
    FindingCode::UnitMetadataMissing,
    FindingCode::UnitMismatch,
    FindingCode::DimensionMismatch,
    FindingCode::TypeMismatch,
    FindingCode::StatusBoundaryViolation,
    FindingCode::UnsupportedGrammarVersion,
    FindingCode::TableMalformed,
    FindingCode::TableOutOfRange,
    FindingCode::TableKeyNotFound,
];

// ---------------------------------------------------------------------------
// Minimal strict JSON reader (test harness only; ASCII subset; no deps)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn parse(text: &'a str) -> Json {
        assert!(text.is_ascii(), "corpus files must be ASCII");
        let mut reader = Reader {
            bytes: text.as_bytes(),
            pos: 0,
        };
        let value = reader.value();
        reader.skip_whitespace();
        assert!(
            reader.pos == reader.bytes.len(),
            "trailing content after JSON value at byte {}",
            reader.pos
        );
        value
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.bytes.len()
            && matches!(self.bytes[self.pos], b' ' | b'\t' | b'\n' | b'\r')
        {
            self.pos += 1;
        }
    }

    fn peek(&mut self) -> u8 {
        self.skip_whitespace();
        assert!(self.pos < self.bytes.len(), "unexpected end of JSON");
        self.bytes[self.pos]
    }

    fn expect(&mut self, byte: u8) {
        let actual = self.peek();
        assert_eq!(
            actual, byte,
            "expected '{}' at byte {}, found '{}'",
            byte as char, self.pos, actual as char
        );
        self.pos += 1;
    }

    fn value(&mut self) -> Json {
        match self.peek() {
            b'{' => self.object(),
            b'[' => self.array(),
            b'"' => Json::String(self.string()),
            b't' | b'f' | b'n' => self.literal(),
            _ => self.number(),
        }
    }

    fn object(&mut self) -> Json {
        self.expect(b'{');
        let mut map = BTreeMap::new();
        if self.peek() == b'}' {
            self.pos += 1;
            return Json::Object(map);
        }
        loop {
            let key = self.string_after_whitespace();
            self.expect(b':');
            let value = self.value();
            assert!(
                map.insert(key.clone(), value).is_none(),
                "duplicate object key '{key}'"
            );
            match self.peek() {
                b',' => self.pos += 1,
                b'}' => {
                    self.pos += 1;
                    return Json::Object(map);
                }
                other => panic!("expected ',' or '}}', found '{}'", other as char),
            }
        }
    }

    fn array(&mut self) -> Json {
        self.expect(b'[');
        let mut items = Vec::new();
        if self.peek() == b']' {
            self.pos += 1;
            return Json::Array(items);
        }
        loop {
            items.push(self.value());
            match self.peek() {
                b',' => self.pos += 1,
                b']' => {
                    self.pos += 1;
                    return Json::Array(items);
                }
                other => panic!("expected ',' or ']', found '{}'", other as char),
            }
        }
    }

    fn string_after_whitespace(&mut self) -> String {
        assert_eq!(self.peek(), b'"', "expected string key");
        self.string()
    }

    fn string(&mut self) -> String {
        self.expect(b'"');
        let mut out = String::new();
        loop {
            assert!(self.pos < self.bytes.len(), "unterminated string");
            let byte = self.bytes[self.pos];
            self.pos += 1;
            match byte {
                b'"' => return out,
                b'\\' => {
                    assert!(self.pos < self.bytes.len(), "unterminated escape");
                    let escape = self.bytes[self.pos];
                    self.pos += 1;
                    match escape {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'n' => out.push('\n'),
                        b't' => out.push('\t'),
                        b'r' => out.push('\r'),
                        other => panic!("unsupported escape '\\{}'", other as char),
                    }
                }
                0x20..=0x7e => out.push(byte as char),
                other => panic!("unsupported byte 0x{other:02x} in string"),
            }
        }
    }

    fn literal(&mut self) -> Json {
        self.skip_whitespace();
        let rest = &self.bytes[self.pos..];
        for (token, value) in [
            ("true", Json::Bool(true)),
            ("false", Json::Bool(false)),
            ("null", Json::Null),
        ] {
            if rest.starts_with(token.as_bytes()) {
                self.pos += token.len();
                return value;
            }
        }
        panic!("invalid literal at byte {}", self.pos);
    }

    fn number(&mut self) -> Json {
        self.skip_whitespace();
        let start = self.pos;
        while self.pos < self.bytes.len()
            && matches!(
                self.bytes[self.pos],
                b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E'
            )
        {
            self.pos += 1;
        }
        let token = std::str::from_utf8(&self.bytes[start..self.pos]).expect("ascii");
        let value: f64 = token
            .parse()
            .unwrap_or_else(|_| panic!("invalid number token '{token}'"));
        assert!(value.is_finite(), "corpus numbers must be finite");
        Json::Number(value)
    }
}

// ---------------------------------------------------------------------------
// Decoding helpers
// ---------------------------------------------------------------------------

fn as_object<'a>(value: &'a Json, context: &str) -> &'a BTreeMap<String, Json> {
    let Json::Object(map) = value else {
        panic!("{context}: expected object");
    };
    map
}

fn field<'a>(map: &'a BTreeMap<String, Json>, key: &str, context: &str) -> &'a Json {
    map.get(key)
        .unwrap_or_else(|| panic!("{context}: missing field '{key}'"))
}

fn string_field(map: &BTreeMap<String, Json>, key: &str, context: &str) -> String {
    let Json::String(value) = field(map, key, context) else {
        panic!("{context}: field '{key}' must be a string");
    };
    value.clone()
}

fn number_field(map: &BTreeMap<String, Json>, key: &str, context: &str) -> f64 {
    let Json::Number(value) = field(map, key, context) else {
        panic!("{context}: field '{key}' must be a number");
    };
    *value
}

fn array_field<'a>(map: &'a BTreeMap<String, Json>, key: &str, context: &str) -> &'a [Json] {
    let Json::Array(items) = field(map, key, context) else {
        panic!("{context}: field '{key}' must be an array");
    };
    items
}

fn decode_dimension(token: &str, context: &str) -> Dimension {
    match token {
        "dimensionless" => Dimension::Dimensionless,
        "length" => Dimension::Length,
        "mass" => Dimension::Mass,
        "time" => Dimension::Time,
        "temperature" => Dimension::Temperature,
        "temperature_interval" => Dimension::TemperatureInterval,
        "angle" => Dimension::Angle,
        "rotation" => Dimension::Rotation,
        "force" => Dimension::Force,
        "moment" => Dimension::Moment,
        "pressure" => Dimension::Pressure,
        "stress" => Dimension::Stress,
        "area" => Dimension::Area,
        "volume" => Dimension::Volume,
        "density" => Dimension::Density,
        "linear_stiffness" => Dimension::LinearStiffness,
        "rotational_stiffness" => Dimension::RotationalStiffness,
        "displacement" => Dimension::Displacement,
        "velocity" => Dimension::Velocity,
        "acceleration" => Dimension::Acceleration,
        "thermal_conductivity" => Dimension::ThermalConductivity,
        "specific_heat" => Dimension::SpecificHeat,
        "thermal_expansion_coefficient" => Dimension::ThermalExpansionCoefficient,
        "second_moment_area" => Dimension::SecondMomentArea,
        "section_modulus" => Dimension::SectionModulus,
        "mass_per_length" => Dimension::MassPerLength,
        "volume_per_length" => Dimension::VolumePerLength,
        "slope" => Dimension::Slope,
        "tbd" => Dimension::Tbd,
        other => panic!("{context}: unknown dimension token '{other}'"),
    }
}

fn decode_status(token: &str, context: &str) -> AnalysisStatus {
    match token {
        "model_incomplete" => AnalysisStatus::ModelIncomplete,
        "mechanics_solved" => AnalysisStatus::MechanicsSolved,
        "rule_inputs_incomplete" => AnalysisStatus::RuleInputsIncomplete,
        "user_rule_checked" => AnalysisStatus::UserRuleChecked,
        "user_rule_failed" => AnalysisStatus::UserRuleFailed,
        "human_review_required" => AnalysisStatus::HumanReviewRequired,
        "human_approved_for_project" => AnalysisStatus::HumanApprovedForProject,
        other => panic!("{context}: unknown status token '{other}'"),
    }
}

fn decode_source(token: &str, context: &str) -> BindingSource {
    match token {
        "rule_pack_required_input" => BindingSource::RulePackRequiredInput,
        "user_supplied_value" => BindingSource::UserSuppliedValue,
        "solver_result_field" => BindingSource::SolverResultField,
        other => panic!("{context}: unknown binding source token '{other}'"),
    }
}

fn decode_finding_code(token: &str, context: &str) -> FindingCode {
    match token {
        "unsafe_construct" => FindingCode::UnsafeConstruct,
        "unsupported_expression_form" => FindingCode::UnsupportedExpressionForm,
        "missing_variable" => FindingCode::MissingVariable,
        "duplicate_binding" => FindingCode::DuplicateBinding,
        "invalid_reference" => FindingCode::InvalidReference,
        "missing_required_value" => FindingCode::MissingRequiredValue,
        "non_finite_input" => FindingCode::NonFiniteInput,
        "division_by_zero" => FindingCode::DivisionByZero,
        "unit_metadata_missing" => FindingCode::UnitMetadataMissing,
        "unit_mismatch" => FindingCode::UnitMismatch,
        "dimension_mismatch" => FindingCode::DimensionMismatch,
        "type_mismatch" => FindingCode::TypeMismatch,
        "status_boundary_violation" => FindingCode::StatusBoundaryViolation,
        "unsupported_grammar_version" => FindingCode::UnsupportedGrammarVersion,
        "table_malformed" => FindingCode::TableMalformed,
        "table_out_of_range" => FindingCode::TableOutOfRange,
        "table_key_not_found" => FindingCode::TableKeyNotFound,
        other => panic!("{context}: unknown finding code token '{other}'"),
    }
}

fn decode_quantity(value: &Json, context: &str) -> Quantity {
    let map = as_object(value, context);
    let unit_required = match map.get("unit_required") {
        Some(Json::Bool(flag)) => *flag,
        None => true,
        Some(_) => panic!("{context}: unit_required must be a boolean"),
    };
    let dimension_check_required = match map.get("dimension_check_required") {
        Some(Json::Bool(flag)) => *flag,
        None => true,
        Some(_) => panic!("{context}: dimension_check_required must be a boolean"),
    };
    Quantity {
        value: number_field(map, "value", context),
        dimension: decode_dimension(&string_field(map, "dimension", context), context),
        unit_ref: string_field(map, "unit_ref", context),
        unit_required,
        dimension_check_required,
    }
}

fn decode_table(value: &Json, context: &str) -> UserTable {
    let map = as_object(value, context);
    let rows = array_field(map, "rows", context)
        .iter()
        .map(|row| {
            let row = as_object(row, context);
            TableRow {
                argument: number_field(row, "argument", context),
                result: number_field(row, "result", context),
            }
        })
        .collect();
    UserTable {
        table_id: string_field(map, "table_id", context),
        argument_dimension: decode_dimension(
            &string_field(map, "argument_dimension", context),
            context,
        ),
        argument_unit_ref: string_field(map, "argument_unit_ref", context),
        result_dimension: decode_dimension(
            &string_field(map, "result_dimension", context),
            context,
        ),
        result_unit_ref: string_field(map, "result_unit_ref", context),
        rows,
    }
}

/// Decodes an expression node and records the grammar features it uses for
/// the coverage-floor check.
fn decode_expression(value: &Json, context: &str, features: &mut BTreeSet<String>) -> Expression {
    let map = as_object(value, context);
    let node = string_field(map, "node", context);
    match node.as_str() {
        "literal" => {
            features.insert("literal".to_string());
            Expression::Literal(decode_quantity(field(map, "quantity", context), context))
        }
        "variable_ref" => {
            features.insert("variable_ref".to_string());
            Expression::VariableRef(string_field(map, "variable_id", context))
        }
        "unary" => {
            let operator = string_field(map, "operator", context);
            features.insert(format!("unary:{operator}"));
            let operator = match operator.as_str() {
                "negate" => UnaryOperator::Negate,
                "abs" => UnaryOperator::Abs,
                "not" => UnaryOperator::Not,
                other => panic!("{context}: unknown unary operator '{other}'"),
            };
            Expression::Unary {
                operator,
                operand: Box::new(decode_expression(
                    field(map, "operand", context),
                    context,
                    features,
                )),
            }
        }
        "binary" => {
            let operator = string_field(map, "operator", context);
            features.insert(format!("binary:{operator}"));
            let operator = match operator.as_str() {
                "add" => BinaryOperator::Add,
                "subtract" => BinaryOperator::Subtract,
                "multiply" => BinaryOperator::Multiply,
                "divide" => BinaryOperator::Divide,
                other => panic!("{context}: unknown binary operator '{other}'"),
            };
            Expression::Binary {
                operator,
                left: Box::new(decode_expression(
                    field(map, "left", context),
                    context,
                    features,
                )),
                right: Box::new(decode_expression(
                    field(map, "right", context),
                    context,
                    features,
                )),
            }
        }
        "compare" => {
            let operator = string_field(map, "operator", context);
            features.insert(format!("compare:{operator}"));
            let operator = match operator.as_str() {
                "less_than" => ComparisonOperator::LessThan,
                "less_than_or_equal" => ComparisonOperator::LessThanOrEqual,
                "greater_than" => ComparisonOperator::GreaterThan,
                "greater_than_or_equal" => ComparisonOperator::GreaterThanOrEqual,
                "equal" => ComparisonOperator::Equal,
                "not_equal" => ComparisonOperator::NotEqual,
                other => panic!("{context}: unknown comparison operator '{other}'"),
            };
            Expression::Compare {
                operator,
                left: Box::new(decode_expression(
                    field(map, "left", context),
                    context,
                    features,
                )),
                right: Box::new(decode_expression(
                    field(map, "right", context),
                    context,
                    features,
                )),
            }
        }
        "logical" => {
            let operator = string_field(map, "operator", context);
            features.insert(format!("logical:{operator}"));
            let operator = match operator.as_str() {
                "and" => LogicalOperator::And,
                "or" => LogicalOperator::Or,
                other => panic!("{context}: unknown logical operator '{other}'"),
            };
            Expression::Logical {
                operator,
                left: Box::new(decode_expression(
                    field(map, "left", context),
                    context,
                    features,
                )),
                right: Box::new(decode_expression(
                    field(map, "right", context),
                    context,
                    features,
                )),
            }
        }
        "select" => {
            features.insert("select".to_string());
            Expression::Select {
                condition: Box::new(decode_expression(
                    field(map, "condition", context),
                    context,
                    features,
                )),
                then_branch: Box::new(decode_expression(
                    field(map, "then", context),
                    context,
                    features,
                )),
                else_branch: Box::new(decode_expression(
                    field(map, "else", context),
                    context,
                    features,
                )),
            }
        }
        "aggregate" => {
            let function = string_field(map, "function", context);
            features.insert(format!("aggregate:{function}"));
            let function = match function.as_str() {
                "min" => AggregateFunction::Min,
                "max" => AggregateFunction::Max,
                other => panic!("{context}: unknown aggregate function '{other}'"),
            };
            Expression::Aggregate {
                function,
                operands: array_field(map, "operands", context)
                    .iter()
                    .map(|operand| decode_expression(operand, context, features))
                    .collect(),
            }
        }
        "interpolate" => {
            features.insert("interpolate".to_string());
            Expression::Interpolate {
                table: decode_table(field(map, "table", context), context),
                argument: Box::new(decode_expression(
                    field(map, "argument", context),
                    context,
                    features,
                )),
            }
        }
        "lookup" => {
            let mode = string_field(map, "mode", context);
            features.insert(format!("lookup:{mode}"));
            let mode = match mode.as_str() {
                "exact" => LookupMode::Exact,
                "step" => LookupMode::Step,
                other => panic!("{context}: unknown lookup mode '{other}'"),
            };
            Expression::Lookup {
                table: decode_table(field(map, "table", context), context),
                mode,
                argument: Box::new(decode_expression(
                    field(map, "argument", context),
                    context,
                    features,
                )),
            }
        }
        "unsupported_form" => Expression::UnsupportedForm {
            form_id: string_field(map, "form_id", context),
        },
        "unsafe_host_access" => Expression::UnsafeHostAccess {
            request: string_field(map, "request", context),
        },
        other => panic!("{context}: unknown expression node '{other}'"),
    }
}

struct ExpectedOutcome {
    value: Option<EvaluationValue>,
    findings: Vec<(FindingCode, String)>,
    statuses: Vec<AnalysisStatus>,
    source_variable_ids: Vec<String>,
}

fn decode_expected(value: &Json, context: &str) -> ExpectedOutcome {
    let map = as_object(value, context);
    let expected_value = match field(map, "value", context) {
        Json::Null => None,
        node => {
            let node = as_object(node, context);
            match string_field(node, "kind", context).as_str() {
                "boolean" => {
                    let Json::Bool(flag) = field(node, "value", context) else {
                        panic!("{context}: boolean expected value must be a bool");
                    };
                    Some(EvaluationValue::Boolean(*flag))
                }
                "quantity" => Some(EvaluationValue::Quantity(decode_quantity(
                    field(map, "value", context),
                    context,
                ))),
                other => panic!("{context}: unknown expected value kind '{other}'"),
            }
        }
    };
    let findings = array_field(map, "findings", context)
        .iter()
        .map(|finding| {
            let finding = as_object(finding, context);
            (
                decode_finding_code(&string_field(finding, "code", context), context),
                string_field(finding, "subject_id", context),
            )
        })
        .collect();
    let statuses = array_field(map, "statuses", context)
        .iter()
        .map(|status| {
            let Json::String(token) = status else {
                panic!("{context}: statuses must be strings");
            };
            decode_status(token, context)
        })
        .collect();
    let source_variable_ids = array_field(map, "source_variable_ids", context)
        .iter()
        .map(|id| {
            let Json::String(id) = id else {
                panic!("{context}: source_variable_ids must be strings");
            };
            id.clone()
        })
        .collect();
    ExpectedOutcome {
        value: expected_value,
        findings,
        statuses,
        source_variable_ids,
    }
}

// ---------------------------------------------------------------------------
// Runner
// ---------------------------------------------------------------------------

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../fixtures/rule_expressions/conformance_corpus")
}

fn case_paths() -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = fs::read_dir(corpus_dir())
        .expect("conformance corpus directory must exist")
        .filter_map(|entry| entry.ok().map(|item| item.path()))
        .filter(|path| {
            path.is_file()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with("case_") && name.ends_with(".json"))
        })
        .collect();
    paths.sort();
    paths
}

#[test]
fn conformance_corpus_pins_the_frozen_grammar() {
    let paths = case_paths();
    assert!(
        paths.len() >= 60,
        "conformance corpus must not shrink (found {} cases)",
        paths.len()
    );

    let mut accept_features: BTreeSet<String> = BTreeSet::new();
    let mut blocked_codes: BTreeSet<String> = BTreeSet::new();
    let mut has_unsupported_grammar_case = false;
    let mut case_ids: BTreeSet<String> = BTreeSet::new();

    for path in &paths {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .expect("case file name")
            .to_string();
        let text = fs::read_to_string(path).expect("case file must be readable");
        let json = Reader::parse(&text);
        let case = as_object(&json, &file_name);
        let case_id = string_field(case, "case_id", &file_name);
        assert_eq!(
            format!("{case_id}.json"),
            file_name,
            "case_id must match the file name"
        );
        assert!(case_ids.insert(case_id.clone()), "duplicate case id");
        let context = case_id.as_str();

        let mut features: BTreeSet<String> = BTreeSet::new();
        let expression =
            decode_expression(field(case, "expression", context), context, &mut features);
        let bindings = array_field(case, "bindings", context)
            .iter()
            .map(|binding| {
                let binding = as_object(binding, context);
                let variable_id = string_field(binding, "variable_id", context);
                let source = decode_source(&string_field(binding, "source", context), context);
                match field(binding, "quantity", context) {
                    Json::Null => VariableBinding {
                        variable_id,
                        source,
                        quantity: None,
                    },
                    quantity => VariableBinding {
                        variable_id,
                        source,
                        quantity: Some(decode_quantity(quantity, context)),
                    },
                }
            })
            .collect();
        let required_variable_ids = array_field(case, "required_variable_ids", context)
            .iter()
            .map(|id| {
                let Json::String(id) = id else {
                    panic!("{context}: required_variable_ids must be strings");
                };
                id.clone()
            })
            .collect();
        let statuses = array_field(case, "statuses", context)
            .iter()
            .map(|status| {
                let Json::String(token) = status else {
                    panic!("{context}: statuses must be strings");
                };
                decode_status(token, context)
            })
            .collect();
        let declared_grammar_version = string_field(case, "declared_grammar_version", context);
        let expected = decode_expected(field(case, "expected", context), context);

        let input = EvaluationInput {
            expression,
            bindings,
            required_variable_ids,
            statuses,
            declared_grammar_version: declared_grammar_version.clone(),
        };

        let result = evaluate(&input);
        let rerun = evaluate(&input);
        assert_eq!(result, rerun, "{context}: evaluation must be deterministic");

        let actual_findings: Vec<(FindingCode, String)> = result
            .findings
            .iter()
            .map(|finding| (finding.code, finding.subject_id.clone()))
            .collect();
        assert_eq!(
            actual_findings, expected.findings,
            "{context}: findings (code, subject_id) mismatch"
        );
        assert_eq!(result.value, expected.value, "{context}: value mismatch");
        assert_eq!(
            result.statuses, expected.statuses,
            "{context}: statuses mismatch"
        );
        assert_eq!(
            result.source_variable_ids, expected.source_variable_ids,
            "{context}: source variable ids mismatch"
        );

        if expected.findings.is_empty() {
            assert_eq!(
                declared_grammar_version, GRAMMAR_VERSION,
                "{context}: accepting cases must declare the frozen grammar version"
            );
            accept_features.extend(features);
        } else {
            for (code, _) in &expected.findings {
                blocked_codes.insert(format!("{code:?}"));
                if *code == FindingCode::UnsupportedGrammarVersion {
                    has_unsupported_grammar_case = true;
                }
            }
        }
    }

    for feature in REQUIRED_ACCEPT_FEATURES {
        assert!(
            accept_features.contains(*feature),
            "coverage floor: no accepting case exercises '{feature}'"
        );
    }
    for code in REQUIRED_BLOCK_CODES {
        assert!(
            blocked_codes.contains(&format!("{code:?}")),
            "coverage floor: no blocking case produces {code:?}"
        );
    }
    assert!(
        has_unsupported_grammar_case,
        "coverage floor: corpus must pin grammar-version blocking"
    );
}

// ---------------------------------------------------------------------------
// Static type check parity (W-17, DEL-06-02 `Design/rule-pack-unit-check.md`)
// ---------------------------------------------------------------------------

/// The finding codes the static check can derive without values (design
/// §2.2, the set S).
const STATIC_CODES: &[FindingCode] = &[
    FindingCode::TypeMismatch,
    FindingCode::DimensionMismatch,
    FindingCode::UnitMismatch,
    FindingCode::UnsupportedExpressionForm,
    FindingCode::UnsafeConstruct,
    FindingCode::InvalidReference,
    FindingCode::MissingVariable,
    FindingCode::UnitMetadataMissing,
    FindingCode::TableMalformed,
];

/// Cases with a value-, binding- or status-dependent expected finding, or a
/// grammar-version finding: compared for soundness only (design §4).
const NON_COMPARABLE_CASES: &[u32] = &[30, 31, 32, 44, 45, 46, 47, 48, 58, 62, 63, 66];

struct StaticCase {
    case_id: String,
    number: u32,
    expression: Expression,
    bindings: Vec<VariableBinding>,
    required_variable_ids: Vec<String>,
    expected: Vec<(FindingCode, String)>,
}

fn load_static_cases() -> Vec<StaticCase> {
    case_paths()
        .iter()
        .map(|path| {
            let text = fs::read_to_string(path).expect("case file must be readable");
            let json = Reader::parse(&text);
            let case = as_object(&json, "case");
            let case_id = string_field(case, "case_id", "case");
            let context = case_id.as_str();
            let number: u32 = case_id
                .strip_prefix("case_")
                .and_then(|rest| rest.split('_').next())
                .and_then(|digits| digits.parse().ok())
                .unwrap_or_else(|| panic!("{context}: case id has no number"));
            let expression = decode_expression(
                field(case, "expression", context),
                context,
                &mut BTreeSet::new(),
            );
            let bindings = array_field(case, "bindings", context)
                .iter()
                .map(|binding| {
                    let binding = as_object(binding, context);
                    VariableBinding {
                        variable_id: string_field(binding, "variable_id", context),
                        source: decode_source(&string_field(binding, "source", context), context),
                        quantity: match field(binding, "quantity", context) {
                            Json::Null => None,
                            quantity => Some(decode_quantity(quantity, context)),
                        },
                    }
                })
                .collect();
            let required_variable_ids = array_field(case, "required_variable_ids", context)
                .iter()
                .map(|id| match id {
                    Json::String(id) => id.clone(),
                    _ => panic!("{context}: required_variable_ids must be strings"),
                })
                .collect();
            let expected = decode_expected(field(case, "expected", context), context).findings;
            StaticCase {
                case_id,
                number,
                expression,
                bindings,
                required_variable_ids,
                expected,
            }
        })
        .collect()
}

/// The corpus adapter (design §4). The type environment is built from all of
/// a case's bindings, in the evaluator's binding order (`build_binding_map`):
/// an empty id is an invalid reference; a repeated id keeps its first
/// binding; a null quantity is `Unknown`, not missing; a quantity without
/// its unit metadata reports `unit_metadata_missing` and is then unbound. Any
/// other AST variable is undeclared. `required_variable_ids` does not build
/// the environment; only its static check (an empty id) is taken from it.
/// The findings come back in the evaluator's order: bindings, required
/// variables, then the expression.
fn static_findings(case: &StaticCase) -> Vec<(FindingCode, String)> {
    let mut findings = Vec::new();
    let mut env = TypeEnv::new();
    let mut seen = BTreeSet::new();
    for binding in &case.bindings {
        if binding.variable_id.trim().is_empty() {
            findings.push((FindingCode::InvalidReference, "binding".to_string()));
            continue;
        }
        if !seen.insert(binding.variable_id.clone()) {
            continue;
        }
        match &binding.quantity {
            None => env.insert_unknown(&binding.variable_id),
            Some(quantity)
                if !(quantity.unit_required
                    && quantity.dimension_check_required
                    && !quantity.unit_ref.trim().is_empty()) =>
            {
                findings.push((FindingCode::UnitMetadataMissing, binding.variable_id.clone()));
            }
            Some(quantity) => {
                env.insert_typed(&binding.variable_id, quantity.dimension, &quantity.unit_ref)
            }
        }
    }
    for id in &case.required_variable_ids {
        if id.trim().is_empty() {
            findings.push((FindingCode::InvalidReference, "required_variable".to_string()));
        }
    }
    let check = check_expression_types(&case.expression, &env);
    assert_eq!(
        check,
        check_expression_types(&case.expression, &env),
        "{}: the static check must be deterministic",
        case.case_id
    );
    findings.extend(
        check
            .findings
            .into_iter()
            .map(|finding| (finding.code, finding.subject_id)),
    );
    findings
}

#[test]
fn static_type_check_is_sound_and_exact_on_comparable_corpus_cases() {
    let cases = load_static_cases();
    assert!(cases.len() >= 60, "corpus must not shrink");
    let mut non_comparable = Vec::new();
    for case in &cases {
        let actual = static_findings(case);

        // Soundness: every expected finding in S is derived statically, with
        // the same code and subject.
        for expected in case
            .expected
            .iter()
            .filter(|(code, _)| STATIC_CODES.contains(code))
        {
            assert!(
                actual.contains(expected),
                "{}: static check misses {expected:?}; static findings {actual:?}",
                case.case_id
            );
        }

        // Equality on comparable cases: every expected finding lies in S (or
        // there is none), and the static findings are exactly the expected ones.
        if case
            .expected
            .iter()
            .all(|(code, _)| STATIC_CODES.contains(code))
        {
            assert_eq!(
                actual, case.expected,
                "{}: static findings differ from the evaluator's",
                case.case_id
            );
        } else {
            non_comparable.push(case.number);
        }
    }
    assert_eq!(
        non_comparable, NON_COMPARABLE_CASES,
        "the cases compared for soundness only are those the design names"
    );
}

fn literal(value: f64, dimension: Dimension, unit_ref: &str) -> Expression {
    Expression::Literal(Quantity::new(value, dimension, unit_ref).expect("invented quantity"))
}

fn evaluated_findings(expression: &Expression) -> Vec<(FindingCode, String)> {
    evaluate(&EvaluationInput {
        expression: expression.clone(),
        bindings: Vec::new(),
        required_variable_ids: Vec::new(),
        statuses: vec![AnalysisStatus::MechanicsSolved],
        declared_grammar_version: GRAMMAR_VERSION.to_string(),
    })
    .findings
    .into_iter()
    .map(|finding| (finding.code, finding.subject_id))
    .collect()
}

fn checked_findings(expression: &Expression) -> Vec<(FindingCode, String)> {
    check_expression_types(expression, &TypeEnv::new())
        .findings
        .into_iter()
        .map(|finding| (finding.code, finding.subject_id))
        .collect()
}

#[test]
fn static_check_reports_the_quotient_form_behind_a_zero_divisor() {
    // Case 57's operands with a zero divisor: the evaluator tests the divisor
    // first, so it reports only the division by zero; statically the
    // quotient Stress / Time is unrepresentable on every run.
    let expression = Expression::Binary {
        operator: BinaryOperator::Divide,
        left: Box::new(literal(12.0, Dimension::Stress, "invented_stress_unit")),
        right: Box::new(literal(0.0, Dimension::Time, "invented_time_unit")),
    };
    assert_eq!(
        evaluated_findings(&expression),
        vec![(FindingCode::DivisionByZero, "divide".to_string())]
    );
    assert_eq!(
        checked_findings(&expression),
        vec![(
            FindingCode::UnsupportedExpressionForm,
            "divide".to_string()
        )]
    );
}

#[test]
fn static_check_reports_a_unit_mismatch_behind_a_table_range_failure() {
    // The left operand is out of the table's range and the right operand's
    // unit differs from the table result's: the evaluator stops at the range
    // finding; statically the sum's units never match.
    let table = UserTable {
        table_id: "invented_lookup_table".to_string(),
        argument_dimension: Dimension::Temperature,
        argument_unit_ref: "invented_temperature_unit".to_string(),
        result_dimension: Dimension::Stress,
        result_unit_ref: "invented_stress_unit".to_string(),
        rows: vec![
            TableRow {
                argument: 10.0,
                result: 1.5,
            },
            TableRow {
                argument: 20.0,
                result: 2.5,
            },
        ],
    };
    let expression = Expression::Binary {
        operator: BinaryOperator::Add,
        left: Box::new(Expression::Interpolate {
            table,
            argument: Box::new(literal(
                5.0,
                Dimension::Temperature,
                "invented_temperature_unit",
            )),
        }),
        right: Box::new(literal(1.0, Dimension::Stress, "invented_other_stress_unit")),
    };
    assert_eq!(
        evaluated_findings(&expression),
        vec![(
            FindingCode::TableOutOfRange,
            "invented_lookup_table".to_string()
        )]
    );
    assert_eq!(
        checked_findings(&expression),
        vec![(FindingCode::UnitMismatch, "add_subtract".to_string())]
    );
}

#[test]
fn static_check_gives_unknown_operands_no_finding() {
    // An Unknown variable makes its ancestors unknown without findings; a
    // typed sibling mismatch elsewhere is still reported.
    let mut env = TypeEnv::new();
    env.insert_unknown("invented_tbd_input");
    env.insert_typed("invented_stress_input", Dimension::Stress, "invented_stress_unit");
    let expression = Expression::Logical {
        operator: LogicalOperator::And,
        left: Box::new(Expression::Compare {
            operator: ComparisonOperator::LessThan,
            left: Box::new(Expression::VariableRef("invented_tbd_input".to_string())),
            right: Box::new(literal(1.0, Dimension::Length, "invented_length_unit")),
        }),
        right: Box::new(Expression::Compare {
            operator: ComparisonOperator::LessThan,
            left: Box::new(Expression::VariableRef("invented_stress_input".to_string())),
            right: Box::new(literal(1.0, Dimension::Length, "invented_length_unit")),
        }),
    };
    let check = check_expression_types(&expression, &env);
    let findings: Vec<(FindingCode, String, String)> = check
        .findings
        .iter()
        .map(|finding| (finding.code, finding.subject_id.clone(), finding.path.clone()))
        .collect();
    assert_eq!(
        findings,
        vec![(
            FindingCode::DimensionMismatch,
            "comparison".to_string(),
            "expression.right".to_string()
        )]
    );
}
