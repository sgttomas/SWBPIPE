//! Per-case exact load ledger (S11 containment, sections 4.2 and 4.3).
//!
//! Producers push each load contribution, term by term, into one
//! `LoadLedger`. `finish` gives the `AssembledForce`: each DOF's correctly
//! rounded net of its terms, through `exact_sum`. The force vector of a typed
//! solve can be built only here: `AssembledForce` has no public constructor,
//! no `From`, `Default`, `Clone` or `Deserialize`, and no `&mut` access.
//! `ReducedForce` is built only by the kernel's typed reduction functions.
//!
//! S11-G (the formation-noise guard, `S11G_GUARD.md` revision 2.1 section
//! 3.3): a producer may attach a formation record to a formed term
//! (`push_formed`). The records sit in a vector parallel to the terms; they
//! change no value, no term and no `finish` bit, and the hand-written `Debug`
//! of `LoadLedger` and `AssembledForce` prints exactly today's fields.
//! `formation_rows` gives, per loaded row, the exact 12-scaled formation
//! defects in two accumulators (net and self-equilibrated), with no binary64
//! intermediate before the guard's decision.
use crate::exact_sum::{ExactAccumulator, SumError};
use std::fmt;

/// One contribution as the producer formed it (S11 section 4.2 granularity).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ForceTermKind {
    /// A rounded formed term, pushed as its binary64 value.
    Term(f64),
    /// A load-proportional product, pushed exactly as `a * b`.
    Product(f64, f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForceTerm {
    pub source: String,
    pub dof: usize,
    pub kind: ForceTermKind,
}

impl ForceTerm {
    /// Adds this term (or its negation) exactly to an accumulator.
    pub fn accumulate(
        &self,
        accumulator: &mut ExactAccumulator,
        negate: bool,
    ) -> Result<(), SumError> {
        match self.kind {
            ForceTermKind::Term(value) => accumulator.add(if negate { -value } else { value }),
            ForceTermKind::Product(a, b) => accumulator.add_product(if negate { -a } else { a }, b),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LedgerError {
    EmptySource { index: usize },
    DofOutOfRange { dof: usize, size: usize },
    Sum { dof: usize, error: SumError },
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySource { index } => write!(f, "load ledger term {index} has no source"),
            Self::DofOutOfRange { dof, size } => {
                write!(f, "load ledger DOF {dof} is outside a {size}-DOF system")
            }
            Self::Sum { dof, error } => write!(f, "load ledger DOF {dof}: {error}"),
        }
    }
}

impl std::error::Error for LedgerError {}

/// Evidence recorded by `finish`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LedgerEvidence {
    /// DOFs whose nonzero exact net rounded to +0.0 (a load below 2^-1075
    /// in SI units has been lost; range handling stays W2's).
    pub underflowed_dofs: Vec<usize>,
}

/// How a formed term's binary64 value relates to the intended formula of
/// its held operands (S11-G section 3.2). An input term (nodal load,
/// constant effort) carries no record.
#[derive(Debug, Clone, PartialEq)]
pub enum Formation {
    /// `scaled_intended` is a list of binary64 components whose exact sum is
    /// `scale * formula(held operands)`; `scale` is 1 or 3.
    Exact {
        scale: f64,
        scaled_intended: Vec<f64>,
    },
    /// `value = k * fl(a * b)` with `k` exact: the defect is `-k * lo(a * b)`.
    RoundedProduct { k: f64, a: f64, b: f64 },
    /// `|defect| <= bound` (rounded upward).
    Bounded { bound: f64 },
    /// No conservative bound is available: the case demotes.
    CannotBound,
}

/// A formed term's record, parallel to its `ForceTerm`.
#[derive(Debug, Clone, PartialEq)]
struct FormationRecord {
    formation: Formation,
    /// Bound on the formation error of the held operand, absolute.
    operand_bound: f64,
    /// Balances within its own element (straight thrust, thermal and eigen
    /// pairs, curved thermal K*u_free, curved pressure caps).
    self_equilibrated: bool,
}

#[derive(Default)]
pub struct LoadLedger {
    terms: Vec<ForceTerm>,
    formations: Vec<Option<FormationRecord>>,
}

// Exactly the derived output of today's single-field struct (S11-G N-6).
impl fmt::Debug for LoadLedger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LoadLedger")
            .field("terms", &self.terms)
            .finish()
    }
}

impl LoadLedger {
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes one rounded formed term.
    pub fn push(&mut self, source: impl Into<String>, dof: usize, value: f64) {
        self.terms.push(ForceTerm {
            source: source.into(),
            dof,
            kind: ForceTermKind::Term(value),
        });
        self.formations.push(None);
    }

    /// Pushes one load-proportional product, kept exact.
    pub fn push_product(&mut self, source: impl Into<String>, dof: usize, a: f64, b: f64) {
        self.terms.push(ForceTerm {
            source: source.into(),
            dof,
            kind: ForceTermKind::Product(a, b),
        });
        self.formations.push(None);
    }

    /// Pushes one rounded formed term with its formation record (S11-G).
    /// The term is exactly what `push` would push.
    pub fn push_formed(
        &mut self,
        source: impl Into<String>,
        dof: usize,
        value: f64,
        formation: Formation,
        operand_bound: f64,
        self_equilibrated: bool,
    ) {
        self.push(source, dof, value);
        self.attach(formation, operand_bound, self_equilibrated);
    }

    /// Pushes one exact product term with its formation record (S11-G): the
    /// term is exactly what `push_product` would push.
    #[allow(clippy::too_many_arguments)]
    pub fn push_formed_product(
        &mut self,
        source: impl Into<String>,
        dof: usize,
        a: f64,
        b: f64,
        formation: Formation,
        operand_bound: f64,
        self_equilibrated: bool,
    ) {
        self.push_product(source, dof, a, b);
        self.attach(formation, operand_bound, self_equilibrated);
    }

    fn attach(&mut self, formation: Formation, operand_bound: f64, self_equilibrated: bool) {
        if let Some(slot) = self.formations.last_mut() {
            *slot = Some(FormationRecord {
                formation,
                operand_bound,
                self_equilibrated,
            });
        }
    }

    pub fn terms(&self) -> &[ForceTerm] {
        &self.terms
    }

    /// Each DOF's correctly rounded net, rounded once. +0.0 for a zero net.
    pub fn finish(self, size: usize) -> Result<AssembledForce, LedgerError> {
        let mut by_dof = vec![Vec::new(); size];
        for (index, term) in self.terms.iter().enumerate() {
            if term.source.is_empty() {
                return Err(LedgerError::EmptySource { index });
            }
            if term.dof >= size {
                return Err(LedgerError::DofOutOfRange {
                    dof: term.dof,
                    size,
                });
            }
            by_dof[term.dof].push(index);
        }
        let mut values = Vec::with_capacity(size);
        let mut evidence = LedgerEvidence::default();
        for (dof, indices) in by_dof.iter().enumerate() {
            let mut accumulator = ExactAccumulator::new();
            for &index in indices {
                self.terms[index]
                    .accumulate(&mut accumulator, false)
                    .map_err(|error| LedgerError::Sum { dof, error })?;
            }
            let value = accumulator
                .round()
                .map_err(|error| LedgerError::Sum { dof, error })?;
            if value == 0.0 && !accumulator.is_zero() {
                evidence.underflowed_dofs.push(dof);
            }
            values.push(value);
        }
        Ok(AssembledForce {
            values,
            terms: self.terms,
            by_dof,
            evidence,
            formations: self.formations,
        })
    }
}

/// A case force vector built only by `LoadLedger::finish`.
pub struct AssembledForce {
    values: Vec<f64>,
    terms: Vec<ForceTerm>,
    by_dof: Vec<Vec<usize>>,
    evidence: LedgerEvidence,
    formations: Vec<Option<FormationRecord>>,
}

// Exactly the derived output of today's four fields; the S11-G formation
// records are excluded (V1 N-6).
impl fmt::Debug for AssembledForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AssembledForce")
            .field("values", &self.values)
            .field("terms", &self.terms)
            .field("by_dof", &self.by_dof)
            .field("evidence", &self.evidence)
            .finish()
    }
}

impl AssembledForce {
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn get(&self, dof: usize) -> Option<f64> {
        self.values.get(dof).copied()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn terms(&self) -> &[ForceTerm] {
        &self.terms
    }

    /// The terms pushed for one DOF, in push order.
    pub fn terms_for(&self, dof: usize) -> impl Iterator<Item = &ForceTerm> + '_ {
        self.by_dof
            .get(dof)
            .into_iter()
            .flatten()
            .map(move |&index| &self.terms[index])
    }

    pub fn evidence(&self) -> &LedgerEvidence {
        &self.evidence
    }

    /// S11-G section 3.3: one `FormationRow` per loaded row (a DOF with at
    /// least one term), in DOF order. Range failures are carried in the row
    /// (it fires); this never errs.
    pub fn formation_rows(&self) -> Vec<FormationRow> {
        self.by_dof
            .iter()
            .enumerate()
            .filter(|(_, indices)| !indices.is_empty())
            .map(|(dof, indices)| formation_row(self, dof, indices))
            .collect()
    }

    /// True when any term carries a formation record.
    pub fn has_formation_records(&self) -> bool {
        self.formations.iter().any(Option::is_some)
    }

    /// Adds this DOF's terms (or their negation) exactly.
    pub fn accumulate_dof(
        &self,
        dof: usize,
        accumulator: &mut ExactAccumulator,
        negate: bool,
    ) -> Result<(), SumError> {
        for term in self.terms_for(dof) {
            term.accumulate(accumulator, negate)?;
        }
        Ok(())
    }

    /// K2b (T3 D1 revision 5a.2 §4.7, formation-time scaling; ROOT's K2b
    /// ruling 4): this force with every term times 2^b, exactly.
    /// - A term x becomes x·2^b. A product x·y scales one factor, x first,
    ///   else y, or splits b across both, whichever keeps each factor normal.
    ///   A nonzero term that cannot be scaled exactly and normally is
    ///   `ScaledEvaluation`. A zero term, or a product with a zero factor, is
    ///   kept as it is.
    /// - Each DOF's net is the exact sum of its scaled terms, rounded once
    ///   (as `LoadLedger::finish` rounds it), with the same underflow evidence.
    /// - Sources, DOFs and the per-DOF term order are unchanged. Nothing is
    ///   pushed to a ledger. The S11-G formation records are not carried: the
    ///   kernel's solve never reads them (they serve the product's guard,
    ///   which reads the unscaled force).
    pub fn force_scaled(
        &self,
        scale: crate::ForceScale,
    ) -> Result<AssembledForce, crate::structural::ForceScaleReason> {
        let failed = crate::structural::ForceScaleReason::ScaledEvaluation;
        let terms = self
            .terms
            .iter()
            .map(|term| force_scaled_term(term, scale).ok_or(failed.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        let mut values = Vec::with_capacity(self.values.len());
        let mut evidence = LedgerEvidence::default();
        for (dof, indices) in self.by_dof.iter().enumerate() {
            let mut accumulator = ExactAccumulator::new();
            for &index in indices {
                terms[index]
                    .accumulate(&mut accumulator, false)
                    .map_err(|_| failed.clone())?;
            }
            let value = accumulator.round().map_err(|_| failed.clone())?;
            if value == 0.0 && !accumulator.is_zero() {
                evidence.underflowed_dofs.push(dof);
            }
            values.push(value);
        }
        Ok(AssembledForce {
            values,
            by_dof: self.by_dof.clone(),
            formations: vec![None; terms.len()],
            terms,
            evidence,
        })
    }
}

/// K2b: one term times 2^b, exactly (see `AssembledForce::force_scaled`).
fn force_scaled_term(term: &ForceTerm, scale: crate::ForceScale) -> Option<ForceTerm> {
    let b = scale.exponent();
    let kind = match term.kind {
        _ if b == 0 => term.kind,
        ForceTermKind::Term(x) if x == 0.0 => term.kind,
        ForceTermKind::Term(x) => ForceTermKind::Term(crate::exact_normal_scaling(x, b)?),
        ForceTermKind::Product(x, y) if x == 0.0 || y == 0.0 => term.kind,
        ForceTermKind::Product(x, y) => {
            if let Some(scaled) = crate::exact_normal_scaling(x, b) {
                ForceTermKind::Product(scaled, y)
            } else if let Some(scaled) = crate::exact_normal_scaling(y, b) {
                ForceTermKind::Product(x, scaled)
            } else {
                if !x.is_normal() || !y.is_normal() {
                    return None;
                }
                // x·2^s and y·2^(b-s) both normal: e(x) + s and
                // e(y) + b - s in [-1022, 1023].
                let (ex, ey) = (
                    crate::structural::binary_exponent(x),
                    crate::structural::binary_exponent(y),
                );
                let low = (-1022 - ex).max(b - 1023 + ey);
                let high = (1023 - ex).min(b + 1022 + ey);
                if low > high {
                    return None;
                }
                ForceTermKind::Product(
                    crate::exact_normal_scaling(x, low)?,
                    crate::exact_normal_scaling(y, b - low)?,
                )
            }
        }
    };
    Some(ForceTerm {
        source: term.source.clone(),
        dof: term.dof,
        kind,
    })
}

/// Unit roundoff of binary64, 2^-53.
pub const UNIT_ROUNDOFF: f64 = 1.0 / 9_007_199_254_740_992.0;

/// gamma_k = k*u / (1 - k*u), rounded upward (k*u and 1 - k*u are exact for
/// k < 2^20; the one division is followed by one step up).
pub fn gamma(k: u32) -> f64 {
    let ku = f64::from(k) * UNIT_ROUNDOFF;
    (ku / (1.0 - ku)).next_up()
}

/// The exact value of `accumulator`, rounded upward to binary64. A value
/// above the binary64 range is `+inf` for a positive value (a bound that
/// cannot be represented, which fires); a negative out-of-range value is an
/// error.
pub fn round_upward(accumulator: &ExactAccumulator) -> Result<f64, SumError> {
    let nearest = match accumulator.round() {
        Ok(value) => value,
        Err(SumError::NonRepresentable) if accumulator.signum() > 0 => return Ok(f64::INFINITY),
        Err(error) => return Err(error),
    };
    let mut excess = accumulator.clone();
    excess.add(-nearest)?;
    Ok(if excess.signum() > 0 {
        nearest.next_up()
    } else {
        nearest
    })
}

/// A lower bound of the exact |value| of `accumulator`, rounded toward zero.
pub fn magnitude_downward(accumulator: &ExactAccumulator) -> Result<f64, SumError> {
    let nearest = accumulator.round()?;
    let mut excess = accumulator.clone();
    excess.add(-nearest)?;
    // |nearest| > |exact| exactly when nearest overshoots in the value's own
    // direction.
    let overshoots = match accumulator.signum() {
        1 => excess.signum() < 0,
        -1 => excess.signum() > 0,
        _ => false,
    };
    let magnitude = nearest.abs();
    Ok(if overshoots {
        magnitude.next_down().max(0.0)
    } else {
        magnitude
    })
}

/// `a * b` for nonnegative finite operands, rounded upward.
pub fn product_upward(a: f64, b: f64) -> f64 {
    let p = a * b;
    if p.is_finite() && a.mul_add(b, -p) > 0.0 {
        p.next_up()
    } else if p == 0.0 && a != 0.0 && b != 0.0 {
        // Underflow below the smallest subnormal.
        f64::from_bits(1)
    } else {
        p
    }
}

/// `a * b` for nonnegative finite operands, rounded downward.
pub fn product_downward(a: f64, b: f64) -> f64 {
    let p = a * b;
    if p.is_finite() && a.mul_add(b, -p) < 0.0 {
        p.next_down().max(0.0)
    } else {
        p
    }
}

/// `a / b` for nonnegative finite `a` and positive `b`, rounded downward.
pub fn quotient_downward(a: f64, b: f64) -> f64 {
    let q = a / b;
    if q.is_finite() && q.mul_add(b, -a) > 0.0 {
        q.next_down().max(0.0)
    } else {
        q
    }
}

/// S11-G section 3.3: one loaded row's formation evidence. Every decision
/// quantity is exact (`net_defect`, `self_equilibrated_defect` and
/// `twelve_intended_net` are exact accumulators, scaled by 12); the binary64
/// fields are directed roundings for the threshold side and for text.
#[derive(Clone)]
pub struct FormationRow {
    pub dof: usize,
    /// At least one formed term (a formation record) at this row.
    pub formed: bool,
    /// A_net: 12 * the sum of the defects of every formed term that is not
    /// self-equilibrated (never floored).
    pub net_defect: ExactAccumulator,
    /// A_se: 12 * the sum of the defects of the self-equilibrated formed terms.
    pub self_equilibrated_defect: ExactAccumulator,
    /// 12 * the intended net: the sum over terms of 12 * the intended formula
    /// (inputs and bounded terms at their value), exactly.
    pub twelve_intended_net: ExactAccumulator,
    /// B_d: the sum of `Bounded` bounds and operand bounds, rounded upward
    /// (`+inf` when it cannot be represented).
    pub bound: f64,
    /// P_d: the sum of |value| over the self-equilibrated formed terms,
    /// rounded upward.
    pub self_equilibrated_magnitude: f64,
    /// A lower bound of |intended net| (rounded toward zero, then divided by
    /// 12 downward); used on the threshold side.
    pub intended_net_lower: f64,
    /// Sources of `CannotBound` terms at this row, in push order, deduplicated.
    pub cannot_bound_sources: Vec<String>,
    /// Sources of every formed term at this row, in push order, deduplicated.
    pub formed_sources: Vec<String>,
    /// A range failure (a non-finite operand, a scaling that overflows, an
    /// intended net outside the binary64 range): the row fires.
    pub range_failure: Option<&'static str>,
}

// Kept compact: the accumulators are not printed.
impl fmt::Debug for FormationRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormationRow")
            .field("dof", &self.dof)
            .field("formed", &self.formed)
            .field(
                "net_defect_over_12",
                &self.net_defect.round().ok().map(|v| v / 12.0),
            )
            .field(
                "self_equilibrated_defect_over_12",
                &self.self_equilibrated_defect.round().ok().map(|v| v / 12.0),
            )
            .field("bound", &self.bound)
            .field(
                "self_equilibrated_magnitude",
                &self.self_equilibrated_magnitude,
            )
            .field("intended_net_lower", &self.intended_net_lower)
            .field("cannot_bound_sources", &self.cannot_bound_sources)
            .field("range_failure", &self.range_failure)
            .finish()
    }
}

/// Smallest |a*b| (a, b nonzero) whose `fma` error term is exact (V1 DN-2).
const FMA_EXACT_PRODUCT_MIN: f64 = f64::from_bits(54 << 52); // 2^-969

/// Adds 12 * the exact defect of one formed term to `target`, and its bound
/// (if any) to `bounds`. Returns a range failure instead of an error.
/// Also adds 12 * the term's intended value to `intended`: the formula itself
/// where it is exact, otherwise the term's value (whose defect `bounds` or
/// `CannotBound` then covers).
fn add_defect(
    term: &ForceTerm,
    record: &FormationRecord,
    target: &mut ExactAccumulator,
    intended: &mut ExactAccumulator,
    bounds: &mut ExactAccumulator,
    cannot_bound: &mut bool,
) -> Result<(), &'static str> {
    bounds
        .add(record.operand_bound)
        .map_err(|_| "operand bound is not finite")?;
    match &record.formation {
        Formation::Exact {
            scale,
            scaled_intended,
        } => {
            let value = match term.kind {
                ForceTermKind::Term(value) => value,
                ForceTermKind::Product(..) => return Err("exact formation on a product term"),
            };
            let factor = if *scale == 1.0 {
                12.0
            } else if *scale == 3.0 {
                4.0
            } else {
                return Err("exact formation scale is not 1 or 3");
            };
            target
                .add_product(12.0, value)
                .map_err(|_| "12-scaled value is out of range")?;
            for &component in scaled_intended {
                target
                    .add_product(-factor, component)
                    .map_err(|_| "intended component is not finite")?;
                intended
                    .add_product(factor, component)
                    .map_err(|_| "intended component is not finite")?;
            }
        }
        Formation::RoundedProduct { k, a, b } => {
            let four_k = 4.0 * k;
            let p = a * b;
            let lo = a.mul_add(*b, -p);
            let underflow = *a != 0.0 && *b != 0.0 && p.abs() < FMA_EXACT_PRODUCT_MIN;
            add_twelve_value(term, intended).map_err(|_| "12-scaled value is out of range")?;
            if !four_k.is_finite() || !p.is_finite() || !lo.is_finite() || underflow {
                // DN-2 / D21-2: the defect is bounded, never exact.
                let magnitude = match term.kind {
                    ForceTermKind::Term(value) => value.abs(),
                    ForceTermKind::Product(x, y) => product_upward(x.abs(), y.abs()),
                };
                let bound = product_upward(gamma(2), magnitude);
                bounds
                    .add(bound)
                    .map_err(|_| "rounded-product bound is not finite")?;
                if underflow {
                    // |k| * 2^-1074, rounded upward.
                    let absolute = product_upward(k.abs(), f64::from_bits(1));
                    bounds
                        .add(absolute)
                        .map_err(|_| "rounded-product bound is not finite")?;
                }
            } else {
                // defect = -k * lo; 12 * defect = 3 * (-4k * lo), each exact;
                // 12 * formula = 12 * value + 3 * (4k * lo).
                for _ in 0..3 {
                    target
                        .add_product(-four_k, lo)
                        .map_err(|_| "rounded-product defect is out of range")?;
                    intended
                        .add_product(four_k, lo)
                        .map_err(|_| "rounded-product defect is out of range")?;
                }
            }
        }
        Formation::Bounded { bound } => {
            add_twelve_value(term, intended).map_err(|_| "12-scaled value is out of range")?;
            bounds
                .add(*bound)
                .map_err(|_| "formation bound is not finite")?;
        }
        Formation::CannotBound => {
            add_twelve_value(term, intended).map_err(|_| "12-scaled value is out of range")?;
            *cannot_bound = true;
        }
    }
    Ok(())
}

fn add_twelve_value(term: &ForceTerm, target: &mut ExactAccumulator) -> Result<(), SumError> {
    match term.kind {
        ForceTermKind::Term(value) => target.add_product(12.0, value),
        ForceTermKind::Product(a, b) => {
            // 12 * a * b = 3 * (4a * b); 4a is exact unless it overflows.
            let four_a = 4.0 * a;
            for _ in 0..3 {
                target.add_product(four_a, b)?;
            }
            Ok(())
        }
    }
}

fn push_unique(list: &mut Vec<String>, source: &str) {
    if !list.iter().any(|s| s == source) {
        list.push(source.to_string());
    }
}

fn formation_row(force: &AssembledForce, dof: usize, indices: &[usize]) -> FormationRow {
    let mut row = FormationRow {
        dof,
        formed: false,
        net_defect: ExactAccumulator::new(),
        self_equilibrated_defect: ExactAccumulator::new(),
        twelve_intended_net: ExactAccumulator::new(),
        bound: 0.0,
        self_equilibrated_magnitude: 0.0,
        intended_net_lower: 0.0,
        cannot_bound_sources: Vec::new(),
        formed_sources: Vec::new(),
        range_failure: None,
    };
    let mut bounds = ExactAccumulator::new();
    let mut magnitude = ExactAccumulator::new();
    let mut failure = None;
    for &index in indices {
        let term = &force.terms[index];
        let Some(Some(record)) = force.formations.get(index) else {
            // An input term: its intended value is its value.
            if add_twelve_value(term, &mut row.twelve_intended_net).is_err() {
                failure = failure.or(Some("12-scaled value is out of range"));
            }
            continue;
        };
        row.formed = true;
        push_unique(&mut row.formed_sources, &term.source);
        let target = if record.self_equilibrated {
            let added = match term.kind {
                ForceTermKind::Term(value) => magnitude.add(value.abs()),
                ForceTermKind::Product(a, b) => magnitude.add_product(a.abs(), b.abs()),
            };
            if added.is_err() {
                failure = failure.or(Some("self-equilibrated magnitude is out of range"));
            }
            &mut row.self_equilibrated_defect
        } else {
            &mut row.net_defect
        };
        let mut cannot = false;
        if let Err(reason) = add_defect(
            term,
            record,
            target,
            &mut row.twelve_intended_net,
            &mut bounds,
            &mut cannot,
        ) {
            failure = failure.or(Some(reason));
        }
        if cannot {
            push_unique(&mut row.cannot_bound_sources, &term.source);
        }
    }
    row.bound = round_upward(&bounds).unwrap_or(f64::INFINITY);
    row.self_equilibrated_magnitude = round_upward(&magnitude).unwrap_or(f64::INFINITY);
    match magnitude_downward(&row.twelve_intended_net) {
        Ok(twelve) => row.intended_net_lower = quotient_downward(twelve, 12.0),
        Err(_) => failure = failure.or(Some("intended net is outside the binary64 range")),
    }
    if !row.bound.is_finite() {
        failure = failure.or(Some("formation bound is not finite"));
    }
    row.range_failure = failure;
    row
}

/// A reduced right-hand side built only by the kernel's typed reductions.
#[derive(Debug, PartialEq)]
pub struct ReducedForce {
    values: Vec<f64>,
}

impl ReducedForce {
    pub(crate) fn from_exact_rows(values: Vec<f64>) -> Self {
        Self { values }
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finish_rounds_each_dof_once_and_records_underflow() {
        let mut ledger = LoadLedger::new();
        ledger.push("load:a", 0, 1e8);
        ledger.push("load:b", 0, 0.3);
        ledger.push("load:c", 0, -1e8);
        ledger.push_product("thermal:t", 1, 1.35, 4.1e7);
        ledger.push("nodal:n", 1, 1.3);
        ledger.push_product("thermal:t", 1, -1.35, 4.1e7);
        let p = 2.0_f64.powi(-540);
        ledger.push_product("tiny:t", 2, -p, p);
        let force = ledger.finish(4).unwrap();
        // Hand-derived: G terms cancel exactly, so the nets are 0.3 and 1.3.
        assert_eq!(force.values()[0], 0.3);
        assert_eq!(force.values()[1], 1.3);
        assert_eq!(force.values()[2].to_bits(), 0);
        assert_eq!(force.values()[3].to_bits(), 0);
        assert_eq!(force.evidence().underflowed_dofs, vec![2]);
        assert_eq!(force.terms_for(1).count(), 3);
        assert_eq!(force.terms().len(), 7);
        // Precondition: a binary64 fold of DOF 0 in push order is not 0.3.
        assert_ne!(1e8 + 0.3 - 1e8, 0.3);
    }

    // ---------------------------------------------------------- S11-G

    fn negated_matches(accumulator: &ExactAccumulator, expected: &[(f64, f64)]) -> bool {
        // accumulator - sum(expected products) == 0, exactly.
        let mut copy = accumulator.clone();
        for &(a, b) in expected {
            copy.add_product(-a, b).unwrap();
        }
        copy.is_zero()
    }

    /// S11-G section 3.3 (T9's premise): a formation record changes no value,
    /// no term, no `finish` bit and no `Debug` byte of the ledger or force.
    #[test]
    fn s11g_push_formed_changes_no_value_term_or_debug() {
        let build = |formed: bool| {
            let mut ledger = LoadLedger::new();
            ledger.push("nodal:n", 0, 1.3);
            if formed {
                ledger.push_formed(
                    "udl:a",
                    0,
                    0.1 * 3.0,
                    Formation::RoundedProduct {
                        k: 1.0,
                        a: 0.1,
                        b: 3.0,
                    },
                    0.0,
                    false,
                );
                ledger.push_formed_product(
                    "th:c",
                    1,
                    1.35,
                    4.1e7 * 1.1,
                    Formation::RoundedProduct {
                        k: 1.35,
                        a: 4.1e7,
                        b: 1.1,
                    },
                    0.0,
                    true,
                );
                ledger.push_formed("udl:b", 1, 2.5, Formation::CannotBound, 0.0, false);
            } else {
                ledger.push("udl:a", 0, 0.1 * 3.0);
                ledger.push_product("th:c", 1, 1.35, 4.1e7 * 1.1);
                ledger.push("udl:b", 1, 2.5);
            }
            ledger
        };
        let (plain, formed) = (build(false), build(true));
        assert_eq!(format!("{plain:?}"), format!("{formed:?}"));
        assert_eq!(plain.terms(), formed.terms());
        let (plain, formed) = (plain.finish(3).unwrap(), formed.finish(3).unwrap());
        assert_eq!(format!("{plain:?}"), format!("{formed:?}"));
        assert_eq!(
            plain
                .values()
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            formed
                .values()
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        assert!(!plain.has_formation_records());
        assert!(formed.has_formation_records());
        assert!(plain.formation_rows().iter().all(|row| !row.formed));
    }

    /// The rounded-product defect -k lo(a b) enters the net or the
    /// self-equilibrated accumulator exactly, 12-scaled, and the intended net
    /// is the exact formula k a b.
    #[test]
    fn s11g_rounded_product_defect_is_exact() {
        let (a, b) = (0.1_f64, 3.0_f64);
        let value = a * b;
        // Precondition: fl(a*b) is inexact.
        assert_ne!(a.mul_add(b, -value), 0.0);
        for self_equilibrated in [false, true] {
            let mut ledger = LoadLedger::new();
            ledger.push_formed(
                "t",
                0,
                -value,
                Formation::RoundedProduct { k: -1.0, a, b },
                0.0,
                self_equilibrated,
            );
            let row = &ledger.finish(1).unwrap().formation_rows()[0];
            let (target, other) = if self_equilibrated {
                (&row.self_equilibrated_defect, &row.net_defect)
            } else {
                (&row.net_defect, &row.self_equilibrated_defect)
            };
            // 12 * (-fl(ab) - (-ab)) = -12 fl(ab) + 12 ab.
            // (12 a b as three exact products 4a * b.)
            assert!(negated_matches(
                target,
                &[(-12.0, value), (4.0 * a, b), (4.0 * a, b), (4.0 * a, b)]
            ));
            assert!(other.is_zero());
            // 12 * intended = -12 a b.
            assert!(negated_matches(
                &row.twelve_intended_net,
                &[(-4.0 * a, b), (-4.0 * a, b), (-4.0 * a, b)]
            ));
            assert_eq!(row.bound, 0.0);
            assert_eq!(
                row.self_equilibrated_magnitude,
                if self_equilibrated { value.abs() } else { 0.0 }
            );
        }
    }

    /// Exact formation: 12 value - (12/scale) sum(scaled_intended), scale 3.
    #[test]
    fn s11g_exact_formation_defect_with_scale_three() {
        let value = 1.0 / 3.0;
        let mut ledger = LoadLedger::new();
        ledger.push_formed(
            "r",
            0,
            value,
            Formation::Exact {
                scale: 3.0,
                scaled_intended: vec![1.0],
            },
            0.0,
            false,
        );
        ledger.push("n", 0, 0.25);
        let row = &ledger.finish(1).unwrap().formation_rows()[0];
        assert!(row.formed);
        assert!(negated_matches(
            &row.net_defect,
            &[(12.0, value), (-4.0, 1.0)]
        ));
        assert!(!row.net_defect.is_zero());
        // 12 * intended net = 12 * (1/3 + 0.25) = 4 + 3 = 7, exactly.
        assert!(negated_matches(&row.twelve_intended_net, &[(7.0, 1.0)]));
        assert_eq!(row.intended_net_lower, quotient_downward(7.0, 12.0));
        assert!(row.intended_net_lower <= 7.0 / 12.0);
    }

    /// N-5 and DN-2 (T14, ledger part): range failures are carried in the row
    /// and fire; nothing errs.
    #[test]
    fn s11g_range_failures_fall_back_or_fire_and_never_err() {
        // DN-2: |a*b| < 2^-969 makes lo inexact: Bounded gamma_2 |value| + |k| 2^-1074.
        let (a, b, k) = (1e-150_f64, 1e-150_f64, 3.0_f64);
        assert!((a * b).abs() < FMA_EXACT_PRODUCT_MIN);
        let mut ledger = LoadLedger::new();
        ledger.push_formed_product(
            "tiny",
            0,
            k,
            a * b,
            Formation::RoundedProduct { k, a, b },
            0.0,
            true,
        );
        let row = &ledger.finish(1).unwrap().formation_rows()[0];
        assert!(
            row.self_equilibrated_defect.is_zero(),
            "no exact defect below 2^-969"
        );
        let expected = product_upward(gamma(2), product_upward(k, a * b));
        // gamma_2 |value| plus the absolute |k| 2^-1074 (D21-2).
        assert!(row.bound > expected, "{} vs {expected}", row.bound);
        assert!(row.range_failure.is_none());
        // 4k overflows: Bounded, never exact.
        let mut ledger = LoadLedger::new();
        ledger.push_formed(
            "k",
            0,
            1.0,
            Formation::RoundedProduct {
                k: 1e308,
                a: 1e-308,
                b: 1.0,
            },
            0.0,
            false,
        );
        let row = &ledger.finish(1).unwrap().formation_rows()[0];
        assert!(row.net_defect.is_zero());
        assert!(row.bound >= gamma(2));
        // A value above ~1.5e307: 12 * intended is outside binary64 (DN-2), the row fires.
        let mut ledger = LoadLedger::new();
        ledger.push_formed(
            "big",
            0,
            1.6e307,
            Formation::Exact {
                scale: 1.0,
                scaled_intended: vec![1.6e307],
            },
            0.0,
            false,
        );
        let row = &ledger.finish(1).unwrap().formation_rows()[0];
        assert!(row.range_failure.is_some(), "{row:?}");
        // A non-finite bound fires.
        let mut ledger = LoadLedger::new();
        ledger.push_formed(
            "inf",
            0,
            1.0,
            Formation::Bounded {
                bound: f64::INFINITY,
            },
            0.0,
            false,
        );
        let row = &ledger.finish(1).unwrap().formation_rows()[0];
        assert!(row.range_failure.is_some());
        // CannotBound is named.
        let mut ledger = LoadLedger::new();
        ledger.push_formed("curved:w", 0, 1.0, Formation::CannotBound, 0.0, false);
        let row = &ledger.finish(1).unwrap().formation_rows()[0];
        assert_eq!(row.cannot_bound_sources, vec!["curved:w".to_string()]);
    }

    /// The directed roundings used on the threshold side.
    #[test]
    fn s11g_directed_roundings() {
        for k in [2_u32, 4, 16, 20] {
            let g = gamma(k);
            // g * (1 - k u) >= k u, exactly.
            let ku = f64::from(k) * UNIT_ROUNDOFF;
            let mut check = ExactAccumulator::new();
            check.add_product(g, 1.0 - ku).unwrap();
            check.add(-ku).unwrap();
            assert!(check.signum() >= 0, "gamma({k})");
            assert!(g < 1.01 * ku / (1.0 - ku));
        }
        let mut third = ExactAccumulator::new();
        third.add_product(1.0 / 3.0, 3.0).unwrap(); // just below 1
        assert_eq!(round_upward(&third).unwrap(), 1.0);
        assert!(magnitude_downward(&third).unwrap() < 1.0);
        let mut negative = ExactAccumulator::new();
        negative.add_product(-(1.0 / 3.0), 3.0).unwrap();
        assert!(magnitude_downward(&negative).unwrap() < 1.0);
        assert_eq!(product_downward(0.1, 3.0), (0.1_f64 * 3.0).next_down());
        assert_eq!(product_upward(0.1, 3.0), 0.1_f64 * 3.0);
        assert!(quotient_downward(1.0, 3.0) * 3.0 <= 1.0);
    }

    #[test]
    fn finish_rejects_bad_terms() {
        let mut ledger = LoadLedger::new();
        ledger.push("load:a", 5, 1.0);
        assert_eq!(
            ledger.finish(3).unwrap_err(),
            LedgerError::DofOutOfRange { dof: 5, size: 3 }
        );
        let mut ledger = LoadLedger::new();
        ledger.push("", 0, 1.0);
        assert_eq!(
            ledger.finish(1).unwrap_err(),
            LedgerError::EmptySource { index: 0 }
        );
        let mut ledger = LoadLedger::new();
        ledger.push("load:a", 0, f64::NAN);
        assert_eq!(
            ledger.finish(1).unwrap_err(),
            LedgerError::Sum {
                dof: 0,
                error: SumError::NonFinite
            }
        );
    }
}
