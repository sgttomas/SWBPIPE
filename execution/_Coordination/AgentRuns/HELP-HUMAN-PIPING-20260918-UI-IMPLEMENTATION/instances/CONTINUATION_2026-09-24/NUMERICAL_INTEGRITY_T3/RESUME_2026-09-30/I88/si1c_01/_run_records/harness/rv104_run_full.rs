//! I88 variant of RV104's scratch harness rv104_run_diff.rs: every mode is
//! dumped as full JSON (five `full` arguments; nothing else differs).
//! RV104 scratch harness (not repository content). Dumps `run_rule_checks`
//! and `run_rule_checks_with_bounds` results over the committed rule packs and
//! RV104's own generated packs and values, with panics caught and labelled.
//! Point-path lines keep the full serialized `RuleCheckRunResult`; bounded
//! lines keep a 64-bit hash of it. All values are invented.

use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;

use open_pipe_stress_expression_evaluator::{
    AggregateFunction, AnalysisStatus, BinaryOperator, ComparisonOperator, Dimension, Expression,
    LogicalOperator, LookupMode, Quantity, TableRow, UnaryOperator, UserTable,
};
use open_pipe_stress_rule_check_runner::{
    run_rule_checks, run_rule_checks_with_bounds, LibraryValueBinding, RuleCheckRunInput,
    SolverResultBinding, SolverResultBound, SuppliedValueBinding,
};
use open_pipe_stress_rule_pack_document::{encode_dimension, encode_expression};
use serde_json::{json, Value};

struct Rng(u64);
impl Rng {
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

fn fnv(text: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in text.bytes() {
        h ^= byte as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
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

const MAX: f64 = f64::MAX;

fn finite_pool() -> Vec<f64> {
    vec![
        0.0, -0.0, 5e-324, -5e-324, f64::MIN_POSITIVE, 1e-308, 1e-300, 1e-160, 1e-154, 0.5, 1.0,
        -1.0, 2.0, 3.0, -2.5, 1.0f64.next_down(), 1.0f64.next_up(), 1e154, 1e155, 1e300, 1e308, MAX,
        -MAX, MAX.next_down(),
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
fn all_pool() -> Vec<f64> {
    let mut v = finite_pool();
    v.extend(special_pool());
    v
}

/// One required input of a generated pack.
#[derive(Clone)]
struct Decl {
    id: &'static str,
    source: &'static str,
    dim: Dimension,
    unit: &'static str,
}

#[derive(Clone)]
struct CheckDecl {
    check_id: String,
    ast: Value,
    inputs: Vec<&'static str>,
    slot: Option<(Dimension, &'static str)>,
    relation: Option<&'static str>,
}

fn pack(decls: &[Decl], checks: &[CheckDecl]) -> Value {
    let required_inputs: Vec<Value> = decls
        .iter()
        .map(|d| {
            json!({ "input_id": d.id, "name": d.id, "source_kind": d.source,
                "required_for": "rule_check", "provenance_required": true,
                "redistribution_status_required": true,
                "quantity_intent": { "dimension": encode_dimension(d.dim), "unit_ref": d.unit,
                    "unit_required": true, "dimension_check_required": true } })
        })
        .collect();
    let mut formulas = Vec::new();
    let mut slots = Vec::new();
    let mut check_values = Vec::new();
    for c in checks {
        let refs: Vec<Value> = c
            .inputs
            .iter()
            .map(|id| json!({ "ref_id": id, "ref_type": "required_input" }))
            .collect();
        formulas.push(json!({ "formula_id": format!("f_{}", c.check_id),
            "declaration_payload": { "expression_ast": c.ast }, "input_refs": refs }));
        let mut check = json!({ "check_id": c.check_id, "required_input_refs": refs,
            "formula_ref": { "ref_id": format!("f_{}", c.check_id), "ref_type": "formula" },
            "result_statuses": [ "RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED" ],
            "diagnostic_policy": { "missing_input": "RULE_INPUT_MISSING",
                                   "evaluator_error": "RULE_EVALUATOR_ERROR" } });
        if let Some((dim, unit)) = c.slot {
            let slot_id = format!("slot_{}", c.check_id);
            slots.push(json!({ "slot_id": slot_id, "slot_kind": "limit",
                "quantity_intent": { "dimension": encode_dimension(dim), "unit_ref": unit,
                    "unit_required": true, "dimension_check_required": true } }));
            check["value_slot_refs"] = json!([{ "ref_id": slot_id, "ref_type": "value_slot" }]);
        }
        if let Some(rel) = c.relation {
            check["acceptability_relation"] = json!(rel);
        }
        check_values.push(check);
    }
    json!({ "grammar_version": "1.0.0", "metadata": { "rule_pack_id": "rv104_pack" },
        "required_inputs": required_inputs, "formula_declarations": formulas,
        "value_slots": slots, "check_definitions": check_values })
}

/// Values for one run: per input (value, entered unit), and per slot value.
struct Values {
    inputs: Vec<(&'static str, &'static str, Dimension, f64, String)>,
    slots: Vec<(String, Dimension, &'static str, f64)>,
}

fn run_input<'a>(doc: &'a Value, v: &Values) -> RuleCheckRunInput<'a> {
    let mut solver_results = Vec::new();
    let mut supplied_values = Vec::new();
    let mut library_values = Vec::new();
    for (id, source, dim, value, unit) in &v.inputs {
        match *source {
            "solver_result" => solver_results.push(SolverResultBinding {
                input_id: id.to_string(),
                result_id: format!("result:{id}"),
                value: *value,
                unit: unit.clone(),
            }),
            "private_library_value" => library_values.push(LibraryValueBinding {
                input_id: id.to_string(),
                value: *value,
                unit: unit.clone(),
                library_kind: "invented".into(),
                library_id: "lib".into(),
                record_id: "rec".into(),
                slot_id: "slot".into(),
            }),
            _ => supplied_values.push(SuppliedValueBinding {
                ref_id: id.to_string(),
                value: *value,
                unit: unit.clone(),
                dimension: encode_dimension(*dim).to_string(),
            }),
        }
    }
    for (slot_id, dim, unit, value) in &v.slots {
        supplied_values.push(SuppliedValueBinding {
            ref_id: slot_id.clone(),
            value: *value,
            unit: unit.to_string(),
            dimension: encode_dimension(*dim).to_string(),
        });
    }
    RuleCheckRunInput {
        rule_pack_document: doc,
        solver_results,
        refused_solver_results: Vec::new(),
        supplied_values,
        library_values,
        current_statuses: vec![AnalysisStatus::MechanicsSolved],
    }
}

fn run_text(input: &RuleCheckRunInput, bounds: Option<&[SolverResultBound]>) -> Result<String, String> {
    match catch_unwind(AssertUnwindSafe(|| match bounds {
        None => run_rule_checks(input),
        Some(b) => run_rule_checks_with_bounds(input, b),
    })) {
        Ok(result) => Ok(serde_json::to_string(&result).unwrap()),
        Err(p) => Err(panic_msg(&p).replace('\n', " ")),
    }
}

struct Out(String);
impl Out {
    fn line(&mut self, label: &str, mode: &str, text: &Result<String, String>, full: bool) {
        let body = match text {
            Ok(t) if full => format!("{:016x}\t{t}", fnv(t)),
            Ok(t) => format!("{:016x}", fnv(t)),
            Err(m) => format!("PANIC({m})"),
        };
        self.0.push_str(&format!("{label}\t{mode}\t{body}\n"));
    }

    /// The point run (full), b = 0 on every solver input, and the bounded modes.
    fn all_modes(&mut self, label: &str, doc: &Value, v: &Values) {
        let input = run_input(doc, v);
        let solver: Vec<(String, f64)> = input
            .solver_results
            .iter()
            .map(|s| (s.input_id.clone(), s.value))
            .collect();
        let bounds = |f: &dyn Fn(f64) -> f64| -> Vec<SolverResultBound> {
            solver
                .iter()
                .map(|(id, q)| SolverResultBound {
                    input_id: id.clone(),
                    absolute_bound: f(*q),
                })
                .collect()
        };
        self.line(label, "p", &run_text(&input, None), true);
        let full = true; // I88: every mode as full JSON (was: label.starts_with("m_tab"))
        self.line(label, "b0", &run_text(&input, Some(&bounds(&|_| 0.0))), full);
        self.line(label, "b1", &run_text(&input, Some(&bounds(&|_| 1.0))), full);
        self.line(
            label,
            "bR",
            &run_text(&input, Some(&bounds(&|q| q.abs() * 1e-9 + 1e-300))),
            full,
        );
        self.line(label, "bH", &run_text(&input, Some(&bounds(&|_| 1e300))), full);
        if let Some((id, _)) = solver.first() {
            let bad = vec![SolverResultBound { input_id: id.clone(), absolute_bound: f64::NAN }];
            self.line(label, "bN", &run_text(&input, Some(&bad)), full);
            let dup = vec![
                SolverResultBound { input_id: id.clone(), absolute_bound: 0.0 },
                SolverResultBound { input_id: id.clone(), absolute_bound: 0.0 },
            ];
            self.line(label, "bD", &run_text(&input, Some(&dup)), full);
        }
    }
}

fn read_json(rel: &str) -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

fn family_committed(out: &mut Out) {
    let packs = [
        ("demo", read_json("../../../examples/rule_packs/invented_demo.yaml")),
        ("pp", read_json("../../../fixtures/product_preview/invented_demo_rule_pack.json")),
    ];
    let pool = all_pool();
    let slots = [1.0, f64::NAN, MAX, 5e-324, f64::INFINITY];
    for (name, doc) in &packs {
        for (ai, &a) in pool.iter().enumerate() {
            for (bi, &b) in pool.iter().enumerate() {
                for (si, &s) in slots.iter().enumerate() {
                    if si >= 3 && (ai + bi) % 4 != 0 {
                        continue;
                    }
                    let v = Values {
                        inputs: vec![
                            ("demo_actual_quantity", "solver_result", Dimension::Stress, a, "demo_unit".into()),
                            ("demo_limit_quantity", "user_supplied_rule_value", Dimension::Stress, b, "demo_unit".into()),
                        ],
                        slots: vec![("demo_limit_slot".into(), Dimension::Dimensionless, "ratio", s)],
                    };
                    out.all_modes(&format!("k_{name}_{ai}_{bi}_{si}"), doc, &v);
                }
            }
        }
    }
}

fn family_catalog_units(out: &mut Out) {
    let decls = [
        Decl { id: "a", source: "solver_result", dim: Dimension::Stress, unit: "Pa" },
        Decl { id: "b", source: "user_supplied_rule_value", dim: Dimension::Stress, unit: "Pa" },
    ];
    let ratio_ast = encode_expression(&Expression::Binary {
        operator: BinaryOperator::Divide,
        left: Box::new(Expression::VariableRef("a".into())),
        right: Box::new(Expression::VariableRef("b".into())),
    });
    let doc = pack(
        &decls,
        &[CheckDecl {
            check_id: "u".into(),
            ast: ratio_ast,
            inputs: vec!["a", "b"],
            slot: Some((Dimension::Dimensionless, "ratio")),
            relation: None,
        }],
    );
    let pool = all_pool();
    for (ui, unit) in ["Pa", "kPa", "MPa", "psi"].iter().enumerate() {
        for (ai, &a) in pool.iter().enumerate() {
            for (bi, &b) in pool.iter().enumerate() {
                let v = Values {
                    inputs: vec![
                        ("a", "solver_result", Dimension::Stress, a, unit.to_string()),
                        ("b", "user_supplied_rule_value", Dimension::Stress, b, "Pa".into()),
                    ],
                    slots: vec![("slot_u".into(), Dimension::Dimensionless, "ratio", 1.0)],
                };
                out.all_modes(&format!("u_{ui}_{ai}_{bi}"), &doc, &v);
            }
        }
    }
}

// Generated formulas over a fixed set of inputs.
const DECLS: &[Decl] = &[
    Decl { id: "s1", source: "solver_result", dim: Dimension::Stress, unit: "demo_unit" },
    Decl { id: "s2", source: "solver_result", dim: Dimension::Stress, unit: "demo_unit" },
    Decl { id: "s3", source: "user_supplied_rule_value", dim: Dimension::Stress, unit: "demo_unit" },
    Decl { id: "l1", source: "private_library_value", dim: Dimension::Length, unit: "lu" },
    Decl { id: "z1", source: "user_supplied_rule_value", dim: Dimension::Dimensionless, unit: "ratio" },
    Decl { id: "t1", source: "solver_result", dim: Dimension::Temperature, unit: "tu" },
];
// Quantity kinds: 0 stress, 1 length, 2 ratio, 3 temperature.
const KINDS: &[(Dimension, &str, &[&str])] = &[
    (Dimension::Stress, "demo_unit", &["s1", "s2", "s3"]),
    (Dimension::Length, "lu", &["l1"]),
    (Dimension::Dimensionless, "ratio", &["z1"]),
    (Dimension::Temperature, "tu", &["t1"]),
];

fn lit(value: f64, dimension: Dimension, unit: &str) -> Expression {
    Expression::Literal(Quantity {
        value,
        dimension,
        unit_ref: unit.into(),
        unit_required: true,
        dimension_check_required: true,
    })
}
fn bin(op: BinaryOperator, a: Expression, b: Expression) -> Expression {
    Expression::Binary { operator: op, left: Box::new(a), right: Box::new(b) }
}

fn gq(r: &mut Rng, k: usize, depth: u32) -> Expression {
    let (dim, unit, vars) = KINDS[k];
    if depth == 0 || r.chance(4) {
        if r.chance(2) {
            return Expression::VariableRef(r.pick(vars).to_string());
        }
        return lit(r.pick(&finite_pool()), dim, unit);
    }
    let dn = depth - 1;
    match r.below(10) {
        0 => bin(
            if r.chance(2) { BinaryOperator::Add } else { BinaryOperator::Subtract },
            gq(r, k, dn),
            gq(r, k, dn),
        ),
        1 => bin(BinaryOperator::Multiply, gq(r, 2, dn), gq(r, k, dn)),
        2 | 3 => {
            if k == 2 {
                let x = r.below(4) as usize;
                bin(BinaryOperator::Divide, gq(r, x, dn), gq(r, x, dn))
            } else {
                bin(BinaryOperator::Divide, gq(r, k, dn), gq(r, 2, dn))
            }
        }
        4 => Expression::Unary {
            operator: if r.chance(2) { UnaryOperator::Negate } else { UnaryOperator::Abs },
            operand: Box::new(gq(r, k, dn)),
        },
        5 => Expression::Aggregate {
            function: if r.chance(2) { AggregateFunction::Min } else { AggregateFunction::Max },
            operands: (0..1 + r.below(3)).map(|_| gq(r, k, dn)).collect(),
        },
        6 => Expression::Select {
            condition: Box::new(gb(r, dn)),
            then_branch: Box::new(gq(r, k, dn)),
            else_branch: Box::new(gq(r, k, dn)),
        },
        7 if k == 0 => {
            let (arg_k, rows): (usize, Vec<(f64, f64)>) = match r.below(3) {
                0 => (2, vec![(-2.0, 1.0), (0.0, -3.0), (0.5, 4.0), (3.0, 4.5)]),
                1 => (3, vec![(10.0, 1.5), (20.0, 2.5), (40.0, 3.5)]),
                _ => (2, vec![(-MAX, -1e308), (0.0, 0.0), (MAX, 1e308)]),
            };
            let (ad, au, _) = KINDS[arg_k];
            let table = UserTable {
                table_id: "rv104_table".into(),
                argument_dimension: ad,
                argument_unit_ref: au.into(),
                result_dimension: Dimension::Stress,
                result_unit_ref: "demo_unit".into(),
                rows: rows.iter().map(|&(argument, result)| TableRow { argument, result }).collect(),
            };
            let argument = Box::new(gq(r, arg_k, dn));
            match r.below(3) {
                0 => Expression::Interpolate { table, argument },
                1 => Expression::Lookup { table, mode: LookupMode::Step, argument },
                _ => Expression::Lookup { table, mode: LookupMode::Exact, argument },
            }
        }
        _ => gq(r, k, dn),
    }
}

fn gb(r: &mut Rng, depth: u32) -> Expression {
    let ops = [
        ComparisonOperator::LessThan,
        ComparisonOperator::LessThanOrEqual,
        ComparisonOperator::GreaterThan,
        ComparisonOperator::GreaterThanOrEqual,
        ComparisonOperator::Equal,
        ComparisonOperator::NotEqual,
    ];
    let op = r.pick(&ops);
    let dn = depth.saturating_sub(1);
    match if depth == 0 { 0 } else { r.below(5) } {
        0 | 1 => {
            let k = r.below(4) as usize;
            Expression::Compare { operator: op, left: Box::new(gq(r, k, dn)), right: Box::new(gq(r, k, dn)) }
        }
        2 => Expression::Logical {
            operator: if r.chance(2) { LogicalOperator::And } else { LogicalOperator::Or },
            left: Box::new(gb(r, dn)),
            right: Box::new(gb(r, dn)),
        },
        3 => Expression::Unary { operator: UnaryOperator::Not, operand: Box::new(gb(r, dn)) },
        _ => Expression::Select {
            condition: Box::new(gb(r, dn)),
            then_branch: Box::new(gb(r, dn)),
            else_branch: Box::new(gb(r, dn)),
        },
    }
}

fn random_values(r: &mut Rng, slots: &[(String, Dimension, &'static str)]) -> Values {
    let fin = finite_pool();
    let spec = special_pool();
    let pick = |r: &mut Rng| -> f64 {
        if r.chance(12) {
            r.pick(&spec)
        } else if r.chance(3) {
            r.pick(&[1.0, 2.0, 0.5, 3.0, 15.0])
        } else {
            r.pick(&fin)
        }
    };
    let inputs = DECLS
        .iter()
        .map(|d| (d.id, d.source, d.dim, pick(r), d.unit.to_string()))
        .collect();
    let slots = slots
        .iter()
        .map(|(id, dim, unit)| (id.clone(), *dim, *unit, pick(r)))
        .collect();
    Values { inputs, slots }
}

fn generated_check(r: &mut Rng, check_id: &str) -> CheckDecl {
    let all_ids: Vec<&'static str> = DECLS.iter().map(|d| d.id).collect();
    if r.chance(2) {
        CheckDecl {
            check_id: check_id.into(),
            ast: encode_expression(&gb(r, 3)),
            inputs: all_ids,
            slot: None,
            relation: None,
        }
    } else {
        let k = r.below(4) as usize;
        let (dim, unit, _) = KINDS[k];
        let rel = r.pick(&[None, Some("less_than"), Some("greater_than_or_equal"), Some("equal")]);
        CheckDecl {
            check_id: check_id.into(),
            ast: encode_expression(&gq(r, k, 3)),
            inputs: all_ids,
            slot: Some((dim, unit)),
            relation: rel,
        }
    }
}

fn slots_of(checks: &[CheckDecl]) -> Vec<(String, Dimension, &'static str)> {
    checks
        .iter()
        .filter_map(|c| c.slot.map(|(d, u)| (format!("slot_{}", c.check_id), d, u)))
        .collect()
}

fn family_generated(out: &mut Out) {
    let mut r = Rng(0x5256_3130_3452_554E);
    for i in 0..3500 {
        let check = generated_check(&mut r, "g");
        let checks = vec![check];
        let doc = pack(DECLS, &checks);
        let slots = slots_of(&checks);
        for j in 0..4 {
            let v = random_values(&mut r, &slots);
            out.all_modes(&format!("g_{i}_{j}"), &doc, &v);
        }
    }
}

/// Three checks in one pack, and each alone: a generated check, a ratio
/// check and a boolean check. The analysis compares the multi-check outcomes
/// with the single-check runs (a blocked check must not stop the others).
fn family_multi(out: &mut Out) {
    let mut r = Rng(0x5256_3130_344D_554C);
    let ratio = CheckDecl {
        check_id: "ratio".into(),
        ast: encode_expression(&bin(
            BinaryOperator::Divide,
            Expression::VariableRef("s1".into()),
            Expression::VariableRef("s3".into()),
        )),
        inputs: vec!["s1", "s3"],
        slot: Some((Dimension::Dimensionless, "ratio")),
        relation: None,
    };
    let boolean = CheckDecl {
        check_id: "bool".into(),
        ast: encode_expression(&Expression::Compare {
            operator: ComparisonOperator::LessThanOrEqual,
            left: Box::new(Expression::VariableRef("s2".into())),
            right: Box::new(Expression::VariableRef("s3".into())),
        }),
        inputs: vec!["s2", "s3"],
        slot: None,
        relation: None,
    };
    for i in 0..1500 {
        let generated = generated_check(&mut r, "gen");
        let order = r.below(3);
        let checks: Vec<CheckDecl> = match order {
            0 => vec![generated.clone(), ratio.clone(), boolean.clone()],
            1 => vec![ratio.clone(), generated.clone(), boolean.clone()],
            _ => vec![ratio.clone(), boolean.clone(), generated.clone()],
        };
        let doc = pack(DECLS, &checks);
        let slots = slots_of(&checks);
        for j in 0..3 {
            let v = random_values(&mut r, &slots);
            let label = format!("m_{i}_{j}_o{order}");
            out.all_modes(&label, &doc, &v);
            for (ci, c) in checks.iter().enumerate() {
                let single = pack(DECLS, std::slice::from_ref(c));
                let input = run_input(&single, &v);
                out.line(&format!("{label}_single{ci}"), "p", &run_text(&input, None), true);
            }
        }
    }
}

/// Table checks over carried NaN arguments: three user-input table checks
/// (interpolate, step, exact; point path even when bounds are supplied), a
/// bounded boolean check, and a table check over a solver input.
fn family_tables(out: &mut Out) {
    let decls = [
        Decl { id: "s1", source: "solver_result", dim: Dimension::Stress, unit: "demo_unit" },
        Decl { id: "z1", source: "user_supplied_rule_value", dim: Dimension::Dimensionless, unit: "ratio" },
        Decl { id: "z2", source: "solver_result", dim: Dimension::Dimensionless, unit: "ratio" },
    ];
    let ratio_lit = |v: f64| lit(v, Dimension::Dimensionless, "ratio");
    let var = |id: &str| Expression::VariableRef(id.to_string());
    let huge = |id: &str| {
        bin(BinaryOperator::Multiply, ratio_lit(MAX), bin(BinaryOperator::Multiply, ratio_lit(MAX), var(id)))
    };
    let form = |k: usize, id: &str| -> Expression {
        match k {
            0 => bin(BinaryOperator::Subtract, huge(id), huge(id)),
            1 => bin(BinaryOperator::Multiply, ratio_lit(0.0), huge(id)),
            2 => bin(
                BinaryOperator::Subtract,
                bin(BinaryOperator::Multiply, var(id), ratio_lit(1e300)),
                bin(BinaryOperator::Multiply, var(id), ratio_lit(1e300)),
            ),
            3 => Expression::Unary { operator: UnaryOperator::Negate, operand: Box::new(huge(id)) },
            _ => var(id),
        }
    };
    let table = UserTable {
        table_id: "rv104_nan_table".into(),
        argument_dimension: Dimension::Dimensionless,
        argument_unit_ref: "ratio".into(),
        result_dimension: Dimension::Stress,
        result_unit_ref: "demo_unit".into(),
        rows: [(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)]
            .iter()
            .map(|&(argument, result)| TableRow { argument, result })
            .collect(),
    };
    let table_check = |id: &str, mode: usize, argument: Expression, input: &'static str| CheckDecl {
        check_id: id.into(),
        ast: encode_expression(&match mode {
            0 => Expression::Interpolate { table: table.clone(), argument: Box::new(argument) },
            1 => Expression::Lookup { table: table.clone(), mode: LookupMode::Step, argument: Box::new(argument) },
            _ => Expression::Lookup { table: table.clone(), mode: LookupMode::Exact, argument: Box::new(argument) },
        }),
        inputs: vec![input],
        slot: Some((Dimension::Stress, "demo_unit")),
        relation: None,
    };
    let pool = all_pool();
    for k in 0..5 {
        let checks = vec![
            table_check("ti", 0, form(k, "z1"), "z1"),
            table_check("ts", 1, form(k, "z1"), "z1"),
            table_check("te", 2, form(k, "z1"), "z1"),
            CheckDecl {
                check_id: "b".into(),
                ast: encode_expression(&Expression::Compare {
                    operator: ComparisonOperator::LessThanOrEqual,
                    left: Box::new(var("s1")),
                    right: Box::new(lit(100.0, Dimension::Stress, "demo_unit")),
                }),
                inputs: vec!["s1"],
                slot: None,
                relation: None,
            },
            table_check("tz", (k % 3) as usize, form(k, "z2"), "z2"),
        ];
        let doc = pack(&decls, &checks);
        let slots = slots_of(&checks);
        for (ai, &z1) in pool.iter().enumerate() {
            for (bi, &z2) in [1.0, 1e10, 0.5, f64::NAN, 1e-320].iter().enumerate() {
                let v = Values {
                    inputs: vec![
                        ("s1", "solver_result", Dimension::Stress, 50.0, "demo_unit".into()),
                        ("z1", "user_supplied_rule_value", Dimension::Dimensionless, z1, "ratio".into()),
                        ("z2", "solver_result", Dimension::Dimensionless, z2, "ratio".into()),
                    ],
                    slots: slots.iter().map(|(id, d, u)| (id.clone(), *d, *u, 10.0)).collect(),
                };
                let label = format!("m_tab_{k}_{ai}_{bi}");
                out.all_modes(&label, &doc, &v);
                for (ci, c) in checks.iter().enumerate() {
                    let single = pack(&decls, std::slice::from_ref(c));
                    let input = run_input(&single, &v);
                    out.line(&format!("{label}_single{ci}"), "p", &run_text(&input, None), true);
                }
            }
        }
    }
}

#[test]
fn rv104_runner_dump() {
    std::panic::set_hook(Box::new(|_| {}));
    let mut out = Out(String::new());
    family_committed(&mut out);
    family_catalog_units(&mut out);
    family_generated(&mut out);
    family_multi(&mut out);
    family_tables(&mut out);
    let _ = std::panic::take_hook();
    let path = std::env::var("RV104_RUN_OUT").expect("RV104_RUN_OUT");
    fs::write(path, out.0).unwrap();
}
