//! Sandboxed declarative rule-pack expression evaluator.
//!
//! This crate evaluates explicit expression trees. It does not parse text,
//! execute host-language code, access files, access the network, spawn
//! processes, load plugins, embed protected standards content, or emit
//! professional/code-compliance claims.
//!
//! Grammar freeze (DEC-022 / D-02 Option A): the typed AST in this crate is
//! the canonical rule-pack expression grammar
//! (`expression_language: open_pipe_stress_declared_expression`), frozen at
//! [`GRAMMAR_VERSION`]. The frozen v1.0.0 function set is: negate; abs;
//! add/subtract/multiply/divide with enumerated dimension-product algebra;
//! the six comparisons; boolean and/or/not; eager select; n-ary min/max over
//! same-dimension same-unit quantities; piecewise-linear interpolation and
//! exact/step lookup over user-supplied monotone tables (out-of-range is a
//! blocking diagnostic — never silent extrapolation or clamping). No text
//! syntax exists at this freeze (deferred to ruling D-02b). The golden
//! conformance corpus under `fixtures/rule_expressions/conformance_corpus/`
//! is the freeze artifact; grammar changes require a corpus extension and a
//! version bump.

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

/// Grammar version implemented by this evaluator (DEC-022 freeze).
///
/// The declared `grammar_version` of a rule pack sits inside the
/// JCS-canonicalized payload bytes hashed by `rule_pack_checksum`
/// (see `core/rules/rule_pack_lifecycle`), so the reported checksum binds the
/// grammar version. Minor versions are additive-only; any breaking change
/// requires a new major version and a recorded human ruling.
pub const GRAMMAR_VERSION: &str = "1.0.0";

/// Grammar versions this evaluator accepts. A declared version outside this
/// set produces a blocking [`FindingCode::UnsupportedGrammarVersion`]
/// finding — never silent best-effort evaluation.
pub const SUPPORTED_GRAMMAR_VERSIONS: &[&str] = &["1.0.0"];

/// Returns true when `version` is a declared grammar version this evaluator
/// supports (exact match against [`SUPPORTED_GRAMMAR_VERSIONS`]).
pub fn grammar_version_supported(version: &str) -> bool {
    SUPPORTED_GRAMMAR_VERSIONS.contains(&version.trim())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dimension {
    Dimensionless,
    Length,
    Mass,
    Time,
    Temperature,
    TemperatureInterval,
    Angle,
    Rotation,
    Force,
    Moment,
    Pressure,
    Stress,
    Area,
    Volume,
    Density,
    LinearStiffness,
    RotationalStiffness,
    Displacement,
    Velocity,
    Acceleration,
    ThermalConductivity,
    SpecificHeat,
    ThermalExpansionCoefficient,
    SecondMomentArea,
    SectionModulus,
    MassPerLength,
    VolumePerLength,
    Slope,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisStatus {
    ModelIncomplete,
    MechanicsSolved,
    RuleInputsIncomplete,
    UserRuleChecked,
    UserRuleFailed,
    HumanReviewRequired,
    HumanApprovedForProject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingSource {
    RulePackRequiredInput,
    UserSuppliedValue,
    SolverResultField,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Quantity {
    pub value: f64,
    pub dimension: Dimension,
    pub unit_ref: String,
    pub unit_required: bool,
    pub dimension_check_required: bool,
}

impl Quantity {
    pub fn new(
        value: f64,
        dimension: Dimension,
        unit_ref: impl Into<String>,
    ) -> Result<Self, EvaluationError> {
        validate_finite("quantity", value)?;
        let unit_ref = unit_ref.into();
        if unit_ref.trim().is_empty() {
            return Err(EvaluationError::MissingUnitRef { name: "quantity" });
        }
        Ok(Self {
            value,
            dimension,
            unit_ref: unit_ref.trim().to_string(),
            unit_required: true,
            dimension_check_required: true,
        })
    }

    pub fn dimensionless(value: f64, unit_ref: impl Into<String>) -> Result<Self, EvaluationError> {
        Self::new(value, Dimension::Dimensionless, unit_ref)
    }

    fn with_value(&self, value: f64) -> Self {
        let mut quantity = self.clone();
        quantity.value = value;
        quantity
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableBinding {
    pub variable_id: String,
    pub source: BindingSource,
    pub quantity: Option<Quantity>,
}

impl VariableBinding {
    pub fn new(variable_id: impl Into<String>, source: BindingSource, quantity: Quantity) -> Self {
        Self {
            variable_id: variable_id.into(),
            source,
            quantity: Some(quantity),
        }
    }

    pub fn missing(variable_id: impl Into<String>, source: BindingSource) -> Self {
        Self {
            variable_id: variable_id.into(),
            source,
            quantity: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Negate,
    Abs,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregateFunction {
    Min,
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookupMode {
    /// The argument must exactly equal a row argument; in-range misses are
    /// blocking [`FindingCode::TableKeyNotFound`] findings.
    Exact,
    /// Step function: the row with the largest argument less than or equal to
    /// the lookup argument applies. Arguments outside the closed
    /// `[first, last]` row-argument range are blocking
    /// [`FindingCode::TableOutOfRange`] findings — never clamped.
    Step,
}

/// One row of a user-supplied table. Values are user value-slots; the table
/// mechanism is public, the values are not repository content.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TableRow {
    pub argument: f64,
    pub result: f64,
}

/// User-supplied monotone table for interpolation and lookup.
///
/// Row arguments must be strictly increasing. Out-of-range arguments are
/// blocking diagnostics: this evaluator never extrapolates and never clamps.
#[derive(Debug, Clone, PartialEq)]
pub struct UserTable {
    pub table_id: String,
    pub argument_dimension: Dimension,
    pub argument_unit_ref: String,
    pub result_dimension: Dimension,
    pub result_unit_ref: String,
    pub rows: Vec<TableRow>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Quantity),
    VariableRef(String),
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Compare {
        operator: ComparisonOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// Boolean conjunction/disjunction. Evaluation is eager: both operands
    /// are always evaluated, so diagnostics in either operand always surface.
    Logical {
        operator: LogicalOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// Eager conditional: condition, then-branch, and else-branch are all
    /// evaluated (in that fixed order) regardless of the condition value, so
    /// diagnostics in the unselected branch still block. Branches must both
    /// be booleans or both be quantities of the same dimension with matching
    /// unit references.
    Select {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Box<Expression>,
    },
    /// N-ary min/max over same-dimension, same-unit quantities. Operands are
    /// evaluated left to right; at least one operand is required.
    Aggregate {
        function: AggregateFunction,
        operands: Vec<Expression>,
    },
    /// Piecewise-linear interpolation over a user-supplied monotone table
    /// (at least two rows). Arguments outside the closed row-argument range
    /// are blocking findings — no extrapolation, no clamping.
    Interpolate {
        table: UserTable,
        argument: Box<Expression>,
    },
    /// Exact or step lookup over a user-supplied monotone table.
    Lookup {
        table: UserTable,
        mode: LookupMode,
        argument: Box<Expression>,
    },
    UnsupportedForm {
        form_id: String,
    },
    UnsafeHostAccess {
        request: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvaluationValue {
    Quantity(Quantity),
    Boolean(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingCode {
    UnsafeConstruct,
    UnsupportedExpressionForm,
    MissingVariable,
    DuplicateBinding,
    InvalidReference,
    MissingRequiredValue,
    NonFiniteInput,
    DivisionByZero,
    UnitMetadataMissing,
    UnitMismatch,
    DimensionMismatch,
    TypeMismatch,
    StatusBoundaryViolation,
    /// The declared rule-pack grammar version is missing, malformed, or not
    /// in [`SUPPORTED_GRAMMAR_VERSIONS`]. Always blocking (`RULE_EVALUATOR_ERROR`
    /// class) — the evaluator never falls back to best-effort evaluation.
    UnsupportedGrammarVersion,
    /// User-supplied table is structurally invalid (empty id, missing unit
    /// metadata, too few rows, non-finite values, or non-strictly-increasing
    /// arguments).
    TableMalformed,
    /// Interpolation/lookup argument is outside the closed row-argument
    /// range. Blocking: no silent extrapolation, no clamping.
    TableOutOfRange,
    /// Exact-mode lookup argument is inside the table range but matches no
    /// row argument exactly.
    TableKeyNotFound,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluationFinding {
    pub code: FindingCode,
    pub subject_id: String,
    pub message: String,
}

impl EvaluationFinding {
    fn new(code: FindingCode, subject_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code,
            subject_id: subject_id.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluationInput {
    pub expression: Expression,
    pub bindings: Vec<VariableBinding>,
    pub required_variable_ids: Vec<String>,
    pub statuses: Vec<AnalysisStatus>,
    /// Grammar version declared by the rule pack (the same value bound inside
    /// the JCS-hashed `rule_pack_checksum` payload). A version outside
    /// [`SUPPORTED_GRAMMAR_VERSIONS`] blocks evaluation.
    pub declared_grammar_version: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluationResult {
    pub value: Option<EvaluationValue>,
    pub statuses: Vec<AnalysisStatus>,
    pub source_variable_ids: Vec<String>,
    pub findings: Vec<EvaluationFinding>,
}

impl EvaluationResult {
    pub fn is_blocked(&self) -> bool {
        !self.findings.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvaluationError {
    NonFiniteInput { name: &'static str, value: f64 },
    MissingUnitRef { name: &'static str },
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
            Self::MissingUnitRef { name } => {
                write!(f, "{name} must include an explicit unit reference")
            }
        }
    }
}

impl Error for EvaluationError {}

pub fn evaluate(input: &EvaluationInput) -> EvaluationResult {
    let mut findings = Vec::new();
    check_grammar_version(&input.declared_grammar_version, &mut findings);
    let statuses = collect_statuses(&input.statuses, &mut findings);
    let binding_map = build_binding_map(&input.bindings, &mut findings);
    check_required_variables(&input.required_variable_ids, &binding_map, &mut findings);

    let mut source_variable_ids = Vec::new();
    let value = eval_expression(
        &input.expression,
        &binding_map,
        &mut source_variable_ids,
        &mut findings,
    );

    source_variable_ids.sort();
    source_variable_ids.dedup();

    EvaluationResult {
        value: if findings.is_empty() { value } else { None },
        statuses,
        source_variable_ids,
        findings,
    }
}

fn build_binding_map<'a>(
    bindings: &'a [VariableBinding],
    findings: &mut Vec<EvaluationFinding>,
) -> HashMap<&'a str, &'a VariableBinding> {
    let mut map = HashMap::new();
    let mut seen = HashSet::new();
    for binding in bindings {
        if binding.variable_id.trim().is_empty() {
            findings.push(EvaluationFinding::new(
                FindingCode::InvalidReference,
                "binding",
                "variable binding id must not be empty",
            ));
            continue;
        }
        if !seen.insert(binding.variable_id.as_str()) {
            findings.push(EvaluationFinding::new(
                FindingCode::DuplicateBinding,
                &binding.variable_id,
                "duplicate variable binding",
            ));
            continue;
        }
        if let Some(quantity) = &binding.quantity {
            if !quantity.value.is_finite() {
                findings.push(EvaluationFinding::new(
                    FindingCode::NonFiniteInput,
                    &binding.variable_id,
                    "variable binding quantity must be finite",
                ));
                continue;
            }
            if !quantity_has_required_metadata(quantity) {
                findings.push(unit_metadata_missing(&binding.variable_id));
                continue;
            }
        }
        map.insert(binding.variable_id.as_str(), binding);
    }
    map
}

fn check_required_variables(
    required_variable_ids: &[String],
    bindings: &HashMap<&str, &VariableBinding>,
    findings: &mut Vec<EvaluationFinding>,
) {
    let mut seen = HashSet::new();
    for variable_id in required_variable_ids {
        if variable_id.trim().is_empty() {
            findings.push(EvaluationFinding::new(
                FindingCode::InvalidReference,
                "required_variable",
                "required variable id must not be empty",
            ));
            continue;
        }
        if !seen.insert(variable_id.as_str()) {
            findings.push(EvaluationFinding::new(
                FindingCode::DuplicateBinding,
                variable_id,
                "duplicate required variable id",
            ));
            continue;
        }
        match bindings.get(variable_id.as_str()) {
            Some(binding) if binding.quantity.is_some() => {}
            Some(_) => findings.push(EvaluationFinding::new(
                FindingCode::MissingRequiredValue,
                variable_id,
                "required variable has no supplied value",
            )),
            None => findings.push(EvaluationFinding::new(
                FindingCode::MissingRequiredValue,
                variable_id,
                "required variable is not bound",
            )),
        }
    }
}

fn eval_expression(
    expression: &Expression,
    bindings: &HashMap<&str, &VariableBinding>,
    source_variable_ids: &mut Vec<String>,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    match expression {
        Expression::Literal(quantity) => {
            if !quantity.value.is_finite() {
                findings.push(EvaluationFinding::new(
                    FindingCode::NonFiniteInput,
                    "literal",
                    "literal quantity must be finite",
                ));
                None
            } else if !quantity_has_required_metadata(quantity) {
                findings.push(unit_metadata_missing("literal"));
                None
            } else {
                Some(EvaluationValue::Quantity(quantity.clone()))
            }
        }
        Expression::VariableRef(variable_id) => {
            eval_variable_ref(variable_id, bindings, source_variable_ids, findings)
        }
        Expression::Unary { operator, operand } => {
            let value = eval_expression(operand, bindings, source_variable_ids, findings)?;
            eval_unary(*operator, value, findings)
        }
        Expression::Binary {
            operator,
            left,
            right,
        } => {
            let left = eval_expression(left, bindings, source_variable_ids, findings)?;
            let right = eval_expression(right, bindings, source_variable_ids, findings)?;
            eval_binary(*operator, left, right, findings)
        }
        Expression::Compare {
            operator,
            left,
            right,
        } => {
            let left = eval_expression(left, bindings, source_variable_ids, findings)?;
            let right = eval_expression(right, bindings, source_variable_ids, findings)?;
            eval_compare(*operator, left, right, findings)
        }
        Expression::Logical {
            operator,
            left,
            right,
        } => {
            // Eager: both operands always evaluated; no value short-circuit.
            let left = eval_expression(left, bindings, source_variable_ids, findings)?;
            let right = eval_expression(right, bindings, source_variable_ids, findings)?;
            eval_logical(*operator, left, right, findings)
        }
        Expression::Select {
            condition,
            then_branch,
            else_branch,
        } => {
            // Eager: condition, then-branch, else-branch all evaluated in
            // this fixed order regardless of the condition value.
            let condition = eval_expression(condition, bindings, source_variable_ids, findings)?;
            let then_value = eval_expression(then_branch, bindings, source_variable_ids, findings)?;
            let else_value = eval_expression(else_branch, bindings, source_variable_ids, findings)?;
            eval_select(condition, then_value, else_value, findings)
        }
        Expression::Aggregate { function, operands } => {
            eval_aggregate(*function, operands, bindings, source_variable_ids, findings)
        }
        Expression::Interpolate { table, argument } => eval_table_expression(
            table,
            None,
            argument,
            bindings,
            source_variable_ids,
            findings,
        ),
        Expression::Lookup {
            table,
            mode,
            argument,
        } => eval_table_expression(
            table,
            Some(*mode),
            argument,
            bindings,
            source_variable_ids,
            findings,
        ),
        Expression::UnsupportedForm { form_id } => {
            findings.push(EvaluationFinding::new(
                FindingCode::UnsupportedExpressionForm,
                form_id,
                "expression form is not supported by this evaluator",
            ));
            None
        }
        Expression::UnsafeHostAccess { request } => {
            findings.push(EvaluationFinding::new(
                FindingCode::UnsafeConstruct,
                request,
                "host-language, filesystem, network, process, plugin, or reflection access is not permitted",
            ));
            None
        }
    }
}

fn eval_variable_ref(
    variable_id: &str,
    bindings: &HashMap<&str, &VariableBinding>,
    source_variable_ids: &mut Vec<String>,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    if variable_id.trim().is_empty() {
        findings.push(EvaluationFinding::new(
            FindingCode::InvalidReference,
            "variable_ref",
            "variable reference must not be empty",
        ));
        return None;
    }

    let Some(binding) = bindings.get(variable_id) else {
        findings.push(EvaluationFinding::new(
            FindingCode::MissingVariable,
            variable_id,
            "variable reference is not bound",
        ));
        return None;
    };
    let Some(quantity) = &binding.quantity else {
        findings.push(EvaluationFinding::new(
            FindingCode::MissingRequiredValue,
            variable_id,
            "variable reference has no supplied value",
        ));
        return None;
    };
    source_variable_ids.push(variable_id.to_string());
    Some(EvaluationValue::Quantity(quantity.clone()))
}

fn eval_unary(
    operator: UnaryOperator,
    value: EvaluationValue,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    match (operator, value) {
        (UnaryOperator::Negate, EvaluationValue::Quantity(quantity)) => Some(
            EvaluationValue::Quantity(quantity.with_value(-quantity.value)),
        ),
        (UnaryOperator::Negate, EvaluationValue::Boolean(_)) => {
            findings.push(EvaluationFinding::new(
                FindingCode::TypeMismatch,
                "unary_negate",
                "cannot negate a boolean expression",
            ));
            None
        }
        (UnaryOperator::Abs, EvaluationValue::Quantity(quantity)) => Some(
            EvaluationValue::Quantity(quantity.with_value(quantity.value.abs())),
        ),
        (UnaryOperator::Abs, EvaluationValue::Boolean(_)) => {
            findings.push(EvaluationFinding::new(
                FindingCode::TypeMismatch,
                "unary_abs",
                "cannot take the absolute value of a boolean expression",
            ));
            None
        }
        (UnaryOperator::Not, EvaluationValue::Boolean(value)) => {
            Some(EvaluationValue::Boolean(!value))
        }
        (UnaryOperator::Not, EvaluationValue::Quantity(_)) => {
            findings.push(EvaluationFinding::new(
                FindingCode::TypeMismatch,
                "unary_not",
                "logical not requires a boolean expression",
            ));
            None
        }
    }
}

fn eval_logical(
    operator: LogicalOperator,
    left: EvaluationValue,
    right: EvaluationValue,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    let (EvaluationValue::Boolean(left), EvaluationValue::Boolean(right)) = (left, right) else {
        findings.push(EvaluationFinding::new(
            FindingCode::TypeMismatch,
            "logical_expression",
            "logical and/or requires boolean operands",
        ));
        return None;
    };
    let result = match operator {
        LogicalOperator::And => left && right,
        LogicalOperator::Or => left || right,
    };
    Some(EvaluationValue::Boolean(result))
}

fn eval_select(
    condition: EvaluationValue,
    then_value: EvaluationValue,
    else_value: EvaluationValue,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    let EvaluationValue::Boolean(condition) = condition else {
        findings.push(EvaluationFinding::new(
            FindingCode::TypeMismatch,
            "select_condition",
            "select condition must be a boolean expression",
        ));
        return None;
    };
    match (then_value, else_value) {
        (EvaluationValue::Boolean(then_value), EvaluationValue::Boolean(else_value)) => {
            Some(EvaluationValue::Boolean(if condition {
                then_value
            } else {
                else_value
            }))
        }
        (EvaluationValue::Quantity(then_value), EvaluationValue::Quantity(else_value)) => {
            if then_value.dimension != else_value.dimension {
                findings.push(dimension_mismatch(
                    "select_branches",
                    then_value.dimension,
                    else_value.dimension,
                ));
                return None;
            }
            if !quantity_units_match(&then_value, &else_value) {
                findings.push(unit_mismatch("select_branches", &then_value, &else_value));
                return None;
            }
            Some(EvaluationValue::Quantity(if condition {
                then_value
            } else {
                else_value
            }))
        }
        _ => {
            findings.push(EvaluationFinding::new(
                FindingCode::TypeMismatch,
                "select_branches",
                "select branches must both be quantities or both be booleans",
            ));
            None
        }
    }
}

fn eval_aggregate(
    function: AggregateFunction,
    operands: &[Expression],
    bindings: &HashMap<&str, &VariableBinding>,
    source_variable_ids: &mut Vec<String>,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    let subject_id = match function {
        AggregateFunction::Min => "min",
        AggregateFunction::Max => "max",
    };
    if operands.is_empty() {
        findings.push(EvaluationFinding::new(
            FindingCode::UnsupportedExpressionForm,
            subject_id,
            "min/max requires at least one operand",
        ));
        return None;
    }

    let mut quantities = Vec::with_capacity(operands.len());
    for operand in operands {
        let value = eval_expression(operand, bindings, source_variable_ids, findings)?;
        let EvaluationValue::Quantity(quantity) = value else {
            findings.push(EvaluationFinding::new(
                FindingCode::TypeMismatch,
                subject_id,
                "min/max operands must be numeric quantities",
            ));
            return None;
        };
        quantities.push(quantity);
    }

    let first = quantities[0].clone();
    let mut selected = first.value;
    for quantity in &quantities[1..] {
        if quantity.dimension != first.dimension {
            findings.push(dimension_mismatch(
                subject_id,
                first.dimension,
                quantity.dimension,
            ));
            return None;
        }
        if !quantity_units_match(&first, quantity) {
            findings.push(unit_mismatch(subject_id, &first, quantity));
            return None;
        }
        selected = match function {
            AggregateFunction::Min => selected.min(quantity.value),
            AggregateFunction::Max => selected.max(quantity.value),
        };
    }
    Some(EvaluationValue::Quantity(first.with_value(selected)))
}

/// Validates a user-supplied table. Findings are pushed in a fixed order
/// (id, unit metadata, row count, per-row finiteness, monotonicity) so the
/// diagnostic stream stays deterministic.
fn validate_table(
    table: &UserTable,
    minimum_rows: usize,
    findings: &mut Vec<EvaluationFinding>,
) -> bool {
    let mut valid = true;
    let subject_id = if table.table_id.trim().is_empty() {
        valid = false;
        findings.push(EvaluationFinding::new(
            FindingCode::TableMalformed,
            "table",
            "table id must not be empty",
        ));
        "table".to_string()
    } else {
        table.table_id.trim().to_string()
    };

    if table.argument_unit_ref.trim().is_empty() || table.result_unit_ref.trim().is_empty() {
        valid = false;
        findings.push(EvaluationFinding::new(
            FindingCode::TableMalformed,
            &subject_id,
            "table argument and result unit references must not be empty",
        ));
    }

    if table.rows.len() < minimum_rows {
        valid = false;
        findings.push(EvaluationFinding::new(
            FindingCode::TableMalformed,
            &subject_id,
            format!(
                "table requires at least {minimum_rows} row(s), got {}",
                table.rows.len()
            ),
        ));
    }

    for row in &table.rows {
        if !row.argument.is_finite() || !row.result.is_finite() {
            valid = false;
            findings.push(EvaluationFinding::new(
                FindingCode::TableMalformed,
                &subject_id,
                "table rows must contain finite arguments and results",
            ));
        }
    }

    for pair in table.rows.windows(2) {
        if !(pair[0].argument < pair[1].argument) {
            valid = false;
            findings.push(EvaluationFinding::new(
                FindingCode::TableMalformed,
                &subject_id,
                "table arguments must be strictly increasing (monotone)",
            ));
        }
    }

    valid
}

/// Shared evaluation path for `Interpolate` (`mode == None`) and `Lookup`
/// (`mode == Some(_)`). Table structure is validated first (it is static
/// pack data), then the argument expression is evaluated; findings from both
/// steps surface in that fixed order.
fn eval_table_expression(
    table: &UserTable,
    mode: Option<LookupMode>,
    argument: &Expression,
    bindings: &HashMap<&str, &VariableBinding>,
    source_variable_ids: &mut Vec<String>,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    let minimum_rows = if mode.is_none() { 2 } else { 1 };
    let table_valid = validate_table(table, minimum_rows, findings);
    let argument_value = eval_expression(argument, bindings, source_variable_ids, findings);

    let argument_value = argument_value?;
    if !table_valid {
        return None;
    }
    let subject_id = table.table_id.trim().to_string();

    let EvaluationValue::Quantity(argument) = argument_value else {
        findings.push(EvaluationFinding::new(
            FindingCode::TypeMismatch,
            &subject_id,
            "table argument must be a numeric quantity",
        ));
        return None;
    };
    if argument.dimension != table.argument_dimension {
        findings.push(dimension_mismatch(
            &subject_id,
            argument.dimension,
            table.argument_dimension,
        ));
        return None;
    }
    if argument.unit_ref.trim() != table.argument_unit_ref.trim() {
        findings.push(EvaluationFinding::new(
            FindingCode::UnitMismatch,
            &subject_id,
            format!(
                "unit mismatch: argument={}, table={}",
                argument.unit_ref, table.argument_unit_ref
            ),
        ));
        return None;
    }

    let x = argument.value;
    let first = table.rows[0].argument;
    let last = table.rows[table.rows.len() - 1].argument;

    let result_value = match mode {
        Some(LookupMode::Exact) => {
            if let Some(row) = table.rows.iter().find(|row| row.argument == x) {
                row.result
            } else if x < first || x > last {
                findings.push(table_out_of_range(&subject_id, x, first, last));
                return None;
            } else {
                findings.push(EvaluationFinding::new(
                    FindingCode::TableKeyNotFound,
                    &subject_id,
                    "exact lookup argument matches no table row argument",
                ));
                return None;
            }
        }
        Some(LookupMode::Step) => {
            if x < first || x > last {
                findings.push(table_out_of_range(&subject_id, x, first, last));
                return None;
            }
            table
                .rows
                .iter()
                .rev()
                .find(|row| row.argument <= x)
                .expect("in-range step lookup always has a governing row")
                .result
        }
        None => {
            if x < first || x > last {
                findings.push(table_out_of_range(&subject_id, x, first, last));
                return None;
            }
            if let Some(row) = table.rows.iter().find(|row| row.argument == x) {
                row.result
            } else {
                let pair = table
                    .rows
                    .windows(2)
                    .find(|pair| pair[0].argument < x && x < pair[1].argument)
                    .expect("in-range interpolation always has a bracketing pair");
                let (low, high) = (pair[0], pair[1]);
                low.result
                    + (high.result - low.result)
                        * ((x - low.argument) / (high.argument - low.argument))
            }
        }
    };

    Some(EvaluationValue::Quantity(Quantity {
        value: result_value,
        dimension: table.result_dimension,
        unit_ref: table.result_unit_ref.trim().to_string(),
        unit_required: true,
        dimension_check_required: true,
    }))
}

fn table_out_of_range(
    subject_id: impl Into<String>,
    argument: f64,
    first: f64,
    last: f64,
) -> EvaluationFinding {
    EvaluationFinding::new(
        FindingCode::TableOutOfRange,
        subject_id,
        format!(
            "table argument {argument} is outside the table range [{first}, {last}]; \
             extrapolation and clamping are not permitted"
        ),
    )
}

fn eval_binary(
    operator: BinaryOperator,
    left: EvaluationValue,
    right: EvaluationValue,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    let (EvaluationValue::Quantity(left), EvaluationValue::Quantity(right)) = (left, right) else {
        findings.push(EvaluationFinding::new(
            FindingCode::TypeMismatch,
            "binary_expression",
            "binary arithmetic requires numeric quantities",
        ));
        return None;
    };

    match operator {
        BinaryOperator::Add => add_or_subtract(left, right, 1.0, findings),
        BinaryOperator::Subtract => add_or_subtract(left, right, -1.0, findings),
        BinaryOperator::Multiply => multiply(left, right, findings),
        BinaryOperator::Divide => divide(left, right, findings),
    }
}

fn add_or_subtract(
    left: Quantity,
    right: Quantity,
    sign: f64,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    if left.dimension != right.dimension {
        findings.push(dimension_mismatch(
            "add_subtract",
            left.dimension,
            right.dimension,
        ));
        return None;
    }
    if !quantity_units_match(&left, &right) {
        findings.push(unit_mismatch("add_subtract", &left, &right));
        return None;
    }
    Some(EvaluationValue::Quantity(
        left.with_value(left.value + sign * right.value),
    ))
}

/// Enumerated dimension-product table over the closed [`Dimension`] enum
/// (DEC-022 dimension-product algebra). Each entry reads
/// `(factor_a, factor_b, product)` and is commutative. Products that are not
/// representable in this table are blocking
/// [`FindingCode::UnsupportedExpressionForm`] findings — the closed enum is
/// never silently extended and nothing falls back to `Dimension::Tbd`.
///
/// These are unit-free dimensional mechanics relations only; no
/// standards-derived values appear here.
const DIMENSION_PRODUCTS: &[(Dimension, Dimension, Dimension)] = &[
    (Dimension::Length, Dimension::Length, Dimension::Area),
    (Dimension::Area, Dimension::Length, Dimension::Volume),
    (Dimension::Force, Dimension::Length, Dimension::Moment),
    (Dimension::Pressure, Dimension::Area, Dimension::Force),
    (Dimension::Stress, Dimension::Area, Dimension::Force),
    (Dimension::Mass, Dimension::Acceleration, Dimension::Force),
    (Dimension::Density, Dimension::Volume, Dimension::Mass),
    (Dimension::MassPerLength, Dimension::Length, Dimension::Mass),
    (
        Dimension::VolumePerLength,
        Dimension::Length,
        Dimension::Volume,
    ),
    (
        Dimension::LinearStiffness,
        Dimension::Length,
        Dimension::Force,
    ),
    (
        Dimension::LinearStiffness,
        Dimension::Displacement,
        Dimension::Force,
    ),
    (
        Dimension::RotationalStiffness,
        Dimension::Angle,
        Dimension::Moment,
    ),
    (
        Dimension::RotationalStiffness,
        Dimension::Rotation,
        Dimension::Moment,
    ),
    (
        Dimension::Stress,
        Dimension::SectionModulus,
        Dimension::Moment,
    ),
    (
        Dimension::SectionModulus,
        Dimension::Length,
        Dimension::SecondMomentArea,
    ),
    (Dimension::Velocity, Dimension::Time, Dimension::Length),
    (
        Dimension::Acceleration,
        Dimension::Time,
        Dimension::Velocity,
    ),
    (
        Dimension::ThermalExpansionCoefficient,
        Dimension::TemperatureInterval,
        Dimension::Dimensionless,
    ),
];

/// Resolves the commutative product of two (non-dimensionless) dimensions
/// against the enumerated [`DIMENSION_PRODUCTS`] table.
pub fn dimension_product(left: Dimension, right: Dimension) -> Option<Dimension> {
    DIMENSION_PRODUCTS.iter().find_map(|&(a, b, product)| {
        if (a == left && b == right) || (a == right && b == left) {
            Some(product)
        } else {
            None
        }
    })
}

/// Quotient resolution over the enumerated dimension-product table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionQuotient {
    Unique(Dimension),
    /// More than one enumerated dimension satisfies
    /// `candidate x denominator = numerator` (the closed enum keeps
    /// dimensionally identical vocabulary members such as `Pressure` and
    /// `Stress` distinct), so the quotient is ambiguous and blocks.
    Ambiguous,
    Unrepresentable,
}

/// Resolves `numerator / denominator` by inverting [`DIMENSION_PRODUCTS`]:
/// the quotient is the unique dimension `c` with `c x denominator = numerator`.
pub fn dimension_quotient(numerator: Dimension, denominator: Dimension) -> DimensionQuotient {
    let mut candidates: Vec<Dimension> = Vec::new();
    for &(a, b, product) in DIMENSION_PRODUCTS {
        if product != numerator {
            continue;
        }
        if b == denominator && !candidates.contains(&a) {
            candidates.push(a);
        }
        if a == denominator && !candidates.contains(&b) {
            candidates.push(b);
        }
    }
    match candidates.as_slice() {
        [] => DimensionQuotient::Unrepresentable,
        [unique] => DimensionQuotient::Unique(*unique),
        _ => DimensionQuotient::Ambiguous,
    }
}

/// Canonical unit reference for a derived product quantity (frozen in
/// grammar v1.0.0): derived dimensionless results reuse the existing `ratio`
/// token; all other products join the operand unit references with `*` in
/// lexicographic byte order so multiplication stays commutative. This crate
/// owns no unit conversion (DEC-018 places conversion at the catalog
/// boundary), so derived unit references are composed, never converted.
fn derived_product_unit_ref(left: &Quantity, right: &Quantity, product: Dimension) -> String {
    if product == Dimension::Dimensionless {
        return "ratio".to_string();
    }
    let (a, b) = (left.unit_ref.trim(), right.unit_ref.trim());
    if a <= b {
        format!("{a}*{b}")
    } else {
        format!("{b}*{a}")
    }
}

/// Canonical unit reference for a derived quotient quantity (frozen in
/// grammar v1.0.0): `numerator_ref/denominator_ref` (order-preserving).
fn derived_quotient_unit_ref(left: &Quantity, right: &Quantity, quotient: Dimension) -> String {
    if quotient == Dimension::Dimensionless {
        return "ratio".to_string();
    }
    format!("{}/{}", left.unit_ref.trim(), right.unit_ref.trim())
}

fn multiply(
    left: Quantity,
    right: Quantity,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    match (left.dimension, right.dimension) {
        (Dimension::Dimensionless, _) => Some(EvaluationValue::Quantity(
            right.with_value(left.value * right.value),
        )),
        (_, Dimension::Dimensionless) => Some(EvaluationValue::Quantity(
            left.with_value(left.value * right.value),
        )),
        (left_dim, right_dim) => match dimension_product(left_dim, right_dim) {
            Some(product) => Some(EvaluationValue::Quantity(Quantity {
                value: left.value * right.value,
                unit_ref: derived_product_unit_ref(&left, &right, product),
                dimension: product,
                unit_required: true,
                dimension_check_required: true,
            })),
            None => {
                findings.push(EvaluationFinding::new(
                    FindingCode::UnsupportedExpressionForm,
                    "multiply",
                    format!(
                        "no enumerated dimension product exists for \
                         {left_dim:?} x {right_dim:?} in grammar v{GRAMMAR_VERSION}"
                    ),
                ));
                None
            }
        },
    }
}

fn divide(
    left: Quantity,
    right: Quantity,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    if right.value == 0.0 {
        findings.push(EvaluationFinding::new(
            FindingCode::DivisionByZero,
            "divide",
            "division by zero is not permitted",
        ));
        return None;
    }

    match (left.dimension, right.dimension) {
        (dim, Dimension::Dimensionless) => Some(EvaluationValue::Quantity(Quantity {
            value: left.value / right.value,
            dimension: dim,
            unit_ref: left.unit_ref,
            unit_required: left.unit_required,
            dimension_check_required: left.dimension_check_required,
        })),
        (left_dim, right_dim) if left_dim == right_dim => {
            if !quantity_units_match(&left, &right) {
                findings.push(unit_mismatch("divide", &left, &right));
                return None;
            }
            Some(EvaluationValue::Quantity(ratio_quantity(
                left.value / right.value,
            )))
        }
        (left_dim, right_dim) => match dimension_quotient(left_dim, right_dim) {
            DimensionQuotient::Unique(quotient) => Some(EvaluationValue::Quantity(Quantity {
                value: left.value / right.value,
                unit_ref: derived_quotient_unit_ref(&left, &right, quotient),
                dimension: quotient,
                unit_required: true,
                dimension_check_required: true,
            })),
            DimensionQuotient::Ambiguous => {
                findings.push(EvaluationFinding::new(
                    FindingCode::UnsupportedExpressionForm,
                    "divide",
                    format!(
                        "the dimension quotient {left_dim:?} / {right_dim:?} is ambiguous \
                         over the closed dimension vocabulary in grammar v{GRAMMAR_VERSION}"
                    ),
                ));
                None
            }
            DimensionQuotient::Unrepresentable => {
                findings.push(EvaluationFinding::new(
                    FindingCode::UnsupportedExpressionForm,
                    "divide",
                    format!(
                        "no enumerated dimension quotient exists for \
                         {left_dim:?} / {right_dim:?} in grammar v{GRAMMAR_VERSION}"
                    ),
                ));
                None
            }
        },
    }
}

fn eval_compare(
    operator: ComparisonOperator,
    left: EvaluationValue,
    right: EvaluationValue,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    let (EvaluationValue::Quantity(left), EvaluationValue::Quantity(right)) = (left, right) else {
        findings.push(EvaluationFinding::new(
            FindingCode::TypeMismatch,
            "comparison",
            "comparison requires numeric quantities",
        ));
        return None;
    };
    if left.dimension != right.dimension {
        findings.push(dimension_mismatch(
            "comparison",
            left.dimension,
            right.dimension,
        ));
        return None;
    }
    if !quantity_units_match(&left, &right) {
        findings.push(unit_mismatch("comparison", &left, &right));
        return None;
    }
    let result = match operator {
        ComparisonOperator::LessThan => left.value < right.value,
        ComparisonOperator::LessThanOrEqual => left.value <= right.value,
        ComparisonOperator::GreaterThan => left.value > right.value,
        ComparisonOperator::GreaterThanOrEqual => left.value >= right.value,
        ComparisonOperator::Equal => left.value == right.value,
        ComparisonOperator::NotEqual => left.value != right.value,
    };
    Some(EvaluationValue::Boolean(result))
}

fn check_grammar_version(declared: &str, findings: &mut Vec<EvaluationFinding>) {
    let declared = declared.trim();
    if declared.is_empty() {
        findings.push(EvaluationFinding::new(
            FindingCode::UnsupportedGrammarVersion,
            "grammar_version",
            "rule pack must declare an explicit expression grammar version",
        ));
        return;
    }
    if !is_semver(declared) {
        findings.push(EvaluationFinding::new(
            FindingCode::UnsupportedGrammarVersion,
            "grammar_version",
            format!(
                "declared grammar version '{declared}' is not a MAJOR.MINOR.PATCH \
                 semantic version"
            ),
        ));
        return;
    }
    if !grammar_version_supported(declared) {
        findings.push(EvaluationFinding::new(
            FindingCode::UnsupportedGrammarVersion,
            "grammar_version",
            format!(
                "declared grammar version '{declared}' is not supported by this \
                 evaluator (supported: {SUPPORTED_GRAMMAR_VERSIONS:?})"
            ),
        ));
    }
}

/// Strict `MAJOR.MINOR.PATCH` check: three dot-separated decimal components,
/// no signs, no leading zeros, no pre-release/build suffixes.
fn is_semver(version: &str) -> bool {
    let component_ok = |part: &str| {
        !part.is_empty()
            && part.len() <= 9
            && part.bytes().all(|byte| byte.is_ascii_digit())
            && (part == "0" || !part.starts_with('0'))
    };
    let parts: Vec<&str> = version.split('.').collect();
    parts.len() == 3 && parts.iter().all(|part| component_ok(part))
}

fn collect_statuses(
    statuses: &[AnalysisStatus],
    findings: &mut Vec<EvaluationFinding>,
) -> Vec<AnalysisStatus> {
    let mut collected = Vec::new();
    if statuses.is_empty() {
        collected.push(AnalysisStatus::RuleInputsIncomplete);
        return collected;
    }
    for status in statuses {
        if *status == AnalysisStatus::HumanApprovedForProject {
            findings.push(EvaluationFinding::new(
                FindingCode::StatusBoundaryViolation,
                "analysis_status",
                "human approval is an external project record, not an evaluator output",
            ));
            collected.push(AnalysisStatus::HumanReviewRequired);
            continue;
        }
        if !collected.contains(status) {
            collected.push(*status);
        }
    }
    collected
}

fn dimension_mismatch(
    subject_id: impl Into<String>,
    left: Dimension,
    right: Dimension,
) -> EvaluationFinding {
    EvaluationFinding::new(
        FindingCode::DimensionMismatch,
        subject_id,
        format!("dimension mismatch: left={left:?}, right={right:?}"),
    )
}

fn unit_metadata_missing(subject_id: impl Into<String>) -> EvaluationFinding {
    EvaluationFinding::new(
        FindingCode::UnitMetadataMissing,
        subject_id,
        "quantity must include explicit unit metadata and dimension-check intent",
    )
}

fn unit_mismatch(
    subject_id: impl Into<String>,
    left: &Quantity,
    right: &Quantity,
) -> EvaluationFinding {
    EvaluationFinding::new(
        FindingCode::UnitMismatch,
        subject_id,
        format!(
            "unit mismatch: left={}, right={}",
            left.unit_ref, right.unit_ref
        ),
    )
}

fn quantity_has_required_metadata(quantity: &Quantity) -> bool {
    quantity.unit_required
        && quantity.dimension_check_required
        && !quantity.unit_ref.trim().is_empty()
}

fn quantity_units_match(left: &Quantity, right: &Quantity) -> bool {
    left.unit_ref.trim() == right.unit_ref.trim()
}

fn ratio_quantity(value: f64) -> Quantity {
    Quantity::dimensionless(value, "ratio").expect("ratio unit reference is non-empty")
}

fn validate_finite(name: &'static str, value: f64) -> Result<(), EvaluationError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(EvaluationError::NonFiniteInput { name, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binding(id: &str, value: f64, dimension: Dimension) -> VariableBinding {
        VariableBinding::new(
            id,
            BindingSource::RulePackRequiredInput,
            Quantity::new(value, dimension, unit_ref_for_dimension(dimension)).unwrap(),
        )
    }

    fn binding_with_unit(
        id: &str,
        value: f64,
        dimension: Dimension,
        unit_ref: &str,
    ) -> VariableBinding {
        VariableBinding::new(
            id,
            BindingSource::RulePackRequiredInput,
            Quantity::new(value, dimension, unit_ref).unwrap(),
        )
    }

    fn unit_ref_for_dimension(dimension: Dimension) -> &'static str {
        match dimension {
            Dimension::Dimensionless => "ratio",
            Dimension::Length => "length_unit",
            Dimension::Mass => "mass_unit",
            Dimension::Time => "time_unit",
            Dimension::Temperature => "temperature_unit",
            Dimension::TemperatureInterval => "temperature_interval_unit",
            Dimension::Angle => "angle_unit",
            Dimension::Rotation => "rotation_unit",
            Dimension::Force => "force_unit",
            Dimension::Moment => "moment_unit",
            Dimension::Pressure => "pressure_unit",
            Dimension::Stress => "stress_unit",
            Dimension::Area => "area_unit",
            Dimension::Volume => "volume_unit",
            Dimension::Density => "density_unit",
            Dimension::LinearStiffness => "linear_stiffness_unit",
            Dimension::RotationalStiffness => "rotational_stiffness_unit",
            Dimension::Displacement => "displacement_unit",
            Dimension::Velocity => "velocity_unit",
            Dimension::Acceleration => "acceleration_unit",
            Dimension::ThermalConductivity => "thermal_conductivity_unit",
            Dimension::SpecificHeat => "specific_heat_unit",
            Dimension::ThermalExpansionCoefficient => "thermal_expansion_coefficient_unit",
            Dimension::SecondMomentArea => "second_moment_area_unit",
            Dimension::SectionModulus => "section_modulus_unit",
            Dimension::MassPerLength => "mass_per_length_unit",
            Dimension::VolumePerLength => "volume_per_length_unit",
            Dimension::Slope => "slope_unit",
            Dimension::Tbd => "TBD",
        }
    }

    #[test]
    fn dimension_enum_tracks_pkg02_canonical_vocabulary() {
        let dimensions = [
            Dimension::Dimensionless,
            Dimension::Length,
            Dimension::Mass,
            Dimension::Time,
            Dimension::Temperature,
            Dimension::TemperatureInterval,
            Dimension::Angle,
            Dimension::Rotation,
            Dimension::Force,
            Dimension::Moment,
            Dimension::Pressure,
            Dimension::Stress,
            Dimension::Area,
            Dimension::Volume,
            Dimension::Density,
            Dimension::LinearStiffness,
            Dimension::RotationalStiffness,
            Dimension::Displacement,
            Dimension::Velocity,
            Dimension::Acceleration,
            Dimension::ThermalConductivity,
            Dimension::SpecificHeat,
            Dimension::ThermalExpansionCoefficient,
            Dimension::SecondMomentArea,
            Dimension::SectionModulus,
            Dimension::MassPerLength,
            Dimension::VolumePerLength,
            Dimension::Slope,
            Dimension::Tbd,
        ];
        assert_eq!(dimensions.len(), 29);
        let rendered = format!("{dimensions:?}");
        assert!(!rendered.contains("TemperatureDifference"));
        assert!(!rendered.contains("AreaMoment"));
        assert!(!rendered.contains("[Stiffness"));
        assert!(!rendered.contains(", Stiffness,"));
        assert!(!rendered.contains(", Stiffness]"));
        assert!(rendered.contains("LinearStiffness"));
        assert!(rendered.contains("RotationalStiffness"));
        assert!(rendered.contains("SecondMomentArea"));
    }

    fn input(expression: Expression, bindings: Vec<VariableBinding>) -> EvaluationInput {
        EvaluationInput {
            expression,
            bindings,
            required_variable_ids: vec![],
            statuses: vec![AnalysisStatus::MechanicsSolved],
            declared_grammar_version: GRAMMAR_VERSION.to_string(),
        }
    }

    fn invented_table() -> UserTable {
        UserTable {
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
                TableRow {
                    argument: 40.0,
                    result: 3.5,
                },
            ],
        }
    }

    #[test]
    fn evaluates_invented_dimensionally_compatible_expression() {
        let expression = Expression::Compare {
            operator: ComparisonOperator::LessThanOrEqual,
            left: Box::new(Expression::Binary {
                operator: BinaryOperator::Add,
                left: Box::new(Expression::VariableRef("actual_stress".to_string())),
                right: Box::new(Expression::Literal(
                    Quantity::new(5.0, Dimension::Stress, "stress_unit").unwrap(),
                )),
            }),
            right: Box::new(Expression::VariableRef("invented_limit".to_string())),
        };
        let result = evaluate(&input(
            expression,
            vec![
                binding("actual_stress", 95.0, Dimension::Stress),
                binding("invented_limit", 120.0, Dimension::Stress),
            ],
        ));

        assert_eq!(result.value, Some(EvaluationValue::Boolean(true)));
        assert!(result.findings.is_empty());
        assert_eq!(
            result.source_variable_ids,
            vec!["actual_stress".to_string(), "invented_limit".to_string()]
        );
    }

    #[test]
    fn rejects_unsafe_host_access() {
        let result = evaluate(&input(
            Expression::UnsafeHostAccess {
                request: "filesystem".to_string(),
            },
            vec![],
        ));

        assert!(result.is_blocked());
        assert_eq!(result.findings[0].code, FindingCode::UnsafeConstruct);
        assert_eq!(result.value, None);
    }

    #[test]
    fn rejects_unsupported_expression_form() {
        let result = evaluate(&input(
            Expression::UnsupportedForm {
                form_id: "loop".to_string(),
            },
            vec![],
        ));

        assert_eq!(
            result.findings[0].code,
            FindingCode::UnsupportedExpressionForm
        );
        assert_eq!(result.value, None);
    }

    #[test]
    fn reports_missing_variable_binding() {
        let result = evaluate(&input(
            Expression::VariableRef("missing".to_string()),
            vec![],
        ));

        assert_eq!(result.findings[0].code, FindingCode::MissingVariable);
        assert_eq!(result.value, None);
    }

    #[test]
    fn reports_duplicate_binding() {
        let result = evaluate(&input(
            Expression::VariableRef("x".to_string()),
            vec![
                binding("x", 1.0, Dimension::Force),
                binding("x", 2.0, Dimension::Force),
            ],
        ));

        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::DuplicateBinding));
        assert_eq!(result.value, None);
    }

    #[test]
    fn reports_invalid_empty_reference() {
        let result = evaluate(&input(Expression::VariableRef(" ".to_string()), vec![]));

        assert_eq!(result.findings[0].code, FindingCode::InvalidReference);
        assert_eq!(result.value, None);
    }

    #[test]
    fn reports_missing_required_value() {
        let mut evaluation = input(
            Expression::VariableRef("allowable".to_string()),
            vec![VariableBinding::missing(
                "allowable",
                BindingSource::UserSuppliedValue,
            )],
        );
        evaluation.required_variable_ids = vec!["allowable".to_string()];

        let result = evaluate(&evaluation);

        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::MissingRequiredValue));
        assert_eq!(result.value, None);
    }

    #[test]
    fn reports_non_finite_literal() {
        let result = evaluate(&input(
            Expression::Literal(Quantity {
                value: f64::NAN,
                dimension: Dimension::Force,
                unit_ref: "force_unit".to_string(),
                unit_required: true,
                dimension_check_required: true,
            }),
            vec![],
        ));

        assert_eq!(result.findings[0].code, FindingCode::NonFiniteInput);
        assert_eq!(result.value, None);
    }

    #[test]
    fn rejects_division_by_zero() {
        let result = evaluate(&input(
            Expression::Binary {
                operator: BinaryOperator::Divide,
                left: Box::new(Expression::VariableRef("force".to_string())),
                right: Box::new(Expression::Literal(
                    Quantity::dimensionless(0.0, "ratio").unwrap(),
                )),
            },
            vec![binding("force", 10.0, Dimension::Force)],
        ));

        assert_eq!(result.findings[0].code, FindingCode::DivisionByZero);
        assert_eq!(result.value, None);
    }

    #[test]
    fn rejects_dimension_mismatch() {
        let result = evaluate(&input(
            Expression::Compare {
                operator: ComparisonOperator::LessThan,
                left: Box::new(Expression::VariableRef("force".to_string())),
                right: Box::new(Expression::VariableRef("stress".to_string())),
            },
            vec![
                binding("force", 10.0, Dimension::Force),
                binding("stress", 10.0, Dimension::Stress),
            ],
        ));

        assert_eq!(result.findings[0].code, FindingCode::DimensionMismatch);
        assert_eq!(result.value, None);
    }

    #[test]
    fn rejects_missing_unit_metadata_at_boundary() {
        let result = evaluate(&input(
            Expression::VariableRef("force".to_string()),
            vec![VariableBinding::new(
                "force",
                BindingSource::RulePackRequiredInput,
                Quantity {
                    value: 10.0,
                    dimension: Dimension::Force,
                    unit_ref: "".to_string(),
                    unit_required: true,
                    dimension_check_required: true,
                },
            )],
        ));

        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::UnitMetadataMissing));
        assert_eq!(result.value, None);
    }

    #[test]
    fn rejects_same_dimension_unit_mismatch_without_conversion_policy() {
        let result = evaluate(&input(
            Expression::Compare {
                operator: ComparisonOperator::LessThan,
                left: Box::new(Expression::VariableRef("actual".to_string())),
                right: Box::new(Expression::VariableRef("limit".to_string())),
            },
            vec![
                binding_with_unit("actual", 10.0, Dimension::Stress, "MPa"),
                binding_with_unit("limit", 12.0, Dimension::Stress, "psi"),
            ],
        ));

        assert_eq!(result.findings[0].code, FindingCode::UnitMismatch);
        assert_eq!(result.value, None);
    }

    #[test]
    fn permits_dimensionless_scaling_without_derived_dimension_policy() {
        let result = evaluate(&input(
            Expression::Binary {
                operator: BinaryOperator::Multiply,
                left: Box::new(Expression::Literal(
                    Quantity::dimensionless(2.0, "ratio").unwrap(),
                )),
                right: Box::new(Expression::VariableRef("force".to_string())),
            },
            vec![binding("force", 10.0, Dimension::Force)],
        ));

        assert_eq!(
            result.value,
            Some(EvaluationValue::Quantity(Quantity {
                value: 20.0,
                dimension: Dimension::Force,
                unit_ref: "force_unit".to_string(),
                unit_required: true,
                dimension_check_required: true,
            }))
        );
        assert!(result.findings.is_empty());
    }

    #[test]
    fn resolves_enumerated_dimensional_multiplication_to_derived_dimension() {
        let result = evaluate(&input(
            Expression::Binary {
                operator: BinaryOperator::Multiply,
                left: Box::new(Expression::VariableRef("force".to_string())),
                right: Box::new(Expression::VariableRef("length".to_string())),
            },
            vec![
                binding("force", 10.0, Dimension::Force),
                binding("length", 2.0, Dimension::Length),
            ],
        ));

        assert_eq!(
            result.value,
            Some(EvaluationValue::Quantity(Quantity {
                value: 20.0,
                dimension: Dimension::Moment,
                unit_ref: "force_unit*length_unit".to_string(),
                unit_required: true,
                dimension_check_required: true,
            }))
        );
        assert!(result.findings.is_empty());
    }

    #[test]
    fn derived_product_unit_ref_is_commutative() {
        let left = evaluate(&input(
            Expression::Binary {
                operator: BinaryOperator::Multiply,
                left: Box::new(Expression::VariableRef("length".to_string())),
                right: Box::new(Expression::VariableRef("force".to_string())),
            },
            vec![
                binding("force", 10.0, Dimension::Force),
                binding("length", 2.0, Dimension::Length),
            ],
        ));
        let Some(EvaluationValue::Quantity(product)) = left.value else {
            panic!("expected a derived quantity");
        };
        assert_eq!(product.unit_ref, "force_unit*length_unit");
        assert_eq!(product.dimension, Dimension::Moment);
    }

    #[test]
    fn rejects_unrepresentable_dimensional_multiplication() {
        let result = evaluate(&input(
            Expression::Binary {
                operator: BinaryOperator::Multiply,
                left: Box::new(Expression::VariableRef("stress_a".to_string())),
                right: Box::new(Expression::VariableRef("stress_b".to_string())),
            },
            vec![
                binding("stress_a", 3.0, Dimension::Stress),
                binding("stress_b", 4.0, Dimension::Stress),
            ],
        ));

        assert_eq!(
            result.findings[0].code,
            FindingCode::UnsupportedExpressionForm
        );
        assert_eq!(result.value, None);
    }

    #[test]
    fn resolves_unique_dimension_quotient() {
        let result = evaluate(&input(
            Expression::Binary {
                operator: BinaryOperator::Divide,
                left: Box::new(Expression::VariableRef("moment".to_string())),
                right: Box::new(Expression::VariableRef("length".to_string())),
            },
            vec![
                binding("moment", 30.0, Dimension::Moment),
                binding("length", 4.0, Dimension::Length),
            ],
        ));

        assert_eq!(
            result.value,
            Some(EvaluationValue::Quantity(Quantity {
                value: 7.5,
                dimension: Dimension::Force,
                unit_ref: "moment_unit/length_unit".to_string(),
                unit_required: true,
                dimension_check_required: true,
            }))
        );
        assert!(result.findings.is_empty());
    }

    #[test]
    fn rejects_ambiguous_dimension_quotient() {
        // Force / Area is ambiguous over the closed vocabulary because both
        // Pressure x Area and Stress x Area are enumerated products.
        let result = evaluate(&input(
            Expression::Binary {
                operator: BinaryOperator::Divide,
                left: Box::new(Expression::VariableRef("force".to_string())),
                right: Box::new(Expression::VariableRef("area".to_string())),
            },
            vec![
                binding("force", 12.0, Dimension::Force),
                binding("area", 3.0, Dimension::Area),
            ],
        ));

        assert_eq!(
            result.findings[0].code,
            FindingCode::UnsupportedExpressionForm
        );
        assert!(result.findings[0].message.contains("ambiguous"));
        assert_eq!(result.value, None);
    }

    #[test]
    fn dimension_product_table_has_unique_commutative_keys() {
        let mut seen: Vec<(Dimension, Dimension)> = Vec::new();
        for &(a, b, product) in DIMENSION_PRODUCTS {
            assert_ne!(a, Dimension::Dimensionless);
            assert_ne!(b, Dimension::Dimensionless);
            assert_ne!(a, Dimension::Tbd);
            assert_ne!(b, Dimension::Tbd);
            assert_ne!(product, Dimension::Tbd);
            assert!(
                !seen.contains(&(a, b)) && !seen.contains(&(b, a)),
                "duplicate commutative product key {a:?} x {b:?}"
            );
            seen.push((a, b));
        }
    }

    #[test]
    fn converts_same_dimension_division_to_dimensionless_ratio() {
        let result = evaluate(&input(
            Expression::Binary {
                operator: BinaryOperator::Divide,
                left: Box::new(Expression::VariableRef("actual".to_string())),
                right: Box::new(Expression::VariableRef("limit".to_string())),
            },
            vec![
                binding("actual", 25.0, Dimension::Stress),
                binding("limit", 100.0, Dimension::Stress),
            ],
        ));

        assert_eq!(
            result.value,
            Some(EvaluationValue::Quantity(Quantity {
                value: 0.25,
                dimension: Dimension::Dimensionless,
                unit_ref: "ratio".to_string(),
                unit_required: true,
                dimension_check_required: true,
            }))
        );
        assert!(result.findings.is_empty());
    }

    #[test]
    fn maps_human_approval_status_to_boundary_finding() {
        let mut evaluation = input(
            Expression::Literal(Quantity::dimensionless(1.0, "ratio").unwrap()),
            vec![],
        );
        evaluation.statuses = vec![AnalysisStatus::HumanApprovedForProject];

        let result = evaluate(&evaluation);

        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::StatusBoundaryViolation));
        assert!(result
            .statuses
            .contains(&AnalysisStatus::HumanReviewRequired));
        assert_eq!(result.value, None);
    }

    #[test]
    fn blocks_unsupported_grammar_version() {
        let mut evaluation = input(
            Expression::Literal(Quantity::dimensionless(1.0, "ratio").unwrap()),
            vec![],
        );
        evaluation.declared_grammar_version = "9.0.0".to_string();

        let result = evaluate(&evaluation);

        assert_eq!(
            result.findings[0].code,
            FindingCode::UnsupportedGrammarVersion
        );
        assert_eq!(result.value, None);
    }

    #[test]
    fn blocks_malformed_or_missing_grammar_version() {
        for declared in ["", "1.0", "1.0.0-beta", "v1.0.0", "01.0.0"] {
            let mut evaluation = input(
                Expression::Literal(Quantity::dimensionless(1.0, "ratio").unwrap()),
                vec![],
            );
            evaluation.declared_grammar_version = declared.to_string();
            let result = evaluate(&evaluation);
            assert_eq!(
                result.findings[0].code,
                FindingCode::UnsupportedGrammarVersion,
                "declared version {declared:?} must block"
            );
            assert_eq!(result.value, None);
        }
        assert!(grammar_version_supported(GRAMMAR_VERSION));
    }

    #[test]
    fn evaluates_logical_select_aggregate_and_abs() {
        let boolean = |value: f64, limit: f64| Expression::Compare {
            operator: ComparisonOperator::LessThan,
            left: Box::new(Expression::Literal(
                Quantity::dimensionless(value, "ratio").unwrap(),
            )),
            right: Box::new(Expression::Literal(
                Quantity::dimensionless(limit, "ratio").unwrap(),
            )),
        };

        let logical = evaluate(&input(
            Expression::Logical {
                operator: LogicalOperator::And,
                left: Box::new(boolean(1.0, 2.0)),
                right: Box::new(Expression::Unary {
                    operator: UnaryOperator::Not,
                    operand: Box::new(boolean(3.0, 2.0)),
                }),
            },
            vec![],
        ));
        assert_eq!(logical.value, Some(EvaluationValue::Boolean(true)));

        let select = evaluate(&input(
            Expression::Select {
                condition: Box::new(boolean(1.0, 2.0)),
                then_branch: Box::new(Expression::VariableRef("low".to_string())),
                else_branch: Box::new(Expression::VariableRef("high".to_string())),
            },
            vec![
                binding("low", 5.0, Dimension::Stress),
                binding("high", 9.0, Dimension::Stress),
            ],
        ));
        assert_eq!(
            select.value,
            Some(EvaluationValue::Quantity(Quantity {
                value: 5.0,
                dimension: Dimension::Stress,
                unit_ref: "stress_unit".to_string(),
                unit_required: true,
                dimension_check_required: true,
            }))
        );

        let aggregate = evaluate(&input(
            Expression::Aggregate {
                function: AggregateFunction::Max,
                operands: vec![
                    Expression::VariableRef("low".to_string()),
                    Expression::VariableRef("high".to_string()),
                    Expression::Unary {
                        operator: UnaryOperator::Abs,
                        operand: Box::new(Expression::Unary {
                            operator: UnaryOperator::Negate,
                            operand: Box::new(Expression::VariableRef("low".to_string())),
                        }),
                    },
                ],
            },
            vec![
                binding("low", 5.0, Dimension::Stress),
                binding("high", 9.0, Dimension::Stress),
            ],
        ));
        assert_eq!(
            aggregate.value,
            Some(EvaluationValue::Quantity(Quantity {
                value: 9.0,
                dimension: Dimension::Stress,
                unit_ref: "stress_unit".to_string(),
                unit_required: true,
                dimension_check_required: true,
            }))
        );
    }

    #[test]
    fn select_is_eager_so_unselected_branch_diagnostics_block() {
        let result = evaluate(&input(
            Expression::Select {
                condition: Box::new(Expression::Compare {
                    operator: ComparisonOperator::LessThan,
                    left: Box::new(Expression::Literal(
                        Quantity::dimensionless(1.0, "ratio").unwrap(),
                    )),
                    right: Box::new(Expression::Literal(
                        Quantity::dimensionless(2.0, "ratio").unwrap(),
                    )),
                }),
                then_branch: Box::new(Expression::Literal(
                    Quantity::dimensionless(1.0, "ratio").unwrap(),
                )),
                else_branch: Box::new(Expression::VariableRef("never_bound".to_string())),
            },
            vec![],
        ));

        assert_eq!(result.findings[0].code, FindingCode::MissingVariable);
        assert_eq!(result.value, None);
    }

    #[test]
    fn interpolates_and_looks_up_user_tables() {
        let argument = |value: f64| {
            Box::new(Expression::Literal(
                Quantity::new(value, Dimension::Temperature, "invented_temperature_unit").unwrap(),
            ))
        };

        let interpolated = evaluate(&input(
            Expression::Interpolate {
                table: invented_table(),
                argument: argument(15.0),
            },
            vec![],
        ));
        assert_eq!(
            interpolated.value,
            Some(EvaluationValue::Quantity(Quantity {
                value: 2.0,
                dimension: Dimension::Stress,
                unit_ref: "invented_stress_unit".to_string(),
                unit_required: true,
                dimension_check_required: true,
            }))
        );

        let exact = evaluate(&input(
            Expression::Lookup {
                table: invented_table(),
                mode: LookupMode::Exact,
                argument: argument(20.0),
            },
            vec![],
        ));
        assert_eq!(
            exact.value,
            Some(EvaluationValue::Quantity(Quantity {
                value: 2.5,
                dimension: Dimension::Stress,
                unit_ref: "invented_stress_unit".to_string(),
                unit_required: true,
                dimension_check_required: true,
            }))
        );

        let step = evaluate(&input(
            Expression::Lookup {
                table: invented_table(),
                mode: LookupMode::Step,
                argument: argument(25.0),
            },
            vec![],
        ));
        assert_eq!(
            step.value,
            Some(EvaluationValue::Quantity(Quantity {
                value: 2.5,
                dimension: Dimension::Stress,
                unit_ref: "invented_stress_unit".to_string(),
                unit_required: true,
                dimension_check_required: true,
            }))
        );
    }

    #[test]
    fn blocks_out_of_range_table_arguments_without_extrapolation_or_clamping() {
        for (mode, value) in [
            (None, 5.0),
            (None, 45.0),
            (Some(LookupMode::Step), 9.5),
            (Some(LookupMode::Exact), 41.0),
        ] {
            let argument = Box::new(Expression::Literal(
                Quantity::new(value, Dimension::Temperature, "invented_temperature_unit").unwrap(),
            ));
            let expression = match mode {
                None => Expression::Interpolate {
                    table: invented_table(),
                    argument,
                },
                Some(mode) => Expression::Lookup {
                    table: invented_table(),
                    mode,
                    argument,
                },
            };
            let result = evaluate(&input(expression, vec![]));
            assert_eq!(
                result.findings[0].code,
                FindingCode::TableOutOfRange,
                "argument {value} must block out of range"
            );
            assert_eq!(result.value, None);
        }
    }

    #[test]
    fn blocks_exact_lookup_key_miss_inside_range() {
        let result = evaluate(&input(
            Expression::Lookup {
                table: invented_table(),
                mode: LookupMode::Exact,
                argument: Box::new(Expression::Literal(
                    Quantity::new(15.0, Dimension::Temperature, "invented_temperature_unit")
                        .unwrap(),
                )),
            },
            vec![],
        ));

        assert_eq!(result.findings[0].code, FindingCode::TableKeyNotFound);
        assert_eq!(result.value, None);
    }

    #[test]
    fn blocks_malformed_tables() {
        let mut non_monotone = invented_table();
        non_monotone.rows[2].argument = 12.0;
        let mut non_finite = invented_table();
        non_finite.rows[1].result = f64::NAN;
        let mut too_few_rows = invented_table();
        too_few_rows.rows.truncate(1);

        for table in [non_monotone, non_finite, too_few_rows] {
            let result = evaluate(&input(
                Expression::Interpolate {
                    table,
                    argument: Box::new(Expression::Literal(
                        Quantity::new(15.0, Dimension::Temperature, "invented_temperature_unit")
                            .unwrap(),
                    )),
                },
                vec![],
            ));
            assert_eq!(result.findings[0].code, FindingCode::TableMalformed);
            assert_eq!(result.value, None);
        }
    }

    #[test]
    fn blocks_table_argument_dimension_and_unit_mismatches() {
        let dimension_mismatch = evaluate(&input(
            Expression::Interpolate {
                table: invented_table(),
                argument: Box::new(Expression::Literal(
                    Quantity::new(15.0, Dimension::Stress, "invented_stress_unit").unwrap(),
                )),
            },
            vec![],
        ));
        assert_eq!(
            dimension_mismatch.findings[0].code,
            FindingCode::DimensionMismatch
        );

        let unit_mismatch = evaluate(&input(
            Expression::Interpolate {
                table: invented_table(),
                argument: Box::new(Expression::Literal(
                    Quantity::new(15.0, Dimension::Temperature, "other_temperature_unit").unwrap(),
                )),
            },
            vec![],
        ));
        assert_eq!(unit_mismatch.findings[0].code, FindingCode::UnitMismatch);
    }
}
