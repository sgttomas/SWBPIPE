//! RV104 scratch harness (not repository content). Dumps the point `evaluate`
//! and interval `evaluate_interval` results for RV104's own generated inputs
//! and the committed conformance corpus, one line per evaluation, with any
//! panic caught and labelled. Base and candidate dumps are compared outside.
//! All values are invented.

use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;

use open_pipe_stress_expression_evaluator::{
    enclosure_from_bound, evaluate, evaluate_interval, AggregateFunction, AnalysisStatus,
    BinaryOperator, BindingSource, ComparisonOperator, Dimension, Enclosure, EvaluationInput,
    EvaluationValue, Expression, IntervalBinding, IntervalValue, LogicalOperator, LookupMode,
    Quantity, TableRow, UnaryOperator, UserTable, VariableBinding, GRAMMAR_VERSION,
};
use open_pipe_stress_rule_pack_document::{decode_dimension, decode_expression};
use serde_json::Value;

// ---------------------------------------------------------------- utilities

struct Rng(u64);
impl Rng {
    // splitmix64
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    fn pick<T: Copy>(&mut self, v: &[T]) -> T {
        v[self.below(v.len() as u64) as usize]
    }
    fn chance(&mut self, k: u64) -> bool {
        self.below(k) == 0
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

fn point_line(input: &EvaluationInput) -> String {
    match catch_unwind(AssertUnwindSafe(|| evaluate(input))) {
        Ok(r) => {
            let bits = match &r.value {
                Some(EvaluationValue::Quantity(q)) => format!("0x{:016x}", q.value.to_bits()),
                Some(EvaluationValue::Boolean(b)) => b.to_string(),
                None => "none".to_string(),
            };
            format!("{bits}|{r:?}")
        }
        Err(p) => format!("PANIC({})", panic_msg(&p).replace('\n', " ")),
    }
}

fn interval_line(input: &EvaluationInput, intervals: &[IntervalBinding]) -> String {
    match catch_unwind(AssertUnwindSafe(|| evaluate_interval(input, intervals))) {
        Ok(r) => {
            let bits = match &r.value {
                Some(IntervalValue::Quantity(q)) => match q.enclosure {
                    Some(e) => e.bits_text(),
                    None => "noenc".to_string(),
                },
                Some(IntervalValue::Boolean(t)) => format!("{t:?}"),
                None => "none".to_string(),
            };
            // Compact: the value, counts, and a 64-bit FNV-1a hash of the full
            // Debug text (any difference in the result changes the hash).
            let text = format!("{r:?}");
            let mut h: u64 = 0xcbf2_9ce4_8422_2325;
            for byte in text.bytes() {
                h ^= byte as u64;
                h = h.wrapping_mul(0x0100_0000_01b3);
            }
            let codes: Vec<String> = r.findings.iter().map(|f| format!("{:?}", f.code)).collect();
            let notes: Vec<&str> = r.notes.iter().map(|n| n.code.as_str()).collect();
            format!("{bits}|f={}|n={}|{h:016x}", codes.join(","), notes.join(","))
        }
        Err(p) => format!("PANIC({})", panic_msg(&p).replace('\n', " ")),
    }
}

struct Out(String);
impl Out {
    fn point(&mut self, label: &str, input: &EvaluationInput) {
        self.0.push_str(label);
        self.0.push_str("\tP\t");
        self.0.push_str(&point_line(input));
        self.0.push('\n');
    }
    fn interval(&mut self, label: &str, k: usize, input: &EvaluationInput, iv: &[IntervalBinding]) {
        self.0.push_str(&format!("{label}#i{k}\tI\t"));
        self.0.push_str(&interval_line(input, iv));
        self.0.push('\n');
    }
    /// The point line plus four interval variants over the bound variables.
    fn both(&mut self, label: &str, input: &EvaluationInput) {
        self.point(label, input);
        let ids: Vec<(String, f64)> = input
            .bindings
            .iter()
            .filter_map(|b| b.quantity.as_ref().map(|q| (b.variable_id.clone(), q.value)))
            .collect();
        let variants: [Box<dyn Fn(f64) -> Option<Option<Enclosure>>>; 4] = [
            Box::new(|_| None),
            Box::new(|q| Some(enclosure_from_bound(q, 0.0))),
            Box::new(|q| Some(enclosure_from_bound(q, 1.0))),
            Box::new(|q| Some(enclosure_from_bound(q, q.abs() * 1.0e-6 + 1.0e-300))),
        ];
        for (k, v) in variants.iter().enumerate() {
            let mut iv = Vec::new();
            for (id, q) in &ids {
                if let Some(enclosure) = v(*q) {
                    if !iv.iter().any(|b: &IntervalBinding| &b.variable_id == id) {
                        iv.push(IntervalBinding {
                            variable_id: id.clone(),
                            enclosure,
                        });
                    }
                }
            }
            self.interval(label, k, input, &iv);
        }
    }
}

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
    Expression::Binary {
        operator: op,
        left: Box::new(a),
        right: Box::new(b),
    }
}
fn cmp(op: ComparisonOperator, a: Expression, b: Expression) -> Expression {
    Expression::Compare {
        operator: op,
        left: Box::new(a),
        right: Box::new(b),
    }
}
fn neg(a: Expression) -> Expression {
    Expression::Unary {
        operator: UnaryOperator::Negate,
        operand: Box::new(a),
    }
}
fn bind(id: &str, value: f64, dimension: Dimension, unit: &str) -> VariableBinding {
    VariableBinding::new(id, BindingSource::SolverResultField, q(value, dimension, unit))
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

const MAX: f64 = f64::MAX;

/// The finite extreme pool.
fn finite_pool() -> Vec<f64> {
    vec![
        0.0,
        -0.0,
        5e-324,
        -5e-324,
        f64::MIN_POSITIVE,
        1e-308,
        1e-300,
        1e-160,
        1e-154,
        0.5,
        1.0,
        -1.0,
        2.0,
        3.0,
        -2.5,
        1.0f64.next_down(),
        1.0f64.next_up(),
        1e154,
        1e155,
        1e300,
        1e308,
        MAX,
        -MAX,
        MAX.next_down(),
    ]
}

fn special_pool() -> Vec<f64> {
    vec![
        f64::NAN,
        -f64::NAN,
        f64::from_bits(0x7ff8_0000_0000_0001),
        f64::INFINITY,
        f64::NEG_INFINITY,
    ]
}

/// Operand forms over a variable: plain, literal copy, and carried
/// intermediates (overflow, NaN from inf - inf and from 0 * inf, underflow).
fn form(kind: usize, id: &str, value: f64, dimension: Dimension, unit: &str) -> Expression {
    let huge = || bin(BinaryOperator::Multiply, ratio(MAX), bin(BinaryOperator::Multiply, ratio(MAX), var(id)));
    match kind {
        0 => var(id),
        1 => lit(value, dimension, unit),
        2 => bin(BinaryOperator::Multiply, ratio(1e300), var(id)),
        3 => huge(),
        4 => neg(huge()),
        5 => bin(BinaryOperator::Subtract, huge(), huge()),
        6 => bin(BinaryOperator::Multiply, ratio(0.0), huge()),
        7 => bin(
            BinaryOperator::Multiply,
            ratio(1e-300),
            bin(BinaryOperator::Multiply, ratio(1e-300), var(id)),
        ),
        _ => bin(BinaryOperator::Subtract, var(id), var(id)),
    }
}
const FORMS: usize = 9;

// ---------------------------------------------------------------- families

fn family_quotients(out: &mut Out) {
    let dims: &[(Dimension, &str, Dimension, &str)] = &[
        (Dimension::Stress, "su", Dimension::Stress, "su"),
        (Dimension::Stress, "su", Dimension::Stress, "su2"),
        (Dimension::Length, "lu", Dimension::Length, "lu"),
        (Dimension::Dimensionless, "ratio", Dimension::Dimensionless, "ratio"),
        (Dimension::Dimensionless, "ratio", Dimension::Dimensionless, "other_ratio"),
        (Dimension::Stress, "su", Dimension::Dimensionless, "ratio"),
        (Dimension::Moment, "mu", Dimension::Length, "lu"),
        (Dimension::Force, "fu", Dimension::Area, "au"),
        (Dimension::Dimensionless, "ratio", Dimension::Stress, "su"),
        (Dimension::Stress, "su", Dimension::Length, "lu"),
    ];
    let pool = finite_pool();
    let small: Vec<f64> = vec![0.0, 5e-324, 1.0, -2.5, 1e300, MAX];
    for (di, &(nd, nu, dd, du)) in dims.iter().enumerate() {
        for (ai, &a) in pool.iter().enumerate() {
            for (bi, &b) in pool.iter().enumerate() {
                let e = bin(BinaryOperator::Divide, var("a"), var("b"));
                let inp = input(e, vec![bind("a", a, nd, nu), bind("b", b, dd, du)]);
                out.both(&format!("q_vv_{di}_{ai}_{bi}"), &inp);
            }
        }
        for (ai, &a) in small.iter().enumerate() {
            for (bi, &b) in small.iter().enumerate() {
                for fa in 0..FORMS {
                    for fb in 0..FORMS {
                        let e = bin(
                            BinaryOperator::Divide,
                            form(fa, "a", a, nd, nu),
                            form(fb, "b", b, dd, du),
                        );
                        let inp = input(e, vec![bind("a", a, nd, nu), bind("b", b, dd, du)]);
                        out.both(&format!("q_ff_{di}_{ai}_{bi}_{fa}_{fb}"), &inp);
                    }
                }
            }
        }
    }
    // Nested ratios: (a/b)/(c/d), a/(b/c), ((a/b)/c)/d over stresses and a ratio.
    let mut r = Rng(0x5256_3130_3451_0001);
    for i in 0..6000 {
        let vals: Vec<f64> = (0..4)
            .map(|_| {
                let v = r.pick(&pool);
                if v == 0.0 && r.chance(2) { 1.0 } else { v }
            })
            .collect();
        let shape = r.below(6);
        let e = match shape {
            0 => bin(
                BinaryOperator::Divide,
                bin(BinaryOperator::Divide, var("a"), var("b")),
                bin(BinaryOperator::Divide, var("c"), var("d")),
            ),
            1 => bin(
                BinaryOperator::Divide,
                var("a"),
                bin(BinaryOperator::Divide, var("b"), var("z")),
            ),
            2 => bin(
                BinaryOperator::Divide,
                bin(BinaryOperator::Divide, bin(BinaryOperator::Divide, var("a"), var("b")), var("z")),
                var("z"),
            ),
            3 => bin(
                BinaryOperator::Divide,
                bin(BinaryOperator::Multiply, bin(BinaryOperator::Divide, var("a"), var("b")), var("c")),
                var("d"),
            ),
            4 => cmp(
                ComparisonOperator::LessThanOrEqual,
                bin(BinaryOperator::Divide, var("a"), bin(BinaryOperator::Subtract, var("b"), var("c"))),
                ratio(1.0),
            ),
            _ => bin(
                BinaryOperator::Divide,
                bin(BinaryOperator::Add, var("a"), var("c")),
                bin(BinaryOperator::Subtract, var("b"), var("d")),
            ),
        };
        let z = r.pick(&pool);
        let inp = input(
            e,
            vec![
                bind("a", vals[0], Dimension::Stress, "su"),
                bind("b", vals[1], Dimension::Stress, "su"),
                bind("c", vals[2], Dimension::Stress, "su"),
                bind("d", vals[3], Dimension::Stress, "su"),
                bind("z", z, Dimension::Dimensionless, "ratio"),
            ],
        );
        out.both(&format!("q_nest_{i}"), &inp);
    }
}

fn table(id: &str, rows: &[(f64, f64)], arg_dim: Dimension, arg_unit: &str) -> UserTable {
    UserTable {
        table_id: id.to_string(),
        argument_dimension: arg_dim,
        argument_unit_ref: arg_unit.to_string(),
        result_dimension: Dimension::Stress,
        result_unit_ref: "su".to_string(),
        rows: rows
            .iter()
            .map(|&(argument, result)| TableRow { argument, result })
            .collect(),
    }
}

fn table_expr(t: &UserTable, mode: usize, argument: Expression) -> Expression {
    match mode {
        0 => Expression::Interpolate {
            table: t.clone(),
            argument: Box::new(argument),
        },
        1 => Expression::Lookup {
            table: t.clone(),
            mode: LookupMode::Step,
            argument: Box::new(argument),
        },
        _ => Expression::Lookup {
            table: t.clone(),
            mode: LookupMode::Exact,
            argument: Box::new(argument),
        },
    }
}

fn family_tables(out: &mut Out) {
    let rd = Dimension::Dimensionless;
    let tables = vec![
        table("t_reg", &[(-2.0, 1.0), (0.0, -3.0), (0.5, 4.0), (3.0, 4.5)], rd, "ratio"),
        table("t_ext", &[(-MAX, -MAX), (0.0, 0.0), (MAX, MAX)], rd, "ratio"),
        table("t_rise", &[(0.0, -1e308), (1.0, 1e308)], rd, "ratio"),
        table("t_sub", &[(0.0, 0.0), (5e-324, 1.0), (1e-323, 2.0)], rd, "ratio"),
        table("t_one", &[(1.0, 5.0)], rd, "ratio"),
        table("t_nanrow", &[(0.0, 1.0), (f64::NAN, 2.0)], rd, "ratio"),
        table("t_infrow", &[(0.0, 1.0), (f64::INFINITY, 2.0)], rd, "ratio"),
        table("t_nonmono", &[(1.0, 1.0), (0.0, 2.0)], rd, "ratio"),
        table("t_negzero", &[(-0.0, 1.0), (1.0, 2.0)], rd, "ratio"),
        table("t_empty", &[], rd, "ratio"),
        table("t_stressarg", &[(0.0, 1.0), (2.0, 3.0)], Dimension::Stress, "su"),
        table("  ", &[(0.0, 1.0), (2.0, 3.0)], rd, "ratio"),
    ];
    let mut pool = finite_pool();
    pool.push(0.25);
    pool.push(-2.0);
    pool.push(1e-323);
    for (ti, t) in tables.iter().enumerate() {
        for mode in 0..3 {
            for (vi, &z) in pool.iter().enumerate() {
                for f in 0..FORMS {
                    let argument = form(f, "z", z, rd, "ratio");
                    let wraps = [
                        table_expr(t, mode, argument.clone()),
                        bin(BinaryOperator::Divide, table_expr(t, mode, argument.clone()), var("s")),
                        bin(BinaryOperator::Divide, var("s"), table_expr(t, mode, argument.clone())),
                        cmp(
                            ComparisonOperator::LessThanOrEqual,
                            table_expr(t, mode, argument.clone()),
                            lit(100.0, Dimension::Stress, "su"),
                        ),
                    ];
                    for (wi, e) in wraps.into_iter().enumerate() {
                        let inp = input(
                            e,
                            vec![
                                bind("z", z, rd, "ratio"),
                                bind("s", if vi % 3 == 0 { 1e-308 } else { 2.0 }, Dimension::Stress, "su"),
                            ],
                        );
                        out.both(&format!("t_{ti}_{mode}_{vi}_{f}_{wi}"), &inp);
                    }
                }
                // Wrong-unit and wrong-dimension arguments, with a NaN carried.
                for (k, argument) in [
                    bin(BinaryOperator::Subtract, form(3, "z2", z, rd, "ratio2"), form(3, "z2", z, rd, "ratio2")),
                    form(5, "s", z, Dimension::Stress, "su"),
                    var("z2"),
                    var("s"),
                ]
                .into_iter()
                .enumerate()
                {
                    let inp = input(
                        table_expr(t, mode, argument),
                        vec![
                            bind("z2", z, rd, "ratio2"),
                            bind("s", if z == 0.0 { 1.0 } else { z }, Dimension::Stress, "su"),
                        ],
                    );
                    out.both(&format!("t_mis_{ti}_{mode}_{vi}_{k}"), &inp);
                }
            }
        }
    }
}

fn family_non_finite_bindings(out: &mut Out) {
    let specials = special_pool();
    let mut values = specials.clone();
    values.extend([5e-324, -0.0, MAX]);
    let t = table("t_reg", &[(-2.0, 1.0), (0.0, -3.0), (0.5, 4.0), (3.0, 4.5)], Dimension::Dimensionless, "ratio");
    for (vi, &v) in values.iter().enumerate() {
        let exprs = [
            bin(BinaryOperator::Divide, var("a"), var("b")),
            bin(BinaryOperator::Divide, var("b"), var("a")),
            cmp(ComparisonOperator::LessThanOrEqual, var("a"), var("b")),
            cmp(
                ComparisonOperator::LessThanOrEqual,
                bin(BinaryOperator::Divide, var("b"), var("b")),
                lit(v, Dimension::Dimensionless, "ratio"),
            ),
            bin(BinaryOperator::Divide, var("b"), lit(v, Dimension::Stress, "su")),
            bin(BinaryOperator::Divide, lit(v, Dimension::Stress, "su"), var("b")),
            table_expr(&t, 0, lit(v, Dimension::Dimensionless, "ratio")),
            table_expr(&t, 1, var("zr")),
            table_expr(&t, 2, var("zr")),
            Expression::Aggregate {
                function: AggregateFunction::Max,
                operands: vec![var("b"), lit(v, Dimension::Stress, "su")],
            },
            var("b"),
        ];
        for (ei, e) in exprs.into_iter().enumerate() {
            for req in 0..3 {
                let mut inp = input(
                    e.clone(),
                    vec![
                        bind("a", v, Dimension::Stress, "su"),
                        bind("b", 2.0, Dimension::Stress, "su"),
                        bind("zr", v, Dimension::Dimensionless, "ratio"),
                    ],
                );
                inp.required_variable_ids = match req {
                    0 => vec![],
                    1 => vec!["a".to_string()],
                    _ => vec!["a".to_string(), "b".to_string(), "missing".to_string()],
                };
                out.both(&format!("b_{vi}_{ei}_{req}"), &inp);
                // Interval overlays that are themselves non-finite or inverted.
                let overlays = [
                    vec![IntervalBinding { variable_id: "b".into(), enclosure: Some(Enclosure { lo: v, hi: 3.0 }) }],
                    vec![IntervalBinding { variable_id: "b".into(), enclosure: Some(Enclosure { lo: 1.0, hi: v }) }],
                    vec![IntervalBinding { variable_id: "b".into(), enclosure: None }],
                    vec![IntervalBinding { variable_id: "a".into(), enclosure: Some(Enclosure { lo: 3.0, hi: 1.0 }) }],
                ];
                for (k, iv) in overlays.iter().enumerate() {
                    out.interval(&format!("b_{vi}_{ei}_{req}"), 10 + k, &inp, iv);
                }
            }
        }
    }
}

// The random whole-grammar generator.
const QDIMS: &[(Dimension, &str)] = &[
    (Dimension::Stress, "su"),
    (Dimension::Stress, "su2"),
    (Dimension::Length, "lu"),
    (Dimension::Area, "au"),
    (Dimension::Force, "fu"),
    (Dimension::Moment, "mu"),
    (Dimension::Dimensionless, "ratio"),
    (Dimension::Temperature, "tu"),
];
const VARS: &[(&str, usize)] = &[
    ("s1", 0),
    ("s2", 0),
    ("s3", 1),
    ("l1", 2),
    ("l2", 2),
    ("a1", 3),
    ("f1", 4),
    ("m1", 5),
    ("z1", 6),
    ("z2", 6),
    ("t1", 7),
];

fn leaf_value(r: &mut Rng, pool: &[f64]) -> f64 {
    if r.chance(3) {
        r.pick(&[1.0, 2.0, 0.5, 3.0, -1.0])
    } else {
        r.pick(pool)
    }
}

fn gen_q(r: &mut Rng, d: usize, depth: u32, pool: &[f64]) -> Expression {
    let (dim, unit) = QDIMS[d];
    if depth == 0 || r.chance(4) {
        let vars: Vec<&str> = VARS.iter().filter(|v| v.1 == d).map(|v| v.0).collect();
        if !vars.is_empty() && r.chance(2) {
            return var(r.pick(&vars));
        }
        if r.chance(60) {
            return lit(r.pick(&special_pool()), dim, unit);
        }
        return lit(leaf_value(r, pool), dim, unit);
    }
    let dn = depth - 1;
    match r.below(14) {
        0 | 1 => {
            let d2 = if r.chance(25) { r.below(QDIMS.len() as u64) as usize } else { d };
            let op = if r.chance(2) { BinaryOperator::Add } else { BinaryOperator::Subtract };
            bin(op, gen_q(r, d, dn, pool), gen_q(r, d2, dn, pool))
        }
        2 => {
            if r.chance(2) {
                bin(BinaryOperator::Multiply, gen_q(r, 6, dn, pool), gen_q(r, d, dn, pool))
            } else {
                bin(BinaryOperator::Multiply, gen_q(r, d, dn, pool), gen_q(r, 6, dn, pool))
            }
        }
        3 => match dim {
            Dimension::Moment => bin(BinaryOperator::Multiply, gen_q(r, 4, dn, pool), gen_q(r, 2, dn, pool)),
            Dimension::Area => bin(BinaryOperator::Multiply, gen_q(r, 2, dn, pool), gen_q(r, 2, dn, pool)),
            Dimension::Force => bin(BinaryOperator::Multiply, gen_q(r, 0, dn, pool), gen_q(r, 3, dn, pool)),
            _ => bin(BinaryOperator::Multiply, gen_q(r, d, dn, pool), gen_q(r, 6, dn, pool)),
        },
        4 | 5 | 6 => {
            if dim == Dimension::Dimensionless {
                // The same-dimension quotient site, over any dimension.
                let x = r.below(QDIMS.len() as u64) as usize;
                let y = if r.chance(15) { if x == 0 { 1 } else { x } } else { x };
                bin(BinaryOperator::Divide, gen_q(r, x, dn, pool), gen_q(r, y, dn, pool))
            } else if dim == Dimension::Force && r.chance(2) {
                bin(BinaryOperator::Divide, gen_q(r, 5, dn, pool), gen_q(r, 2, dn, pool))
            } else {
                bin(BinaryOperator::Divide, gen_q(r, d, dn, pool), gen_q(r, 6, dn, pool))
            }
        }
        7 => Expression::Unary {
            operator: if r.chance(2) { UnaryOperator::Negate } else { UnaryOperator::Abs },
            operand: Box::new(gen_q(r, d, dn, pool)),
        },
        8 => Expression::Aggregate {
            function: if r.chance(2) { AggregateFunction::Min } else { AggregateFunction::Max },
            operands: (0..r.below(4)).map(|_| gen_q(r, d, dn, pool)).collect(),
        },
        9 => Expression::Select {
            condition: Box::new(gen_b(r, dn, pool)),
            then_branch: Box::new(gen_q(r, d, dn, pool)),
            else_branch: Box::new(gen_q(r, d, dn, pool)),
        },
        10 | 11 if dim == Dimension::Stress && unit == "su" => {
            let (arg_d, rows): (usize, Vec<(f64, f64)>) = match r.below(4) {
                0 => (6, vec![(-2.0, 1.0), (0.0, -3.0), (0.5, 4.0), (3.0, 4.5)]),
                1 => (7, vec![(10.0, 1.5), (20.0, 2.5), (40.0, 3.5)]),
                2 => (6, vec![(-MAX, -1e308), (0.0, 0.0), (MAX, 1e308)]),
                _ => (6, vec![(0.0, 0.0), (5e-324, 1.0), (1.0, MAX)]),
            };
            let (ad, au) = QDIMS[arg_d];
            let t = table("gen_table", &rows, ad, au);
            let arg_d = if r.chance(20) { r.below(QDIMS.len() as u64) as usize } else { arg_d };
            table_expr(&t, r.below(3) as usize, gen_q(r, arg_d, dn, pool))
        }
        12 if r.chance(10) => {
            if r.chance(2) {
                Expression::UnsupportedForm { form_id: "power".to_string() }
            } else {
                cmp(ComparisonOperator::Equal, gen_q(r, d, dn, pool), gen_q(r, d, dn, pool))
            }
        }
        _ => gen_q(r, d, dn, pool),
    }
}

fn gen_b(r: &mut Rng, depth: u32, pool: &[f64]) -> Expression {
    let ops = [
        ComparisonOperator::LessThan,
        ComparisonOperator::LessThanOrEqual,
        ComparisonOperator::GreaterThan,
        ComparisonOperator::GreaterThanOrEqual,
        ComparisonOperator::Equal,
        ComparisonOperator::NotEqual,
    ];
    let op = r.pick(&ops);
    if depth == 0 {
        let d = r.below(QDIMS.len() as u64) as usize;
        return cmp(op, gen_q(r, d, 0, pool), gen_q(r, d, 0, pool));
    }
    let dn = depth - 1;
    match r.below(7) {
        0 | 1 | 2 => {
            let d = r.below(QDIMS.len() as u64) as usize;
            let d2 = if r.chance(30) { r.below(QDIMS.len() as u64) as usize } else { d };
            cmp(op, gen_q(r, d, dn, pool), gen_q(r, d2, dn, pool))
        }
        3 => Expression::Logical {
            operator: if r.chance(2) { LogicalOperator::And } else { LogicalOperator::Or },
            left: Box::new(gen_b(r, dn, pool)),
            right: Box::new(gen_b(r, dn, pool)),
        },
        4 => Expression::Unary {
            operator: UnaryOperator::Not,
            operand: Box::new(gen_b(r, dn, pool)),
        },
        5 => Expression::Select {
            condition: Box::new(gen_b(r, dn, pool)),
            then_branch: Box::new(gen_b(r, dn, pool)),
            else_branch: Box::new(gen_b(r, dn, pool)),
        },
        _ => cmp(
            op,
            gen_q(r, 6, dn, pool),
            ratio(r.pick(&[1.0, 0.0, 100.0, 1e308])),
        ),
    }
}

fn gen_bindings(r: &mut Rng, pool: &[f64]) -> Vec<VariableBinding> {
    let mut out = Vec::new();
    for &(id, d) in VARS {
        let (dim, unit) = QDIMS[d];
        if r.chance(25) {
            out.push(VariableBinding::missing(id, BindingSource::UserSuppliedValue));
            continue;
        }
        let v = if r.chance(40) { r.pick(&special_pool()) } else { leaf_value(r, pool) };
        out.push(bind(id, v, dim, unit));
    }
    if r.chance(40) {
        out.push(bind("s1", 1.0, Dimension::Stress, "su"));
    }
    out
}

pub fn generated_inputs(seed: u64, n: u32) -> Vec<(String, EvaluationInput)> {
    let mut r = Rng(seed);
    let pool = finite_pool();
    let status_sets: [Vec<AnalysisStatus>; 4] = [
        vec![AnalysisStatus::MechanicsSolved],
        vec![],
        vec![AnalysisStatus::HumanApprovedForProject, AnalysisStatus::MechanicsSolved],
        vec![AnalysisStatus::ModelIncomplete],
    ];
    let mut all = Vec::new();
    for i in 0..n {
        let e = if r.chance(2) {
            gen_b(&mut r, 4, &pool)
        } else {
            let d = r.below(QDIMS.len() as u64) as usize;
            gen_q(&mut r, d, 4, &pool)
        };
        for j in 0..5 {
            let bindings = gen_bindings(&mut r, &pool);
            let required_variable_ids = if r.chance(6) {
                vec!["s1".to_string(), "z1".to_string()]
            } else {
                vec![]
            };
            let statuses = status_sets[r.below(4) as usize].clone();
            let declared_grammar_version = if r.chance(60) { "2.0.0".to_string() } else { GRAMMAR_VERSION.to_string() };
            all.push((
                format!("g_{i}_{j}"),
                EvaluationInput {
                    expression: e.clone(),
                    bindings,
                    required_variable_ids,
                    statuses,
                    declared_grammar_version,
                },
            ));
        }
    }
    all
}

fn family_generated(out: &mut Out) {
    for (label, inp) in generated_inputs(0x5256_3130_3447_454E, 9000) {
        out.both(&label, &inp);
    }
}

fn decode_status(token: &str) -> AnalysisStatus {
    match token {
        "model_incomplete" => AnalysisStatus::ModelIncomplete,
        "mechanics_solved" => AnalysisStatus::MechanicsSolved,
        "rule_inputs_incomplete" => AnalysisStatus::RuleInputsIncomplete,
        "user_rule_checked" => AnalysisStatus::UserRuleChecked,
        "user_rule_failed" => AnalysisStatus::UserRuleFailed,
        "human_review_required" => AnalysisStatus::HumanReviewRequired,
        "human_approved_for_project" => AnalysisStatus::HumanApprovedForProject,
        other => panic!("unknown status {other}"),
    }
}

fn decode_source(token: &str) -> BindingSource {
    match token {
        "rule_pack_required_input" => BindingSource::RulePackRequiredInput,
        "user_supplied_value" => BindingSource::UserSuppliedValue,
        "solver_result_field" => BindingSource::SolverResultField,
        other => panic!("unknown source {other}"),
    }
}

fn family_corpus(out: &mut Out) {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../fixtures/rule_expressions/conformance_corpus");
    let mut paths: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().map_or(false, |x| x == "json"))
        .collect();
    paths.sort();
    assert_eq!(paths.len(), 69, "corpus case count");
    for path in paths {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let case: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        let expression = match decode_expression(&case["expression"]) {
            Ok(e) => e,
            Err(err) => {
                out.0.push_str(&format!("c_{name}\tP\tDECODE_ERROR {err:?}\n"));
                continue;
            }
        };
        let bindings = case["bindings"]
            .as_array()
            .unwrap()
            .iter()
            .map(|b| {
                let id = b["variable_id"].as_str().unwrap().to_string();
                let source = decode_source(b["source"].as_str().unwrap());
                match &b["quantity"] {
                    Value::Null => VariableBinding { variable_id: id, source, quantity: None },
                    qv => VariableBinding {
                        variable_id: id,
                        source,
                        quantity: Some(Quantity {
                            value: qv["value"].as_f64().unwrap(),
                            dimension: decode_dimension(qv["dimension"].as_str().unwrap()).unwrap(),
                            unit_ref: qv["unit_ref"].as_str().unwrap_or("").to_string(),
                            unit_required: qv["unit_required"].as_bool().unwrap_or(true),
                            dimension_check_required: qv["dimension_check_required"].as_bool().unwrap_or(true),
                        }),
                    },
                }
            })
            .collect();
        let inp = EvaluationInput {
            expression,
            bindings,
            required_variable_ids: case["required_variable_ids"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect(),
            statuses: case["statuses"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| decode_status(v.as_str().unwrap()))
                .collect(),
            declared_grammar_version: case["declared_grammar_version"].as_str().unwrap_or("").to_string(),
        };
        out.both(&format!("c_{name}"), &inp);
    }
}

#[test]
fn rv104_evaluator_dump() {
    std::panic::set_hook(Box::new(|_| {}));
    let mut out = Out(String::new());
    family_corpus(&mut out);
    family_quotients(&mut out);
    family_tables(&mut out);
    family_non_finite_bindings(&mut out);
    family_generated(&mut out);
    let _ = std::panic::take_hook();
    let path = std::env::var("RV104_EE_OUT").expect("RV104_EE_OUT");
    fs::write(path, out.0).unwrap();
}
