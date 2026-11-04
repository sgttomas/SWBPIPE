//! RT-A3 §4: the reader of `contract_evidence.combination_certificate` on the
//! preview-physics-2 and pressure-2 routes (DEL-05-02 Work `combination-certificate.md`;
//! `operating-case.md` §4.3). The Python mirror is
//! `tests/verifiers/combination_certificate.py`.
//!
//! The reader re-derives, bit for bit and without the producer:
//! - which load cases are eligible operands, from their published integrity
//!   statements (Passed, or Sensitive by the recovery guard after a formed member bound);
//! - each operand group's covered rows, from the closed coverage table, and its
//!   binding (q equal to the correctly rounded norm of the rows);
//! - every derived group: its rows (by their source rows), the exact rounding error,
//!   B_D, q_D, q̂_D, the floor scale, the verdict and D-6's statement;
//! - each combination's assessment and diagnostic.
//!
//! It does not re-derive B_g and S*_g: those are the operand cases' published
//! statements (PRD §12.8). A forged B_D, verdict, row binding, operand-bound binding,
//! eligibility or diagnostic is refused.
use open_pipe_stress_correct_norm::{norm2, norm3};
use open_pipe_stress_load_case_algebra::Dyadic;
use serde_json::{json, Value};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub(crate) const MEMBER: &str = "combination_certificate";
const WITHIN: &str = "COMBINATION_ROWS_WITHIN_STATED_BOUND";
const NOT_ASSESSED: &str = "COMBINATION_ROWS_NOT_ASSESSED";
const DIAGNOSTIC_PREFIX: &str = "diagnostic:combination-certificate:";
const SOURCE: &str = "core/product_physics";
const PASSED: &str = "NUMERICAL_INTEGRITY_CHECKS_PASSED";
const SENSITIVE: &str = "NUMERICAL_INTEGRITY_SENSITIVE";
/// formation_guard's constants: RD(1e-9), 2^-34 and 2^-988.
const CRITERION: f64 = f64::from_bits(1e-9_f64.to_bits() - 1);
const DESIGN_FLOOR: f64 = f64::from_bits(0x3DD0_0000_0000_0000);
const DESIGN_FLOOR_MIN_SCALE: f64 = f64::from_bits((1023 - 988) << 52);

const AXIAL: &str = "element_local_axial_force";
const SHEAR_Y: &str = "element_local_shear_force_y";
const SHEAR_Z: &str = "element_local_shear_force_z";
const TORSION: &str = "element_local_torsional_moment";
const BENDING_Y: &str = "element_local_bending_moment_y";
const BENDING_Z: &str = "element_local_bending_moment_z";
const EFFECTIVE: &str = "pipe_effective_axial_force_v2";
const WALL: &str = "pipe_wall_axial_force_v2";
const END_ACTION: &str = "pipe_wall_endpoint_action_v2";
const SUPPORT_COMPONENT: &str = "support_reaction_component_v2";
const SUPPORT_FORCE: &str = "support_reaction_force_magnitude_v2";
const SUPPORT_MOMENT: &str = "support_reaction_moment_magnitude_v2";
const MODULUS_RECORD: &str = "combination_modulus_basis_record";

const MEMBER_KEYS: [&str; 2] = ["combinations", "operand_cases"];
const CASE_KEYS: [&str; 2] = ["groups", "load_case_id"];
const OPERAND_GROUP_KEYS: [&str; 8] = ["bound", "entity", "floor_scale", "group", "location", "moment", "q", "rows"];
const RECORD_KEYS: [&str; 8] = ["assessment", "basis", "combination_id", "expression", "groups", "reason", "subtrahend_support_state", "uncovered_row_count"];

type Check = Result<(), String>;

fn fail(code: &str) -> String {
    format!("SOURCE_COMBINATION_CERTIFICATE_{code}")
}
fn require(ok: bool, code: &str) -> Check {
    if ok { Ok(()) } else { Err(fail(code)) }
}
fn items(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}
fn keys(v: &Value, wanted: &[&str]) -> bool {
    v.as_object().is_some_and(|o| o.len() == wanted.len() && wanted.iter().all(|k| o.contains_key(*k)))
}
fn number(x: f64) -> Value {
    if x.is_finite() { json!(x) } else { Value::Null }
}
fn identity(parts: &[&str]) -> String {
    parts.iter().map(|p| format!("{}:{p}", p.len())).collect()
}

// ------------------------------------------------------------------ upward arithmetic (Work §1)

fn exact(x: f64) -> Option<Dyadic> {
    Dyadic::from_f64(x)
}
fn up_add(a: f64, b: f64) -> f64 {
    let s = a + b;
    if !s.is_finite() {
        return f64::INFINITY;
    }
    let is_exact = matches!((exact(a), exact(b), exact(s)), (Some(a), Some(b), Some(s)) if a.plus(&b).is_some_and(|t| t == s));
    if is_exact { s } else { s.next_up() }
}
fn up_mul(a: f64, b: f64) -> f64 {
    let p = a * b;
    if !p.is_finite() {
        return f64::INFINITY;
    }
    let is_exact = matches!((exact(a), exact(p)), (Some(a), Some(p)) if a.times_f64(b).is_some_and(|t| t == p));
    if is_exact { p } else { p.next_up() }
}
fn up_sqrt(x: f64) -> f64 {
    if !x.is_finite() {
        return f64::INFINITY;
    }
    let r = x.sqrt();
    let is_exact = matches!((exact(r), exact(x)), (Some(re), Some(x)) if re.times_f64(r).is_some_and(|t| t == x));
    if is_exact { r } else { r.next_up() }
}
fn up_norm(values: &[f64]) -> f64 {
    let mut sum = 0.0;
    for v in values {
        sum = up_add(sum, up_mul(v.abs(), v.abs()));
    }
    up_sqrt(sum)
}
/// The smallest binary64 not below |d| by bisection over the ordered bit patterns.
fn round_up_magnitude(d: &Dyadic) -> f64 {
    if d.is_zero() {
        return 0.0;
    }
    let at_least = |c: f64| exact(c).is_some_and(|c| c.compare_magnitude(d) != Ordering::Less);
    if !at_least(f64::MAX) {
        return f64::INFINITY;
    }
    let (mut low, mut high) = (0u64, f64::MAX.to_bits());
    while low < high {
        let middle = low + (high - low) / 2;
        if at_least(f64::from_bits(middle)) { high = middle } else { low = middle + 1 }
    }
    f64::from_bits(high)
}
fn rounding_error(derived: f64, terms: &[(f64, f64)]) -> f64 {
    let Some(mut sum) = exact(derived) else { return f64::INFINITY };
    for (factor, value) in terms {
        match exact(*value).and_then(|v| v.times_f64(*factor)).and_then(|p| sum.minus(&p)) {
            Some(next) => sum = next,
            None => return f64::INFINITY,
        }
    }
    round_up_magnitude(&sum)
}
/// B > RD(1e-9)·q exactly.
fn exceeds(bound: f64, q: f64) -> Option<bool> {
    let lhs = exact(bound)?;
    let rhs = exact(q)?.times_f64(CRITERION)?;
    Some(lhs.minus(&rhs)?.signum() > 0)
}
fn verdict(q: f64, q_hat: f64, bound: f64, s: f64) -> (&'static str, Option<&'static str>) {
    let unavailable = ("not_established", Some("comparison_unavailable"));
    if ![q, q_hat, bound, s].iter().all(|x| x.is_finite()) || q < 0.0 || q_hat < q || bound < 0.0 || s < 0.0 {
        return unavailable;
    }
    let Some(fires) = exceeds(bound, q) else { return unavailable };
    if !(s >= DESIGN_FLOOR_MIN_SCALE && q_hat >= DESIGN_FLOOR * s) {
        ("not_assessed", Some("below_floor"))
    } else if fires {
        ("not_established", Some("bound_exceeds_criterion"))
    } else {
        ("established", None)
    }
}
fn group_norm(values: &[f64]) -> Option<f64> {
    match values {
        [a] => Some(a.abs()),
        [a, b] => Some(norm2(*a, *b)),
        [a, b, c] => Some(norm3(*a, *b, *c)),
        _ => None,
    }
}

// ------------------------------------------------------------------ coverage (Work §1 table)

type Spec = (&'static str, &'static str, Option<&'static str>);

fn row_sets(group: &str, location: &str) -> Vec<Vec<Spec>> {
    let at = ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"].into_iter().find(|l| *l == location);
    let station = matches!(location, "quarter_1" | "midspan" | "quarter_3");
    let mut out: Vec<Vec<Spec>> = Vec::new();
    let one = |k: &'static str| at.map(|l| vec![(k, l, None)]);
    let pair = |a: &'static str, b: &'static str| at.map(|l| vec![(a, l, None), (b, l, None)]);
    match group {
        "axial" => {
            out.extend(one(AXIAL));
            if station {
                out.extend(one(EFFECTIVE));
            }
            match location {
                "cut_i" => out.push(vec![(EFFECTIVE, "end_i", None)]),
                "cut_j" => out.push(vec![(EFFECTIVE, "end_j", None)]),
                _ => {}
            }
        }
        "shear" => out.extend(pair(SHEAR_Y, SHEAR_Z)),
        "torsion" => out.extend(one(TORSION)),
        "bending" => out.extend(pair(BENDING_Y, BENDING_Z)),
        "pipe_wall" => match location {
            "wall_end_i" => out.push(vec![(END_ACTION, "end_i", None)]),
            "wall_end_j" => out.push(vec![(END_ACTION, "end_j", None)]),
            "wall_cut_i" => out.push(vec![(WALL, "end_i", None)]),
            "wall_cut_j" => out.push(vec![(WALL, "end_j", None)]),
            "wall_quarter_1" => out.push(vec![(WALL, "quarter_1", None)]),
            "wall_midspan" => out.push(vec![(WALL, "midspan", None)]),
            "wall_quarter_3" => out.push(vec![(WALL, "quarter_3", None)]),
            _ => {}
        },
        "support_force" if location == "node" => out.push(vec![(SUPPORT_COMPONENT, "node", Some("Fx")), (SUPPORT_COMPONENT, "node", Some("Fy")), (SUPPORT_COMPONENT, "node", Some("Fz"))]),
        "support_moment" if location == "node" => out.push(vec![(SUPPORT_COMPONENT, "node", Some("Mx")), (SUPPORT_COMPONENT, "node", Some("My")), (SUPPORT_COMPONENT, "node", Some("Mz"))]),
        "support_force_magnitude" if location == "node" => out.push(vec![(SUPPORT_FORCE, "node", None)]),
        "support_moment_magnitude" if location == "node" => out.push(vec![(SUPPORT_MOMENT, "node", None)]),
        _ => {}
    }
    out
}
/// The class of a covered group kind; `None` for kinds the table does not cover.
fn moment_class(group: &str) -> Option<bool> {
    match group {
        "axial" | "shear" | "pipe_wall" | "support_force" | "support_force_magnitude" => Some(false),
        "torsion" | "bending" | "support_moment" | "support_moment_magnitude" => Some(true),
        _ => None,
    }
}

struct Rows<'s> {
    results: &'s [Value],
    case_index: HashMap<(&'s str, &'s str, &'s str, &'s str, Option<&'s str>), Vec<usize>>,
    by_id: HashMap<&'s str, usize>,
    by_sources: HashMap<(&'s str, Vec<&'s str>), Vec<usize>>,
    of_combination: HashMap<&'s str, Vec<usize>>,
}

impl<'s> Rows<'s> {
    fn new(results: &'s [Value]) -> Self {
        let mut rows = Rows { results, case_index: HashMap::new(), by_id: HashMap::new(), by_sources: HashMap::new(), of_combination: HashMap::new() };
        for (i, row) in results.iter().enumerate() {
            if let Some(id) = row["id"].as_str() {
                rows.by_id.insert(id, i);
            }
            let (Some(ref_type), Some(ref_id)) = (row["basis_ref"]["ref_type"].as_str(), row["basis_ref"]["ref_id"].as_str()) else { continue };
            let kind = row["kind"].as_str().unwrap_or_default();
            if ref_type == "load_case" {
                let (Some(entity), Some(location)) = (row["entity_ref"].as_str(), row["metadata"]["location"].as_str()) else { continue };
                let component = (kind == SUPPORT_COMPONENT).then(|| row["metadata"]["component"].as_str().unwrap_or_default());
                rows.case_index.entry((ref_id, kind, entity, location, component)).or_default().push(i);
            } else if ref_type == "combination" && kind != MODULUS_RECORD {
                rows.of_combination.entry(ref_id).or_default().push(i);
                let sources: Vec<&str> = items(&row["source_result_refs"]).iter().filter_map(Value::as_str).collect();
                rows.by_sources.entry((ref_id, sources)).or_default().push(i);
            }
        }
        rows
    }
    fn value(&self, i: usize) -> f64 {
        self.results[i]["value"].as_f64().unwrap_or(f64::NAN)
    }
    fn id(&self, i: usize) -> &'s str {
        self.results[i]["id"].as_str().unwrap_or_default()
    }
    /// The usable row sets of a group (Work §1): present once each and bound to q.
    fn coverage(&self, case: &str, entity: &str, group: &str, location: &str, q: f64) -> Vec<Option<Vec<usize>>> {
        row_sets(group, location)
            .iter()
            .map(|specs| {
                let rows: Vec<usize> = specs
                    .iter()
                    .map(|(kind, l, c)| match self.case_index.get(&(case, *kind, entity, *l, *c)).map(Vec::as_slice) {
                        Some([one]) => Some(*one),
                        _ => None,
                    })
                    .collect::<Option<_>>()?;
                let values: Vec<f64> = rows.iter().map(|&i| self.value(i)).collect();
                (group_norm(&values)?.to_bits() == q.to_bits()).then_some(rows)
            })
            .collect()
    }
}

/// Whether a case's published integrity statement shows certified rows (Work §1).
fn eligible(diagnostics: &[Value], case: &str) -> bool {
    let id = format!("diagnostic:numerical-integrity:{case}");
    let Some(d) = diagnostics.iter().find(|d| d["id"] == id.as_str()) else { return false };
    d["code"] == PASSED
        || (d["code"] == SENSITIVE
            && d["message"].as_str().is_some_and(|m| m.contains(&format!("S11-G recovery guard (R-b'): load case {case} is Sensitive because"))))
}

struct OperandGroup<'v> {
    entity: &'v str,
    location: &'v str,
    group: &'v str,
    q: f64,
    bound: Option<f64>,
    floor_scale: f64,
}

/// Premise (Work `combination-certificate.md` §1): the operand bounds are each operand
/// case's own. A held subtrahend (any `subtrahend_support_state` but absent or `own`)
/// is a different system from its load case: not assessed, fail closed.
fn held_reason(token: &Value) -> Result<Option<String>, String> {
    match token {
        Value::Null => Ok(None),
        Value::String(t) if t == "own" => Ok(None),
        Value::String(t) => Ok(Some(format!(
            "its subtrahend is held at another case's support state (subtrahend_support_state {t}), a different system from its load case; the operand bounds are each operand case's own"
        ))),
        _ => Err(fail("RECORD_SHAPE")),
    }
}

/// The record token of a model combination's `subtrahend_support_state`.
fn support_token(combination: &Value) -> Value {
    match combination.get("subtrahend_support_state") {
        None => Value::Null,
        Some(Value::String(t)) if t == "own" || t == "minuend_final" => json!(t),
        Some(_) => json!("unrecognized"),
    }
}

/// The operand load cases and factors of a combination record, or why it is outside.
fn terms_of(record: &Value, cases: &HashSet<&str>) -> Result<Vec<(String, f64)>, &'static str> {
    let expression = &record["expression"];
    match record["basis"].as_str() {
        Some("mechanics") => {
            let terms = items(&expression["terms"]);
            if terms.iter().any(|t| t.get("combination_id").is_some() || !t["load_case_id"].as_str().is_some_and(|c| cases.contains(c))) {
                return Err("a combination operand is outside the certificate");
            }
            Ok(terms.iter().map(|t| (t["load_case_id"].as_str().unwrap_or_default().to_string(), t["factor"].as_f64().unwrap_or(f64::NAN))).collect())
        }
        Some("result_state_subtraction") => {
            let (Some(m), Some(s)) = (expression["minuend_id"].as_str(), expression["subtrahend_id"].as_str()) else {
                return Err("an incomplete expression");
            };
            if !cases.contains(m) || !cases.contains(s) {
                return Err("a combination operand is outside the certificate");
            }
            Ok(vec![(m.to_string(), 1.0), (s.to_string(), -1.0)])
        }
        Some("range_envelope") => Err("range-envelope certification is deferred"),
        _ => Err("absolute sums and SRSS are outside the certificate"),
    }
}

fn region_of<'v>(pressure: &'v [Value], case: &str, member: &str) -> Option<&'v Value> {
    let mut hits = pressure.iter().filter(|r| r["load_case_id"] == case && items(&r["member_pipe_ids"]).iter().any(|p| p == member));
    let first = hits.next()?;
    hits.next().is_none().then_some(first)
}
fn equal_pressure(pressure: &[Value], minuend: &str, subtrahend: &str, member: &str) -> bool {
    let (Some(a), Some(b)) = (region_of(pressure, minuend, member), region_of(pressure, subtrahend, member)) else { return false };
    let entry = |r: &'_ Value, f: &str| items(&r[f]).iter().find(|e| e["pipe_id"] == member).cloned();
    let equal = crate::section_basis::bit_equal;
    ["p_pa", "external_pressure_increment_pa", "pressure_basis"].iter().all(|f| a.get(*f).is_some() && b.get(*f).is_some() && equal(&a[*f], &b[*f]))
        && ["geometry", "materials"].iter().all(|f| matches!((entry(a, f), entry(b, f)), (Some(x), Some(y)) if equal(&x, &y)))
}

fn within_message(id: &str, subtraction: bool, e: usize, n: usize, a: usize, u: usize) -> String {
    let states = if subtraction { "the difference of two independently solved states" } else { "a combination of independently solved states" };
    format!(
        "combination {id}: {e} derived row group(s) are established within their stated bound against the operand cases' intended systems; \
{n} are not established (the bound exceeds the 1e-9 criterion, as where the operands nearly cancel, or the comparison is unavailable); \
{a} are not assessed (below the floor); {u} of its rows are covered by no group and remain inspection results. \
This is a numerical statement about {states}; it does not establish the meaning of \"expansion\" or engineering acceptance, \
and these rows do not count toward the document's numerical eligibility. Each group's bound and verdict are stated in contract_evidence.combination_certificate."
    )
}
fn not_assessed_message(id: &str, reason: &str) -> String {
    format!("combination {id}: its rows are not assessed against a stated bound ({reason}); they remain inspection results and do not count toward the document's numerical eligibility.")
}
fn diagnostic(id: &str, code: &str, message: String) -> Value {
    json!({"id": format!("{DIAGNOSTIC_PREFIX}{}", identity(&[id])), "code": code, "severity": "info", "message": message, "source": SOURCE, "affected_refs": [id]})
}

struct Derived {
    json: Value,
    rows: Vec<usize>,
    bound: f64,
    floor_scale: f64,
}

/// The document's gates: (combination id, withheld) in evaluation order, and each
/// combination's stated expression record, from `combination_gates` (PF) or
/// `exact_combinations` (EX).
fn gates(evidence: &Value) -> Option<Vec<(String, bool)>> {
    if let Some(gates) = evidence.get("combination_gates").and_then(Value::as_array) {
        return Some(gates.iter().map(|g| (g["combination_id"].as_str().unwrap_or_default().to_string(), g["withheld"] == true)).collect());
    }
    evidence.get("exact_combinations").and_then(Value::as_array).map(|records| {
        records.iter().map(|r| (r["combination_id"].as_str().unwrap_or_default().to_string(), r["gate"]["withheld"] == true)).collect()
    })
}

/// The member's checks; a no-op on a document without it and without a combination.
pub(crate) fn check(source: &Value) -> Check {
    let evidence = &source["contract_evidence"];
    let solved = source["status"]["mechanics"] == "MECHANICS_SOLVED";
    let gate_list = gates(evidence).unwrap_or_default();
    let member = evidence.get(MEMBER);
    require(member.is_some() == (solved && !gate_list.is_empty()), "PRESENCE")?;
    let Some(member) = member else { return Ok(()) };
    require(keys(member, &MEMBER_KEYS), "SHAPE")?;
    let results = items(&source["results"]);
    let diagnostics = items(&source["diagnostics"]);
    let rows = Rows::new(results);
    let case_order: Vec<&str> = items(&source["numerical_quality"]["cases"])
        .iter()
        .filter(|c| c["basis_ref"]["ref_type"] == "load_case")
        .filter_map(|c| c["basis_ref"]["ref_id"].as_str())
        .collect();
    let cases: HashSet<&str> = case_order.iter().copied().collect();
    let pressure = items(&evidence["pressure"]);

    // The operand records: shape, uniqueness, coverage and binding.
    let mut operands: HashMap<&str, Vec<OperandGroup>> = HashMap::new();
    for case in items(&member["operand_cases"]) {
        require(keys(case, &CASE_KEYS), "SHAPE")?;
        let id = case["load_case_id"].as_str().ok_or_else(|| fail("SHAPE"))?;
        require(cases.contains(id) && !operands.contains_key(id), "OPERAND_CASE")?;
        require(eligible(diagnostics, id), "OPERAND_ELIGIBILITY")?;
        let mut groups = Vec::new();
        let mut seen = HashSet::new();
        for g in items(&case["groups"]) {
            require(keys(g, &OPERAND_GROUP_KEYS), "SHAPE")?;
            let (Some(entity), Some(location), Some(group), Some(moment), Some(q)) =
                (g["entity"].as_str(), g["location"].as_str(), g["group"].as_str(), g["moment"].as_bool(), g["q"].as_f64())
            else {
                return Err(fail("SHAPE"));
            };
            require(seen.insert((entity, location, group)), "OPERAND_GROUP_DUPLICATE")?;
            require(moment_class(group).is_none_or(|m| m == moment), "OPERAND_GROUP_CLASS")?;
            let floor_scale = g["floor_scale"].as_f64().ok_or_else(|| fail("SHAPE"))?;
            let bound = match &g["bound"] {
                Value::Null => None,
                b => Some(b.as_f64().ok_or_else(|| fail("SHAPE"))?),
            };
            require(q.is_finite() && q >= 0.0 && floor_scale.is_finite() && floor_scale >= 0.0 && bound.is_none_or(|b| b.is_finite() && b >= 0.0), "OPERAND_GROUP_VALUE")?;
            // The covered rows are re-derived and must be the stated ones (the operand-bound binding).
            let expected: Vec<Value> = rows
                .coverage(id, entity, group, location, q)
                .into_iter()
                .map(|set| set.map_or(Value::Null, |r| json!(r.iter().map(|&i| rows.id(i)).collect::<Vec<_>>())))
                .collect();
            require(crate::section_basis::bit_equal(&g["rows"], &json!(expected)), "OPERAND_ROWS")?;
            groups.push(OperandGroup { entity, location, group, q, bound, floor_scale });
        }
        operands.insert(id, groups);
    }

    // The combination records, re-derived whole.
    let records = items(&member["combinations"]);
    require(records.len() == gate_list.len(), "RECORDS")?;
    let mut used: HashSet<String> = HashSet::new();
    let mut expected_diagnostics: Vec<Value> = Vec::new();
    for (record, (id, withheld)) in records.iter().zip(&gate_list) {
        require(keys(record, &RECORD_KEYS) && record["combination_id"] == id.as_str(), "RECORD_SHAPE")?;
        // The stated expression is the evidence's own (EX record; PF operand record when present).
        for stated in items(&evidence["exact_combinations"]).iter().chain(items(&evidence["combination_operands"])).filter(|r| r["combination_id"] == id.as_str()) {
            require(crate::section_basis::bit_equal(&stated["expression"], &record["expression"]) && stated["basis"] == record["basis"], "EXPRESSION")?;
        }
        let rows_of = rows.of_combination.get(id.as_str()).cloned().unwrap_or_default();
        let assessed = if let Some(reason) = held_reason(&record["subtrahend_support_state"])? {
            Err(reason)
        } else if *withheld {
            Err("withheld by its gate".to_string())
        } else {
            terms_of(record, &cases).map_err(str::to_string).and_then(|terms| {
                let mut missing: Vec<&str> = Vec::new();
                for (case, _) in &terms {
                    if !eligible(diagnostics, case) && !missing.contains(&case.as_str()) {
                        missing.push(case);
                    }
                }
                if missing.is_empty() { Ok(terms) } else { Err(format!("operand case(s) {} carry no certified rows", missing.join(", "))) }
            })
        };
        let terms = match assessed {
            Ok(terms) => terms,
            Err(reason) => {
                if !*withheld {
                    expected_diagnostics.push(diagnostic(id, NOT_ASSESSED, not_assessed_message(id, &reason)));
                }
                let expected = json!({"combination_id": id, "basis": record["basis"], "expression": record["expression"],
                    "subtrahend_support_state": record["subtrahend_support_state"], "assessment": "not_assessed",
                    "reason": reason, "groups": [], "uncovered_row_count": rows_of.len()});
                require(crate::section_basis::bit_equal(record, &expected), "RECORD")?;
                continue;
            }
        };
        for (case, _) in &terms {
            require(operands.contains_key(case.as_str()), "OPERAND_CASE_MISSING")?;
            used.insert(case.clone());
        }
        let subtraction = record["basis"] == "result_state_subtraction";
        let mut derived: Vec<Derived> = Vec::new();
        let mut formed: HashMap<(String, String, String, usize), usize> = HashMap::new();
        for group in &operands[terms[0].0.as_str()] {
            let magnitude = match group.group {
                "support_force_magnitude" => Some(("support_force", SUPPORT_FORCE)),
                "support_moment_magnitude" => Some(("support_moment", SUPPORT_MOMENT)),
                _ => None,
            };
            if let Some((components, kind)) = magnitude {
                let Some(&index) = formed.get(&(group.entity.to_string(), group.location.to_string(), components.to_string(), 0)) else { continue };
                let component = &derived[index];
                let candidates: Vec<usize> = rows_of.iter().copied().filter(|&r| results[r]["kind"] == kind && results[r]["entity_ref"] == group.entity).collect();
                let [row] = candidates.as_slice() else { continue };
                let values: Vec<f64> = component.rows.iter().map(|&r| rows.value(r)).collect();
                let value = rows.value(*row);
                if values.len() != 3 || norm3(values[0], values[1], values[2]).to_bits() != value.to_bits() {
                    continue;
                }
                let root = if value == 0.0 { 0.0 } else { value.next_up() - value };
                let bound = up_add(component.bound, root);
                let q = value.abs();
                let q_hat = q.max(up_add(q, bound));
                let (v, reason) = verdict(q, q_hat, bound, component.floor_scale);
                derived.push(Derived {
                    json: json!({"entity": group.entity, "location": group.location, "group": group.group, "row_set": 0, "rows": [rows.id(*row)],
                        "factors_bound": number(component.bound), "rounding_error": number(root), "bound": number(bound), "q": number(q), "q_hat": number(q_hat),
                        "floor_scale": number(component.floor_scale), "verdict": v, "reason": reason, "equal_pressure": Value::Null}),
                    rows: vec![*row],
                    bound,
                    floor_scale: component.floor_scale,
                });
                continue;
            }
            let mut per_operand = Vec::new();
            for (case, _) in &terms {
                let Some(own) = operands[case.as_str()].iter().find(|g| g.entity == group.entity && g.location == group.location && g.group == group.group) else { break };
                let Some(bound) = own.bound else { break };
                per_operand.push((own, bound, rows.coverage(case, own.entity, own.group, own.location, own.q)));
            }
            if per_operand.len() != terms.len() {
                continue;
            }
            let sets = per_operand.iter().map(|(_, _, c)| c.len()).min().unwrap_or(0);
            for set in 0..sets {
                let Some(operand_rows) = per_operand.iter().map(|(_, _, c)| c[set].clone()).collect::<Option<Vec<Vec<usize>>>>() else { continue };
                let spec = |i: usize| -> Vec<(Value, Value)> { operand_rows[i].iter().map(|&r| (results[r]["kind"].clone(), results[r]["metadata"]["location"].clone())).collect() };
                if (1..terms.len()).any(|i| spec(i) != spec(0)) {
                    continue;
                }
                let n = operand_rows[0].len();
                let mut found = Vec::with_capacity(n);
                for c in 0..n {
                    let sources: Vec<&str> = operand_rows.iter().map(|r| rows.id(r[c])).collect();
                    match rows.by_sources.get(&(id.as_str(), sources)).map(Vec::as_slice) {
                        Some([one]) => found.push(*one),
                        _ => break,
                    }
                }
                if found.len() != n {
                    continue;
                }
                let values: Vec<f64> = found.iter().map(|&r| rows.value(r)).collect();
                let errors: Vec<f64> = (0..n)
                    .map(|c| rounding_error(values[c], &terms.iter().zip(&operand_rows).map(|((_, f), r)| (*f, rows.value(r[c]))).collect::<Vec<_>>()))
                    .collect();
                let mut factors_bound = 0.0;
                let mut floor_scale: f64 = 0.0;
                for ((_, f), (own, bound, _)) in terms.iter().zip(&per_operand) {
                    factors_bound = up_add(factors_bound, up_mul(f.abs(), *bound));
                    floor_scale = floor_scale.max(up_mul(f.abs(), own.floor_scale));
                }
                let error = up_norm(&errors);
                let bound = up_add(factors_bound, error);
                let q = group_norm(&values).unwrap_or(f64::NAN);
                let q_hat = q.max(up_add(q, bound));
                let (v, reason) = verdict(q, q_hat, bound, floor_scale);
                formed.insert((group.entity.to_string(), group.location.to_string(), group.group.to_string(), set), derived.len());
                derived.push(Derived {
                    json: json!({"entity": group.entity, "location": group.location, "group": group.group, "row_set": set,
                        "rows": found.iter().map(|&r| rows.id(r)).collect::<Vec<_>>(), "factors_bound": number(factors_bound), "rounding_error": number(error),
                        "bound": number(bound), "q": number(q), "q_hat": number(q_hat), "floor_scale": number(floor_scale), "verdict": v, "reason": reason,
                        "equal_pressure": Value::Null}),
                    rows: found,
                    bound,
                    floor_scale,
                });
            }
        }
        if subtraction && !pressure.is_empty() {
            for d in derived.iter_mut() {
                if d.json["group"] != "pipe_wall" {
                    continue;
                }
                let member = d.json["entity"].as_str().unwrap_or_default().to_string();
                if !equal_pressure(pressure, &terms[0].0, &terms[1].0, &member) {
                    continue;
                }
                let location = d.json["location"].as_str().unwrap_or_default().trim_start_matches("wall_").to_string();
                let elastic = (0..4)
                    .find(|set| formed.contains_key(&(member.clone(), location.clone(), "axial".to_string(), *set)))
                    .map(|set| json!({"entity": member, "location": location, "group": "axial", "row_set": set}))
                    .unwrap_or(Value::Null);
                d.json["equal_pressure"] = json!({"elastic_group": elastic});
            }
        }
        let covered: HashSet<usize> = derived.iter().flat_map(|d| d.rows.iter().copied()).collect();
        let uncovered = rows_of.iter().filter(|r| !covered.contains(r)).count();
        let count = |v: &str| derived.iter().filter(|d| d.json["verdict"] == v).count();
        expected_diagnostics.push(diagnostic(id, WITHIN, within_message(id, subtraction, count("established"), count("not_established"), count("not_assessed"), uncovered)));
        let expected = json!({"combination_id": id, "basis": record["basis"], "expression": record["expression"],
            "subtrahend_support_state": record["subtrahend_support_state"], "assessment": "assessed", "reason": Value::Null,
            "groups": derived.iter().map(|d| d.json.clone()).collect::<Vec<_>>(), "uncovered_row_count": uncovered});
        require(crate::section_basis::bit_equal(record, &expected), "RECORD")?;
    }
    // Exactly the cases the assessed combinations use, in the document's case order.
    let stated: Vec<&str> = items(&member["operand_cases"]).iter().filter_map(|c| c["load_case_id"].as_str()).collect();
    let wanted: Vec<&str> = case_order.iter().copied().filter(|c| used.contains(*c)).collect();
    require(stated == wanted, "OPERAND_CASES")?;
    // The diagnostics, whole, and none other in the namespace.
    let published: Vec<&Value> = diagnostics.iter().filter(|d| d["id"].as_str().is_some_and(|i| i.starts_with(DIAGNOSTIC_PREFIX))).collect();
    require(published.len() == expected_diagnostics.len(), "DIAGNOSTICS")?;
    for expected in &expected_diagnostics {
        let matching: Vec<&&Value> = published.iter().filter(|d| d["id"] == expected["id"]).collect();
        require(matching.len() == 1 && crate::section_basis::bit_equal(matching[0], expected), "DIAGNOSTICS")?;
    }
    Ok(())
}

/// With the captured model: each record's id, basis and expression are the model's.
pub(crate) fn model_checks(source: &Value, model: &Value) -> Check {
    let Some(member) = source["contract_evidence"].get(MEMBER) else { return Ok(()) };
    let combinations = items(&model["combinations"]);
    for record in items(&member["combinations"]) {
        let id = record["combination_id"].as_str().unwrap_or_default();
        let c = combinations.iter().find(|c| c["id"] == id).ok_or_else(|| fail("MODEL_BINDING"))?;
        require(c["basis"] == record["basis"], "MODEL_BINDING")?;
        let expression = match c["basis"].as_str() {
            Some("mechanics" | "absolute_sum" | "srss") => json!({"terms": items(&c["terms"]).iter().map(|t| match t.get("combination").filter(|v| !v.is_null()) {
                Some(op) => json!({"combination_id": op, "factor": t["factor"]}),
                None => json!({"load_case_id": t["load_case"], "factor": t["factor"]}),
            }).collect::<Vec<_>>()}),
            Some("result_state_subtraction") => json!({"minuend_id": c["minuend_id"], "subtrahend_id": c["subtrahend_id"]}),
            _ => json!({"operand_ids": c["operand_ids"], "mode": c["mode"]}),
        };
        require(crate::section_basis::bit_equal(&record["expression"], &expression), "MODEL_BINDING")?;
        require(record["subtrahend_support_state"] == support_token(c), "MODEL_BINDING")?;
    }
    // A combination withheld before the solve (S1's interim, document_members) has no
    // gate and so no record; the records are the document's gates (`check`).
    Ok(())
}

/// The document without the member, for the unchanged route readers.
pub(crate) fn without_member(source: &Value) -> Option<Value> {
    source["contract_evidence"].get(MEMBER)?;
    let mut view = source.clone();
    if let Some(evidence) = view["contract_evidence"].as_object_mut() {
        evidence.remove(MEMBER);
    }
    Some(view)
}

