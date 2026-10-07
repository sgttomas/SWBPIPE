//! RV104 scratch probe (not repository content): named inputs for the
//! ordering and wording of the point path's findings. Prints one line each.
//! All values are invented.

use std::panic::{catch_unwind, AssertUnwindSafe};

use open_pipe_stress_expression_evaluator::{
    evaluate, AnalysisStatus, BinaryOperator, BindingSource, ComparisonOperator, Dimension,
    EvaluationInput, Expression, LogicalOperator, LookupMode, Quantity, TableRow, UnaryOperator,
    UserTable, VariableBinding, GRAMMAR_VERSION,
};

const MAX: f64 = f64::MAX;

fn q(value: f64, dimension: Dimension, unit: &str) -> Quantity {
    Quantity { value, dimension, unit_ref: unit.into(), unit_required: true, dimension_check_required: true }
}
fn lit(v: f64, d: Dimension, u: &str) -> Expression {
    Expression::Literal(q(v, d, u))
}
fn ratio(v: f64) -> Expression {
    lit(v, Dimension::Dimensionless, "ratio")
}
fn var(id: &str) -> Expression {
    Expression::VariableRef(id.into())
}
fn bin(op: BinaryOperator, a: Expression, b: Expression) -> Expression {
    Expression::Binary { operator: op, left: Box::new(a), right: Box::new(b) }
}
fn div(a: Expression, b: Expression) -> Expression {
    bin(BinaryOperator::Divide, a, b)
}
fn huge(e: Expression) -> Expression {
    bin(BinaryOperator::Multiply, ratio(MAX), bin(BinaryOperator::Multiply, ratio(MAX), e))
}
fn nan_of(e: Expression) -> Expression {
    bin(BinaryOperator::Subtract, huge(e.clone()), huge(e))
}
fn table(id: &str, rows: &[(f64, f64)], d: Dimension, u: &str) -> UserTable {
    UserTable {
        table_id: id.into(),
        argument_dimension: d,
        argument_unit_ref: u.into(),
        result_dimension: Dimension::Stress,
        result_unit_ref: "su".into(),
        rows: rows.iter().map(|&(argument, result)| TableRow { argument, result }).collect(),
    }
}
fn interp(t: UserTable, a: Expression) -> Expression {
    Expression::Interpolate { table: t, argument: Box::new(a) }
}
fn lookup(t: UserTable, m: LookupMode, a: Expression) -> Expression {
    Expression::Lookup { table: t, mode: m, argument: Box::new(a) }
}

#[test]
fn rv104_probe() {
    std::panic::set_hook(Box::new(|_| {}));
    let s = |id: &str, v: f64| VariableBinding::new(id, BindingSource::SolverResultField, q(v, Dimension::Stress, "su"));
    let s2 = |id: &str, v: f64| VariableBinding::new(id, BindingSource::SolverResultField, q(v, Dimension::Stress, "su2"));
    let z = |id: &str, v: f64| VariableBinding::new(id, BindingSource::UserSuppliedValue, q(v, Dimension::Dimensionless, "ratio"));
    let t = || table("t", &[(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)], Dimension::Dimensionless, "ratio");
    let cases: Vec<(&str, Expression, Vec<VariableBinding>)> = vec![
        ("01 overflow 1e308/1e-308 same unit", div(var("a"), var("b")), vec![s("a", 1e308), s("b", 1e-308)]),
        ("02 overflow with unit mismatch", div(var("a"), var("b")), vec![s("a", 1e308), s2("b", 1e-308)]),
        ("03 NaN numerator over zero", div(nan_of(var("a")), var("b")), vec![s("a", 1.0), s("b", 0.0)]),
        ("04 NaN numerator over -0", div(nan_of(var("a")), var("b")), vec![s("a", 1.0), s("b", -0.0)]),
        ("05 finite over carried NaN divisor", div(var("a"), nan_of(var("b"))), vec![s("a", 1.0), s("b", 1.0)]),
        ("06 carried inf over carried inf", div(huge(var("a")), huge(var("b"))), vec![s("a", 1.0), s("b", 1.0)]),
        ("07 finite over carried inf (ratio 0)", div(var("a"), huge(var("b"))), vec![s("a", 1.0), s("b", 1.0)]),
        ("08 largest finite ratio MAX/1", div(var("a"), var("b")), vec![s("a", MAX), s("b", 1.0)]),
        ("09 MAX/next_down(1) overflows", div(var("a"), var("b")), vec![s("a", MAX), s("b", 1.0f64.next_down())]),
        ("10 subnormal over MAX underflows to 0", div(var("a"), var("b")), vec![s("a", 5e-324), s("b", MAX)]),
        ("11 overflow stress/length (unrepresentable)",
            div(var("a"), var("l")),
            vec![s("a", 1e308), VariableBinding::new("l", BindingSource::SolverResultField, q(1e-308, Dimension::Length, "lu"))]),
        ("12 overflow moment/length (derived; carried)",
            div(var("m"), var("l")),
            vec![VariableBinding::new("m", BindingSource::SolverResultField, q(1e308, Dimension::Moment, "mu")),
                 VariableBinding::new("l", BindingSource::SolverResultField, q(1e-308, Dimension::Length, "lu"))]),
        ("13 overflow stress/ratio (carried)", div(var("a"), var("z")), vec![s("a", 1e308), z("z", 1e-308)]),
        ("14 overflow ratio/ratio (carried)", div(var("y"), var("z")), vec![z("y", 1e308), z("z", 1e-308)]),
        ("15 nested (a/b)/(c/d), a/b overflows",
            div(div(var("a"), var("b")), div(var("c"), var("d"))),
            vec![s("a", 1e308), s("b", 1e-308), s("c", 1.0), s("d", 0.0)]),
        ("16 untaken select branch overflows",
            Expression::Select {
                condition: Box::new(Expression::Compare { operator: ComparisonOperator::LessThan, left: Box::new(var("c")), right: Box::new(var("d")) }),
                then_branch: Box::new(ratio(1.0)),
                else_branch: Box::new(div(var("a"), var("b"))),
            },
            vec![s("a", 1e308), s("b", 1e-308), s("c", 1.0), s("d", 2.0)]),
        ("17 and(ratio>1, missing) left blocks first",
            Expression::Logical {
                operator: LogicalOperator::And,
                left: Box::new(Expression::Compare { operator: ComparisonOperator::GreaterThan, left: Box::new(div(var("a"), var("b"))), right: Box::new(ratio(1.0)) }),
                right: Box::new(Expression::Compare { operator: ComparisonOperator::GreaterThan, left: Box::new(var("missing")), right: Box::new(ratio(1.0)) }),
            },
            vec![s("a", 1e308), s("b", 1e-308)]),
        ("18 interpolate NaN", interp(t(), nan_of(var("z"))), vec![z("z", 1.0)]),
        ("19 step NaN", lookup(t(), LookupMode::Step, nan_of(var("z"))), vec![z("z", 1.0)]),
        ("20 exact NaN", lookup(t(), LookupMode::Exact, nan_of(var("z"))), vec![z("z", 1.0)]),
        ("21 interpolate +inf", interp(t(), huge(var("z"))), vec![z("z", 1.0)]),
        ("22 step -inf", lookup(t(), LookupMode::Step, Expression::Unary { operator: UnaryOperator::Negate, operand: Box::new(huge(var("z"))) }), vec![z("z", 1.0)]),
        ("23 interpolate 0*inf NaN", interp(t(), bin(BinaryOperator::Multiply, ratio(0.0), huge(var("z")))), vec![z("z", 1.0)]),
        ("24 interpolate NaN, argument unit mismatch",
            interp(table("t", &[(0.0, 1.0), (2.0, 3.0)], Dimension::Dimensionless, "other_ratio"), nan_of(var("z"))), vec![z("z", 1.0)]),
        ("25 interpolate NaN, argument dimension mismatch",
            interp(table("t", &[(0.0, 1.0), (2.0, 3.0)], Dimension::Stress, "su"), nan_of(var("z"))), vec![z("z", 1.0)]),
        ("26 interpolate NaN, malformed table",
            interp(table("t", &[(1.0, 1.0), (0.0, 2.0)], Dimension::Dimensionless, "ratio"), nan_of(var("z"))), vec![z("z", 1.0)]),
        ("27 step NaN, padded table id",
            lookup(table("  padded_t  ", &[(0.0, 1.0)], Dimension::Dimensionless, "ratio"), LookupMode::Step, nan_of(var("z"))), vec![z("z", 1.0)]),
        ("28 interpolate NaN, table rows near MAX",
            interp(table("t", &[(-MAX, -MAX), (MAX, MAX)], Dimension::Dimensionless, "ratio"), nan_of(var("z"))), vec![z("z", 1.0)]),
        ("29 interpolate in range, result overflows (carried, then ratio blocks)",
            div(interp(table("t", &[(0.0, -1e308), (1.0, 1e308)], Dimension::Dimensionless, "ratio"), var("z")), var("a")),
            vec![z("z", 0.5), s("a", 1.0)]),
        ("30 NaN binding (blocked at binding)", div(var("a"), var("b")), vec![s("a", f64::NAN), s("b", 1.0)]),
        ("31 inf literal limit", Expression::Compare { operator: ComparisonOperator::LessThanOrEqual, left: Box::new(div(var("a"), var("b"))), right: Box::new(ratio(f64::INFINITY)) }, vec![s("a", 1.0), s("b", 2.0)]),
    ];
    for (name, expression, bindings) in cases {
        let input = EvaluationInput {
            expression,
            bindings,
            required_variable_ids: vec![],
            statuses: vec![AnalysisStatus::MechanicsSolved],
            declared_grammar_version: GRAMMAR_VERSION.to_string(),
        };
        let line = match catch_unwind(AssertUnwindSafe(|| evaluate(&input))) {
            Ok(r) => format!("{r:?}"),
            Err(_) => "PANIC".to_string(),
        };
        println!("RV104PROBE\t{name}\t{line}");
    }
}
