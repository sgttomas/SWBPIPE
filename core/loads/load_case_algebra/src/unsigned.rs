//! Unsigned combinations: the absolute sum and the SRSS (DEL-05-02 Work
//! `absolute-sum-srss-combinations.md` §3).
//!
//! - Absolute sum: A = RN(Σ |fᵢ|·|xᵢ|), the exact sum of the exact products,
//!   rounded once (`ExactAccumulator::round`).
//! - SRSS: zᵢ = RN(fᵢ·xᵢ), R = RN(√S) with S = Σ zᵢ² held exactly. A candidate
//!   from √ of S rounded at a scale chosen from the largest |zᵢ| is moved one ulp
//!   at a time until the exact sign of S − m² at each neighbouring midpoint m
//!   says it is nearest, ties to even. With g the gap above r and g′ the gap below
//!   it, 4(S − m²) = 4S − 4r² ∓ 4·r·g − g·g (g′ in the lower case), a sum of
//!   products of finite binary64 values, so the sign is exact in a second
//!   accumulator and no neighbour r⁺ is formed (at r = MAX it would be infinite).
//!
//! Both are +0.0 for an exact zero and depend only on the multiset of terms. A
//! value outside the binary64 range is a `NonRepresentable` finding, never an
//! infinity: for the SRSS, an overflowing zᵢ, or S ≥ (MAX + 2⁹⁷⁰)² (the tie at
//! that midpoint goes to the even 2¹⁰²⁴, which overflows).

use super::{
    collect_statuses, finalize_result, same_dimension_or_record, AlgebraFinding, AlgebraOperand,
    AlgebraOperation, AlgebraQuantity, AlgebraResult, AnalysisStatus, CombinationTerm, FindingCode,
    LoadDimension,
};
use open_pipe_stress_frame_kernel::exact_sum::{ExactAccumulator, SumError};
use std::collections::{HashMap, HashSet};

/// The most terms an unsigned combination accepts. The accumulator's carry bits
/// would hold 2⁶² SRSS terms; the kernel refuses beyond 2³² by name instead of
/// relying on that bound.
pub const UNSIGNED_COMBINATION_MAX_TERMS: u64 = 1 << 32;

pub(crate) fn exceeds_term_limit(count: usize) -> bool {
    count as u64 > UNSIGNED_COMBINATION_MAX_TERMS
}

/// The validated (factor, value) pairs of an unsigned combination, in term order.
struct Gathered {
    pairs: Vec<(f64, f64)>,
    statuses: Vec<AnalysisStatus>,
    source_operand_ids: Vec<String>,
    dimension: Option<LoadDimension>,
    findings: Vec<AlgebraFinding>,
}

/// The term checks of `evaluate_linear_combination` (empty, non-finite factor,
/// duplicate, missing, status, dimension), plus the term limit and a non-finite
/// operand value, which has no representable unsigned result.
fn gather(
    operand_by_id: &HashMap<&str, &AlgebraOperand>,
    terms: &[CombinationTerm],
    subject: &'static str,
    name: &'static str,
) -> Gathered {
    let mut gathered = Gathered {
        pairs: Vec::new(),
        statuses: Vec::new(),
        source_operand_ids: Vec::new(),
        dimension: None,
        findings: Vec::new(),
    };
    let findings = &mut gathered.findings;
    if terms.is_empty() {
        findings.push(AlgebraFinding::new(
            FindingCode::EmptyExpression,
            subject,
            format!("{name} requires at least one term"),
        ));
    }
    if exceeds_term_limit(terms.len()) {
        findings.push(AlgebraFinding::new(
            FindingCode::UnsupportedExpressionShape,
            subject,
            format!("{name} accepts at most 2^32 terms"),
        ));
        return gathered;
    }

    let mut seen = HashSet::new();
    for term in terms {
        if !term.factor.is_finite() {
            findings.push(AlgebraFinding::new(
                FindingCode::NonFiniteFactor,
                &term.operand_id,
                "combination factor must be finite",
            ));
            continue;
        }
        if !seen.insert(term.operand_id.as_str()) {
            findings.push(AlgebraFinding::new(
                FindingCode::DuplicateOperand,
                &term.operand_id,
                format!("{name} contains a duplicate operand"),
            ));
            continue;
        }
        let Some(operand) = operand_by_id.get(term.operand_id.as_str()) else {
            findings.push(AlgebraFinding::new(
                FindingCode::MissingOperand,
                &term.operand_id,
                format!("{name} operand is missing"),
            ));
            continue;
        };
        collect_statuses(operand, &mut gathered.statuses, findings);
        gathered.source_operand_ids.push(operand.operand_id.clone());
        if !same_dimension_or_record(operand, &mut gathered.dimension, findings) {
            continue;
        }
        if !operand.quantity.value.is_finite() {
            findings.push(AlgebraFinding::new(
                FindingCode::NonRepresentable,
                &operand.operand_id,
                format!("{name} operand value is not finite"),
            ));
            continue;
        }
        gathered.pairs.push((term.factor, operand.quantity.value));
    }
    gathered
}

fn evaluate_unsigned(
    operand_by_id: &HashMap<&str, &AlgebraOperand>,
    terms: &[CombinationTerm],
    operation: AlgebraOperation,
    value_of: fn(&[(f64, f64)]) -> Result<f64, SumError>,
    subject: &'static str,
    name: &'static str,
) -> AlgebraResult {
    let mut gathered = gather(operand_by_id, terms, subject, name);
    let mut value = None;
    if gathered.findings.is_empty() {
        match value_of(&gathered.pairs) {
            Ok(v) => value = Some(v),
            Err(error) => gathered.findings.push(AlgebraFinding::new(
                FindingCode::NonRepresentable,
                subject,
                format!("{name} is not representable: {error}"),
            )),
        }
    }
    finalize_result(
        operation,
        gathered
            .dimension
            .zip(value)
            .map(|(dimension, value)| AlgebraQuantity { value, dimension }),
        gathered.statuses,
        gathered.source_operand_ids,
        gathered.findings,
    )
}

/// A = RN(Σ |fᵢ·xᵢ|), one rounding of the exact sum (§3).
pub fn evaluate_absolute_sum(
    operand_by_id: &HashMap<&str, &AlgebraOperand>,
    terms: &[CombinationTerm],
) -> AlgebraResult {
    evaluate_unsigned(
        operand_by_id,
        terms,
        AlgebraOperation::AbsoluteSum,
        absolute_sum_value,
        "absolute_sum",
        "absolute sum",
    )
}

/// R = RN(√Σ RN(fᵢ·xᵢ)²), one rounding of the exact root (§3).
pub fn evaluate_srss(
    operand_by_id: &HashMap<&str, &AlgebraOperand>,
    terms: &[CombinationTerm],
) -> AlgebraResult {
    evaluate_unsigned(
        operand_by_id,
        terms,
        AlgebraOperation::Srss,
        srss_value,
        "srss",
        "srss",
    )
}

/// The exact Σ |f|·|x| rounded once; +0.0 for an exact zero.
pub(crate) fn absolute_sum_value(pairs: &[(f64, f64)]) -> Result<f64, SumError> {
    let mut sum = ExactAccumulator::new();
    for &(factor, value) in pairs {
        sum.add_product(factor.abs(), value.abs())?;
    }
    sum.round()
}

/// RN(√Σ zᵢ²) with zᵢ = RN(fᵢ·xᵢ); +0.0 for an exact zero.
pub(crate) fn srss_value(pairs: &[(f64, f64)]) -> Result<f64, SumError> {
    let mut squares = Vec::with_capacity(pairs.len());
    let mut largest = 0.0_f64;
    for &(factor, value) in pairs {
        let z = factor * value;
        if !z.is_finite() {
            return Err(SumError::NonRepresentable);
        }
        largest = largest.max(z.abs());
        squares.push((z, z));
    }
    if largest == 0.0 {
        return Ok(0.0);
    }
    rounded_root(&squares, exponent(largest))
}

/// RN(√S) for S = Σ aᵢ·bᵢ ≥ 0, exact, with 2²ᵏ ≤ S < 2³⁴·2²ᵏ (the SRSS takes k
/// from its largest |zᵢ|, so the scaled value is finite and at least 1).
pub(crate) fn rounded_root(products: &[(f64, f64)], k: i32) -> Result<f64, SumError> {
    let mut sum = ExactAccumulator::new();
    let mut four = ExactAccumulator::new();
    for &(a, b) in products {
        sum.add_product(a, b)?;
        for _ in 0..4 {
            four.add_product(a, b)?;
        }
    }
    if sum.is_zero() {
        return Ok(0.0);
    }
    let candidate = scale(sum.round_scaled(-2 * k)?.sqrt(), k);
    adjust(&four, candidate)
}

/// Moves `candidate` one ulp at a time to RN(√S), given `four` = 4S exactly.
pub(crate) fn adjust(four: &ExactAccumulator, candidate: f64) -> Result<f64, SumError> {
    // An infinite or over-MAX candidate is clamped to MAX before adjustment.
    let mut r = if candidate.is_finite() {
        candidate.max(0.0)
    } else {
        f64::MAX
    };
    loop {
        let bits = r.to_bits();
        let odd = bits & 1 == 1;
        // A zero (or underflowed) candidate has no lower midpoint; its upper
        // gap is the subnormal quantum 2⁻¹⁰⁷⁴.
        let above = midpoint_sign(four, r, upper_gap(r), Midpoint::Above)?;
        if above > 0 || (above == 0 && odd) {
            if r == f64::MAX {
                return Err(SumError::NonRepresentable);
            }
            r = f64::from_bits(bits + 1);
            continue;
        }
        if r > 0.0 {
            let below = midpoint_sign(four, r, lower_gap(r), Midpoint::Below)?;
            if below < 0 || (below == 0 && odd) {
                r = f64::from_bits(bits - 1);
                continue;
            }
        }
        return Ok(r);
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Midpoint {
    Above,
    Below,
}

/// The exact sign of 4(S − m²) = 4S − 4r² ∓ 4·r·g − g·g, with m = r + g/2
/// (above) or m = r − g/2 (below) and g that side's gap.
pub(crate) fn midpoint_sign(
    four: &ExactAccumulator,
    r: f64,
    gap: f64,
    side: Midpoint,
) -> Result<i8, SumError> {
    let mut difference = four.clone();
    let cross = match side {
        Midpoint::Above => -r,
        Midpoint::Below => r,
    };
    for _ in 0..4 {
        difference.add_product(-r, r)?;
        difference.add_product(cross, gap)?;
    }
    difference.add_product(-gap, gap)?;
    Ok(difference.signum())
}

const MANTISSA: u64 = (1 << 52) - 1;

/// The gap above the non-negative finite double r (MAX's is 2⁹⁷¹).
pub(crate) fn upper_gap(r: f64) -> f64 {
    let biased = (r.to_bits() >> 52) as i32;
    pow2(biased.max(1) - 1075)
}

/// The gap below the positive finite double r: half the upper gap at the bottom
/// of a normal binade above the smallest.
pub(crate) fn lower_gap(r: f64) -> f64 {
    let bits = r.to_bits();
    let biased = (bits >> 52) as i32;
    if bits & MANTISSA == 0 && biased > 1 {
        pow2(biased - 1076)
    } else {
        upper_gap(r)
    }
}

/// 2^k for k in [−1074, 1023], subnormal powers included.
fn pow2(k: i32) -> f64 {
    debug_assert!((-1074..=1023).contains(&k));
    if k >= -1022 {
        f64::from_bits(((k + 1023) as u64) << 52)
    } else {
        f64::from_bits(1u64 << (k + 1074))
    }
}

/// floor(log₂ x) for finite x > 0, subnormals included.
pub(crate) fn exponent(x: f64) -> i32 {
    let bits = x.to_bits();
    let biased = (bits >> 52) as i32;
    if biased == 0 {
        63 - bits.leading_zeros() as i32 - 1074
    } else {
        biased - 1023
    }
}

/// x·2^k, applied in steps that stay in the normal range; only the last step
/// can round (a subnormal result) or overflow (clamped by `adjust`).
fn scale(mut x: f64, mut k: i32) -> f64 {
    while k > 1000 {
        x *= pow2(1000);
        k -= 1000;
    }
    while k < -1000 {
        x *= pow2(-1000);
        k += 1000;
    }
    x * pow2(k)
}
