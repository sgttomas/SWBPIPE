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
/// The reserved code family of the certificate's notices.
const CODE_PREFIX: &str = "COMBINATION_ROWS_";
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
/// A member array, refused by its shape when it is anything but an array.
fn array<'v>(v: &'v Value) -> Result<&'v [Value], String> {
    v.as_array().map(Vec::as_slice).ok_or_else(|| fail("SHAPE"))
}
/// Whether a diagnostic is in the certificate's closed reserved vocabulary (Work
/// `combination-certificate.md` §2), whatever its id spelling: its id is in the
/// certificate's namespace, its code in the reserved `COMBINATION_ROWS_` family, or its
/// message states rows within, or not assessed against, a stated bound. Such a notice
/// stands only as the one a valid member re-derives.
fn reserved_notice(d: &Value) -> bool {
    let text = |k: &str| d[k].as_str().unwrap_or_default();
    let message = text("message");
    text("id").starts_with(DIAGNOSTIC_PREFIX)
        || text("code").starts_with(CODE_PREFIX)
        || (message.contains("stated bound") && (message.contains("within") || message.contains("against")))
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

/// The member's checks (a no-op on a document without it), then the closed combination-diagnostic
/// roster, without the model.
pub(crate) fn check(source: &Value) -> Check {
    check_with(source, None)
}

/// The member's checks, then the closed combination-diagnostic roster (with the model when the
/// caller has it: S1's interim withholdings and the headline's authored order are then the model's).
pub(crate) fn check_with(source: &Value, model: Option<&Value>) -> Check {
    member_check(source)?;
    roster(source, model)
}

/// The member's checks; a no-op on a document without it.
fn member_check(source: &Value) -> Check {
    let evidence = &source["contract_evidence"];
    let solved = source["status"]["mechanics"] == "MECHANICS_SOLVED";
    let gate_list = gates(evidence).unwrap_or_default();
    let member = evidence.get(MEMBER);
    // Absence of the member is no §4 certificate claim, whatever the document's age; it
    // selects no historical semantics (only a recorded `{id, sha256}` binding does). The
    // combination rows are then inspection results and no certificate notice may stand,
    // under any id. A stated member is read in full, and only on a solved document with a
    // combination; a partial or malformed one is refused, never read as absence.
    let Some(member) = member else {
        return require(!items(&source["diagnostics"]).iter().any(reserved_notice), "PRESENCE");
    };
    require(solved && !gate_list.is_empty(), "PRESENCE")?;
    require(keys(member, &MEMBER_KEYS), "SHAPE")?;
    let operand_cases = array(&member["operand_cases"])?;
    let records = array(&member["combinations"])?;
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
    for case in operand_cases {
        require(keys(case, &CASE_KEYS), "SHAPE")?;
        let id = case["load_case_id"].as_str().ok_or_else(|| fail("SHAPE"))?;
        require(cases.contains(id) && !operands.contains_key(id), "OPERAND_CASE")?;
        require(eligible(diagnostics, id), "OPERAND_ELIGIBILITY")?;
        let mut groups = Vec::new();
        let mut seen = HashSet::new();
        for g in array(&case["groups"])? {
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
    require(records.len() == gate_list.len(), "RECORDS")?;
    let mut used: HashSet<String> = HashSet::new();
    let mut expected_diagnostics: Vec<Value> = Vec::new();
    for (record, (id, withheld)) in records.iter().zip(&gate_list) {
        require(keys(record, &RECORD_KEYS) && record["combination_id"] == id.as_str(), "RECORD_SHAPE")?;
        // The stated expression is the EX record's; on PF it is bound by the model (`model_checks`),
        // as slice 1a's operand records are.
        for stated in items(&evidence["exact_combinations"]).iter().filter(|r| r["combination_id"] == id.as_str()) {
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
    let stated: Vec<&str> = operand_cases.iter().filter_map(|c| c["load_case_id"].as_str()).collect();
    let wanted: Vec<&str> = case_order.iter().copied().filter(|c| used.contains(*c)).collect();
    require(stated == wanted, "OPERAND_CASES")?;
    // The diagnostics, whole, and none other in the reserved vocabulary (any id spelling).
    let published: Vec<&Value> = diagnostics.iter().filter(|d| reserved_notice(d)).collect();
    require(published.len() == expected_diagnostics.len(), "DIAGNOSTICS")?;
    for expected in &expected_diagnostics {
        let matching: Vec<&&Value> = published.iter().filter(|d| d["id"] == expected["id"]).collect();
        require(matching.len() == 1 && crate::section_basis::bit_equal(matching[0], expected), "DIAGNOSTICS")?;
    }
    Ok(())
}

// ------------------------------------------------------------------ the closed combination-diagnostic roster (Work §2)
//
// A diagnostic is a combination diagnostic when an affected ref names a combination identity of the
// document (`combination_population`), whatever its id or code. Each must be an entry of the closed
// roster: its id names one entry, and its code, severity, message, source and refs are that entry's
// canonical form, derived from the evidence. The reserved certificate codes and canonical notices stay
// protected by `check` whatever their id; that protection is of the known codes and canonical texts,
// not a semantic reading of arbitrary prose, which the roster instead refuses under a combination ref.

const HEADLINE_ID: &str = "diagnostic:preview-physics:headline-scope";
const HEADLINE: &str = "PREVIEW_HEADLINE_SCOPE_LOAD_CASES";
const HEADLINE_TEXT: &str = "headlines cover the load cases only; combinations are excluded";
const MAXIMUM: &str = "COMBINATION_STRESS_MAXIMUM_UNAVAILABLE";
const MAXIMUM_TEXT: &str = "stress maxima and section stress extremes are not combined; combine the signed stress component rows instead";
const INTENSIFIED: &str = "COMBINATION_INTENSIFIED_STRESS_UNAVAILABLE";
const INTENSIFIED_TEXT: &str = "intensified measures are never combined (their ranges and combinations follow the owner's rule in T6)";
/// The producer's `has_maximum` and `has_intensified` row kinds (`preview_physics.rs`).
const MAXIMUM_KINDS: [&str; 3] = ["pipe_elastic_normal_stress_maximum_v2", "element_local_normal_stress_extreme", "element_local_transverse_shear_stress_maximum"];
const INTENSIFIED_KINDS: [&str; 3] = [
    "component_equal_factor_intensified_bending_stress_v1",
    "curved_bend_directional_intensified_bending_stress_v1",
    "curved_bend_directional_intensified_bending_maximum_v1",
];
const PREVIEW_PREFIX: &str = "diagnostic:preview-physics:combination-";
const EXACT_PREFIX: &str = "diagnostic:exact-combination:";
const INTERIM_PREFIX: &str = "diagnostic:combination:";
const RETAINED_PREFIX: &str = "diagnostic:retained-precision:";
const SOURCE_MISSING: &str = "LOAD_COMBINATION_SOURCE_RESULT_MISSING";
const ROW_WITHHELD: &str = "LOAD_COMBINATION_ROW_WITHHELD";
const VECTOR_MISSING: &str = "LOAD_COMBINATION_VECTOR_COMPONENT_MISSING";
const VECTOR_INVALID: &str = "LOAD_COMBINATION_VECTOR_INVALID";
const DIFFERENCE_BASIS: &str = "LOAD_COMBINATION_DIFFERENCE_BASIS_UNSUPPORTED";
const SUBTRAHEND_STATE: &str = "LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED";
const OPERAND_STATES: &str = "COMBINATION_OPERAND_SUPPORT_STATES_DIFFER";
const OPERAND_STANDING: &str = "COMBINATION_OPERAND_CHECKS_NOT_PASSED";
const RETAINED_SELECTED: &str = "RETAINED_PRECISION_SELECTED";
const RETAINED_UNAVAILABLE: &str = "RETAINED_PRECISION_UNAVAILABLE";
/// PP `retained_wire.rs` `COMBINATION_SELECTED_MESSAGE` and `COMBINATION_UNAVAILABLE_MESSAGE`.
const RETAINED_SELECTED_TEXT: &str = "Retained-precision recovery (contribution_preserving_multiprecision_v1) is selected for this load combination, from its own solve and certificate. Its published rows carry recovery_method; the retained_precision receipt binds their certified classes, the native attempts and work, and its operands' sources.";
const RETAINED_UNAVAILABLE_TEXT: &str = "Retained-precision recovery is unavailable for this load combination. Its published rows keep their ordinary values and diagnostics and are withheld from rule binding; the retained_precision receipt records the actual attempt and its typed cause.";
const WITHHELD_REASONS: [&str; 3] = ["no algebra dimension", "load-case algebra refused", "source support row missing in an operand case"];
/// Every code a roster entry carries (the certificate's own are `check`'s): a diagnostic with one of
/// these under a combination ref and an id that names no entry is a renamed notice.
const ROSTER_CODES: [&str; 25] = [
    HEADLINE,
    MAXIMUM,
    INTENSIFIED,
    "NONLINEAR_COMBINATION_REQUIRES_SOLVE",
    "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE",
    "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND",
    "CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED",
    "COMBINATION_MODULUS_BASIS_MIXED",
    "COMBINATION_RESOLVED_STIFFNESS_MIXED",
    "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE",
    "INSTALLED_REFERENCE_UNSIGNED_COMBINATION_OPERAND",
    crate::combination_graph::OPERAND_WITHHELD,
    crate::combination_graph::FIXED_LOAD_COUNT_UNREPRESENTABLE,
    SOURCE_MISSING,
    ROW_WITHHELD,
    VECTOR_MISSING,
    VECTOR_INVALID,
    DIFFERENCE_BASIS,
    SUBTRAHEND_STATE,
    OPERAND_STATES,
    OPERAND_STANDING,
    RETAINED_SELECTED,
    RETAINED_UNAVAILABLE,
    "EXACT_COMBINATION_DERIVED_FROM_CASE_RESULTS",
    "EXACT_COMBINATION_SOURCE_RECOVERY_EXCLUDED",
];

fn stable_suffix(id: &str) -> String {
    id.replace(':', "-")
}
/// The producer's `gate_message` on preview-physics-2 (`preview_physics.rs`).
fn preview_gate_message(code: &str) -> &'static str {
    match code {
        "NONLINEAR_COMBINATION_REQUIRES_SOLVE" => "Combination withheld: the model has nonlinear supports, so superposing case states is not a solved state.",
        "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE" => "Combination withheld: a constant-effort support would be counted by the sum of factors instead of once.",
        "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND" => "Combination withheld: a constant-effort support's force acts in every case state; an absolute sum or SRSS of operands whose fixed-load count is not exactly 0 would count it once per term with its sign discarded.",
        "CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED" => "Combination withheld: an operand's fixed-load count is undefined, so the number of times this combination counts a constant-effort support's force cannot be decided.",
        _ => "Combination withheld: its load cases were solved on different modulus bases.",
    }
}
/// The producer's `operand_withheld_message` (both routes).
fn operand_withheld_message(named: &[(&str, &str)]) -> String {
    let named = named.iter().map(|(id, code)| format!("{id} ({code})")).collect::<Vec<_>>().join(", ");
    format!("Combination withheld: its operand combination(s) {named} are withheld, so it is withheld whole.")
}
fn roster_entry(id: String, code: &str, severity: &str, message: String, refs: Vec<&str>) -> Value {
    json!({"id": id, "code": code, "severity": severity, "message": message, "source": SOURCE, "affected_refs": refs})
}
fn refs_of(d: &Value) -> Vec<&str> {
    items(&d["affected_refs"]).iter().filter_map(Value::as_str).collect()
}

/// The combination identities the document states (and, with it, the model's): its gates, its EX
/// records, its certificate and operand records, and the owners of its combination-typed rows and
/// numerical cases. An identity that is also another entity's (a load case, a row's entity, another
/// model record) is not recognizable as a combination's in an untyped ref, and is left out.
fn combination_population<'s>(source: &'s Value, model: Option<&'s Value>) -> HashSet<&'s str> {
    let evidence = &source["contract_evidence"];
    let typed = |b: &'s Value| if b["ref_type"] == "combination" { b["ref_id"].as_str() } else { None };
    let mut ids: HashSet<&str> = HashSet::new();
    for member in ["combination_gates", "exact_combinations", "combination_operands"] {
        ids.extend(items(&evidence[member]).iter().filter_map(|r| r["combination_id"].as_str()));
    }
    ids.extend(items(&evidence[MEMBER]["combinations"]).iter().filter_map(|r| r["combination_id"].as_str()));
    ids.extend(items(&source["results"]).iter().filter_map(|r| typed(&r["basis_ref"])));
    ids.extend(items(&source["numerical_quality"]["cases"]).iter().filter_map(|c| typed(&c["basis_ref"])));
    let mut other: HashSet<&str> = HashSet::new();
    other.extend(items(&source["numerical_quality"]["cases"]).iter().filter(|c| typed(&c["basis_ref"]).is_none()).filter_map(|c| c["basis_ref"]["ref_id"].as_str()));
    for member in ["preview_cases", "exact_cases"] {
        other.extend(items(&evidence[member]).iter().filter_map(|c| c["load_case_id"].as_str()));
    }
    other.extend(items(&source["results"]).iter().filter_map(|r| r["entity_ref"].as_str()));
    if let Some(model) = model {
        ids.extend(items(&model["combinations"]).iter().filter_map(|c| c["id"].as_str()));
        for (key, list) in model.as_object().into_iter().flatten() {
            if key != "combinations" {
                other.extend(items(list).iter().filter_map(|e| e["id"].as_str()));
            }
        }
    }
    ids.retain(|id| !id.is_empty() && !other.contains(id));
    ids
}

/// S1's interim withholding (PP `document_members::withhold_interim_combinations`) as the model states
/// it: the notices, and the model's combinations it leaves, in authored order.
fn interim<'m>(model: &'m Value) -> (Vec<Value>, Vec<&'m Value>) {
    use crate::combination_graph::{model_operands, model_order, Operand, OPERAND_WITHHELD};
    let all = items(&model["combinations"]);
    let cases: HashSet<&str> = items(&model["load_cases"]).iter().filter_map(|c| c["id"].as_str()).collect();
    let mut notices = Vec::new();
    let mut withheld: HashMap<&str, &str> = HashMap::new();
    for c in all {
        let id = c["id"].as_str().unwrap_or_default();
        let mut reasons: Vec<(&str, String)> = Vec::new();
        if c.get("difference_basis").is_some_and(|b| !b.is_null()) {
            reasons.push((DIFFERENCE_BASIS, format!("combination `{id}` declares a difference basis (operating minus sustained); its gates are not built yet, so the combination is withheld")));
        }
        if c["subtrahend_support_state"] == "minuend_final" {
            reasons.push((SUBTRAHEND_STATE, format!("combination `{id}` solves its subtrahend at the minuend's final support status; that solve is not built yet, so the combination is withheld")));
        }
        for (code, message) in &reasons {
            notices.push(roster_entry(format!("{INTERIM_PREFIX}{}:{}", stable_suffix(id), code.to_ascii_lowercase()), code, "warning", message.clone(), vec![id]));
        }
        if let Some((code, _)) = reasons.first() {
            withheld.insert(id, *code);
        }
    }
    if !withheld.is_empty() {
        for index in model_order(all, &cases) {
            let c = &all[index];
            let id = c["id"].as_str().unwrap_or_default();
            if withheld.contains_key(id) {
                continue;
            }
            let mut named: Vec<(&str, &str)> = Vec::new();
            for operand in model_operands(c, &cases) {
                if let Operand::Combination(o) = operand {
                    if let Some(code) = withheld.get(o) {
                        if !named.iter().any(|(n, _)| *n == o) {
                            named.push((o, *code));
                        }
                    }
                }
            }
            if named.is_empty() {
                continue;
            }
            let mut refs = vec![id];
            refs.extend(named.iter().map(|(n, _)| *n));
            notices.push(roster_entry(format!("{INTERIM_PREFIX}{}:{}", stable_suffix(id), OPERAND_WITHHELD.to_ascii_lowercase()), OPERAND_WITHHELD, "warning", operand_withheld_message(&named), refs));
            withheld.insert(id, OPERAND_WITHHELD);
        }
    }
    let kept = all.iter().filter(|c| !c["id"].as_str().is_some_and(|id| withheld.contains_key(id))).collect();
    (notices, kept)
}

/// A `LOAD_COMBINATION_ROW_WITHHELD` list (`preview_physics.rs`): `count` withheld rows, the first
/// eight named, each name one of the evidence's row bases or supports with a producer reason.
fn withheld_names(named: &str, count: usize, tokens: &HashSet<String>) -> bool {
    fn split(rest: &str, left: usize, tokens: &HashSet<String>) -> bool {
        rest.match_indices(')').any(|(at, _)| {
            let (token, after) = (&rest[..=at], &rest[at + 1..]);
            tokens.contains(token)
                && if left == 1 { after.is_empty() } else { after.strip_prefix(", ").is_some_and(|next| split(next, left - 1, tokens)) }
        })
    }
    let body = if count > 8 { named.strip_suffix(&format!(", and {} more", count - 8)) } else { Some(named) };
    count > 0 && body.is_some_and(|body| split(body, count.min(8), tokens))
}

/// The canonical form of a form-bound entry (its parameters read from its id and refs, each bound to
/// the evidence), `Some(Null)` when the id is in such a family but names no evidence-bound entry, and
/// `None` when the id is in no form-bound family.
fn form_bound(d: &Value, preview: bool, index: &crate::preview_combination_gates::RowIndex<'_>, supports: &HashSet<&str>, case_ids: &HashSet<&str>, population: &HashSet<&str>) -> Option<Value> {
    let id = d["id"].as_str()?;
    let code = d["code"].as_str().unwrap_or_default();
    let refs = refs_of(d);
    let cid = refs.first().copied().filter(|c| population.contains(c));
    if let Some(rest) = id.strip_prefix(RETAINED_PREFIX) {
        let cid = cid?;
        let canonical = match rest.strip_prefix(cid) {
            Some(":selected") => roster_entry(id.to_string(), RETAINED_SELECTED, "info", RETAINED_SELECTED_TEXT.into(), vec![cid]),
            Some(":unavailable") => roster_entry(id.to_string(), RETAINED_UNAVAILABLE, "info", RETAINED_UNAVAILABLE_TEXT.into(), vec![cid]),
            _ => Value::Null,
        };
        return Some(canonical);
    }
    if !preview {
        return None;
    }
    if let Some(rest) = id.strip_prefix(PREVIEW_PREFIX) {
        let Some(cid) = cid else { return Some(Value::Null) };
        let own = identity(&[cid]);
        let canonical = if rest == format!("fixed-load-count:{own}") {
            roster_entry(id.to_string(), crate::combination_graph::FIXED_LOAD_COUNT_UNREPRESENTABLE, "info", format!("combination {cid}: its fixed-load count (the number of times it carries each fixed load) needs more than 4,096 significand bits exactly, so it is undefined; a gated use never treats an undefined count as passing"), vec![cid])
        } else if let Some(base) = rest.strip_prefix(&format!("source-missing:{own}")).and_then(|b| b.split_once(':')).filter(|(n, b)| n.parse::<usize>().ok() == Some(b.len()) && n.parse::<usize>().ok().map(|x| x.to_string()).as_deref() == Some(*n) && index.cases.contains_key(*b)).map(|(_, b)| b) {
            roster_entry(id.to_string(), SOURCE_MISSING, "warning", format!("combination {cid} omits {base} because a matching source row is missing in an operand case"), vec![cid])
        } else if rest == format!("row-withheld:{own}") {
            let mut tokens: HashSet<String> = HashSet::new();
            for base in index.cases.keys() {
                tokens.extend(WITHHELD_REASONS[..2].iter().map(|r| format!("{base} ({r})")));
            }
            for s in supports {
                tokens.extend(WITHHELD_REASONS[1..].iter().map(|r| format!("support-action:{}:{s} ({r})", s.len())));
            }
            let message = d["message"].as_str().unwrap_or_default();
            let lead = format!("combination {cid} withholds ");
            let listed = message.strip_prefix(&lead).and_then(|m| m.split_once(" combined row(s) that could not be formed: ")).filter(|(n, named)| {
                n.parse::<usize>().ok().is_some_and(|count| count.to_string() == *n && withheld_names(named, count, &tokens))
            });
            match listed {
                Some(_) => roster_entry(id.to_string(), ROW_WITHHELD, "warning", message.to_string(), vec![cid]),
                None => Value::Null,
            }
        } else {
            return None;
        };
        return Some(canonical);
    }
    if let (Some(rest), true) = (id.strip_prefix(INTERIM_PREFIX), matches!(code, VECTOR_MISSING | VECTOR_INVALID)) {
        let Some(cid) = cid else { return Some(Value::Null) };
        let middle = rest.strip_prefix(&format!("{}:", stable_suffix(cid))).and_then(|r| r.strip_suffix(&format!(":{}", stable_suffix(code))));
        let found = middle.and_then(|middle| {
            index.cases.iter().filter(|(base, _)| stable_suffix(base) == middle).find(|(base, rows)| match (code, refs.as_slice()) {
                (VECTOR_MISSING, [_, target, entity, b]) => {
                    *b == base.as_str() && (case_ids.contains(*target) || population.contains(*target)) && rows.values().any(|r| r["entity_ref"] == *entity)
                }
                (VECTOR_INVALID, [_, row, b]) => *b == base.as_str() && rows.values().any(|r| r["id"] == *row),
                _ => false,
            })
        });
        let message = if code == VECTOR_MISSING {
            "combined magnitude withheld because a signed source vector component is unavailable"
        } else {
            "combined magnitude withheld because signed vector algebra failed"
        };
        return Some(match found {
            Some(_) => roster_entry(id.to_string(), code, "warning", message.into(), refs),
            None => Value::Null,
        });
    }
    None
}

/// The closed roster of a solved document's combination diagnostics (Work `combination-certificate.md`
/// §2): `ROSTER_EXTRA`, `ROSTER_RENAMED`, `ROSTER_CONTRADICTION` and `ROSTER_MISSING`. Delegated to the
/// readers that re-derive them whole: the certificate's own notices (`check`), EX's namespace
/// (`exact_combination_evidence`) and PF's operand notices (`preview_combination_operands`).
fn roster(source: &Value, model: Option<&Value>) -> Check {
    if source["status"]["mechanics"] != "MECHANICS_SOLVED" {
        return Ok(());
    }
    let population = combination_population(source, model);
    if population.is_empty() {
        return Ok(());
    }
    let evidence = &source["contract_evidence"];
    let preview = evidence.get("combination_gates").is_some();
    let exact = evidence.get("exact_combinations").is_some();
    let results = items(&source["results"]);
    let diagnostics = items(&source["diagnostics"]);
    let mut case_ids: HashSet<&str> = items(&source["numerical_quality"]["cases"]).iter().filter(|c| c["basis_ref"]["ref_type"] == "load_case").filter_map(|c| c["basis_ref"]["ref_id"].as_str()).collect();
    if let Some(model) = model {
        case_ids.extend(items(&model["load_cases"]).iter().filter_map(|c| c["id"].as_str()));
    }
    let named = |d: &Value| refs_of(d).iter().any(|r| population.contains(r));

    // The set-bound entries, each required once.
    let (mut expected, kept) = match model {
        Some(model) => {
            let (notices, kept) = interim(model);
            (notices, Some(kept))
        }
        None => (Vec::new(), None),
    };
    let mut headline: Option<usize> = None;
    let gates: Vec<(&str, bool, &str)> = items(&evidence["combination_gates"])
        .iter()
        .map(|g| (g["combination_id"].as_str().unwrap_or_default(), g["withheld"] == true, g["reason"].as_str().unwrap_or_default()))
        .collect();
    if preview && !gates.is_empty() {
        let refs: Vec<&str> = match &kept {
            Some(kept) => kept.iter().filter_map(|c| c["id"].as_str()).collect(),
            None => gates.iter().map(|g| g.0).collect(),
        };
        headline = Some(expected.len());
        expected.push(roster_entry(HEADLINE_ID.into(), HEADLINE, "info", HEADLINE_TEXT.into(), refs));
        let case_row = |kinds: &[&str]| results.iter().any(|r| r["basis_ref"]["ref_type"] == "load_case" && r["kind"].as_str().is_some_and(|k| kinds.contains(&k)));
        let (has_maximum, has_intensified) = (case_row(&MAXIMUM_KINDS), case_row(&INTENSIFIED_KINDS));
        let model_cases: HashSet<&str> = model.map(|m| items(&m["load_cases"]).iter().filter_map(|c| c["id"].as_str()).collect()).unwrap_or_default();
        let mut withheld: HashMap<&str, &str> = HashMap::new();
        for &(id, gated, code) in &gates {
            let own = identity(&[id]);
            if !gated {
                if has_maximum {
                    expected.push(roster_entry(format!("{PREVIEW_PREFIX}maximum:{own}"), MAXIMUM, "info", MAXIMUM_TEXT.into(), vec![id]));
                }
                if has_intensified {
                    expected.push(roster_entry(format!("{PREVIEW_PREFIX}intensified:{own}"), INTENSIFIED, "info", INTENSIFIED_TEXT.into(), vec![id]));
                }
                continue;
            }
            let gate_id = format!("{PREVIEW_PREFIX}gate:{own}");
            let (message, refs) = if code == crate::combination_graph::OPERAND_WITHHELD {
                // The withheld operand combinations, in operand order: the model's, or as the notice names them.
                let operands: Vec<&str> = match kept.as_ref().and_then(|k| k.iter().find(|c| c["id"] == id)) {
                    Some(c) => crate::combination_graph::model_operands(c, &model_cases)
                        .into_iter()
                        .filter_map(|o| match o {
                            crate::combination_graph::Operand::Combination(o) => Some(o),
                            crate::combination_graph::Operand::Case(_) => None,
                        })
                        .collect(),
                    None => diagnostics.iter().find(|d| d["id"] == gate_id.as_str()).map(|d| refs_of(d).into_iter().skip(1).collect()).unwrap_or_default(),
                };
                let mut named_ops: Vec<(&str, &str)> = Vec::new();
                for o in operands {
                    if let Some(c) = withheld.get(o) {
                        if !named_ops.iter().any(|(n, _)| *n == o) {
                            named_ops.push((o, *c));
                        }
                    }
                }
                let mut refs = vec![id];
                refs.extend(named_ops.iter().map(|(n, _)| *n));
                (operand_withheld_message(&named_ops), refs)
            } else {
                (preview_gate_message(code).to_string(), vec![id])
            };
            expected.push(roster_entry(gate_id, code, "warning", message, refs));
            withheld.insert(id, code);
        }
    }

    let index = crate::preview_combination_gates::RowIndex::new(results);
    let supports: HashSet<&str> = results
        .iter()
        .filter(|r| r["basis_ref"]["ref_type"] == "load_case" && r["kind"].as_str().is_some_and(|k| k.starts_with("support_reaction_")))
        .filter_map(|r| r["entity_ref"].as_str())
        .collect();
    let mut seen = vec![false; expected.len()];
    for d in diagnostics.iter().filter(|d| named(d)) {
        let code = d["code"].as_str().unwrap_or_default();
        // Read whole by their own readers, which precede this one.
        if reserved_notice(d)
            || (preview && matches!(code, OPERAND_STATES | OPERAND_STANDING))
            || (exact && d["id"].as_str().is_some_and(|i| i.starts_with(EXACT_PREFIX)))
        {
            continue;
        }
        if let Some(at) = expected.iter().position(|e| e["id"] == d["id"]) {
            require(!seen[at], "ROSTER_EXTRA")?;
            seen[at] = true;
            let mut canonical = expected[at].clone();
            // Without the model the document does not fix the headline's authored order: its refs are
            // the gates' identities, each once, in any order.
            if Some(at) == headline && model.is_none() {
                let (mut stated, mut wanted) = (refs_of(d), refs_of(&canonical));
                stated.sort_unstable();
                wanted.sort_unstable();
                if stated == wanted && d["affected_refs"].as_array().is_some_and(|a| a.len() == wanted.len()) {
                    canonical["affected_refs"] = d["affected_refs"].clone();
                }
            }
            require(crate::section_basis::bit_equal(d, &canonical), "ROSTER_CONTRADICTION")?;
            continue;
        }
        match form_bound(d, preview, &index, &supports, &case_ids, &population) {
            Some(canonical) => require(crate::section_basis::bit_equal(d, &canonical), "ROSTER_CONTRADICTION")?,
            None => {
            // A known notice's code under an id outside every roster family is a renamed notice.
            let family = d["id"].as_str().is_some_and(|i| [PREVIEW_PREFIX, HEADLINE_ID, INTERIM_PREFIX, RETAINED_PREFIX, EXACT_PREFIX, DIAGNOSTIC_PREFIX].iter().any(|p| i.starts_with(p)));
            return Err(fail(if !family && ROSTER_CODES.contains(&code) { "ROSTER_RENAMED" } else { "ROSTER_EXTRA" }));
        }
        }
    }
    require(seen.iter().all(|s| *s), "ROSTER_MISSING")
}

/// With the captured model: each record's id, basis and expression are the model's.
pub(crate) fn model_checks(source: &Value, model: &Value) -> Check {
    let Some(member) = source["contract_evidence"].get(MEMBER) else { return Ok(()) };
    let combinations = items(&model["combinations"]);
    // A record that is not an object is left to `check`, which refuses it by its shape
    // (`RECORD_SHAPE`), so a malformed record has the same refusal with and without the model.
    for record in items(&member["combinations"]).iter().filter(|r| r.is_object()) {
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

