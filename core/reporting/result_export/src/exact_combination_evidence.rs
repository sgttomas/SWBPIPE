//! W-16 slice b (DEL-05-02 Design `exact-combinations.md` §6): the combination
//! pass of the pressure-2 reader.
//!
//! The direct-case pass validates a document's primitive cases first, on a view
//! without its combination rows and `contract_evidence.exact_combinations`
//! ([`direct_view`]). This pass then re-derives every combination of the
//! received document from the complete expression its record states, the
//! validated operand rows and the case evidence (sections, materials, pressure
//! regions, load/reference-state members and connector records), and compares
//! complete coverage: the published combination rows in order (id, kind, unit,
//! entity, metadata, value bits and source refs), the operand pairing and
//! recovery witnesses, the gate, the operands' standing, the missing sources and
//! withheld rows, and the diagnostics that state them. Any difference is a
//! forgery and is refused. [`model_checks`] binds each record's expression and
//! operand modulus selections to the captured model (G2), and binds gate 4's
//! input (RT-A3 increment 3: whether the model has a consuming constant-effort
//! support) to the model's DEC-049 conditions ([`consuming_constant_effort`]),
//! each case's applied-load rows the producer's whole rows for the model's
//! consuming supports (their nodes and acting DOFs included).
//!
//! The algebra is this reader's own: a linear combination is the exact sum of
//! the exact products of the represented binary64 factors and operand values,
//! rounded once to nearest (ties to even, an exact or underflowed zero as +0.0);
//! a subtraction is one binary64 difference; a range envelope selects by mode,
//! ties to the smallest operand id; a recomputed magnitude is the correctly
//! rounded norm of the combined components. Nothing here calls product code.
//!
//! RT-A4 (DEL-05-02 `absolute-sum-srss-combinations.md` §2-§6): absolute sums and
//! SRSS are re-derived with the kernel's definitions (`load_case_algebra`), and so
//! is the fixed-load count κ that gate 3 reads; a combination operand supplies its
//! own published rows; the records follow the evaluation order (each operand
//! combination before its dependants), and a withheld operand withholds its
//! dependants (`combination_graph`).
//!
//! These checks establish internal source consistency, not solver accuracy.
//! Combination rows carry no numerical check of their own and never enter
//! numerical eligibility (§5), which stays with the primitive cases.
use crate::combination_graph::{self as graph, Graph, Operand};
use crate::section_basis::bit_equal;
use open_pipe_stress_correct_norm::norm3;
use open_pipe_stress_units::{canonical_unit, Dimension};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

type Check = Result<(), String>;

pub(crate) const STANDING: &str = "derived_from_case_results";
pub(crate) const RECOVERY_METHOD: &str = "pressure_free_operand_recovery";
pub(crate) const SOURCE_RECOVERY_EXCLUSION: &str = "excluded: a document with a load combination attempts no retained-source recovery for any of its cases, primitive cases included; every case publishes ordinary structural recovery";
const MODULUS_BASIS_MIXED: &str = "COMBINATION_MODULUS_BASIS_MIXED";
const RESOLVED_STIFFNESS_MIXED: &str = "COMBINATION_RESOLVED_STIFFNESS_MIXED";
const INSTALLED_REFERENCE: &str = "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE";
/// Gate 4 (RT-A3 increment 3): PF's reason, after gate 3 (the producer's
/// `exact_combinations::CONSTANT_EFFORT` states the order).
const CONSTANT_EFFORT: &str = "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE";
/// The producer's per-case row of a consuming constant-effort support.
const APPLIED_LOAD: &str = "constant_effort_support_applied_load";
const DERIVED: &str = "EXACT_COMBINATION_DERIVED_FROM_CASE_RESULTS";
const SOURCE_RECOVERY_EXCLUDED: &str = "EXACT_COMBINATION_SOURCE_RECOVERY_EXCLUDED";
const SOURCE_MISSING: &str = "LOAD_COMBINATION_SOURCE_RESULT_MISSING";
const ROW_WITHHELD: &str = "LOAD_COMBINATION_ROW_WITHHELD";
const MAXIMUM_UNAVAILABLE: &str = "COMBINATION_STRESS_MAXIMUM_UNAVAILABLE";
const INTENSIFIED_UNAVAILABLE: &str = "COMBINATION_INTENSIFIED_STRESS_UNAVAILABLE";
const DIAGNOSTIC_PREFIX: &str = "diagnostic:exact-combination:";
const DIAGNOSTIC_SOURCE: &str = "core/product_physics";
/// The producer's diagnostic texts (PP `exact_combinations.rs` `append`), re-derived whole.
const DERIVED_TEXT: &str = "publishes rows derived from case results: they never count towards numerical eligibility, and only the derived row groups that contract_evidence.combination_certificate establishes carry a stated bound; each operand's standing and numerical findings are stated in contract_evidence.exact_combinations, and the headlines and stress maxima cover the load cases only";
const MAXIMUM_TEXT: &str = "stress maxima and section stress extremes are not combined; combine the signed stress component rows instead";
const INTENSIFIED_TEXT: &str = "intensified measures are never combined (their ranges and combinations follow the owner's rule in T6)";
const EXCLUDED_TEXT: &str = "this exact-route document has load combinations, so retained-source recovery is not attempted for any of its cases, primitive cases included; every case publishes ordinary structural recovery";
const EXCLUDED_ID: &str = "diagnostic:exact-combination:source-recovery-excluded";
const RECORD_KEYS: [&str; 11] = [
    "combination_id", "basis", "expression", "operands", "gate", "rows", "recoveries",
    "missing_sources", "withheld", "standing", "source_recovery",
];
const OPERAND_KEYS: [&str; 4] = ["load_case_id", "modulus_basis_ref", "modulus_basis_temperature", "numerical_quality"];
const SECTION_SEPARATOR: &str = "; section_basis=";
const LINEAR_BASIS: &str = "explicit_user_linear_combination";
const SUBTRACTION_BASIS: &str = "explicit_user_result_state_subtraction";
const RANGE_BASIS: &str = "explicit_user_range_envelope";
const LINEAR_SIGN: &str = "positive value follows explicit user linear combination of matching source result sign conventions";
const SUBTRACTION_SIGN: &str = "positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source result sign conventions; a signed difference of solved states, not an equilibrium state";
const MODULUS_SIGN: &str = "presence record; each range operand's solve basis is recorded explicitly";
const INSTALLED_REFERENCE_UNSIGNED: &str = "INSTALLED_REFERENCE_UNSIGNED_COMBINATION_OPERAND";
/// absolute-sum-srss §5.1: gate 4 on an absolute sum or SRSS (PF's code and text).
const CONSTANT_EFFORT_UNSIGNED: &str = "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND";
/// absolute-sum-srss §5.1: gate 4 on a `mechanics` combination naming an operand whose
/// κ is undefined (PF's code and text).
const CONSTANT_EFFORT_OPERAND_MIXED: &str = "CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED";
const NONLINEAR: &str = "NONLINEAR_COMBINATION_REQUIRES_SOLVE";
/// A published nonlinear support state row marks a model with nonlinear supports.
const STATE_CODE: &str = "nonlinear_support_active_set_state_code";
const TIE_ROD_KIND: &str = "connector_tie_rod_force_v1";
const SUPPORT_COMPONENT: &str = "support_reaction_component_v2";
const SUPPORT_FORCE: &str = "support_reaction_force_magnitude_v2";
const SUPPORT_MOMENT: &str = "support_reaction_moment_magnitude_v2";
const SUPPORT_COMPONENTS: [&str; 6] = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];
const DISPLACEMENT_COMPONENTS: [(&str, &str); 3] = [
    ("global_nodal_displacement_x", "nodal_displacement_x"),
    ("global_nodal_displacement_y", "nodal_displacement_y"),
    ("global_nodal_displacement_z", "nodal_displacement_z"),
];
const ZERO_AT_NO_PRESSURE: [&str; 4] = [
    "pipe_lame_radial_stress_v2",
    "pipe_lame_hoop_stress_v2",
    "pipe_toroidal_membrane_hoop_stress_v1",
    "pipe_toroidal_radial_surface_stress_v1",
];
const WALL_FORCE: &str = "pipe_wall_axial_force_v2";
const EFFECTIVE_FORCE: &str = "pipe_effective_axial_force_v2";
const END_ACTION: &str = "pipe_wall_endpoint_action_v2";
const MEMBRANE: &str = "pipe_axial_membrane_stress_v2";
const AXIAL_FORCE: &str = "element_local_axial_force";
const TIE_ROD_ROW: &str = "tie-rod row";
const MISSING_KEY_FIELDS: &str = "row missing key fields";
const DIMENSIONLESS: &str = "dimensionless row";
const ALGEBRA_REFUSED: &str = "load-case algebra refused";
const SUPPORT_SOURCE_MISSING: &str = "source support row missing in an operand case";
const MAXIMUM_KINDS: [&str; 3] = [
    "pipe_elastic_normal_stress_maximum_v2",
    "element_local_normal_stress_extreme",
    "element_local_transverse_shear_stress_maximum",
];
const INTENSIFIED_KINDS: [&str; 3] = [
    "curved_bend_directional_intensified_bending_stress_v1",
    "curved_bend_directional_intensified_bending_maximum_v1",
    "branch_intensified_bending_stress_v1",
];

fn fail(code: &str) -> String {
    format!("SOURCE_PHYSICS_COMBINATION_{code}")
}
fn require(ok: bool, code: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(fail(code))
    }
}
fn keys(v: &Value, required: &[&str]) -> bool {
    v.as_object()
        .is_some_and(|o| o.len() == required.len() && required.iter().all(|k| o.contains_key(*k)))
}
fn text<'a>(v: &'a Value, code: &str) -> Result<&'a str, String> {
    v.as_str().filter(|s| !s.is_empty()).ok_or_else(|| fail(code))
}
fn array<'a>(v: &'a Value, code: &str) -> Result<&'a [Value], String> {
    v.as_array().map(Vec::as_slice).ok_or_else(|| fail(code))
}
fn items(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}
fn identity(parts: &[&str]) -> String {
    parts.iter().map(|part| format!("{}:{part}", part.len())).collect()
}
fn stable_suffix(id: &str) -> String {
    id.replace(':', "-")
}
fn is_combination_row(row: &Value) -> bool {
    row["basis_ref"]["ref_type"] == "combination"
}
fn case_of(row: &Value) -> Option<&str> {
    (row["basis_ref"]["ref_type"] == "load_case").then(|| row["basis_ref"]["ref_id"].as_str()).flatten()
}

/// Whether the received document carries a combination: its evidence member or
/// a combination row.
pub(crate) fn present(source: &Value) -> bool {
    // RT-U4 (single pass): `validate_pressure_evidence` asks this first, and no
    // other caller asks it, so the count is that reader's passes (tests only).
    #[cfg(test)]
    PRESSURE_PASSES.with(|n| n.set(n.get() + 1));
    source["contract_evidence"].get("exact_combinations").is_some()
        || items(&source["results"]).iter().any(is_combination_row)
}
#[cfg(test)]
thread_local! {
    /// [`present`]'s calls on this thread, `validate_pressure_evidence`'s passes
    /// (`reader_verdict_single_pass_tests.rs`).
    pub(crate) static PRESSURE_PASSES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The direct-case view: the document without its combination rows and its
/// `exact_combinations` record, for the unchanged primitive-case pass. The
/// record must be present and non-empty whenever a combination row is.
pub(crate) fn direct_view(source: &Value) -> Result<Value, String> {
    let records = source["contract_evidence"].get("exact_combinations").ok_or_else(|| fail("EVIDENCE_MISSING"))?;
    require(records.as_array().is_some_and(|r| !r.is_empty()), "EVIDENCE_SHAPE")?;
    let mut view = source.clone();
    if let Some(evidence) = view["contract_evidence"].as_object_mut() {
        evidence.remove("exact_combinations");
    }
    if let Some(rows) = view["results"].as_array_mut() {
        rows.retain(|row| !is_combination_row(row));
    }
    Ok(view)
}

// ------------------------------------------------------------------ exact sum

/// An exact signed sum of binary64 values and exact products of two binary64
/// values, in fixed point at 2^-2148 (the quantum of such a product), two's
/// complement over 68 limbs: every finite product occupies bits below 4196, and
/// the 156 spare bits leave room for any practical term count.
#[derive(Clone)]
struct ExactProducts {
    limbs: [u64; 68],
}

impl ExactProducts {
    fn new() -> Self {
        ExactProducts { limbs: [0; 68] }
    }
    /// (mantissa, shift) with x = ±mantissa · 2^(shift − 1074).
    fn split(x: f64) -> (u64, usize) {
        let bits = x.to_bits();
        let exponent = ((bits >> 52) & 0x7ff) as usize;
        let fraction = bits & ((1u64 << 52) - 1);
        if exponent == 0 {
            (fraction, 0)
        } else {
            (fraction | (1u64 << 52), exponent - 1)
        }
    }
    /// Adds ±magnitude · 2^(position − 2148) exactly.
    fn add_scaled(&mut self, magnitude: u128, position: usize, negative: bool) {
        if magnitude == 0 {
            return;
        }
        let mut addend = [0u64; 68];
        let (limb, offset) = (position / 64, position % 64);
        // magnitude < 2^106, so it spans at most three limbs after the offset.
        let low = magnitude << offset;
        let high = if offset == 0 { 0 } else { magnitude >> (128 - offset) };
        addend[limb] = low as u64;
        addend[limb + 1] = (low >> 64) as u64;
        if limb + 2 < 68 {
            addend[limb + 2] = high as u64;
        }
        if negative {
            let mut carry = 1u64;
            for slot in addend.iter_mut() {
                let (v, c) = (!*slot).overflowing_add(carry);
                *slot = v;
                carry = u64::from(c);
            }
        }
        let mut carry = 0u64;
        for (slot, add) in self.limbs.iter_mut().zip(addend) {
            let (v, c1) = slot.overflowing_add(add);
            let (v, c2) = v.overflowing_add(carry);
            *slot = v;
            carry = u64::from(c1) + u64::from(c2);
        }
    }
    /// Adds a finite binary64 exactly.
    fn add(&mut self, x: f64) {
        let (m, shift) = Self::split(x);
        self.add_scaled(m as u128, shift + 1074, x.is_sign_negative());
    }
    /// Adds the exact product of two finite binary64 values.
    fn add_product(&mut self, a: f64, b: f64) {
        let (ma, sa) = Self::split(a);
        let (mb, sb) = Self::split(b);
        self.add_scaled(ma as u128 * mb as u128, sa + sb, a.is_sign_negative() != b.is_sign_negative());
    }
    fn negative(&self) -> bool {
        self.limbs[67] >> 63 == 1
    }
    fn signum(&self) -> i8 {
        if self.negative() {
            -1
        } else if self.limbs.iter().all(|l| *l == 0) {
            0
        } else {
            1
        }
    }
    /// The sum rounded once to nearest, ties to even; +0.0 for an exact or an
    /// underflowed zero; `None` beyond the binary64 range.
    fn round(&self) -> Option<f64> {
        let negative = self.negative();
        let mut magnitude = self.limbs;
        if negative {
            let mut carry = 1u64;
            for slot in magnitude.iter_mut() {
                let (v, c) = (!*slot).overflowing_add(carry);
                *slot = v;
                carry = u64::from(c);
            }
        }
        let Some(top) = (0..68).rev().find(|&i| magnitude[i] != 0) else { return Some(0.0) };
        let high = 64 * top + 63 - magnitude[top].leading_zeros() as usize;
        let bit = |i: usize| (magnitude[i / 64] >> (i % 64)) & 1;
        // The binary64 quantum of the result's binade, in units of 2^-2148:
        // 2^-1074 (position 1074) in the subnormal range.
        let mut q = high.saturating_sub(52).max(1074);
        let mut m = 0u64;
        if high >= q {
            for i in (q..=high).rev() {
                m = (m << 1) | bit(i);
            }
        }
        let half = q >= 1 && bit(q - 1) == 1;
        let sticky = q >= 2 && (0..q - 1).any(|i| bit(i) == 1);
        if half && (sticky || m & 1 == 1) {
            m += 1;
        }
        if m == 1u64 << 53 {
            m >>= 1;
            q += 1;
        }
        if m == 0 {
            return Some(0.0);
        }
        // value = m · 2^(q − 2148).
        let exponent = q as i64 - 2148;
        let value = if m < 1u64 << 52 {
            // Only at q = 1074: a subnormal.
            f64::from_bits(m)
        } else {
            let biased = exponent + 52 + 1023;
            if biased > 2046 {
                return None;
            }
            f64::from_bits((biased as u64) << 52 | (m - (1u64 << 52)))
        };
        Some(if negative { -value } else { value })
    }
}

/// §4 gates 3 and 4's rule over flat factors: the exact sum of the factors is within 2^-46
/// of 1, compared exactly and inclusively. Gates 3 and 4 read κ (absolute-sum-srss §5.1),
/// which equals this sum for a flat combination; the test below pins the rule.
#[cfg(test)]
fn unit_factor_sum(factors: &[f64]) -> bool {
    let bound = 2f64.powi(-46);
    let mut sum = ExactProducts::new();
    for factor in factors {
        if !factor.is_finite() {
            return false;
        }
        sum.add(*factor);
    }
    sum.add(-1.0);
    let mut above = sum.clone();
    above.add(-bound);
    let mut below = sum;
    below.add(bound);
    above.signum() <= 0 && below.signum() >= 0
}

// ------------------------------------------------------------------ expression

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Min,
    Max,
    MinAbs,
    MaxAbs,
}

enum Expression<'a> {
    /// `mechanics`, `absolute_sum` or `srss`: (operand, factor) in term order.
    Terms(&'a str, Vec<(Operand<'a>, f64)>),
    Subtraction(&'a str, &'a str),
    Envelope { authored: Vec<&'a str>, sorted: Vec<&'a str>, mode: Mode },
}

impl<'a> Expression<'a> {
    fn read(basis: &'a str, v: &'a Value) -> Result<Self, String> {
        match basis {
            "mechanics" | "absolute_sum" | "srss" => {
                require(keys(v, &["terms"]), "EXPRESSION_SHAPE")?;
                let mut terms: Vec<(Operand<'a>, f64)> = Vec::new();
                for term in array(&v["terms"], "EXPRESSION_SHAPE")? {
                    let (operand, factor) = graph::evidence_term(term).ok_or_else(|| {
                        if term["factor"].as_f64().is_some_and(|f| !f.is_finite()) { fail("EXPRESSION_FACTOR") } else { fail("EXPRESSION_SHAPE") }
                    })?;
                    require(terms.iter().all(|(o, _)| *o != operand), "EXPRESSION_DUPLICATE")?;
                    terms.push((operand, factor));
                }
                require(!terms.is_empty(), "EXPRESSION_SHAPE")?;
                Ok(Expression::Terms(basis, terms))
            }
            "result_state_subtraction" => {
                require(keys(v, &["minuend_id", "subtrahend_id"]), "EXPRESSION_SHAPE")?;
                let (m, s) = (text(&v["minuend_id"], "EXPRESSION_SHAPE")?, text(&v["subtrahend_id"], "EXPRESSION_SHAPE")?);
                require(m != s, "EXPRESSION_DUPLICATE")?;
                Ok(Expression::Subtraction(m, s))
            }
            "range_envelope" => {
                require(keys(v, &["operand_ids", "mode"]), "EXPRESSION_SHAPE")?;
                let mut authored = Vec::new();
                for id in array(&v["operand_ids"], "EXPRESSION_SHAPE")? {
                    let id = text(id, "EXPRESSION_SHAPE")?;
                    require(!authored.contains(&id), "EXPRESSION_DUPLICATE")?;
                    authored.push(id);
                }
                require(!authored.is_empty(), "EXPRESSION_SHAPE")?;
                let mode = match v["mode"].as_str() {
                    Some("min") => Mode::Min,
                    Some("max") => Mode::Max,
                    Some("min_abs") => Mode::MinAbs,
                    Some("max_abs") => Mode::MaxAbs,
                    _ => return Err(fail("EXPRESSION_MODE")),
                };
                let mut sorted = authored.clone();
                sorted.sort_unstable();
                Ok(Expression::Envelope { authored, sorted, mode })
            }
            _ => Err(fail("BASIS")),
        }
    }
    /// The operand order: term order, minuend then subtrahend, or sorted ids. An id
    /// names the load case when one has it, a combination otherwise (§2).
    fn operands(&self, cases: &HashSet<&str>) -> Vec<Operand<'a>> {
        let resolve = |id: &'a str| if cases.contains(id) { Operand::Case(id) } else { Operand::Combination(id) };
        match self {
            Expression::Terms(_, terms) => terms.iter().map(|(o, _)| *o).collect(),
            Expression::Subtraction(m, s) => vec![resolve(m), resolve(s)],
            Expression::Envelope { sorted, .. } => sorted.iter().map(|id| resolve(id)).collect(),
        }
    }
    fn factors(&self) -> Vec<f64> {
        match self {
            Expression::Terms(_, terms) => terms.iter().map(|(_, f)| *f).collect(),
            _ => Vec::new(),
        }
    }
    fn basis(&self) -> &str {
        match self {
            Expression::Terms(basis, _) => basis,
            Expression::Subtraction(..) => "result_state_subtraction",
            Expression::Envelope { .. } => "range_envelope",
        }
    }
    fn range(&self) -> bool {
        matches!(self, Expression::Envelope { .. })
    }
    fn unsigned(&self) -> bool {
        matches!(self, Expression::Terms("absolute_sum" | "srss", _))
    }
    fn metadata_basis(&self) -> &'static str {
        match self {
            Expression::Terms("absolute_sum", _) => graph::ABSOLUTE_SUM_BASIS,
            Expression::Terms("srss", _) => graph::SRSS_BASIS,
            Expression::Terms(..) => LINEAR_BASIS,
            Expression::Subtraction(..) => SUBTRACTION_BASIS,
            Expression::Envelope { .. } => RANGE_BASIS,
        }
    }
    /// The sign convention; `solved_state_plus_increments` is decision OQ-1's admission.
    fn sign(&self, solved_state_plus_increments: bool) -> String {
        match self {
            Expression::Terms("mechanics", _) if solved_state_plus_increments => graph::SOLVED_STATE_PLUS_INCREMENTS_SIGN.to_string(),
            Expression::Terms("absolute_sum", _) => graph::ABSOLUTE_SUM_SIGN.to_string(),
            Expression::Terms("srss", _) => graph::SRSS_SIGN.to_string(),
            Expression::Terms(..) => LINEAR_SIGN.to_string(),
            Expression::Subtraction(..) => SUBTRACTION_SIGN.to_string(),
            Expression::Envelope { mode, .. } => format!(
                "value is the {} mode-selected source result across explicit user range-envelope operands and keeps that source result sign convention",
                match mode {
                    Mode::Min => "min",
                    Mode::Max => "max",
                    Mode::MinAbs => "min_abs",
                    Mode::MaxAbs => "max_abs",
                }
            ),
        }
    }
    /// The combined value of one operand value per operand, in operand order.
    fn evaluate(&self, values: &[f64]) -> Option<f64> {
        match self {
            Expression::Terms(basis @ ("absolute_sum" | "srss"), _) => graph::unsigned_value(*basis == "srss", &self.factors(), values),
            Expression::Terms(_, terms) => {
                let mut sum = ExactProducts::new();
                for ((_, factor), value) in terms.iter().zip(values) {
                    sum.add_product(*factor, *value);
                }
                sum.round()
            }
            Expression::Subtraction(..) => Some(values[0] - values[1]),
            Expression::Envelope { mode, .. } => {
                // Sorted ids: the first operand wins a tie.
                let mut best = values[0];
                for value in &values[1..] {
                    let better = match mode {
                        Mode::Min => *value < best,
                        Mode::Max => *value > best,
                        Mode::MinAbs => value.abs() < best.abs(),
                        Mode::MaxAbs => value.abs() > best.abs(),
                    };
                    if better {
                        best = *value;
                    }
                }
                Some(best)
            }
        }
    }
}

/// The evidence of an operand's source row.
fn row_source(operand: &Operand<'_>, id: &str) -> Value {
    match operand {
        Operand::Case(case) => json!({"load_case_id": case, "result_id": id}),
        Operand::Combination(c) => json!({"combination_id": c, "result_id": id}),
    }
}

// ------------------------------------------------------------------ keys and facts

/// The typed structural pairing key (§3), from the row's structured fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Key {
    kind: String,
    unit: String,
    entity: String,
    location: String,
    component: String,
    frame: String,
    section: String,
}

impl Key {
    fn of(row: &Value) -> Option<Key> {
        let field = |v: &Value| v.as_str().map(str::to_string);
        let kind = field(&row["kind"])?;
        // A magnitude is keyed by kind, unit and node: a load case's row carries no
        // metadata, and an unsigned or nested combination's states only its derivation.
        let (location, component, frame, section) = match row.get("metadata") {
            _ if kind == "displacement_magnitude" => Default::default(),
            Some(md) => {
                let basis = md["basis"].as_str()?;
                (field(&md["location"])?, field(&md["component"])?, field(&md["coordinate_system"])?,
                    basis.split_once(SECTION_SEPARATOR).map_or(String::new(), |(_, s)| s.to_string()))
            }
            None => return None,
        };
        Some(Key { kind, unit: field(&row["unit"])?, entity: field(&row["entity_ref"])?, location, component, frame, section })
    }
    fn with(&self, kind: &str, component: &str, location: &str, frame: &str, unit: &str, section: &str) -> Key {
        Key { kind: kind.into(), unit: unit.into(), entity: self.entity.clone(), location: location.into(),
            component: component.into(), frame: frame.into(), section: section.into() }
    }
    fn combined_id(&self, combination: &str) -> String {
        format!("result:combination:{}", identity(&[combination, &self.kind, &self.unit, &self.entity, &self.location, &self.component, &self.frame, &self.section]))
    }
}

struct CaseRows<'s> {
    by_key: HashMap<Key, Vec<&'s Value>>,
    order: Vec<&'s Value>,
}

fn case_rows<'s>(results: &'s [Value], operand: &Operand<'_>) -> CaseRows<'s> {
    let mut by_key: HashMap<Key, Vec<&Value>> = HashMap::new();
    let mut order = Vec::new();
    for row in results.iter().filter(|r| operand.owns(r)) {
        order.push(row);
        if let Some(key) = Key::of(row) {
            by_key.entry(key).or_default().push(row);
        }
    }
    CaseRows { by_key, order }
}

struct CaseFacts<'s> {
    sections: HashMap<&'s str, &'s Value>,
    materials: HashMap<&'s str, &'s Value>,
    material_basis: &'s Value,
    region_members: HashSet<&'s str>,
    states: HashMap<&'s str, &'s Value>,
    members: Vec<&'s str>,
}

fn case_facts<'s>(evidence: &'s Value, case: &str) -> CaseFacts<'s> {
    let exact = items(&evidence["exact_cases"]).iter().find(|c| c["load_case_id"] == case);
    let indexed = |key: &str| -> Vec<(&'s str, &'s Value)> {
        exact.map(|c| items(&c[key]).iter().filter_map(|item| item["pipe_id"].as_str().map(|id| (id, item))).collect()).unwrap_or_default()
    };
    let (materials, sections) = (indexed("pipe_materials"), indexed("pipe_sections"));
    let mut members = Vec::new();
    for (pipe, _) in materials.iter().chain(&sections) {
        if !members.contains(pipe) {
            members.push(*pipe);
        }
    }
    CaseFacts {
        sections: sections.into_iter().collect(),
        materials: materials.into_iter().collect(),
        material_basis: exact.map_or(&Value::Null, |c| &c["material_basis"]),
        region_members: items(&evidence["pressure"])
            .iter()
            .filter(|r| r["load_case_id"] == case)
            .flat_map(|r| items(&r["member_pipe_ids"]))
            .filter_map(Value::as_str)
            .collect(),
        states: items(&evidence["load_reference_states"])
            .iter()
            .find(|r| r["load_case_id"] == case)
            .map(|r| items(&r["members"]).iter().filter_map(|m| m["pipe_id"].as_str().map(|id| (id, m))).collect())
            .unwrap_or_default(),
        members,
    }
}

/// §4 gate 2's per-member, per-case stiffness statement.
fn stiffness(case: &CaseFacts<'_>, pipe: &str) -> Value {
    let material = case.materials.get(pipe).copied().unwrap_or(&Value::Null);
    let state = case.states.get(pipe).copied().unwrap_or(&Value::Null);
    json!({
        "E": material["E_pa"], "nu": material["nu"], "G": material["G_pa"],
        "material": material["material_id"], "constitutive": material["constitutive_basis"],
        "selection": material.get("material_selection_kind").cloned().unwrap_or(Value::Null),
        "case_basis": case.material_basis,
        "state": if state.is_null() { Value::Null } else { json!([
            state["material_selection_kind"], state["material_selection_temperature_k"],
            state["interpolation_fraction"], state["selected_E_pa"], state["selected_nu"],
        ]) },
        "section": case.sections.get(pipe).copied().unwrap_or(&Value::Null),
    })
}

/// Structural equality whose numbers compare as binary64 values (so -0.0 is
/// 0.0, and `2` is `2.0`), as the producer's gate compares its own statements.
fn value_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_f64() == y.as_f64(),
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(p, q)| value_eq(p, q)),
        (Value::Object(x), Value::Object(y)) => x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| value_eq(v, w))),
        _ => a == b,
    }
}

fn installed_reference(connector: &Value) -> bool {
    items(connector).iter().any(|record| {
        items(&record["q_ref"]).iter().any(|v| v.as_f64().is_some_and(|v| v != 0.0))
            || items(&record["tie_rods"]).iter().any(|rod| rod["reference_extension_m"].as_f64().is_some_and(|v| v != 0.0))
    })
}

/// A modulus-basis key of one operand record, as PF's `modulus_basis_key`.
fn modulus_key(operand: &Value) -> Result<Option<String>, String> {
    match (&operand["modulus_basis_ref"], &operand["modulus_basis_temperature"]) {
        (Value::Null, Value::Null) => Ok(None),
        (r, Value::Null) => Ok(Some(format!("exact:{}", text(r, "OPERAND_SHAPE")?))),
        (Value::Null, t) => {
            require(keys(t, &["value", "unit"]), "OPERAND_SHAPE")?;
            text(&t["unit"], "OPERAND_SHAPE")?;
            let value = t["value"].as_f64().filter(|v| v.is_finite()).ok_or_else(|| fail("OPERAND_SHAPE"))?;
            Ok(Some(format!("temperature_kelvin_bits:{:016x}", value.to_bits())))
        }
        _ => Err(fail("OPERAND_SHAPE")),
    }
}

/// The producer's applied-load sign convention
/// (PP `CONSTANT_EFFORT_APPLIED_SIGN_CONVENTION`), re-derived whole.
const APPLIED_LOAD_SIGN_CONVENTION: &str = "positive value is the user-entered constant support force applied along the positive axis of the single declared translational restraint DOF in every solved load case; the ideal constant-effort element contributes zero stiffness and adds no solve restraint row; no gravity coupling, catalog/default value, or inferred direction is supplied";

/// One consuming constant-effort support as the producer resolves it: its id,
/// its model node, the normalized name of its one translational DOF (the
/// acting DOF), its force in newtons and its hanger references as the
/// applied-load row states them.
#[derive(Debug, PartialEq)]
struct ConstantEffortSupport<'a> {
    id: &'a str,
    node: &'a str,
    dof: &'static str,
    force: f64,
    source: &'a str,
    manufacturer: &'a str,
    load_side_review: &'a str,
}

impl ConstantEffortSupport<'_> {
    /// The applied-load row the producer publishes for this support in the
    /// solved case at `position` of `exact_cases` (PP
    /// `append_constant_effort_support_results`): every field, with the id the
    /// first case keeps and every later case qualifies under
    /// `result:loadcase:{case}:` (`qualified_load_case_result_id`).
    fn applied_load_row(&self, case: &str, position: usize) -> Value {
        let tail = format!("constant-effort-support:{}:applied-load", stable_suffix(self.id));
        let id = if position == 0 { format!("result:{tail}") } else { format!("result:loadcase:{}:{tail}", stable_suffix(case)) };
        json!({
            "id": id,
            "kind": APPLIED_LOAD,
            "value": self.force,
            "unit": "N",
            "entity_ref": self.id,
            "basis_ref": {"ref_type": "load_case", "ref_id": case},
            "metadata": {
                "component": APPLIED_LOAD,
                "coordinate_system": "global",
                "location": format!("{}:{} applied load", self.node, self.dof),
                "basis": format!(
                    "support_family=constant_effort_support;consumed_dof={};source={};manufacturer={};load_side_review={};mechanics_consumption=assembled_solve;dec_ref=DEC-049",
                    self.dof, self.source, self.manufacturer, self.load_side_review
                ),
                "sign_convention": APPLIED_LOAD_SIGN_CONVENTION,
            },
        })
    }
}

/// Gate 4's model condition, read from the captured model as the producer's
/// `classify_constant_effort_consumption` reads its normalized model (DEC-049,
/// `validation/hand_calcs/mechanics/constant_effort_support_applied_load.md`):
/// a support the solver reads as constant effort
/// ([`crate::source_blocks::solver_constant_effort_support`]) with no
/// `nonlinear` field, whose `hanger.constant_load` normalizes (by the units
/// authority the producer uses) to a finite positive force, whose every
/// declared restraint is a frame DOF with exactly one translational entry, and
/// whose node is a model node. Returns each consuming support resolved (its
/// node and normalized acting DOF retained), in model order; any other record
/// applies nothing.
fn consuming_constant_effort(model: &Value) -> Vec<ConstantEffortSupport<'_>> {
    let nodes: HashSet<&str> = items(&model["nodes"]).iter().filter_map(|n| n["id"].as_str()).collect();
    // The producer's hanger reference, or its stated absence when missing or blank.
    fn reference<'a>(support: &'a Value, field: &str, missing: &'static str) -> Option<&'a str> {
        match &support["hanger"][field] {
            Value::Null => Some(missing),
            Value::String(text) => Some(if text.trim().is_empty() { missing } else { text.as_str() }),
            _ => None,
        }
    }
    items(&model["supports"])
        .iter()
        .filter(|support| crate::source_blocks::solver_constant_effort_support(support) && support["nonlinear"].is_null())
        .filter_map(|support| {
            let load = &support["hanger"]["constant_load"];
            if load.is_null() {
                return None;
            }
            let force = crate::physics_source::normalized_quantity(load, Dimension::Force).ok().filter(|f| f.is_finite() && *f > 0.0)?;
            let mut translational = Vec::new();
            for dof in items(&support["restraints"]) {
                match dof.as_str()? {
                    "UX" | "Ux" | "ux" => translational.push("UX"),
                    "UY" | "Uy" | "uy" => translational.push("UY"),
                    "UZ" | "Uz" | "uz" => translational.push("UZ"),
                    "RX" | "Rx" | "rx" | "RY" | "Ry" | "ry" | "RZ" | "Rz" | "rz" => {}
                    _ => return None,
                }
            }
            let [dof] = translational[..] else { return None };
            let id = support["id"].as_str()?;
            let node = support["node"].as_str().filter(|n| nodes.contains(n))?;
            Some(ConstantEffortSupport {
                id,
                node,
                dof,
                force,
                source: reference(support, "source_reference", "source_reference_missing")?,
                manufacturer: reference(support, "manufacturer_reference", "manufacturer_reference_missing")?,
                load_side_review: reference(support, "load_side_review_reference", "load_side_review_reference_missing")?,
            })
        })
        .collect()
}

/// The gates (§4; absolute-sum-srss §5) of the combination `id`, read after every
/// earlier record: a withheld operand combination; then, for the term bases, gates 1
/// and 2 over every load case the combination reaches, and gates 3 and 4 on κ.
#[allow(clippy::too_many_arguments)]
fn gate(
    id: &str,
    expression: &Expression<'_>,
    operands: &[Operand<'_>],
    graph: &Graph<'_>,
    modulus: &HashMap<&str, Option<String>>,
    withheld: &[(&str, &str)],
    evidence: &Value,
    nonlinear: bool,
    constant_effort: bool,
) -> Result<Option<(&'static str, String, Vec<String>)>, String> {
    let mut named: Vec<(&str, &str)> = Vec::new();
    for operand in operands {
        if let Operand::Combination(c) = operand {
            if let Some((_, code)) = withheld.iter().find(|(w, _)| w == c) {
                if !named.iter().any(|(n, _)| n == c) {
                    named.push((c, code));
                }
            }
        }
    }
    if !named.is_empty() {
        let text = named.iter().map(|(c, code)| format!("{c} ({code})")).collect::<Vec<_>>().join(", ");
        let refs = std::iter::once(id.to_string()).chain(named.iter().map(|(c, _)| c.to_string())).collect();
        return Ok(Some((graph::OPERAND_WITHHELD, format!("Combination withheld: its operand combination(s) {text} are withheld, so it is withheld whole."), refs)));
    }
    let gated = |code: &'static str| Ok(Some((code, gate_message(code).to_string(), vec![id.to_string()])));
    let basis = expression.basis();
    if !graph::term_basis(basis) {
        return Ok(None);
    }
    // absolute-sum-srss §5.2: with nonlinear supports, a mechanics combination outside
    // decision OQ-1 is withheld.
    if let Expression::Terms("mechanics", terms) = expression {
        if nonlinear && !graph.solved_state_plus_increments(terms) {
            return gated(NONLINEAR);
        }
    }
    let reached = graph.reached(id);
    let mut keys = HashSet::new();
    for case in reached {
        keys.insert(modulus.get(case).cloned().ok_or_else(|| fail("OPERAND_UNRESOLVED"))?);
    }
    if keys.len() > 1 {
        return gated(MODULUS_BASIS_MIXED);
    }
    let cases: Vec<CaseFacts<'_>> = reached.iter().map(|case| case_facts(evidence, case)).collect();
    let mut members: Vec<&str> = Vec::new();
    for case in &cases {
        for pipe in &case.members {
            if !members.contains(pipe) {
                members.push(pipe);
            }
        }
    }
    for pipe in members {
        let first = stiffness(&cases[0], pipe);
        if cases[1..].iter().any(|case| !value_eq(&stiffness(case, pipe), &first)) {
            return gated(RESOLVED_STIFFNESS_MIXED);
        }
    }
    // Gates 3 and 4 (absolute-sum-srss §5.1): one κ rule per fixed-load source,
    // each under its own codes, gate 3 first where both sources hold. Gate 3 names
    // an undefined mechanics operand under its own code (§10); gate 4, as on PF,
    // as CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED.
    let fixed_load_withholding = || {
        let kappas: Vec<_> = operands.iter().map(|o| graph.kappa_of(o)).collect();
        if basis == "mechanics" {
            open_pipe_stress_load_case_algebra::mechanics_fixed_load_gate(&graph.kappa_of(&Operand::Combination(id)), kappas.iter())
        } else {
            open_pipe_stress_load_case_algebra::unsigned_fixed_load_gate(kappas.iter())
        }
    };
    if installed_reference(&evidence["connector"]) && fixed_load_withholding().is_some() {
        return gated(if basis == "mechanics" { INSTALLED_REFERENCE } else { INSTALLED_REFERENCE_UNSIGNED });
    }
    if constant_effort {
        if let Some(withholding) = fixed_load_withholding() {
            use open_pipe_stress_load_case_algebra::FixedLoadWithholding as W;
            return gated(match withholding {
                W::UnitFactorSum => CONSTANT_EFFORT,
                W::OperandUndefined => CONSTANT_EFFORT_OPERAND_MIXED,
                W::UnsignedOperand => CONSTANT_EFFORT_UNSIGNED,
            });
        }
    }
    Ok(None)
}

// ------------------------------------------------------------------ derivation

fn dimension(unit: &str) -> bool {
    matches!(unit, "mm" | "rad" | "N" | "N*m" | "MPa" | "Pa" | "m")
}

/// §3: the recovered operand value of a pressure-family key, and its witness.
fn recover(key: &Key, case: &str, rows: &CaseRows<'_>, facts: &CaseFacts<'_>) -> Option<(f64, Value)> {
    let pipe = key.entity.as_str();
    if !facts.sections.contains_key(pipe) || facts.region_members.contains(pipe) {
        return None;
    }
    let witness = |source: Option<&Value>, sign: &str, area: Option<f64>, value: f64| {
        json!({
            "load_case_id": case, "pipe_id": pipe, "kind": key.kind, "component": key.component,
            "location": key.location, "section_basis": if key.section.is_empty() { Value::Null } else { json!(key.section) },
            "method": RECOVERY_METHOD, "source_result_id": source.map(|row| row["id"].clone()),
            "sign": sign, "area_m2": area,
            "rounding": if area.is_some() { "one_binary64_division" } else { "none" },
            "value": value,
        })
    };
    if ZERO_AT_NO_PRESSURE.contains(&key.kind.as_str()) {
        return Some((0.0, witness(None, "none", None, 0.0)));
    }
    let axial = |location: &str| -> Option<(&Value, f64)> {
        let found = rows.by_key.get(&key.with(AXIAL_FORCE, "axial_force", location, "element_local", "N", ""))?;
        if found.len() != 1 {
            return None;
        }
        Some((found[0], found[0]["value"].as_f64()?))
    };
    match key.kind.as_str() {
        END_ACTION => {
            let (row, value) = axial(&key.location)?;
            Some((value, witness(Some(row), "unchanged", None, value)))
        }
        WALL_FORCE | EFFECTIVE_FORCE | MEMBRANE => {
            let (row, value) = axial(&key.location)?;
            // The producer negates as 0 - N: a negated zero is +0.0.
            let (n, sign) = if key.location == "end_i" { (0.0 - value, "negated") } else { (value, "unchanged") };
            if key.kind == MEMBRANE {
                let area = facts.sections.get(pipe)?.get(key.section.as_str())?["As_m2"].as_f64()?;
                let value = n / area;
                Some((value, witness(Some(row), sign, Some(area), value)))
            } else {
                Some((n, witness(Some(row), sign, None, n)))
            }
        }
        _ => None,
    }
}

enum Outcome {
    Row { value: f64, sources: Vec<Value>, refs: Vec<String> },
    Missing(Vec<String>),
    Withheld,
}

#[derive(Default)]
struct Built {
    rows: Vec<Value>,
    row_evidence: Vec<Value>,
    recoveries: Vec<Value>,
    missing: Vec<Value>,
    withheld: Vec<Value>,
}

impl Built {
    fn withhold(&mut self, name: String, reason: &str) {
        if !self.withheld.iter().any(|w| w["name"] == name.as_str()) {
            self.withheld.push(json!({"name": name, "reason": reason}));
        }
    }
}

fn combined_row(combination: &str, expression: &Expression<'_>, oq1: bool, template: &Value, key: &Key, value: f64, refs: Vec<String>) -> Value {
    let mut row = template.clone();
    row["id"] = json!(key.combined_id(combination));
    row["value"] = json!(value);
    row["basis_ref"] = json!({"ref_type": "combination", "ref_id": combination});
    if let Some(o) = row.as_object_mut() {
        if refs.is_empty() {
            o.remove("source_result_refs");
        } else {
            o.insert("source_result_refs".into(), json!(refs));
        }
    }
    if row.get("metadata").is_some() {
        // A stress row keeps its operands' metadata basis (§2, as built).
        if key.section.is_empty() {
            row["metadata"]["basis"] = json!(expression.metadata_basis());
        }
        row["metadata"]["sign_convention"] = json!(expression.sign(oq1));
    }
    row
}

#[allow(clippy::too_many_arguments)]
fn derive(combination: &str, expression: &Expression<'_>, oq1: bool, operands: &[Operand<'_>], results: &[Value], evidence: &Value, linear: &[&str], recomputed: &[&str]) -> Built {
    let range = expression.range();
    let rows: Vec<CaseRows<'_>> = operands.iter().map(|operand| case_rows(results, operand)).collect();
    // Only a load case's pressure-family value is recovered; a combination operand
    // publishes every row it forms.
    let facts: Vec<Option<CaseFacts<'_>>> = operands.iter().map(|operand| match operand {
        Operand::Case(case) => Some(case_facts(evidence, case)),
        Operand::Combination(_) => None,
    }).collect();
    let mut built = Built::default();
    let support_kind = |kind: &str| matches!(kind, SUPPORT_COMPONENT | SUPPORT_FORCE | SUPPORT_MOMENT);
    let is_recomputed = |kind: &str| recomputed.contains(&kind);

    let mut order: Vec<(Key, &Value)> = Vec::new();
    let mut seen: HashSet<Key> = HashSet::new();
    for case_rows in &rows {
        for row in &case_rows.order {
            let kind = row["kind"].as_str().unwrap_or_default();
            if !(linear.contains(&kind) || is_recomputed(kind)) || support_kind(kind) {
                continue;
            }
            let Some(key) = Key::of(row) else {
                built.withhold(row["id"].as_str().unwrap_or_default().to_string(), MISSING_KEY_FIELDS);
                continue;
            };
            if seen.insert(key.clone()) {
                order.push((key, *row));
            }
        }
    }

    let mut outcomes: HashMap<Key, Outcome> = HashMap::new();
    for (key, template) in &order {
        if is_recomputed(&key.kind) && !range {
            continue;
        }
        let template_id = template["id"].as_str().unwrap_or_default().to_string();
        if let Some(several) = rows.iter().find_map(|r| r.by_key.get(key).filter(|found| found.len() > 1)) {
            let reason = if key.kind == TIE_ROD_KIND { TIE_ROD_ROW } else { MISSING_KEY_FIELDS };
            for row in several {
                built.withhold(row["id"].as_str().unwrap_or_default().to_string(), reason);
            }
            outcomes.insert(key.clone(), Outcome::Withheld);
            continue;
        }
        if key.kind == TIE_ROD_KIND {
            built.withhold(template_id, TIE_ROD_ROW);
            outcomes.insert(key.clone(), Outcome::Withheld);
            continue;
        }
        let mut sources = Vec::new();
        let mut refs = Vec::new();
        let mut values = Vec::new();
        let mut missing = Vec::new();
        let mut witnesses = Vec::new();
        for (index, operand) in operands.iter().enumerate() {
            let case = operand.id();
            if let Some(found) = rows[index].by_key.get(key) {
                let Some(value) = found[0]["value"].as_f64() else {
                    missing.push(case.to_string());
                    continue;
                };
                values.push(value);
                let id = found[0]["id"].as_str().unwrap_or_default().to_string();
                sources.push(row_source(operand, &id));
                refs.push(id);
            } else if let Some((value, witness)) = facts[index].as_ref().and_then(|facts| recover(key, case, &rows[index], facts)) {
                values.push(value);
                sources.push(json!({"load_case_id": case, "recovery_index": built.recoveries.len() + witnesses.len()}));
                if let Some(id) = witness["source_result_id"].as_str() {
                    refs.push(id.to_string());
                }
                witnesses.push(witness);
            } else {
                missing.push(case.to_string());
            }
        }
        if !missing.is_empty() {
            outcomes.insert(key.clone(), Outcome::Missing(missing));
            continue;
        }
        if !dimension(&key.unit) {
            built.withhold(template_id, DIMENSIONLESS);
            outcomes.insert(key.clone(), Outcome::Withheld);
            continue;
        }
        match expression.evaluate(&values) {
            Some(value) => {
                built.recoveries.extend(witnesses);
                outcomes.insert(key.clone(), Outcome::Row { value, sources, refs });
            }
            None => {
                built.withhold(template_id, ALGEBRA_REFUSED);
                outcomes.insert(key.clone(), Outcome::Withheld);
            }
        }
    }

    for (key, template) in &order {
        let template_id = template["id"].as_str().unwrap_or_default().to_string();
        if is_recomputed(&key.kind) && !range {
            let components: Vec<Key> = DISPLACEMENT_COMPONENTS
                .iter()
                .map(|(kind, component)| key.with(kind, component, "node", "global", &key.unit, ""))
                .collect();
            let mut vector = [0.0; 3];
            let mut missing: Vec<String> = Vec::new();
            let mut refused = false;
            for (slot, component) in components.iter().enumerate() {
                match outcomes.get(component) {
                    Some(Outcome::Row { value, .. }) => vector[slot] = *value,
                    Some(Outcome::Withheld) => refused = true,
                    other => {
                        let cases: Vec<String> = match other {
                            Some(Outcome::Missing(cases)) => cases.clone(),
                            _ => operands.iter().map(|c| c.id().to_string()).collect(),
                        };
                        for case in cases {
                            if !missing.contains(&case) {
                                missing.push(case);
                            }
                        }
                    }
                }
            }
            if !missing.is_empty() {
                built.missing.push(json!({"source_result_id": template_id, "missing_load_case_ids": missing}));
                continue;
            }
            if refused {
                built.withhold(template_id, ALGEBRA_REFUSED);
                continue;
            }
            // An unsigned magnitude outside the binary64 range is refused, never an infinity.
            let value = norm3(vector[0], vector[1], vector[2]);
            if expression.unsigned() && !value.is_finite() {
                built.withhold(template_id, ALGEBRA_REFUSED);
                continue;
            }
            let refs = components.iter().map(|c| c.combined_id(combination)).collect();
            let row = combined_row(combination, expression, oq1, template, key, value, refs);
            built.row_evidence.push(json!({"result_id": row["id"], "sources": []}));
            built.rows.push(row);
            continue;
        }
        match outcomes.remove(key) {
            Some(Outcome::Row { value, sources, refs }) => {
                let row = combined_row(combination, expression, oq1, template, key, value, refs);
                built.row_evidence.push(json!({"result_id": row["id"], "sources": sources}));
                built.rows.push(row);
            }
            Some(Outcome::Missing(cases)) => {
                built.missing.push(json!({"source_result_id": template_id, "missing_load_case_ids": cases}));
            }
            Some(Outcome::Withheld) | None => {}
        }
    }

    let mut supports: Vec<&str> = Vec::new();
    for case_rows in &rows {
        for row in &case_rows.order {
            if row["kind"] == SUPPORT_COMPONENT && row["metadata"]["component"] == "Fx" {
                if let Some(id) = row["entity_ref"].as_str() {
                    if !supports.contains(&id) {
                        supports.push(id);
                    }
                }
            }
        }
    }
    for support in supports {
        let find = |index: usize, kind: &str, component: &str| -> Option<&Value> {
            rows[index].order.iter().copied().find(|r| r["kind"] == kind && r["entity_ref"] == support && r["metadata"]["component"] == component)
        };
        let group = || -> Result<Vec<(Value, Value)>, &'static str> {
            let mut out = Vec::new();
            let mut combined = [0.0; 6];
            let mut component_ids: Vec<String> = Vec::new();
            for (slot, component) in SUPPORT_COMPONENTS.iter().enumerate() {
                let sources: Vec<&Value> = (0..operands.len()).map(|i| find(i, SUPPORT_COMPONENT, component)).collect::<Option<_>>().ok_or(SUPPORT_SOURCE_MISSING)?;
                let values: Vec<f64> = sources.iter().map(|s| s["value"].as_f64()).collect::<Option<_>>().ok_or(SUPPORT_SOURCE_MISSING)?;
                let value = expression.evaluate(&values).ok_or(ALGEBRA_REFUSED)?;
                combined[slot] = value;
                let key = Key::of(sources[0]).ok_or(SUPPORT_SOURCE_MISSING)?;
                let refs: Vec<String> = sources.iter().map(|s| s["id"].as_str().unwrap_or_default().to_string()).collect();
                let row = combined_row(combination, expression, oq1, sources[0], &key, value, refs.clone());
                component_ids.push(row["id"].as_str().unwrap_or_default().to_string());
                let evidence = json!({"result_id": row["id"], "sources": operands.iter().zip(&refs).map(|(operand, id)| row_source(operand, id)).collect::<Vec<_>>()});
                out.push((row, evidence));
            }
            for (kind, component, slots) in [(SUPPORT_FORCE, "force_magnitude", 0..3), (SUPPORT_MOMENT, "moment_magnitude", 3..6)] {
                let sources: Vec<&Value> = (0..operands.len()).filter_map(|i| find(i, kind, component)).collect();
                let template = *sources.first().ok_or(SUPPORT_SOURCE_MISSING)?;
                let key = Key::of(template).ok_or(SUPPORT_SOURCE_MISSING)?;
                let (value, refs, evidence_sources) = if range {
                    if sources.len() != operands.len() {
                        return Err(SUPPORT_SOURCE_MISSING);
                    }
                    let values: Vec<f64> = sources.iter().map(|s| s["value"].as_f64()).collect::<Option<_>>().ok_or(SUPPORT_SOURCE_MISSING)?;
                    let value = expression.evaluate(&values).ok_or(ALGEBRA_REFUSED)?;
                    let refs: Vec<String> = sources.iter().map(|s| s["id"].as_str().unwrap_or_default().to_string()).collect();
                    let evidence = operands.iter().zip(&refs).map(|(operand, id)| row_source(operand, id)).collect::<Vec<_>>();
                    (value, refs, evidence)
                } else {
                    let c = &combined[slots.clone()];
                    let value = norm3(c[0], c[1], c[2]);
                    if expression.unsigned() && !value.is_finite() {
                        return Err(ALGEBRA_REFUSED);
                    }
                    (value, component_ids[slots].to_vec(), Vec::new())
                };
                let row = combined_row(combination, expression, oq1, template, &key, value, refs);
                let evidence = json!({"result_id": row["id"], "sources": evidence_sources});
                out.push((row, evidence));
            }
            Ok(out)
        };
        match group() {
            Ok(group) => {
                for (row, evidence) in group {
                    built.rows.push(row);
                    built.row_evidence.push(evidence);
                }
            }
            Err(reason) => built.withhold(format!("support-action:{}:{}", support.len(), support), reason),
        }
    }
    // A magnitude row of an unsigned or nested combination states its derivation; a
    // flat combination's keeps the load case's absent metadata.
    if expression.unsigned() || operands.iter().any(|o| matches!(o, Operand::Combination(_))) {
        for row in built.rows.iter_mut().filter(|r| r["kind"] == "displacement_magnitude") {
            row["metadata"] = graph::derived_magnitude_metadata(expression.metadata_basis(), &expression.sign(oq1));
        }
    }
    built
}

// ------------------------------------------------------------------ the pass

/// The combination pass over a received pressure-2 document (§6), after its
/// direct-case pass.
pub(crate) fn validate(source: &Value) -> Check {
    let evidence = &source["contract_evidence"];
    let records = array(&evidence["exact_combinations"], "EVIDENCE_SHAPE")?;
    require(!records.is_empty(), "EVIDENCE_SHAPE")?;
    let policy = &crate::semantic_contract::pressure_rows_contract()["combination_policy"];
    let listed = |key: &str| -> Vec<&str> { items(&policy[key]).iter().filter_map(Value::as_str).collect() };
    let (linear, recomputed) = (listed("linear_state_kinds"), listed("recomputed_magnitude_kinds"));
    require(!linear.is_empty() && !recomputed.is_empty(), "POLICY")?;
    let results = array(&source["results"], "ROWS")?;
    let diagnostics = array(&source["diagnostics"], "DIAGNOSTICS")?;
    let quality = items(&source["numerical_quality"]["cases"]);
    let case_ids: HashSet<&str> = items(&evidence["exact_cases"]).iter().filter_map(|c| c["load_case_id"].as_str()).collect();
    let case_rows_present = |kinds: &[&str]| results.iter().any(|r| case_of(r).is_some() && kinds.iter().any(|k| r["kind"] == *k));
    let (has_maximum, has_intensified) = (case_rows_present(&MAXIMUM_KINDS), case_rows_present(&INTENSIFIED_KINDS));
    // Gate 4's input on the received document: a consuming constant-effort
    // support publishes its applied-load row in every solved case.
    // `model_checks` binds those rows, case by case, to the model's DEC-049
    // decision, so with the captured model the gate is decided by the model.
    let constant_effort = case_rows_present(&[APPLIED_LOAD]);

    let mut expected_rows: Vec<Value> = Vec::new();
    // Each expected diagnostic whole (id, code, severity, message, source and
    // affected refs), as the producer states it.
    let mut expected_diagnostics: Vec<Value> = Vec::new();
    let mut modulus_rows: Vec<Value> = Vec::new();
    let mut ids: Vec<&str> = Vec::new();
    // absolute-sum-srss §2-§5: the records in evaluation order, κ and the reached load
    // cases of each, the modulus key of every load-case operand stated so far, and
    // the combinations withheld whole with their codes.
    let mut combination_graph = Graph::default();
    let nonlinear = results.iter().any(|r| case_of(r).is_some() && r["kind"] == STATE_CODE);
    let mut modulus: HashMap<&str, Option<String>> = HashMap::new();
    let mut withheld_combinations: Vec<(&str, &str)> = Vec::new();
    for record in records {
        require(keys(record, &RECORD_KEYS), "RECORD_SHAPE")?;
        let id = text(&record["combination_id"], "RECORD_SHAPE")?;
        // Ids are unique per entity kind: a combination may share a load case's id.
        require(!ids.contains(&id), "ID_DUPLICATE")?;
        ids.push(id);
        let basis = text(&record["basis"], "BASIS")?;
        let expression = Expression::read(basis, &record["expression"])?;
        let operand_ids = expression.operands(&case_ids);
        for operand in &operand_ids {
            match operand {
                Operand::Case(case) => require(case_ids.contains(case), "OPERAND_UNRESOLVED")?,
                // An operand combination is evaluated, and stated, before its dependants.
                Operand::Combination(c) => require(combination_graph.knows(c), "OPERAND_UNRESOLVED")?,
            }
        }
        let operands = array(&record["operands"], "OPERAND_SHAPE")?;
        require(operands.len() == operand_ids.len(), "OPERAND_SHAPE")?;
        for (operand, target) in operands.iter().zip(&operand_ids) {
            let case = match target {
                Operand::Combination(c) => {
                    require(bit_equal(operand, &json!({"combination_id": c})), "OPERAND_SHAPE")?;
                    continue;
                }
                Operand::Case(case) => *case,
            };
            require(keys(operand, &OPERAND_KEYS) && operand["load_case_id"] == case, "OPERAND_SHAPE")?;
            let key = modulus_key(operand)?;
            require(modulus.get(case).is_none_or(|known| *known == key), "OPERAND_SHAPE")?;
            modulus.insert(case, key);
            // §5: the operand's standing and numerical findings are its case's.
            let stated = quality.iter().find(|q| q["basis_ref"]["ref_type"] == "load_case" && q["basis_ref"]["ref_id"] == case);
            require(stated.is_some_and(|q| bit_equal(q, &operand["numerical_quality"])), "OPERAND_STANDING")?;
        }
        require(record["standing"] == STANDING, "STANDING")?;
        require(record["source_recovery"] == SOURCE_RECOVERY_EXCLUSION, "SOURCE_RECOVERY")?;
        let kappa = combination_graph.add(id, expression.basis(), &operand_ids, &expression.factors());
        if kappa.is_unrepresentable() {
            expected_diagnostics.push(json!({
                "id": format!("{DIAGNOSTIC_PREFIX}fixed-load-count:{}", identity(&[id])), "code": graph::FIXED_LOAD_COUNT_UNREPRESENTABLE, "severity": "info",
                "message": format!("combination {id}: its fixed-load count (the number of times it carries each fixed load) needs more than 4,096 significand bits exactly, so it is undefined; a gated use never treats an undefined count as passing"),
                "source": DIAGNOSTIC_SOURCE, "affected_refs": [id],
            }));
        }
        let reason = gate(id, &expression, &operand_ids, &combination_graph, &modulus, &withheld_combinations, evidence, nonlinear, constant_effort)?;
        require(bit_equal(&record["gate"], &json!({"withheld": reason.is_some(), "reason": reason.as_ref().map(|r| r.0)})), "GATE")?;
        if !graph::term_basis(expression.basis()) {
            // DEC-068 item 1: each load-case operand's solve basis, in authored order
            // (an operand combination records none).
            let authored: Vec<&str> = match &expression {
                Expression::Subtraction(m, s) => vec![m, s],
                Expression::Envelope { authored, .. } => authored.clone(),
                Expression::Terms(..) => Vec::new(),
            };
            for case in authored.into_iter().filter(|case| case_ids.contains(case)) {
                let label = results
                    .iter()
                    .filter(|r| r["kind"] == "modulus_basis_record" && r["entity_ref"] == case && case_of(r).is_some())
                    .last()
                    .and_then(|r| r["metadata"]["basis"].as_str())
                    .unwrap_or("material_base_values");
                modulus_rows.push(json!({
                    "id": format!("result:combination:{}:modulus-basis:{}", stable_suffix(id), stable_suffix(case)),
                    "kind": "combination_modulus_basis_record", "value": 1.0, "unit": "record", "entity_ref": case,
                    "basis_ref": {"ref_type": "combination", "ref_id": id},
                    "metadata": {"component": "material_modulus_basis", "coordinate_system": "not_applicable", "location": id,
                        "basis": label, "sign_convention": MODULUS_SIGN},
                }));
            }
        }
        if let Some((code, message, refs)) = reason {
            for member in ["rows", "recoveries", "missing_sources", "withheld"] {
                require(items(&record[member]).is_empty() && record[member].is_array(), "GATED_RECORD")?;
            }
            expected_diagnostics.push(json!({"id": format!("{DIAGNOSTIC_PREFIX}gate:{}", identity(&[id])), "code": code, "severity": "warning",
                "message": message, "source": DIAGNOSTIC_SOURCE, "affected_refs": refs}));
            withheld_combinations.push((id, code));
            continue;
        }
        // Decision OQ-1's rows state their own sign convention (§5.2).
        let oq1 = nonlinear && match &expression {
            Expression::Terms("mechanics", terms) => combination_graph.solved_state_plus_increments(terms),
            _ => false,
        };
        let built = derive(id, &expression, oq1, &operand_ids, results, evidence, &linear, &recomputed);
        require(bit_equal(&record["rows"], &json!(built.row_evidence)), "ROW_EVIDENCE")?;
        require(bit_equal(&record["recoveries"], &json!(built.recoveries)), "RECOVERY")?;
        require(bit_equal(&record["missing_sources"], &json!(built.missing)), "MISSING_SOURCES")?;
        require(bit_equal(&record["withheld"], &json!(built.withheld)), "WITHHELD")?;
        for missing in &built.missing {
            let source = missing["source_result_id"].as_str().unwrap_or_default();
            let cases = items(&missing["missing_load_case_ids"]).iter().filter_map(Value::as_str).collect::<Vec<_>>().join(", ");
            expected_diagnostics.push(diagnostic(format!("{DIAGNOSTIC_PREFIX}source-missing:{}", identity(&[id, source])), SOURCE_MISSING, "warning",
                format!("combination {id} omits {source} because a matching source row is missing in operand case {cases}"), id));
        }
        if !built.withheld.is_empty() {
            const NAMED: usize = 8;
            let mut named = built.withheld.iter().take(NAMED)
                .map(|w| format!("{} ({})", w["name"].as_str().unwrap_or_default(), w["reason"].as_str().unwrap_or_default()))
                .collect::<Vec<_>>().join(", ");
            if built.withheld.len() > NAMED {
                named.push_str(&format!(", and {} more", built.withheld.len() - NAMED));
            }
            expected_diagnostics.push(diagnostic(format!("{DIAGNOSTIC_PREFIX}row-withheld:{}", identity(&[id])), ROW_WITHHELD, "warning",
                format!("combination {id} withholds {} combined row(s) that could not be formed: {named}", built.withheld.len()), id));
        }
        expected_diagnostics.push(diagnostic(format!("{DIAGNOSTIC_PREFIX}derived:{}", identity(&[id])), DERIVED, "info", format!("combination {id} {DERIVED_TEXT}"), id));
        if has_maximum {
            expected_diagnostics.push(diagnostic(format!("{DIAGNOSTIC_PREFIX}maximum:{}", identity(&[id])), MAXIMUM_UNAVAILABLE, "info", MAXIMUM_TEXT.to_string(), id));
        }
        if has_intensified {
            expected_diagnostics.push(diagnostic(format!("{DIAGNOSTIC_PREFIX}intensified:{}", identity(&[id])), INTENSIFIED_UNAVAILABLE, "info", INTENSIFIED_TEXT.to_string(), id));
        }
        expected_rows.extend(built.rows);
    }
    expected_rows.extend(modulus_rows);
    // Complete coverage: exactly the re-derived rows, in order (no extra, no
    // omission, every provenance and value bit).
    let published: Vec<&Value> = results.iter().filter(|r| is_combination_row(r)).collect();
    require(published.len() == expected_rows.len(), "ROW_COVERAGE")?;
    for (row, expected) in published.iter().zip(&expected_rows) {
        require(bit_equal(row, expected), "ROW")?;
    }
    // The diagnostics that state the gates, the omissions, the standing and the
    // source-recovery exclusion: each once, and none other in this namespace.
    let stated: Vec<&Value> = diagnostics.iter().filter(|d| d["id"].as_str().is_some_and(|i| i.starts_with(DIAGNOSTIC_PREFIX))).collect();
    require(stated.len() == expected_diagnostics.len() + 1, "DIAGNOSTICS")?;
    expected_diagnostics.push(json!({
        "id": EXCLUDED_ID, "code": SOURCE_RECOVERY_EXCLUDED, "severity": "info", "message": EXCLUDED_TEXT,
        "source": DIAGNOSTIC_SOURCE, "affected_refs": ids,
    }));
    for expected in &expected_diagnostics {
        let matching: Vec<&&Value> = stated.iter().filter(|d| d["id"] == expected["id"]).collect();
        require(matching.len() == 1 && bit_equal(matching[0], expected), "DIAGNOSTICS")?;
    }
    Ok(())
}

/// One expected diagnostic, whole, as the producer's `diag` states it.
fn diagnostic(id: String, code: &str, severity: &str, message: String, combination: &str) -> Value {
    json!({"id": id, "code": code, "severity": severity, "message": message, "source": DIAGNOSTIC_SOURCE, "affected_refs": [combination]})
}

fn gate_message(code: &str) -> &'static str {
    match code {
        NONLINEAR => "Combination withheld: the model has nonlinear supports, so superposing case states is not a solved state.",
        MODULUS_BASIS_MIXED => "Combination withheld: its load cases were solved on different modulus bases.",
        RESOLVED_STIFFNESS_MIXED => "Combination withheld: a member's resolved stiffness (its selected E/nu, property-temperature selection, section bases or shear model) differs between its load cases, so superposing their states is not a state of one system.",
        CONSTANT_EFFORT => "Combination withheld: a constant-effort support would be counted by the sum of factors instead of once.",
        CONSTANT_EFFORT_UNSIGNED => "Combination withheld: a constant-effort support's force acts in every case state; an absolute sum or SRSS of operands whose fixed-load count is not exactly 0 would count it once per term with its sign discarded.",
        CONSTANT_EFFORT_OPERAND_MIXED => "Combination withheld: an operand's fixed-load count is undefined, so the number of times this combination counts a constant-effort support's force cannot be decided.",
        INSTALLED_REFERENCE_UNSIGNED => "Combination withheld: a connector or tie rod has a nonzero installed reference, which acts in every case state; an absolute sum or SRSS of operands whose fixed-load count is not exactly 0 would count it once per term with its sign discarded.",
        _ => "Combination withheld: a connector or tie rod has a nonzero installed reference, which superposition counts by the sum of the factors instead of once, and the factors do not sum to 1 within 64 epsilon.",
    }
}

/// G2: each record's expression and its operands' modulus selections are the
/// captured model's, one record per model combination in the model's evaluation
/// order (absolute-sum-srss §2: topological, the earliest ready in the array first).
/// A supplied model is complete: an absent or null `combinations` is the input's
/// default, none, so the document then states no combination record.
pub(crate) fn model_checks(source: &Value, model: &Value) -> Check {
    let combinations = match model.get("combinations") {
        None | Some(Value::Null) => &[][..],
        Some(Value::Array(list)) => list.as_slice(),
        Some(_) => return Err(fail("MODEL_COVERAGE")),
    };
    let records = items(&source["contract_evidence"]["exact_combinations"]);
    let cases = items(&model["load_cases"]);
    let case_ids: HashSet<&str> = cases.iter().filter_map(|c| c["id"].as_str()).collect();
    let order = graph::model_order(combinations, &case_ids);
    require(records.len() == combinations.len() && order.len() == combinations.len(), "MODEL_COVERAGE")?;
    for (record, &index) in records.iter().zip(&order) {
        let combination = &combinations[index];
        require(record["combination_id"] == combination["id"] && record["basis"] == combination["basis"], "MODEL_EXPRESSION")?;
        require(bit_equal(&record["expression"], &graph::evidence_expression(combination)), "MODEL_EXPRESSION")?;
        for operand in items(&record["operands"]) {
            if operand.get("combination_id").is_some() {
                continue;
            }
            let case = cases.iter().find(|c| c["id"] == operand["load_case_id"]).ok_or_else(|| fail("MODEL_OPERAND"))?;
            let reference = case.get("modulus_basis_ref").cloned().unwrap_or(Value::Null);
            // The producer records the selection temperature normalized to the
            // canonical unit (kelvin) by the units authority; the captured
            // quantity is normalized the same way before the comparison.
            let temperature = match case.get("modulus_basis_temperature").filter(|t| !t.is_null()) {
                Some(t) => {
                    let kelvin = crate::physics_source::normalized_quantity(t, Dimension::Temperature).map_err(|_| fail("MODEL_OPERAND"))?;
                    json!({"value": kelvin, "unit": canonical_unit(Dimension::Temperature).ok_or_else(|| fail("MODEL_OPERAND"))?.definition().symbol})
                }
                None => Value::Null,
            };
            require(bit_equal(&operand["modulus_basis_ref"], &reference) && bit_equal(&operand["modulus_basis_temperature"], &temperature), "MODEL_OPERAND")?;
        }
    }
    // Gate 4's input decided from the model (RT-A3 increment 3): every solved
    // case publishes exactly the model's consuming constant-effort supports'
    // applied-load rows, in model order, each the producer's whole row (id,
    // value bits, unit, entity, basis, location naming the support's node and
    // its normalized acting DOF, consumed-DOF basis, hanger references and
    // sign convention), and nothing else of that kind. A record dropped from a
    // case or state, one added for a non-consuming record, a changed force, or
    // a row whose node or DOF is not the model's (review of deafe7153: the
    // model's DOF changed, or the row relocated) is refused, so the received
    // document's consumption fact is the model's.
    let consuming = consuming_constant_effort(model);
    let results = items(&source["results"]);
    for (position, case) in items(&source["contract_evidence"]["exact_cases"]).iter().enumerate() {
        let id = case["load_case_id"].as_str().ok_or_else(|| fail("MODEL_CONSTANT_EFFORT"))?;
        let published: Vec<&Value> = results.iter().filter(|r| r["kind"] == APPLIED_LOAD && r["basis_ref"]["ref_type"] == "load_case" && r["basis_ref"]["ref_id"] == id).collect();
        require(published.len() == consuming.len(), "MODEL_CONSTANT_EFFORT")?;
        for (row, support) in published.iter().zip(&consuming) {
            require(bit_equal(row, &support.applied_load_row(id, position)), "MODEL_CONSTANT_EFFORT")?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exact(terms: &[(f64, f64)]) -> Option<f64> {
        let mut sum = ExactProducts::new();
        for (a, b) in terms {
            sum.add_product(*a, *b);
        }
        sum.round()
    }

    #[test]
    fn exact_products_round_once_across_the_whole_range() {
        let tiny = f64::from_bits(1);
        assert_eq!(exact(&[]), Some(0.0));
        assert_eq!(exact(&[(1.0, 1.0), (2f64.powi(-60), 1.0), (-1.0, 1.0)]), Some(2f64.powi(-60)));
        assert_eq!(exact(&[(f64::MAX, 1.0), (-f64::MAX, 1.0), (tiny, 1.0)]), Some(tiny));
        assert_eq!(exact(&[(f64::MAX, 2.0)]), None);
        assert_eq!(exact(&[(1.0, 2f64.powi(-53))]).map(|v| v + 1.0), Some(1.0));
        // Ties to even, and a further term below breaks the tie.
        assert_eq!(exact(&[(1.0, 1.0), (2f64.powi(-53), 1.0)]), Some(1.0));
        assert_eq!(exact(&[(1.0, 1.0), (2f64.powi(-53), 1.0), (2f64.powi(-100), 1.0)]), Some(1.0 + f64::EPSILON));
        assert_eq!(exact(&[(-1.5, 1.0), (-2f64.powi(-53), 1.0)]), Some(-1.5));
        // Products below the subnormal range: half of 2^-1074 ties to +0.0,
        // just above it rounds to 2^-1074; a negative underflow is +0.0.
        assert_eq!(exact(&[(2f64.powi(-537), 2f64.powi(-538))]).map(f64::to_bits), Some(0));
        assert_eq!(exact(&[(2f64.powi(-537), 2f64.powi(-538)), (2f64.powi(-600), 2f64.powi(-600))]), Some(tiny));
        assert_eq!(exact(&[(-tiny, 0.5)]).map(f64::to_bits), Some(0));
        // A cancelling combination is +0.0, never -0.0.
        assert_eq!(exact(&[(1.3, -0.0), (0.0, -2.0)]).map(f64::to_bits), Some(0));
        assert_eq!(exact(&[(1.3, 5.0), (-1.3, 5.0)]).map(f64::to_bits), Some(0));
        // fl(0.3 * 7) against the exact product.
        assert_eq!(exact(&[(0.3, 7.0)]), Some(0.3 * 7.0));
        // The exact sum 0.1·3 + 0.2·3 of the represented values is
        // 0.90000000000000004996..., nearest 0.9 (Fraction arithmetic agrees);
        // evaluating in binary64 step by step gives 0.9000000000000001.
        assert_eq!(exact(&[(0.1, 3.0), (0.2, 3.0)]), Some(0.9));
        assert_ne!(0.1 * 3.0 + 0.2 * 3.0, 0.9);
        // The smallest normal and the largest subnormal.
        assert_eq!(exact(&[(f64::MIN_POSITIVE, 1.0), (-tiny, 1.0)]), Some(f64::MIN_POSITIVE - tiny));
    }

    /// Review r1 (4): the magnitude derivation (`correct_norm::norm3`) is the
    /// correctly rounded norm across the whole binary64 range: subnormal,
    /// extreme and overflowing magnitudes. The vectors are shared with the Python
    /// mirror's test (`tests/test_exact_combination_verifier.py`), whose exact
    /// rational derivation computed the expected bits.
    #[test]
    fn magnitudes_are_correctly_rounded_across_the_binary64_range() {
        let tiny = f64::from_bits(1);
        assert_eq!(norm3(1e-200, 0.0, 0.0), 1e-200);
        assert_eq!(norm3(1e200, 0.0, 0.0), 1e200);
        assert_eq!(norm3(3.0 * 2f64.powi(-700), 4.0 * 2f64.powi(-700), 0.0), 5.0 * 2f64.powi(-700));
        assert_eq!(norm3(3.0 * 2f64.powi(900), 4.0 * 2f64.powi(900), 0.0), 5.0 * 2f64.powi(900));
        assert_eq!(norm3(3.0 * tiny, 4.0 * tiny, 0.0), 5.0 * tiny);
        assert_eq!(norm3(f64::MAX, 0.0, 0.0), f64::MAX);
        assert_eq!(norm3(f64::MAX, f64::MAX, 0.0), f64::INFINITY);
        assert_eq!(norm3(1e-160, 2e-160, 3e-160), 3.7416573867739414e-160);
        const SHARED: [(u64, u64, u64, u64); 28] = [
        (0x0000000000000b92, 0x0000000000000f04, 0x0000000000000f61, 0x0000000000001869),
        (0x0000000000000849, 0x000000000000079d, 0x000000000000071f, 0x0000000000000d51),
        (0x000000000000048b, 0x80000000000009ed, 0x00000000000000b6, 0x0000000000000af0),
        (0x00000000000004ee, 0x00000000000000e1, 0x0000000000000ee0, 0x0000000000000fb2),
        (0x000000009cb706ad, 0x0000000ae4ee4542, 0x00000003935dd386, 0x0000000b7b7cf22f),
        (0x000001900977a960, 0x00000000010b4404, 0x0000ea91ac1fffc3, 0x0000ea93013c3a59),
        (0x12f4f126169bc518, 0x12775fc622e83df0, 0x9342a0caf96608b6, 0x1342a3bc244895bc),
        (0x9427dd509407296c, 0x13bd702c24f33d74, 0x133cbb58103ef1fe, 0x1427dd99346cb9ef),
        (0x1def7d457c49b3b2, 0x1d43b757e7d6cc30, 0x1c9e59279ebd0e32, 0x1def7d45df0bc176),
        (0x9d851c287ef477d6, 0x1e2ab2359fe7d0f2, 0x1dcc86713d5c804a, 0x1e2ab329f71f3be1),
        (0xb2ab523aa02fae74, 0x3191f1b98c7c7bf0, 0x31a64f1c8b404c28, 0x32ab523aa03a4349),
        (0x331f0c9a81b18ad6, 0x3356486f444e6d9a, 0x321a36b0956309a0, 0x33565e06a1179d74),
        (0x3443e93c81d6850c, 0x3451aa1e9f48246f, 0x34e448162ec6d36e, 0x34e44818b77ae67f),
        (0x34275f3f1a850e88, 0xb570f21a420f8721, 0x35632bd65adaec3b, 0x3573780f45fd833d),
        (0x3e4cfe91246e8da2, 0xbe46a7b47543ddcc, 0x3ef48a4c3e3b547b, 0x3ef48a4cc2104c16),
        (0x3fde2089564330e0, 0x3f1675559add9dd8, 0x3e6399175cca8484, 0x3fde20895ea21ef5),
        (0x4966873c6e6adbac, 0x4816d35125b18c4b, 0x4937cfbd6c9bfc28, 0x4966b95a983d9649),
        (0x48afe453bb92c56e, 0x488bcb6014ff7ade, 0x49088cc1df49eccf, 0x49088e1d01b72aa7),
        (0x4c18dab51e28b8b7, 0x4bea1ea121fb7eb2, 0xcbe1b79efd72ad17, 0x4c192a5dee3e4eeb),
        (0x4aaba5665782a09e, 0x4ba836b2b4ab6f9d, 0x4bf2be53c0331d6a, 0x4bf2c23c503c7ecc),
        (0x5e630db06ce88204, 0x5f4b7e715406b1d8, 0x5ea000dd74717b6c, 0x5f4b7e719ef59e60),
        (0x5e0e70073d47ca1c, 0xde162e052f4006a2, 0x5fe25b5eb3550eb0, 0x5fe25b5eb3550eb0),
        (0x6b1c9f35a1b0dea9, 0xeb2daa52ff14040e, 0x6b4d3a270d46e7f0, 0x6b4e5d6c112b135d),
        (0x6b69d7d5d739a5e4, 0x6a3cddc541057a1c, 0x6ac78b5fc7293aca, 0x6b69d7d682d3ee73),
        (0x7cce80edb45874ba, 0x7cb308b1ce0b8024, 0x7e0ee68356bf64e1, 0x7e0ee68356bf7567),
        (0x7ca18db6dc4f8976, 0xfdc2f938bfdc0196, 0x7dfb057bf58412e6, 0x7dfb2014060080fb),
        (0xfed341cc49226afc, 0x7fd173bb005d3a1d, 0x7e216ffd5eec53cf, 0x7fd173bb0067d9f1),
        (0x7e5b9c93558f51b2, 0xfedc5d12426909ba, 0x7eb0ae96d781778f, 0x7edcab32098a2c82),
        ];
        for (x, y, z, r) in SHARED {
            let n = norm3(f64::from_bits(x), f64::from_bits(y), f64::from_bits(z));
            assert_eq!(n.to_bits(), r, "{x:016x} {y:016x} {z:016x}");
        }
    }

    #[test]
    fn the_unit_factor_sum_is_exact_and_inclusive() {
        let p46 = 2f64.powi(-46);
        for factors in [vec![1.0], vec![0.3, 0.7], vec![1.0, 2f64.powi(-54)], vec![1.0, p46], vec![1.0, -p46], vec![1.2, -0.2]] {
            assert!(unit_factor_sum(&factors), "{factors:?}");
        }
        for factors in [vec![2.0], vec![0.0], vec![2f64.powi(53), -2f64.powi(53)], vec![1.0, p46, 2f64.powi(-100)],
            vec![1.0, -p46, -2f64.powi(-100)], vec![f64::MAX, f64::MAX], vec![f64::NAN], vec![]] {
            assert!(!unit_factor_sum(&factors), "{factors:?}");
        }
    }

    /// Gate 4's model condition mirrors the producer's DEC-049 classification:
    /// the hanger-type reading, no nonlinear field, a finite positive force in
    /// newtons by the units authority, recognized DOFs with exactly one
    /// translational entry (case spellings counted as entered), a model node.
    #[test]
    fn constant_effort_consumption_is_decided_from_the_model() {
        let support = |patch: &dyn Fn(&mut Value)| {
            let mut s = json!({"id": "support:ce", "node": "node:b", "family": "constant_effort_support", "restraints": ["UZ"],
                "hanger": {"hanger_type": "constant_effort_support", "constant_load": {"value": 8.0, "unit": "kN"}}});
            patch(&mut s);
            json!({"nodes": [{"id": "node:a"}, {"id": "node:b"}], "supports": [{"id": "support:a", "node": "node:a", "family": "anchor", "restraints": ["UX"]}, s]})
        };
        let resolved = |patch: &dyn Fn(&mut Value)| {
            let model = support(patch);
            consuming_constant_effort(&model).iter().map(|s| (s.id.to_string(), s.node.to_string(), s.dof, s.force, s.source.to_string(), s.manufacturer.to_string(), s.load_side_review.to_string())).collect::<Vec<_>>()
        };
        let missing = |id: &str, node: &str, dof: &'static str| (id.to_string(), node.to_string(), dof, 8000.0, "source_reference_missing".to_string(), "manufacturer_reference_missing".to_string(), "load_side_review_reference_missing".to_string());
        assert_eq!(resolved(&|_| {}), [missing("support:ce", "node:b", "UZ")]);
        // The node and the normalized acting DOF are retained (a lower-case
        // spelling resolves to the producer's name).
        assert_eq!(resolved(&|s: &mut Value| s["restraints"] = json!(["ux", "RZ"])), [missing("support:ce", "node:b", "UX")]);
        assert_eq!(resolved(&|s: &mut Value| s["restraints"] = json!(["Uy"])), [missing("support:ce", "node:b", "UY")]);
        // The hanger references as the row states them: entered text kept
        // verbatim, blank text stated missing.
        let entered = resolved(&|s: &mut Value| {
            s["hanger"]["source_reference"] = json!(" catalog p. 4 ");
            s["hanger"]["manufacturer_reference"] = json!(" ");
            s["hanger"]["load_side_review_reference"] = json!("LSR-1");
        });
        assert_eq!((entered[0].4.as_str(), entered[0].5.as_str(), entered[0].6.as_str()), (" catalog p. 4 ", "manufacturer_reference_missing", "LSR-1"));
        let model = support(&|_| {});
        let row = consuming_constant_effort(&model)[0].applied_load_row("case:b", 1);
        assert_eq!(row["id"], "result:loadcase:case-b:constant-effort-support:support-ce:applied-load");
        assert_eq!(row["metadata"]["location"], "node:b:UZ applied load");
        assert_eq!(consuming_constant_effort(&model)[0].applied_load_row("case:a", 0)["id"], "result:constant-effort-support:support-ce:applied-load");
        // Consumed: a rotation beside the one translation; the type read from the family; a null nonlinear field.
        for patch in [
            &(|s: &mut Value| s["restraints"] = json!(["uz", "RX"])) as &dyn Fn(&mut Value),
            &|s: &mut Value| s["hanger"]["hanger_type"] = json!(" "),
            &|s: &mut Value| s["nonlinear"] = Value::Null,
        ] {
            assert_eq!(consuming_constant_effort(&support(patch)).len(), 1);
        }
        // Not consumed.
        for patch in [
            &(|s: &mut Value| s["restraints"] = json!(["UY", "UZ"])) as &dyn Fn(&mut Value),
            &|s: &mut Value| s["restraints"] = json!(["UZ", "uz"]),
            &|s: &mut Value| s["restraints"] = json!(["RX"]),
            &|s: &mut Value| s["restraints"] = json!(["UZ", "TX"]),
            &|s: &mut Value| s["hanger"]["constant_load"] = json!({"value": 0.0, "unit": "N"}),
            &|s: &mut Value| s["hanger"]["constant_load"] = json!({"value": -1.0, "unit": "N"}),
            &|s: &mut Value| s["hanger"]["constant_load"] = Value::Null,
            &|s: &mut Value| s["hanger"]["constant_load"] = json!({"value": 1.0, "unit": "m"}),
            &|s: &mut Value| s["nonlinear"] = json!({"behavior": "gap"}),
            &|s: &mut Value| s["node"] = json!("node:z"),
            &|s: &mut Value| s["hanger"]["hanger_type"] = json!("variable_spring_hanger"),
        ] {
            assert!(consuming_constant_effort(&support(patch)).is_empty());
        }
    }

    /// Review of deafe7153 (finding 1): the binding compares each case's
    /// applied-load rows with the producer's whole rows, written out here as
    /// PP publishes them for a two-case document. Forgery (a): the model's
    /// support acting along UX; forgery (b): a row relocated to another node
    /// and DOF; then one control per bound field, including those the
    /// direct-case pass refuses first on a full document. The Python mirror's
    /// test states the same controls.
    #[test]
    fn the_model_binding_binds_every_field_of_the_applied_load_rows() {
        const REFUSED: &str = "SOURCE_PHYSICS_COMBINATION_MODEL_CONSTANT_EFFORT";
        let model = |patch: &dyn Fn(&mut Value)| {
            let mut s = json!({"id": "support:ce", "node": "node:b", "family": "constant_effort_support", "restraints": ["UZ"],
                "hanger": {"hanger_type": "constant_effort_support", "constant_load": {"value": 8.0, "unit": "kN"}}});
            patch(&mut s);
            json!({"nodes": [{"id": "node:a"}, {"id": "node:b"}], "supports": [{"id": "support:a", "node": "node:a", "family": "anchor", "restraints": ["UX"]}, s]})
        };
        let row = |case: &str, id: &str| json!({"id": id, "kind": APPLIED_LOAD, "value": 8000.0, "unit": "N", "entity_ref": "support:ce",
            "basis_ref": {"ref_type": "load_case", "ref_id": case},
            "metadata": {"component": APPLIED_LOAD, "coordinate_system": "global", "location": "node:b:UZ applied load",
                "basis": "support_family=constant_effort_support;consumed_dof=UZ;source=source_reference_missing;manufacturer=manufacturer_reference_missing;load_side_review=load_side_review_reference_missing;mechanics_consumption=assembled_solve;dec_ref=DEC-049",
                "sign_convention": "positive value is the user-entered constant support force applied along the positive axis of the single declared translational restraint DOF in every solved load case; the ideal constant-effort element contributes zero stiffness and adds no solve restraint row; no gravity coupling, catalog/default value, or inferred direction is supplied"}});
        let source = || json!({"results": [row("case:A", "result:constant-effort-support:support-ce:applied-load"),
                row("case:B", "result:loadcase:case-B:constant-effort-support:support-ce:applied-load")],
            "contract_evidence": {"exact_cases": [{"load_case_id": "case:A"}, {"load_case_id": "case:B"}], "exact_combinations": []}});
        let base = model(&|_| {});
        assert_eq!(model_checks(&source(), &base), Ok(()));
        assert_eq!(model_checks(&source(), &model(&|s: &mut Value| s["restraints"] = json!(["uz"]))), Ok(()));
        // (a) The model's support changed to UX; the model's support moved
        // to another model node; its source reference entered.
        for patch in [
            &(|s: &mut Value| s["restraints"] = json!(["UX"])) as &dyn Fn(&mut Value),
            &|s: &mut Value| s["node"] = json!("node:a"),
            &|s: &mut Value| s["hanger"]["source_reference"] = json!("catalog"),
        ] {
            assert_eq!(model_checks(&source(), &model(patch)), Err(REFUSED.to_string()));
        }
        // (b) and one control per bound row field.
        let basis = row("case:A", "")["metadata"]["basis"].as_str().unwrap().to_string();
        for (pointer, value) in [
            ("/metadata/location", json!("node:A:UX applied load")),
            ("/metadata/location", json!("node:b:UX applied load")),
            ("/metadata/location", json!("node:a:UZ applied load")),
            ("/id", json!("result:forged")),
            ("/id", json!("result:loadcase:case-A:constant-effort-support:support-ce:applied-load")),
            ("/value", json!(8000.000000000001)),
            ("/unit", json!("kN")),
            ("/entity_ref", json!("support:a")),
            ("/metadata/component", json!("support_reaction_component_v2")),
            ("/metadata/coordinate_system", json!("local")),
            ("/metadata/basis", json!(basis.replace("consumed_dof=UZ", "consumed_dof=UX"))),
            ("/metadata/basis", json!(basis.replace("source=source_reference_missing", "source=catalog"))),
            ("/metadata/sign_convention", json!("negative value is the support force")),
        ] {
            let mut forged = source();
            *forged.pointer_mut(&format!("/results/0{pointer}")).unwrap() = value.clone();
            assert_eq!(model_checks(&forged, &base), Err(REFUSED.to_string()), "{pointer} {value}");
        }
        for key in ["source_result_refs", "component_body"] {
            let mut forged = source();
            forged["results"][0][key] = json!(["result:forged"]);
            assert_eq!(model_checks(&forged, &base), Err(REFUSED.to_string()), "{key}");
        }
        // The second case's id unqualified; a field removed; a row dropped; a row added.
        let mut forged = source();
        forged["results"][1]["id"] = json!("result:constant-effort-support:support-ce:applied-load");
        assert_eq!(model_checks(&forged, &base), Err(REFUSED.to_string()));
        let mut forged = source();
        forged["results"][1]["metadata"].as_object_mut().unwrap().remove("sign_convention");
        assert_eq!(model_checks(&forged, &base), Err(REFUSED.to_string()));
        let mut forged = source();
        forged["results"].as_array_mut().unwrap().remove(1);
        assert_eq!(model_checks(&forged, &base), Err(REFUSED.to_string()));
        let mut forged = source();
        let extra = forged["results"][0].clone();
        forged["results"].as_array_mut().unwrap().push(extra);
        assert_eq!(model_checks(&forged, &base), Err(REFUSED.to_string()));
    }

    #[test]
    fn combined_ids_are_length_prefixed() {
        let key = Key { kind: "k".into(), unit: "N".into(), entity: "p".into(), location: "end_i".into(), component: "c".into(), frame: "f".into(), section: String::new() };
        assert_eq!(key.combined_id("comb:1"), "result:combination:6:comb:11:k1:N1:p5:end_i1:c1:f0:");
        assert_eq!(identity(&["é"]), "2:é");
    }
}
