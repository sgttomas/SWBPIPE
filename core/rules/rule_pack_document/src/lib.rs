//! Rule-pack document seam (Phase C2 backend, DEC-022 basis).
//!
//! This crate is the production bridge between rule-pack JSON documents
//! (`schemas/rule_pack.schema.yaml`) and the frozen expression grammar
//! implemented in `open_pipe_stress_expression_evaluator`:
//!
//! - a codec between the node-tagged JSON expression encoding (pinned by
//!   `fixtures/rule_expressions/conformance_corpus/`) and the evaluator's
//!   typed AST;
//! - RFC 8785 (JCS) checksum computation over a pack document with the
//!   `checksums` member excluded, grammar-version-bound per DEC-022 through
//!   `open_pipe_stress_rule_pack_lifecycle`;
//! - document-level validation surfacing lifecycle, grammar, expression, and
//!   checksum findings for the desktop editor.
//!
//! It does not parse expression text (a writable text syntax awaits the
//! D-02b human ruling), does not store private payloads, does not choose
//! storage paths, and emits no professional, certification, sealing,
//! authentication, or code-compliance claims.

use serde::Serialize;
use serde_json::{Map, Value};

use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_expression_evaluator::{
    grammar_version_supported, AggregateFunction, BinaryOperator, ComparisonOperator, Dimension,
    Expression, LogicalOperator, LookupMode, Quantity, TableRow, UnaryOperator, UserTable,
    GRAMMAR_VERSION, SUPPORTED_GRAMMAR_VERSIONS,
};
use open_pipe_stress_rule_pack_lifecycle::{
    is_semver_grammar_version, validate_lifecycle, Canonicalization, ChecksumAlgorithm,
    ChecksumRecord, LifecycleStatus, LifecycleValidationInput, PayloadScope, PrivacyClass,
    ProfessionalBoundary, RedistributionStatus, ReviewStatus, RulePackLifecycleRecord,
};

/// Document member excluded from the canonical checksum payload. The
/// checksum cannot cover itself; excluding exactly this member keeps the
/// hash well-defined and is recorded in checksum evidence as
/// `payload_excludes`.
pub const CHECKSUM_PAYLOAD_EXCLUDES: &[&str] = &["checksums"];

/// Stable label for how rule-pack document checksums are computed here.
pub const CHECKSUM_BASIS: &str = "rfc8785_jcs_sha256_excluding_checksums";

// ---------------------------------------------------------------------------
// Dimension token mapping (snake_case document tokens <-> evaluator enum)
// ---------------------------------------------------------------------------

const DIMENSION_TOKENS: &[(&str, Dimension)] = &[
    ("dimensionless", Dimension::Dimensionless),
    ("length", Dimension::Length),
    ("mass", Dimension::Mass),
    ("time", Dimension::Time),
    ("temperature", Dimension::Temperature),
    ("temperature_interval", Dimension::TemperatureInterval),
    ("angle", Dimension::Angle),
    ("rotation", Dimension::Rotation),
    ("force", Dimension::Force),
    ("moment", Dimension::Moment),
    ("pressure", Dimension::Pressure),
    ("stress", Dimension::Stress),
    ("area", Dimension::Area),
    ("volume", Dimension::Volume),
    ("density", Dimension::Density),
    ("linear_stiffness", Dimension::LinearStiffness),
    ("rotational_stiffness", Dimension::RotationalStiffness),
    ("displacement", Dimension::Displacement),
    ("velocity", Dimension::Velocity),
    ("acceleration", Dimension::Acceleration),
    ("thermal_conductivity", Dimension::ThermalConductivity),
    ("specific_heat", Dimension::SpecificHeat),
    (
        "thermal_expansion_coefficient",
        Dimension::ThermalExpansionCoefficient,
    ),
    ("second_moment_area", Dimension::SecondMomentArea),
    ("section_modulus", Dimension::SectionModulus),
    ("mass_per_length", Dimension::MassPerLength),
    ("volume_per_length", Dimension::VolumePerLength),
    ("slope", Dimension::Slope),
    ("TBD", Dimension::Tbd),
];

/// Decode a snake_case dimension token from the document vocabulary.
pub fn decode_dimension(token: &str) -> Option<Dimension> {
    DIMENSION_TOKENS
        .iter()
        .find(|(name, _)| *name == token)
        .map(|(_, dimension)| *dimension)
}

/// Encode an evaluator dimension as its document token.
pub fn encode_dimension(dimension: Dimension) -> &'static str {
    DIMENSION_TOKENS
        .iter()
        .find(|(_, candidate)| *candidate == dimension)
        .map(|(name, _)| *name)
        .expect("every Dimension variant has a document token")
}

// ---------------------------------------------------------------------------
// Expression codec
// ---------------------------------------------------------------------------

/// Decode failure with a JSON-path-style breadcrumb for editor diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DecodeError {
    pub path: String,
    pub message: String,
}

impl DecodeError {
    fn new(path: &str, message: impl Into<String>) -> Self {
        Self {
            path: path.to_string(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}

impl std::error::Error for DecodeError {}

fn object<'a>(value: &'a Value, path: &str) -> Result<&'a Map<String, Value>, DecodeError> {
    value
        .as_object()
        .ok_or_else(|| DecodeError::new(path, "expected a JSON object"))
}

fn member<'a>(
    map: &'a Map<String, Value>,
    key: &str,
    path: &str,
) -> Result<&'a Value, DecodeError> {
    map.get(key)
        .ok_or_else(|| DecodeError::new(path, format!("missing required member '{key}'")))
}

fn string_member(map: &Map<String, Value>, key: &str, path: &str) -> Result<String, DecodeError> {
    member(map, key, path)?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| DecodeError::new(path, format!("member '{key}' must be a string")))
}

fn number_member(map: &Map<String, Value>, key: &str, path: &str) -> Result<f64, DecodeError> {
    member(map, key, path)?
        .as_f64()
        .ok_or_else(|| DecodeError::new(path, format!("member '{key}' must be a finite number")))
}

fn optional_bool(map: &Map<String, Value>, key: &str, default: bool) -> bool {
    map.get(key).and_then(Value::as_bool).unwrap_or(default)
}

fn decode_quantity(value: &Value, path: &str) -> Result<Quantity, DecodeError> {
    let map = object(value, path)?;
    let dimension_token = string_member(map, "dimension", path)?;
    let dimension = decode_dimension(&dimension_token).ok_or_else(|| {
        DecodeError::new(path, format!("unknown dimension token '{dimension_token}'"))
    })?;
    Ok(Quantity {
        value: number_member(map, "value", path)?,
        dimension,
        unit_ref: string_member(map, "unit_ref", path)?,
        unit_required: optional_bool(map, "unit_required", true),
        dimension_check_required: optional_bool(map, "dimension_check_required", true),
    })
}

fn decode_table(value: &Value, path: &str) -> Result<UserTable, DecodeError> {
    let map = object(value, path)?;
    let argument_dimension_token = string_member(map, "argument_dimension", path)?;
    let result_dimension_token = string_member(map, "result_dimension", path)?;
    let argument_dimension = decode_dimension(&argument_dimension_token).ok_or_else(|| {
        DecodeError::new(
            path,
            format!("unknown argument dimension token '{argument_dimension_token}'"),
        )
    })?;
    let result_dimension = decode_dimension(&result_dimension_token).ok_or_else(|| {
        DecodeError::new(
            path,
            format!("unknown result dimension token '{result_dimension_token}'"),
        )
    })?;
    let rows_value = member(map, "rows", path)?;
    let rows_array = rows_value
        .as_array()
        .ok_or_else(|| DecodeError::new(path, "member 'rows' must be an array"))?;
    let mut rows = Vec::with_capacity(rows_array.len());
    for (index, row) in rows_array.iter().enumerate() {
        let row_path = format!("{path}.rows[{index}]");
        let row_map = object(row, &row_path)?;
        rows.push(TableRow {
            argument: number_member(row_map, "argument", &row_path)?,
            result: number_member(row_map, "result", &row_path)?,
        });
    }
    Ok(UserTable {
        table_id: string_member(map, "table_id", path)?,
        argument_dimension,
        argument_unit_ref: string_member(map, "argument_unit_ref", path)?,
        result_dimension,
        result_unit_ref: string_member(map, "result_unit_ref", path)?,
        rows,
    })
}

/// Decode a node-tagged JSON expression into the evaluator AST.
///
/// The full corpus encoding is accepted — including the evaluator refusal
/// markers `unsupported_form` / `unsafe_host_access`, which always evaluate
/// to blocking findings. Pack-document validation additionally rejects
/// refusal markers as authorable content (see
/// [`expression_contains_refusal_marker`]); the schema does the same.
pub fn decode_expression(value: &Value) -> Result<Expression, DecodeError> {
    decode_expression_at(value, "expression")
}

fn decode_expression_at(value: &Value, path: &str) -> Result<Expression, DecodeError> {
    let map = object(value, path)?;
    let node = string_member(map, "node", path)?;
    match node.as_str() {
        "literal" => Ok(Expression::Literal(decode_quantity(
            member(map, "quantity", path)?,
            &format!("{path}.quantity"),
        )?)),
        "variable_ref" => Ok(Expression::VariableRef(string_member(
            map,
            "variable_id",
            path,
        )?)),
        "unary" => {
            let operator = match string_member(map, "operator", path)?.as_str() {
                "negate" => UnaryOperator::Negate,
                "abs" => UnaryOperator::Abs,
                "not" => UnaryOperator::Not,
                other => {
                    return Err(DecodeError::new(
                        path,
                        format!("unknown unary operator '{other}'"),
                    ))
                }
            };
            Ok(Expression::Unary {
                operator,
                operand: Box::new(decode_expression_at(
                    member(map, "operand", path)?,
                    &format!("{path}.operand"),
                )?),
            })
        }
        "binary" => {
            let operator = match string_member(map, "operator", path)?.as_str() {
                "add" => BinaryOperator::Add,
                "subtract" => BinaryOperator::Subtract,
                "multiply" => BinaryOperator::Multiply,
                "divide" => BinaryOperator::Divide,
                other => {
                    return Err(DecodeError::new(
                        path,
                        format!("unknown binary operator '{other}'"),
                    ))
                }
            };
            Ok(Expression::Binary {
                operator,
                left: Box::new(decode_expression_at(
                    member(map, "left", path)?,
                    &format!("{path}.left"),
                )?),
                right: Box::new(decode_expression_at(
                    member(map, "right", path)?,
                    &format!("{path}.right"),
                )?),
            })
        }
        "compare" => {
            let operator = match string_member(map, "operator", path)?.as_str() {
                "less_than" => ComparisonOperator::LessThan,
                "less_than_or_equal" => ComparisonOperator::LessThanOrEqual,
                "greater_than" => ComparisonOperator::GreaterThan,
                "greater_than_or_equal" => ComparisonOperator::GreaterThanOrEqual,
                "equal" => ComparisonOperator::Equal,
                "not_equal" => ComparisonOperator::NotEqual,
                other => {
                    return Err(DecodeError::new(
                        path,
                        format!("unknown comparison operator '{other}'"),
                    ))
                }
            };
            Ok(Expression::Compare {
                operator,
                left: Box::new(decode_expression_at(
                    member(map, "left", path)?,
                    &format!("{path}.left"),
                )?),
                right: Box::new(decode_expression_at(
                    member(map, "right", path)?,
                    &format!("{path}.right"),
                )?),
            })
        }
        "logical" => {
            let operator = match string_member(map, "operator", path)?.as_str() {
                "and" => LogicalOperator::And,
                "or" => LogicalOperator::Or,
                other => {
                    return Err(DecodeError::new(
                        path,
                        format!("unknown logical operator '{other}'"),
                    ))
                }
            };
            Ok(Expression::Logical {
                operator,
                left: Box::new(decode_expression_at(
                    member(map, "left", path)?,
                    &format!("{path}.left"),
                )?),
                right: Box::new(decode_expression_at(
                    member(map, "right", path)?,
                    &format!("{path}.right"),
                )?),
            })
        }
        "select" => Ok(Expression::Select {
            condition: Box::new(decode_expression_at(
                member(map, "condition", path)?,
                &format!("{path}.condition"),
            )?),
            then_branch: Box::new(decode_expression_at(
                member(map, "then", path)?,
                &format!("{path}.then"),
            )?),
            else_branch: Box::new(decode_expression_at(
                member(map, "else", path)?,
                &format!("{path}.else"),
            )?),
        }),
        "aggregate" => {
            let function = match string_member(map, "function", path)?.as_str() {
                "min" => AggregateFunction::Min,
                "max" => AggregateFunction::Max,
                other => {
                    return Err(DecodeError::new(
                        path,
                        format!("unknown aggregate function '{other}'"),
                    ))
                }
            };
            let operands_value = member(map, "operands", path)?;
            let operands_array = operands_value
                .as_array()
                .ok_or_else(|| DecodeError::new(path, "member 'operands' must be an array"))?;
            let mut operands = Vec::with_capacity(operands_array.len());
            for (index, operand) in operands_array.iter().enumerate() {
                operands.push(decode_expression_at(
                    operand,
                    &format!("{path}.operands[{index}]"),
                )?);
            }
            Ok(Expression::Aggregate { function, operands })
        }
        "interpolate" => Ok(Expression::Interpolate {
            table: decode_table(member(map, "table", path)?, &format!("{path}.table"))?,
            argument: Box::new(decode_expression_at(
                member(map, "argument", path)?,
                &format!("{path}.argument"),
            )?),
        }),
        "lookup" => {
            let mode = match string_member(map, "mode", path)?.as_str() {
                "exact" => LookupMode::Exact,
                "step" => LookupMode::Step,
                other => {
                    return Err(DecodeError::new(
                        path,
                        format!("unknown lookup mode '{other}'"),
                    ))
                }
            };
            Ok(Expression::Lookup {
                table: decode_table(member(map, "table", path)?, &format!("{path}.table"))?,
                mode,
                argument: Box::new(decode_expression_at(
                    member(map, "argument", path)?,
                    &format!("{path}.argument"),
                )?),
            })
        }
        "unsupported_form" => Ok(Expression::UnsupportedForm {
            form_id: string_member(map, "form_id", path)?,
        }),
        "unsafe_host_access" => Ok(Expression::UnsafeHostAccess {
            request: string_member(map, "request", path)?,
        }),
        other => Err(DecodeError::new(
            path,
            format!("unknown expression node kind '{other}'"),
        )),
    }
}

fn quantity_value(quantity: &Quantity) -> Value {
    let mut map = Map::new();
    map.insert(
        "value".to_string(),
        serde_json::Number::from_f64(quantity.value)
            .map(Value::Number)
            .unwrap_or(Value::Null),
    );
    map.insert(
        "dimension".to_string(),
        Value::String(encode_dimension(quantity.dimension).to_string()),
    );
    map.insert(
        "unit_ref".to_string(),
        Value::String(quantity.unit_ref.clone()),
    );
    if !quantity.unit_required {
        map.insert("unit_required".to_string(), Value::Bool(false));
    }
    if !quantity.dimension_check_required {
        map.insert("dimension_check_required".to_string(), Value::Bool(false));
    }
    Value::Object(map)
}

fn table_value(table: &UserTable) -> Value {
    let mut map = Map::new();
    map.insert(
        "table_id".to_string(),
        Value::String(table.table_id.clone()),
    );
    map.insert(
        "argument_dimension".to_string(),
        Value::String(encode_dimension(table.argument_dimension).to_string()),
    );
    map.insert(
        "argument_unit_ref".to_string(),
        Value::String(table.argument_unit_ref.clone()),
    );
    map.insert(
        "result_dimension".to_string(),
        Value::String(encode_dimension(table.result_dimension).to_string()),
    );
    map.insert(
        "result_unit_ref".to_string(),
        Value::String(table.result_unit_ref.clone()),
    );
    map.insert(
        "rows".to_string(),
        Value::Array(
            table
                .rows
                .iter()
                .map(|row| {
                    let mut row_map = Map::new();
                    row_map.insert(
                        "argument".to_string(),
                        serde_json::Number::from_f64(row.argument)
                            .map(Value::Number)
                            .unwrap_or(Value::Null),
                    );
                    row_map.insert(
                        "result".to_string(),
                        serde_json::Number::from_f64(row.result)
                            .map(Value::Number)
                            .unwrap_or(Value::Null),
                    );
                    Value::Object(row_map)
                })
                .collect(),
        ),
    );
    Value::Object(map)
}

fn node_value(kind: &str, members: Vec<(&str, Value)>) -> Value {
    let mut map = Map::new();
    map.insert("node".to_string(), Value::String(kind.to_string()));
    for (key, value) in members {
        map.insert(key.to_string(), value);
    }
    Value::Object(map)
}

/// Encode an evaluator AST into the node-tagged JSON encoding.
pub fn encode_expression(expression: &Expression) -> Value {
    match expression {
        Expression::Literal(quantity) => {
            node_value("literal", vec![("quantity", quantity_value(quantity))])
        }
        Expression::VariableRef(variable_id) => node_value(
            "variable_ref",
            vec![("variable_id", Value::String(variable_id.clone()))],
        ),
        Expression::Unary { operator, operand } => node_value(
            "unary",
            vec![
                (
                    "operator",
                    Value::String(
                        match operator {
                            UnaryOperator::Negate => "negate",
                            UnaryOperator::Abs => "abs",
                            UnaryOperator::Not => "not",
                        }
                        .to_string(),
                    ),
                ),
                ("operand", encode_expression(operand)),
            ],
        ),
        Expression::Binary {
            operator,
            left,
            right,
        } => node_value(
            "binary",
            vec![
                (
                    "operator",
                    Value::String(
                        match operator {
                            BinaryOperator::Add => "add",
                            BinaryOperator::Subtract => "subtract",
                            BinaryOperator::Multiply => "multiply",
                            BinaryOperator::Divide => "divide",
                        }
                        .to_string(),
                    ),
                ),
                ("left", encode_expression(left)),
                ("right", encode_expression(right)),
            ],
        ),
        Expression::Compare {
            operator,
            left,
            right,
        } => node_value(
            "compare",
            vec![
                (
                    "operator",
                    Value::String(
                        match operator {
                            ComparisonOperator::LessThan => "less_than",
                            ComparisonOperator::LessThanOrEqual => "less_than_or_equal",
                            ComparisonOperator::GreaterThan => "greater_than",
                            ComparisonOperator::GreaterThanOrEqual => "greater_than_or_equal",
                            ComparisonOperator::Equal => "equal",
                            ComparisonOperator::NotEqual => "not_equal",
                        }
                        .to_string(),
                    ),
                ),
                ("left", encode_expression(left)),
                ("right", encode_expression(right)),
            ],
        ),
        Expression::Logical {
            operator,
            left,
            right,
        } => node_value(
            "logical",
            vec![
                (
                    "operator",
                    Value::String(
                        match operator {
                            LogicalOperator::And => "and",
                            LogicalOperator::Or => "or",
                        }
                        .to_string(),
                    ),
                ),
                ("left", encode_expression(left)),
                ("right", encode_expression(right)),
            ],
        ),
        Expression::Select {
            condition,
            then_branch,
            else_branch,
        } => node_value(
            "select",
            vec![
                ("condition", encode_expression(condition)),
                ("then", encode_expression(then_branch)),
                ("else", encode_expression(else_branch)),
            ],
        ),
        Expression::Aggregate { function, operands } => node_value(
            "aggregate",
            vec![
                (
                    "function",
                    Value::String(
                        match function {
                            AggregateFunction::Min => "min",
                            AggregateFunction::Max => "max",
                        }
                        .to_string(),
                    ),
                ),
                (
                    "operands",
                    Value::Array(operands.iter().map(encode_expression).collect()),
                ),
            ],
        ),
        Expression::Interpolate { table, argument } => node_value(
            "interpolate",
            vec![
                ("table", table_value(table)),
                ("argument", encode_expression(argument)),
            ],
        ),
        Expression::Lookup {
            table,
            mode,
            argument,
        } => node_value(
            "lookup",
            vec![
                ("table", table_value(table)),
                (
                    "mode",
                    Value::String(
                        match mode {
                            LookupMode::Exact => "exact",
                            LookupMode::Step => "step",
                        }
                        .to_string(),
                    ),
                ),
                ("argument", encode_expression(argument)),
            ],
        ),
        Expression::UnsupportedForm { form_id } => node_value(
            "unsupported_form",
            vec![("form_id", Value::String(form_id.clone()))],
        ),
        Expression::UnsafeHostAccess { request } => node_value(
            "unsafe_host_access",
            vec![("request", Value::String(request.clone()))],
        ),
    }
}

/// True when the expression tree contains an evaluator refusal marker.
/// Refusal markers are engine vocabulary, never authorable pack content.
pub fn expression_contains_refusal_marker(expression: &Expression) -> bool {
    match expression {
        Expression::UnsupportedForm { .. } | Expression::UnsafeHostAccess { .. } => true,
        Expression::Literal(_) | Expression::VariableRef(_) => false,
        Expression::Unary { operand, .. } => expression_contains_refusal_marker(operand),
        Expression::Binary { left, right, .. }
        | Expression::Compare { left, right, .. }
        | Expression::Logical { left, right, .. } => {
            expression_contains_refusal_marker(left) || expression_contains_refusal_marker(right)
        }
        Expression::Select {
            condition,
            then_branch,
            else_branch,
        } => {
            expression_contains_refusal_marker(condition)
                || expression_contains_refusal_marker(then_branch)
                || expression_contains_refusal_marker(else_branch)
        }
        Expression::Aggregate { operands, .. } => {
            operands.iter().any(expression_contains_refusal_marker)
        }
        Expression::Interpolate { argument, .. } | Expression::Lookup { argument, .. } => {
            expression_contains_refusal_marker(argument)
        }
    }
}

// ---------------------------------------------------------------------------
// Document checksum (JCS over the document minus `checksums`)
// ---------------------------------------------------------------------------

/// Document-level failure that prevents checksum computation or validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DocumentError {
    pub subject: String,
    pub message: String,
}

impl DocumentError {
    fn new(subject: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for DocumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.subject, self.message)
    }
}

impl std::error::Error for DocumentError {}

fn document_string(document: &Value, pointer: &str) -> Option<String> {
    document
        .pointer(pointer)
        .and_then(Value::as_str)
        .map(str::to_string)
}

/// The canonical (RFC 8785) byte payload a rule-pack checksum covers: the
/// document with the members in [`CHECKSUM_PAYLOAD_EXCLUDES`] removed.
pub fn canonical_payload_bytes(document: &Value) -> Result<Vec<u8>, DocumentError> {
    let map = document.as_object().ok_or_else(|| {
        DocumentError::new("document", "rule-pack document must be a JSON object")
    })?;
    let mut payload = map.clone();
    for excluded in CHECKSUM_PAYLOAD_EXCLUDES {
        payload.remove(*excluded);
    }
    Ok(canonical_json(&Value::Object(payload)).into_bytes())
}

/// Computed rule-pack checksum evidence, serializable for command envelopes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ComputedRulePackChecksum {
    pub algorithm: String,
    pub canonicalization: String,
    pub payload_scope: String,
    pub payload_ref: String,
    pub payload_excludes: Vec<String>,
    pub basis: String,
    pub grammar_version: String,
    pub grammar_version_bound: bool,
    pub value: String,
}

/// Compute the grammar-version-bound JCS checksum of a rule-pack document.
///
/// The canonicalization label is `JCS` (not the lifecycle crate's
/// caller-supplied marker) because this function produces the canonical
/// bytes itself via `open_pipe_stress_canonical_json` (RFC 8785).
pub fn compute_rule_pack_checksum(
    document: &Value,
) -> Result<ComputedRulePackChecksum, DocumentError> {
    let rule_pack_id = document_string(document, "/metadata/rule_pack_id")
        .ok_or_else(|| DocumentError::new("metadata.rule_pack_id", "missing rule-pack identity"))?;
    let grammar_version = document_string(document, "/grammar_version").ok_or_else(|| {
        DocumentError::new(
            "grammar_version",
            "rule-pack document must declare a top-level grammar_version (DEC-022)",
        )
    })?;
    let payload = canonical_payload_bytes(document)?;
    let record = ChecksumRecord::sha256_caller_supplied_jcs_bytes_with_grammar_version(
        PayloadScope::RulePackPayload,
        rule_pack_id.clone(),
        &payload,
        &grammar_version,
    )
    .map_err(|error| DocumentError::new("grammar_version", error.to_string()))?;
    Ok(ComputedRulePackChecksum {
        algorithm: "sha256".to_string(),
        canonicalization: "JCS".to_string(),
        payload_scope: "rule_pack_payload".to_string(),
        payload_ref: rule_pack_id,
        payload_excludes: CHECKSUM_PAYLOAD_EXCLUDES
            .iter()
            .map(|member| member.to_string())
            .collect(),
        basis: CHECKSUM_BASIS.to_string(),
        grammar_version,
        grammar_version_bound: true,
        value: record.value,
    })
}

// ---------------------------------------------------------------------------
// Document-level validation
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationFinding {
    pub code: String,
    pub subject: String,
    pub message: String,
    pub blocking: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FormulaValidation {
    pub formula_id: String,
    pub declaration_form: String,
    pub decode_status: String,
    pub findings: Vec<ValidationFinding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChecksumValidation {
    pub declared_value: Option<String>,
    pub computed: Option<ComputedRulePackChecksum>,
    /// `match` | `mismatch_review_required` | `declared_placeholder` |
    /// `not_computable`
    pub match_status: String,
}

/// Document validation envelope for the desktop editor. Statuses describe
/// software findings only; nothing here is a professional, certification,
/// sealing, authentication, or code-compliance claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RulePackDocumentValidation {
    pub document_kind: String,
    pub rule_pack_id: String,
    pub rule_pack_name: String,
    pub rule_pack_version: String,
    pub schema_version: String,
    pub grammar_version: String,
    pub grammar_version_supported: bool,
    pub supported_grammar_versions: Vec<String>,
    pub evaluator_grammar_version: String,
    pub document_findings: Vec<ValidationFinding>,
    pub lifecycle_findings: Vec<ValidationFinding>,
    pub lifecycle_blocked: bool,
    pub formulas: Vec<FormulaValidation>,
    pub checksum: ChecksumValidation,
    pub publicly_exportable: bool,
    pub professional_boundary_notice: String,
    pub has_blocking_findings: bool,
}

fn map_privacy_class(token: &str) -> PrivacyClass {
    match token {
        "public_schema_only" => PrivacyClass::PublicSchemaOnly,
        "public_invented_example" => PrivacyClass::PublicInventedExample,
        "private_user_data" => PrivacyClass::PrivateUserData,
        "private_project_data" => PrivacyClass::PrivateProjectData,
        "quarantined" => PrivacyClass::Quarantined,
        _ => PrivacyClass::Tbd,
    }
}

fn map_redistribution_status(token: &str) -> RedistributionStatus {
    match token {
        "public_permissive" => RedistributionStatus::PublicPermissive,
        "private_only" => RedistributionStatus::PrivateOnly,
        "unknown" => RedistributionStatus::Unknown,
        "protected_suspected" => RedistributionStatus::ProtectedSuspected,
        "invented_non_engineering_example" => RedistributionStatus::InventedNonEngineeringExample,
        _ => RedistributionStatus::Tbd,
    }
}

fn map_review_status(token: &str) -> ReviewStatus {
    match token {
        "pending" => ReviewStatus::Pending,
        "accepted" => ReviewStatus::Accepted,
        "rejected" => ReviewStatus::Rejected,
        "quarantined" => ReviewStatus::Quarantined,
        _ => ReviewStatus::Tbd,
    }
}

// The schema's lifecycle_status vocabulary (draft, review_pending,
// accepted_private, accepted_public_example, rejected, quarantined, TBD) is
// wider than the crate enum; `validate_lifecycle` does not branch on this
// field, so unrepresentable tokens map to Tbd rather than being interpreted.
fn map_lifecycle_status(token: &str) -> LifecycleStatus {
    match token {
        "draft" => LifecycleStatus::Draft,
        "accepted_public_example" => LifecycleStatus::AcceptedPublicExample,
        "quarantined" => LifecycleStatus::Quarantined,
        _ => LifecycleStatus::Tbd,
    }
}

fn lifecycle_finding_code(
    code: open_pipe_stress_rule_pack_lifecycle::LifecycleFindingCode,
) -> &'static str {
    use open_pipe_stress_rule_pack_lifecycle::LifecycleFindingCode as Code;
    match code {
        Code::MissingRulePackIdentity => "MISSING_RULE_PACK_IDENTITY",
        Code::MissingVersion => "MISSING_VERSION",
        Code::MissingGrammarVersion => "MISSING_GRAMMAR_VERSION",
        Code::GrammarVersionMalformed => "GRAMMAR_VERSION_MALFORMED",
        Code::GrammarVersionNotBound => "GRAMMAR_VERSION_NOT_BOUND",
        Code::MissingSourceNotice => "MISSING_SOURCE_NOTICE",
        Code::MissingRedistributionStatus => "MISSING_REDISTRIBUTION_STATUS",
        Code::MissingReviewStatus => "MISSING_REVIEW_STATUS",
        Code::MissingChecksum => "MISSING_CHECKSUM",
        Code::StaleChecksum => "STALE_CHECKSUM",
        Code::ProtectedContentSuspected => "PROTECTED_CONTENT_SUSPECTED",
        Code::PrivateExportBlocked => "PRIVATE_EXPORT_BLOCKED",
        Code::ProfessionalBoundaryViolation => "PROFESSIONAL_BOUNDARY_VIOLATION",
    }
}

fn is_placeholder_checksum(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.is_empty() || trimmed.to_ascii_uppercase().starts_with("TBD")
}

/// Validate a rule-pack document: shape-critical members, grammar version,
/// every declarative-AST formula payload, lifecycle/privacy/provenance
/// findings via `open_pipe_stress_rule_pack_lifecycle`, and the recorded
/// checksum against a fresh JCS computation.
pub fn validate_rule_pack_document(
    document: &Value,
    public_export_requested: bool,
) -> RulePackDocumentValidation {
    let mut document_findings = Vec::new();

    let rule_pack_id = document_string(document, "/metadata/rule_pack_id").unwrap_or_default();
    let rule_pack_name = document_string(document, "/metadata/name").unwrap_or_default();
    let rule_pack_version =
        document_string(document, "/metadata/rule_pack_version").unwrap_or_default();
    let schema_version = document_string(document, "/schema_version").unwrap_or_default();
    let grammar_version = document_string(document, "/grammar_version").unwrap_or_default();

    if document_string(document, "/rule_pack_kind").as_deref() != Some("open_pipe_stress_rule_pack")
    {
        document_findings.push(ValidationFinding {
            code: "UNEXPECTED_DOCUMENT_KIND".to_string(),
            subject: "rule_pack_kind".to_string(),
            message: "document must declare rule_pack_kind 'open_pipe_stress_rule_pack'"
                .to_string(),
            blocking: true,
        });
    }

    let grammar_supported = is_semver_grammar_version(grammar_version.trim())
        && grammar_version_supported(&grammar_version);
    if grammar_version.trim().is_empty() {
        document_findings.push(ValidationFinding {
            code: "MISSING_GRAMMAR_VERSION".to_string(),
            subject: "grammar_version".to_string(),
            message: "document must declare a top-level grammar_version (DEC-022)".to_string(),
            blocking: true,
        });
    } else if !grammar_supported {
        document_findings.push(ValidationFinding {
            code: "UNSUPPORTED_GRAMMAR_VERSION".to_string(),
            subject: "grammar_version".to_string(),
            message: format!(
                "declared grammar version '{}' is not in the evaluator's supported set {:?}; \
                 evaluation would block (never best-effort)",
                grammar_version, SUPPORTED_GRAMMAR_VERSIONS
            ),
            blocking: true,
        });
    }

    // Formula payload validation (declarative-AST formulas only; other
    // declaration forms carry no executable payload by construction).
    let mut formulas = Vec::new();
    if let Some(declarations) = document
        .pointer("/formula_declarations")
        .and_then(Value::as_array)
    {
        for (index, declaration) in declarations.iter().enumerate() {
            let formula_id = declaration
                .pointer("/formula_id")
                .and_then(Value::as_str)
                .unwrap_or("unidentified_formula")
                .to_string();
            let declaration_form = declaration
                .pointer("/declaration_form")
                .and_then(Value::as_str)
                .unwrap_or("TBD")
                .to_string();
            let mut findings = Vec::new();
            let mut decode_status = "not_applicable".to_string();
            if declaration_form == "declarative_ast" {
                match declaration.pointer("/declaration_payload/expression_ast") {
                    Some(ast_value) => match decode_expression(ast_value) {
                        Ok(expression) => {
                            decode_status = "decoded".to_string();
                            if expression_contains_refusal_marker(&expression) {
                                findings.push(ValidationFinding {
                                    code: "REFUSAL_MARKER_AUTHORED".to_string(),
                                    subject: formula_id.clone(),
                                    message: "expression contains an evaluator refusal marker \
                                              (unsupported_form/unsafe_host_access); these are \
                                              engine vocabulary, not authorable pack content"
                                        .to_string(),
                                    blocking: true,
                                });
                            }
                        }
                        Err(error) => {
                            decode_status = "decode_failed".to_string();
                            findings.push(ValidationFinding {
                                code: "EXPRESSION_DECODE_FAILED".to_string(),
                                subject: format!("formula_declarations[{index}].{}", error.path),
                                message: error.message,
                                blocking: true,
                            });
                        }
                    },
                    None => {
                        decode_status = "missing_expression_ast".to_string();
                        findings.push(ValidationFinding {
                            code: "MISSING_EXPRESSION_AST".to_string(),
                            subject: formula_id.clone(),
                            message: "declarative_ast formulas must carry \
                                      declaration_payload.expression_ast"
                                .to_string(),
                            blocking: true,
                        });
                    }
                }
            }
            formulas.push(FormulaValidation {
                formula_id,
                declaration_form,
                decode_status,
                findings,
            });
        }
    } else {
        document_findings.push(ValidationFinding {
            code: "MISSING_FORMULA_DECLARATIONS".to_string(),
            subject: "formula_declarations".to_string(),
            message: "document must declare formula_declarations".to_string(),
            blocking: true,
        });
    }

    // Checksum: recompute and compare with the declared value.
    let declared_value = document_string(document, "/checksums/rule_pack_checksum/value");
    let computed = compute_rule_pack_checksum(document).ok();
    let match_status = match (&declared_value, &computed) {
        (Some(declared), Some(computed)) if is_placeholder_checksum(declared) => {
            let _ = computed;
            "declared_placeholder".to_string()
        }
        (Some(declared), Some(computed)) => {
            if declared.trim() == computed.value {
                "match".to_string()
            } else {
                "mismatch_review_required".to_string()
            }
        }
        _ => "not_computable".to_string(),
    };
    if match_status == "mismatch_review_required" {
        document_findings.push(ValidationFinding {
            code: "CHECKSUM_MISMATCH".to_string(),
            subject: "checksums.rule_pack_checksum".to_string(),
            message: "declared rule_pack_checksum does not match the recomputed JCS checksum \
                      of the document (excluding the checksums member)"
                .to_string(),
            blocking: true,
        });
    }

    // Lifecycle validation through the DEL-06-04 crate.
    let privacy_class = map_privacy_class(
        document_string(document, "/classification/privacy_class")
            .unwrap_or_default()
            .as_str(),
    );
    let redistribution_status = map_redistribution_status(
        document_string(document, "/classification/redistribution_status")
            .unwrap_or_default()
            .as_str(),
    );
    let review_status = map_review_status(
        document_string(document, "/provenance/review_status")
            .unwrap_or_default()
            .as_str(),
    );
    let protected_content_suspected = matches!(
        document_string(document, "/provenance/protected_content_review").as_deref(),
        Some("quarantined_protected_suspected")
    ) || redistribution_status
        == RedistributionStatus::ProtectedSuspected;

    let boundary = ProfessionalBoundary {
        software_makes_compliance_claim: document
            .pointer("/professional_boundary/software_makes_compliance_claim")
            .and_then(Value::as_bool)
            .unwrap_or(true),
        software_makes_certification_claim: document
            .pointer("/professional_boundary/software_makes_certification_claim")
            .and_then(Value::as_bool)
            .unwrap_or(true),
        software_makes_sealing_claim: document
            .pointer("/professional_boundary/software_makes_sealing_claim")
            .and_then(Value::as_bool)
            .unwrap_or(true),
        human_review_required: document
            .pointer("/professional_boundary/human_review_required")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        human_acceptance_record_software_generated: document
            .pointer("/professional_boundary/human_acceptance_record_software_generated")
            .and_then(Value::as_bool)
            .unwrap_or(true),
    };

    let crate_checksum = declared_value
        .as_deref()
        .filter(|value| !is_placeholder_checksum(value))
        .map(|value| ChecksumRecord {
            algorithm: ChecksumAlgorithm::Sha256,
            canonicalization: Canonicalization::CallerSuppliedJcsBytesUnverified,
            payload_scope: PayloadScope::RulePackPayload,
            payload_ref: rule_pack_id.clone(),
            value: value.trim().to_string(),
        });
    let expected_checksum = computed.as_ref().map(|computed| ChecksumRecord {
        algorithm: ChecksumAlgorithm::Sha256,
        canonicalization: Canonicalization::CallerSuppliedJcsBytesUnverified,
        payload_scope: PayloadScope::RulePackPayload,
        payload_ref: rule_pack_id.clone(),
        value: computed.value.clone(),
    });

    let record = RulePackLifecycleRecord {
        rule_pack_id: rule_pack_id.clone(),
        name: rule_pack_name.clone(),
        schema_version: schema_version.clone(),
        rule_pack_version: rule_pack_version.clone(),
        grammar_version: grammar_version.clone(),
        lifecycle_status: map_lifecycle_status(
            document_string(document, "/metadata/lifecycle_status")
                .unwrap_or_default()
                .as_str(),
        ),
        source_notice: document_string(document, "/metadata/source_notice").unwrap_or_default(),
        privacy_class,
        redistribution_status,
        review_status,
        protected_content_review_required: document
            .pointer("/classification/protected_content_review_required")
            .and_then(Value::as_bool)
            .unwrap_or(true),
        protected_content_suspected,
        checksum: crate_checksum,
        professional_boundary: boundary,
    };

    let canonical_payload = canonical_payload_bytes(document).ok();
    let lifecycle_result = validate_lifecycle(&LifecycleValidationInput {
        record: &record,
        expected_checksum: expected_checksum.as_ref(),
        canonical_payload: canonical_payload.as_deref(),
        public_export_requested,
    });
    let lifecycle_blocked = lifecycle_result.is_blocked();
    let lifecycle_findings: Vec<ValidationFinding> = lifecycle_result
        .findings
        .iter()
        .map(|finding| ValidationFinding {
            code: lifecycle_finding_code(finding.code).to_string(),
            subject: finding.subject_id.clone(),
            message: finding.message.clone(),
            blocking: true,
        })
        .collect();

    let has_blocking_findings = document_findings.iter().any(|finding| finding.blocking)
        || lifecycle_findings.iter().any(|finding| finding.blocking)
        || formulas
            .iter()
            .any(|formula| formula.findings.iter().any(|finding| finding.blocking));

    RulePackDocumentValidation {
        document_kind: "openpipestress.rule_pack.document_validation".to_string(),
        rule_pack_id,
        rule_pack_name,
        rule_pack_version,
        schema_version,
        grammar_version,
        grammar_version_supported: grammar_supported,
        supported_grammar_versions: SUPPORTED_GRAMMAR_VERSIONS
            .iter()
            .map(|version| version.to_string())
            .collect(),
        evaluator_grammar_version: GRAMMAR_VERSION.to_string(),
        document_findings,
        lifecycle_findings,
        lifecycle_blocked,
        formulas,
        checksum: ChecksumValidation {
            declared_value,
            computed,
            match_status,
        },
        publicly_exportable: record.is_publicly_exportable(),
        professional_boundary_notice:
            "Document validation results are engineering decision-support information \
             computed from user-supplied rule-pack content; acceptance and professional \
             judgment remain with the responsible engineer. Human review remains required."
                .to_string(),
        has_blocking_findings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn invented_quantity(value: f64) -> Value {
        json!({
            "value": value,
            "dimension": "stress",
            "unit_ref": "invented_stress_unit"
        })
    }

    #[test]
    fn decodes_and_encodes_every_node_kind_round_trip() {
        let ast = json!({
            "node": "select",
            "condition": {
                "node": "logical",
                "operator": "and",
                "left": {
                    "node": "compare",
                    "operator": "greater_than_or_equal",
                    "left": {"node": "variable_ref", "variable_id": "invented_a"},
                    "right": {"node": "literal", "quantity": invented_quantity(10.0)}
                },
                "right": {
                    "node": "unary",
                    "operator": "not",
                    "operand": {
                        "node": "compare",
                        "operator": "equal",
                        "left": {"node": "variable_ref", "variable_id": "invented_a"},
                        "right": {"node": "literal", "quantity": invented_quantity(99.0)}
                    }
                }
            },
            "then": {
                "node": "aggregate",
                "function": "min",
                "operands": [
                    {"node": "variable_ref", "variable_id": "invented_a"},
                    {
                        "node": "binary",
                        "operator": "add",
                        "left": {"node": "literal", "quantity": invented_quantity(1.0)},
                        "right": {"node": "literal", "quantity": invented_quantity(2.0)}
                    }
                ]
            },
            "else": {
                "node": "interpolate",
                "table": {
                    "table_id": "invented_table",
                    "argument_dimension": "stress",
                    "argument_unit_ref": "invented_stress_unit",
                    "result_dimension": "stress",
                    "result_unit_ref": "invented_stress_unit",
                    "rows": [
                        {"argument": 10.0, "result": 1.5},
                        {"argument": 20.0, "result": 2.5}
                    ]
                },
                "argument": {"node": "variable_ref", "variable_id": "invented_a"}
            }
        });
        let expression = decode_expression(&ast).expect("decode");
        let encoded = encode_expression(&expression);
        let round_trip = decode_expression(&encoded).expect("round trip decode");
        assert_eq!(expression, round_trip);
    }

    #[test]
    fn decode_reports_breadcrumb_paths() {
        let ast = json!({
            "node": "binary",
            "operator": "divide",
            "left": {"node": "variable_ref", "variable_id": "invented_a"},
            "right": {"node": "literal", "quantity": {"value": 1.0, "dimension": "bogus", "unit_ref": "x"}}
        });
        let error = decode_expression(&ast).expect_err("must fail");
        assert_eq!(error.path, "expression.right.quantity");
        assert!(error.message.contains("bogus"));
    }

    #[test]
    fn refusal_markers_decode_but_are_flagged() {
        let ast = json!({"node": "unsafe_host_access", "request": "filesystem"});
        let expression = decode_expression(&ast).expect("decode");
        assert!(expression_contains_refusal_marker(&expression));
    }

    #[test]
    fn checksum_requires_grammar_version() {
        let document = json!({
            "metadata": {"rule_pack_id": "invented_pack"},
            "checksums": {}
        });
        let error = compute_rule_pack_checksum(&document).expect_err("must fail");
        assert_eq!(error.subject, "grammar_version");
    }

    #[test]
    fn checksum_excludes_checksums_member_and_binds_grammar_version() {
        let base = json!({
            "rule_pack_kind": "open_pipe_stress_rule_pack",
            "grammar_version": "1.0.0",
            "metadata": {"rule_pack_id": "invented_pack"}
        });
        let mut with_checksums = base.clone();
        with_checksums.as_object_mut().unwrap().insert(
            "checksums".to_string(),
            json!({"rule_pack_checksum": {"value": "anything"}}),
        );
        let computed_base = compute_rule_pack_checksum(&base).expect("base");
        let computed_with = compute_rule_pack_checksum(&with_checksums).expect("with");
        assert_eq!(computed_base.value, computed_with.value);
        assert!(computed_base.grammar_version_bound);
        assert_eq!(computed_base.payload_excludes, vec!["checksums"]);

        let mut other_grammar = base.clone();
        other_grammar
            .as_object_mut()
            .unwrap()
            .insert("grammar_version".to_string(), json!("1.1.0"));
        let computed_other = compute_rule_pack_checksum(&other_grammar).expect("other");
        assert_ne!(computed_base.value, computed_other.value);
    }

    #[test]
    fn validation_blocks_missing_grammar_version() {
        let document = json!({
            "rule_pack_kind": "open_pipe_stress_rule_pack",
            "metadata": {
                "rule_pack_id": "invented_pack",
                "name": "Invented",
                "rule_pack_version": "0.1.0",
                "lifecycle_status": "draft",
                "source_notice": "invented"
            },
            "schema_version": "0.2.0",
            "formula_declarations": []
        });
        let outcome = validate_rule_pack_document(&document, false);
        assert!(outcome.has_blocking_findings);
        assert!(outcome
            .document_findings
            .iter()
            .any(|finding| finding.code == "MISSING_GRAMMAR_VERSION"));
    }
}
