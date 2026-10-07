
// ---------------------------------------------------------------------------
// I73 S-I1 point-mode differential (scratch harness, not repository content).
// I79 T3-SI1b extension: a panic hook records each panic's site in a side
// file (the main dump bytes are unchanged), `gen` takes its literal pool as a
// parameter (same RNG call sequence), `i79_extreme_dump` adds a second
// seeded set with extreme magnitudes, and `i79_table_dump` a third set whose
// root is a table over a deep extreme-magnitude ratio argument. Every input
// is also run through `evaluate_interval` with four overlay variants, into
// `<dump>.interval` (interval mode must be unchanged).
// Dumps `evaluate` results over the committed conformance corpus and a seeded
// set of generated expressions with point bindings. Base and candidate dumps
// must be byte-identical.
// ---------------------------------------------------------------------------

use std::fmt::Write as _;
use open_pipe_stress_expression_evaluator::{enclosure_from_bound, evaluate_interval, IntervalBinding};

thread_local! {
    static INTERVAL_OUT: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
    static LAST_PANIC: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
}
static HOOK: std::sync::Once = std::sync::Once::new();

fn install_hook() {
    HOOK.call_once(|| {
        std::panic::set_hook(Box::new(|info| {
            let site = info
                .location()
                .map(|l| {
                    let file = std::path::Path::new(l.file()).file_name().unwrap().to_string_lossy().to_string();
                    format!("{file}:{}:{}", l.line(), l.column())
                })
                .unwrap_or_default();
            let message = info
                .payload()
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| info.payload().downcast_ref::<String>().cloned())
                .unwrap_or_default();
            LAST_PANIC.with(|p| *p.borrow_mut() = format!("{site}\t{message}"));
        }));
    });
}

fn dump_interval(label: &str, input: &EvaluationInput) {
    let quantities: Vec<(&str, f64)> = input
        .bindings
        .iter()
        .filter_map(|b| b.quantity.as_ref().map(|q| (b.variable_id.as_str(), q.value)))
        .collect();
    let mut variants: Vec<(&str, Vec<IntervalBinding>)> = vec![("points", vec![])];
    if let Some(&(id, v)) = quantities.first() {
        variants.push(("first_b0.5", vec![IntervalBinding { variable_id: id.to_string(), enclosure: enclosure_from_bound(v, 0.5) }]));
        variants.push(("first_none", vec![IntervalBinding { variable_id: id.to_string(), enclosure: None }]));
    }
    variants.push(("all_b0.25", quantities.iter().map(|&(id, v)| IntervalBinding { variable_id: id.to_string(), enclosure: enclosure_from_bound(v, 0.25) }).collect()));
    for (name, overlays) in variants {
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| evaluate_interval(input, &overlays)));
        let line = match outcome {
            Ok(result) => format!("{label}\t{name}\t{result:?}\n"),
            Err(_) => format!("{label}\t{name}\tPANIC\n"),
        };
        INTERVAL_OUT.with(|o| o.borrow_mut().push_str(&line));
    }
}

fn take_interval() -> String {
    INTERVAL_OUT.with(|o| std::mem::take(&mut *o.borrow_mut()))
}

fn dump_result(out: &mut String, side: &mut String, label: &str, input: &EvaluationInput) {
    install_hook();
    dump_interval(label, input);
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| evaluate(input)));
    match outcome {
        Ok(result) => {
            let bits = match &result.value {
                Some(EvaluationValue::Quantity(q)) => format!("0x{:016x}", q.value.to_bits()),
                Some(EvaluationValue::Boolean(b)) => format!("{b}"),
                None => "none".to_string(),
            };
            writeln!(out, "{label}\t{bits}\t{result:?}").unwrap();
        }
        Err(_) => {
            writeln!(out, "{label}\tPANIC").unwrap();
            let site = LAST_PANIC.with(|p| p.borrow().clone());
            writeln!(side, "{label}\t{site}\t{input:?}").unwrap();
        }
    }
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
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

const U: &str = "invented_stress_unit";
const LITS: &[f64] = &[0.0, -0.0, 1.0, -1.0, 0.1, 3.0, -2.5, 7.0, 1.0e-300, 1.0e300, 0.5, 2.0];

fn st(v: f64) -> Expression {
    Expression::Literal(Quantity::new(v, Dimension::Stress, U).unwrap())
}
fn ra(v: f64) -> Expression {
    Expression::Literal(Quantity::dimensionless(v, "ratio").unwrap())
}
fn vr(id: &str) -> Expression {
    Expression::VariableRef(id.to_string())
}
fn bx(e: Expression) -> Box<Expression> {
    Box::new(e)
}
fn tbl(rows: &[(f64, f64)]) -> UserTable {
    UserTable {
        table_id: "diff_table".to_string(),
        argument_dimension: Dimension::Dimensionless,
        argument_unit_ref: "ratio".to_string(),
        result_dimension: Dimension::Stress,
        result_unit_ref: U.to_string(),
        rows: rows.iter().map(|&(argument, result)| TableRow { argument, result }).collect(),
    }
}

const LITS_X: &[f64] = &[
    0.0, -0.0, 1.0, -1.0, 0.1, 3.0, -2.5, 7.0, 1.0e-300, 1.0e300, 0.5, 2.0,
    1.0e308, -1.0e308, f64::MAX, 5.0e-324, -5.0e-324, 1.0e-308, 1.0e154,
];

// kind: 0 stress, 1 ratio, 2 boolean
fn gen(r: &mut Rng, kind: u64, depth: u32) -> Expression {
    gen_l(r, LITS, kind, depth)
}

fn gen_l(r: &mut Rng, lits: &[f64], kind: u64, depth: u32) -> Expression {
    let kind = if r.below(30) == 0 { r.below(3) } else { kind };
    let leaf = depth == 0 || r.below(4) == 0;
    let bin = |op, a, b| Expression::Binary { operator: op, left: bx(a), right: bx(b) };
    match kind {
        0 => {
            if leaf {
                return match r.below(4) { 0 => vr("x"), 1 => vr("y"), 2 => vr("missing"), _ => st(r.pick(lits)) };
            }
            match r.below(12) {
                0 => bin(BinaryOperator::Add, gen_l(r, lits, 0, depth - 1), gen_l(r, lits, 0, depth - 1)),
                1 => bin(BinaryOperator::Subtract, gen_l(r, lits, 0, depth - 1), gen_l(r, lits, 0, depth - 1)),
                2 => bin(BinaryOperator::Multiply, gen_l(r, lits, 1, depth - 1), gen_l(r, lits, 0, depth - 1)),
                3 => bin(BinaryOperator::Divide, gen_l(r, lits, 0, depth - 1), gen_l(r, lits, 1, depth - 1)),
                4 => Expression::Unary { operator: UnaryOperator::Negate, operand: bx(gen_l(r, lits, 0, depth - 1)) },
                5 => Expression::Unary { operator: UnaryOperator::Abs, operand: bx(gen_l(r, lits, 0, depth - 1)) },
                6 => Expression::Aggregate {
                    function: if r.below(2) == 0 { AggregateFunction::Min } else { AggregateFunction::Max },
                    operands: (0..r.below(4)).map(|_| gen_l(r, lits, 0, depth - 1)).collect(),
                },
                7 => Expression::Select { condition: bx(gen_l(r, lits, 2, depth - 1)), then_branch: bx(gen_l(r, lits, 0, depth - 1)), else_branch: bx(gen_l(r, lits, 0, depth - 1)) },
                8 => Expression::Interpolate { table: tbl(&[(-2.0, 1.0), (0.0, -3.0), (0.5, 4.0), (3.0, 4.5)]), argument: bx(gen_l(r, lits, 1, depth - 1)) },
                9 => Expression::Lookup { table: tbl(&[(-2.0, 1.0), (0.0, -3.0), (0.5, 4.0), (3.0, 4.5)]), mode: LookupMode::Step, argument: bx(gen_l(r, lits, 1, depth - 1)) },
                10 => Expression::Lookup { table: tbl(&[(-1.0, 1.0), (0.0, -3.0), (0.5, 4.0), (1.0, 4.5)]), mode: LookupMode::Exact, argument: bx(gen_l(r, lits, 1, depth - 1)) },
                _ => Expression::UnsupportedForm { form_id: "power".to_string() },
            }
        }
        1 => {
            if leaf {
                return if r.below(2) == 0 { vr("z") } else { ra(r.pick(lits)) };
            }
            match r.below(5) {
                0 => bin(BinaryOperator::Divide, gen_l(r, lits, 0, depth - 1), gen_l(r, lits, 0, depth - 1)),
                1 => bin(BinaryOperator::Add, gen_l(r, lits, 1, depth - 1), gen_l(r, lits, 1, depth - 1)),
                2 => bin(BinaryOperator::Multiply, gen_l(r, lits, 1, depth - 1), gen_l(r, lits, 1, depth - 1)),
                3 => Expression::Unary { operator: UnaryOperator::Abs, operand: bx(gen_l(r, lits, 1, depth - 1)) },
                _ => bin(BinaryOperator::Subtract, gen_l(r, lits, 1, depth - 1), gen_l(r, lits, 1, depth - 1)),
            }
        }
        _ => {
            let ops = [
                ComparisonOperator::LessThan, ComparisonOperator::LessThanOrEqual, ComparisonOperator::GreaterThan,
                ComparisonOperator::GreaterThanOrEqual, ComparisonOperator::Equal, ComparisonOperator::NotEqual,
            ];
            let op = ops[r.below(6) as usize];
            if leaf {
                return Expression::Compare { operator: op, left: bx(gen_l(r, lits, 0, 0)), right: bx(st(r.pick(lits))) };
            }
            match r.below(6) {
                0 | 1 => Expression::Compare { operator: op, left: bx(gen_l(r, lits, 0, depth - 1)), right: bx(gen_l(r, lits, 0, depth - 1)) },
                2 => Expression::Compare { operator: op, left: bx(gen_l(r, lits, 1, depth - 1)), right: bx(gen_l(r, lits, 1, depth - 1)) },
                3 => Expression::Logical {
                    operator: if r.below(2) == 0 { LogicalOperator::And } else { LogicalOperator::Or },
                    left: bx(gen_l(r, lits, 2, depth - 1)), right: bx(gen_l(r, lits, 2, depth - 1)),
                },
                4 => Expression::Unary { operator: UnaryOperator::Not, operand: bx(gen_l(r, lits, 2, depth - 1)) },
                _ => Expression::Select { condition: bx(gen_l(r, lits, 2, depth - 1)), then_branch: bx(gen_l(r, lits, 2, depth - 1)), else_branch: bx(gen_l(r, lits, 2, depth - 1)) },
            }
        }
    }
}

#[test]
fn i73_point_mode_dump() {
    let mut out = String::new();
    let mut side = String::new();
    // 1. The committed conformance corpus.
    for path in case_paths() {
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let text = fs::read_to_string(&path).unwrap();
        let json = Reader::parse(&text);
        let case = as_object(&json, &file_name);
        let context = file_name.as_str();
        let mut features: BTreeSet<String> = BTreeSet::new();
        let expression = decode_expression(field(case, "expression", context), context, &mut features);
        let bindings = array_field(case, "bindings", context)
            .iter()
            .map(|binding| {
                let binding = as_object(binding, context);
                let variable_id = string_field(binding, "variable_id", context);
                let source = decode_source(&string_field(binding, "source", context), context);
                match field(binding, "quantity", context) {
                    Json::Null => VariableBinding { variable_id, source, quantity: None },
                    quantity => VariableBinding { variable_id, source, quantity: Some(decode_quantity(quantity, context)) },
                }
            })
            .collect();
        let required_variable_ids = array_field(case, "required_variable_ids", context)
            .iter()
            .map(|id| { let Json::String(id) = id else { panic!() }; id.clone() })
            .collect();
        let statuses = array_field(case, "statuses", context)
            .iter()
            .map(|s| { let Json::String(t) = s else { panic!() }; decode_status(t, context) })
            .collect();
        let declared_grammar_version = string_field(case, "declared_grammar_version", context);
        let input = EvaluationInput { expression, bindings, required_variable_ids, statuses, declared_grammar_version };
        dump_result(&mut out, &mut side, &file_name, &input);
    }
    // 2. Seeded generated expressions at point bindings.
    let mut r = Rng(0xD1FF_0073_5EED_0001);
    let status_sets: [Vec<AnalysisStatus>; 4] = [
        vec![AnalysisStatus::MechanicsSolved],
        vec![],
        vec![AnalysisStatus::HumanApprovedForProject],
        vec![AnalysisStatus::ModelIncomplete, AnalysisStatus::MechanicsSolved],
    ];
    for i in 0..6000u32 {
        let kind = [2, 2, 0, 1][r.below(4) as usize];
        let expression = gen(&mut r, kind, 3);
        for j in 0..6u32 {
            let mut bindings = vec![
                VariableBinding::new("x", BindingSource::SolverResultField, Quantity::new(r.pick(&[-3.0, 0.0, 1.0, 7.0, 0.1, 1.0e300]) + if r.below(2) == 0 { r.unit() } else { 0.0 }, Dimension::Stress, U).unwrap()),
                VariableBinding::new("y", BindingSource::UserSuppliedValue, Quantity::new(r.pick(&[-2.5, 0.0, 3.0, 1.0e-300]), Dimension::Stress, U).unwrap()),
                VariableBinding::new("z", BindingSource::RulePackRequiredInput, Quantity::new(r.pick(&[-1.0, 0.0, 0.5, 1.0, 1.5, 2.0, 0.25]), Dimension::Dimensionless, "ratio").unwrap()),
            ];
            if r.below(10) == 0 {
                bindings.push(VariableBinding::missing("w", BindingSource::UserSuppliedValue));
            }
            let required_variable_ids = if r.below(8) == 0 { vec!["w".to_string(), "x".to_string()] } else { vec![] };
            let statuses = status_sets[r.below(4) as usize].clone();
            let declared_grammar_version = if r.below(50) == 0 { "1.1.0".to_string() } else { GRAMMAR_VERSION.to_string() };
            let input = EvaluationInput { expression: expression.clone(), bindings, required_variable_ids, statuses, declared_grammar_version };
            dump_result(&mut out, &mut side, &format!("gen_{i}_{j}"), &input);
        }
    }
    let path = std::env::var("I73_DUMP_OUT").expect("I73_DUMP_OUT");
    fs::write(&path, out).unwrap();
    fs::write(format!("{path}.panics"), side).unwrap();
    fs::write(format!("{path}.interval"), take_interval()).unwrap();
}

/// I79: a second seeded set at extreme magnitudes (same grammar generator,
/// literal pool `LITS_X`, extreme point bindings).
#[test]
fn i79_extreme_dump() {
    let mut out = String::new();
    let mut side = String::new();
    let mut r = Rng(0xD1FF_0079_5EED_0002);
    let status_sets: [Vec<AnalysisStatus>; 4] = [
        vec![AnalysisStatus::MechanicsSolved],
        vec![],
        vec![AnalysisStatus::HumanApprovedForProject],
        vec![AnalysisStatus::ModelIncomplete, AnalysisStatus::MechanicsSolved],
    ];
    let xs = [-3.0, 0.0, 1.0, 7.0, 0.1, 1.0e300, 1.0e308, -1.0e308, f64::MAX, 5.0e-324];
    let ys = [-2.5, 0.0, 3.0, 1.0e-300, 1.0e-308, 5.0e-324, -5.0e-324, 1.0e308];
    let zs = [-1.0, 0.0, 0.5, 1.0, 1.5, 2.0, 0.25, 1.0e300, 1.0e-300, 1.0e308];
    for i in 0..6000u32 {
        let kind = [2, 2, 0, 1][r.below(4) as usize];
        let expression = gen_l(&mut r, LITS_X, kind, 3);
        for j in 0..6u32 {
            let mut bindings = vec![
                VariableBinding::new("x", BindingSource::SolverResultField, Quantity::new(r.pick(&xs) + if r.below(2) == 0 { r.unit() } else { 0.0 }, Dimension::Stress, U).unwrap()),
                VariableBinding::new("y", BindingSource::UserSuppliedValue, Quantity::new(r.pick(&ys), Dimension::Stress, U).unwrap()),
                VariableBinding::new("z", BindingSource::RulePackRequiredInput, Quantity::new(r.pick(&zs), Dimension::Dimensionless, "ratio").unwrap()),
            ];
            if r.below(10) == 0 {
                bindings.push(VariableBinding::missing("w", BindingSource::UserSuppliedValue));
            }
            let required_variable_ids = if r.below(8) == 0 { vec!["w".to_string(), "x".to_string()] } else { vec![] };
            let statuses = status_sets[r.below(4) as usize].clone();
            let declared_grammar_version = if r.below(50) == 0 { "1.1.0".to_string() } else { GRAMMAR_VERSION.to_string() };
            let input = EvaluationInput { expression: expression.clone(), bindings, required_variable_ids, statuses, declared_grammar_version };
            dump_result(&mut out, &mut side, &format!("x_{i}_{j}"), &input);
        }
    }
    let path = std::env::var("I79_EXTREME_OUT").expect("I79_EXTREME_OUT");
    fs::write(&path, out).unwrap();
    fs::write(format!("{path}.panics"), side).unwrap();
    fs::write(format!("{path}.interval"), take_interval()).unwrap();
}

/// I79: a third seeded set: every formula's root is an interpolation, a step
/// lookup or an exact lookup over a depth-3 ratio argument drawn from
/// `LITS_X`, so non-finite (including NaN) table arguments occur.
#[test]
fn i79_table_dump() {
    let mut out = String::new();
    let mut side = String::new();
    let mut r = Rng(0xD1FF_0079_5EED_0003);
    let zs = [-1.0, 0.0, 0.5, 1.0, 1.5, 2.0, 0.25, 1.0e300, 1.0e-300, 1.0e308, f64::MAX];
    let xs = [-3.0, 0.0, 1.0, 7.0, 0.1, 1.0e300, 1.0e308, -1.0e308, f64::MAX, 5.0e-324];
    let ys = [-2.5, 0.0, 3.0, 1.0e-300, 1.0e-308, 5.0e-324, -5.0e-324, 1.0e308];
    for i in 0..3000u32 {
        let argument = bx(gen_l(&mut r, LITS_X, 1, 3));
        let expression = match r.below(3) {
            0 => Expression::Interpolate { table: tbl(&[(-2.0, 1.0), (0.0, -3.0), (0.5, 4.0), (3.0, 4.5)]), argument },
            1 => Expression::Lookup { table: tbl(&[(-2.0, 1.0), (0.0, -3.0), (0.5, 4.0), (3.0, 4.5)]), mode: LookupMode::Step, argument },
            _ => Expression::Lookup { table: tbl(&[(-1.0, 1.0), (0.0, -3.0), (0.5, 4.0), (1.0, 4.5)]), mode: LookupMode::Exact, argument },
        };
        for j in 0..4u32 {
            let bindings = vec![
                VariableBinding::new("x", BindingSource::SolverResultField, Quantity::new(r.pick(&xs), Dimension::Stress, U).unwrap()),
                VariableBinding::new("y", BindingSource::UserSuppliedValue, Quantity::new(r.pick(&ys), Dimension::Stress, U).unwrap()),
                VariableBinding::new("z", BindingSource::RulePackRequiredInput, Quantity::new(r.pick(&zs), Dimension::Dimensionless, "ratio").unwrap()),
            ];
            let input = EvaluationInput {
                expression: expression.clone(),
                bindings,
                required_variable_ids: vec![],
                statuses: vec![AnalysisStatus::MechanicsSolved],
                declared_grammar_version: GRAMMAR_VERSION.to_string(),
            };
            dump_result(&mut out, &mut side, &format!("t_{i}_{j}"), &input);
        }
    }
    let path = std::env::var("I79_TABLE_OUT").expect("I79_TABLE_OUT");
    fs::write(&path, out).unwrap();
    fs::write(format!("{path}.panics"), side).unwrap();
    fs::write(format!("{path}.interval"), take_interval()).unwrap();
}
