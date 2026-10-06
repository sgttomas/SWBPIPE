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

// ---------------------------------------------------------------------------
// Interval mode: conservative interval binding (T3 D2 §4.11, option C)
// ---------------------------------------------------------------------------
//
// [`evaluate_interval`] evaluates the same frozen formula language over
// enclosures instead of points. It is additive: [`evaluate`] and every
// point-path helper above are unchanged, and the interval path calls them for
// every structural decision (types, dimensions, units, tables, grammar
// version, statuses and bindings), so an expression the point path rejects
// for its structure is rejected here with the same findings.
//
// Soundness (D2 §4.11.4). Every floating operation is followed by one outward
// ulp step on each end (`next_down` on the lower end, `next_up` on the upper
// end), which emulates directed rounding. Each enclosure therefore contains
// both the exact real result and the binary64 result the point path computes,
// for every point assignment inside the input box. A predicate reads `True`
// only if it holds at every point of the box, `False` only if it fails at
// every point, and `Indeterminate` otherwise. Any part of the formula that
// has no sound finite enclosure makes the whole result indeterminate, not
// just that part: a value-dependent point-path block (a zero divisor, a table
// argument out of range, an exact-lookup miss) possible somewhere in the box,
// or a non-finite intermediate (where the point path computes infinities or
// NaN, and can fail on them). Evaluation is eager, as in the point path, so
// such a part decides the check even inside a branch that is not taken.

/// Kleene three-valued truth of an interval-mode predicate (D2 §4.11.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Truth {
    /// The predicate holds for every value in the input box.
    True,
    /// The predicate fails for every value in the input box.
    False,
    /// The predicate holds for some values and fails for others, or the
    /// result cannot be enclosed soundly. Never a pass.
    Indeterminate,
}

impl Truth {
    fn negate(self) -> Self {
        match self {
            Truth::True => Truth::False,
            Truth::False => Truth::True,
            Truth::Indeterminate => Truth::Indeterminate,
        }
    }

    fn and(self, other: Self) -> Self {
        match (self, other) {
            (Truth::False, _) | (_, Truth::False) => Truth::False,
            (Truth::True, Truth::True) => Truth::True,
            (Truth::True, Truth::Indeterminate)
            | (Truth::Indeterminate, Truth::True)
            | (Truth::Indeterminate, Truth::Indeterminate) => Truth::Indeterminate,
        }
    }

    fn or(self, other: Self) -> Self {
        match (self, other) {
            (Truth::True, _) | (_, Truth::True) => Truth::True,
            (Truth::False, Truth::False) => Truth::False,
            (Truth::False, Truth::Indeterminate)
            | (Truth::Indeterminate, Truth::False)
            | (Truth::Indeterminate, Truth::Indeterminate) => Truth::Indeterminate,
        }
    }
}

/// A closed binary64 enclosure `[lo, hi]` with finite ends and `lo <= hi`.
/// A point is the degenerate enclosure `[x, x]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Enclosure {
    pub lo: f64,
    pub hi: f64,
}

impl Enclosure {
    /// The degenerate enclosure of one exact value.
    pub fn point(value: f64) -> Self {
        Self {
            lo: value,
            hi: value,
        }
    }

    /// True when the enclosure holds a single real value.
    pub fn is_point(&self) -> bool {
        self.lo == self.hi
    }

    /// The fixed bit form used in runner findings (D2 §4.11.5):
    /// `[0x<lo bits>,0x<hi bits>]`.
    pub fn bits_text(&self) -> String {
        format!(
            "[0x{:016x},0x{:016x}]",
            self.lo.to_bits(),
            self.hi.to_bits()
        )
    }
}

/// Forms the binding enclosure of a verified row from its published value
/// `value` (q) and its listed bound `bound` (b), D2 §4.11.2:
/// `[next_down(fl(q - b)), next_up(fl(q + b))]`.
///
/// `bound == 0` binds the exact point `q` with no widening. Returns `None`
/// when `q` is not finite, when `b` is not a finite non-negative number, or
/// when an end of the enclosure is not finite. Callers validate `b` first and
/// treat `None` from a valid `b` as an indeterminate input.
pub fn enclosure_from_bound(value: f64, bound: f64) -> Option<Enclosure> {
    if !value.is_finite() || !bound.is_finite() || bound < 0.0 {
        return None;
    }
    if bound == 0.0 {
        return Some(Enclosure::point(value));
    }
    outward(value - bound, value + bound)
}

/// One outward ulp step on each end of a just-rounded floating result
/// (directed-rounding emulation, D2 §4.11.3). Any non-finite end gives `None`.
fn outward(lo: f64, hi: f64) -> Option<Enclosure> {
    if !lo.is_finite() || !hi.is_finite() {
        return None;
    }
    let (lo, hi) = (lo.next_down(), hi.next_up());
    if lo.is_finite() && hi.is_finite() {
        Some(Enclosure { lo, hi })
    } else {
        None
    }
}

/// The smaller of two values with a fixed comparison order (a tie, including
/// signed zeros, keeps the first operand), so every language computes the
/// same bits.
fn min2(a: f64, b: f64) -> f64 {
    if b < a {
        b
    } else {
        a
    }
}

/// The larger of two values with a fixed comparison order (see [`min2`]).
fn max2(a: f64, b: f64) -> f64 {
    if b > a {
        b
    } else {
        a
    }
}

fn interval_add(a: Enclosure, b: Enclosure) -> Option<Enclosure> {
    outward(a.lo + b.lo, a.hi + b.hi)
}

fn interval_subtract(a: Enclosure, b: Enclosure) -> Option<Enclosure> {
    outward(a.lo - b.hi, a.hi - b.lo)
}

fn interval_multiply(a: Enclosure, b: Enclosure) -> Option<Enclosure> {
    let products = [a.lo * b.lo, a.lo * b.hi, a.hi * b.lo, a.hi * b.hi];
    let (mut lo, mut hi) = (products[0], products[0]);
    for &product in &products[1..] {
        lo = min2(lo, product);
        hi = max2(hi, product);
    }
    outward(lo, hi)
}

/// The caller refuses a divisor enclosure that contains zero, so here
/// `b.lo > 0` or `b.hi < 0`.
fn interval_divide(a: Enclosure, b: Enclosure) -> Option<Enclosure> {
    let quotients = [a.lo / b.lo, a.lo / b.hi, a.hi / b.lo, a.hi / b.hi];
    let (mut lo, mut hi) = (quotients[0], quotients[0]);
    for &quotient in &quotients[1..] {
        lo = min2(lo, quotient);
        hi = max2(hi, quotient);
    }
    outward(lo, hi)
}

fn interval_contains_zero(e: Enclosure) -> bool {
    e.lo <= 0.0 && 0.0 <= e.hi
}

fn interval_hull(a: Enclosure, b: Enclosure) -> Enclosure {
    Enclosure {
        lo: min2(a.lo, b.lo),
        hi: max2(a.hi, b.hi),
    }
}

/// `abs` over an enclosure (exact, D2 §4.11.3).
fn interval_abs(e: Enclosure) -> Enclosure {
    if e.lo >= 0.0 {
        e
    } else if e.hi <= 0.0 {
        Enclosure {
            lo: -e.hi,
            hi: -e.lo,
        }
    } else {
        Enclosure {
            lo: 0.0,
            hi: max2(-e.lo, e.hi),
        }
    }
}

/// The six comparisons over enclosures (D2 §4.11.3). Strictness is respected
/// at the ends; `equal` is `True` only for two equal points.
fn interval_compare(
    operator: ComparisonOperator,
    left: Option<Enclosure>,
    right: Option<Enclosure>,
) -> Truth {
    let (Some(a), Some(b)) = (left, right) else {
        return Truth::Indeterminate;
    };
    match operator {
        ComparisonOperator::LessThanOrEqual => {
            if a.hi <= b.lo {
                Truth::True
            } else if a.lo > b.hi {
                Truth::False
            } else {
                Truth::Indeterminate
            }
        }
        ComparisonOperator::LessThan => {
            if a.hi < b.lo {
                Truth::True
            } else if a.lo >= b.hi {
                Truth::False
            } else {
                Truth::Indeterminate
            }
        }
        ComparisonOperator::GreaterThanOrEqual => {
            if a.lo >= b.hi {
                Truth::True
            } else if a.hi < b.lo {
                Truth::False
            } else {
                Truth::Indeterminate
            }
        }
        ComparisonOperator::GreaterThan => {
            if a.lo > b.hi {
                Truth::True
            } else if a.hi <= b.lo {
                Truth::False
            } else {
                Truth::Indeterminate
            }
        }
        ComparisonOperator::Equal => interval_equal(a, b),
        ComparisonOperator::NotEqual => interval_equal(a, b).negate(),
    }
}

fn interval_equal(a: Enclosure, b: Enclosure) -> Truth {
    if a.is_point() && b.is_point() && a.lo == b.lo {
        Truth::True
    } else if a.hi < b.lo || b.hi < a.lo {
        Truth::False
    } else {
        Truth::Indeterminate
    }
}

/// Why part of an interval-mode result has no sound finite enclosure. Any
/// note makes the whole result indeterminate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntervalNoteCode {
    /// A divisor enclosure contains zero (or has no finite enclosure): the
    /// point path would block at some value. The runner reports it as
    /// `RULE_INTERVAL_DIVIDE_BY_ZERO_RANGE` (D2 §4.11.3).
    DivideByZeroRange,
    /// An interpolation or step-lookup argument enclosure is not inside the
    /// table's closed row-argument range (or has no finite enclosure): the
    /// point path would block at some value.
    TableArgumentRange,
    /// An exact-lookup argument is not a single point (or has no finite
    /// enclosure): the point path would block at some value or select
    /// different rows.
    ExactLookupRange,
    /// An operation produced a non-finite end, or an input has no finite
    /// enclosure, so no finite enclosure exists.
    NonFiniteEnclosure,
}

impl IntervalNoteCode {
    /// Stable token shared with the Python reference evaluator and the shared
    /// case file.
    pub fn as_str(self) -> &'static str {
        match self {
            IntervalNoteCode::DivideByZeroRange => "divide_by_zero_range",
            IntervalNoteCode::TableArgumentRange => "table_argument_range",
            IntervalNoteCode::ExactLookupRange => "exact_lookup_range",
            IntervalNoteCode::NonFiniteEnclosure => "non_finite_enclosure",
        }
    }
}

/// A non-blocking interval-mode note, in evaluation order. Any note makes the
/// whole result indeterminate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntervalNote {
    pub code: IntervalNoteCode,
    pub subject_id: String,
}

/// An interval overlay for one bound variable: in interval mode the
/// variable's enclosure replaces its point value. The variable must also be
/// bound, with its point value, dimension and unit, in
/// [`EvaluationInput::bindings`]. `enclosure: None` marks a verified input
/// whose range has no finite binary64 enclosure (indeterminate).
#[derive(Debug, Clone, PartialEq)]
pub struct IntervalBinding {
    pub variable_id: String,
    pub enclosure: Option<Enclosure>,
}

/// An interval-mode quantity. `enclosure: None` means no sound finite
/// enclosure exists (an indeterminate quantity).
#[derive(Debug, Clone, PartialEq)]
pub struct IntervalQuantity {
    pub enclosure: Option<Enclosure>,
    pub dimension: Dimension,
    pub unit_ref: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntervalValue {
    Quantity(IntervalQuantity),
    Boolean(Truth),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntervalEvaluationResult {
    /// `None` exactly when `findings` is non-empty (blocked), as in
    /// [`EvaluationResult`].
    pub value: Option<IntervalValue>,
    pub statuses: Vec<AnalysisStatus>,
    pub source_variable_ids: Vec<String>,
    /// Blocking findings, identical in kind and wording to the point path's.
    pub findings: Vec<EvaluationFinding>,
    /// Non-blocking notes, in evaluation order. When any is present the value
    /// is indeterminate: `Truth::Indeterminate`, or a quantity with no
    /// enclosure.
    pub notes: Vec<IntervalNote>,
}

impl IntervalEvaluationResult {
    pub fn is_blocked(&self) -> bool {
        !self.findings.is_empty()
    }
}

/// Interval mode over the whole formula language (D2 §4.11.3).
///
/// `input` is read exactly as [`evaluate`] reads it; `intervals` overlays
/// enclosures on some of its bound variables. With no overlays every input is
/// a point, but every floating operation still widens outward, so this is not
/// a substitute for [`evaluate`]: callers use interval mode only for a check
/// with at least one interval input (D2 §4.11.1).
pub fn evaluate_interval(
    input: &EvaluationInput,
    intervals: &[IntervalBinding],
) -> IntervalEvaluationResult {
    let mut findings = Vec::new();
    check_grammar_version(&input.declared_grammar_version, &mut findings);
    let statuses = collect_statuses(&input.statuses, &mut findings);
    let binding_map = build_binding_map(&input.bindings, &mut findings);
    check_required_variables(&input.required_variable_ids, &binding_map, &mut findings);
    let overlays = build_interval_overlays(intervals, &binding_map, &mut findings);

    let mut state = IntervalState {
        source_variable_ids: Vec::new(),
        findings,
        notes: Vec::new(),
    };
    let env = IntervalEnv {
        bindings: &binding_map,
        overlays: &overlays,
    };
    let value = eval_interval_expression(&input.expression, &env, &mut state);

    let IntervalState {
        mut source_variable_ids,
        findings,
        notes,
    } = state;
    source_variable_ids.sort();
    source_variable_ids.dedup();

    let value = if findings.is_empty() {
        value.map(|value| value.into_public(!notes.is_empty()))
    } else {
        None
    };
    IntervalEvaluationResult {
        value,
        statuses,
        source_variable_ids,
        findings,
        notes,
    }
}

fn build_interval_overlays(
    intervals: &[IntervalBinding],
    bindings: &HashMap<&str, &VariableBinding>,
    findings: &mut Vec<EvaluationFinding>,
) -> HashMap<String, Option<Enclosure>> {
    let mut overlays = HashMap::new();
    for interval in intervals {
        let variable_id = interval.variable_id.as_str();
        if variable_id.trim().is_empty() {
            findings.push(EvaluationFinding::new(
                FindingCode::InvalidReference,
                "interval_binding",
                "interval binding id must not be empty",
            ));
            continue;
        }
        if overlays.contains_key(variable_id) {
            findings.push(EvaluationFinding::new(
                FindingCode::DuplicateBinding,
                variable_id,
                "duplicate interval binding",
            ));
            continue;
        }
        match bindings.get(variable_id) {
            Some(binding) if binding.quantity.is_some() => {}
            _ => {
                findings.push(EvaluationFinding::new(
                    FindingCode::InvalidReference,
                    variable_id,
                    "interval binding names no bound variable with a supplied value",
                ));
                continue;
            }
        }
        if let Some(enclosure) = interval.enclosure {
            if !enclosure.lo.is_finite() || !enclosure.hi.is_finite() {
                findings.push(EvaluationFinding::new(
                    FindingCode::NonFiniteInput,
                    variable_id,
                    "interval binding ends must be finite",
                ));
                continue;
            }
            if enclosure.lo > enclosure.hi {
                findings.push(EvaluationFinding::new(
                    FindingCode::InvalidReference,
                    variable_id,
                    "interval binding lower end exceeds its upper end",
                ));
                continue;
            }
        }
        overlays.insert(variable_id.to_string(), interval.enclosure);
    }
    overlays
}

struct IntervalEnv<'a> {
    bindings: &'a HashMap<&'a str, &'a VariableBinding>,
    overlays: &'a HashMap<String, Option<Enclosure>>,
}

struct IntervalState {
    source_variable_ids: Vec<String>,
    findings: Vec<EvaluationFinding>,
    notes: Vec<IntervalNote>,
}

impl IntervalState {
    fn note(&mut self, code: IntervalNoteCode, subject_id: impl Into<String>) {
        self.notes.push(IntervalNote {
            code,
            subject_id: subject_id.into(),
        });
    }

    /// Notes a non-finite operation result and passes the enclosure through.
    fn finite(&mut self, enclosure: Option<Enclosure>, subject_id: &str) -> Option<Enclosure> {
        if enclosure.is_none() {
            self.note(IntervalNoteCode::NonFiniteEnclosure, subject_id);
        }
        enclosure
    }
}

/// Internal interval value. A quantity keeps the point path's metadata
/// (`meta`, whose `value` is never read) beside its enclosure.
#[derive(Debug, Clone)]
enum IValue {
    Quantity {
        meta: Quantity,
        enclosure: Option<Enclosure>,
    },
    Boolean(Truth),
}

impl IValue {
    /// A point-path value of the same kind and metadata, used to run the
    /// point path's own structural checks. Its numeric value is a non-zero
    /// placeholder and never decides anything.
    fn shadow(&self) -> EvaluationValue {
        match self {
            IValue::Quantity { meta, .. } => EvaluationValue::Quantity(meta.with_value(1.0)),
            IValue::Boolean(_) => EvaluationValue::Boolean(true),
        }
    }

    /// `indeterminate`: some part had no sound finite enclosure (a note).
    fn into_public(self, indeterminate: bool) -> IntervalValue {
        match self {
            IValue::Quantity { meta, enclosure } => IntervalValue::Quantity(IntervalQuantity {
                enclosure: if indeterminate { None } else { enclosure },
                dimension: meta.dimension,
                unit_ref: meta.unit_ref,
            }),
            IValue::Boolean(truth) => IntervalValue::Boolean(if indeterminate {
                Truth::Indeterminate
            } else {
                truth
            }),
        }
    }
}

/// Unreachable after a successful point-path structural check; it keeps the
/// operator matches exhaustive with no default arm, and blocks if reached.
fn shadow_kind_mismatch(state: &mut IntervalState, subject_id: &str) -> Option<IValue> {
    state.findings.push(EvaluationFinding::new(
        FindingCode::TypeMismatch,
        subject_id,
        "interval operand kind does not match the checked expression kind",
    ));
    None
}

fn eval_interval_expression(
    expression: &Expression,
    env: &IntervalEnv,
    state: &mut IntervalState,
) -> Option<IValue> {
    match expression {
        Expression::Literal(_) => {
            // The point path's own literal checks; a literal is a point.
            match eval_expression(
                expression,
                env.bindings,
                &mut state.source_variable_ids,
                &mut state.findings,
            )? {
                EvaluationValue::Quantity(quantity) => Some(IValue::Quantity {
                    enclosure: Some(Enclosure::point(quantity.value)),
                    meta: quantity,
                }),
                EvaluationValue::Boolean(_) => shadow_kind_mismatch(state, "literal"),
            }
        }
        Expression::VariableRef(variable_id) => {
            match eval_variable_ref(
                variable_id,
                env.bindings,
                &mut state.source_variable_ids,
                &mut state.findings,
            )? {
                EvaluationValue::Quantity(quantity) => {
                    let enclosure = match env.overlays.get(variable_id.as_str()) {
                        Some(Some(enclosure)) => Some(*enclosure),
                        Some(None) => {
                            state.note(IntervalNoteCode::NonFiniteEnclosure, variable_id);
                            None
                        }
                        None => Some(Enclosure::point(quantity.value)),
                    };
                    Some(IValue::Quantity {
                        meta: quantity,
                        enclosure,
                    })
                }
                EvaluationValue::Boolean(_) => shadow_kind_mismatch(state, variable_id),
            }
        }
        Expression::Unary { operator, operand } => {
            let value = eval_interval_expression(operand, env, state)?;
            let checked = eval_unary(*operator, value.shadow(), &mut state.findings)?;
            match (operator, value, checked) {
                (
                    UnaryOperator::Negate,
                    IValue::Quantity { enclosure, .. },
                    EvaluationValue::Quantity(meta),
                ) => Some(IValue::Quantity {
                    meta,
                    enclosure: enclosure.map(|e| Enclosure {
                        lo: -e.hi,
                        hi: -e.lo,
                    }),
                }),
                (
                    UnaryOperator::Abs,
                    IValue::Quantity { enclosure, .. },
                    EvaluationValue::Quantity(meta),
                ) => Some(IValue::Quantity {
                    meta,
                    enclosure: enclosure.map(interval_abs),
                }),
                (UnaryOperator::Not, IValue::Boolean(truth), EvaluationValue::Boolean(_)) => {
                    Some(IValue::Boolean(truth.negate()))
                }
                (UnaryOperator::Negate, _, _)
                | (UnaryOperator::Abs, _, _)
                | (UnaryOperator::Not, _, _) => shadow_kind_mismatch(state, "unary"),
            }
        }
        Expression::Binary {
            operator,
            left,
            right,
        } => {
            let left = eval_interval_expression(left, env, state)?;
            let right = eval_interval_expression(right, env, state)?;
            let checked = eval_binary(
                *operator,
                left.shadow(),
                right.shadow(),
                &mut state.findings,
            )?;
            let (
                IValue::Quantity {
                    enclosure: left_enclosure,
                    ..
                },
                IValue::Quantity {
                    enclosure: right_enclosure,
                    ..
                },
                EvaluationValue::Quantity(meta),
            ) = (left, right, checked)
            else {
                return shadow_kind_mismatch(state, "binary_expression");
            };
            let enclosure = match operator {
                BinaryOperator::Add => match (left_enclosure, right_enclosure) {
                    (Some(a), Some(b)) => state.finite(interval_add(a, b), "add"),
                    _ => None,
                },
                BinaryOperator::Subtract => match (left_enclosure, right_enclosure) {
                    (Some(a), Some(b)) => state.finite(interval_subtract(a, b), "subtract"),
                    _ => None,
                },
                BinaryOperator::Multiply => match (left_enclosure, right_enclosure) {
                    (Some(a), Some(b)) => state.finite(interval_multiply(a, b), "multiply"),
                    _ => None,
                },
                BinaryOperator::Divide => match right_enclosure {
                    // A divisor that may be zero somewhere in the box, or has
                    // no finite enclosure, would block the point path there.
                    Some(b) if !interval_contains_zero(b) => match left_enclosure {
                        Some(a) => state.finite(interval_divide(a, b), "divide"),
                        None => None,
                    },
                    Some(_) | None => {
                        state.note(IntervalNoteCode::DivideByZeroRange, "divide");
                        None
                    }
                },
            };
            Some(IValue::Quantity { meta, enclosure })
        }
        Expression::Compare {
            operator,
            left,
            right,
        } => {
            let left = eval_interval_expression(left, env, state)?;
            let right = eval_interval_expression(right, env, state)?;
            eval_compare(
                *operator,
                left.shadow(),
                right.shadow(),
                &mut state.findings,
            )?;
            let (
                IValue::Quantity {
                    enclosure: left_enclosure,
                    ..
                },
                IValue::Quantity {
                    enclosure: right_enclosure,
                    ..
                },
            ) = (left, right)
            else {
                return shadow_kind_mismatch(state, "comparison");
            };
            Some(IValue::Boolean(interval_compare(
                *operator,
                left_enclosure,
                right_enclosure,
            )))
        }
        Expression::Logical {
            operator,
            left,
            right,
        } => {
            // Eager, as in the point path.
            let left = eval_interval_expression(left, env, state)?;
            let right = eval_interval_expression(right, env, state)?;
            eval_logical(
                *operator,
                left.shadow(),
                right.shadow(),
                &mut state.findings,
            )?;
            let (IValue::Boolean(left), IValue::Boolean(right)) = (left, right) else {
                return shadow_kind_mismatch(state, "logical_expression");
            };
            Some(IValue::Boolean(match operator {
                LogicalOperator::And => left.and(right),
                LogicalOperator::Or => left.or(right),
            }))
        }
        Expression::Select {
            condition,
            then_branch,
            else_branch,
        } => {
            // Eager, as in the point path: all three are always evaluated.
            let condition = eval_interval_expression(condition, env, state)?;
            let then_value = eval_interval_expression(then_branch, env, state)?;
            let else_value = eval_interval_expression(else_branch, env, state)?;
            eval_select(
                condition.shadow(),
                then_value.shadow(),
                else_value.shadow(),
                &mut state.findings,
            )?;
            let IValue::Boolean(condition) = condition else {
                return shadow_kind_mismatch(state, "select_condition");
            };
            match condition {
                Truth::True => Some(then_value),
                Truth::False => Some(else_value),
                Truth::Indeterminate => match (then_value, else_value) {
                    (IValue::Boolean(then_truth), IValue::Boolean(else_truth)) => {
                        Some(IValue::Boolean(if then_truth == else_truth {
                            then_truth
                        } else {
                            Truth::Indeterminate
                        }))
                    }
                    (
                        IValue::Quantity {
                            meta,
                            enclosure: then_enclosure,
                        },
                        IValue::Quantity {
                            enclosure: else_enclosure,
                            ..
                        },
                    ) => Some(IValue::Quantity {
                        meta,
                        enclosure: match (then_enclosure, else_enclosure) {
                            (Some(a), Some(b)) => Some(interval_hull(a, b)),
                            _ => None,
                        },
                    }),
                    (IValue::Boolean(_), IValue::Quantity { .. })
                    | (IValue::Quantity { .. }, IValue::Boolean(_)) => {
                        shadow_kind_mismatch(state, "select_branches")
                    }
                },
            }
        }
        Expression::Aggregate { function, operands } => {
            eval_interval_aggregate(*function, operands, env, state)
        }
        Expression::Interpolate { table, argument } => {
            eval_interval_table(table, None, argument, env, state)
        }
        Expression::Lookup {
            table,
            mode,
            argument,
        } => eval_interval_table(table, Some(*mode), argument, env, state),
        Expression::UnsupportedForm { .. } | Expression::UnsafeHostAccess { .. } => {
            // Blocked exactly as in the point path.
            eval_expression(
                expression,
                env.bindings,
                &mut state.source_variable_ids,
                &mut state.findings,
            );
            None
        }
    }
}

/// Mirrors [`eval_aggregate`]'s checks and their order, then takes the
/// endpoint-wise min or max (exact).
fn eval_interval_aggregate(
    function: AggregateFunction,
    operands: &[Expression],
    env: &IntervalEnv,
    state: &mut IntervalState,
) -> Option<IValue> {
    let subject_id = match function {
        AggregateFunction::Min => "min",
        AggregateFunction::Max => "max",
    };
    if operands.is_empty() {
        state.findings.push(EvaluationFinding::new(
            FindingCode::UnsupportedExpressionForm,
            subject_id,
            "min/max requires at least one operand",
        ));
        return None;
    }

    let mut values: Vec<(Quantity, Option<Enclosure>)> = Vec::with_capacity(operands.len());
    for operand in operands {
        let value = eval_interval_expression(operand, env, state)?;
        let IValue::Quantity { meta, enclosure } = value else {
            state.findings.push(EvaluationFinding::new(
                FindingCode::TypeMismatch,
                subject_id,
                "min/max operands must be numeric quantities",
            ));
            return None;
        };
        values.push((meta, enclosure));
    }

    let first = values[0].0.clone();
    for (meta, _) in &values[1..] {
        if meta.dimension != first.dimension {
            state.findings.push(dimension_mismatch(
                subject_id,
                first.dimension,
                meta.dimension,
            ));
            return None;
        }
        if !quantity_units_match(&first, meta) {
            state.findings.push(unit_mismatch(subject_id, &first, meta));
            return None;
        }
    }

    let mut selected = values[0].1;
    for (_, enclosure) in &values[1..] {
        selected = match (selected, enclosure) {
            (Some(a), Some(b)) => Some(match function {
                AggregateFunction::Min => Enclosure {
                    lo: min2(a.lo, b.lo),
                    hi: min2(a.hi, b.hi),
                },
                AggregateFunction::Max => Enclosure {
                    lo: max2(a.lo, b.lo),
                    hi: max2(a.hi, b.hi),
                },
            }),
            _ => None,
        };
    }
    Some(IValue::Quantity {
        meta: first,
        enclosure: selected,
    })
}

/// Mirrors [`eval_table_expression`]'s checks and their order; the value part
/// follows D2 §4.11.3.
fn eval_interval_table(
    table: &UserTable,
    mode: Option<LookupMode>,
    argument: &Expression,
    env: &IntervalEnv,
    state: &mut IntervalState,
) -> Option<IValue> {
    let minimum_rows = if mode.is_none() { 2 } else { 1 };
    let table_valid = validate_table(table, minimum_rows, &mut state.findings);
    let argument_value = eval_interval_expression(argument, env, state);

    let argument_value = argument_value?;
    if !table_valid {
        return None;
    }
    let subject_id = table.table_id.trim().to_string();

    let IValue::Quantity {
        meta: argument_meta,
        enclosure: argument_enclosure,
    } = argument_value
    else {
        state.findings.push(EvaluationFinding::new(
            FindingCode::TypeMismatch,
            &subject_id,
            "table argument must be a numeric quantity",
        ));
        return None;
    };
    if argument_meta.dimension != table.argument_dimension {
        state.findings.push(dimension_mismatch(
            &subject_id,
            argument_meta.dimension,
            table.argument_dimension,
        ));
        return None;
    }
    if argument_meta.unit_ref.trim() != table.argument_unit_ref.trim() {
        state.findings.push(EvaluationFinding::new(
            FindingCode::UnitMismatch,
            &subject_id,
            format!(
                "unit mismatch: argument={}, table={}",
                argument_meta.unit_ref, table.argument_unit_ref
            ),
        ));
        return None;
    }

    let first = table.rows[0].argument;
    let last = table.rows[table.rows.len() - 1].argument;

    let enclosure = match mode {
        Some(LookupMode::Exact) => match argument_enclosure {
            Some(argument) if argument.is_point() => {
                // A point argument follows the point path exactly.
                let x = argument.lo;
                if let Some(row) = table.rows.iter().find(|row| row.argument == x) {
                    Some(Enclosure::point(row.result))
                } else if x < first || x > last {
                    state
                        .findings
                        .push(table_out_of_range(&subject_id, x, first, last));
                    return None;
                } else {
                    state.findings.push(EvaluationFinding::new(
                        FindingCode::TableKeyNotFound,
                        &subject_id,
                        "exact lookup argument matches no table row argument",
                    ));
                    return None;
                }
            }
            Some(_) | None => {
                state.note(IntervalNoteCode::ExactLookupRange, &subject_id);
                None
            }
        },
        Some(LookupMode::Step) => match argument_enclosure {
            Some(argument) if first <= argument.lo && argument.hi <= last => {
                Some(step_lookup_enclosure(&table.rows, argument))
            }
            Some(_) | None => {
                state.note(IntervalNoteCode::TableArgumentRange, &subject_id);
                None
            }
        },
        None => match argument_enclosure {
            Some(argument) if first <= argument.lo && argument.hi <= last => {
                let enclosure = interpolate_enclosure(&table.rows, argument);
                state.finite(enclosure, &subject_id)
            }
            Some(_) | None => {
                state.note(IntervalNoteCode::TableArgumentRange, &subject_id);
                None
            }
        },
    };

    Some(IValue::Quantity {
        meta: Quantity {
            value: 1.0,
            dimension: table.result_dimension,
            unit_ref: table.result_unit_ref.trim().to_string(),
            unit_required: true,
            dimension_check_required: true,
        },
        enclosure,
    })
}

/// Index of the row that governs a step lookup at `x` (the last row whose
/// argument is `<= x`); `x` is inside the table range.
fn step_governing_row(rows: &[TableRow], x: f64) -> usize {
    let mut index = 0;
    for (candidate, row) in rows.iter().enumerate() {
        if row.argument <= x {
            index = candidate;
        }
    }
    index
}

/// The hull of the results of every row the argument enclosure spans. Row
/// results are exact, so no widening applies.
fn step_lookup_enclosure(rows: &[TableRow], argument: Enclosure) -> Enclosure {
    let from = step_governing_row(rows, argument.lo);
    let to = step_governing_row(rows, argument.hi);
    let mut enclosure = Enclosure::point(rows[from].result);
    for row in &rows[from + 1..=to] {
        enclosure = interval_hull(enclosure, Enclosure::point(row.result));
    }
    enclosure
}

/// Interpolation over an argument enclosure inside the table range.
///
/// A point argument equal to a row argument gives that row's exact result, as
/// the point path does. Otherwise, for every segment the argument range meets,
/// the point path's formula `low.result + (high.result - low.result) *
/// ((x - low.argument) / (high.argument - low.argument))` is evaluated over
/// the clipped argument range with an outward step after each of its floating
/// operations, and the segment enclosures are joined. Each segment enclosure
/// contains the exact line and the point path's binary64 values on that
/// segment, including the row values at its ends. `None` when an end is not
/// finite.
fn interpolate_enclosure(rows: &[TableRow], argument: Enclosure) -> Option<Enclosure> {
    if argument.is_point() {
        if let Some(row) = rows.iter().find(|row| row.argument == argument.lo) {
            return Some(Enclosure::point(row.result));
        }
    }
    let mut joined: Option<Enclosure> = None;
    for pair in rows.windows(2) {
        let (low, high) = (pair[0], pair[1]);
        if !(low.argument < argument.hi && argument.lo < high.argument) {
            continue;
        }
        let clipped = Enclosure {
            lo: max2(argument.lo, low.argument),
            hi: min2(argument.hi, high.argument),
        };
        let segment = interpolate_segment(low, high, clipped)?;
        joined = Some(match joined {
            Some(enclosure) => interval_hull(enclosure, segment),
            None => segment,
        });
    }
    joined
}

fn interpolate_segment(low: TableRow, high: TableRow, x: Enclosure) -> Option<Enclosure> {
    let rise = interval_subtract(Enclosure::point(high.result), Enclosure::point(low.result))?;
    let offset = interval_subtract(x, Enclosure::point(low.argument))?;
    let run = interval_subtract(
        Enclosure::point(high.argument),
        Enclosure::point(low.argument),
    )?;
    if interval_contains_zero(run) {
        return None;
    }
    let fraction = interval_divide(offset, run)?;
    let scaled = interval_multiply(rise, fraction)?;
    interval_add(Enclosure::point(low.result), scaled)
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

/// Interval mode (T3 D2 §4.11): per-operator enclosure bits, three-valued
/// outcomes, structural parity with the point path, and a seeded soundness
/// property against the unchanged point path. All values are invented.
#[cfg(test)]
mod interval_tests {
    use super::*;

    const UNIT: &str = "invented_stress_unit";

    fn nd(x: f64) -> f64 {
        x.next_down()
    }

    fn nu(x: f64) -> f64 {
        x.next_up()
    }

    fn enc(lo: f64, hi: f64) -> Enclosure {
        Enclosure { lo, hi }
    }

    fn stress(value: f64) -> Expression {
        Expression::Literal(Quantity::new(value, Dimension::Stress, UNIT).unwrap())
    }

    fn ratio(value: f64) -> Expression {
        Expression::Literal(Quantity::dimensionless(value, "ratio").unwrap())
    }

    fn var(id: &str) -> Expression {
        Expression::VariableRef(id.to_string())
    }

    fn bin(operator: BinaryOperator, left: Expression, right: Expression) -> Expression {
        Expression::Binary {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn cmp(operator: ComparisonOperator, left: Expression, right: Expression) -> Expression {
        Expression::Compare {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn logical(operator: LogicalOperator, left: Expression, right: Expression) -> Expression {
        Expression::Logical {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn unary(operator: UnaryOperator, operand: Expression) -> Expression {
        Expression::Unary {
            operator,
            operand: Box::new(operand),
        }
    }

    fn select(
        condition: Expression,
        then_branch: Expression,
        else_branch: Expression,
    ) -> Expression {
        Expression::Select {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        }
    }

    /// One input: `(id, dimension, unit, q, b)`. `b > 0` adds an overlay
    /// `enclosure_from_bound(q, b)`; `b == 0` leaves a point.
    type Input<'a> = (&'a str, Dimension, &'a str, f64, f64);

    fn run(expression: Expression, inputs: &[Input]) -> IntervalEvaluationResult {
        let mut bindings = Vec::new();
        let mut intervals = Vec::new();
        for &(id, dimension, unit, q, b) in inputs {
            bindings.push(VariableBinding::new(
                id,
                BindingSource::SolverResultField,
                Quantity::new(q, dimension, unit).unwrap(),
            ));
            if b > 0.0 {
                intervals.push(IntervalBinding {
                    variable_id: id.to_string(),
                    enclosure: enclosure_from_bound(q, b),
                });
            }
        }
        evaluate_interval(
            &EvaluationInput {
                expression,
                bindings,
                required_variable_ids: vec![],
                statuses: vec![AnalysisStatus::MechanicsSolved],
                declared_grammar_version: GRAMMAR_VERSION.to_string(),
            },
            &intervals,
        )
    }

    fn sx(q: f64, b: f64) -> Input<'static> {
        ("x", Dimension::Stress, UNIT, q, b)
    }

    fn sy(q: f64, b: f64) -> Input<'static> {
        ("y", Dimension::Stress, UNIT, q, b)
    }

    fn rz(q: f64, b: f64) -> Input<'static> {
        ("z", Dimension::Dimensionless, "ratio", q, b)
    }

    fn truth(result: &IntervalEvaluationResult) -> Truth {
        assert!(result.findings.is_empty(), "blocked: {:?}", result.findings);
        match result.value {
            Some(IntervalValue::Boolean(truth)) => truth,
            ref other => panic!("expected a truth, got {other:?}"),
        }
    }

    fn enclosure(result: &IntervalEvaluationResult) -> Option<Enclosure> {
        assert!(result.findings.is_empty(), "blocked: {:?}", result.findings);
        match &result.value {
            Some(IntervalValue::Quantity(quantity)) => quantity.enclosure,
            other => panic!("expected a quantity, got {other:?}"),
        }
    }

    fn note_codes(result: &IntervalEvaluationResult) -> Vec<IntervalNoteCode> {
        result.notes.iter().map(|note| note.code).collect()
    }

    fn assert_bits(actual: Option<Enclosure>, lo: f64, hi: f64) {
        let actual = actual.expect("expected an enclosure");
        assert_eq!(
            (actual.lo.to_bits(), actual.hi.to_bits()),
            (lo.to_bits(), hi.to_bits()),
            "enclosure {} != {}",
            actual.bits_text(),
            enc(lo, hi).bits_text()
        );
    }

    #[test]
    fn bound_forms_outward_ends_and_zero_bound_is_an_exact_point() {
        assert_eq!(enclosure_from_bound(1.0, 0.0), Some(Enclosure::point(1.0)));
        assert_eq!(enclosure_from_bound(1.0, 0.5), Some(enc(nd(0.5), nu(1.5))));
        // A subnormal bound needs no special case (D2 §4.11.2).
        let tiny = f64::from_bits(1);
        assert_eq!(
            enclosure_from_bound(0.0, tiny),
            Some(enc(nd(-tiny), nu(tiny)))
        );
        assert_eq!(enclosure_from_bound(1.0, -0.5), None);
        assert_eq!(enclosure_from_bound(1.0, f64::NAN), None);
        assert_eq!(enclosure_from_bound(1.0, f64::INFINITY), None);
        assert_eq!(enclosure_from_bound(f64::MAX, f64::MAX), None);
        assert_eq!(
            enc(1.0, 2.0).bits_text(),
            "[0x3ff0000000000000,0x4000000000000000]"
        );
    }

    #[test]
    fn every_floating_operation_steps_outward_even_on_points() {
        let sum = run(bin(BinaryOperator::Add, stress(1.0), stress(2.0)), &[]);
        assert_bits(enclosure(&sum), nd(3.0), nu(3.0));
        let difference = run(bin(BinaryOperator::Subtract, stress(1.0), stress(2.0)), &[]);
        assert_bits(enclosure(&difference), nd(-1.0), nu(-1.0));
        let product = run(bin(BinaryOperator::Multiply, ratio(3.0), stress(2.0)), &[]);
        assert_bits(enclosure(&product), nd(6.0), nu(6.0));
        let quotient = run(bin(BinaryOperator::Divide, stress(1.0), ratio(3.0)), &[]);
        assert_bits(enclosure(&quotient), nd(1.0 / 3.0), nu(1.0 / 3.0));
        // Negation, abs, min and max are exact: no step.
        let negated = run(unary(UnaryOperator::Negate, stress(2.0)), &[]);
        assert_bits(enclosure(&negated), -2.0, -2.0);
    }

    #[test]
    fn rounding_that_hides_a_real_excess_is_not_a_pass() {
        // fl(1 + 2^-53) = 1 rounds the exact sum down onto the limit; the
        // outward step keeps the exact sum inside, so `<=` cannot be True.
        let half_ulp = 2f64.powi(-53);
        let result = run(
            cmp(
                ComparisonOperator::LessThanOrEqual,
                bin(BinaryOperator::Add, var("x"), stress(half_ulp)),
                stress(1.0),
            ),
            &[sx(1.0, 0.0)],
        );
        assert_eq!(truth(&result), Truth::Indeterminate);
    }

    #[test]
    fn interval_arithmetic_encloses_every_sign_combination() {
        let x = sx(-1.0, 2.0); // about [-3, 1]
        let z = rz(2.0, 1.0); // about [1, 3]
        let xe = enclosure_from_bound(-1.0, 2.0).unwrap();
        let ze = enclosure_from_bound(2.0, 1.0).unwrap();
        let product = run(bin(BinaryOperator::Multiply, var("z"), var("x")), &[x, z]);
        let candidates = [ze.lo * xe.lo, ze.lo * xe.hi, ze.hi * xe.lo, ze.hi * xe.hi];
        let lo = candidates.iter().cloned().fold(f64::INFINITY, f64::min);
        let hi = candidates.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        assert_bits(enclosure(&product), nd(lo), nu(hi));

        let quotient = run(bin(BinaryOperator::Divide, var("x"), var("z")), &[x, z]);
        let candidates = [xe.lo / ze.lo, xe.lo / ze.hi, xe.hi / ze.lo, xe.hi / ze.hi];
        let lo = candidates.iter().cloned().fold(f64::INFINITY, f64::min);
        let hi = candidates.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        assert_bits(enclosure(&quotient), nd(lo), nu(hi));

        let sum = run(
            bin(BinaryOperator::Add, var("x"), var("y")),
            &[x, sy(10.0, 1.0)],
        );
        let ye = enclosure_from_bound(10.0, 1.0).unwrap();
        assert_bits(enclosure(&sum), nd(xe.lo + ye.lo), nu(xe.hi + ye.hi));
        let difference = run(
            bin(BinaryOperator::Subtract, var("x"), var("y")),
            &[x, sy(10.0, 1.0)],
        );
        assert_bits(enclosure(&difference), nd(xe.lo - ye.hi), nu(xe.hi - ye.lo));
    }

    #[test]
    fn abs_covers_its_three_branches() {
        let straddle = run(unary(UnaryOperator::Abs, var("x")), &[sx(-1.0, 2.0)]);
        let xe = enclosure_from_bound(-1.0, 2.0).unwrap();
        assert_bits(enclosure(&straddle), 0.0, -xe.lo);
        let negative = run(unary(UnaryOperator::Abs, var("x")), &[sx(-5.0, 1.0)]);
        let ne = enclosure_from_bound(-5.0, 1.0).unwrap();
        assert_bits(enclosure(&negative), -ne.hi, -ne.lo);
        let positive = run(unary(UnaryOperator::Abs, var("x")), &[sx(5.0, 1.0)]);
        let pe = enclosure_from_bound(5.0, 1.0).unwrap();
        assert_bits(enclosure(&positive), pe.lo, pe.hi);
        // Negative control: abs(x) >= c with x straddling 0 is never True.
        let control = run(
            cmp(
                ComparisonOperator::GreaterThanOrEqual,
                unary(UnaryOperator::Abs, var("x")),
                stress(0.5),
            ),
            &[sx(0.0, 1.0)],
        );
        assert_eq!(truth(&control), Truth::Indeterminate);
    }

    #[test]
    fn comparisons_are_three_valued_with_strict_ends() {
        use ComparisonOperator::*;
        // x in about [9, 11].
        let x = sx(10.0, 1.0);
        let cases = [
            (LessThanOrEqual, 12.0, Truth::True),
            (LessThanOrEqual, 8.0, Truth::False),
            (LessThanOrEqual, 10.0, Truth::Indeterminate),
            (LessThan, 12.0, Truth::True),
            (LessThan, 8.0, Truth::False),
            (LessThan, 10.0, Truth::Indeterminate),
            (GreaterThanOrEqual, 8.0, Truth::True),
            (GreaterThanOrEqual, 12.0, Truth::False),
            (GreaterThanOrEqual, 10.0, Truth::Indeterminate),
            (GreaterThan, 8.0, Truth::True),
            (GreaterThan, 12.0, Truth::False),
            (GreaterThan, 10.0, Truth::Indeterminate),
            (Equal, 12.0, Truth::False),
            (Equal, 10.0, Truth::Indeterminate),
            (NotEqual, 12.0, Truth::True),
            (NotEqual, 10.0, Truth::Indeterminate),
        ];
        for (operator, limit, expected) in cases {
            let result = run(cmp(operator, var("x"), stress(limit)), &[x]);
            assert_eq!(truth(&result), expected, "{operator:?} {limit}");
        }
        // Strictness at a shared end: [a, b] < [b, c] is not True, <= is.
        let xe = enclosure_from_bound(10.0, 1.0).unwrap();
        let at_end = run(cmp(LessThan, var("x"), stress(xe.hi)), &[x]);
        assert_eq!(truth(&at_end), Truth::Indeterminate);
        let at_end = run(cmp(LessThanOrEqual, var("x"), stress(xe.hi)), &[x]);
        assert_eq!(truth(&at_end), Truth::True);
        let at_end = run(cmp(GreaterThan, var("x"), stress(xe.lo)), &[x]);
        assert_eq!(truth(&at_end), Truth::Indeterminate);
        let at_end = run(cmp(GreaterThanOrEqual, var("x"), stress(xe.lo)), &[x]);
        assert_eq!(truth(&at_end), Truth::True);
        // Equality is True only for two equal points.
        let points = run(cmp(Equal, var("x"), stress(10.0)), &[sx(10.0, 0.0)]);
        assert_eq!(truth(&points), Truth::True);
        let points = run(cmp(NotEqual, var("x"), stress(10.0)), &[sx(10.0, 0.0)]);
        assert_eq!(truth(&points), Truth::False);
    }

    #[test]
    fn kleene_logic_and_negative_controls() {
        use ComparisonOperator::*;
        let x = sx(10.0, 1.0);
        let unknown = || cmp(LessThanOrEqual, var("x"), stress(10.0));
        let yes = || cmp(LessThanOrEqual, var("x"), stress(100.0));
        let no = || cmp(GreaterThan, var("x"), stress(100.0));
        let and = |l, r| logical(LogicalOperator::And, l, r);
        let or = |l, r| logical(LogicalOperator::Or, l, r);
        assert_eq!(truth(&run(and(no(), unknown()), &[x])), Truth::False);
        assert_eq!(
            truth(&run(and(yes(), unknown()), &[x])),
            Truth::Indeterminate
        );
        assert_eq!(truth(&run(and(yes(), yes()), &[x])), Truth::True);
        assert_eq!(truth(&run(or(yes(), unknown()), &[x])), Truth::True);
        assert_eq!(truth(&run(or(no(), unknown()), &[x])), Truth::Indeterminate);
        assert_eq!(truth(&run(or(no(), no()), &[x])), Truth::False);
        assert_eq!(
            truth(&run(unary(UnaryOperator::Not, unknown()), &[x])),
            Truth::Indeterminate
        );
        assert_eq!(
            truth(&run(unary(UnaryOperator::Not, yes()), &[x])),
            Truth::False
        );
        // Negative control: not(x > c) with x straddling c.
        let control = run(
            unary(UnaryOperator::Not, cmp(GreaterThan, var("x"), stress(10.0))),
            &[x],
        );
        assert_eq!(truth(&control), Truth::Indeterminate);
        // Negative control: x*x <= c where the box's square straddles c.
        let control = run(
            cmp(
                LessThanOrEqual,
                bin(
                    BinaryOperator::Multiply,
                    bin(BinaryOperator::Divide, var("x"), stress(1.0)),
                    var("x"),
                ),
                stress(1.0),
            ),
            &[sx(0.0, 1.5)],
        );
        assert_eq!(truth(&control), Truth::Indeterminate);
    }

    #[test]
    fn select_takes_a_branch_or_the_hull() {
        use ComparisonOperator::*;
        let x = sx(10.0, 1.0);
        let unknown = || cmp(LessThanOrEqual, var("x"), stress(10.0));
        let yes = || cmp(LessThanOrEqual, var("x"), stress(100.0));
        let chosen = run(select(yes(), stress(1.0), stress(2.0)), &[x]);
        assert_bits(enclosure(&chosen), 1.0, 1.0);
        let hull = run(select(unknown(), stress(1.0), stress(2.0)), &[x]);
        assert_bits(enclosure(&hull), 1.0, 2.0);
        let agree = run(select(unknown(), yes(), yes()), &[x]);
        assert_eq!(truth(&agree), Truth::True);
        let disagree = run(
            select(unknown(), yes(), cmp(GreaterThan, var("x"), stress(100.0))),
            &[x],
        );
        assert_eq!(truth(&disagree), Truth::Indeterminate);
        // Negative control: a select with an indeterminate condition whose
        // branches straddle the limit.
        let control = run(
            cmp(
                LessThanOrEqual,
                select(unknown(), stress(1.0), stress(20.0)),
                stress(5.0),
            ),
            &[x],
        );
        assert_eq!(truth(&control), Truth::Indeterminate);
    }

    #[test]
    fn aggregates_are_endpoint_wise_and_an_interior_extremum_is_indeterminate() {
        let x = sx(10.0, 1.0);
        let y = sy(5.0, 10.0);
        let xe = enclosure_from_bound(10.0, 1.0).unwrap();
        let ye = enclosure_from_bound(5.0, 10.0).unwrap();
        let min = run(
            Expression::Aggregate {
                function: AggregateFunction::Min,
                operands: vec![var("x"), var("y")],
            },
            &[x, y],
        );
        assert_bits(enclosure(&min), ye.lo, xe.hi.min(ye.hi));
        let max = run(
            Expression::Aggregate {
                function: AggregateFunction::Max,
                operands: vec![var("x"), var("y")],
            },
            &[x, y],
        );
        assert_bits(enclosure(&max), xe.lo, ye.hi);
        // Negative control: two interval inputs, the extremum interior.
        let control = run(
            cmp(
                ComparisonOperator::LessThanOrEqual,
                bin(BinaryOperator::Subtract, var("x"), var("y")),
                stress(5.0),
            ),
            &[x, y],
        );
        assert_eq!(truth(&control), Truth::Indeterminate);
    }

    #[test]
    fn division_by_a_range_containing_zero_is_indeterminate() {
        let result = run(
            cmp(
                ComparisonOperator::LessThanOrEqual,
                bin(BinaryOperator::Divide, stress(1.0), var("z")),
                stress(1.0e9),
            ),
            &[rz(0.0, 1.0)],
        );
        assert_eq!(truth(&result), Truth::Indeterminate);
        assert_eq!(
            note_codes(&result),
            vec![IntervalNoteCode::DivideByZeroRange]
        );
        // The block is possible somewhere in the box, so even a branch the
        // condition does not take makes the whole check indeterminate (the
        // point path evaluates eagerly and would block there).
        let unselected = run(
            cmp(
                ComparisonOperator::LessThanOrEqual,
                select(
                    cmp(ComparisonOperator::GreaterThan, stress(2.0), stress(1.0)),
                    stress(0.0),
                    bin(BinaryOperator::Divide, stress(1.0), var("z")),
                ),
                stress(1.0),
            ),
            &[rz(0.0, 1.0)],
        );
        assert_eq!(truth(&unselected), Truth::Indeterminate);
        let shadowed = run(
            logical(
                LogicalOperator::Or,
                cmp(ComparisonOperator::GreaterThan, stress(2.0), stress(1.0)),
                cmp(
                    ComparisonOperator::GreaterThan,
                    bin(BinaryOperator::Divide, stress(1.0), var("z")),
                    stress(0.0),
                ),
            ),
            &[rz(0.0, 1.0)],
        );
        assert_eq!(truth(&shadowed), Truth::Indeterminate);
    }

    #[test]
    fn an_overflowed_end_makes_the_whole_check_indeterminate() {
        let overflow = || {
            cmp(
                ComparisonOperator::LessThanOrEqual,
                bin(BinaryOperator::Multiply, var("z"), stress(f64::MAX)),
                stress(1.0),
            )
        };
        let result = run(overflow(), &[rz(1.5, 0.5)]);
        assert_eq!(truth(&result), Truth::Indeterminate);
        assert_eq!(
            note_codes(&result),
            vec![IntervalNoteCode::NonFiniteEnclosure]
        );
        // The point path computes infinities or NaN there (and can fail on
        // them), so not even Kleene logic decides around it.
        let decided = run(
            logical(
                LogicalOperator::Or,
                cmp(ComparisonOperator::GreaterThan, stress(2.0), stress(1.0)),
                overflow(),
            ),
            &[rz(1.5, 0.5)],
        );
        assert_eq!(truth(&decided), Truth::Indeterminate);
    }

    /// The ordinary point path panics on these inputs (an overflowing
    /// same-dimension quotient; a NaN interpolation or step-lookup argument),
    /// a pre-existing defect routed to T3-SI1b. Interval mode reads them
    /// indeterminate and never panics, with point or interval inputs.
    #[test]
    fn interval_mode_never_panics_where_the_point_path_can() {
        use ComparisonOperator::*;
        let quotient = || bin(BinaryOperator::Divide, var("x"), var("y"));
        for (x_bound, y_bound) in [(0.0, 0.0), (1.0e292, 0.0), (0.0, 1.0e-320)] {
            let result = run(
                cmp(LessThanOrEqual, quotient(), ratio(1.0)),
                &[sx(1.0e308, x_bound), sy(1.0e-308, y_bound)],
            );
            assert_eq!(truth(&result), Truth::Indeterminate);
            assert!(note_codes(&result).contains(&IntervalNoteCode::NonFiniteEnclosure));
        }
        // (z * 1e300) * 1e300 - (z * 1e300) * 1e300 is inf - inf = NaN in the
        // point path.
        let huge = || {
            bin(
                BinaryOperator::Multiply,
                bin(BinaryOperator::Multiply, var("z"), ratio(1.0e300)),
                ratio(1.0e300),
            )
        };
        let nan_argument = || bin(BinaryOperator::Subtract, huge(), huge());
        let rows = [(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)];
        for z_bound in [0.0, 0.5] {
            for expression in [
                interpolate(&rows, nan_argument()),
                lookup(&rows, LookupMode::Step, nan_argument()),
                lookup(&rows, LookupMode::Exact, nan_argument()),
            ] {
                let result = run(
                    cmp(LessThanOrEqual, expression, stress(10.0)),
                    &[rz(1.0, z_bound)],
                );
                assert_eq!(truth(&result), Truth::Indeterminate);
                assert!(note_codes(&result).contains(&IntervalNoteCode::NonFiniteEnclosure));
            }
        }
    }

    fn table(rows: &[(f64, f64)]) -> UserTable {
        UserTable {
            table_id: "invented_interval_table".to_string(),
            argument_dimension: Dimension::Dimensionless,
            argument_unit_ref: "ratio".to_string(),
            result_dimension: Dimension::Stress,
            result_unit_ref: UNIT.to_string(),
            rows: rows
                .iter()
                .map(|&(argument, result)| TableRow { argument, result })
                .collect(),
        }
    }

    fn interpolate(rows: &[(f64, f64)], argument: Expression) -> Expression {
        Expression::Interpolate {
            table: table(rows),
            argument: Box::new(argument),
        }
    }

    fn lookup(rows: &[(f64, f64)], mode: LookupMode, argument: Expression) -> Expression {
        Expression::Lookup {
            table: table(rows),
            mode,
            argument: Box::new(argument),
        }
    }

    #[test]
    fn interpolation_joins_outward_segment_enclosures() {
        let rows = [(0.0, 0.0), (1.0, 10.0), (2.0, 0.0)];
        // A point at a row argument is that row's exact result.
        let at_row = run(interpolate(&rows, var("z")), &[rz(1.0, 0.0)]);
        assert_bits(enclosure(&at_row), 10.0, 10.0);
        // A range spanning the interior peak contains the peak.
        let spanning = enclosure(&run(interpolate(&rows, var("z")), &[rz(1.0, 0.25)])).unwrap();
        assert!(spanning.lo < 7.5 && spanning.hi >= 10.0 && spanning.lo > 7.0);
        // Partly out of range: indeterminate (the point path would block).
        let partly = run(
            cmp(
                ComparisonOperator::LessThanOrEqual,
                interpolate(&rows, var("z")),
                stress(100.0),
            ),
            &[rz(1.9, 0.5)],
        );
        assert_eq!(truth(&partly), Truth::Indeterminate);
        assert_eq!(
            note_codes(&partly),
            vec![IntervalNoteCode::TableArgumentRange]
        );
    }

    #[test]
    fn interpolation_covers_point_path_rounding_near_a_row() {
        // The point path's formula just left of the row at 1 rounds to 0
        // here (the rise 8000 - 1e20 rounds to -1e20 and the fraction to 1),
        // far below the row value 8000. Joining point values at lo, hi and
        // the interior row would miss it; per-operation outward segments do
        // not.
        let rows = [(-1.0e10, 1.0e20), (1.0, 8000.0), (2.0, 8000.0)];
        let left_of_row = 1.0f64.next_down();
        let point = evaluate(&EvaluationInput {
            expression: interpolate(&rows, ratio(left_of_row)),
            bindings: vec![],
            required_variable_ids: vec![],
            statuses: vec![AnalysisStatus::MechanicsSolved],
            declared_grammar_version: GRAMMAR_VERSION.to_string(),
        });
        let Some(EvaluationValue::Quantity(point)) = point.value else {
            panic!("point path value");
        };
        assert_eq!(point.value, 0.0);
        let range = run(interpolate(&rows, var("z")), &[rz(1.2, 0.7)]);
        let e = enclosure(&range).unwrap();
        assert!(e.lo <= 0.0 && e.hi >= 5_000_007_999.5, "{}", e.bits_text());
        let check = run(
            cmp(
                ComparisonOperator::GreaterThanOrEqual,
                interpolate(&rows, var("z")),
                stress(4000.0),
            ),
            &[rz(1.2, 0.7)],
        );
        assert_ne!(truth(&check), Truth::True);
    }

    #[test]
    fn lookups_follow_the_point_path_for_points_and_hull_or_refuse_ranges() {
        let rows = [(1.0, 5.0), (2.0, 7.0), (3.0, 6.0)];
        let step = run(lookup(&rows, LookupMode::Step, var("z")), &[rz(2.0, 0.6)]);
        assert_bits(enclosure(&step), 5.0, 7.0);
        let step = run(lookup(&rows, LookupMode::Step, var("z")), &[rz(2.5, 0.25)]);
        assert_bits(enclosure(&step), 7.0, 7.0);
        let out = run(lookup(&rows, LookupMode::Step, var("z")), &[rz(2.9, 0.2)]);
        assert_eq!(enclosure(&out), None);
        assert_eq!(note_codes(&out), vec![IntervalNoteCode::TableArgumentRange]);

        let exact = run(lookup(&rows, LookupMode::Exact, var("z")), &[rz(2.0, 0.0)]);
        assert_bits(enclosure(&exact), 7.0, 7.0);
        let range = run(lookup(&rows, LookupMode::Exact, var("z")), &[rz(2.0, 0.5)]);
        assert_eq!(enclosure(&range), None);
        assert_eq!(note_codes(&range), vec![IntervalNoteCode::ExactLookupRange]);
        // A point miss blocks exactly as the point path does.
        let miss = run(lookup(&rows, LookupMode::Exact, var("z")), &[rz(2.5, 0.0)]);
        assert_eq!(miss.findings[0].code, FindingCode::TableKeyNotFound);
        assert_eq!(miss.value, None);
    }

    #[test]
    fn overlay_inputs_are_validated() {
        let bindings = vec![VariableBinding::new(
            "x",
            BindingSource::SolverResultField,
            Quantity::new(1.0, Dimension::Stress, UNIT).unwrap(),
        )];
        let check = |intervals: Vec<IntervalBinding>| {
            evaluate_interval(
                &EvaluationInput {
                    expression: var("x"),
                    bindings: bindings.clone(),
                    required_variable_ids: vec![],
                    statuses: vec![AnalysisStatus::MechanicsSolved],
                    declared_grammar_version: GRAMMAR_VERSION.to_string(),
                },
                &intervals,
            )
        };
        let unbound = check(vec![IntervalBinding {
            variable_id: "w".to_string(),
            enclosure: Some(enc(0.0, 1.0)),
        }]);
        assert_eq!(unbound.findings[0].code, FindingCode::InvalidReference);
        let reversed = check(vec![IntervalBinding {
            variable_id: "x".to_string(),
            enclosure: Some(enc(2.0, 1.0)),
        }]);
        assert_eq!(reversed.findings[0].code, FindingCode::InvalidReference);
        let infinite = check(vec![IntervalBinding {
            variable_id: "x".to_string(),
            enclosure: Some(enc(0.0, f64::INFINITY)),
        }]);
        assert_eq!(infinite.findings[0].code, FindingCode::NonFiniteInput);
        let duplicate = check(vec![
            IntervalBinding {
                variable_id: "x".to_string(),
                enclosure: Some(enc(0.0, 1.0)),
            },
            IntervalBinding {
                variable_id: "x".to_string(),
                enclosure: Some(enc(0.0, 1.0)),
            },
        ]);
        assert_eq!(duplicate.findings[0].code, FindingCode::DuplicateBinding);
        // An input with no finite enclosure is indeterminate, not blocked.
        let unknown = check(vec![IntervalBinding {
            variable_id: "x".to_string(),
            enclosure: None,
        }]);
        assert!(unknown.findings.is_empty());
        assert_eq!(enclosure(&unknown), None);
        assert_eq!(
            note_codes(&unknown),
            vec![IntervalNoteCode::NonFiniteEnclosure]
        );
    }

    /// Structural findings come from the point path's own checks, so they are
    /// identical, in order and wording, to `evaluate`'s.
    #[test]
    fn structural_findings_match_the_point_path() {
        let temperature = |v| {
            Expression::Literal(Quantity::new(v, Dimension::Temperature, "invented_t").unwrap())
        };
        let boolean = || cmp(ComparisonOperator::LessThan, stress(1.0), stress(2.0));
        let expressions = vec![
            bin(BinaryOperator::Add, stress(1.0), temperature(1.0)),
            bin(
                BinaryOperator::Add,
                stress(1.0),
                Expression::Literal(Quantity::new(1.0, Dimension::Stress, "other_unit").unwrap()),
            ),
            bin(BinaryOperator::Multiply, stress(1.0), temperature(2.0)),
            bin(BinaryOperator::Divide, stress(1.0), temperature(2.0)),
            bin(BinaryOperator::Add, boolean(), stress(1.0)),
            cmp(ComparisonOperator::Equal, stress(1.0), temperature(1.0)),
            cmp(ComparisonOperator::Equal, boolean(), stress(1.0)),
            logical(LogicalOperator::And, boolean(), stress(1.0)),
            unary(UnaryOperator::Not, stress(1.0)),
            unary(UnaryOperator::Negate, boolean()),
            unary(UnaryOperator::Abs, boolean()),
            select(stress(1.0), stress(1.0), stress(2.0)),
            select(boolean(), stress(1.0), boolean()),
            select(boolean(), stress(1.0), temperature(2.0)),
            Expression::Aggregate {
                function: AggregateFunction::Max,
                operands: vec![],
            },
            Expression::Aggregate {
                function: AggregateFunction::Min,
                operands: vec![stress(1.0), boolean()],
            },
            Expression::Aggregate {
                function: AggregateFunction::Min,
                operands: vec![stress(1.0), temperature(1.0)],
            },
            interpolate(&[(0.0, 1.0)], ratio(0.5)),
            interpolate(&[(0.0, 1.0), (0.0, 2.0)], ratio(0.5)),
            interpolate(&[(0.0, 1.0), (1.0, 2.0)], stress(0.5)),
            interpolate(&[(0.0, 1.0), (1.0, 2.0)], boolean()),
            lookup(&[(0.0, 1.0), (1.0, 2.0)], LookupMode::Exact, ratio(0.5)),
            lookup(&[(0.0, 1.0), (1.0, 2.0)], LookupMode::Exact, ratio(5.0)),
            var("missing"),
            var(""),
            Expression::UnsupportedForm {
                form_id: "power".to_string(),
            },
            Expression::UnsafeHostAccess {
                request: "filesystem".to_string(),
            },
        ];
        for expression in expressions {
            let input = EvaluationInput {
                expression: expression.clone(),
                bindings: vec![],
                required_variable_ids: vec!["r".to_string()],
                statuses: vec![AnalysisStatus::MechanicsSolved],
                declared_grammar_version: GRAMMAR_VERSION.to_string(),
            };
            let point = evaluate(&input);
            let interval = evaluate_interval(&input, &[]);
            assert!(!point.findings.is_empty(), "{expression:?}");
            assert_eq!(interval.findings, point.findings, "{expression:?}");
            assert_eq!(interval.value, None);
            assert_eq!(interval.statuses, point.statuses);
            assert_eq!(interval.source_variable_ids, point.source_variable_ids);
        }
        // Grammar version and status boundary findings are shared too.
        let input = EvaluationInput {
            expression: stress(1.0),
            bindings: vec![],
            required_variable_ids: vec![],
            statuses: vec![AnalysisStatus::HumanApprovedForProject],
            declared_grammar_version: "2.0.0".to_string(),
        };
        assert_eq!(
            evaluate_interval(&input, &[]).findings,
            evaluate(&input).findings
        );
    }

    // -- Seeded soundness property against the unchanged point path ---------

    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            // xorshift64*
            self.0 ^= self.0 >> 12;
            self.0 ^= self.0 << 25;
            self.0 ^= self.0 >> 27;
            self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
        }

        fn below(&mut self, n: u64) -> u64 {
            self.next() % n
        }

        fn unit(&mut self) -> f64 {
            (self.next() >> 11) as f64 / (1u64 << 53) as f64
        }

        fn pick(&mut self, values: &[f64]) -> f64 {
            values[self.below(values.len() as u64) as usize]
        }
    }

    const LITERALS: &[f64] = &[0.0, 1.0, -1.0, 0.1, 3.0, -2.5, 7.0, 1.0e-300, 1.0e300, 0.5];

    #[derive(Clone, Copy, PartialEq)]
    enum Kind {
        Stress,
        Ratio,
        Boolean,
    }

    fn gen(rng: &mut Rng, kind: Kind, depth: u32) -> Expression {
        // Occasionally build a structurally wrong subtree.
        let kind = if rng.below(40) == 0 {
            [Kind::Stress, Kind::Ratio, Kind::Boolean][rng.below(3) as usize]
        } else {
            kind
        };
        let leaf = depth == 0 || rng.below(4) == 0;
        match kind {
            Kind::Stress => {
                if leaf {
                    return match rng.below(3) {
                        0 => var("x"),
                        1 => var("y"),
                        _ => stress(rng.pick(LITERALS)),
                    };
                }
                match rng.below(11) {
                    0 => bin(
                        BinaryOperator::Add,
                        gen(rng, Kind::Stress, depth - 1),
                        gen(rng, Kind::Stress, depth - 1),
                    ),
                    1 => bin(
                        BinaryOperator::Subtract,
                        gen(rng, Kind::Stress, depth - 1),
                        gen(rng, Kind::Stress, depth - 1),
                    ),
                    2 => bin(
                        BinaryOperator::Multiply,
                        gen(rng, Kind::Ratio, depth - 1),
                        gen(rng, Kind::Stress, depth - 1),
                    ),
                    3 => bin(
                        BinaryOperator::Divide,
                        gen(rng, Kind::Stress, depth - 1),
                        gen(rng, Kind::Ratio, depth - 1),
                    ),
                    4 => unary(UnaryOperator::Negate, gen(rng, Kind::Stress, depth - 1)),
                    5 => unary(UnaryOperator::Abs, gen(rng, Kind::Stress, depth - 1)),
                    6 => Expression::Aggregate {
                        function: if rng.below(2) == 0 {
                            AggregateFunction::Min
                        } else {
                            AggregateFunction::Max
                        },
                        operands: (0..1 + rng.below(3))
                            .map(|_| gen(rng, Kind::Stress, depth - 1))
                            .collect(),
                    },
                    7 => select(
                        gen(rng, Kind::Boolean, depth - 1),
                        gen(rng, Kind::Stress, depth - 1),
                        gen(rng, Kind::Stress, depth - 1),
                    ),
                    8 => {
                        let rows: &[(f64, f64)] = if rng.below(3) == 0 {
                            &[(-1.0e10, 1.0e20), (1.0, 8000.0), (2.0, 8000.0)]
                        } else {
                            &[(-2.0, 1.0), (0.0, -3.0), (0.5, 4.0), (3.0, 4.5)]
                        };
                        interpolate(rows, gen(rng, Kind::Ratio, depth - 1))
                    }
                    9 => lookup(
                        &[(-2.0, 1.0), (0.0, -3.0), (0.5, 4.0), (3.0, 4.5)],
                        LookupMode::Step,
                        gen(rng, Kind::Ratio, depth - 1),
                    ),
                    _ => lookup(
                        &[(-1.0, 1.0), (0.0, -3.0), (0.5, 4.0), (1.0, 4.5)],
                        LookupMode::Exact,
                        if rng.below(2) == 0 {
                            ratio(rng.pick(&[-1.0, 0.0, 0.5, 1.0, 0.25]))
                        } else {
                            gen(rng, Kind::Ratio, depth - 1)
                        },
                    ),
                }
            }
            Kind::Ratio => {
                if leaf {
                    return if rng.below(2) == 0 {
                        var("z")
                    } else {
                        ratio(rng.pick(LITERALS))
                    };
                }
                match rng.below(5) {
                    0 => bin(
                        BinaryOperator::Divide,
                        gen(rng, Kind::Stress, depth - 1),
                        gen(rng, Kind::Stress, depth - 1),
                    ),
                    1 => bin(
                        BinaryOperator::Add,
                        gen(rng, Kind::Ratio, depth - 1),
                        gen(rng, Kind::Ratio, depth - 1),
                    ),
                    2 => bin(
                        BinaryOperator::Multiply,
                        gen(rng, Kind::Ratio, depth - 1),
                        gen(rng, Kind::Ratio, depth - 1),
                    ),
                    3 => unary(UnaryOperator::Abs, gen(rng, Kind::Ratio, depth - 1)),
                    _ => bin(
                        BinaryOperator::Subtract,
                        gen(rng, Kind::Ratio, depth - 1),
                        gen(rng, Kind::Ratio, depth - 1),
                    ),
                }
            }
            Kind::Boolean => {
                let operators = [
                    ComparisonOperator::LessThan,
                    ComparisonOperator::LessThanOrEqual,
                    ComparisonOperator::GreaterThan,
                    ComparisonOperator::GreaterThanOrEqual,
                    ComparisonOperator::Equal,
                    ComparisonOperator::NotEqual,
                ];
                let operator = operators[rng.below(6) as usize];
                if leaf {
                    return cmp(
                        operator,
                        gen(rng, Kind::Stress, 0),
                        stress(rng.pick(LITERALS)),
                    );
                }
                match rng.below(6) {
                    0 | 1 => cmp(
                        operator,
                        gen(rng, Kind::Stress, depth - 1),
                        gen(rng, Kind::Stress, depth - 1),
                    ),
                    2 => cmp(
                        operator,
                        gen(rng, Kind::Ratio, depth - 1),
                        gen(rng, Kind::Ratio, depth - 1),
                    ),
                    3 => logical(
                        if rng.below(2) == 0 {
                            LogicalOperator::And
                        } else {
                            LogicalOperator::Or
                        },
                        gen(rng, Kind::Boolean, depth - 1),
                        gen(rng, Kind::Boolean, depth - 1),
                    ),
                    4 => unary(UnaryOperator::Not, gen(rng, Kind::Boolean, depth - 1)),
                    _ => select(
                        gen(rng, Kind::Boolean, depth - 1),
                        gen(rng, Kind::Boolean, depth - 1),
                        gen(rng, Kind::Boolean, depth - 1),
                    ),
                }
            }
        }
    }

    fn samples(rng: &mut Rng, enclosure: Enclosure) -> Vec<f64> {
        let mut values = vec![
            enclosure.lo,
            enclosure.hi,
            nu(enclosure.lo).min(enclosure.hi),
            nd(enclosure.hi).max(enclosure.lo),
        ];
        for _ in 0..4 {
            let t = rng.unit();
            let v = enclosure.lo + (enclosure.hi - enclosure.lo) * t;
            values.push(v.max(enclosure.lo).min(enclosure.hi));
        }
        // Row arguments and zero, when inside, are where blocks and kinks sit.
        for special in [0.0, -1.0, 0.5, 1.0, 1.0f64.next_down(), 2.0, 3.0] {
            if enclosure.lo <= special && special <= enclosure.hi {
                values.push(special);
            }
        }
        values
    }

    #[test]
    fn interval_outcomes_are_sound_against_the_point_path() {
        let mut rng = Rng(0x5EED_1A73_0000_0001);
        let mut tally = [0usize; 5]; // T, F, U, quantity, blocked
        let mut panics = 0usize;
        for _ in 0..4000 {
            let kind = [Kind::Boolean, Kind::Boolean, Kind::Stress][rng.below(3) as usize];
            let expression = gen(&mut rng, kind, 3);
            let mut inputs: Vec<Input> = Vec::new();
            let spreads = [0.0, 1.0e-9, 0.25, 1.0, 4.0];
            inputs.push(sx(
                rng.pick(&[-3.0, 0.0, 1.0, 7.0, 0.1]),
                rng.pick(&spreads),
            ));
            inputs.push(sy(rng.pick(&[-2.5, 0.0, 3.0, 1.0e300]), rng.pick(&spreads)));
            inputs.push(rz(
                rng.pick(&[-1.0, 0.0, 0.5, 1.0, 1.5, 2.0]),
                rng.pick(&spreads),
            ));
            let result = run(expression.clone(), &inputs);

            let boxes: Vec<Enclosure> = inputs
                .iter()
                .map(|&(_, _, _, q, b)| enclosure_from_bound(q, b).unwrap())
                .collect();
            let per_input: Vec<Vec<f64>> = boxes.iter().map(|b| samples(&mut rng, *b)).collect();
            let mut points = Vec::new();
            for i in 0..12 {
                points.push([
                    per_input[0][i % per_input[0].len()],
                    per_input[1][(i * 7 + 3) % per_input[1].len()],
                    per_input[2][(i * 5 + 1) % per_input[2].len()],
                ]);
            }
            for &(a, b, c) in &[(0usize, 0usize, 0usize), (1, 1, 1), (0, 1, 2), (1, 0, 3)] {
                points.push([
                    per_input[0][a],
                    per_input[1][b],
                    per_input[2][c % per_input[2].len()],
                ]);
            }

            for point in &points {
                let bindings = inputs
                    .iter()
                    .zip(point.iter())
                    .map(|(&(id, dimension, unit, _, _), &value)| {
                        VariableBinding::new(
                            id,
                            BindingSource::SolverResultField,
                            Quantity::new(value, dimension, unit).unwrap(),
                        )
                    })
                    .collect();
                let point_input = EvaluationInput {
                    expression: expression.clone(),
                    bindings,
                    required_variable_ids: vec![],
                    statuses: vec![AnalysisStatus::MechanicsSolved],
                    declared_grammar_version: GRAMMAR_VERSION.to_string(),
                };
                // The point path can panic on a non-finite intermediate (a
                // same-dimension quotient that overflows, or a NaN table
                // argument); a panic is neither a pass nor a fail, so it
                // counts as blocked here.
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    evaluate(&point_input)
                }))
                .unwrap_or_else(|_| {
                    panics += 1;
                    EvaluationResult {
                        value: None,
                        statuses: vec![],
                        source_variable_ids: vec![],
                        findings: vec![EvaluationFinding::new(
                            FindingCode::NonFiniteInput,
                            "point_path_panic",
                            "the point path panicked",
                        )],
                    }
                });
                if result.is_blocked() {
                    assert!(outcome.is_blocked(), "interval blocked, point did not: {expression:?} at {point:?} -> {result:?}");
                    continue;
                }
                match (&result.value, &outcome.value) {
                    (Some(IntervalValue::Boolean(Truth::True)), value) => {
                        assert_eq!(
                            value,
                            &Some(EvaluationValue::Boolean(true)),
                            "{expression:?} at {point:?} -> {result:?}"
                        )
                    }
                    (Some(IntervalValue::Boolean(Truth::False)), value) => {
                        assert_eq!(
                            value,
                            &Some(EvaluationValue::Boolean(false)),
                            "{expression:?} at {point:?} -> {result:?}"
                        )
                    }
                    (Some(IntervalValue::Boolean(Truth::Indeterminate)), _) => {}
                    (Some(IntervalValue::Quantity(quantity)), value) => {
                        if let Some(e) = quantity.enclosure {
                            let Some(EvaluationValue::Quantity(q)) = value else {
                                panic!("point blocked inside a finite enclosure: {expression:?} at {point:?} -> {result:?}");
                            };
                            assert!(
                                e.lo <= q.value && q.value <= e.hi,
                                "{expression:?} at {point:?} -> {result:?}"
                            );
                        }
                    }
                    (None, _) => {
                        panic!("unblocked result without a value: {expression:?} -> {result:?}")
                    }
                }
            }
            match &result.value {
                _ if result.is_blocked() => tally[4] += 1,
                Some(IntervalValue::Boolean(Truth::True)) => tally[0] += 1,
                Some(IntervalValue::Boolean(Truth::False)) => tally[1] += 1,
                Some(IntervalValue::Boolean(Truth::Indeterminate)) => tally[2] += 1,
                _ => tally[3] += 1,
            }
        }
        // The generator must exercise every outcome.
        assert!(tally.iter().all(|&count| count >= 100), "{tally:?}");
        eprintln!("interval soundness property: tally {tally:?}, point-path panics {panics}");
    }
}
