//! RT-A3: the §4 combination certificate (DEL-05-02 Work
//! `combination-certificate.md`; `operating-case.md` §4, option (i) without (ii)).
//!
//! For every derived group of a subtraction or mechanics combination over load
//! cases whose operand cases carry certified rows (their member bound was formed:
//! K-D5 §3.1), the combination's rows are established within the bound
//!
//!   B_D = RU(Σ|fᵢ|·Bᵢ + ‖v_D − Σ fᵢ·vᵢ‖),
//!
//! the last term formed exactly from the published rows, and judged by K-D5's own
//! 1e-9 predicate with q_D = ‖v_D‖, q̂_D = max(q_D, RU(q_D + B_D)) and the larger
//! factor-scaled operand coupled scale as the floor scale. Below-floor groups are
//! not assessed; everything else fails closed. Agent 0's decision (b), 2026-11-04:
//! an established derived group does not count toward the document's numerical
//! eligibility, which is unchanged.
//!
//! Readers: RE `combination_certificate.rs` and `tests/verifiers/combination_certificate.py`
//! re-derive every value bit for bit from the published rows and operand bounds.
use super::{diag, exact_combinations, exact_source_identity, formation_guard, Diagnostic, MechanicsEnvelope};
use open_pipe_stress_correct_norm::{norm2, norm3};
use open_pipe_stress_load_case_algebra::Dyadic;
use serde_json::{json, Value};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

/// The contract-evidence member.
pub(crate) const MEMBER: &str = "combination_certificate";
/// Info: an assessed combination's per-verdict counts.
pub(crate) const WITHIN: &str = "COMBINATION_ROWS_WITHIN_STATED_BOUND";
/// Info: a combination with rows whose rows are not assessed, with the reason.
pub(crate) const NOT_ASSESSED: &str = "COMBINATION_ROWS_NOT_ASSESSED";
const DIAGNOSTIC_PREFIX: &str = "diagnostic:combination-certificate:";

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

/// One K-D5 group of an operand case (`formation_guard::RecoveryGroup`), with the
/// coupled scale its predicate used.
#[derive(Debug, Clone)]
pub(crate) struct OperandGroup {
    entity: String,
    location: String,
    group: String,
    moment: bool,
    q: f64,
    /// `None`: the case's bound for this entity is unavailable (named in its demotion).
    bound: Option<f64>,
    floor_scale: f64,
}

/// An eligible operand case's groups: formed only when its member bound was
/// (pre-guard Passed, no load-row finding, linear Passed or a confirmed
/// nonlinear state), so a gap-bearing case has none until G2 confirms it.
#[derive(Debug, Clone)]
pub(crate) struct Operand {
    groups: Vec<OperandGroup>,
}

/// The operand record of one case from the records its unified predicate judged.
pub(crate) fn operand(
    records: &[formation_guard::RecoveryRecord],
    bodies: &formation_guard::Bodies,
) -> Operand {
    let scales = formation_guard::member_scales(records, bodies);
    let mut groups = Vec::new();
    for record in records {
        let scale = record.body.and_then(|body| scales.get(&body).copied()).unwrap_or_default();
        for group in &record.groups {
            let floor = if group.moment { scale.moment } else { scale.force };
            groups.push(OperandGroup {
                entity: record.member.clone(),
                location: group.location.clone(),
                group: group.kind.clone(),
                moment: group.moment,
                q: group.q,
                // A non-finite bound or scale is unusable: the group forms no derived group.
                bound: group.bound.as_ref().ok().copied().filter(|b| b.is_finite() && floor.is_finite()),
                floor_scale: floor,
            });
        }
    }
    Operand { groups }
}

// ------------------------------------------------------------------ coverage

/// One covered row: kind, location and (support components only) component.
type Spec = (&'static str, &'static str, Option<&'static str>);

fn static_location(location: &str) -> Option<&'static str> {
    ["end_i", "end_j", "quarter_1", "midspan", "quarter_3", "node"].into_iter().find(|l| *l == location)
}

/// The closed coverage table (Work `combination-certificate.md` §1): the row sets
/// of a group, in order. A row set is used only when its rows bind to the group's
/// q (`coverage`); on a pressurized straight EX member the elastic axial rows are
/// not published (its wall and effective rows are), so its end `axial` groups
/// have no row.
pub(crate) fn row_sets(group: &str, location: &str) -> Vec<Vec<Spec>> {
    let station = matches!(location, "quarter_1" | "midspan" | "quarter_3");
    let end = matches!(location, "end_i" | "end_j");
    let at = static_location(location);
    let pair = |a: &'static str, b: &'static str| at.map(|l| vec![(a, l, None), (b, l, None)]);
    let one = |a: &'static str| at.map(|l| vec![(a, l, None)]);
    let mut out: Vec<Vec<Spec>> = Vec::new();
    match group {
        "axial" => {
            if end || station {
                out.extend(one(AXIAL));
            }
            if station {
                out.extend(one(EFFECTIVE));
            }
            match location {
                "cut_i" => out.push(vec![(EFFECTIVE, "end_i", None)]),
                "cut_j" => out.push(vec![(EFFECTIVE, "end_j", None)]),
                _ => {}
            }
        }
        "shear" if end || station => out.extend(pair(SHEAR_Y, SHEAR_Z)),
        "torsion" if end || station => out.extend(one(TORSION)),
        "bending" if end || station => out.extend(pair(BENDING_Y, BENDING_Z)),
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
        "support_force" if location == "node" => {
            out.push(vec![(SUPPORT_COMPONENT, "node", Some("Fx")), (SUPPORT_COMPONENT, "node", Some("Fy")), (SUPPORT_COMPONENT, "node", Some("Fz"))])
        }
        "support_moment" if location == "node" => {
            out.push(vec![(SUPPORT_COMPONENT, "node", Some("Mx")), (SUPPORT_COMPONENT, "node", Some("My")), (SUPPORT_COMPONENT, "node", Some("Mz"))])
        }
        "support_force_magnitude" if location == "node" => out.push(vec![(SUPPORT_FORCE, "node", None)]),
        "support_moment_magnitude" if location == "node" => out.push(vec![(SUPPORT_MOMENT, "node", None)]),
        _ => {}
    }
    out
}

/// The 2-norm K-D5 forms for q: |a|, `norm2` or `norm3`, correctly rounded.
pub(crate) fn group_norm(values: &[f64]) -> Option<f64> {
    match values {
        [a] => Some(a.abs()),
        [a, b] => Some(norm2(*a, *b)),
        [a, b, c] => Some(norm3(*a, *b, *c)),
        _ => None,
    }
}

/// Load-case rows by (case, kind, entity, location, component of a support component row).
struct CaseIndex<'a> {
    rows: HashMap<(&'a str, &'a str, &'a str, &'a str, Option<&'a str>), Vec<usize>>,
}

impl<'a> CaseIndex<'a> {
    fn new(results: &'a [super::ResultItem]) -> Self {
        let mut rows: HashMap<_, Vec<usize>> = HashMap::new();
        for (index, row) in results.iter().enumerate() {
            let Some(basis) = row.basis_ref.as_ref().filter(|b| b.ref_type == "load_case") else { continue };
            let Some(metadata) = row.metadata.as_ref() else { continue };
            let component = (row.kind == SUPPORT_COMPONENT).then_some(metadata.component.as_str());
            rows.entry((basis.ref_id.as_str(), row.kind.as_str(), row.entity_ref.as_str(), metadata.location.as_str(), component)).or_default().push(index);
        }
        CaseIndex { rows }
    }
    /// The unique row of each spec, or `None`.
    fn row_set(&self, case: &str, entity: &str, specs: &[Spec]) -> Option<Vec<usize>> {
        specs
            .iter()
            .map(|(kind, location, component)| match self.rows.get(&(case, *kind, entity, *location, *component)).map(Vec::as_slice) {
                Some([one]) => Some(*one),
                _ => None,
            })
            .collect()
    }
}

/// An operand group's usable row sets (by row-set index): every row present once
/// and the group's q equal to the norm of their published values, bit for bit.
fn coverage(index: &CaseIndex<'_>, results: &[super::ResultItem], case: &str, group: &OperandGroup) -> Vec<Option<Vec<usize>>> {
    row_sets(&group.group, &group.location)
        .iter()
        .map(|specs| {
            let rows = index.row_set(case, &group.entity, specs)?;
            let values: Vec<f64> = rows.iter().map(|&i| results[i].value).collect();
            (group_norm(&values)?.to_bits() == group.q.to_bits()).then_some(rows)
        })
        .collect()
}

// ------------------------------------------------------------------ upward arithmetic

fn exact(x: f64) -> Option<Dyadic> {
    Dyadic::from_f64(x)
}

/// RU(a + b) for a, b ≥ 0: the binary64 sum, or its successor unless exact.
pub(crate) fn up_add(a: f64, b: f64) -> f64 {
    let s = a + b;
    if !s.is_finite() {
        return f64::INFINITY;
    }
    let is_exact = match (exact(a), exact(b), exact(s)) {
        (Some(a), Some(b), Some(s)) => a.plus(&b).is_some_and(|t| t == s),
        _ => false,
    };
    if is_exact { s } else { s.next_up() }
}

/// RU(a · b) for a, b ≥ 0.
pub(crate) fn up_mul(a: f64, b: f64) -> f64 {
    let p = a * b;
    if !p.is_finite() {
        return f64::INFINITY;
    }
    let is_exact = match (exact(a), exact(p)) {
        (Some(a), Some(p)) => a.times_f64(b).is_some_and(|t| t == p),
        _ => false,
    };
    if is_exact { p } else { p.next_up() }
}

/// RU(√x) for x ≥ 0.
pub(crate) fn up_sqrt(x: f64) -> f64 {
    if !x.is_finite() {
        return f64::INFINITY;
    }
    let r = x.sqrt();
    let is_exact = match (exact(r), exact(x)) {
        (Some(r_exact), Some(x)) => r_exact.times_f64(r).is_some_and(|t| t == x),
        _ => false,
    };
    if is_exact { r } else { r.next_up() }
}

/// RU(‖x‖₂): squares and their sum rounded upward in component order.
pub(crate) fn up_norm(values: &[f64]) -> f64 {
    let mut sum = 0.0;
    for value in values {
        sum = up_add(sum, up_mul(value.abs(), value.abs()));
    }
    up_sqrt(sum)
}

/// The smallest binary64 not below |d| (+inf beyond the finite range).
pub(crate) fn round_up_magnitude(d: &Dyadic, estimate: f64) -> f64 {
    if d.is_zero() {
        return 0.0;
    }
    let at_least = |c: f64| exact(c).is_some_and(|c| c.compare_magnitude(d) != Ordering::Less);
    if !at_least(f64::MAX) {
        return f64::INFINITY;
    }
    let guess = if estimate.is_finite() { estimate.abs() } else { f64::MAX };
    if at_least(guess) && (guess == 0.0 || !at_least(guess.next_down())) {
        return guess;
    }
    // Nonnegative binary64 bit patterns are ordered as their values.
    let (mut low, mut high) = (0u64, f64::MAX.to_bits());
    while low < high {
        let middle = low + (high - low) / 2;
        if at_least(f64::from_bits(middle)) {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    f64::from_bits(high)
}

/// RU(|v_D − Σ fᵢ·vᵢ|), formed exactly (TwoSum's error for a subtraction).
pub(crate) fn rounding_error(derived: f64, terms: &[(f64, f64)]) -> f64 {
    let mut sum = match exact(derived) {
        Some(d) => d,
        None => return f64::INFINITY,
    };
    for (factor, value) in terms {
        match exact(*value).and_then(|v| v.times_f64(*factor)).and_then(|p| sum.minus(&p)) {
            Some(next) => sum = next,
            None => return f64::INFINITY,
        }
    }
    // A cheap estimate: exact for a subtraction whose value is its rounded difference.
    let estimate = if let [(fa, a), (fb, b)] = terms {
        let (x, y) = (fa * a, fb * b);
        let s = x + y;
        let bb = s - x;
        let error = (x - (s - bb)) + (y - bb);
        (derived - s) - error
    } else {
        derived - terms.iter().map(|(f, v)| f * v).sum::<f64>()
    };
    round_up_magnitude(&sum, estimate)
}

/// The verdict of a derived group: (verdict, reason).
pub(crate) fn verdict(q: f64, q_hat: f64, bound: f64, floor_scale: f64) -> (&'static str, Option<&'static str>) {
    match formation_guard::member_group_decision(q, q_hat, bound, floor_scale) {
        Err(_) => ("not_established", Some("comparison_unavailable")),
        Ok(fires) => {
            let floor = floor_scale >= formation_guard::DESIGN_FLOOR_MIN_SCALE && q_hat >= formation_guard::DESIGN_FLOOR * floor_scale;
            if !floor {
                ("not_assessed", Some("below_floor"))
            } else if fires {
                ("not_established", Some("bound_exceeds_criterion"))
            } else {
                ("established", None)
            }
        }
    }
}

// ------------------------------------------------------------------ the record

struct Derived {
    json: Value,
    rows: Vec<usize>,
    bound: f64,
    floor_scale: f64,
    verdict: &'static str,
}

/// The record token of a combination's `subtrahend_support_state` (DEL-05-02
/// `operating-case.md` §7): null when absent, `own`, `minuend_final`, or
/// `unrecognized` for any other present value (null included).
pub(crate) fn subtrahend_support_token(combination: &super::PreviewCombination) -> Option<&'static str> {
    use super::document_members::SubtrahendSupportStateSetting as S;
    combination.subtrahend_support_state.map(|s| match s {
        S::Own => "own",
        S::MinuendFinal => "minuend_final",
        S::Unknown => "unrecognized",
    })
}

/// Premise (Work `combination-certificate.md` §1): the operand bounds are each operand
/// case's own. A held subtrahend (any support state but the default independent solve)
/// is a different system from its load case, so the combination is not assessed.
pub(crate) fn held_reason(token: Option<&str>) -> Option<String> {
    match token {
        None | Some("own") => None,
        Some(token) => Some(format!(
            "its subtrahend is held at another case's support state (subtrahend_support_state {token}), a different system from its load case; the operand bounds are each operand case's own"
        )),
    }
}

/// The combination's operand load cases and factors, in evaluation order, or the
/// reason it is outside the certificate.
fn operands_of(case_ids: &[&str], combination: &super::PreviewCombination) -> Result<Vec<(String, f64)>, String> {
    let is_case = |id: &str| case_ids.contains(&id);
    match combination.basis.as_str() {
        "mechanics" => {
            if combination.terms.iter().any(|t| t.combination.is_some() || !is_case(&t.load_case)) {
                return Err("a combination operand is outside the certificate".into());
            }
            Ok(combination.terms.iter().map(|t| (t.load_case.clone(), t.factor)).collect())
        }
        "result_state_subtraction" => {
            let (Some(m), Some(s)) = (&combination.minuend_id, &combination.subtrahend_id) else {
                return Err("an incomplete expression".into());
            };
            if !is_case(m) || !is_case(s) {
                return Err("a combination operand is outside the certificate".into());
            }
            Ok(vec![(m.clone(), 1.0), (s.clone(), -1.0)])
        }
        "range_envelope" => Err("range-envelope certification is deferred".into()),
        _ => Err("absolute sums and SRSS are outside the certificate".into()),
    }
}

/// A member's pressure-region record of one case (exactly one), or `None`.
fn region_of<'v>(pressure: &'v [Value], case: &str, member: &str) -> Option<&'v Value> {
    let mut hits = pressure.iter().filter(|r| r["load_case_id"] == case && r["member_pipe_ids"].as_array().is_some_and(|m| m.iter().any(|p| p == member)));
    let first = hits.next()?;
    hits.next().is_none().then_some(first)
}

fn bits_text(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

/// D-6's premise on one member of a subtraction: bit-identical pressure inputs.
fn equal_pressure(pressure: &[Value], minuend: &str, subtrahend: &str, member: &str) -> bool {
    let (Some(a), Some(b)) = (region_of(pressure, minuend, member), region_of(pressure, subtrahend, member)) else { return false };
    let entry = |region: &Value, field: &str| -> Option<String> {
        region[field].as_array()?.iter().find(|e| e["pipe_id"] == member).map(bits_text)
    };
    ["p_pa", "external_pressure_increment_pa", "pressure_basis"].iter().all(|f| a.get(*f).is_some() && bits_text(&a[*f]) == bits_text(&b[*f]))
        && ["geometry", "materials"].iter().all(|f| entry(a, f).is_some() && entry(a, f) == entry(b, f))
}

/// Publishes `contract_evidence.combination_certificate` and one diagnostic per
/// combination with rows. `operands`: the eligible cases' groups.
pub(crate) fn publish(combinations: &[super::PreviewCombination], case_ids: &[&str], envelope: &mut MechanicsEnvelope, operands: &HashMap<String, Operand>) {
    if combinations.is_empty() || envelope.status.mechanics != "MECHANICS_SOLVED" {
        return;
    }
    let Some(evidence) = envelope.contract_evidence.as_ref() else { return };
    // Gate order and outcome: PF `combination_gates`, EX `exact_combinations`.
    let gates: Vec<(String, bool)> = if let Some(gates) = evidence.get("combination_gates").and_then(Value::as_array) {
        gates.iter().map(|g| (g["combination_id"].as_str().unwrap_or_default().to_string(), g["withheld"] == true)).collect()
    } else if let Some(records) = evidence.get("exact_combinations").and_then(Value::as_array) {
        records.iter().map(|r| (r["combination_id"].as_str().unwrap_or_default().to_string(), r["gate"]["withheld"] == true)).collect()
    } else {
        return;
    };
    let pressure: Vec<Value> = evidence.get("pressure").and_then(Value::as_array).cloned().unwrap_or_default();
    let results = &envelope.results;
    let index = CaseIndex::new(results);
    let mut by_sources: HashMap<(&str, Vec<&str>), Vec<usize>> = HashMap::new();
    let mut combination_rows: HashMap<&str, Vec<usize>> = HashMap::new();
    for (i, row) in results.iter().enumerate() {
        let Some(basis) = row.basis_ref.as_ref().filter(|b| b.ref_type == "combination") else { continue };
        if row.kind == MODULUS_RECORD {
            continue;
        }
        combination_rows.entry(basis.ref_id.as_str()).or_default().push(i);
        by_sources.entry((basis.ref_id.as_str(), row.source_result_refs.iter().map(String::as_str).collect())).or_default().push(i);
    }
    let case_order: Vec<String> = serde_json::to_value(&envelope.numerical_quality.cases)
        .ok()
        .and_then(|v| v.as_array().cloned())
        .unwrap_or_default()
        .iter()
        .filter(|c| c["basis_ref"]["ref_type"] == "load_case")
        .filter_map(|c| c["basis_ref"]["ref_id"].as_str().map(str::to_string))
        .collect();

    let mut used_cases: HashSet<String> = HashSet::new();
    let mut records = Vec::new();
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    for (id, withheld) in &gates {
        let Some(combination) = combinations.iter().find(|c| &c.id == id) else { continue };
        let rows_of = combination_rows.get(id.as_str()).cloned().unwrap_or_default();
        let support_state = subtrahend_support_token(combination);
        let assessed = if let Some(reason) = held_reason(support_state) {
            Err(reason)
        } else if *withheld {
            Err("withheld by its gate".to_string())
        } else {
            operands_of(case_ids, combination).and_then(|terms| {
                let mut missing: Vec<&str> = Vec::new();
                for (case, _) in &terms {
                    if !operands.contains_key(case) && !missing.contains(&case.as_str()) {
                        missing.push(case);
                    }
                }
                if missing.is_empty() {
                    Ok(terms)
                } else {
                    Err(format!("operand case(s) {} carry no certified rows", missing.join(", ")))
                }
            })
        };
        let terms = match assessed {
            Ok(terms) => terms,
            Err(reason) => {
                if !*withheld {
                    diagnostics.push(diag(
                        &format!("{DIAGNOSTIC_PREFIX}{}", exact_source_identity(&[id])),
                        NOT_ASSESSED,
                        "info",
                        not_assessed_message(id, &reason),
                        vec![id.clone()],
                    ));
                }
                records.push(json!({"combination_id": id, "basis": combination.basis, "expression": exact_combinations::expression_evidence(combination),
                    "subtrahend_support_state": support_state, "assessment": "not_assessed", "reason": reason, "groups": [], "uncovered_row_count": rows_of.len()}));
                continue;
            }
        };
        for (case, _) in &terms {
            used_cases.insert(case.clone());
        }
        let subtraction = combination.basis == "result_state_subtraction";
        let mut derived: Vec<Derived> = Vec::new();
        // Derived component groups by (entity, location, group, row set), for the
        // magnitudes and D-6.
        let mut formed: HashMap<(String, String, String, usize), usize> = HashMap::new();
        let first = &operands[&terms[0].0];
        for group in &first.groups {
            let magnitude = match group.group.as_str() {
                "support_force_magnitude" => Some("support_force"),
                "support_moment_magnitude" => Some("support_moment"),
                _ => None,
            };
            if let Some(components) = magnitude {
                if let Some(d) = magnitude_group(results, id, group, components, &formed, &derived, &combination_rows) {
                    derived.push(d);
                }
                continue;
            }
            // Every operand's same group, with a bound, and its coverage.
            let mut per_operand = Vec::new();
            for (case, _) in &terms {
                let operand = &operands[case];
                let Some(own) = operand.groups.iter().find(|g| g.entity == group.entity && g.location == group.location && g.group == group.group) else { break };
                let Some(bound) = own.bound else { break };
                per_operand.push((own, bound, coverage(&index, results, case, own)));
            }
            if per_operand.len() != terms.len() {
                continue;
            }
            let sets = per_operand.iter().map(|(_, _, c)| c.len()).min().unwrap_or(0);
            for set in 0..sets {
                let Some(operand_rows) = per_operand.iter().map(|(_, _, c)| c[set].clone()).collect::<Option<Vec<Vec<usize>>>>() else { continue };
                // The specs must agree across operands (same kinds and locations).
                let specs = |i: usize| -> Vec<(&str, &str)> {
                    operand_rows[i].iter().map(|&r| (results[r].kind.as_str(), results[r].metadata.as_ref().map_or("", |m| m.location.as_str()))).collect()
                };
                if (1..terms.len()).any(|i| specs(i) != specs(0)) {
                    continue;
                }
                let components = operand_rows[0].len();
                let mut rows = Vec::with_capacity(components);
                for c in 0..components {
                    let sources: Vec<&str> = operand_rows.iter().map(|r| results[r[c]].id.as_str()).collect();
                    match by_sources.get(&(id.as_str(), sources)).map(Vec::as_slice) {
                        Some([one]) => rows.push(*one),
                        _ => break,
                    }
                }
                if rows.len() != components {
                    continue;
                }
                let values: Vec<f64> = rows.iter().map(|&r| results[r].value).collect();
                let errors: Vec<f64> = (0..components)
                    .map(|c| rounding_error(values[c], &terms.iter().zip(&operand_rows).map(|((_, f), r)| (*f, results[r[c]].value)).collect::<Vec<_>>()))
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
                let (verdict, reason) = verdict(q, q_hat, bound, floor_scale);
                formed.insert((group.entity.clone(), group.location.clone(), group.group.clone(), set), derived.len());
                derived.push(Derived {
                    json: json!({"entity": group.entity, "location": group.location, "group": group.group, "row_set": set,
                        "rows": rows.iter().map(|&r| results[r].id.clone()).collect::<Vec<_>>(),
                        "factors_bound": factors_bound, "rounding_error": error, "bound": bound, "q": q, "q_hat": q_hat,
                        "floor_scale": floor_scale, "verdict": verdict, "reason": reason, "equal_pressure": Value::Null}),
                    rows,
                    bound,
                    floor_scale,
                    verdict,
                });
            }
        }
        // D-6 (R-4 §6.3): a pipe-wall group of an equal-pressure member names the
        // derived elastic axial group at its location.
        if subtraction && !pressure.is_empty() {
            for d in derived.iter_mut() {
                if d.json["group"] != "pipe_wall" {
                    continue;
                }
                let member = d.json["entity"].as_str().unwrap_or_default().to_string();
                if !equal_pressure(&pressure, &terms[0].0, &terms[1].0, &member) {
                    continue;
                }
                let location = d.json["location"].as_str().unwrap_or_default().trim_start_matches("wall_").to_string();
                let elastic = (0..4)
                    .find_map(|set| formed.get(&(member.clone(), location.clone(), "axial".to_string(), set)).map(|_| set))
                    .map(|set| json!({"entity": member, "location": location, "group": "axial", "row_set": set}))
                    .unwrap_or(Value::Null);
                d.json["equal_pressure"] = json!({"elastic_group": elastic});
            }
        }
        let covered: HashSet<usize> = derived.iter().flat_map(|d| d.rows.iter().copied()).collect();
        let uncovered = rows_of.iter().filter(|r| !covered.contains(r)).count();
        let count = |v: &str| derived.iter().filter(|d| d.verdict == v).count();
        diagnostics.push(diag(
            &format!("{DIAGNOSTIC_PREFIX}{}", exact_source_identity(&[id])),
            WITHIN,
            "info",
            within_message(id, subtraction, count("established"), count("not_established"), count("not_assessed"), uncovered),
            vec![id.clone()],
        ));
        records.push(json!({"combination_id": id, "basis": combination.basis, "expression": exact_combinations::expression_evidence(combination),
            "subtrahend_support_state": support_state, "assessment": "assessed", "reason": Value::Null, "groups": derived.iter().map(|d| d.json.clone()).collect::<Vec<_>>(),
            "uncovered_row_count": uncovered}));
    }
    let operand_cases: Vec<Value> = case_order
        .iter()
        .filter(|case| used_cases.contains(*case))
        .filter_map(|case| operands.get(case).map(|o| (case, o)))
        .map(|(case, operand)| {
            json!({"load_case_id": case, "groups": operand.groups.iter().map(|g| {
                let rows: Vec<Value> = coverage(&index, results, case, g).into_iter()
                    .map(|set| set.map_or(Value::Null, |rows| json!(rows.iter().map(|&r| results[r].id.clone()).collect::<Vec<_>>())))
                    .collect();
                json!({"entity": g.entity, "location": g.location, "group": g.group, "moment": g.moment, "rows": rows,
                    "q": g.q, "bound": g.bound, "floor_scale": g.floor_scale})
            }).collect::<Vec<_>>()})
        })
        .collect();
    let member = json!({"operand_cases": operand_cases, "combinations": records});
    if let Some(evidence) = envelope.contract_evidence.as_mut() {
        evidence[MEMBER] = member;
    }
    envelope.diagnostics.extend(diagnostics);
}

/// A support magnitude row of the combination: `norm3` of its derived component
/// rows, bounded by their bound plus the correctly rounded root's error.
fn magnitude_group(
    results: &[super::ResultItem],
    combination: &str,
    group: &OperandGroup,
    components: &str,
    formed: &HashMap<(String, String, String, usize), usize>,
    derived: &[Derived],
    combination_rows: &HashMap<&str, Vec<usize>>,
) -> Option<Derived> {
    let kind = if components == "support_force" { SUPPORT_FORCE } else { SUPPORT_MOMENT };
    let component = &derived[*formed.get(&(group.entity.clone(), group.location.clone(), components.to_string(), 0))?];
    let candidates: Vec<usize> = combination_rows.get(combination)?.iter().copied().filter(|&r| results[r].kind == kind && results[r].entity_ref == group.entity).collect();
    let [row] = candidates.as_slice() else { return None };
    let values: Vec<f64> = component.rows.iter().map(|&r| results[r].value).collect();
    let value = results[*row].value;
    if values.len() != 3 || norm3(values[0], values[1], values[2]).to_bits() != value.to_bits() {
        return None;
    }
    let root = if value == 0.0 { 0.0 } else { value.next_up() - value };
    let bound = up_add(component.bound, root);
    let q = value.abs();
    let q_hat = q.max(up_add(q, bound));
    let (verdict, reason) = verdict(q, q_hat, bound, component.floor_scale);
    Some(Derived {
        json: json!({"entity": group.entity, "location": group.location, "group": group.group, "row_set": 0,
            "rows": [results[*row].id], "factors_bound": component.bound, "rounding_error": root, "bound": bound, "q": q, "q_hat": q_hat,
            "floor_scale": component.floor_scale, "verdict": verdict, "reason": reason, "equal_pressure": Value::Null}),
        rows: vec![*row],
        bound,
        floor_scale: component.floor_scale,
        verdict,
    })
}

pub(crate) fn within_message(id: &str, subtraction: bool, established: usize, not_established: usize, not_assessed: usize, uncovered: usize) -> String {
    let states = if subtraction { "the difference of two independently solved states" } else { "a combination of independently solved states" };
    format!(
        "combination {id}: {established} derived row group(s) are established within their stated bound against the operand cases' intended systems; \
{not_established} are not established (the bound exceeds the 1e-9 criterion, as where the operands nearly cancel, or the comparison is unavailable); \
{not_assessed} are not assessed (below the floor); {uncovered} of its rows are covered by no group and remain inspection results. \
This is a numerical statement about {states}; it does not establish the meaning of \"expansion\" or engineering acceptance, \
and these rows do not count toward the document's numerical eligibility. Each group's bound and verdict are stated in contract_evidence.combination_certificate."
    )
}

pub(crate) fn not_assessed_message(id: &str, reason: &str) -> String {
    format!(
        "combination {id}: its rows are not assessed against a stated bound ({reason}); they remain inspection results and do not count toward the document's numerical eligibility."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn combination(state: Option<serde_json::Value>) -> super::super::PreviewCombination {
        let mut c = json!({"id": "comb:x", "basis": "result_state_subtraction", "minuend_id": "case:O", "subtrahend_id": "case:S",
            "difference_basis": "operating_minus_sustained"});
        if let Some(state) = state {
            c["subtrahend_support_state"] = state;
        }
        serde_json::from_value(c).expect("a combination record")
    }

    /// Premise (Work `combination-certificate.md` §1): the operand bounds are each operand
    /// case's own; a held subtrahend is a different system and is not assessed, failing
    /// closed on every value but the default independent solve.
    #[test]
    fn a_held_subtrahend_is_never_assessed() {
        assert_eq!(subtrahend_support_token(&combination(None)), None);
        assert_eq!(held_reason(subtrahend_support_token(&combination(None))), None);
        assert_eq!(held_reason(subtrahend_support_token(&combination(Some(json!("own"))))), None);
        for (value, token) in [(json!("minuend_final"), "minuend_final"), (json!("frozen"), "unrecognized"), (Value::Null, "unrecognized"), (json!(1), "unrecognized")] {
            let c = combination(Some(value));
            assert_eq!(subtrahend_support_token(&c), Some(token));
            let reason = held_reason(subtrahend_support_token(&c)).expect("not assessed");
            assert!(reason.contains("a different system from its load case; the operand bounds are each operand case's own"));
        }
    }
}
