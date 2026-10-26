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
//! operand modulus selections to the captured model (G2).
//!
//! The algebra is this reader's own: a linear combination is the exact sum of
//! the exact products of the represented binary64 factors and operand values,
//! rounded once to nearest (ties to even, an exact or underflowed zero as +0.0);
//! a subtraction is one binary64 difference; a range envelope selects by mode,
//! ties to the smallest operand id; a recomputed magnitude is the correctly
//! rounded norm of the combined components. The unit factor-sum rule of gate 3
//! is decided on the exact sum of the factors. Nothing here calls product code.
//!
//! These checks establish internal source consistency, not solver accuracy.
//! Combination rows carry no numerical check of their own and never enter
//! numerical eligibility (§5), which stays with the primitive cases.
use crate::section_basis::bit_equal;
use open_pipe_stress_correct_norm::norm3;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

type Check = Result<(), String>;

pub(crate) const STANDING: &str = "derived_from_case_results";
pub(crate) const RECOVERY_METHOD: &str = "pressure_free_operand_recovery";
pub(crate) const SOURCE_RECOVERY_EXCLUSION: &str = "excluded: a document with a load combination attempts no retained-source recovery for any of its cases, primitive cases included; every case publishes ordinary structural recovery";
const MODULUS_BASIS_MIXED: &str = "COMBINATION_MODULUS_BASIS_MIXED";
const RESOLVED_STIFFNESS_MIXED: &str = "COMBINATION_RESOLVED_STIFFNESS_MIXED";
const INSTALLED_REFERENCE: &str = "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE";
const DERIVED: &str = "EXACT_COMBINATION_DERIVED_FROM_CASE_RESULTS";
const SOURCE_RECOVERY_EXCLUDED: &str = "EXACT_COMBINATION_SOURCE_RECOVERY_EXCLUDED";
const SOURCE_MISSING: &str = "LOAD_COMBINATION_SOURCE_RESULT_MISSING";
const ROW_WITHHELD: &str = "LOAD_COMBINATION_ROW_WITHHELD";
const MAXIMUM_UNAVAILABLE: &str = "COMBINATION_STRESS_MAXIMUM_UNAVAILABLE";
const INTENSIFIED_UNAVAILABLE: &str = "COMBINATION_INTENSIFIED_STRESS_UNAVAILABLE";
const DIAGNOSTIC_PREFIX: &str = "diagnostic:exact-combination:";
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
const INTENSIFIED_KINDS: [&str; 2] = [
    "curved_bend_directional_intensified_bending_stress_v1",
    "curved_bend_directional_intensified_bending_maximum_v1",
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
    source["contract_evidence"].get("exact_combinations").is_some()
        || items(&source["results"]).iter().any(is_combination_row)
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

/// §4 gate 3's rule: the exact sum of the factors is within 2^-46 of 1, compared
/// exactly and inclusively.
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
    Mechanics(Vec<(&'a str, f64)>),
    Subtraction(&'a str, &'a str),
    Envelope { authored: Vec<&'a str>, sorted: Vec<&'a str>, mode: Mode },
}

impl<'a> Expression<'a> {
    fn read(basis: &str, v: &'a Value) -> Result<Self, String> {
        match basis {
            "mechanics" => {
                require(keys(v, &["terms"]), "EXPRESSION_SHAPE")?;
                let mut terms = Vec::new();
                for term in array(&v["terms"], "EXPRESSION_SHAPE")? {
                    require(keys(term, &["load_case_id", "factor"]), "EXPRESSION_SHAPE")?;
                    let factor = term["factor"].as_f64().filter(|f| f.is_finite()).ok_or_else(|| fail("EXPRESSION_FACTOR"))?;
                    let case = text(&term["load_case_id"], "EXPRESSION_SHAPE")?;
                    require(terms.iter().all(|(c, _)| *c != case), "EXPRESSION_DUPLICATE")?;
                    terms.push((case, factor));
                }
                require(!terms.is_empty(), "EXPRESSION_SHAPE")?;
                Ok(Expression::Mechanics(terms))
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
    /// The operand order: term order, minuend then subtrahend, or sorted ids.
    fn operands(&self) -> Vec<&'a str> {
        match self {
            Expression::Mechanics(terms) => terms.iter().map(|(c, _)| *c).collect(),
            Expression::Subtraction(m, s) => vec![m, s],
            Expression::Envelope { sorted, .. } => sorted.clone(),
        }
    }
    fn range(&self) -> bool {
        matches!(self, Expression::Envelope { .. })
    }
    fn metadata_basis(&self) -> &'static str {
        match self {
            Expression::Mechanics(_) => LINEAR_BASIS,
            Expression::Subtraction(..) => SUBTRACTION_BASIS,
            Expression::Envelope { .. } => RANGE_BASIS,
        }
    }
    fn sign(&self) -> String {
        match self {
            Expression::Mechanics(_) => LINEAR_SIGN.to_string(),
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
            Expression::Mechanics(terms) => {
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
        let (location, component, frame, section) = match row.get("metadata") {
            Some(md) => {
                let basis = md["basis"].as_str()?;
                (field(&md["location"])?, field(&md["component"])?, field(&md["coordinate_system"])?,
                    basis.split_once(SECTION_SEPARATOR).map_or(String::new(), |(_, s)| s.to_string()))
            }
            None if kind == "displacement_magnitude" => Default::default(),
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

fn case_rows<'s>(results: &'s [Value], case: &str) -> CaseRows<'s> {
    let mut by_key: HashMap<Key, Vec<&Value>> = HashMap::new();
    let mut order = Vec::new();
    for row in results.iter().filter(|r| case_of(r) == Some(case)) {
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

fn gate(expression: &Expression<'_>, operands: &[&Value], cases: &[CaseFacts<'_>], connector: &Value) -> Result<Option<&'static str>, String> {
    let Expression::Mechanics(terms) = expression else { return Ok(None) };
    let mut modulus = HashSet::new();
    for operand in operands {
        modulus.insert(modulus_key(operand)?);
    }
    if modulus.len() > 1 {
        return Ok(Some(MODULUS_BASIS_MIXED));
    }
    let mut members: Vec<&str> = Vec::new();
    for case in cases {
        for pipe in &case.members {
            if !members.contains(pipe) {
                members.push(pipe);
            }
        }
    }
    for pipe in members {
        let first = stiffness(&cases[0], pipe);
        if cases[1..].iter().any(|case| !value_eq(&stiffness(case, pipe), &first)) {
            return Ok(Some(RESOLVED_STIFFNESS_MIXED));
        }
    }
    let factors: Vec<f64> = terms.iter().map(|(_, f)| *f).collect();
    if installed_reference(connector) && !unit_factor_sum(&factors) {
        return Ok(Some(INSTALLED_REFERENCE));
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
            let (n, sign) = if key.location == "end_i" { (-value, "negated") } else { (value, "unchanged") };
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

fn combined_row(combination: &str, expression: &Expression<'_>, template: &Value, key: &Key, value: f64, refs: Vec<String>) -> Value {
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
        row["metadata"]["sign_convention"] = json!(expression.sign());
    }
    row
}

fn derive(combination: &str, expression: &Expression<'_>, operands: &[&str], results: &[Value], evidence: &Value, linear: &[&str], recomputed: &[&str]) -> Built {
    let range = expression.range();
    let rows: Vec<CaseRows<'_>> = operands.iter().map(|case| case_rows(results, case)).collect();
    let facts: Vec<CaseFacts<'_>> = operands.iter().map(|case| case_facts(evidence, case)).collect();
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
        for (index, case) in operands.iter().enumerate() {
            if let Some(found) = rows[index].by_key.get(key) {
                let Some(value) = found[0]["value"].as_f64() else {
                    missing.push(case.to_string());
                    continue;
                };
                values.push(value);
                let id = found[0]["id"].as_str().unwrap_or_default().to_string();
                sources.push(json!({"load_case_id": case, "result_id": id}));
                refs.push(id);
            } else if let Some((value, witness)) = recover(key, case, &rows[index], &facts[index]) {
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
                            _ => operands.iter().map(|c| c.to_string()).collect(),
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
            let refs = components.iter().map(|c| c.combined_id(combination)).collect();
            let row = combined_row(combination, expression, template, key, norm3(vector[0], vector[1], vector[2]), refs);
            built.row_evidence.push(json!({"result_id": row["id"], "sources": []}));
            built.rows.push(row);
            continue;
        }
        match outcomes.remove(key) {
            Some(Outcome::Row { value, sources, refs }) => {
                let row = combined_row(combination, expression, template, key, value, refs);
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
                let row = combined_row(combination, expression, sources[0], &key, value, refs.clone());
                component_ids.push(row["id"].as_str().unwrap_or_default().to_string());
                let evidence = json!({"result_id": row["id"], "sources": operands.iter().zip(&refs).map(|(case, id)| json!({"load_case_id": case, "result_id": id})).collect::<Vec<_>>()});
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
                    let evidence = operands.iter().zip(&refs).map(|(case, id)| json!({"load_case_id": case, "result_id": id})).collect::<Vec<_>>();
                    (value, refs, evidence)
                } else {
                    let c = &combined[slots.clone()];
                    (norm3(c[0], c[1], c[2]), component_ids[slots].to_vec(), Vec::new())
                };
                let row = combined_row(combination, expression, template, &key, value, refs);
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

    let mut expected_rows: Vec<Value> = Vec::new();
    let mut expected_diagnostics: Vec<(String, &str, &str)> = Vec::new();
    let mut modulus_rows: Vec<Value> = Vec::new();
    let mut ids: Vec<&str> = Vec::new();
    for record in records {
        require(keys(record, &RECORD_KEYS), "RECORD_SHAPE")?;
        let id = text(&record["combination_id"], "RECORD_SHAPE")?;
        require(!ids.contains(&id) && !case_ids.contains(id), "ID_DUPLICATE")?;
        ids.push(id);
        let basis = text(&record["basis"], "BASIS")?;
        let expression = Expression::read(basis, &record["expression"])?;
        let operand_ids = expression.operands();
        require(operand_ids.iter().all(|c| case_ids.contains(c)), "OPERAND_UNRESOLVED")?;
        let operands = array(&record["operands"], "OPERAND_SHAPE")?;
        require(operands.len() == operand_ids.len(), "OPERAND_SHAPE")?;
        let mut operand_records = Vec::new();
        for (operand, case) in operands.iter().zip(&operand_ids) {
            require(keys(operand, &OPERAND_KEYS) && operand["load_case_id"] == *case, "OPERAND_SHAPE")?;
            modulus_key(operand)?;
            // §5: the operand's standing and numerical findings are its case's.
            let stated = quality.iter().find(|q| q["basis_ref"]["ref_type"] == "load_case" && q["basis_ref"]["ref_id"] == *case);
            require(stated.is_some_and(|q| bit_equal(q, &operand["numerical_quality"])), "OPERAND_STANDING")?;
            operand_records.push(operand);
        }
        require(record["standing"] == STANDING, "STANDING")?;
        require(record["source_recovery"] == SOURCE_RECOVERY_EXCLUSION, "SOURCE_RECOVERY")?;
        let facts: Vec<CaseFacts<'_>> = operand_ids.iter().map(|case| case_facts(evidence, case)).collect();
        let reason = gate(&expression, &operand_records, &facts, &evidence["connector"])?;
        require(bit_equal(&record["gate"], &json!({"withheld": reason.is_some(), "reason": reason})), "GATE")?;
        if !matches!(expression, Expression::Mechanics(_)) {
            // DEC-068 item 1: each operand's solve basis, in authored order.
            let authored: Vec<&str> = match &expression {
                Expression::Subtraction(m, s) => vec![m, s],
                Expression::Envelope { authored, .. } => authored.clone(),
                Expression::Mechanics(_) => Vec::new(),
            };
            for case in authored {
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
        if let Some(code) = reason {
            for member in ["rows", "recoveries", "missing_sources", "withheld"] {
                require(items(&record[member]).is_empty() && record[member].is_array(), "GATED_RECORD")?;
            }
            expected_diagnostics.push((format!("{DIAGNOSTIC_PREFIX}gate:{}", identity(&[id])), code, "warning"));
            continue;
        }
        let built = derive(id, &expression, &operand_ids, results, evidence, &linear, &recomputed);
        require(bit_equal(&record["rows"], &json!(built.row_evidence)), "ROW_EVIDENCE")?;
        require(bit_equal(&record["recoveries"], &json!(built.recoveries)), "RECOVERY")?;
        require(bit_equal(&record["missing_sources"], &json!(built.missing)), "MISSING_SOURCES")?;
        require(bit_equal(&record["withheld"], &json!(built.withheld)), "WITHHELD")?;
        for missing in &built.missing {
            let source = missing["source_result_id"].as_str().unwrap_or_default();
            expected_diagnostics.push((format!("{DIAGNOSTIC_PREFIX}source-missing:{}", identity(&[id, source])), SOURCE_MISSING, "warning"));
        }
        if !built.withheld.is_empty() {
            expected_diagnostics.push((format!("{DIAGNOSTIC_PREFIX}row-withheld:{}", identity(&[id])), ROW_WITHHELD, "warning"));
        }
        expected_diagnostics.push((format!("{DIAGNOSTIC_PREFIX}derived:{}", identity(&[id])), DERIVED, "info"));
        if has_maximum {
            expected_diagnostics.push((format!("{DIAGNOSTIC_PREFIX}maximum:{}", identity(&[id])), MAXIMUM_UNAVAILABLE, "info"));
        }
        if has_intensified {
            expected_diagnostics.push((format!("{DIAGNOSTIC_PREFIX}intensified:{}", identity(&[id])), INTENSIFIED_UNAVAILABLE, "info"));
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
    for (id, code, severity) in &expected_diagnostics {
        require(stated.iter().filter(|d| d["id"] == id.as_str()).count() == 1
            && stated.iter().any(|d| d["id"] == id.as_str() && d["code"] == *code && d["severity"] == *severity), "DIAGNOSTICS")?;
    }
    let excluded = stated.iter().find(|d| d["id"] == EXCLUDED_ID).ok_or_else(|| fail("DIAGNOSTICS"))?;
    require(excluded["code"] == SOURCE_RECOVERY_EXCLUDED && excluded["severity"] == "info" && excluded["affected_refs"] == json!(ids), "DIAGNOSTICS")
}

/// G2: each record's expression and its operands' modulus selections are the
/// captured model's, one record per model combination in model order. Runs
/// when the model states its combinations.
pub(crate) fn model_checks(source: &Value, model: &Value) -> Check {
    let Some(combinations) = model.get("combinations").filter(|v| !v.is_null()) else { return Ok(()) };
    let combinations = items(combinations);
    let records = items(&source["contract_evidence"]["exact_combinations"]);
    require(records.len() == combinations.len(), "MODEL_COVERAGE")?;
    let cases = items(&model["load_cases"]);
    for (record, combination) in records.iter().zip(combinations) {
        require(record["combination_id"] == combination["id"] && record["basis"] == combination["basis"], "MODEL_EXPRESSION")?;
        let expression = match combination["basis"].as_str() {
            Some("mechanics") => json!({"terms": items(&combination["terms"]).iter().map(|t| json!({"load_case_id": t["load_case"], "factor": t["factor"]})).collect::<Vec<_>>()}),
            Some("result_state_subtraction") => json!({"minuend_id": combination["minuend_id"], "subtrahend_id": combination["subtrahend_id"]}),
            _ => json!({"operand_ids": combination["operand_ids"], "mode": combination["mode"]}),
        };
        require(bit_equal(&record["expression"], &expression), "MODEL_EXPRESSION")?;
        for operand in items(&record["operands"]) {
            let case = cases.iter().find(|c| c["id"] == operand["load_case_id"]).ok_or_else(|| fail("MODEL_OPERAND"))?;
            let reference = case.get("modulus_basis_ref").cloned().unwrap_or(Value::Null);
            let temperature = match case.get("modulus_basis_temperature").filter(|t| !t.is_null()) {
                Some(t) => json!({"value": t["value"], "unit": t["unit"]}),
                None => Value::Null,
            };
            require(bit_equal(&operand["modulus_basis_ref"], &reference) && bit_equal(&operand["modulus_basis_temperature"], &temperature), "MODEL_OPERAND")?;
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
        assert_eq!(exact(&[(0.1, 3.0), (0.2, 3.0)]), Some(0.9000000000000001));
        // The smallest normal and the largest subnormal.
        assert_eq!(exact(&[(f64::MIN_POSITIVE, 1.0), (-tiny, 1.0)]), Some(f64::MIN_POSITIVE - tiny));
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

    #[test]
    fn combined_ids_are_length_prefixed() {
        let key = Key { kind: "k".into(), unit: "N".into(), entity: "p".into(), location: "end_i".into(), component: "c".into(), frame: "f".into(), section: String::new() };
        assert_eq!(key.combined_id("comb:1"), "result:combination:6:comb:11:k1:N1:p5:end_i1:c1:f0:");
        assert_eq!(identity(&["é"]), "2:é");
    }
}
