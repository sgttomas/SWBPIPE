//! RV111 (T3-SI1c review): base-against-candidate differential harness.
//! Written from scratch by RV111; scratch only, never committed. Public API only, except the
//! instrumented base's `rv111_flags` event log (compiled in with `--cfg rv111_ibase`).
//!
//! Families (evaluator): pc (every D producer x every consumer, depth 1-3), bd (finite and
//! overflowing boundaries per arm), tbl (interpolation steps), gen (random typed formulas over the
//! whole grammar with extreme leaves), corpus (the committed conformance corpus).
//! Families (runner): rpc (pc formulas over a solver input), rn4 (N-4 values: inputs and limits,
//! raw and after normalization), rmulti (three-check packs), rdemo (the two committed packs).
//! Every case runs `evaluate` and `evaluate_interval` (3 overlay variants); every pack runs plain,
//! b = 0, b = 1, b relative, b = NaN and a duplicate bound.

use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;

use open_pipe_stress_expression_evaluator::{
    dimension_product, dimension_quotient, enclosure_from_bound, evaluate, evaluate_interval,
    AggregateFunction, AnalysisStatus, BinaryOperator, BindingSource, ComparisonOperator,
    Dimension, DimensionQuotient, EvaluationInput, EvaluationResult, EvaluationValue, Expression,
    IntervalBinding, LogicalOperator, LookupMode, Quantity, TableRow, UnaryOperator, UserTable,
    VariableBinding,
};
use open_pipe_stress_rule_check_runner::{
    run_rule_checks, run_rule_checks_with_bounds, LibraryValueBinding, RuleCheckRunInput,
    SolverResultBinding, SolverResultBound, SuppliedValueBinding,
};
use open_pipe_stress_rule_pack_document::{decode_dimension, decode_expression, encode_dimension, encode_expression};
use open_pipe_stress_units::{convert_for_dimension, unit_by_symbol, Dimension as UnitDimension};
use serde_json::{json, Value};

const MAX: f64 = f64::MAX;

#[cfg(rv111_ibase)]
fn take_events() -> String {
    open_pipe_stress_expression_evaluator::rv111_flags::take().join(";")
}
#[cfg(not(rv111_ibase))]
fn take_events() -> String {
    String::new()
}

fn out_dir() -> PathBuf {
    PathBuf::from(std::env::var("RV111_OUT").expect("RV111_OUT"))
}

// ----------------------------------------------------------------------------- PRNG

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    fn chance(&mut self, p: f64) -> bool {
        ((self.next() >> 11) as f64) / ((1u64 << 53) as f64) < p
    }
    fn pick<X: Clone>(&mut self, xs: &[X]) -> X {
        xs[self.below(xs.len())].clone()
    }
}

// ----------------------------------------------------------------------------- builders

#[derive(Clone)]
struct T {
    e: Expression,
    d: Dimension,
    u: String,
}

fn u_of(d: Dimension) -> String {
    if d == Dimension::Dimensionless {
        "ratio".to_string()
    } else {
        format!("u_{}", encode_dimension(d))
    }
}
fn qty(v: f64, d: Dimension, u: &str) -> Quantity {
    Quantity {
        value: v,
        dimension: d,
        unit_ref: u.to_string(),
        unit_required: true,
        dimension_check_required: true,
    }
}
fn litu(v: f64, d: Dimension, u: &str) -> T {
    T {
        e: Expression::Literal(qty(v, d, u)),
        d,
        u: u.to_string(),
    }
}
fn lit(v: f64, d: Dimension) -> T {
    litu(v, d, &u_of(d))
}
fn rlit(v: f64) -> T {
    lit(v, Dimension::Dimensionless)
}
fn var(id: &str, d: Dimension, u: &str) -> T {
    T {
        e: Expression::VariableRef(id.to_string()),
        d,
        u: u.to_string(),
    }
}
fn bx(e: Expression) -> Box<Expression> {
    Box::new(e)
}
fn bin(op: BinaryOperator, l: Expression, r: Expression) -> Expression {
    Expression::Binary {
        operator: op,
        left: bx(l),
        right: bx(r),
    }
}
fn cmp(op: ComparisonOperator, l: Expression, r: Expression) -> Expression {
    Expression::Compare {
        operator: op,
        left: bx(l),
        right: bx(r),
    }
}
fn un(op: UnaryOperator, e: Expression) -> Expression {
    Expression::Unary {
        operator: op,
        operand: bx(e),
    }
}
fn logical(op: LogicalOperator, l: Expression, r: Expression) -> Expression {
    Expression::Logical {
        operator: op,
        left: bx(l),
        right: bx(r),
    }
}
fn sel(c: Expression, t: Expression, e: Expression) -> Expression {
    Expression::Select {
        condition: bx(c),
        then_branch: bx(t),
        else_branch: bx(e),
    }
}
fn agg(f: AggregateFunction, ops: Vec<Expression>) -> Expression {
    Expression::Aggregate {
        function: f,
        operands: ops,
    }
}
fn table(id: &str, ad: Dimension, au: &str, rd: Dimension, ru: &str, rows: &[(f64, f64)]) -> UserTable {
    UserTable {
        table_id: id.to_string(),
        argument_dimension: ad,
        argument_unit_ref: au.to_string(),
        result_dimension: rd,
        result_unit_ref: ru.to_string(),
        rows: rows
            .iter()
            .map(|&(argument, result)| TableRow { argument, result })
            .collect(),
    }
}
fn interp(t: UserTable, arg: Expression) -> Expression {
    Expression::Interpolate {
        table: t,
        argument: bx(arg),
    }
}
fn lookup(t: UserTable, mode: LookupMode, arg: Expression) -> Expression {
    Expression::Lookup {
        table: t,
        mode,
        argument: bx(arg),
    }
}
fn truth(v: bool) -> Expression {
    cmp(
        if v {
            ComparisonOperator::LessThan
        } else {
            ComparisonOperator::GreaterThan
        },
        rlit(1.0).e,
        rlit(2.0).e,
    )
}
fn binding(id: &str, v: f64, d: Dimension, u: &str) -> VariableBinding {
    VariableBinding::new(id, BindingSource::RulePackRequiredInput, qty(v, d, u))
}

const CMPS: [ComparisonOperator; 6] = [
    ComparisonOperator::LessThan,
    ComparisonOperator::LessThanOrEqual,
    ComparisonOperator::GreaterThan,
    ComparisonOperator::GreaterThanOrEqual,
    ComparisonOperator::Equal,
    ComparisonOperator::NotEqual,
];
fn cmp_name(op: ComparisonOperator) -> &'static str {
    match op {
        ComparisonOperator::LessThan => "lt",
        ComparisonOperator::LessThanOrEqual => "le",
        ComparisonOperator::GreaterThan => "gt",
        ComparisonOperator::GreaterThanOrEqual => "ge",
        ComparisonOperator::Equal => "eq",
        ComparisonOperator::NotEqual => "ne",
    }
}

// ----------------------------------------------------------------------------- rendering

fn render_value(v: &Option<EvaluationValue>) -> String {
    match v {
        None => "None".to_string(),
        Some(EvaluationValue::Boolean(b)) => format!("B({b})"),
        Some(EvaluationValue::Quantity(q)) => format!(
            "Q({:#018x}|{:?}|{:?}|{}|{}|{})",
            q.value.to_bits(),
            q.value,
            q.dimension,
            q.unit_ref,
            q.unit_required,
            q.dimension_check_required
        ),
    }
}
fn render_point(r: &EvaluationResult) -> String {
    let findings: Vec<String> = r
        .findings
        .iter()
        .map(|f| format!("[{:?}|{}|{}]", f.code, f.subject_id, f.message))
        .collect();
    format!(
        "v={} st={:?} src={:?} f={}",
        render_value(&r.value),
        r.statuses,
        r.source_variable_ids,
        findings.join("")
    )
}

fn clean(s: &str) -> String {
    s.replace(['\t', '\n'], " ")
}

// ----------------------------------------------------------------------------- evaluator cases

struct ECase {
    label: String,
    expr: Expression,
    bindings: Vec<VariableBinding>,
    required: Vec<String>,
    statuses: Vec<AnalysisStatus>,
    grammar: String,
    py: bool,
}

fn case(label: String, expr: Expression, bindings: Vec<VariableBinding>) -> ECase {
    ECase {
        label,
        expr,
        bindings,
        required: Vec::new(),
        statuses: vec![AnalysisStatus::MechanicsSolved],
        grammar: "1.0.0".to_string(),
        py: true,
    }
}

fn binding_json(b: &VariableBinding) -> Value {
    match &b.quantity {
        Some(q) => json!({ "id": b.variable_id, "value": q.value, "bits": format!("{:#018x}", q.value.to_bits()),
                           "dim": encode_dimension(q.dimension), "unit": q.unit_ref }),
        None => json!({ "id": b.variable_id, "missing": true }),
    }
}

fn overlay_sets(c: &ECase) -> Vec<Vec<IntervalBinding>> {
    let mut b0 = Vec::new();
    let mut rel = Vec::new();
    for b in &c.bindings {
        if let Some(q) = &b.quantity {
            b0.push(IntervalBinding {
                variable_id: b.variable_id.clone(),
                enclosure: enclosure_from_bound(q.value, 0.0),
            });
            rel.push(IntervalBinding {
                variable_id: b.variable_id.clone(),
                enclosure: enclosure_from_bound(q.value, (q.value.abs() * 1e-9).max(1e-300)),
            });
        }
    }
    vec![Vec::new(), b0, rel]
}

fn run_case(c: &ECase, point: &mut impl Write, interval: &mut impl Write, cases: &mut impl Write) {
    let input = EvaluationInput {
        expression: c.expr.clone(),
        bindings: c.bindings.clone(),
        required_variable_ids: c.required.clone(),
        statuses: c.statuses.clone(),
        declared_grammar_version: c.grammar.clone(),
    };
    let _ = take_events();
    let rendered = match catch_unwind(AssertUnwindSafe(|| evaluate(&input))) {
        Ok(r) => render_point(&r),
        Err(_) => "PANIC".to_string(),
    };
    let events = take_events();
    writeln!(point, "{}\t{}\t{}", c.label, clean(&rendered), events).unwrap();
    for (i, overlays) in overlay_sets(c).iter().enumerate() {
        let rendered = match catch_unwind(AssertUnwindSafe(|| evaluate_interval(&input, overlays))) {
            Ok(r) => format!("{r:?}"),
            Err(_) => "PANIC".to_string(),
        };
        let _ = take_events();
        writeln!(interval, "{}|I{}\t{}", c.label, i, clean(&rendered)).unwrap();
    }
    let record = json!({
        "label": c.label, "py": c.py, "expr": encode_expression(&c.expr),
        "bindings": c.bindings.iter().map(binding_json).collect::<Vec<_>>(),
        "required": c.required, "grammar": c.grammar,
    });
    writeln!(cases, "{record}").unwrap();
}

// --- pc family: producers x consumers

/// Non-finite (or absorbed) sources from finite operands, with their dimension and unit.
fn pc_sources() -> Vec<(String, T)> {
    use BinaryOperator::*;
    use Dimension as D;
    let s = |v: f64| lit(v, D::Stress);
    let x = || var("x", D::Stress, "u_stress");
    let st = |e: Expression| T {
        e,
        d: D::Stress,
        u: "u_stress".to_string(),
    };
    let mut v: Vec<(String, T)> = Vec::new();
    v.push(("add+".into(), st(bin(Add, s(MAX).e, s(MAX).e))));
    v.push(("add-".into(), st(bin(Add, s(-MAX).e, s(-MAX).e))));
    v.push(("sub+".into(), st(bin(Subtract, s(MAX).e, s(-MAX).e))));
    v.push(("sub-".into(), st(bin(Subtract, s(-MAX).e, s(MAX).e))));
    v.push(("muldl+".into(), st(bin(Multiply, rlit(1e300).e, x().e))));
    v.push(("muldl-".into(), st(bin(Multiply, rlit(-1e300).e, x().e))));
    v.push(("muldr+".into(), st(bin(Multiply, x().e, rlit(1e300).e))));
    v.push(("muldr-".into(), st(bin(Multiply, x().e, rlit(-1e300).e))));
    v.push(("divdd+".into(), st(bin(Divide, x().e, rlit(1e-300).e))));
    v.push(("divdd-".into(), st(bin(Divide, x().e, rlit(-1e-300).e))));
    v.push(("divdd_sub".into(), st(bin(Divide, s(1.0).e, rlit(5e-324).e))));
    let stress_table = |id: &str, rows: &[(f64, f64)]| {
        table(id, D::Dimensionless, "ratio", D::Stress, "u_stress", rows)
    };
    v.push((
        "interp_rise+".into(),
        st(interp(stress_table("t_rise", &[(0.0, -1e308), (1.0, 1e308)]), rlit(0.5).e)),
    ));
    v.push((
        "interp_rise-".into(),
        st(interp(stress_table("t_rise", &[(0.0, 1e308), (1.0, -1e308)]), rlit(0.5).e)),
    ));
    v.push((
        "interp_offset".into(),
        st(interp(stress_table("t_wide", &[(-MAX, 0.0), (MAX, 10.0)]), rlit(1e308).e)),
    ));
    v.push((
        "interp_sum".into(),
        st(interp(
            stress_table("t_sum", &[(-1e20, 3.0 * 2f64.powi(970)), (1.0, MAX)]),
            rlit(0.5).e,
        )),
    ));
    v.push((
        "interp_run_absorbed".into(),
        st(interp(stress_table("t_wide", &[(-MAX, 0.0), (MAX, 10.0)]), rlit(0.0).e)),
    ));
    // NaN from two carried infinities (the first producer flags first).
    let addp = || bin(Add, s(MAX).e, s(MAX).e);
    let mulx = || bin(Multiply, rlit(1e300).e, x().e);
    v.push(("nan_inf-inf".into(), st(bin(Subtract, addp(), addp()))));
    v.push(("nan_inf+-inf".into(), st(bin(Add, mulx(), bin(Multiply, rlit(-1e300).e, x().e)))));
    v.push(("nan_0*inf".into(), st(bin(Multiply, rlit(0.0).e, mulx()))));
    v.push(("nan_inf*0".into(), st(bin(Multiply, mulx(), rlit(0.0).e))));
    v.push((
        "nan_inf/inf".into(),
        st(bin(Divide, mulx(), bin(Multiply, rlit(1e300).e, rlit(1e300).e))),
    ));
    // Derived-unit stress (Moment / SectionModulus).
    v.push((
        "divderived_stress".into(),
        T {
            e: bin(Divide, lit(1e300, D::Moment).e, lit(1e-300, D::SectionModulus).e),
            d: D::Stress,
            u: "u_moment/u_section_modulus".into(),
        },
    ));
    // Ratio sources.
    let rt = |e: Expression| T {
        e,
        d: D::Dimensionless,
        u: "ratio".into(),
    };
    v.push(("r_add+".into(), rt(bin(Add, rlit(MAX).e, rlit(MAX).e))));
    v.push(("r_mul".into(), rt(bin(Multiply, rlit(1e300).e, rlit(1e300).e))));
    v.push(("r_div".into(), rt(bin(Divide, rlit(1e300).e, rlit(-1e-300).e))));
    v.push((
        "r_derived_product".into(),
        rt(bin(
            Multiply,
            lit(1e200, D::ThermalExpansionCoefficient).e,
            lit(1e200, D::TemperatureInterval).e,
        )),
    ));
    v.push(("r_ratio_arm".into(), rt(bin(Divide, s(1e300).e, s(1e-300).e))));
    v.push((
        "r_nan".into(),
        rt(bin(Subtract, bin(Add, rlit(MAX).e, rlit(MAX).e), bin(Add, rlit(MAX).e, rlit(MAX).e))),
    ));
    v.push((
        "r_interp_rise".into(),
        rt(interp(
            table("t_rr", D::Dimensionless, "ratio", D::Dimensionless, "ratio", &[(0.0, -1e308), (1.0, 1e308)]),
            rlit(0.25).e,
        )),
    ));
    // Length sources.
    let lt = |e: Expression| T {
        e,
        d: D::Length,
        u: "u_length".into(),
    };
    v.push(("l_add+".into(), lt(bin(Add, lit(MAX, D::Length).e, lit(1e300, D::Length).e))));
    v.push(("l_muldl".into(), lt(bin(Multiply, rlit(1e10).e, lit(1e300, D::Length).e))));
    v.push(("l_divdd".into(), lt(bin(Divide, lit(-1e300, D::Length).e, rlit(1e-10).e))));
    v.push((
        "l_divderived".into(),
        T {
            e: bin(Divide, lit(1e300, D::Moment).e, lit(1e-300, D::Force).e),
            d: D::Length,
            u: "u_moment/u_force".into(),
        },
    ));
    // Moment and force (derived arms).
    v.push((
        "m_mulderived".into(),
        T {
            e: bin(Multiply, lit(1e200, D::Force).e, lit(-1e200, D::Length).e),
            d: D::Moment,
            u: "u_force*u_length".into(),
        },
    ));
    v.push((
        "f_divderived".into(),
        T {
            e: bin(Divide, lit(1e300, D::Moment).e, lit(1e-300, D::Length).e),
            d: D::Force,
            u: "u_moment/u_length".into(),
        },
    ));
    v
}

type Wrap = (&'static str, fn(&T) -> T);

fn partner(t: &T, v: f64) -> T {
    litu(v, t.d, &t.u)
}
fn w_neg(t: &T) -> T {
    T { e: un(UnaryOperator::Negate, t.e.clone()), ..t.clone() }
}
fn w_abs(t: &T) -> T {
    T { e: un(UnaryOperator::Abs, t.e.clone()), ..t.clone() }
}
fn w_addp(t: &T) -> T {
    T { e: bin(BinaryOperator::Add, t.e.clone(), partner(t, 2.0).e), ..t.clone() }
}
fn w_mulr(t: &T) -> T {
    T { e: bin(BinaryOperator::Multiply, rlit(2.0).e, t.e.clone()), ..t.clone() }
}
fn w_divr(t: &T) -> T {
    T { e: bin(BinaryOperator::Divide, t.e.clone(), rlit(2.0).e), ..t.clone() }
}
fn w_sel_taken(t: &T) -> T {
    T { e: sel(truth(true), t.e.clone(), partner(t, 2.0).e), ..t.clone() }
}
fn w_sel_else_taken(t: &T) -> T {
    T { e: sel(truth(false), partner(t, 2.0).e, t.e.clone()), ..t.clone() }
}
fn w_sel_untaken(t: &T) -> T {
    T { e: sel(truth(true), partner(t, 2.0).e, t.e.clone()), ..t.clone() }
}
fn w_min(t: &T) -> T {
    T { e: agg(AggregateFunction::Min, vec![t.e.clone(), partner(t, 2.0).e]), ..t.clone() }
}
fn w_max(t: &T) -> T {
    T { e: agg(AggregateFunction::Max, vec![partner(t, 2.0).e, t.e.clone()]), ..t.clone() }
}

fn wrappers() -> Vec<Wrap> {
    vec![
        ("neg", w_neg as fn(&T) -> T),
        ("abs", w_abs as fn(&T) -> T),
        ("addp", w_addp as fn(&T) -> T),
        ("mulr", w_mulr as fn(&T) -> T),
        ("divr", w_divr as fn(&T) -> T),
        ("selT", w_sel_taken as fn(&T) -> T),
        ("selF", w_sel_else_taken as fn(&T) -> T),
        ("selU", w_sel_untaken as fn(&T) -> T),
        ("min", w_min as fn(&T) -> T),
        ("max", w_max as fn(&T) -> T),
    ]
}
fn depth3_wrappers() -> Vec<Wrap> {
    vec![
        ("neg", w_neg as fn(&T) -> T),
        ("addp", w_addp as fn(&T) -> T),
        ("mulr", w_mulr as fn(&T) -> T),
        ("selU", w_sel_untaken as fn(&T) -> T),
        ("min", w_min as fn(&T) -> T),
        ("max", w_max as fn(&T) -> T),
    ]
}

/// A derived-arm numerator dimension n with n / d unique, for a divisor of dimension d.
fn derived_numerator(d: Dimension) -> Option<Dimension> {
    match d {
        Dimension::Length => Some(Dimension::Moment),
        Dimension::Stress => Some(Dimension::Force),
        Dimension::Force => Some(Dimension::Moment),
        Dimension::Dimensionless => None,
        _ => None,
    }
}
/// A derived-arm denominator dimension e with d / e unique, for a numerator of dimension d.
fn derived_denominator(d: Dimension) -> Option<Dimension> {
    match d {
        Dimension::Moment => Some(Dimension::Length),
        Dimension::Force => Some(Dimension::Stress),
        Dimension::Length => Some(Dimension::Time),
        Dimension::Dimensionless => Some(Dimension::ThermalExpansionCoefficient),
        _ => None,
    }
}

/// Every consumer of a (non-finite) quantity `n`; `p` is a finite partner of the same unit.
fn pc_consumers(n: &T, p: &T) -> Vec<(String, Expression)> {
    use BinaryOperator::*;
    let mut v: Vec<(String, Expression)> = Vec::new();
    for op in CMPS {
        v.push((format!("cmp_{}_L", cmp_name(op)), cmp(op, n.e.clone(), p.e.clone())));
        v.push((format!("cmp_{}_R", cmp_name(op)), cmp(op, p.e.clone(), n.e.clone())));
        v.push((format!("cmp_{}_NN", cmp_name(op)), cmp(op, n.e.clone(), n.e.clone())));
    }
    let c = || cmp(ComparisonOperator::GreaterThan, n.e.clone(), p.e.clone());
    v.push(("not".into(), un(UnaryOperator::Not, c())));
    v.push(("and_L".into(), logical(LogicalOperator::And, c(), truth(true))));
    v.push(("and_R".into(), logical(LogicalOperator::And, truth(true), c())));
    v.push(("or_L".into(), logical(LogicalOperator::Or, c(), truth(false))));
    v.push(("or_R".into(), logical(LogicalOperator::Or, truth(false), c())));
    v.push(("bsel_cond".into(), sel(c(), truth(true), truth(false))));
    v.push(("bsel_taken".into(), sel(truth(true), c(), truth(false))));
    v.push(("bsel_untaken".into(), sel(truth(true), truth(false), c())));
    v.push(("final".into(), n.e.clone()));
    v.push(("qsel_taken".into(), sel(truth(true), n.e.clone(), p.e.clone())));
    v.push(("qsel_untaken".into(), sel(truth(false), n.e.clone(), p.e.clone())));
    v.push(("qsel_untaken_else".into(), sel(truth(true), p.e.clone(), n.e.clone())));
    v.push(("min_L".into(), agg(AggregateFunction::Min, vec![n.e.clone(), p.e.clone()])));
    v.push(("min_R".into(), agg(AggregateFunction::Min, vec![p.e.clone(), n.e.clone()])));
    v.push(("max_L".into(), agg(AggregateFunction::Max, vec![n.e.clone(), p.e.clone()])));
    v.push(("max_R".into(), agg(AggregateFunction::Max, vec![p.e.clone(), n.e.clone()])));
    v.push(("max_NN".into(), agg(AggregateFunction::Max, vec![n.e.clone(), n.e.clone()])));
    v.push((
        "max_cmp_le".into(),
        cmp(
            ComparisonOperator::LessThanOrEqual,
            agg(AggregateFunction::Max, vec![n.e.clone(), p.e.clone()]),
            partner(n, 100.0).e,
        ),
    ));
    v.push((
        "min_cmp_le".into(),
        cmp(
            ComparisonOperator::LessThanOrEqual,
            agg(AggregateFunction::Min, vec![n.e.clone(), p.e.clone()]),
            partner(n, 100.0).e,
        ),
    ));
    v.push(("div_by_ratio_arm".into(), bin(Divide, p.e.clone(), n.e.clone())));
    v.push(("num_ratio_arm".into(), bin(Divide, n.e.clone(), p.e.clone())));
    v.push(("num_dd".into(), bin(Divide, n.e.clone(), rlit(3.0).e)));
    if n.d == Dimension::Dimensionless {
        v.push(("div_by_dd".into(), bin(Divide, lit(2.0, Dimension::Stress).e, n.e.clone())));
    }
    if let Some(nd) = derived_numerator(n.d) {
        v.push(("div_by_derived".into(), bin(Divide, lit(2.0, nd).e, n.e.clone())));
    }
    if let Some(dd) = derived_denominator(n.d) {
        v.push(("num_derived".into(), bin(Divide, n.e.clone(), lit(2.0, dd).e)));
    }
    v.push((
        "div_by_cmp".into(),
        cmp(
            ComparisonOperator::LessThanOrEqual,
            bin(Divide, p.e.clone(), n.e.clone()),
            rlit(1.0).e,
        ),
    ));
    v.push(("add_operand".into(), bin(Add, p.e.clone(), n.e.clone())));
    v.push(("mul_operand".into(), bin(Multiply, n.e.clone(), rlit(0.0).e)));
    for (name, rows) in [
        ("regular", vec![(-10.0, 1.0), (0.0, 2.0), (10.0, 3.0)]),
        ("wide", vec![(-MAX, 1.0), (MAX, 2.0)]),
    ] {
        let t = table(&format!("arg_{name}"), n.d, &n.u, Dimension::Stress, "u_stress", &rows);
        v.push((format!("tbl_{name}_interp"), interp(t.clone(), n.e.clone())));
        v.push((format!("tbl_{name}_step"), lookup(t.clone(), LookupMode::Step, n.e.clone())));
        v.push((format!("tbl_{name}_exact"), lookup(t, LookupMode::Exact, n.e.clone())));
    }
    v
}

fn pc_bindings() -> Vec<VariableBinding> {
    vec![binding("x", 1e300, Dimension::Stress, "u_stress")]
}

fn pc_cases() -> Vec<ECase> {
    let mut out = Vec::new();
    let wraps = wrappers();
    let d3 = depth3_wrappers();
    for (sname, src) in pc_sources() {
        let mut chains: Vec<(String, T)> = vec![("d1".into(), src.clone())];
        for (wn, wf) in &wraps {
            chains.push((format!("d2.{wn}"), wf(&src)));
        }
        for (an, af) in &d3 {
            for (bn, bf) in &d3 {
                chains.push((format!("d3.{an}.{bn}"), bf(&af(&src))));
            }
        }
        for (cn, chain) in chains {
            let p = partner(&chain, 3.0);
            for (kn, e) in pc_consumers(&chain, &p) {
                out.push(case(format!("pc|{sname}|{cn}|{kn}"), e, pc_bindings()));
            }
        }
    }
    out
}

// --- bd family: finite boundaries and overflows per arm

fn boundary_values() -> Vec<f64> {
    let base = [
        0.0,
        5e-324,
        f64::MIN_POSITIVE,
        1e-308,
        1e-300,
        0.5,
        1.0,
        2.0,
        1e154,
        1.4e154,
        1e300,
        1e308,
        MAX / 2.0,
        2f64.powi(970),
        2f64.powi(970).next_down(),
        2f64.powi(970).next_up(),
        MAX.next_down(),
        MAX,
    ];
    let mut v = Vec::new();
    for b in base {
        v.push(b);
        v.push(-b);
    }
    v
}

fn bd_cases() -> Vec<ECase> {
    use BinaryOperator::*;
    use Dimension as D;
    let vals = boundary_values();
    let mut out = Vec::new();
    for (i, &a) in vals.iter().enumerate() {
        for (j, &b) in vals.iter().enumerate() {
            let arms: Vec<(&str, Expression)> = vec![
                ("add", bin(Add, lit(a, D::Stress).e, lit(b, D::Stress).e)),
                ("sub", bin(Subtract, lit(a, D::Stress).e, lit(b, D::Stress).e)),
                ("muldl", bin(Multiply, rlit(a).e, lit(b, D::Stress).e)),
                ("muldr", bin(Multiply, lit(a, D::Stress).e, rlit(b).e)),
                ("mulderived", bin(Multiply, lit(a, D::Force).e, lit(b, D::Length).e)),
                ("divdd", bin(Divide, lit(a, D::Stress).e, rlit(b).e)),
                ("divrr", bin(Divide, rlit(a).e, rlit(b).e)),
                ("divderived", bin(Divide, lit(a, D::Moment).e, lit(b, D::Length).e)),
                ("divratio", bin(Divide, lit(a, D::Stress).e, lit(b, D::Stress).e)),
            ];
            for (arm, e) in arms {
                out.push(case(format!("bd|{arm}|{i}|{j}"), e, vec![]));
            }
        }
    }
    out
}

// --- tbl family: the six interpolation steps

fn tbl_cases() -> Vec<ECase> {
    use Dimension as D;
    let args = [-MAX, -1e308, -1e300, -1.0, 0.0, 1.0, 1e300, 1e308, MAX];
    let results = [-MAX, -1e308, -3.0 * 2f64.powi(970), 0.0, 1.0, 3.0 * 2f64.powi(970), 1e308, MAX];
    let mut out = Vec::new();
    for (i, &a0) in args.iter().enumerate() {
        for (j, &a1) in args.iter().enumerate() {
            if !(a0 < a1) {
                continue;
            }
            let mut xs = vec![a0.next_up(), a1.next_down(), a0 / 2.0 + a1 / 2.0];
            let mid = a0 + (a1 - a0) / 2.0;
            if mid.is_finite() {
                xs.push(mid);
            }
            if a0 < 0.0 && 0.0 < a1 {
                xs.push(0.0);
            }
            for (k, &r0) in results.iter().enumerate() {
                for (l, &r1) in results.iter().enumerate() {
                    let t = table("tsteps", D::Dimensionless, "ratio", D::Stress, "u_stress", &[(a0, r0), (a1, r1)]);
                    for (m, &x) in xs.iter().enumerate() {
                        if !(a0 < x && x < a1) {
                            continue;
                        }
                        out.push(case(format!("tbl|{i}.{j}|{k}.{l}|{m}"), interp(t.clone(), rlit(x).e), vec![]));
                    }
                }
            }
        }
    }
    // A three-row table: the bracketing pair is the second.
    for (n, &x) in [0.5, 1.5, 1e308, 2.0f64.next_up()].iter().enumerate() {
        let t = table("t3", D::Dimensionless, "ratio", D::Stress, "u_stress", &[(-MAX, 0.0), (1.0, -1e308), (MAX, 1e308)]);
        out.push(case(format!("tbl|three|{n}"), interp(t, rlit(x).e), vec![]));
    }
    out
}

// --- gen family: random typed formulas

const GEN_DIMS: [Dimension; 10] = [
    Dimension::Dimensionless,
    Dimension::Stress,
    Dimension::Length,
    Dimension::Force,
    Dimension::Moment,
    Dimension::Area,
    Dimension::Pressure,
    Dimension::SectionModulus,
    Dimension::ThermalExpansionCoefficient,
    Dimension::TemperatureInterval,
];

fn gen_pool() -> Vec<f64> {
    vec![
        0.0,
        5e-324,
        f64::MIN_POSITIVE,
        1e-300,
        1e-200,
        1e-154,
        1e-10,
        0.5,
        1.0,
        2.0,
        3.0,
        10.0,
        100.0,
        1e10,
        1e154,
        1.4e154,
        1e200,
        1e300,
        1e307,
        1e308,
        MAX / 2.0,
        MAX,
        2f64.powi(970),
    ]
}

struct Gen<'a> {
    rng: &'a mut Rng,
    pool: Vec<f64>,
}

impl Gen<'_> {
    fn value(&mut self) -> f64 {
        let v = self.rng.pick(&self.pool);
        if self.rng.chance(0.5) {
            -v
        } else {
            v
        }
    }
    fn leaf(&mut self, d: Dimension, u: &str) -> T {
        if self.rng.chance(0.45) {
            let i = self.rng.below(2);
            if self.rng.chance(0.02) {
                return var("g_missing", d, u);
            }
            if u == u_of(d) {
                return var(&format!("g_{}_{}", encode_dimension(d), i), d, u);
            }
        }
        let v = self.value();
        litu(v, d, u)
    }
    fn table_for(&mut self, arg: &T) -> (UserTable, Dimension, String) {
        let rd = self.rng.pick(&GEN_DIMS);
        let ru = u_of(rd);
        let templates: [(&str, Vec<(f64, f64)>); 6] = [
            ("regular", vec![(-10.0, 1.0), (0.0, 2.0), (10.0, 3.0)]),
            ("wide", vec![(-MAX, 0.0), (MAX, 10.0)]),
            ("rise", vec![(0.0, -1e308), (1.0, 1e308)]),
            ("sumtie", vec![(-1e20, 3.0 * 2f64.powi(970)), (1.0, MAX)]),
            ("fine", vec![(0.0, 0.0), (5e-324, 1e308), (1.0, -1e308)]),
            ("bigargs", vec![(-1e308, 1.0), (1e308, 2.0)]),
        ];
        let (name, rows) = templates[self.rng.below(templates.len())].clone();
        (table(&format!("t_{name}"), arg.d, &arg.u, rd, &ru, &rows), rd, ru)
    }
    fn q(&mut self, depth: usize) -> T {
        if depth == 0 || self.rng.chance(0.25) {
            let d = self.rng.pick(&GEN_DIMS);
            return self.leaf(d, &u_of(d));
        }
        match self.rng.below(9) {
            0 => {
                let a = self.q(depth - 1);
                let op = if self.rng.chance(0.5) { UnaryOperator::Negate } else { UnaryOperator::Abs };
                T { e: un(op, a.e.clone()), ..a }
            }
            1 => {
                let a = self.q(depth - 1);
                let b = self.matching(depth - 1, a.d, &a.u);
                let op = if self.rng.chance(0.5) { BinaryOperator::Add } else { BinaryOperator::Subtract };
                T { e: bin(op, a.e.clone(), b.e), ..a }
            }
            2 | 3 => {
                let a = self.q(depth - 1);
                let b = if self.rng.chance(0.5) { self.q(depth - 1) } else { self.matching(depth - 1, Dimension::Dimensionless, "ratio") };
                let (d, u) = if a.d == Dimension::Dimensionless {
                    (b.d, b.u.clone())
                } else if b.d == Dimension::Dimensionless {
                    (a.d, a.u.clone())
                } else {
                    match dimension_product(a.d, b.d) {
                        Some(p) if p == Dimension::Dimensionless => (p, "ratio".to_string()),
                        Some(p) => {
                            let (x, y) = (a.u.as_str(), b.u.as_str());
                            (p, if x <= y { format!("{x}*{y}") } else { format!("{y}*{x}") })
                        }
                        None => (Dimension::Tbd, "invalid".to_string()),
                    }
                };
                T { e: bin(BinaryOperator::Multiply, a.e, b.e), d, u }
            }
            4 | 5 => {
                let a = self.q(depth - 1);
                let b = match self.rng.below(3) {
                    0 => self.matching(depth - 1, a.d, &a.u),
                    1 => self.matching(depth - 1, Dimension::Dimensionless, "ratio"),
                    _ => self.q(depth - 1),
                };
                let (d, u) = if b.d == Dimension::Dimensionless {
                    (a.d, a.u.clone())
                } else if a.d == b.d {
                    (Dimension::Dimensionless, "ratio".to_string())
                } else {
                    match dimension_quotient(a.d, b.d) {
                        DimensionQuotient::Unique(q) if q == Dimension::Dimensionless => (q, "ratio".to_string()),
                        DimensionQuotient::Unique(q) => (q, format!("{}/{}", a.u, b.u)),
                        _ => (Dimension::Tbd, "invalid".to_string()),
                    }
                };
                T { e: bin(BinaryOperator::Divide, a.e, b.e), d, u }
            }
            6 => {
                let c = self.b(depth - 1);
                let t = self.q(depth - 1);
                let e = self.matching(depth - 1, t.d, &t.u);
                let (t, e) = if self.rng.chance(0.5) { (t, e) } else { (e, t) };
                T { e: sel(c, t.e.clone(), e.e), ..t }
            }
            7 => {
                let first = self.q(depth - 1);
                let mut ops = vec![first.e.clone()];
                for _ in 0..self.rng.below(3) {
                    ops.push(self.matching(depth - 1, first.d, &first.u).e);
                }
                let f = if self.rng.chance(0.5) { AggregateFunction::Min } else { AggregateFunction::Max };
                T { e: agg(f, ops), ..first }
            }
            _ => {
                let a = self.q(depth - 1);
                let (t, rd, ru) = self.table_for(&a);
                let e = match self.rng.below(3) {
                    0 => interp(t, a.e),
                    1 => lookup(t, LookupMode::Step, a.e),
                    _ => lookup(t, LookupMode::Exact, a.e),
                };
                T { e, d: rd, u: ru }
            }
        }
    }
    fn matching(&mut self, depth: usize, d: Dimension, u: &str) -> T {
        if self.rng.chance(0.04) {
            return self.q(depth);
        }
        if depth == 0 || self.rng.chance(0.4) {
            return self.leaf(d, u);
        }
        match self.rng.below(6) {
            0 => {
                let a = self.matching(depth - 1, d, u);
                T { e: un(UnaryOperator::Negate, a.e.clone()), ..a }
            }
            1 => {
                let a = self.matching(depth - 1, d, u);
                let b = self.matching(depth - 1, d, u);
                let op = if self.rng.chance(0.5) { BinaryOperator::Add } else { BinaryOperator::Subtract };
                T { e: bin(op, a.e.clone(), b.e), ..a }
            }
            2 => {
                let a = self.matching(depth - 1, d, u);
                let r = self.matching(depth - 1, Dimension::Dimensionless, "ratio");
                let e = if self.rng.chance(0.5) {
                    bin(BinaryOperator::Multiply, r.e, a.e.clone())
                } else {
                    bin(BinaryOperator::Divide, a.e.clone(), r.e)
                };
                T { e, ..a }
            }
            3 => {
                let c = self.b(depth - 1);
                let t = self.matching(depth - 1, d, u);
                let e = self.matching(depth - 1, d, u);
                T { e: sel(c, t.e.clone(), e.e), ..t }
            }
            4 => {
                let a = self.matching(depth - 1, d, u);
                let b = self.matching(depth - 1, d, u);
                let f = if self.rng.chance(0.5) { AggregateFunction::Min } else { AggregateFunction::Max };
                T { e: agg(f, vec![a.e.clone(), b.e]), ..a }
            }
            _ => {
                let a = self.q(depth - 1);
                let t = table("t_m", a.d, &a.u.clone(), d, u, &[(-1e300, -1e308), (0.0, 1.0), (1e300, 1e308)]);
                let e = if self.rng.chance(0.5) { interp(t, a.e) } else { lookup(t, LookupMode::Step, a.e) };
                T { e, d, u: u.to_string() }
            }
        }
    }
    fn b(&mut self, depth: usize) -> Expression {
        if depth == 0 || self.rng.chance(0.5) {
            let a = self.q(depth.saturating_sub(1));
            let b = self.matching(depth.saturating_sub(1), a.d, &a.u);
            let op = self.rng.pick(&CMPS);
            return cmp(op, a.e, b.e);
        }
        match self.rng.below(3) {
            0 => un(UnaryOperator::Not, self.b(depth - 1)),
            1 => {
                let op = if self.rng.chance(0.5) { LogicalOperator::And } else { LogicalOperator::Or };
                let l = self.b(depth - 1);
                let r = self.b(depth - 1);
                logical(op, l, r)
            }
            _ => {
                let c = self.b(depth - 1);
                let t = self.b(depth - 1);
                let e = self.b(depth - 1);
                sel(c, t, e)
            }
        }
    }
}

fn gen_bindings(rng: &mut Rng, pool: &[f64]) -> Vec<VariableBinding> {
    let mut v = Vec::new();
    for d in GEN_DIMS {
        for i in 0..2 {
            let mut x = pool[rng.below(pool.len())];
            if rng.chance(0.5) {
                x = -x;
            }
            v.push(binding(&format!("g_{}_{}", encode_dimension(d), i), x, d, &u_of(d)));
        }
    }
    v
}

fn gen_cases(n: usize, seed: u64) -> Vec<ECase> {
    let mut rng = Rng(seed);
    let pool = gen_pool();
    let mut out = Vec::new();
    for k in 0..n {
        let depth = 1 + rng.below(4);
        let boolean = rng.chance(0.5);
        let expr = {
            let mut g = Gen { rng: &mut rng, pool: pool.clone() };
            if boolean { g.b(depth) } else { g.q(depth).e }
        };
        for s in 0..2 {
            let bindings = gen_bindings(&mut rng, &pool);
            out.push(case(format!("gen|{k}|{s}"), expr.clone(), bindings));
        }
    }
    out
}

// --- the committed conformance corpus

fn corpus_cases() -> Vec<ECase> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/rule_expressions/conformance_corpus");
    let mut files: Vec<PathBuf> = fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()).filter(|p| p.extension().is_some_and(|x| x == "json")).collect();
    files.sort();
    let status = |s: &str| match s {
        "model_incomplete" => AnalysisStatus::ModelIncomplete,
        "mechanics_solved" => AnalysisStatus::MechanicsSolved,
        "rule_inputs_incomplete" => AnalysisStatus::RuleInputsIncomplete,
        "user_rule_checked" => AnalysisStatus::UserRuleChecked,
        "user_rule_failed" => AnalysisStatus::UserRuleFailed,
        "human_review_required" => AnalysisStatus::HumanReviewRequired,
        "human_approved_for_project" => AnalysisStatus::HumanApprovedForProject,
        other => panic!("status {other}"),
    };
    let mut out = Vec::new();
    for f in files {
        let doc: Value = serde_json::from_str(&fs::read_to_string(&f).unwrap()).unwrap();
        let Ok(expr) = decode_expression(&doc["expression"]) else { continue };
        let bindings = doc["bindings"].as_array().unwrap().iter().map(|b| {
            let source = match b["source"].as_str().unwrap() {
                "user_supplied_value" => BindingSource::UserSuppliedValue,
                "solver_result_field" => BindingSource::SolverResultField,
                _ => BindingSource::RulePackRequiredInput,
            };
            let id = b["variable_id"].as_str().unwrap();
            match b.get("quantity").filter(|q| !q.is_null()) {
                Some(q) => VariableBinding::new(id, source, Quantity {
                    value: q["value"].as_f64().unwrap(),
                    dimension: decode_dimension(q["dimension"].as_str().unwrap()).unwrap_or(Dimension::Tbd),
                    unit_ref: q["unit_ref"].as_str().unwrap_or("").to_string(),
                    unit_required: true,
                    dimension_check_required: true,
                }),
                None => VariableBinding::missing(id, source),
            }
        }).collect();
        let mut c = case(format!("corpus|{}", doc["case_id"].as_str().unwrap()), expr, bindings);
        c.required = doc["required_variable_ids"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
        c.statuses = doc["statuses"].as_array().unwrap().iter().map(|v| status(v.as_str().unwrap())).collect();
        c.grammar = doc["declared_grammar_version"].as_str().unwrap().to_string();
        c.py = false;
        out.push(c);
    }
    out
}

#[test]
fn rv111_evaluator() {
    let dir = out_dir();
    let mut point = BufWriter::new(File::create(dir.join("ee_point.tsv")).unwrap());
    let mut interval = BufWriter::new(File::create(dir.join("ee_interval.tsv")).unwrap());
    let mut cases = BufWriter::new(File::create(dir.join("ee_cases.jsonl")).unwrap());
    let mut counts = Vec::new();
    for (family, list) in [
        ("pc", pc_cases()),
        ("bd", bd_cases()),
        ("tbl", tbl_cases()),
        ("gen", gen_cases(20_000, 0x5111C_0001)),
        ("corpus", corpus_cases()),
    ] {
        counts.push(format!("{family}={}", list.len()));
        for c in &list {
            run_case(c, &mut point, &mut interval, &mut cases);
        }
    }
    eprintln!("rv111 evaluator cases: {}", counts.join(" "));
}

// ----------------------------------------------------------------------------- runner

fn required_input(id: &str, source: &str, unit: &str) -> Value {
    json!({ "input_id": id, "name": id, "source_kind": source, "required_for": "rule_check",
            "provenance_required": true, "redistribution_status_required": true,
            "quantity_intent": { "dimension": "stress", "unit_ref": unit,
                                 "unit_required": true, "dimension_check_required": true } })
}
fn reference(id: &str) -> Value {
    json!({ "ref_id": id, "ref_type": "required_input" })
}

#[derive(Clone)]
struct PackCheck {
    id: String,
    formula: Value,
    formula_inputs: Vec<&'static str>,
    check_inputs: Vec<&'static str>,
    quantity: bool,
    statuses: Vec<&'static str>,
    policy: bool,
    relation: Option<&'static str>,
}

fn pack_doc(checks: &[PackCheck]) -> Value {
    let mut formulas = Vec::new();
    let mut defs = Vec::new();
    for c in checks {
        formulas.push(json!({ "formula_id": format!("f_{}", c.id),
            "declaration_payload": { "expression_ast": c.formula },
            "input_refs": c.formula_inputs.iter().map(|i| reference(i)).collect::<Vec<_>>() }));
        let mut def = json!({ "check_id": c.id,
            "required_input_refs": c.check_inputs.iter().map(|i| reference(i)).collect::<Vec<_>>(),
            "formula_ref": { "ref_id": format!("f_{}", c.id), "ref_type": "formula" },
            "result_statuses": c.statuses });
        if c.policy {
            def["diagnostic_policy"] = json!({ "missing_input": "RULE_INPUT_MISSING", "evaluator_error": "RULE_EVALUATOR_ERROR" });
        }
        if c.quantity {
            def["value_slot_refs"] = json!([{ "ref_id": "limit", "ref_type": "value_slot" }]);
        }
        if let Some(r) = c.relation {
            def["acceptability_relation"] = json!(r);
        }
        defs.push(def);
    }
    json!({
        "grammar_version": "1.0.0",
        "metadata": { "rule_pack_id": "rv111_pack" },
        "required_inputs": [ required_input("x", "solver_result", "Pa"), required_input("s", "user_supplied_rule_value", "Pa"),
                             required_input("lib", "private_library_value", "Pa"), required_input("u", "user_supplied_rule_value", "Pa") ],
        "formula_declarations": formulas,
        "value_slots": [{ "slot_id": "limit", "slot_kind": "limit",
            "quantity_intent": { "dimension": "stress", "unit_ref": "Pa", "unit_required": true, "dimension_check_required": true } }],
        "check_definitions": defs
    })
}

#[derive(Clone)]
struct Vals {
    x: (f64, &'static str),
    s: (f64, &'static str),
    lib: (f64, &'static str),
    u: (f64, &'static str),
    limit: (f64, &'static str),
}
impl Default for Vals {
    fn default() -> Self {
        Vals { x: (1.0, "Pa"), s: (3.0, "Pa"), lib: (2.0, "Pa"), u: (4.0, "Pa"), limit: (100.0, "Pa") }
    }
}

fn normalized_non_finite(v: (f64, &str), declared: &str, dim: &str) -> bool {
    if !v.0.is_finite() {
        return true;
    }
    if v.1.trim() == declared.trim() {
        return false;
    }
    let Ok(d) = UnitDimension::from_schema_value(dim) else { return false };
    let (Ok(from), Ok(to)) = (unit_by_symbol(v.1.trim(), d), unit_by_symbol(declared.trim(), d)) else { return false };
    match convert_for_dimension(v.0, d, from, to) {
        Ok(n) => !n.is_finite(),
        Err(_) => false,
    }
}

fn vals_n4(v: &Vals, declared: &str) -> String {
    let mut n = Vec::new();
    for (name, val) in [("x", v.x), ("s", v.s), ("lib", v.lib), ("u", v.u), ("limit", v.limit)] {
        if normalized_non_finite(val, declared, "stress") {
            n.push(if val.0.is_finite() { format!("{name}:norm") } else if val.1.trim() == declared { format!("{name}:raw") } else { format!("{name}:rawconv") });
        }
    }
    n.join(",")
}

const MODES: [&str; 6] = ["plain", "b0", "b1", "brel", "bnan", "bdup"];

fn run_pack(doc: &Value, v: &Vals, mode: &str) -> String {
    let input = RuleCheckRunInput {
        rule_pack_document: doc,
        solver_results: vec![SolverResultBinding { input_id: "x".into(), result_id: "r_x".into(), value: v.x.0, unit: v.x.1.into() }],
        refused_solver_results: Vec::new(),
        supplied_values: vec![
            SuppliedValueBinding { ref_id: "s".into(), value: v.s.0, unit: v.s.1.into(), dimension: "stress".into() },
            SuppliedValueBinding { ref_id: "u".into(), value: v.u.0, unit: v.u.1.into(), dimension: "stress".into() },
            SuppliedValueBinding { ref_id: "limit".into(), value: v.limit.0, unit: v.limit.1.into(), dimension: "stress".into() },
        ],
        library_values: vec![LibraryValueBinding { input_id: "lib".into(), value: v.lib.0, unit: v.lib.1.into(),
            library_kind: "material".into(), library_id: "L1".into(), record_id: "R1".into(), slot_id: "S1".into() }],
        current_statuses: vec![AnalysisStatus::MechanicsSolved],
    };
    let bound = |b: f64| SolverResultBound { input_id: "x".into(), absolute_bound: b };
    let result = catch_unwind(AssertUnwindSafe(|| match mode {
        "plain" => run_rule_checks(&input),
        "b0" => run_rule_checks_with_bounds(&input, &[bound(0.0)]),
        "b1" => run_rule_checks_with_bounds(&input, &[bound(1.0)]),
        "brel" => run_rule_checks_with_bounds(&input, &[bound(if v.x.0.is_finite() { (v.x.0.abs() * 1e-6).max(1e-300) } else { 1.0 })]),
        "bnan" => run_rule_checks_with_bounds(&input, &[bound(f64::NAN)]),
        "bdup" => run_rule_checks_with_bounds(&input, &[bound(1.0), bound(1.0)]),
        other => panic!("{other}"),
    }));
    match result {
        Ok(r) => serde_json::to_string(&r).unwrap(),
        Err(_) => "PANIC".to_string(),
    }
}

struct RunOut<W: Write> {
    w: W,
    meta: BufWriter<File>,
}
impl<W: Write> RunOut<W> {
    fn emit(&mut self, label: &str, doc: &Value, v: &Vals, record_meta: bool) {
        for mode in MODES {
            let _ = take_events();
            let line = run_pack(doc, v, mode);
            let events = take_events();
            writeln!(self.w, "{label}|{mode}\t{line}\t{events}").unwrap();
        }
        if record_meta {
            let meta = json!({ "label": label, "n4": vals_n4(v, "Pa"), "doc": doc,
                "vals": { "x": [v.x.0.to_string(), v.x.1], "s": [v.s.0.to_string(), v.s.1], "lib": [v.lib.0.to_string(), v.lib.1],
                          "u": [v.u.0.to_string(), v.u.1], "limit": [v.limit.0.to_string(), v.limit.1] } });
            writeln!(self.meta, "{meta}").unwrap();
        }
    }
}

/// Runner pc formulas: sources over the solver input x (stress, Pa), partner s.
fn rpc_sources() -> Vec<(String, T)> {
    use BinaryOperator::*;
    let x = || var("x", Dimension::Stress, "Pa");
    let st = |e: Expression| T { e, d: Dimension::Stress, u: "Pa".into() };
    let r = |v: f64| rlit(v).e;
    let ratio_x = || bin(Divide, x().e, x().e);
    let t = |id: &str, rows: &[(f64, f64)]| table(id, Dimension::Dimensionless, "ratio", Dimension::Stress, "Pa", rows);
    vec![
        ("x*1e300".into(), st(bin(Multiply, x().e, r(1e300)))),
        ("1e300*x".into(), st(bin(Multiply, r(1e300), x().e))),
        ("x/1e-300".into(), st(bin(Divide, x().e, r(1e-300)))),
        ("x+x".into(), st(bin(Add, x().e, x().e))),
        ("x-(-x)".into(), st(bin(Subtract, x().e, un(UnaryOperator::Negate, x().e)))),
        ("x*1e300-x*1e300".into(), st(bin(Subtract, bin(Multiply, x().e, r(1e300)), bin(Multiply, x().e, r(1e300))))),
        ("0*(x*1e300)".into(), st(bin(Multiply, r(0.0), bin(Multiply, x().e, r(1e300))))),
        ("interp_run(x/x)".into(), st(interp(t("t_wide", &[(-MAX, 0.0), (MAX, 10.0)]), ratio_x()))),
        ("interp_rise(x/x)".into(), st(interp(t("t_rise", &[(0.0, -1e308), (2.0, 1e308)]), ratio_x()))),
        ("x".into(), st(x().e)),
    ]
}

#[test]
fn rv111_runner() {
    let dir = out_dir();
    let mut out = RunOut {
        w: BufWriter::new(File::create(dir.join("run.tsv")).unwrap()),
        meta: BufWriter::new(File::create(dir.join("run_meta.jsonl")).unwrap()),
    };
    let all = vec!["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"];
    let s_partner = T { e: VariableRef("s"), d: Dimension::Stress, u: "Pa".into() };
    #[allow(non_snake_case)]
    fn VariableRef(id: &str) -> Expression {
        Expression::VariableRef(id.to_string())
    }
    // --- rpc: single checks, pc consumers over x.
    let chains: Vec<(&str, fn(&T) -> T)> = vec![("id", (|t: &T| t.clone()) as fn(&T) -> T), ("neg", w_neg as fn(&T) -> T), ("max", w_max as fn(&T) -> T), ("selU", w_sel_untaken as fn(&T) -> T)];
    let xs = [1e300, 1.0, -1e300, 1e-300, MAX];
    let mut n_rpc = 0;
    for (sname, src) in rpc_sources() {
        for (cn, cf) in &chains {
            let n = cf(&src);
            for (kn, e) in pc_consumers(&n, &s_partner) {
                let quantity = !matches!(e, Expression::Compare { .. } | Expression::Logical { .. })
                    && !matches!(&e, Expression::Unary { operator: UnaryOperator::Not, .. })
                    && !matches!(&e, Expression::Select { then_branch, .. } if matches!(**then_branch, Expression::Compare { .. }));
                let check = PackCheck { id: "c".into(), formula: encode_expression(&e), formula_inputs: vec!["x", "s"],
                    check_inputs: vec!["x", "s"], quantity, statuses: all.clone(), policy: true, relation: None };
                let doc = pack_doc(&[check]);
                for &xv in &xs {
                    let v = Vals { x: (xv, "Pa"), ..Vals::default() };
                    out.emit(&format!("rpc|{sname}|{cn}|{kn}|{xv:e}"), &doc, &v, true);
                    n_rpc += 1;
                }
            }
        }
    }
    // --- rn4: N-4 values for inputs and limits, raw and after normalization.
    let n4_values: Vec<(f64, &'static str)> = vec![
        (f64::NAN, "Pa"), (-f64::NAN, "Pa"), (f64::INFINITY, "Pa"), (f64::NEG_INFINITY, "Pa"),
        (1e300, "GPa"), (-1e300, "GPa"), (MAX, "kPa"), (1e300, "kPa"), (f64::NAN, "kPa"), (f64::INFINITY, "kPa"),
        (f64::NAN, "bogus_unit"), (1.0, "Pa"), (1e300, "Pa"), (MAX, "Pa"), (2.0, "kPa"),
    ];
    let formulas: Vec<(&str, Value, Vec<&'static str>, bool)> = vec![
        ("x_le_s", encode_expression(&cmp(ComparisonOperator::LessThanOrEqual, VariableRef("x"), VariableRef("s"))), vec!["x", "s"], false),
        ("x_q", encode_expression(&VariableRef("x")), vec!["x", "s"], true),
        ("s_plus_x", encode_expression(&bin(BinaryOperator::Add, VariableRef("s"), VariableRef("x"))), vec!["x", "s"], true),
        ("lib_le_s", encode_expression(&cmp(ComparisonOperator::LessThanOrEqual, VariableRef("lib"), VariableRef("s"))), vec!["x", "s", "lib"], false),
        ("s_only", encode_expression(&VariableRef("s")), vec!["x", "s"], true),
        ("x_s_lib_q", encode_expression(&agg(AggregateFunction::Max, vec![VariableRef("x"), VariableRef("s"), VariableRef("lib")])), vec!["x", "s", "lib"], true),
        ("x_twice", encode_expression(&cmp(ComparisonOperator::LessThan, VariableRef("x"), VariableRef("x"))), vec!["x", "x", "s"], false),
    ];
    let mut n_rn4 = 0;
    for (fname, f, finputs, quantity) in &formulas {
        for (cvar, cinputs) in [("ref", vec!["x", "s", "lib"]), ("unref", vec!["x", "s", "lib", "u"])] {
            for (svar, statuses, policy) in [("all", all.clone(), true), ("nostatus_incomplete", vec!["USER_RULE_CHECKED", "USER_RULE_FAILED"], true), ("nopolicy", all.clone(), false)] {
                let check = PackCheck { id: "c".into(), formula: f.clone(), formula_inputs: finputs.clone(), check_inputs: cinputs.clone(),
                    quantity: *quantity, statuses: statuses.clone(), policy, relation: None };
                let doc = pack_doc(&[check]);
                for (slot, _) in [("x", 0), ("s", 1), ("lib", 2), ("u", 3), ("limit", 4)] {
                    for (vi, &val) in n4_values.iter().enumerate() {
                        let mut v = Vals::default();
                        match slot {
                            "x" => v.x = val,
                            "s" => v.s = val,
                            "lib" => v.lib = val,
                            "u" => v.u = val,
                            _ => v.limit = val,
                        }
                        out.emit(&format!("rn4|{fname}|{cvar}|{svar}|{slot}=v{vi}:{}{}", val.0, val.1), &doc, &v, true);
                        n_rn4 += 1;
                    }
                }
                // Two at once: x and the limit; x and s.
                for (ai, &a) in n4_values[..6].iter().enumerate() {
                    for (bi, &b) in n4_values[..6].iter().enumerate() {
                        let v = Vals { x: a, limit: b, ..Vals::default() };
                        out.emit(&format!("rn4|{fname}|{cvar}|{svar}|x=v{ai}:{}{},limit=v{bi}:{}{}", a.0, a.1, b.0, b.1), &doc, &v, true);
                        let v = Vals { x: a, s: b, ..Vals::default() };
                        out.emit(&format!("rn4|{fname}|{cvar}|{svar}|x=v{ai}:{}{},s=v{bi}:{}{}", a.0, a.1, b.0, b.1), &doc, &v, true);
                        n_rn4 += 2;
                    }
                }
            }
        }
    }
    // --- rmulti: three checks (a producer overflow, an N-4-sensitive check, a finite check), each also alone.
    let checks = vec![
        PackCheck { id: "c_flag".into(), formula: encode_expression(&cmp(ComparisonOperator::GreaterThanOrEqual, bin(BinaryOperator::Multiply, VariableRef("x"), rlit(1e300).e), VariableRef("s"))),
            formula_inputs: vec!["x", "s"], check_inputs: vec!["x", "s"], quantity: false, statuses: all.clone(), policy: true, relation: None },
        PackCheck { id: "c_n4".into(), formula: encode_expression(&VariableRef("s")), formula_inputs: vec!["s"], check_inputs: vec!["s", "u"],
            quantity: true, statuses: all.clone(), policy: true, relation: Some("greater_than") },
        PackCheck { id: "c_fin".into(), formula: encode_expression(&cmp(ComparisonOperator::LessThanOrEqual, VariableRef("lib"), VariableRef("s"))),
            formula_inputs: vec!["lib", "s"], check_inputs: vec!["lib", "s"], quantity: false, statuses: all.clone(), policy: true, relation: None },
    ];
    let multi = pack_doc(&checks);
    let singles: Vec<Value> = checks.iter().map(|c| pack_doc(std::slice::from_ref(c))).collect();
    let mut n_multi = 0;
    for &xv in &[1e300, 1.0, f64::NAN] {
        for (si, &sv) in n4_values[..8].iter().enumerate() {
            for &lv in &[(100.0, "Pa"), (f64::NAN, "Pa"), (1e300, "GPa")] {
                let v = Vals { x: (xv, "Pa"), s: sv, limit: lv, ..Vals::default() };
                let tag = format!("x={xv:e},s=v{si}:{}{},limit={}{}", sv.0, sv.1, lv.0, lv.1);
                out.emit(&format!("rmulti|all|{tag}"), &multi, &v, true);
                for (i, single) in singles.iter().enumerate() {
                    out.emit(&format!("rmulti|single{i}|{tag}"), single, &v, true);
                }
                n_multi += 1;
            }
        }
    }
    // --- rdemo: the two committed packs over value grids (their declared unit is demo_unit).
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let demo_example: Value = serde_json::from_str(&fs::read_to_string(p.join("../../../examples/rule_packs/invented_demo.yaml")).unwrap()).unwrap();
    let demo_preview: Value = serde_json::from_str(&fs::read_to_string(p.join("../../../fixtures/product_preview/invented_demo_rule_pack.json")).unwrap()).unwrap();
    let grid = [0.0, -0.0, 5e-324, 1.0, -2.0, 1e154, 1e300, 1e308, MAX, -MAX, f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
    let mut n_demo = 0;
    for (dname, doc) in [("example", &demo_example), ("preview", &demo_preview)] {
        for &a in &grid {
            for &l in &grid {
                for &(slot, sunit) in &[(1.0, "ratio"), (f64::NAN, "ratio"), (f64::INFINITY, "ratio"), (MAX, "ratio")] {
                    for &aunit in &["demo_unit", "Pa"] {
                        let input = RuleCheckRunInput {
                            rule_pack_document: doc,
                            solver_results: vec![SolverResultBinding { input_id: "demo_actual_quantity".into(), result_id: "r".into(), value: a, unit: aunit.into() }],
                            refused_solver_results: Vec::new(),
                            supplied_values: vec![
                                SuppliedValueBinding { ref_id: "demo_limit_quantity".into(), value: l, unit: "demo_unit".into(), dimension: "stress".into() },
                                SuppliedValueBinding { ref_id: "demo_limit_slot".into(), value: slot, unit: sunit.into(), dimension: "dimensionless".into() },
                            ],
                            library_values: Vec::new(),
                            current_statuses: vec![AnalysisStatus::MechanicsSolved],
                        };
                        writeln!(out.meta, "{}", json!({ "label": format!("rdemo|{dname}|{a:e}|{l:e}|{slot:e}|{aunit}"), "demo": true })).unwrap();
                        for mode in ["plain", "b0", "b1"] {
                            let _ = take_events();
                            let line = match catch_unwind(AssertUnwindSafe(|| match mode {
                                "plain" => run_rule_checks(&input),
                                "b0" => run_rule_checks_with_bounds(&input, &[SolverResultBound { input_id: "demo_actual_quantity".into(), absolute_bound: 0.0 }]),
                                _ => run_rule_checks_with_bounds(&input, &[SolverResultBound { input_id: "demo_actual_quantity".into(), absolute_bound: 1.0 }]),
                            })) {
                                Ok(r) => serde_json::to_string(&r).unwrap(),
                                Err(_) => "PANIC".into(),
                            };
                            let events = take_events();
                            writeln!(out.w, "rdemo|{dname}|{a:e}|{l:e}|{slot:e}|{aunit}|{mode}\t{line}\t{events}").unwrap();
                            n_demo += 1;
                        }
                    }
                }
            }
        }
    }
    eprintln!("rv111 runner packs: rpc={n_rpc} rn4={n_rn4} rmulti={n_multi} rdemo_runs={n_demo} (x6 modes except rdemo)");
}
