//! I79 scratch probe (not repository content): a boolean formula over a
//! carried non-finite intermediate is decided on the point path (no panic,
//! unchanged by T3-SI1b), while interval mode reads it U.
use open_pipe_stress_expression_evaluator::*;

fn q(v: f64, d: Dimension, u: &str) -> Quantity { Quantity::new(v, d, u).unwrap() }
fn bin(o: BinaryOperator, l: Expression, r: Expression) -> Expression { Expression::Binary { operator: o, left: Box::new(l), right: Box::new(r) } }

#[test]
fn i79_carried_non_finite_probe() {
    let inf = || bin(BinaryOperator::Multiply, Expression::Literal(q(1e300, Dimension::Dimensionless, "ratio")), Expression::VariableRef("x".into()));
    let nan = bin(BinaryOperator::Subtract, inf(), inf());
    let limit = Expression::Literal(q(100.0, Dimension::Stress, "u"));
    let cases = vec![
        ("not(NaN > limit)", Expression::Unary { operator: UnaryOperator::Not, operand: Box::new(Expression::Compare { operator: ComparisonOperator::GreaterThan, left: Box::new(nan.clone()), right: Box::new(limit.clone()) }) }),
        ("NaN != limit", Expression::Compare { operator: ComparisonOperator::NotEqual, left: Box::new(nan.clone()), right: Box::new(limit.clone()) }),
        ("inf >= limit", Expression::Compare { operator: ComparisonOperator::GreaterThanOrEqual, left: Box::new(inf()), right: Box::new(limit.clone()) }),
    ];
    let bindings = vec![VariableBinding::new("x", BindingSource::SolverResultField, q(1e300, Dimension::Stress, "u"))];
    for (name, expression) in cases {
        let input = EvaluationInput { expression, bindings: bindings.clone(), required_variable_ids: vec![],
            statuses: vec![AnalysisStatus::MechanicsSolved], declared_grammar_version: GRAMMAR_VERSION.into() };
        let point = evaluate(&input);
        let interval = evaluate_interval(&input, &[]);
        println!("PROBE {name}: point value={:?} findings={} | interval value={:?} notes={:?}", point.value, point.findings.len(),
                 interval.value, interval.notes.iter().map(|n| n.code.as_str()).collect::<Vec<_>>());
    }
}
