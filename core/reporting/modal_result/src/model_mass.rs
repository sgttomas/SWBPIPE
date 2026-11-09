//! Model-derived mass assurance (DEL-04-09 Design, "Selected mass assurance:
//! next modal unit after CE", with §1.3, §2.1–§2.4 and §5.3.4). With the bound
//! request supplied, the reader decodes the request's model itself and forms
//! the global mass M it implies, then holds the document's formed entries,
//! T_k, the free-DOF mass, the mass centre and every mass-dependent modal
//! quantity (m_i, Γ, m_eff and the fractions, from the published shapes) to
//! that M. Checking totals or M·R_k alone cannot see a coherently wrong M
//! whose rotary, Rayleigh, torsional or coupling entries were changed with
//! matching totals; the full M is compared entry by entry.
//!
//! **Independence.** Nothing here calls product preparation, self-weight,
//! eligibility, the kernel's mass re-formation or the Python reader. The model
//! is read from its JSON; units go through the shared unit catalog
//! (`convert_for_dimension`, the generic conversion every consumer uses). The
//! formulas are written from the Design's contract, coordinate-free:
//! - a straight member's consistent mass in global axes is
//!   B·I + (A − B)·e eᵀ on translations, D·I + (T − D)·e eᵀ on rotations and
//!   C·[e]×ᵀ on translation–rotation, with e the unit chord and A, T, B, D, C
//!   the axial, torsional and plane (Hermitian plus Rayleigh) coefficients
//!   (Design §2.2). A circular section's mass is invariant under rotation
//!   about the chord, so the y reference does not enter;
//! - a realized arc is two rigid half-arcs, each [[m I, −[S]×], [[S]×, J]]
//!   about its end node, from the closed-form integrals over the half-angle α
//!   in the node's tangent/normal/binormal frame, with the cancelling ones
//!   (α − sin α, α/2 − sin 2α/4, 3α/2 − 2 sin α + sin 2α/4) summed as
//!   alternating series with their remainder bound (Design §2.4);
//! - a component body is m = W/a at its global centre-of-gravity offset r:
//!   [[m I, −m[r]×], [m[r]×, m(|r|²I − rrᵀ)]] (Design §2.3).
//!
//! **Rounding (sound, derived from the arithmetic; no new tolerance).** Every
//! quantity is an interval with binary64 ends, every operation rounded
//! outward, so it holds the real value of the contract formula on the decoded
//! inputs (π itself is the interval [fl(π), fl(π)⁺]). Round-to-nearest is
//! monotone, so a binary64 evaluation of an operation on operands inside
//! these intervals rounds into the result's interval. The producer's
//! section scalars (m̄, j̄; Design §2.1–§2.2) are binary64; their products
//! and sums are allowed any association, through Higham's (1 ± u)^(k−1)
//! bound for k nonnegative factors or terms. The global entries are formed
//! by the producer in double-double and rounded once; that formation is
//! covered by the stated premise of [`mm_dd_path`] per unit of each
//! contribution's absolute scale (times 1 + |y|/|y⊥| for a member frame) and
//! per accumulated contribution, and the final rounding by monotonicity. A
//! published value outside its interval is an established contradiction
//! (failed); a published point inside it passes; a published set (a flagged
//! or outside-the-profile value) that overlaps it, or an interval that is not
//! finite, is unresolved, never passed.
//!
//! **Coverage.** No request, a request that is not the bound one, a withheld
//! payload or a model member outside this reader's basis is scoped
//! unsupported, by name. A solved document whose bound model the contract
//! refuses (a missing density, half an insulation pair, an exact section
//! without its corrosion allowance, a mapping conflict, a skewed support, a
//! Timoshenko selection, condition contents on the state case, …) fails.
//! A partition derived from reported nonlinear states is conditional on
//! their qualification, which no payload establishes: it is checked and
//! stays scoped. Omitted real-world masses are not checked: this verifies
//! the entered model's mass only.
use super::{mr_overlap, mr_reach, Num, Parsed, Reach, COMPONENTS, DIRECTIONS, MR_UNRESOLVED, R, U};
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, HashMap, HashSet};

/// The model document version this reader's basis covers.
pub(super) const MM_SCHEMA_VERSION: &str = "0.9.0";
/// The model's declared top-level members (`PreviewModel`'s wire); none of
/// them carries mass except as read below. Any other member is outside this
/// reader's basis and is scoped, never ignored.
pub(super) const MM_MODEL_KEYS: [&str; 24] = [
    "pressure_contract", "schema_version", "document_kind", "project", "analysis_status", "nodes", "pipe_segments", "sections",
    "supports", "components", "materials", "load_cases", "combinations", "reference_configurations", "model_settings",
    "condition_sets", "spectra", "data_boundary", "diagnostics", "lines", "unresolved_assumptions", "constraints",
    "equipment_interfaces", "rule_pack_refs",
];
pub(super) const MM_PIPE_KEYS: [&str; 12] =
    ["id", "from", "to", "section", "section_ref", "condition_set_ref", "material", "y_reference", "provenance", "label", "tags", "notes"];
const MM_SECTION_KEYS: [&str; 8] = [
    "outside_diameter", "wall_thickness", "mill_tolerance", "corrosion_allowance", "material_density", "contents_density",
    "insulation_thickness", "insulation_density",
];
pub(super) const MM_RIGID_FAMILY: [&str; 5] = ["valve", "flange", "reducer", "rigid", "specialty"];
pub(super) const MM_GEOMETRY_ONLY: &str = "mechanics_geometry_only";
pub(super) const MM_ARC: &str = "curved_bend_macro_element";
const MM_TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
/// The component-weight primitives' method (§2.3 item 5): their recorded
/// acceleration agreement is not re-derived here.
const MM_COMPONENT_WEIGHT_METHOD: &str = "component_weight_at_center_of_gravity_global_offset/v1";

/// The premise on the producer's double-double formation (the kernel's
/// element mass re-formation, Design §2.2–§2.4): each double-double operation
/// has relative error below 2⁻¹⁰⁰ and no entry's evaluation path exceeds 2¹⁰
/// operations (frame, local entry, rotation, the series summed to 2⁻¹¹⁰), so
/// one contribution deviates from the real value of its binary64 operands by
/// at most 2⁻⁹⁰·(1 + κ)·Σ|terms|, κ = |y|/|y⊥| the frame's conditioning, and
/// each accumulation adds at most that again. It is far below binary64
/// rounding and decides only where terms cancel to an exact zero.
fn mm_dd_path() -> f64 {
    2f64.powi(-90)
}

/// 2³⁷ = 2⁻⁵³/2⁻⁹⁰: where the premise's allowance reaches binary64 resolution.
const MM_DD_RESOLUTION_LIMIT: f64 = 137438953472.0;
/// 2⁻⁹⁶⁹: below it a double word's low word is no longer normal (the kernel's
/// arc guard, `LOCAL_ENTRY_MIN`), so the relative premise does not hold.
const MM_DD_LOW_WORD_FLOOR: f64 = 2.004168360008973e-292;

/// An absolute bound, at 2⁻¹⁰⁶⁴·max(1, 1/|divisor|), on what one double-word
/// operation loses to gradual underflow (each binary64 operation up to 2⁻¹⁰⁷⁵,
/// a subnormal sum exactly; at most a dozen sites in the kernel's divide). The
/// centre's path is plain double word and takes it; the participation path is
/// the kernel's extended-exponent `WideDd`, which does not underflow.
fn mm_dd_absolute(count: f64, divisor: f64) -> f64 {
    let scaled = mm_up(count * 2f64.powi(-1000) / divisor.min(1.0));
    mm_up(scaled * 2f64.powi(-64))
}

// ---------------------------------------------------------------------------
// Outward-rounded intervals.

fn mm_down(x: f64) -> f64 {
    if x.is_nan() {
        f64::NEG_INFINITY
    } else if x == f64::NEG_INFINITY {
        x
    } else {
        x.next_down()
    }
}

fn mm_up(x: f64) -> f64 {
    if x.is_nan() {
        f64::INFINITY
    } else if x == f64::INFINITY {
        x
    } else {
        x.next_up()
    }
}

/// A closed interval of reals with binary64 ends. Every operation rounds its
/// ends outward by one step, which covers a round-to-nearest error; a product
/// with an exact zero is exactly zero, so structural zeros stay exact.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Iv {
    lo: f64,
    hi: f64,
}

impl Iv {
    const ZERO: Iv = Iv { lo: 0.0, hi: 0.0 };
    const WILD: Iv = Iv { lo: f64::NEG_INFINITY, hi: f64::INFINITY };

    fn pt(x: f64) -> Iv {
        Iv { lo: x, hi: x }
    }
    fn is_zero(&self) -> bool {
        self.lo == 0.0 && self.hi == 0.0
    }
    /// The least magnitude in the interval.
    fn mig(&self) -> f64 {
        if self.lo <= 0.0 && 0.0 <= self.hi {
            0.0
        } else {
            self.lo.abs().min(self.hi.abs())
        }
    }
    fn finite(&self) -> bool {
        self.lo.is_finite() && self.hi.is_finite()
    }
    fn mag(&self) -> f64 {
        self.lo.abs().max(self.hi.abs())
    }
    fn add(self, o: Iv) -> Iv {
        if self.is_zero() {
            return o;
        }
        if o.is_zero() {
            return self;
        }
        Iv { lo: mm_down(self.lo + o.lo), hi: mm_up(self.hi + o.hi) }
    }
    fn neg(self) -> Iv {
        Iv { lo: -self.hi, hi: -self.lo }
    }
    fn sub(self, o: Iv) -> Iv {
        self.add(o.neg())
    }
    fn mul(self, o: Iv) -> Iv {
        if self.is_zero() || o.is_zero() {
            return Iv::ZERO;
        }
        if !self.finite() || !o.finite() {
            return Iv::WILD;
        }
        let p = [self.lo * o.lo, self.lo * o.hi, self.hi * o.lo, self.hi * o.hi];
        let lo = p.iter().fold(f64::INFINITY, |a, &x| a.min(x));
        let hi = p.iter().fold(f64::NEG_INFINITY, |a, &x| a.max(x));
        Iv { lo: mm_down(lo), hi: mm_up(hi) }
    }
    fn div(self, o: Iv) -> Iv {
        if !(o.lo > 0.0 || o.hi < 0.0) || !self.finite() || !o.finite() {
            return Iv::WILD;
        }
        if self.is_zero() {
            return Iv::ZERO;
        }
        let q = [self.lo / o.lo, self.lo / o.hi, self.hi / o.lo, self.hi / o.hi];
        let lo = q.iter().fold(f64::INFINITY, |a, &x| a.min(x));
        let hi = q.iter().fold(f64::NEG_INFINITY, |a, &x| a.max(x));
        Iv { lo: mm_down(lo), hi: mm_up(hi) }
    }
    fn sqrt(self) -> Iv {
        if self.is_zero() {
            return Iv::ZERO;
        }
        if !(self.lo >= 0.0) || !self.hi.is_finite() {
            return Iv::WILD;
        }
        Iv { lo: mm_down(self.lo.sqrt()).max(0.0), hi: mm_up(self.hi.sqrt()) }
    }
    fn square(self) -> Iv {
        if self.lo >= 0.0 {
            self.mul(self)
        } else if self.hi <= 0.0 {
            self.neg().mul(self.neg())
        } else {
            let m = Iv::pt(self.mag());
            Iv { lo: 0.0, hi: m.mul(m).hi }
        }
    }
    /// Grown by `a` ≥ 0 on each side.
    fn widen(self, a: f64) -> Iv {
        if a == 0.0 {
            return self;
        }
        Iv { lo: mm_down(self.lo - a), hi: mm_up(self.hi + a) }
    }
    fn half(self) -> Iv {
        self.mul(Iv::pt(0.5))
    }
}

/// [(1 − u)^k, (1 + u)^k], each end rounded outward.
fn mm_growth(k: usize) -> (f64, f64) {
    let (mut lo, mut hi) = (1.0f64, 1.0f64);
    for _ in 0..k {
        lo = mm_down(lo - lo * U);
        hi = mm_up(hi + hi * U);
    }
    (lo, hi)
}

/// Scales a nonnegative-valued enclosure by [(1 − u)^k, (1 + u)^k]: Higham's
/// bound for k roundings of products or sums of nonnegative operands, in any
/// association or order.
fn mm_rounded(x: Iv, k: usize) -> Iv {
    if x.is_zero() || k == 0 {
        return x;
    }
    let (g_lo, g_hi) = mm_growth(k);
    let lo = if x.lo >= 0.0 { x.lo * g_lo } else { x.lo * g_hi };
    let hi = if x.hi >= 0.0 { x.hi * g_hi } else { x.hi * g_lo };
    Iv { lo: mm_down(lo), hi: mm_up(hi) }
}

/// Nonnegative factors multiplied by the producer in binary64, any association.
fn mm_product_any(factors: &[Iv]) -> Iv {
    let mut p = Iv::pt(1.0);
    for f in factors {
        p = p.mul(*f);
    }
    mm_rounded(p, factors.len().saturating_sub(1))
}

/// Nonnegative terms summed by the producer in binary64, any order.
fn mm_sum_any(terms: &[Iv]) -> Iv {
    let mut s = Iv::ZERO;
    for t in terms {
        s = s.add(*t);
    }
    mm_rounded(s, terms.len().saturating_sub(1))
}

/// π, as the interval [fl(π), fl(π)⁺] (fl(π) < π): it holds both the real
/// constant and the binary64 one the producer uses.
fn mm_pi() -> Iv {
    Iv { lo: std::f64::consts::PI, hi: std::f64::consts::PI.next_up() }
}

// ---------------------------------------------------------------------------
// Series with their alternating remainder bounds (Design §2.4).

/// f₁(x) = x − sin x = Σ_{k≥1} (−1)^{k+1} x^{2k+1}/(2k+1)!, for 0 ≤ x ≤ π.
/// The terms decrease from the first (x² < 4·5), so the remainder after a
/// partial sum is at most the next term's magnitude.
fn mm_f1(x: Iv) -> Iv {
    if !(x.lo >= 0.0 && x.hi <= 3.2) {
        return Iv::WILD;
    }
    if x.is_zero() {
        return Iv::ZERO;
    }
    let x2 = x.mul(x);
    let mut term = x.mul(x2).div(Iv::pt(6.0));
    let mut sum = Iv::ZERO;
    for k in 1..=40u32 {
        sum = if k % 2 == 1 { sum.add(term) } else { sum.sub(term) };
        let denominator = f64::from((2 * k + 2) * (2 * k + 3));
        let next = term.mul(x2).div(Iv::pt(denominator));
        if next.hi <= sum.hi * 2f64.powi(-80) {
            return sum.widen(next.hi);
        }
        term = next;
    }
    Iv::WILD
}

/// f₃(x) = 3x/2 − 2 sin x + sin 2x/4 = Σ_{k≥2} (−1)^k (2^{2k−1} − 2) x^{2k+1}/(2k+1)!,
/// for 0 ≤ x < π/2. The terms decrease from k = 2 (ratio at most
/// 5x²/42 < 1, then below 4.2x²/72), so the remainder is at most the next term.
fn mm_f3(x: Iv) -> Iv {
    if !(x.lo >= 0.0 && x.hi <= 1.6) {
        return Iv::WILD;
    }
    if x.is_zero() {
        return Iv::ZERO;
    }
    let x2 = x.mul(x);
    // p = x^{2k+1}/(2k+1)!, from k = 2.
    let mut p = x.mul(x2).mul(x2).div(Iv::pt(120.0));
    let mut sum = Iv::ZERO;
    for k in 2..=25u32 {
        let coefficient = Iv::pt(2f64.powi(2 * k as i32 - 1) - 2.0);
        let term = p.mul(coefficient);
        sum = if k % 2 == 0 { sum.add(term) } else { sum.sub(term) };
        let next_p = p.mul(x2).div(Iv::pt(f64::from((2 * k + 2) * (2 * k + 3))));
        let next = next_p.mul(Iv::pt(2f64.powi(2 * k as i32 + 1) - 2.0));
        if next.hi <= sum.hi * 2f64.powi(-80) {
            return sum.widen(next.hi);
        }
        p = next_p;
    }
    Iv::WILD
}

/// sin x = x − f₁(x), for 0 ≤ x ≤ π/2.
fn mm_sin(x: Iv) -> Iv {
    x.sub(mm_f1(x))
}

/// α = asin s for 0 < s < 1 (α in (0, π/2)): the library's asin of each end,
/// stepped outward until the series enclosure of sin verifies
/// sin(α_lo) ≤ s_lo and sin(α_hi) ≥ s_hi (sin is increasing there).
fn mm_asin(s: Iv) -> Option<Iv> {
    if !(s.lo > 0.0 && s.hi < 1.0) {
        return None;
    }
    // Candidates widen geometrically (relative 2^(i−52)), since near π/2 sin
    // is flat and fixed ulp steps cannot cross the series width; only the
    // verification, not the candidate, carries the enclosure.
    let (base_lo, base_hi) = (s.lo.asin(), s.hi.asin());
    let mut lo = None;
    for i in 0..48 {
        let candidate = mm_down(base_lo - base_lo * 2f64.powi(i - 52));
        if candidate > 0.0 && mm_sin(Iv::pt(candidate)).hi <= s.lo {
            lo = Some(candidate);
            break;
        }
    }
    for i in 0..48 {
        let candidate = mm_up(base_hi + base_hi * 2f64.powi(i - 52));
        if candidate >= std::f64::consts::FRAC_PI_2 {
            break;
        }
        if mm_sin(Iv::pt(candidate)).lo >= s.hi {
            return lo.map(|lo| Iv { lo, hi: candidate });
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Decoding.

/// Why a derivation stopped.
enum Stop {
    /// Outside this reader's basis, or a required input absent: scoped.
    Scope(String),
    /// The contract refuses the bound model, or it contradicts the
    /// document's own basis: an established contradiction.
    Fail(String),
    /// Decidable only beyond the sound rounding bounds: unresolved.
    Open(String),
}

type S<T> = Result<T, Stop>;

fn mm_scope<T>(message: impl Into<String>) -> S<T> {
    Err(Stop::Scope(message.into()))
}

fn mm_fail<T>(message: impl Into<String>) -> S<T> {
    Err(Stop::Fail(message.into()))
}

fn mm_obj<'v>(value: &'v Value, what: &str) -> S<&'v Map<String, Value>> {
    value.as_object().ok_or_else(|| Stop::Fail(format!("{what}: not an object")))
}

fn mm_str<'v>(value: &'v Value, what: &str) -> S<&'v str> {
    value.as_str().ok_or_else(|| Stop::Fail(format!("{what}: not a string")))
}

fn mm_list<'v>(value: Option<&'v Value>, what: &str) -> S<&'v [Value]> {
    match value {
        None | Some(Value::Null) => Ok(&[]),
        Some(Value::Array(items)) => Ok(items.as_slice()),
        Some(_) => mm_fail(format!("{what}: not an array")),
    }
}

/// A value in `unit` converted to `dimension`'s canonical unit through the
/// unit catalog, finite.
fn mm_convert(value: f64, unit: &str, dimension: Dimension, what: &str) -> S<f64> {
    let from = unit_by_symbol(unit, dimension).map_err(|e| Stop::Fail(format!("{what}: `{unit}` is not a catalog unit ({e})")))?;
    let to = canonical_unit(dimension).ok_or_else(|| Stop::Scope(format!("{what}: the catalog has no canonical unit")))?;
    let converted = convert_for_dimension(value, dimension, from, to).map_err(|e| Stop::Fail(format!("{what}: {e}")))?;
    if !converted.is_finite() {
        return mm_fail(format!("{what}: does not normalize to a finite value"));
    }
    Ok(converted)
}

/// A `{value, unit}` quantity, normalized.
fn mm_quantity(value: &Value, dimension: Dimension, what: &str) -> S<f64> {
    let map = mm_obj(value, what)?;
    let number = map.get("value").and_then(Value::as_f64).filter(|v| v.is_finite());
    let unit = map.get("unit").and_then(Value::as_str);
    match (number, unit) {
        (Some(number), Some(unit)) => mm_convert(number, unit, dimension, what),
        _ => mm_fail(format!("{what}: not a finite quantity with a unit")),
    }
}

fn mm_optional_quantity(map: &Map<String, Value>, key: &str, dimension: Dimension, what: &str) -> S<Option<f64>> {
    match map.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(q) => mm_quantity(q, dimension, &format!("{what}.{key}")).map(Some),
    }
}

fn mm_vec3(value: &Value, what: &str) -> S<[f64; 3]> {
    let map = mm_obj(value, what)?;
    let mut out = [0.0; 3];
    for (k, axis) in ["x", "y", "z"].iter().enumerate() {
        out[k] = map.get(*axis).and_then(Value::as_f64).filter(|v| v.is_finite()).ok_or_else(|| Stop::Fail(format!("{what}.{axis}: not a finite number")))?;
    }
    Ok(out)
}

/// The global DOF index of a DOF token, as the model's parser reads it.
fn mm_dof(token: &str) -> Option<usize> {
    match token {
        "UX" | "Ux" | "ux" => Some(0),
        "UY" | "Uy" | "uy" => Some(1),
        "UZ" | "Uz" | "uz" => Some(2),
        "RX" | "Rx" | "rx" => Some(3),
        "RY" | "Ry" | "ry" => Some(4),
        "RZ" | "Rz" | "rz" => Some(5),
        _ => None,
    }
}

/// A member's section scalars: m̄ (kg/m) and j̄ (kg·m, polar per length);
/// ī = j̄/2 (Design §2.2). Enclosures of the real values and of the
/// producer's binary64 evaluation.
struct SectionMass {
    mbar: Iv,
    jbar: Iv,
}

/// m̄ and j̄ of a pipe section (Design §2.1, §2.2): metal on the route's mass
/// wall (t − c_m on PF, t on EX), contents on the bore wall (t − c_m on PF,
/// (t − c_m) − c_a on EX), insulation outside; annulus areas and polar
/// moments in their cancellation-free forms π·w·(D − w), π(D/2 − w)²,
/// π·d·(D + d), π/8·(D² + (D − 2w)²)(D − w)·w and π/16·(o² + D²)(o + D)·d.
/// Contents enter translation only (D-e).
fn mm_section(section: &Map<String, Value>, exact: bool, what: &str) -> S<SectionMass> {
    if let Some(key) = section.keys().find(|k| !MM_SECTION_KEYS.contains(&k.as_str())) {
        return mm_scope(format!("{what}: section member `{key}` is outside this reader's mass basis"));
    }
    let od = mm_quantity(section.get("outside_diameter").unwrap_or(&Value::Null), Dimension::Length, &format!("{what}.outside_diameter"))?;
    let t = mm_quantity(section.get("wall_thickness").unwrap_or(&Value::Null), Dimension::Length, &format!("{what}.wall_thickness"))?;
    let mill = mm_optional_quantity(section, "mill_tolerance", Dimension::Length, what)?.unwrap_or(0.0);
    if !(mill >= 0.0) {
        return mm_fail(format!("{what}: the mill deduction is negative, which the contract refuses"));
    }
    let density = mm_optional_quantity(section, "material_density", Dimension::Density, what)?;
    let Some(density) = density.filter(|d| *d > 0.0) else {
        return mm_fail(format!("{what}: no positive material density, which the contract requires (never a zero default)"));
    };
    let contents = mm_optional_quantity(section, "contents_density", Dimension::Density, what)?;
    let insulation_t = mm_optional_quantity(section, "insulation_thickness", Dimension::Length, what)?;
    let insulation_d = mm_optional_quantity(section, "insulation_density", Dimension::Density, what)?;
    if contents.is_some_and(|c| c < 0.0) || insulation_d.is_some_and(|c| c < 0.0) || insulation_t.is_some_and(|c| c < 0.0) {
        return mm_fail(format!("{what}: a negative density or insulation thickness, which the contract refuses"));
    }
    let insulation = match (insulation_t, insulation_d) {
        (None, None) => None,
        (Some(thickness), Some(density)) => Some((thickness, density)),
        _ => return mm_fail(format!("{what}: half an insulation pair, which the contract refuses (never a zero default)")),
    };
    let (t_iv, mill_iv, od_iv) = (Iv::pt(t), Iv::pt(mill), Iv::pt(od));
    // One binary64 subtraction each; the interval holds its rounding.
    let reduced = if mill == 0.0 { t_iv } else { t_iv.sub(mill_iv) };
    let (metal_wall, bore_wall) = if exact {
        let allowance = mm_optional_quantity(section, "corrosion_allowance", Dimension::Length, what)?;
        let Some(allowance) = allowance else {
            return mm_fail(format!("{what}: an exact-route section without its explicit corrosion allowance, which the contract refuses"));
        };
        if !(allowance >= 0.0) {
            return mm_fail(format!("{what}: a negative corrosion allowance, which the contract refuses"));
        }
        let bore = if allowance == 0.0 { reduced } else { reduced.sub(Iv::pt(allowance)) };
        (t_iv, bore)
    } else {
        (reduced, reduced)
    };
    // The source-area validity premise (w > 0, w ≤ D − w), decided on the
    // enclosures: a wall that cannot be valid is refused by the contract.
    for (wall, name) in [(metal_wall, "metal wall"), (bore_wall, "bore wall")] {
        let room = od_iv.sub(wall);
        if wall.hi <= 0.0 || room.sub(wall).hi < 0.0 {
            return mm_fail(format!("{what}: its {name} is not a valid annulus wall, which the contract refuses"));
        }
        if !(wall.lo > 0.0 && room.sub(wall).lo >= 0.0) {
            return Err(Stop::Open(format!("{what}: its {name} is within rounding of the annulus validity bound")));
        }
    }
    let pi = mm_pi();
    let metal_area = mm_product_any(&[pi, metal_wall, od_iv.sub(metal_wall)]);
    let mut terms = vec![mm_product_any(&[metal_area, Iv::pt(density)])];
    if let Some(c) = contents {
        let radius = od_iv.half().sub(bore_wall);
        let radius = Iv { lo: radius.lo.max(0.0), hi: radius.hi };
        let bore_area = mm_product_any(&[pi, radius, radius]);
        terms.push(mm_product_any(&[bore_area, Iv::pt(c)]));
    }
    if let Some((thickness, d)) = insulation {
        let d_iv = Iv::pt(thickness);
        let area = mm_product_any(&[pi, d_iv, od_iv.add(d_iv)]);
        terms.push(mm_product_any(&[area, Iv::pt(d)]));
    }
    let mbar = mm_sum_any(&terms);
    // j̄ = ρ_m·J_metal + ρ_ins·J_ins; contents excluded (D-e).
    let inner = od_iv.sub(metal_wall.mul(Iv::pt(2.0)));
    let sum_squares = od_iv.square().add(inner.square());
    let metal_polar = mm_product_any(&[pi.mul(Iv::pt(0.125)), sum_squares, od_iv.sub(metal_wall), metal_wall]);
    let mut polar_terms = vec![mm_product_any(&[metal_polar, Iv::pt(density)])];
    if let Some((thickness, d)) = insulation {
        let d_iv = Iv::pt(thickness);
        let outer = od_iv.add(d_iv.mul(Iv::pt(2.0)));
        let squares = outer.square().add(od_iv.square());
        let polar = mm_product_any(&[pi.mul(Iv::pt(0.0625)), squares, outer.add(od_iv), d_iv]);
        polar_terms.push(mm_product_any(&[polar, Iv::pt(d)]));
    }
    let jbar = mm_sum_any(&polar_terms);
    if !(mbar.finite() && jbar.finite() && mbar.lo > 0.0) {
        return Err(Stop::Open(format!("{what}: its mass per length is outside the binary64 range this reader encloses")));
    }
    Ok(SectionMass { mbar, jbar })
}

/// What a mass-bearing member contributes.
enum Kind {
    Straight,
    Arc { radius: f64 },
    Body { component: String, node: usize, mass: Iv, cog: [f64; 3] },
}

struct Member {
    id: String,
    from: usize,
    to: usize,
    y: Option<[f64; 3]>,
    kind: Kind,
}

/// One accumulated entry: the real enclosure of its contributions, the
/// absolute double-double scale and the number of contributions.
#[derive(Clone, Copy)]
struct Acc {
    v: Iv,
    scale: f64,
    n: usize,
    /// The largest 1 + κ of the entry's contributions.
    cond: f64,
    /// Whether a contribution left the rounding model (an intermediate that
    /// may lie below the low-word floor).
    floor: bool,
}

/// The derived model mass and partition.
pub(super) struct Formed {
    ndof: usize,
    /// Upper entries (row ≤ col): the enclosure of the producer's formed
    /// entry (its double-double value, and that value rounded).
    entries: HashMap<(usize, usize), Iv>,
    /// The entries' positions, ascending: every sum runs in this order, so a
    /// verdict does not depend on hashing.
    order: Vec<(usize, usize)>,
    /// Per global DOF, restrained.
    restrained: Vec<bool>,
    /// Why the partition is conditional (reported nonlinear states).
    conditional: Option<String>,
    /// Coverage left open by this derivation, scoped by name.
    scope: Vec<String>,
    /// The entries outside the rounding model, ascending, with the reason: each
    /// is enclosed by the whole line, so what uses it is unresolved and every
    /// other comparison is still made.
    open: Vec<((usize, usize), &'static str)>,
    positions: Vec<[f64; 3]>,
    /// The formed members as (kind, id): `straight_member` or `arc` by pipe id,
    /// `component_body` by component id (the census pass's self-check).
    pub(super) subjects: Vec<(&'static str, String)>,
}

/// The model-derived basis, or why it is not available.
pub(super) enum Derived {
    Formed(Box<Formed>),
    Scoped(String),
    Unresolved(String),
    Failed(String),
}

/// Derives the bound request's model mass (module docs). `bound` is whether
/// the binding standing passed against the supplied request.
pub(super) fn mm_derive(p: &Parsed, request: Option<&Value>, bound: bool) -> Derived {
    let Some(request) = request else {
        return Derived::Scoped("no request is supplied, so the model's mass is not derived".to_string());
    };
    if !bound {
        return Derived::Scoped("the supplied request is not the bound request, so the model's mass is not derived from it".to_string());
    }
    match mm_form(p, request) {
        Ok(formed) => Derived::Formed(Box::new(formed)),
        Err(Stop::Scope(s)) => Derived::Scoped(s),
        Err(Stop::Open(s)) => Derived::Unresolved(s),
        Err(Stop::Fail(s)) => Derived::Failed(s),
    }
}

fn mm_form(p: &Parsed, request: &Value) -> S<Formed> {
    let model = mm_obj(request.get("model").unwrap_or(&Value::Null), "request.model")?;
    if model.get("schema_version").and_then(Value::as_str) != Some(MM_SCHEMA_VERSION) {
        return mm_scope(format!("model schema_version is not {MM_SCHEMA_VERSION}, the version this reader's mass basis covers"));
    }
    if let Some(key) = model.keys().find(|k| !MM_MODEL_KEYS.contains(&k.as_str())) {
        return mm_scope(format!("model member `{key}` is outside this reader's mass basis"));
    }
    let mut scope = Vec::new();

    // The route (Design §2.1, D-r) and the shear model (Timoshenko is refused).
    let contract = model.get("pressure_contract").filter(|c| !c.is_null());
    let exact = match contract {
        None => false,
        Some(c) => {
            let version = c.get("version").and_then(Value::as_str);
            let mode = c.get("mode").and_then(Value::as_str);
            if version != Some("3.0.0") || mode != Some("exact_pressure_v3") {
                return mm_scope("the pressure contract is not exact_pressure_v3 3.0.0, the exact route this reader covers");
            }
            true
        }
    };
    let route = if exact { "exact_pressure" } else { "pressure_free" };
    if p.route != route {
        return mm_fail(format!("the document's route `{}` is not the bound model's `{route}`", p.route));
    }
    let shear = if exact {
        contract.and_then(|c| c.get("shear_model")).and_then(Value::as_str)
    } else {
        model.get("model_settings").and_then(|s| s.get("shear_model")).and_then(Value::as_str)
    };
    match shear {
        None | Some("euler_bernoulli") => {}
        Some(MM_TIMOSHENKO) => return mm_fail("the bound model selects Timoshenko members, which modal analysis refuses (MODAL_TIMOSHENKO_MASS_UNSUPPORTED)"),
        Some(other) => return mm_scope(format!("shear model `{other}` is outside this reader's mass basis")),
    }

    // Nodes: the model's order, ids and normalized coordinates are the document's.
    let nodes = mm_list(model.get("nodes"), "model.nodes")?;
    if nodes.len() != p.node_ids.len() {
        return mm_fail(format!("the model has {} nodes, the document {}", nodes.len(), p.node_ids.len()));
    }
    let length_unit = model.get("project").and_then(|x| x.get("units")).and_then(|x| x.get("length")).and_then(Value::as_str);
    let Some(length_unit) = length_unit else {
        return mm_fail("the model states no project.units.length, which node coordinates require");
    };
    let mut positions = Vec::new();
    positions.try_reserve(nodes.len()).map_err(|_| Stop::Scope("node positions: allocation refused".to_string()))?;
    let mut node_index: HashMap<&str, usize> = HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        let id = mm_str(node.get("id").unwrap_or(&Value::Null), &format!("model.nodes[{i}].id"))?;
        if id != p.node_ids[i] {
            return mm_fail(format!("node {i} is `{id}` in the model and `{}` in the document", p.node_ids[i]));
        }
        let raw = mm_vec3(node.get("position").unwrap_or(&Value::Null), &format!("node {id} position"))?;
        let mut x = [0.0; 3];
        for k in 0..3 {
            x[k] = mm_convert(raw[k], length_unit, Dimension::Length, &format!("node {id} position"))?;
            if p.nodes[i][k] != Some(x[k]) {
                return mm_fail(format!("node {id}: its published coordinate {} is not the model's {}", DIRECTIONS[k], x[k]));
            }
        }
        if node_index.insert(id, i).is_some() {
            return mm_fail(format!("node id `{id}` is not unique"));
        }
        positions.push(x);
    }
    let ndof = nodes.len().checked_mul(6).ok_or_else(|| Stop::Scope("DOF count overflows".to_string()))?;

    // The state case (Design §1.4): condition contents are refused.
    let cases = mm_list(model.get("load_cases"), "model.load_cases")?;
    if let Some(state) = request.get("state_case").and_then(Value::as_str) {
        let case = cases.iter().find(|c| c.get("id").and_then(Value::as_str) == Some(state));
        let Some(case) = case else {
            return mm_fail(format!("state case `{state}` is not a case of the bound model"));
        };
        if case.get("condition_use").and_then(|u| u.get("contents_density")).is_some_and(|v| !v.is_null()) {
            return mm_fail(format!("state case `{state}` takes condition contents, which modal analysis refuses (LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED)"));
        }
    }
    for case in cases {
        for load in mm_list(case.get("primitive_loads"), "primitive_loads")? {
            if load.get("provenance").and_then(Value::as_str).is_some_and(|text| text.contains(MM_COMPONENT_WEIGHT_METHOD)) {
                scope.push("the agreement of a and the model's applied component-weight primitives (Design §2.3 item 5) is not re-derived".to_string());
            }
        }
    }
    scope.dedup();

    // Components: objective connectors' replaced spans, realized arcs and
    // rigid-family bodies.
    let components = mm_list(model.get("components"), "model.components")?;
    let pipes = mm_list(model.get("pipe_segments"), "model.pipe_segments")?;
    let pipe_ids: HashSet<&str> = pipes.iter().filter_map(|p| p.get("id").and_then(Value::as_str)).collect();
    if pipe_ids.len() != pipes.len() {
        return mm_fail("a pipe id is missing or not unique");
    }
    let mut replaced: HashSet<&str> = HashSet::new();
    let mut arcs: HashMap<&str, (usize, f64)> = HashMap::new();
    for (i, component) in components.iter().enumerate() {
        let kind = component.get("kind").and_then(Value::as_str).unwrap_or("");
        let consumption = component.get("mechanics_interface").and_then(|m| m.get("solver_consumption")).and_then(Value::as_str);
        if component.get("objective_connector").is_some_and(|c| !c.is_null()) {
            if kind != "expansion_joint" {
                return mm_scope(format!("component {i} carries an objective connector but is not an expansion joint"));
            }
            if !exact {
                return mm_scope("an objective connector on the pressure-free route is outside this reader's mass basis");
            }
            let span = component.get("objective_connector").and_then(|c| c.get("topology")).and_then(|t| t.get("span_ref")).and_then(Value::as_str);
            match span {
                Some(span) if pipe_ids.contains(span) => {
                    if !replaced.insert(span) {
                        return mm_fail(format!("pipe {span} is replaced by two connectors, which the contract refuses"));
                    }
                }
                _ => return mm_scope(format!("component {i}: its connector's replaced span is not decoded by this reader")),
            }
        }
        if matches!(kind, "bend" | "elbow") && consumption.unwrap_or(MM_GEOMETRY_ONLY) == MM_ARC {
            let geometry = component.get("geometry");
            let pipe = geometry.and_then(|g| g.get("bend_pipe_ref")).and_then(Value::as_str).filter(|s| !s.trim().is_empty());
            let radius = geometry.and_then(|g| g.get("bend_radius"));
            let (Some(pipe), Some(radius)) = (pipe, radius) else {
                return mm_fail(format!("bend component {i} requests a realized arc without its pipe or radius, which the contract refuses"));
            };
            let radius = mm_quantity(radius, Dimension::Length, &format!("bend component {i} radius"))?;
            if !(radius > 0.0) {
                return mm_fail(format!("bend component {i}: its radius is not positive, which the contract refuses"));
            }
            if arcs.insert(pipe, (i, radius)).is_some() {
                return mm_scope(format!("pipe {pipe} is realized by two arcs, outside this reader's mass basis"));
            }
            let node = component.get("node").and_then(Value::as_str).unwrap_or("");
            let span = pipes.iter().find(|q| q.get("id").and_then(Value::as_str) == Some(pipe));
            let Some(span) = span else {
                return mm_fail(format!("bend component {i} names pipe {pipe}, which is not in the model"));
            };
            if span.get("from").and_then(Value::as_str) != Some(node) && span.get("to").and_then(Value::as_str) != Some(node) {
                return mm_fail(format!("bend component {i}: its node is not an end of its arc span, which the contract refuses"));
            }
        }
    }
    // Rigid-family bodies (Design §2.3): the self-weight eligibility rule,
    // stated here independently. Mapping conflicts refuse; an unmet condition
    // keeps the member's pipe mass.
    let acceleration = match request.get("weight_to_mass_acceleration").filter(|a| !a.is_null()) {
        None => None,
        Some(q) => {
            let a = mm_quantity(q, Dimension::Acceleration, "weight_to_mass_acceleration")?;
            if !(a > 0.0) {
                return mm_fail("weight_to_mass_acceleration is not positive, which the request refuses");
            }
            Some(a)
        }
    };
    let mut mapped: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (i, component) in components.iter().enumerate() {
        let kind = component.get("kind").and_then(Value::as_str).unwrap_or("");
        if !MM_RIGID_FAMILY.contains(&kind) {
            continue;
        }
        let pipe = component.get("geometry").and_then(|g| g.get("rigid_pipe_ref")).and_then(Value::as_str).filter(|s| !s.trim().is_empty());
        if let Some(pipe) = pipe.filter(|p| pipe_ids.contains(p)) {
            if arcs.contains_key(pipe) || replaced.contains(pipe) {
                return mm_fail(format!("rigid component {i} maps pipe {pipe}, a realized arc or replaced span: a mapping conflict the contract refuses"));
            }
            mapped.entry(pipe).or_default().push(i);
        }
    }
    let mut bodies: HashMap<&str, (String, usize, Iv, [f64; 3])> = HashMap::new();
    for (pipe, list) in &mapped {
        if list.len() > 1 {
            return mm_fail(format!("pipe {pipe} is mapped by more than one rigid component: a mapping conflict the contract refuses"));
        }
        let component = &components[list[0]];
        let id = component.get("id").and_then(Value::as_str).unwrap_or("").to_string();
        let consumption = component.get("mechanics_interface").and_then(|m| m.get("solver_consumption")).and_then(Value::as_str);
        // `not_solver_consumed` is honoured silently; any other mode keeps the
        // pipe mass with the self-weight warning.
        if consumption != Some(MM_GEOMETRY_ONLY) {
            continue;
        }
        let node = component.get("node").and_then(Value::as_str).unwrap_or("");
        let span = pipes.iter().find(|q| q.get("id").and_then(Value::as_str) == Some(*pipe));
        let ends = span.map(|s| (s.get("from").and_then(Value::as_str), s.get("to").and_then(Value::as_str)));
        if ends.is_some_and(|(a, b)| a != Some(node) && b != Some(node)) {
            continue;
        }
        let geometry = component.get("geometry");
        // A weight that is not a finite positive force keeps the pipe mass.
        let weight = geometry.and_then(|g| g.get("weight")).filter(|w| !w.is_null()).and_then(|w| {
            let value = w.get("value").and_then(Value::as_f64)?;
            let unit = w.get("unit").and_then(Value::as_str)?;
            mm_convert(value, unit, Dimension::Force, "weight").ok()
        });
        let Some(weight) = weight.filter(|w| *w > 0.0) else { continue };
        let cog = match geometry.and_then(|g| g.get("center_of_gravity")).filter(|c| !c.is_null()) {
            None => continue,
            Some(v) => {
                let entered = [v.get("x"), v.get("y"), v.get("z")].map(|c| c.and_then(Value::as_f64));
                let unit = v.get("unit").and_then(Value::as_str);
                let (Some(unit), [Some(x), Some(y), Some(z)]) = (unit, entered) else { continue };
                if ![x, y, z].iter().all(|c| c.is_finite()) {
                    continue;
                }
                let mut r = [0.0; 3];
                let mut ok = true;
                for (k, c) in [x, y, z].iter().enumerate() {
                    match mm_convert(*c, unit, Dimension::Length, "center_of_gravity") {
                        Ok(v) => r[k] = v,
                        Err(_) => ok = false,
                    }
                }
                if !ok {
                    continue;
                }
                if [x, y, z].iter().zip(&r).any(|(e, n)| *e != 0.0 && *n == 0.0) {
                    return mm_fail(format!("component {id}: a nonzero centre-of-gravity component normalizes to zero, which the contract refuses"));
                }
                r
            }
        };
        let Some(&node_at) = node_index.get(node) else {
            return mm_fail(format!("component {id} names no node of the model"));
        };
        let Some(a) = acceleration else {
            return mm_fail(format!("component {id}'s weight enters the mass, but the request names no weight_to_mass_acceleration (MODAL_WEIGHT_ACCELERATION_REQUIRED)"));
        };
        // The §2.3 item 6 binary64 range premise, in the producer's stated order
        // (`component_range`): a body outside it is refused
        // (MODAL_COMPONENT_MASS_OUT_OF_RANGE), so a solved document bound to it is a
        // contradiction. A nonzero exact product that rounds to zero is out of range.
        let m = weight / a;
        let normal = |x: f64| x.is_finite() && x.abs() >= f64::MIN_POSITIVE;
        let zero_or_normal = |x: f64| x == 0.0 || normal(x);
        let mut premise = normal(weight) && weight > 0.0 && normal(a) && a > 0.0 && normal(m) && normal(a * m)
            && cog.iter().all(|x| x.is_finite());
        for i in 0..3 {
            let first = m * cog[i];
            premise &= zero_or_normal(first) && !(first == 0.0 && cog[i] != 0.0);
            for j in 0..3 {
                let second = first * cog[j];
                premise &= zero_or_normal(second) && !(second == 0.0 && first != 0.0 && cog[j] != 0.0);
            }
        }
        for i in 0..3 {
            let (j, k) = ((i + 1) % 3, (i + 2) % 3);
            premise &= (m * cog[j] * cog[j] + m * cog[k] * cog[k]).is_finite();
        }
        if !premise {
            return mm_fail(format!("component {id}: its body leaves the binary64 range premise (§2.3 item 6), which the producer refuses (MODAL_COMPONENT_MASS_OUT_OF_RANGE)"));
        }
        bodies.insert(*pipe, (id, node_at, Iv::pt(weight).div(Iv::pt(a)), cog));
    }

    // Shared sections (the bound section's OD and wall, with the inline cache
    // equal to it), then the members.
    let sections = mm_list(model.get("sections"), "model.sections")?;
    let mut members = Vec::new();
    members.try_reserve(pipes.len()).map_err(|_| Stop::Scope("members: allocation refused".to_string()))?;
    let mut section_mass: HashMap<usize, SectionMass> = HashMap::new();
    for (i, pipe) in pipes.iter().enumerate() {
        let map = mm_obj(pipe, &format!("model.pipe_segments[{i}]"))?;
        if let Some(key) = map.keys().find(|k| !MM_PIPE_KEYS.contains(&k.as_str())) {
            return mm_scope(format!("pipe member `{key}` is outside this reader's mass basis"));
        }
        let id = mm_str(map.get("id").unwrap_or(&Value::Null), "pipe id")?;
        if replaced.contains(id) {
            continue; // an objective connector's replaced span carries no mass
        }
        let end = |key: &str| -> S<usize> {
            let name = map.get(key).and_then(Value::as_str).unwrap_or("");
            node_index.get(name).copied().ok_or_else(|| Stop::Fail(format!("pipe {id}: its {key} node is not in the model")))
        };
        let (from, to) = (end("from")?, end("to")?);
        if from == to {
            return mm_fail(format!("pipe {id} joins a node to itself, which the contract refuses"));
        }
        let y = match map.get("y_reference").filter(|y| !y.is_null()) {
            None => None,
            Some(v) => Some(mm_vec3(v, &format!("pipe {id} y_reference"))?),
        };
        if let Some((node, component, mass, cog)) = bodies.get(id).map(|(c, n, m, r)| (*n, c.clone(), *m, *r)) {
            members.push(Member { id: id.to_string(), from, to, y, kind: Kind::Body { component, node, mass, cog } });
            continue;
        }
        let mut section = mm_obj(map.get("section").unwrap_or(&Value::Null), &format!("pipe {id} section"))?.clone();
        if let Some(reference) = map.get("section_ref").filter(|r| !r.is_null()) {
            let reference = mm_str(reference, &format!("pipe {id} section_ref"))?;
            let shared: Vec<&Value> = sections.iter().filter(|s| s.get("id").and_then(Value::as_str) == Some(reference)).collect();
            if shared.len() != 1 {
                return mm_fail(format!("pipe {id}: its bound section `{reference}` does not exist exactly once, which the contract refuses"));
            }
            let props = shared[0].get("properties").and_then(Value::as_object);
            let type_ok = shared[0].get("section_type").and_then(Value::as_str) == Some("pipe");
            let Some(props) = props.filter(|p| type_ok && p.keys().all(|k| k == "outside_diameter" || k == "wall_thickness")) else {
                return mm_fail(format!("pipe {id}: its bound section is not a pipe section of OD and wall only, which the contract refuses"));
            };
            for key in ["outside_diameter", "wall_thickness"] {
                let (Some(bound), Some(inline)) = (props.get(key), section.get(key)) else {
                    return mm_fail(format!("pipe {id}: its bound section lacks {key}, which the contract refuses"));
                };
                if bound != inline {
                    return mm_fail(format!("pipe {id}: its inline {key} contradicts its bound section (a stale cache the contract refuses)"));
                }
                section.insert(key.to_string(), bound.clone());
            }
        }
        let mass = mm_section(&section, exact, &format!("pipe {id}"))?;
        let kind = match arcs.get(id) {
            Some((_, radius)) => Kind::Arc { radius: *radius },
            None => Kind::Straight,
        };
        if y.is_none() {
            return mm_fail(format!("pipe {id} has no y_reference, which the mass formation requires"));
        }
        section_mass.insert(members.len(), mass);
        members.push(Member { id: id.to_string(), from, to, y, kind });
    }

    // Supports (Design §1.3): the rigid partition, then the nonlinear states.
    let supports = mm_list(model.get("supports"), "model.supports")?;
    let mut restrained = vec![false; ndof];
    let mut nonlinear: Vec<(&str, &str, usize)> = Vec::new();
    for support in supports {
        let id = support.get("id").and_then(Value::as_str).unwrap_or("");
        let family = support.get("family").and_then(Value::as_str);
        let hanger_type = support
            .get("hanger")
            .and_then(|h| h.get("hanger_type"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .or_else(|| family.map(str::trim));
        let node = support.get("node").and_then(Value::as_str).unwrap_or("");
        let node_of = || -> S<usize> {
            node_index.get(node).copied().ok_or_else(|| Stop::Fail(format!("support {id}: its node is not in the model, which the contract refuses")))
        };
        if family == Some("snubber") {
            // The producer blocks every solve of a model with a snubber (`interim_support_refusals`,
            // SNUBBER_DYNAMIC_ONLY_UNSUPPORTED), so a solved document bound to one is a contradiction.
            return mm_fail(format!("support {id} is a snubber, which this source refuses on every solve (SNUBBER_DYNAMIC_ONLY_UNSUPPORTED)"));
        }
        // Any present direction, null included, is admitted or refused (`deserialize_direction`).
        let direction = support.get("direction");
        // The producer's admission of a direction record (`supports_findings`), which it
        // applies before lowering; then an exactly axis-aligned direction is lowered as the
        // producer does (`lower_axis_aligned_directions`): each DOF token on another axis
        // becomes the direction's translation token.
        let mut lowered_axis = None;
        if let Some(direction) = direction {
            let refused = |reason: &str| Stop::Fail(format!("support {id}: its direction {reason}, which the producer refuses"));
            let components: Option<Vec<f64>> = direction
                .as_object()
                .map(|o| ["x", "y", "z"].iter().filter_map(|k| o.get(*k).and_then(Value::as_f64)).filter(|x| x.is_finite()).collect());
            let Some(c) = components.filter(|c| c.len() == 3) else {
                return Err(refused("is not three finite numbers (SUPPORT_DIRECTION_INVALID)"));
            };
            if c.iter().all(|x| *x == 0.0) {
                return Err(refused("is zero (SUPPORT_DIRECTION_ZERO)"));
            }
            if !matches!(family, Some("line_stop" | "spring" | "snubber")) {
                return Err(refused("is on a family other than line_stop, spring or snubber (SUPPORT_DIRECTION_RECORD_UNSUPPORTED)"));
            }
            if support.get("nonlinear").is_some_and(|v| !v.is_null()) {
                return Err(refused("is beside a nonlinear member (SUPPORT_DIRECTION_RECORD_UNSUPPORTED)"));
            }
            let stiffness = support
                .get("stiffness")
                .filter(|s| !s.is_null())
                .or_else(|| support.get("hanger").and_then(|h| h.get("stiffness")).filter(|s| !s.is_null()));
            let mut tokens: Vec<Option<&str>> =
                support.get("restraints").and_then(Value::as_array).into_iter().flatten().map(Value::as_str).collect();
            if let Some(stiffness) = stiffness {
                tokens.push(stiffness.get("dof").and_then(Value::as_str));
            }
            let mut dofs: Vec<usize> = Vec::new();
            for token in tokens {
                let Some(dof) = token.and_then(mm_dof) else {
                    return Err(refused("is on a record naming a token that is not a DOF (SUPPORT_DIRECTION_RECORD_UNSUPPORTED)"));
                };
                if !dofs.contains(&dof) {
                    dofs.push(dof);
                }
            }
            let [dof] = dofs.as_slice() else {
                return Err(refused("is on a record acting on other than exactly one DOF (SUPPORT_DIRECTION_RECORD_UNSUPPORTED)"));
            };
            if *dof >= 3 {
                return Err(refused("is on a rotational DOF (SUPPORT_DIRECTION_ROTATIONAL_UNSUPPORTED)"));
            }
            if c.iter().filter(|x| **x != 0.0).count() > 1 {
                return mm_fail(format!("support {id} is skewed, which modal-1 refuses before projection (MODAL_DOCUMENT_SKEW_RESTRAINT_UNSUPPORTED)"));
            }
            lowered_axis = c.iter().position(|x| *x != 0.0);
        }
        let restraints = mm_list(support.get("restraints"), &format!("support {id} restraints"))?;
        let has_stiffness = support.get("stiffness").is_some_and(|s| !s.is_null())
            || support.get("hanger").and_then(|h| h.get("stiffness")).is_some_and(|s| !s.is_null());
        if let Some(nl) = support.get("nonlinear").filter(|v| !v.is_null()) {
            let behavior = nl.get("behavior").and_then(Value::as_str).unwrap_or("");
            let behavior = match behavior {
                "one_way" | "one-way" | "oneway" => "one_way",
                "lift_off" | "lift-off" | "liftoff" => "lift_off",
                "gap" => "gap",
                "friction" => "friction",
                _ => return mm_fail(format!("nonlinear support {id}: behavior `{behavior}` is refused by the contract")),
            };
            let dof = nl.get("dof").and_then(Value::as_str).and_then(mm_dof);
            let Some(dof) = dof else {
                return mm_fail(format!("nonlinear support {id}: its DOF is refused by the contract"));
            };
            if behavior == "gap" {
                // The static path never confirms a gap state, so modal refuses every model
                // with a gap support (`modal/state.rs`): a solved document bound to one is a
                // contradiction.
                return mm_fail(format!("nonlinear support {id} is a gap support, which modal analysis refuses until static gap qualification lands"));
            }
            nonlinear.push((id, behavior, 6 * node_of()? + dof));
            if restraints.is_empty() && !has_stiffness {
                continue;
            }
        }
        if hanger_type == Some("constant_effort_support") {
            continue; // zero stiffness and zero mass (Design §1.2)
        }
        if family == Some("spring") || matches!(hanger_type, Some("variable_spring_hanger" | "spring_hanger")) {
            continue; // a spring's DOF is free
        }
        let n = node_of()?;
        for (j, token) in restraints.iter().enumerate() {
            let dof = token.as_str().and_then(mm_dof);
            let Some(mut dof) = dof else {
                return mm_fail(format!("support {id}: restraint {j} is not a DOF token, which the contract refuses"));
            };
            if let Some(axis) = lowered_axis {
                if dof % 3 != axis {
                    dof = axis;
                }
            }
            restrained[6 * n + dof] = true;
        }
    }
    let mut conditional = None;
    if !nonlinear.is_empty() {
        conditional = Some(mm_states(p, &nonlinear, &mut restrained)?);
    } else if p.linearized == Some(true) || !p.support_rows.is_empty() {
        return mm_fail("the document reports nonlinear support states, but the bound model has no nonlinear support");
    }

    // Assembly.
    let mut capacity: usize = 0;
    for member in &members {
        let add = match member.kind {
            Kind::Straight => 78,
            Kind::Arc { .. } => 42,
            Kind::Body { .. } => 21,
        };
        capacity = capacity.checked_add(add).ok_or_else(|| Stop::Scope("entry count overflows".to_string()))?;
    }
    let mut acc: HashMap<(usize, usize), Acc> = HashMap::new();
    acc.try_reserve(capacity).map_err(|_| Stop::Scope("mass entries: allocation refused".to_string()))?;
    for (index, member) in members.iter().enumerate() {
        match &member.kind {
            Kind::Body { component, node, mass, cog } => mm_body(&mut acc, *node, *mass, *cog, component)?,
            Kind::Straight => {
                let section = section_mass.get(&index).ok_or_else(|| Stop::Fail("section not formed".to_string()))?;
                mm_straight(&mut acc, member, &positions, section)?;
            }
            Kind::Arc { radius } => {
                let section = section_mass.get(&index).ok_or_else(|| Stop::Fail("section not formed".to_string()))?;
                mm_arc(&mut acc, member, &positions, section, *radius)?;
            }
        }
    }
    let path = mm_dd_path();
    let mut entries = HashMap::new();
    entries.try_reserve(acc.len()).map_err(|_| Stop::Scope("mass entries: allocation refused".to_string()))?;
    let mut order = Vec::new();
    order.try_reserve(acc.len()).map_err(|_| Stop::Scope("mass entries: allocation refused".to_string()))?;
    let mut open = Vec::new();
    for (key, a) in acc {
        // Once 2⁻⁹⁰·(n + 1)·(1 + κ) reaches one binary64 unit roundoff of the entry's scale,
        // the double-word premise no longer sits below binary64 resolution; an
        // intermediate below the low-word floor is outside it. Either leaves this entry,
        // and only this entry, unresolved.
        let reason = if a.floor {
            Some("a double-word intermediate of its element may be below 2⁻⁹⁶⁹, where the low word is subnormal")
        } else if (a.n as f64 + 1.0) * a.cond >= MM_DD_RESOLUTION_LIMIT {
            Some("its frame conditioning puts the double-word premise at binary64 resolution")
        } else {
            None
        };
        if let Some(reason) = reason {
            open.push((key, reason));
            entries.insert(key, Iv::WILD);
            order.push(key);
            continue;
        }
        let allowance = mm_up(path * (a.n as f64 + 1.0) * a.scale);
        entries.insert(key, a.v.widen(if a.v.is_zero() && a.scale == 0.0 { 0.0 } else { allowance }));
        order.push(key);
    }
    order.sort_unstable();
    open.sort_unstable();
    let subjects = members
        .iter()
        .map(|m| match &m.kind {
            Kind::Straight => ("straight_member", m.id.clone()),
            Kind::Arc { .. } => ("arc", m.id.clone()),
            Kind::Body { component, .. } => ("component_body", component.clone()),
        })
        .collect();
    Ok(Formed { ndof, entries, order, restrained, conditional, scope, open, positions, subjects })
}

/// The reported nonlinear states against the bound model (Design §1.3 as
/// built): one row per nonlinear support, its behaviour and DOF the model's,
/// its treatment the rigid-or-free rule of its state; the augmented
/// partition is the rigid one plus every restrained treatment's DOF.
fn mm_states(p: &Parsed, nonlinear: &[(&str, &str, usize)], restrained: &mut [bool]) -> S<String> {
    if p.linearized != Some(true) {
        return mm_fail("the bound model has nonlinear supports, but the document states no linearized state case");
    }
    if p.support_rows.len() != nonlinear.len() {
        return mm_fail(format!("the document reports {} support states for the model's {} nonlinear supports", p.support_rows.len(), nonlinear.len()));
    }
    let base: Vec<bool> = restrained.to_vec();
    for (id, behavior, dof) in nonlinear {
        let rows: Vec<&super::SupportRow> = p.support_rows.iter().filter(|r| r.id == *id).collect();
        let [row] = rows.as_slice() else {
            return mm_fail(format!("nonlinear support {id} is not reported exactly once"));
        };
        if row.behavior != *behavior || row.dof != *dof {
            return mm_fail(format!("nonlinear support {id}: the reported behaviour or DOF is not the model's"));
        }
        let rigid = match (*behavior, row.state.as_str()) {
            ("one_way" | "lift_off" | "gap", "active") => true,
            ("one_way" | "lift_off" | "gap", "inactive") => false,
            ("friction", "sticking") => true,
            ("friction", "sliding" | "inactive") => false,
            (b, s) => return mm_fail(format!("nonlinear support {id}: state `{s}` is not a state of a {b} support")),
        };
        let treatment = match (rigid, base[*dof], *behavior, row.state.as_str()) {
            (true, true, _, _) => "already_restrained",
            (true, false, _, _) => "restrained",
            (false, _, "friction", "sliding") => "free_frozen_friction",
            (false, _, _, _) => "free",
        };
        if row.treatment != treatment {
            return mm_fail(format!("nonlinear support {id}: state `{}` is treated `{}`, not `{treatment}`", row.state, row.treatment));
        }
        if rigid {
            restrained[*dof] = true;
        }
    }
    Ok("the free-DOF partition follows the reported nonlinear states, which no payload qualifies as the converged, confirmed states of the static solve: the free-DOF mass and the participation are conditional on them".to_string())
}

fn mm_add(acc: &mut HashMap<(usize, usize), Acc>, r: usize, c: usize, v: Iv, scale: f64, cond: f64) {
    if r > c {
        return;
    }
    let entry = acc.entry((r, c)).or_insert(Acc { v: Iv::ZERO, scale: 0.0, n: 0, cond: 1.0, floor: false });
    entry.v = entry.v.add(v);
    entry.scale = mm_up(entry.scale + scale);
    entry.n += 1;
    entry.cond = entry.cond.max(cond);
}

/// Whether a nonzero double-word intermediate may lie below the kernel's
/// low-word floor 2⁻⁹⁶⁹ (`LOCAL_ENTRY_MIN`), outside the relative premise.
fn mm_below(x: Iv) -> bool {
    !x.is_zero() && x.mig() < MM_DD_LOW_WORD_FLOOR
}

/// Marks every entry of the node-pair blocks an element formed as outside the
/// rounding model.
fn mm_mark_floor(acc: &mut HashMap<(usize, usize), Acc>, pairs: &[(usize, usize)]) {
    for (key, entry) in acc.iter_mut() {
        let (a, b) = (key.0 / 6, key.1 / 6);
        if pairs.iter().any(|&(p, q)| (a, b) == (p, q) || (a, b) == (q, p)) {
            entry.floor = true;
        }
    }
}

fn mm_skew(e: &[Iv; 3]) -> [[Iv; 3]; 3] {
    let z = Iv::ZERO;
    [[z, e[2].neg(), e[1]], [e[2], z, e[0].neg()], [e[1].neg(), e[0], z]]
}

/// d, L and the unit chord, as enclosures.
fn mm_chord(positions: &[[f64; 3]], from: usize, to: usize, id: &str) -> S<([Iv; 3], Iv, [Iv; 3])> {
    let mut d = [Iv::ZERO; 3];
    for k in 0..3 {
        d[k] = Iv::pt(positions[to][k]).sub(Iv::pt(positions[from][k]));
    }
    let length = d[0].square().add(d[1].square()).add(d[2].square()).sqrt();
    if !(length.lo > 0.0 && length.finite()) {
        return Err(Stop::Open(format!("member {id}: its length is not enclosed away from zero")));
    }
    let e = [d[0].div(length), d[1].div(length), d[2].div(length)];
    Ok((d, length, e))
}

/// The frame conditioning 1 + |y|/|y⊥| and the unit bow direction y⊥/|y⊥|.
fn mm_frame(y: [f64; 3], e: &[Iv; 3], id: &str) -> S<(f64, [Iv; 3])> {
    let yv = [Iv::pt(y[0]), Iv::pt(y[1]), Iv::pt(y[2])];
    let along = yv[0].mul(e[0]).add(yv[1].mul(e[1])).add(yv[2].mul(e[2]));
    let perp = [yv[0].sub(along.mul(e[0])), yv[1].sub(along.mul(e[1])), yv[2].sub(along.mul(e[2]))];
    let perp_len = perp[0].square().add(perp[1].square()).add(perp[2].square()).sqrt();
    let y_len = yv[0].square().add(yv[1].square()).add(yv[2].square()).sqrt();
    if perp_len.hi == 0.0 && y_len.lo > 0.0 {
        return mm_fail(format!("member {id}: its y reference is parallel to its chord, which the contract refuses"));
    }
    if !(perp_len.lo > 0.0 && y_len.finite()) {
        return Err(Stop::Open(format!("member {id}: its y reference is within rounding of its chord")));
    }
    let kappa = 1.0 + y_len.div(perp_len).hi;
    let unit = [perp[0].div(perp_len), perp[1].div(perp_len), perp[2].div(perp_len)];
    Ok((kappa, unit))
}

/// A straight member's consistent mass (module docs; Design §2.2).
fn mm_straight(acc: &mut HashMap<(usize, usize), Acc>, member: &Member, positions: &[[f64; 3]], s: &SectionMass) -> S<()> {
    let (_, l, e) = mm_chord(positions, member.from, member.to, &member.id)?;
    let (kappa, _) = mm_frame(member.y.unwrap_or([0.0, 0.0, 0.0]), &e, &member.id)?;
    let ibar = s.jbar.half();
    let l2 = l.mul(l);
    let translation = s.mbar.mul(l).div(Iv::pt(420.0));
    let rotary = ibar.div(l.mul(Iv::pt(30.0)));
    // The plane matrix on [v_i, θ_i, v_j, θ_j]: m̄L/420·[156, 22L, 54, −13L; …]
    // + ī/(30L)·[36, 3L, −36, 3L; …] (Design §2.2).
    let mass_plane: [[(f64, u8); 4]; 4] = [
        [(156.0, 0), (22.0, 1), (54.0, 0), (-13.0, 1)],
        [(22.0, 1), (4.0, 2), (13.0, 1), (-3.0, 2)],
        [(54.0, 0), (13.0, 1), (156.0, 0), (-22.0, 1)],
        [(-13.0, 1), (-3.0, 2), (-22.0, 1), (4.0, 2)],
    ];
    let rotary_plane: [[(f64, u8); 4]; 4] = [
        [(36.0, 0), (3.0, 1), (-36.0, 0), (3.0, 1)],
        [(3.0, 1), (4.0, 2), (-3.0, 1), (-1.0, 2)],
        [(-36.0, 0), (-3.0, 1), (36.0, 0), (-3.0, 1)],
        [(3.0, 1), (-1.0, 2), (-3.0, 1), (4.0, 2)],
    ];
    let power = |k: u8| match k {
        0 => Iv::pt(1.0),
        1 => l,
        _ => l2,
    };
    let mut h = [[Iv::ZERO; 4]; 4];
    let mut floor = mm_below(l) || mm_below(translation) || mm_below(rotary);
    for r in 0..4 {
        for c in 0..4 {
            let (cm, pm) = mass_plane[r][c];
            let (cr, pr) = rotary_plane[r][c];
            let (bending, turning) = (translation.mul(Iv::pt(cm)).mul(power(pm)), rotary.mul(Iv::pt(cr)).mul(power(pr)));
            floor |= mm_below(bending) || mm_below(turning);
            h[r][c] = bending.add(turning);
        }
    }
    let third = |x: Iv| x.mul(l).div(Iv::pt(3.0));
    let sixth = |x: Iv| x.mul(l).div(Iv::pt(6.0));
    let axial = [[third(s.mbar), sixth(s.mbar)], [sixth(s.mbar), third(s.mbar)]];
    let torsion = [[third(s.jbar), sixth(s.jbar)], [sixth(s.jbar), third(s.jbar)]];
    floor |= axial.iter().chain(torsion.iter()).flatten().any(|&x| mm_below(x));
    let skew = mm_skew(&e);
    let nodes = [member.from, member.to];
    for a in 0..2 {
        for b in 0..2 {
            let (big_b, big_a) = (h[2 * a][2 * b], axial[a][b]);
            let (big_d, big_t) = (h[2 * a + 1][2 * b + 1], torsion[a][b]);
            let coupling = h[2 * a][2 * b + 1];
            let coupling_t = h[2 * b][2 * a + 1];
            let uu_scale = mm_up(kappa * (big_a.mag() + 2.0 * big_b.mag()));
            let tt_scale = mm_up(kappa * (big_t.mag() + 2.0 * big_d.mag()));
            let ut_scale = mm_up(kappa * 2.0 * coupling.mag());
            let tu_scale = mm_up(kappa * 2.0 * coupling_t.mag());
            let (na, nb) = (6 * nodes[a], 6 * nodes[b]);
            for r in 0..3 {
                for c in 0..3 {
                    let eye = if r == c { Iv::pt(1.0) } else { Iv::ZERO };
                    let ee = e[r].mul(e[c]);
                    let uu = big_b.mul(eye).add(big_a.sub(big_b).mul(ee));
                    let tt = big_d.mul(eye).add(big_t.sub(big_d).mul(ee));
                    // [e]×ᵀ(r, c) = [e]×(c, r).
                    let ut = coupling.mul(skew[c][r]);
                    let tu = coupling_t.mul(skew[r][c]);
                    mm_add(acc, na + r, nb + c, uu, uu_scale, kappa);
                    mm_add(acc, na + 3 + r, nb + 3 + c, tt, tt_scale, kappa);
                    mm_add(acc, na + r, nb + 3 + c, ut, ut_scale, kappa);
                    mm_add(acc, na + 3 + r, nb + c, tu, tu_scale, kappa);
                }
            }
        }
    }
    if floor {
        mm_mark_floor(acc, &[(nodes[0], nodes[0]), (nodes[0], nodes[1]), (nodes[1], nodes[1])]);
    }
    Ok(())
}

/// One rigid lump [[m I, −[S]×], [[S]×, J]] about `node`.
fn mm_lump(acc: &mut HashMap<(usize, usize), Acc>, node: usize, m: Iv, first: [Iv; 3], second: [[Iv; 3]; 3], kappa: f64) {
    let n = 6 * node;
    let skew = mm_skew(&first);
    let m_scale = mm_up(kappa * m.mag());
    // The producer turns the local S and J with its frame; their absolute sums bound each
    // turned entry's terms, and the global sums bound them through the norms a rotation
    // keeps: Σ|S_local| ≤ √3‖S‖ ≤ 2·Σ|S|, Σ|J_local| ≤ 3‖J‖_F ≤ 3·Σ|J|.
    let s_scale = mm_up(kappa * 2.0 * (first[0].mag() + first[1].mag() + first[2].mag()));
    let mut j_scale = 0.0f64;
    for row in &second {
        for x in row {
            j_scale = mm_up(j_scale + x.mag());
        }
    }
    let j_scale = mm_up(kappa * 3.0 * j_scale);
    for r in 0..3 {
        mm_add(acc, n + r, n + r, m, m_scale, kappa);
        for c in 0..3 {
            mm_add(acc, n + r, n + 3 + c, skew[r][c].neg(), s_scale, kappa);
            mm_add(acc, n + 3 + r, n + c, skew[r][c], s_scale, kappa);
            mm_add(acc, n + 3 + r, n + 3 + c, second[r][c], j_scale, kappa);
        }
    }
}

fn mm_outer(u: &[Iv; 3], v: &[Iv; 3]) -> [[Iv; 3]; 3] {
    let mut o = [[Iv::ZERO; 3]; 3];
    for r in 0..3 {
        for c in 0..3 {
            o[r][c] = u[r].mul(v[c]);
        }
    }
    o
}

fn mm_cross(u: &[Iv; 3], v: &[Iv; 3]) -> [Iv; 3] {
    [u[1].mul(v[2]).sub(u[2].mul(v[1])), u[2].mul(v[0]).sub(u[0].mul(v[2])), u[0].mul(v[1]).sub(u[1].mul(v[0]))]
}

/// A realized arc's two half-arc lumps (module docs; Design §2.4). With ĉ the
/// unit chord, p̂ the bow side (the y reference normal to ĉ), s = L/(2R),
/// c = √((1 − s)(1 + s)) and α = asin s, each half about its end node has
/// position ρ(φ) = R sin φ t̂ + R(1 − cos φ) n̂, φ ∈ [0, α], with t̂ the
/// tangent into the half and n̂ toward the centre: at the first node
/// t̂ = c ĉ + s p̂, n̂ = s ĉ − c p̂; at the second t̂ = −c ĉ + s p̂,
/// n̂ = −s ĉ − c p̂; b̂ = t̂ × n̂.
fn mm_arc(acc: &mut HashMap<(usize, usize), Acc>, member: &Member, positions: &[[f64; 3]], s: &SectionMass, radius: f64) -> S<()> {
    let (_, l, chord) = mm_chord(positions, member.from, member.to, &member.id)?;
    let (kappa, bow) = mm_frame(member.y.unwrap_or([0.0, 0.0, 0.0]), &chord, &member.id)?;
    let r = Iv::pt(radius);
    let sin = l.div(r.mul(Iv::pt(2.0)));
    if sin.lo >= 1.0 {
        return mm_fail(format!("arc {}: its radius cannot span its chord, which the contract refuses", member.id));
    }
    if !(sin.hi < 1.0 && sin.lo > 0.0) {
        return Err(Stop::Open(format!("arc {}: its chord is within rounding of its diameter", member.id)));
    }
    let one = Iv::pt(1.0);
    let cos = one.sub(sin).mul(one.add(sin)).sqrt();
    let Some(alpha) = mm_asin(sin) else {
        return Err(Stop::Open(format!("arc {}: its half-angle is not enclosed", member.id)));
    };
    let ibar = s.jbar.half();
    let f1 = mm_f1(alpha);
    let a_tt = mm_f1(alpha.mul(Iv::pt(2.0))).mul(Iv::pt(0.25));
    let a_nn = mm_f3(alpha);
    let one_plus_cos = one.add(cos);
    let one_minus_cos = sin.square().div(one_plus_cos);
    let s2 = sin.square();
    let a_tn = s2.square().div(one_plus_cos.square().mul(Iv::pt(2.0)));
    let c_tt = alpha.half().add(sin.mul(cos).half());
    let c_tn = s2.half();
    for x in [f1, a_tt, a_nn, one_minus_cos, a_tn, c_tt] {
        if !x.finite() {
            return Err(Stop::Open(format!("arc {}: a half-arc integral is not enclosed", member.id)));
        }
    }
    let r2 = r.mul(r);
    let r3 = r2.mul(r);
    let mass = s.mbar.mul(r).mul(alpha);
    let first_t = s.mbar.mul(r2).mul(one_minus_cos);
    let first_n = s.mbar.mul(r2).mul(f1);
    let centre = s.mbar.mul(r3);
    let section = r.mul(ibar);
    // In the node's (t̂, n̂, b̂) frame: J = m̄R³·[A_nn, −A_tn, 0; −A_tn, A_tt, 0; 0, 0, 2f₁]
    // + Rī·[α + C_tt, C_tn, 0; C_tn, α + A_tt, 0; 0, 0, α] (j̄ − ī = ī).
    let j_tt = centre.mul(a_nn).add(section.mul(alpha.add(c_tt)));
    let j_nn = centre.mul(a_tt).add(section.mul(alpha.add(a_tt)));
    let j_tn = centre.mul(a_tn).neg().add(section.mul(c_tn));
    let j_bb = centre.mul(f1.mul(Iv::pt(2.0))).add(section.mul(alpha));
    // The producer forms the half-lump's local entries in double word.
    let floor = [mass, first_t, first_n, j_tt, j_nn, j_tn, j_bb].into_iter().any(mm_below);
    for (node, sign) in [(member.from, 1.0), (member.to, -1.0)] {
        let mut t = [Iv::ZERO; 3];
        let mut n = [Iv::ZERO; 3];
        for k in 0..3 {
            t[k] = cos.mul(Iv::pt(sign)).mul(chord[k]).add(sin.mul(bow[k]));
            n[k] = sin.mul(Iv::pt(sign)).mul(chord[k]).sub(cos.mul(bow[k]));
        }
        let b = mm_cross(&t, &n);
        let mut first = [Iv::ZERO; 3];
        for k in 0..3 {
            first[k] = first_t.mul(t[k]).add(first_n.mul(n[k]));
        }
        let (tt, nn, tn, nt, bb) = (mm_outer(&t, &t), mm_outer(&n, &n), mm_outer(&t, &n), mm_outer(&n, &t), mm_outer(&b, &b));
        let mut second = [[Iv::ZERO; 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                second[row][col] =
                    j_tt.mul(tt[row][col]).add(j_nn.mul(nn[row][col])).add(j_tn.mul(tn[row][col].add(nt[row][col]))).add(j_bb.mul(bb[row][col]));
            }
        }
        mm_lump(acc, node, mass, first, second, kappa);
    }
    if floor {
        mm_mark_floor(acc, &[(member.from, member.from), (member.to, member.to)]);
    }
    Ok(())
}

/// A component body m = W/a at offset r (Design §2.3).
fn mm_body(acc: &mut HashMap<(usize, usize), Acc>, node: usize, m: Iv, r: [f64; 3], component: &str) -> S<()> {
    let rv = [Iv::pt(r[0]), Iv::pt(r[1]), Iv::pt(r[2])];
    let first = [m.mul(rv[0]), m.mul(rv[1]), m.mul(rv[2])];
    let mut second = [[Iv::ZERO; 3]; 3];
    for a in 0..3 {
        for b in 0..3 {
            second[a][b] = if a == b {
                let (p, q) = ((a + 1) % 3, (a + 2) % 3);
                first[p].mul(rv[p]).add(first[q].mul(rv[q]))
            } else {
                first[a].mul(rv[b]).neg()
            };
        }
    }
    if !first.iter().all(Iv::finite) || !second.iter().flatten().all(Iv::finite) {
        return Err(Stop::Open(format!("component {component}: its body block is outside the binary64 range")));
    }
    // The producer forms m, m·r_a and (m·r_a)·r_b in double word (`component_mass`); one
    // that may be nonzero below the low-word floor is outside the relative premise.
    let mut products = (0..9).map(|k| first[k / 3].mul(rv[k % 3]));
    let floor = mm_below(m) || first.iter().copied().any(mm_below) || products.any(mm_below);
    mm_lump(acc, node, m, first, second, 1.0);
    if floor {
        mm_mark_floor(acc, &[(node, node)]);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Comparisons.

enum Judged {
    Pass,
    Open,
    Fail,
}

/// A published value against a derived enclosure: a point inside passes, a
/// point outside fails; a published set (flagged, outside the profile)
/// overlapping it, or an enclosure that is not finite, is unresolved.
fn mm_judge(published: &Num, signed: bool, derived: Iv) -> Judged {
    let Some(reach) = mr_reach(published, signed) else {
        return Judged::Fail;
    };
    if let Some(v) = reach.mr_point() {
        if !derived.finite() {
            return Judged::Open;
        }
        return if derived.lo <= v && v <= derived.hi { Judged::Pass } else { Judged::Fail };
    }
    if !derived.finite() {
        return Judged::Open;
    }
    if mr_overlap(&reach, &Reach::mr_new(vec![(derived.lo, derived.hi, false)], false)) {
        Judged::Open
    } else {
        Judged::Fail
    }
}

fn mm_shown(n: &Num) -> String {
    match (n.value, n.below, n.reason.as_deref()) {
        (Some(v), false, _) => format!("{v:e}"),
        (Some(_), true, _) => "a flagged below-normal value".to_string(),
        (None, _, reason) => format!("null ({})", reason.unwrap_or("")),
    }
}

fn mm_label(p: &Parsed, dof: usize) -> String {
    format!("dof:{}:{}", p.node_ids[dof / 6], COMPONENTS[dof % 6])
}

/// Judges one value; pushes an unresolved item or returns the failure.
fn mm_check(items: &mut Vec<String>, published: &Num, signed: bool, derived: Iv, what: &str) -> R<()> {
    match mm_judge(published, signed, derived) {
        Judged::Pass => Ok(()),
        Judged::Open => {
            items.push(format!("{MR_UNRESOLVED}{what}: undecidable against the model-derived [{:e}, {:e}]", derived.lo, derived.hi));
            Ok(())
        }
        Judged::Fail => Err(format!("model-derived: {what} is {}, outside the model-derived [{:e}, {:e}]", mm_shown(published), derived.lo, derived.hi)),
    }
}

impl Formed {
    /// The entries in ascending position.
    fn sorted(&self) -> impl Iterator<Item = (usize, usize, Iv)> + '_ {
        self.order.iter().map(|&(r, c)| (r, c, self.entries.get(&(r, c)).copied().unwrap_or(Iv::ZERO)))
    }
    fn entry(&self, r: usize, c: usize) -> Option<Iv> {
        let key = if r <= c { (r, c) } else { (c, r) };
        self.entries.get(&key).copied()
    }
    /// Per DOF, (M R_k) over every DOF from the derived entries, with the
    /// producer's double-double row sum and its single rounding covered.
    fn rigid_rows(&self, k: usize) -> Vec<Iv> {
        let mut rows = vec![Iv::ZERO; self.ndof];
        let mut scale = vec![0.0f64; self.ndof];
        let mut count = vec![0usize; self.ndof];
        for (r, c, v) in self.sorted() {
            if c % 6 == k {
                rows[r] = rows[r].add(v);
                scale[r] = mm_up(scale[r] + v.mag());
                count[r] += 1;
            }
            if r != c && r % 6 == k {
                rows[c] = rows[c].add(v);
                scale[c] = mm_up(scale[c] + v.mag());
                count[c] += 1;
            }
        }
        let path = mm_dd_path();
        rows.iter().enumerate().map(|(i, x)| x.widen(mm_up(path * (count[i] as f64 + 1.0) * scale[i]))).collect()
    }
    /// T_k (all DOFs) or the free-DOF mass along k: Σ w·M_ij over the pairs
    /// translational along k, as a double-double sum rounded once.
    fn directional(&self, k: usize, free_only: bool) -> Iv {
        let (mut sum, mut scale, mut count) = (Iv::ZERO, 0.0f64, 0usize);
        for (r, c, v) in self.sorted() {
            if r % 6 != k || c % 6 != k || (free_only && (self.restrained[r] || self.restrained[c])) {
                continue;
            }
            let w = if r == c { Iv::pt(1.0) } else { Iv::pt(2.0) };
            let term = w.mul(v);
            sum = sum.add(term);
            scale = mm_up(scale + term.mag());
            count += 1;
        }
        sum.widen(mm_up(mm_dd_path() * (count as f64 + 1.0) * scale))
    }
}

/// The mass relation's model-derived checks (module docs), appended to the
/// payload relation's items.
pub(super) fn mm_mass_relation(p: &Parsed, d: &Derived, mut items: Vec<String>) -> R<Vec<String>> {
    let f = match d {
        Derived::Scoped(s) => {
            items.push(format!("model-derived mass not checked: {s}"));
            return Ok(items);
        }
        Derived::Unresolved(s) => {
            items.push(format!("{MR_UNRESOLVED}model-derived mass: {s}"));
            return Ok(items);
        }
        Derived::Failed(s) => return Err(format!("model-derived: {s}")),
        Derived::Formed(f) => f,
    };
    if f.ndof != p.free.len() {
        return Err("model-derived: the model's DOF count is not the document's".to_string());
    }
    // The partition.
    for dof in 0..f.ndof {
        if f.restrained[dof] != p.free[dof].is_none() {
            let state = |r: bool| if r { "restrained" } else { "free" };
            return Err(format!("model-derived: {} is {} in the document but {} by the bound model's supports{}", mm_label(p, dof),
                state(p.free[dof].is_none()), state(f.restrained[dof]), if f.conditional.is_some() { " and reported states" } else { "" }));
        }
    }
    if let Some(reason) = &f.conditional {
        items.push(reason.clone());
    }
    for ((r, c), reason) in &f.open {
        items.push(format!("{MR_UNRESOLVED}entry mass:{}|{}: {reason}, outside the rounding model; it and what sums it are undecidable",
            mm_label(p, *r), mm_label(p, *c)));
    }
    // Every entry of the full upper M: the union of the derived and the
    // published positions, an absent one being an exact 0.
    match &p.mass {
        None => items.push("mass_matrix withheld: the formed entries are not compared with the model's (the totals, centre and participation are)".to_string()),
        Some(published) => {
            let mut seen = HashSet::new();
            for (r, c, value) in published {
                seen.insert((*r, *c));
                let derived = f.entry(*r, *c).unwrap_or(Iv::ZERO);
                let what = format!("entry mass:{}|{}", mm_label(p, *r), mm_label(p, *c));
                if derived.is_zero() && value.value != Some(0.0) {
                    return Err(format!("model-derived: {what} is {}, but the bound model forms no mass there", mm_shown(value)));
                }
                mm_check(&mut items, value, r != c, derived, &what)?;
            }
            for (r, c, derived) in f.sorted().filter(|(r, c, _)| !seen.contains(&(*r, *c))) {
                if !(derived.lo <= 0.0 && 0.0 <= derived.hi) {
                    return Err(format!("model-derived: entry mass:{}|{} is not published (0), but the bound model forms [{:e}, {:e}] there",
                        mm_label(p, r), mm_label(p, c), derived.lo, derived.hi));
                }
            }
        }
    }
    // T_k, the free-DOF mass and the mass centre.
    let mut totals = [Iv::ZERO; 3];
    for k in 0..3 {
        totals[k] = f.directional(k, false);
        mm_check(&mut items, &p.totals[k], false, totals[k], &format!("T_{}", DIRECTIONS[k]))?;
        mm_check(&mut items, &p.free_totals[k], false, f.directional(k, true), &format!("free-DOF mass {}", DIRECTIONS[k]))?;
    }
    mm_centre(&mut items, p, f, &totals)?;
    items.extend(f.scope.iter().cloned());
    Ok(items)
}

/// The mass centre from M's first moments (Design §2.2's resultant): with
/// F_k = M R_k, the moment about the origin is T_k·(c × e_k), so
/// c_x = −Mom_z,y/T_z, c_y = Mom_z,x/T_z and c_z = Mom_x,y/T_x. The producer
/// forms it about the node centroid p and publishes fl(p + fl(c − p)): the
/// enclosure is grown by u(2|c| + max|x|) for those two roundings and the
/// premise's share for the centroid.
fn mm_centre(items: &mut Vec<String>, p: &Parsed, f: &Formed, totals: &[Iv; 3]) -> R<()> {
    let have = totals.iter().all(|t| t.lo > 0.0);
    let Some(published) = &p.mass_center else {
        if have {
            return Err("model-derived: the bound model has mass along every axis, but no mass centre is published".to_string());
        }
        return Ok(());
    };
    if !have {
        items.push(format!("{MR_UNRESOLVED}mass centre: the model-derived totals are not enclosed away from zero"));
        return Ok(());
    }
    let moment = |k: usize| -> [Iv; 3] {
        let rows = f.rigid_rows(k);
        let mut m = [Iv::ZERO; 3];
        for (node, x) in f.positions.iter().enumerate() {
            let force = [rows[6 * node], rows[6 * node + 1], rows[6 * node + 2]];
            let arm = [Iv::pt(x[0]), Iv::pt(x[1]), Iv::pt(x[2])];
            let cross = mm_cross(&arm, &force);
            for a in 0..3 {
                m[a] = m[a].add(cross[a]).add(rows[6 * node + 3 + a]);
            }
        }
        m
    };
    let (mz, mx) = (moment(2), moment(0));
    let centre = [mz[1].neg().div(totals[2]), mz[0].div(totals[2]), mx[1].div(totals[0])];
    let reach = f.positions.iter().flat_map(|x| x.iter()).fold(0.0f64, |a, x| a.max(x.abs()));
    for k in 0..3 {
        let c = centre[k];
        // Underflow: two products per node and component in the moment, one division by
        // T_k, and the centroid's division by the node count.
        let total = if k < 2 { totals[2].lo } else { totals[0].lo };
        let underflow = mm_up(mm_dd_absolute(2.0 * f.positions.len() as f64 + 1.0, total) + mm_dd_absolute(1.0, 1.0));
        let grow = mm_up(U * (2.0 * c.mag() + reach) * (1.0 + 2f64.powi(-40)) + 2f64.powi(-80) * (c.mag() + reach) + underflow);
        mm_check(items, &published[k], true, c.widen(grow), &format!("the mass centre {}", DIRECTIONS[k]))?;
    }
    Ok(())
}

/// The participation relation's model-derived checks: m_i = φᵀM_ffφ,
/// p_k = (M R_k)_f with its restrained-point coupling (D-v), q = φᵀp_k,
/// Γ = q/m, m_eff = q²/m and the fraction q²/(m·T_k), from the derived M and
/// the published shapes (the producer's operands: m and q are exact sums of
/// binary64 products rounded once).
pub(super) fn mm_participation_relation(p: &Parsed, d: &Derived, mut items: Vec<String>) -> R<Vec<String>> {
    let f = match d {
        Derived::Scoped(s) => {
            items.push(format!("model-derived participation not checked: {s}"));
            return Ok(items);
        }
        Derived::Unresolved(s) => {
            items.push(format!("{MR_UNRESOLVED}model-derived participation: {s}"));
            return Ok(items);
        }
        Derived::Failed(_) => {
            items.push("model-derived participation not checked: the bound model's mass is contradicted (mass)".to_string());
            return Ok(items);
        }
        Derived::Formed(f) => f,
    };
    if f.ndof != p.free.len() || (0..f.ndof).any(|dof| f.restrained[dof] != p.free[dof].is_none()) {
        items.push("model-derived participation not checked: the restraint partition is contradicted (mass)".to_string());
        return Ok(items);
    }
    let Some(shapes) = &p.shapes else {
        items.push("shapes withheld: m_i, Γ, m_eff and the fractions are not re-derived from the model's mass".to_string());
        return Ok(items);
    };
    if let Some(reason) = &f.conditional {
        items.push(reason.clone());
    }
    let rows: Vec<Vec<Iv>> = (0..3).map(|k| f.rigid_rows(k)).collect();
    let totals: Vec<Iv> = (0..3).map(|k| f.directional(k, false)).collect();
    let free: Vec<usize> = (0..f.ndof).filter(|&dof| !f.restrained[dof]).collect();
    for (m, mode) in p.modes.iter().enumerate() {
        let label = m + 1;
        let mut phi = vec![0.0f64; f.ndof];
        for (dof, value) in shapes[m].iter().enumerate() {
            phi[dof] = value.value.unwrap_or(0.0);
        }
        let mut gm = Iv::ZERO;
        for (r, c, v) in f.sorted() {
            if f.restrained[r] || f.restrained[c] || phi[r] == 0.0 || phi[c] == 0.0 {
                continue;
            }
            let w = if r == c { 1.0 } else { 2.0 };
            gm = gm.add(Iv::pt(w).mul(Iv::pt(phi[r])).mul(Iv::pt(phi[c])).mul(v));
        }
        let gm_num = Num::mr_from(&mode.generalized_mass);
        mm_check(&mut items, &gm_num, false, gm, &format!("mode {label}: m"))?;
        for k in 0..3 {
            let d_ = DIRECTIONS[k];
            let mut q = Iv::ZERO;
            for &dof in &free {
                if phi[dof] != 0.0 {
                    q = q.add(Iv::pt(phi[dof]).mul(rows[k][dof]));
                }
            }
            let row = &p.participation[m][k];
            let gamma = q.div(gm);
            let effective = q.square().div(gm);
            mm_check(&mut items, &Num::mr_from(&row.gamma), true, gamma, &format!("mode {label}: Γ along {d_}"))?;
            mm_check(&mut items, &Num::mr_from(&row.effective), false, effective, &format!("mode {label}: m_eff along {d_}"))?;
            let fraction = Num::mr_from(&row.fraction);
            if fraction.reason.as_deref() == Some("total_mass_zero") {
                if totals[k].lo > 0.0 {
                    return Err(format!("model-derived: mode {label}: the fraction along {d_} claims T_k = 0, but the bound model's T_k is positive"));
                }
                continue;
            }
            mm_check(&mut items, &fraction, false, q.square().div(gm.mul(totals[k])), &format!("mode {label}: the fraction along {d_}"))?;
        }
    }
    items.extend(f.scope.iter().cloned());
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intervals_round_outward_and_keep_exact_zeros() {
        let third = Iv::pt(1.0).div(Iv::pt(3.0));
        assert!(third.lo < third.hi && third.lo <= 1.0 / 3.0 && 1.0 / 3.0 <= third.hi);
        assert!(Iv::ZERO.mul(Iv::pt(7.0)).is_zero());
        assert!(Iv::pt(2.0).add(Iv::ZERO) == Iv::pt(2.0));
        let tiny = Iv::pt(1e-200).mul(Iv::pt(1e-200));
        assert!(tiny.lo <= 0.0 && tiny.hi > 0.0, "an underflowing product is never an exact zero");
        let (lo, hi) = mm_growth(2);
        assert!(lo < 1.0 && hi > 1.0 && (1.0 - lo) < 8.0 * U && (hi - 1.0) < 8.0 * U);
        assert_eq!(mm_dd_path(), 2f64.powi(-90));
        let pi = mm_pi();
        assert!(pi.lo == std::f64::consts::PI && pi.hi > pi.lo);
    }

    #[test]
    fn series_enclose_their_closed_forms() {
        for alpha in [1e-3_f64, 0.3, std::f64::consts::FRAC_PI_4, 1.2, 1.5] {
            let a = Iv::pt(alpha);
            // The closed forms lose accuracy to cancellation; they are held
            // only to an absolute 16uα, the series to a tight relative width.
            let f1 = mm_f1(a);
            let closed = alpha - alpha.sin();
            assert!((f1.lo - closed).abs() <= 16.0 * U * alpha && (f1.hi - closed).abs() <= 16.0 * U * alpha, "{alpha}: {f1:?} {closed}");
            assert!(f1.lo > 0.0 && f1.hi - f1.lo <= 128.0 * U * f1.hi, "f1 width at {alpha}");
            let f3 = mm_f3(a);
            let closed3 = 1.5 * alpha - 2.0 * alpha.sin() + (2.0 * alpha).sin() / 4.0;
            assert!((f3.lo - closed3).abs() <= 16.0 * U * alpha && (f3.hi - closed3).abs() <= 16.0 * U * alpha, "{alpha}: {f3:?} {closed3}");
            assert!(f3.lo > 0.0 && f3.hi - f3.lo <= 128.0 * U * f3.hi, "f3 width at {alpha}");
            let s = Iv::pt(alpha.sin());
            let asin = mm_asin(s).expect("asin enclosure");
            assert!(asin.lo <= alpha.sin().asin() && alpha.sin().asin() <= asin.hi);
        }
        // Small angles keep their leading values: f₃ ≈ α⁵/20.
        let small = mm_f3(Iv::pt(1e-4));
        assert!(small.lo > 0.0 && (small.hi / (1e-20 / 20.0) - 1.0).abs() < 1e-7);
    }
}
