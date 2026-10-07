//! I88 T3-SI1c scratch harness (not repository content): the SI1c family.
//!
//! Every D producer site (add, subtract; multiply in its three arms; divide in
//! its dimensionless-divisor and derived arms; interpolation's rise, offset
//! and run), as +inf, -inf and NaN (inf - inf, 0 * inf, inf / inf), wrapped
//! at depths 1-3, against every consumer (the six comparisons both ways,
//! `not`, `and`, `or`, boolean and quantity `select` taken and untaken, `min`,
//! `max`, divide-by in its three arms, a table argument, the final value),
//! with an overflowing and a finite (control) binding set, plus the finite
//! boundary controls. The runner part runs boolean, quantity and table checks
//! over the same forms, with N-4 values (NaN, +-inf, an overflow at unit
//! normalization, a raw non-finite value in another unit; inputs, limits and
//! an unreferenced input), in seven modes, as full JSON.
//!
//! One dump line per `evaluate`, `evaluate_interval` or run call, in call
//! order. All values are invented.

use std::fmt::Write as _;
use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};

use open_pipe_stress_expression_evaluator::{
    enclosure_from_bound, evaluate, evaluate_interval, AggregateFunction, AnalysisStatus,
    BinaryOperator, BindingSource, ComparisonOperator, Dimension, EvaluationInput,
    EvaluationValue, Expression, IntervalBinding, IntervalValue, LogicalOperator, LookupMode,
    Quantity, TableRow, UnaryOperator, UserTable, VariableBinding, GRAMMAR_VERSION,
};
use open_pipe_stress_rule_check_runner::{
    run_rule_checks, run_rule_checks_with_bounds, RuleCheckRunInput, SolverResultBinding,
    SolverResultBound, SuppliedValueBinding,
};
use open_pipe_stress_rule_pack_document::{encode_dimension, encode_expression};
use serde_json::{json, Value};

const MAX: f64 = f64::MAX;

fn q(value: f64, dimension: Dimension, unit: &str) -> Quantity {
    Quantity {
        value,
        dimension,
        unit_ref: unit.to_string(),
        unit_required: true,
        dimension_check_required: true,
    }
}
fn lit(value: f64, dimension: Dimension, unit: &str) -> Expression {
    Expression::Literal(q(value, dimension, unit))
}
fn ratio(value: f64) -> Expression {
    lit(value, Dimension::Dimensionless, "ratio")
}
fn var(id: &str) -> Expression {
    Expression::VariableRef(id.to_string())
}
fn bin(op: BinaryOperator, a: Expression, b: Expression) -> Expression {
    Expression::Binary { operator: op, left: Box::new(a), right: Box::new(b) }
}
fn cmp(op: ComparisonOperator, a: Expression, b: Expression) -> Expression {
    Expression::Compare { operator: op, left: Box::new(a), right: Box::new(b) }
}
fn unary(op: UnaryOperator, a: Expression) -> Expression {
    Expression::Unary { operator: op, operand: Box::new(a) }
}
fn select(c: Expression, t: Expression, e: Expression) -> Expression {
    Expression::Select { condition: Box::new(c), then_branch: Box::new(t), else_branch: Box::new(e) }
}
fn logical(op: LogicalOperator, a: Expression, b: Expression) -> Expression {
    Expression::Logical { operator: op, left: Box::new(a), right: Box::new(b) }
}
fn aggregate(f: AggregateFunction, operands: Vec<Expression>) -> Expression {
    Expression::Aggregate { function: f, operands }
}
fn table(id: &str, rows: &[(f64, f64)], result_dimension: Dimension, result_unit: &str) -> UserTable {
    UserTable {
        table_id: id.to_string(),
        argument_dimension: Dimension::Dimensionless,
        argument_unit_ref: "ratio".to_string(),
        result_dimension,
        result_unit_ref: result_unit.to_string(),
        rows: rows.iter().map(|&(argument, result)| TableRow { argument, result }).collect(),
    }
}

/// A true and a false finite comparison (for logical and select forms).
fn yes() -> Expression {
    cmp(ComparisonOperator::LessThan, ratio(1.0), ratio(2.0))
}
fn no() -> Expression {
    cmp(ComparisonOperator::GreaterThan, ratio(1.0), ratio(2.0))
}

/// A producer form: its expression, its result dimension and unit, and the
/// site label (the D site it exercises).
#[derive(Clone)]
struct Producer {
    label: String,
    expression: Expression,
    dimension: Dimension,
    unit: &'static str,
}

const SU: &str = "su";
const MU: &str = "mu";
const LU: &str = "lu";
const FU: &str = "fu";

/// Evaluator bindings: overflow set (`o`) and control set (`c`).
fn ee_bindings(set: &str) -> Vec<VariableBinding> {
    let (x, m, l, f, z) = match set {
        "o" => (1.0e308, 1.0e308, 1.0e-10, 1.0e300, 1.0e300),
        "n" => (-1.0e308, -1.0e308, 1.0e-10, -1.0e300, -1.0e300),
        _ => (1.0, 2.0, 0.5, 3.0, 0.25),
    };
    vec![
        VariableBinding::new("x", BindingSource::SolverResultField, q(x, Dimension::Stress, SU)),
        VariableBinding::new("m", BindingSource::SolverResultField, q(m, Dimension::Moment, MU)),
        VariableBinding::new("l", BindingSource::UserSuppliedValue, q(l, Dimension::Length, LU)),
        VariableBinding::new("f", BindingSource::UserSuppliedValue, q(f, Dimension::Force, FU)),
        VariableBinding::new("z", BindingSource::RulePackRequiredInput, q(z, Dimension::Dimensionless, "ratio")),
    ]
}

/// The base producers (each overflows under binding set `o` and `n`, and is
/// finite under `c`, except the interpolation steps, which use literal rows).
fn base_producers() -> Vec<Producer> {
    use BinaryOperator::*;
    let p = |label: &str, expression: Expression, dimension: Dimension, unit: &'static str| Producer {
        label: label.to_string(),
        expression,
        dimension,
        unit,
    };
    vec![
        p("add", bin(Add, var("x"), var("x")), Dimension::Stress, SU),
        p("sub", bin(Subtract, var("x"), unary(UnaryOperator::Negate, var("x"))), Dimension::Stress, SU),
        p("mul_l", bin(Multiply, ratio(1.0e10), var("x")), Dimension::Stress, SU),
        p("mul_r", bin(Multiply, var("x"), ratio(1.0e10)), Dimension::Stress, SU),
        p("mul_d", bin(Multiply, var("f"), bin(Multiply, ratio(1.0e300), var("l"))), Dimension::Moment, "fu*lu"),
        p("mul_rr", bin(Multiply, var("z"), var("z")), Dimension::Dimensionless, "ratio"),
        p("div_d", bin(Divide, var("x"), ratio(1.0e-10)), Dimension::Stress, SU),
        p("div_rr", bin(Divide, var("z"), ratio(1.0e-10)), Dimension::Dimensionless, "ratio"),
        p("div_derived", bin(Divide, var("m"), var("l")), Dimension::Force, "mu/lu"),
        p(
            "interp_rise",
            Expression::Interpolate {
                table: table("t_rise", &[(0.0, -1.0e308), (1.0, 1.0e308)], Dimension::Stress, SU),
                argument: Box::new(bin(Multiply, ratio(0.5), bin(Divide, var("z"), var("z")))),
            },
            Dimension::Stress,
            SU,
        ),
        p(
            "interp_run",
            Expression::Interpolate {
                table: table("t_run", &[(-MAX, 0.0), (MAX, 10.0)], Dimension::Stress, SU),
                argument: Box::new(bin(Multiply, ratio(0.5), var("z"))),
            },
            Dimension::Stress,
            SU,
        ),
        p(
            "interp_sum",
            Expression::Interpolate {
                table: table("t_sum", &[(-1.0e20, 3.0 * 2f64.powi(970)), (1.0, MAX)], Dimension::Stress, SU),
                argument: Box::new(bin(Multiply, ratio(0.5), bin(Divide, var("z"), var("z")))),
            },
            Dimension::Stress,
            SU,
        ),
        p(
            "interp_offset",
            Expression::Interpolate {
                table: table("t_off", &[(-MAX, 0.0), (MAX, 10.0)], Dimension::Stress, SU),
                argument: Box::new(bin(Multiply, ratio(1.0e8), var("z"))),
            },
            Dimension::Stress,
            SU,
        ),
    ]
}

/// +inf (as is), -inf (negated), NaN (inf - inf, 0 * inf, inf / inf).
fn kinds(p: &Producer) -> Vec<Producer> {
    use BinaryOperator::*;
    let with = |suffix: &str, expression: Expression| Producer {
        label: format!("{}_{suffix}", p.label),
        expression,
        dimension: p.dimension,
        unit: p.unit,
    };
    let e = || p.expression.clone();
    let mut out = vec![
        with("pinf", e()),
        with("ninf", unary(UnaryOperator::Negate, e())),
        with("nan_sub", bin(Subtract, e(), e())),
        with("nan_zero", bin(Multiply, ratio(0.0), e())),
    ];
    if p.dimension == Dimension::Dimensionless {
        // inf / inf over a dimensionless divisor (carried NaN on main).
        out.push(with("nan_div", bin(Divide, e(), e())));
    }
    out
}

/// Pass-through layers for depth 2 and 3.
fn wrap(p: &Producer, depth: u32) -> Producer {
    let mut expression = p.expression.clone();
    for level in 1..depth {
        expression = match level % 3 {
            1 => unary(UnaryOperator::Abs, expression),
            2 => select(yes(), expression, lit(1.0, p.dimension, p.unit)),
            _ => unary(UnaryOperator::Negate, expression),
        };
    }
    Producer {
        label: format!("{}_d{depth}", p.label),
        expression,
        dimension: p.dimension,
        unit: p.unit,
    }
}

/// Every consumer of a producer `p`, labelled by consumer row.
fn consumers(p: &Producer) -> Vec<(String, Expression)> {
    use ComparisonOperator::*;
    let e = || p.expression.clone();
    let partner = || lit(2.0, p.dimension, p.unit);
    let mut out: Vec<(String, Expression)> = Vec::new();
    let ops = [
        ("lt", LessThan),
        ("le", LessThanOrEqual),
        ("gt", GreaterThan),
        ("ge", GreaterThanOrEqual),
        ("eq", Equal),
        ("ne", NotEqual),
    ];
    for (name, op) in ops {
        out.push((format!("c1_cmp_{name}_left"), cmp(op, e(), partner())));
        out.push((format!("c1_cmp_{name}_right"), cmp(op, partner(), e())));
    }
    out.push(("c2_not".into(), unary(UnaryOperator::Not, cmp(GreaterThan, e(), partner()))));
    out.push(("c3_and_left".into(), logical(LogicalOperator::And, cmp(GreaterThan, e(), partner()), yes())));
    out.push(("c3_and_right".into(), logical(LogicalOperator::And, yes(), cmp(NotEqual, e(), partner()))));
    out.push(("c3_or_left".into(), logical(LogicalOperator::Or, cmp(LessThan, e(), partner()), no())));
    out.push(("c3_or_right".into(), logical(LogicalOperator::Or, no(), cmp(GreaterThanOrEqual, e(), partner()))));
    out.push(("c4_bselect_cond".into(), select(cmp(GreaterThan, e(), partner()), yes(), no())));
    out.push(("c4_bselect_taken".into(), select(yes(), cmp(LessThanOrEqual, e(), partner()), no())));
    out.push(("c4_bselect_untaken".into(), select(no(), cmp(LessThanOrEqual, e(), partner()), yes())));
    out.push(("c5_qselect_taken_then".into(), select(yes(), e(), partner())));
    out.push(("c5_qselect_untaken_then".into(), select(no(), e(), partner())));
    out.push(("c5_qselect_taken_else".into(), select(no(), partner(), e())));
    out.push(("c5_qselect_untaken_else".into(), select(yes(), partner(), e())));
    out.push(("c6_min_first".into(), aggregate(AggregateFunction::Min, vec![e(), partner()])));
    out.push(("c6_max_first".into(), aggregate(AggregateFunction::Max, vec![e(), partner()])));
    out.push(("c6_min_last".into(), aggregate(AggregateFunction::Min, vec![partner(), e()])));
    out.push(("c6_max_last".into(), aggregate(AggregateFunction::Max, vec![partner(), e()])));
    out.push(("c6_max_cmp".into(), cmp(LessThanOrEqual, aggregate(AggregateFunction::Max, vec![e(), partner()]), lit(3.0, p.dimension, p.unit))));
    // Divide-by: the same dimension (ratio arm, or the dimensionless-divisor
    // arm for a ratio), a stress over a ratio (dimensionless-divisor arm), a
    // force over a length (derived arm) is covered by the length producer.
    out.push(("c7_divby_same".into(), bin(BinaryOperator::Divide, partner(), e())));
    if p.dimension == Dimension::Dimensionless {
        out.push(("c7_divby_dimless".into(), bin(BinaryOperator::Divide, lit(2.0, Dimension::Stress, SU), e())));
        let t = table("t_arg", &[(-2.0, 1.0), (0.0, -3.0), (0.5, 4.0), (3.0, 4.5)], Dimension::Stress, SU);
        out.push(("c10_table_interpolate".into(), Expression::Interpolate { table: t.clone(), argument: Box::new(e()) }));
        out.push(("c10_table_step".into(), Expression::Lookup { table: t.clone(), mode: LookupMode::Step, argument: Box::new(e()) }));
        out.push(("c10_table_exact".into(), Expression::Lookup { table: t, mode: LookupMode::Exact, argument: Box::new(e()) }));
    }
    if p.dimension == Dimension::Length {
        out.push(("c7_divby_derived".into(), bin(BinaryOperator::Divide, lit(2.0, Dimension::Force, FU), e())));
    }
    out.push(("c9_final".into(), e()));
    out.push(("c9_final_cmp".into(), cmp(LessThanOrEqual, e(), lit(3.0, p.dimension, p.unit))));
    out
}

/// A length producer (for the derived divide-by arm).
fn length_producers() -> Vec<Producer> {
    let p = Producer {
        label: "mul_len".into(),
        expression: bin(BinaryOperator::Multiply, var("z"), bin(BinaryOperator::Multiply, ratio(1.0e300), var("l"))),
        dimension: Dimension::Length,
        unit: LU,
    };
    vec![p]
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

fn panic_msg(p: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = p.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = p.downcast_ref::<String>() {
        s.clone()
    } else {
        "<non-string panic>".to_string()
    }
}

struct Out(String);
impl Out {
    fn point(&mut self, label: &str, input: &EvaluationInput) {
        let text = match catch_unwind(AssertUnwindSafe(|| evaluate(input))) {
            Ok(r) => {
                let bits = match &r.value {
                    Some(EvaluationValue::Quantity(q)) => format!("0x{:016x}", q.value.to_bits()),
                    Some(EvaluationValue::Boolean(b)) => b.to_string(),
                    None => "none".to_string(),
                };
                format!("{bits}|{r:?}")
            }
            Err(p) => format!("PANIC({})", panic_msg(&p).replace('\n', " ")),
        };
        writeln!(self.0, "{label}\tP\t{text}").unwrap();
    }
    fn interval(&mut self, label: &str, k: usize, input: &EvaluationInput, iv: &[IntervalBinding]) {
        let text = match catch_unwind(AssertUnwindSafe(|| evaluate_interval(input, iv))) {
            Ok(r) => {
                let bits = match &r.value {
                    Some(IntervalValue::Quantity(q)) => match q.enclosure {
                        Some(e) => e.bits_text(),
                        None => "noenc".to_string(),
                    },
                    Some(IntervalValue::Boolean(t)) => format!("{t:?}"),
                    None => "none".to_string(),
                };
                format!("{bits}|{r:?}")
            }
            Err(p) => format!("PANIC({})", panic_msg(&p).replace('\n', " ")),
        };
        writeln!(self.0, "{label}#i{k}\tI\t{text}").unwrap();
    }
    fn both(&mut self, label: &str, input: &EvaluationInput) {
        self.point(label, input);
        let ids: Vec<(String, f64)> = input
            .bindings
            .iter()
            .filter_map(|b| b.quantity.as_ref().map(|q| (b.variable_id.clone(), q.value)))
            .collect();
        let variants: [&dyn Fn(f64) -> Option<Option<open_pipe_stress_expression_evaluator::Enclosure>>; 4] = [
            &|_| None,
            &|q| Some(enclosure_from_bound(q, 0.0)),
            &|q| Some(enclosure_from_bound(q, 1.0)),
            &|q| Some(enclosure_from_bound(q, q.abs() * 1.0e-6 + 1.0e-300)),
        ];
        for (k, v) in variants.iter().enumerate() {
            let iv: Vec<IntervalBinding> = ids
                .iter()
                .filter_map(|(id, q)| v(*q).map(|enclosure| IntervalBinding { variable_id: id.clone(), enclosure }))
                .collect();
            self.interval(label, k, input, &iv);
        }
    }
}

/// The finite boundary controls (each must evaluate, identically, on both sides).
fn boundary_controls() -> Vec<(String, Expression)> {
    use BinaryOperator::*;
    let s = |v: f64| lit(v, Dimension::Stress, SU);
    let mut out: Vec<(String, Expression)> = vec![
        ("max_plus_zero".into(), bin(Add, s(MAX), s(0.0))),
        ("max_minus_max".into(), bin(Subtract, s(MAX), s(MAX))),
        ("max_times_one".into(), bin(Multiply, ratio(1.0), s(MAX))),
        ("half_max_twice".into(), bin(Add, s(MAX / 2.0), s(MAX / 2.0))),
        ("max_plus_under_half_ulp".into(), bin(Add, s(MAX), s(MAX / 2.0f64.powi(54)))),
        ("max_plus_half_ulp_overflows".into(), bin(Add, s(MAX), s(2.0f64.powi(970)))),
        ("underflow_subnormal".into(), bin(Multiply, s(1.0e-308), ratio(1.0e-10))),
        ("underflow_zero".into(), bin(Multiply, s(5.0e-324), ratio(0.5))),
        ("negative_zero".into(), bin(Add, s(-0.0), s(-0.0))),
        ("largest_ratio".into(), bin(Divide, s(MAX), s(1.0))),
        ("smallest_over_max".into(), bin(Divide, s(5.0e-324), s(MAX))),
        ("max_over_one_dimless".into(), bin(Divide, s(MAX), ratio(1.0))),
        ("moment_over_length_max".into(), bin(Divide, lit(MAX, Dimension::Moment, MU), lit(1.0, Dimension::Length, LU))),
        ("force_times_length_max".into(), bin(Multiply, lit(MAX, Dimension::Force, FU), lit(1.0, Dimension::Length, LU))),
        (
            "wide_interpolation".into(),
            Expression::Interpolate {
                table: table("t_wide", &[(-1.0e307, 0.0), (1.0e307, 10.0)], Dimension::Stress, SU),
                argument: Box::new(ratio(0.0)),
            },
        ),
        (
            "max_rows_at_row".into(),
            Expression::Interpolate {
                table: table("t_run", &[(-MAX, 0.0), (MAX, 10.0)], Dimension::Stress, SU),
                argument: Box::new(ratio(MAX)),
            },
        ),
        (
            "max_rows_step".into(),
            Expression::Lookup {
                table: table("t_run", &[(-MAX, 0.0), (MAX, 10.0)], Dimension::Stress, SU),
                mode: LookupMode::Step,
                argument: Box::new(ratio(0.0)),
            },
        ),
    ];
    let compared: Vec<(String, Expression)> = out
        .iter()
        .map(|(label, e)| (format!("{label}_cmp"), cmp(ComparisonOperator::GreaterThanOrEqual, e.clone(), e.clone())))
        .collect();
    out.extend(compared);
    out
}

#[test]
fn si1c_evaluator_family() {
    std::panic::set_hook(Box::new(|_| {}));
    let mut out = Out(String::new());
    let mut producers: Vec<Producer> = Vec::new();
    for base in base_producers().iter().chain(length_producers().iter()) {
        for kind in kinds(base) {
            for depth in 1..=3 {
                producers.push(wrap(&kind, depth));
            }
        }
    }
    for p in &producers {
        for (consumer, expression) in consumers(p) {
            for set in ["o", "n", "c"] {
                let label = format!("f_{}_{}_{set}", p.label, consumer);
                out.both(&label, &input(expression.clone(), ee_bindings(set)));
            }
        }
    }
    for (label, expression) in boundary_controls() {
        out.both(&format!("b_{label}"), &input(expression, ee_bindings("c")));
    }
    let _ = std::panic::take_hook();
    let path = std::env::var("SI1C_EE_OUT").expect("SI1C_EE_OUT");
    fs::write(path, out.0).unwrap();
}

// ---------------------------------------------------------------- runner

fn input_decl(id: &str, source: &str) -> Value {
    json!({ "input_id": id, "name": id, "source_kind": source, "required_for": "rule_check",
            "provenance_required": true, "redistribution_status_required": true,
            "quantity_intent": { "dimension": "stress", "unit_ref": "Pa",
                                 "unit_required": true, "dimension_check_required": true } })
}

/// One pack with one check: formula over x (solver) and s (user), with u
/// (user) required by the check but not by the formula; a quantity formula is
/// compared with the stress slot `limit` (Pa).
fn pack(check_id: &str, formula: &Expression, quantity: bool, relation: Option<&str>) -> Value {
    let reference = |id: &str| json!({ "ref_id": id, "ref_type": "required_input" });
    let mut check = json!({
        "check_id": check_id,
        "required_input_refs": [reference("x"), reference("s"), reference("u")],
        "formula_ref": { "ref_id": "f", "ref_type": "formula" },
        "result_statuses": [ "RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED" ],
        "diagnostic_policy": { "missing_input": "RULE_INPUT_MISSING", "evaluator_error": "RULE_EVALUATOR_ERROR" }
    });
    if quantity {
        check["value_slot_refs"] = json!([{ "ref_id": "limit", "ref_type": "value_slot" }]);
    }
    if let Some(relation) = relation {
        check["acceptability_relation"] = json!(relation);
    }
    json!({
        "grammar_version": "1.0.0",
        "metadata": { "rule_pack_id": "si1c_family" },
        "required_inputs": [input_decl("x", "solver_result"), input_decl("s", "user_supplied_rule_value"),
                            input_decl("u", "user_supplied_rule_value")],
        "formula_declarations": [{ "formula_id": "f",
            "declaration_payload": { "expression_ast": encode_expression(formula) },
            "input_refs": [reference("x"), reference("s")] }],
        "value_slots": [{ "slot_id": "limit", "slot_kind": "limit",
            "quantity_intent": { "dimension": "stress", "unit_ref": "Pa",
                                 "unit_required": true, "dimension_check_required": true } }],
        "check_definitions": [check]
    })
}

/// Stress producers over the runner inputs (x in Pa).
fn runner_producers() -> Vec<(String, Expression)> {
    use BinaryOperator::*;
    let pa = |v: f64| lit(v, Dimension::Stress, "Pa");
    let ratio_of_x = || bin(Multiply, bin(Divide, var("x"), var("s")), ratio(1.0e300));
    vec![
        ("add".into(), bin(Add, var("x"), var("x"))),
        ("sub".into(), bin(Subtract, var("x"), pa(-1.0e308))),
        ("mul".into(), bin(Multiply, var("x"), ratio(1.0e10))),
        ("div".into(), bin(Divide, var("x"), ratio(1.0e-10))),
        ("nan".into(), bin(Subtract, bin(Multiply, ratio(1.0e300), var("x")), bin(Multiply, ratio(1.0e300), var("x")))),
        ("max_absorb".into(), aggregate(AggregateFunction::Max, vec![bin(Subtract, bin(Multiply, var("x"), ratio(1.0e300)), bin(Multiply, var("x"), ratio(1.0e300))), var("s")])),
        ("select_absorb".into(), select(no(), bin(Multiply, var("x"), ratio(1.0e300)), var("s"))),
        ("divby_absorb".into(), bin(Multiply, var("s"), bin(Divide, var("s"), bin(Multiply, var("x"), ratio(1.0e300))))),
        (
            "interp_run".into(),
            Expression::Interpolate {
                table: table("t_run", &[(-MAX, 0.0), (MAX, 10.0)], Dimension::Stress, "Pa"),
                argument: Box::new(bin(Multiply, bin(Divide, var("x"), var("s")), ratio(0.0))),
            },
        ),
        (
            "interp_argument".into(),
            Expression::Interpolate {
                table: table("t_arg", &[(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)], Dimension::Stress, "Pa"),
                argument: Box::new(bin(Subtract, ratio_of_x(), ratio_of_x())),
            },
        ),
        ("plain".into(), var("x")),
    ]
}

fn runner_formulas() -> Vec<(String, Expression, bool, Option<&'static str>)> {
    use ComparisonOperator::*;
    let pa = |v: f64| lit(v, Dimension::Stress, "Pa");
    let mut out = Vec::new();
    for (name, p) in runner_producers() {
        out.push((format!("{name}_q"), p.clone(), true, None));
        out.push((format!("{name}_q_gt"), p.clone(), true, Some("greater_than")));
        out.push((format!("{name}_b_le"), cmp(LessThanOrEqual, p.clone(), pa(100.0)), false, None));
        out.push((format!("{name}_b_ne"), cmp(NotEqual, p.clone(), pa(100.0)), false, None));
        out.push((format!("{name}_b_not"), unary(UnaryOperator::Not, cmp(GreaterThan, p.clone(), pa(100.0))), false, None));
        out.push((format!("{name}_b_or"), logical(LogicalOperator::Or, cmp(GreaterThanOrEqual, p.clone(), pa(100.0)), no()), false, None));
        out.push((format!("{name}_b_sel"), select(cmp(GreaterThan, p.clone(), pa(1.0)), yes(), no()), false, None));
    }
    out
}

/// (label, x (value, unit), s, u, limit): overflow, controls and N-4 values.
fn runner_values() -> Vec<(&'static str, (f64, &'static str), (f64, &'static str), (f64, &'static str), (f64, &'static str))> {
    vec![
        ("overflow", (1.0e308, "Pa"), (2.0, "Pa"), (3.0, "Pa"), (100.0, "Pa")),
        ("overflow_neg", (-1.0e308, "Pa"), (2.0, "Pa"), (3.0, "Pa"), (100.0, "Pa")),
        ("control", (1.0, "Pa"), (2.0, "Pa"), (3.0, "Pa"), (100.0, "Pa")),
        ("control_mpa", (1.0, "MPa"), (2.0, "Pa"), (3.0, "Pa"), (100.0, "kPa")),
        ("x_nan", (f64::NAN, "Pa"), (2.0, "Pa"), (3.0, "Pa"), (100.0, "Pa")),
        ("x_pinf", (f64::INFINITY, "Pa"), (2.0, "Pa"), (3.0, "Pa"), (100.0, "Pa")),
        ("x_ninf", (f64::NEG_INFINITY, "Pa"), (2.0, "Pa"), (3.0, "Pa"), (100.0, "Pa")),
        ("x_norm_overflow", (1.0e300, "GPa"), (2.0, "Pa"), (3.0, "Pa"), (100.0, "Pa")),
        ("x_norm_overflow_neg", (-1.0e300, "GPa"), (2.0, "Pa"), (3.0, "Pa"), (100.0, "Pa")),
        ("x_nan_kpa", (f64::NAN, "kPa"), (2.0, "Pa"), (3.0, "Pa"), (100.0, "Pa")),
        ("x_pinf_kpa", (f64::INFINITY, "kPa"), (2.0, "Pa"), (3.0, "Pa"), (100.0, "Pa")),
        ("s_nan", (1.0, "Pa"), (f64::NAN, "Pa"), (3.0, "Pa"), (100.0, "Pa")),
        ("s_norm_overflow", (1.0, "Pa"), (1.0e300, "GPa"), (3.0, "Pa"), (100.0, "Pa")),
        ("u_nan", (1.0, "Pa"), (2.0, "Pa"), (f64::NAN, "Pa"), (100.0, "Pa")),
        ("u_norm_overflow", (1.0, "Pa"), (2.0, "Pa"), (1.0e300, "GPa"), (100.0, "Pa")),
        ("u_nan_kpa", (1.0, "Pa"), (2.0, "Pa"), (f64::NAN, "kPa"), (100.0, "Pa")),
        ("limit_nan", (1.0, "Pa"), (2.0, "Pa"), (3.0, "Pa"), (f64::NAN, "Pa")),
        ("limit_pinf", (1.0, "Pa"), (2.0, "Pa"), (3.0, "Pa"), (f64::INFINITY, "Pa")),
        ("limit_norm_overflow", (1.0, "Pa"), (2.0, "Pa"), (3.0, "Pa"), (1.0e300, "GPa")),
        ("limit_nan_kpa", (1.0, "Pa"), (2.0, "Pa"), (3.0, "Pa"), (f64::NAN, "kPa")),
        ("overflow_limit_nan", (1.0e308, "Pa"), (2.0, "Pa"), (3.0, "Pa"), (f64::NAN, "Pa")),
        ("x_nan_s_nan", (f64::NAN, "Pa"), (f64::NAN, "Pa"), (3.0, "Pa"), (100.0, "Pa")),
    ]
}

fn run_line(out: &mut String, label: &str, mode: &str, input: &RuleCheckRunInput, bounds: Option<&[SolverResultBound]>) {
    let text = match catch_unwind(AssertUnwindSafe(|| match bounds {
        None => run_rule_checks(input),
        Some(b) => run_rule_checks_with_bounds(input, b),
    })) {
        Ok(result) => serde_json::to_string(&result).unwrap(),
        Err(p) => format!("PANIC({})", panic_msg(&p).replace('\n', " ")),
    };
    writeln!(out, "{label}\t{mode}\t{text}").unwrap();
}

#[test]
fn si1c_runner_family() {
    std::panic::set_hook(Box::new(|_| {}));
    let mut out = String::new();
    for (name, formula, quantity, relation) in runner_formulas() {
        let document = pack(&format!("c_{name}"), &formula, quantity, relation);
        for (vlabel, x, s, u, limit) in runner_values() {
            let supplied = |ref_id: &str, (value, unit): (f64, &str)| SuppliedValueBinding {
                ref_id: ref_id.to_string(),
                value,
                unit: unit.to_string(),
                dimension: encode_dimension(Dimension::Stress).to_string(),
            };
            let input = RuleCheckRunInput {
                rule_pack_document: &document,
                solver_results: vec![SolverResultBinding {
                    input_id: "x".into(),
                    result_id: "result:x".into(),
                    value: x.0,
                    unit: x.1.into(),
                }],
                refused_solver_results: Vec::new(),
                supplied_values: vec![supplied("s", s), supplied("u", u), supplied("limit", limit)],
                library_values: Vec::new(),
                current_statuses: vec![AnalysisStatus::MechanicsSolved],
            };
            let label = format!("r_{name}_{vlabel}");
            let bound = |b: f64| vec![SolverResultBound { input_id: "x".into(), absolute_bound: b }];
            run_line(&mut out, &label, "p", &input, None);
            run_line(&mut out, &label, "b0", &input, Some(&bound(0.0)));
            run_line(&mut out, &label, "b1", &input, Some(&bound(1.0)));
            run_line(&mut out, &label, "bR", &input, Some(&bound(x.0.abs() * 1.0e-9 + 1.0e-300)));
            run_line(&mut out, &label, "bH", &input, Some(&bound(1.0e300)));
            run_line(&mut out, &label, "bN", &input, Some(&bound(f64::NAN)));
            let dup = vec![
                SolverResultBound { input_id: "x".into(), absolute_bound: 0.0 },
                SolverResultBound { input_id: "x".into(), absolute_bound: 0.0 },
            ];
            run_line(&mut out, &label, "bD", &input, Some(&dup));
        }
    }
    let _ = std::panic::take_hook();
    let path = std::env::var("SI1C_RUN_OUT").expect("SI1C_RUN_OUT");
    fs::write(path, out).unwrap();
}
