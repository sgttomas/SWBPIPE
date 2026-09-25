//! Bounded global enclosure of |a| + sqrt(b²+c²) for supplied quadratic stress
//! fields. The degree-two Bernstein coefficients are interpreted as exact f64
//! values; statics/solution/coefficient-derivation error is the caller's concern.
//! Ordinary IEEE-754 operations with gradual underflow and correctly rounded
//! sqrt are required; no fast-math/reassociation or flush-to-zero is permitted.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// One closed statics interval, independently supplied on both sides of jumps.
/// Arrays contain degree-two Bernstein controls in local u∈[0,1], in Pa.
/// The caller derives these from qualified polynomial statics, not opaque samples.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuadraticStressSpan {
    pub start: f64,
    pub end: f64,
    pub axial: [f64; 3],
    pub bending_y: [f64; 3],
    pub bending_z: [f64; 3],
}

/// Approximate maximizing witness with a certified global value enclosure.
/// The true maximum is in [value_lower, upper_bound]; the value at the witness
/// is in [value_lower, value_upper]. No exact/unique argmax claim is made.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CertifiedStressMaximum {
    pub span_index: usize,
    /// Exact dyadic local parameter; authoritative even when station rounds to an
    /// endpoint on a very short interval. Retain together with the span identity.
    pub local_fraction: f64,
    /// Rounded physical coordinate for presentation; local_fraction defines the witness.
    pub station: f64,
    pub value_lower: f64,
    pub value_upper: f64,
    pub upper_bound: f64,
    pub certified_gap: f64,
    pub subdivisions: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnresolvedReason {
    SubdivisionLimit,
    DepthLimit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExtremaError {
    EmptyInput,
    InvalidSpan {
        span_index: usize,
    },
    NonFiniteCoefficient {
        span_index: usize,
    },
    UnrepresentableBound {
        span_index: usize,
    },
    /// This is not a qualified maximum. Retain the best witness and open bounds
    /// as diagnostics; do not publish its lower bound as the complete maximum.
    Unresolved {
        best: CertifiedStressMaximum,
        reason: UnresolvedReason,
    },
}

const RELATIVE_GAP: f64 = 1e-12;
const ABSOLUTE_GAP_PA: f64 = 1e-12;
const MAX_SUBDIVISIONS: usize = 131_072;
const MAX_DEPTH: u32 = 48;

/// Bound the maximum over the union of all supplied closed spans. Gaps, overlaps,
/// discontinuities, and complete physical load-domain coverage remain caller-owned.
/// Stops only with gap ≤ 1e-12 Pa + 1e-12*value_lower, or returns an explicit error.
/// Storage is O(spans + subdivisions); bounded work is O((spans+subdivisions)
/// log(spans+subdivisions)). No roots or maxima can be lost to a sampling grid.
pub fn bound_piecewise_elastic_maximum(
    spans: &[QuadraticStressSpan],
) -> Result<CertifiedStressMaximum, ExtremaError> {
    bound_with_limits(spans, MAX_SUBDIVISIONS, MAX_DEPTH)
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    lo: f64,
    hi: f64,
}
impl Interval {
    fn exact(x: f64) -> Self {
        Self { lo: x, hi: x }
    }
    fn average(self, rhs: Self) -> Self {
        Self {
            lo: average_down(self.lo, rhs.lo),
            hi: average_up(self.hi, rhs.hi),
        }
    }
    fn absolute(self) -> Self {
        Self {
            lo: if self.lo <= 0.0 && self.hi >= 0.0 {
                0.0
            } else {
                self.lo.abs().min(self.hi.abs())
            },
            hi: self.lo.abs().max(self.hi.abs()),
        }
    }
}
type VectorBounds = [Interval; 3];
type Controls = [VectorBounds; 3];

#[derive(Clone, Debug)]
struct Node {
    controls: Controls,
    span: usize,
    lo: f64,
    hi: f64,
    depth: u32,
    upper: f64,
}
impl PartialEq for Node {
    fn eq(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Equal
    }
}
impl Eq for Node {}
impl PartialOrd for Node {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Ord for Node {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.upper
            .total_cmp(&rhs.upper)
            .then_with(|| rhs.span.cmp(&self.span))
            .then_with(|| rhs.lo.total_cmp(&self.lo))
            .then_with(|| rhs.depth.cmp(&self.depth))
    }
}

fn bound_with_limits(
    spans: &[QuadraticStressSpan],
    split_limit: usize,
    depth_limit: u32,
) -> Result<CertifiedStressMaximum, ExtremaError> {
    if spans.is_empty() {
        return Err(ExtremaError::EmptyInput);
    }
    let mut heap = BinaryHeap::new();
    let mut best: Option<CertifiedStressMaximum> = None;
    for (i, span) in spans.iter().enumerate() {
        if !span.start.is_finite() || !span.end.is_finite() || span.start >= span.end {
            return Err(ExtremaError::InvalidSpan { span_index: i });
        }
        if !span
            .axial
            .iter()
            .chain(&span.bending_y)
            .chain(&span.bending_z)
            .all(|v| v.is_finite())
        {
            return Err(ExtremaError::NonFiniteCoefficient { span_index: i });
        }
        let controls: Controls = std::array::from_fn(|j| {
            [
                Interval::exact(span.axial[j]),
                Interval::exact(span.bending_y[j]),
                Interval::exact(span.bending_z[j]),
            ]
        });
        consider(&mut best, spans, i, 0.0, controls[0])?;
        consider(&mut best, spans, i, 1.0, controls[2])?;
        heap.push(Node {
            upper: control_upper(controls, i)?,
            controls,
            span: i,
            lo: 0.0,
            hi: 1.0,
            depth: 0,
        });
    }
    let mut subdivisions = 0;
    loop {
        let best_value = best.as_ref().unwrap().value_lower;
        while heap.peek().is_some_and(|n| n.upper <= best_value) {
            heap.pop();
        }
        let upper = heap.peek().map_or(best_value, |n| n.upper.max(best_value));
        let mut result = best.unwrap();
        result.upper_bound = upper;
        result.value_upper = result.value_upper.min(upper);
        result.certified_gap = if upper == best_value {
            0.0
        } else {
            (upper - best_value).next_up()
        };
        result.subdivisions = subdivisions;
        let tolerance = (ABSOLUTE_GAP_PA + (RELATIVE_GAP * best_value).next_down().max(0.0))
            .next_down()
            .max(0.0);
        if result.certified_gap <= tolerance {
            return Ok(result);
        }
        if subdivisions >= split_limit {
            return Err(ExtremaError::Unresolved {
                best: result,
                reason: UnresolvedReason::SubdivisionLimit,
            });
        }
        let node = heap.pop().unwrap();
        if node.depth >= depth_limit {
            return Err(ExtremaError::Unresolved {
                best: result,
                reason: UnresolvedReason::DepthLimit,
            });
        }
        let midpoint = (node.lo + node.hi) * 0.5;
        let p01 = average_vector(node.controls[0], node.controls[1]);
        let p12 = average_vector(node.controls[1], node.controls[2]);
        let middle = average_vector(p01, p12);
        consider(&mut best, spans, node.span, midpoint, middle)?;
        for (controls, lo, hi) in [
            ([node.controls[0], p01, middle], node.lo, midpoint),
            ([middle, p12, node.controls[2]], midpoint, node.hi),
        ] {
            // A child is contained in its parent; intersect two independently
            // valid bounds to avoid outward-rounding drift growing the upper bound.
            let upper = control_upper(controls, node.span)?.min(node.upper);
            if upper > best.as_ref().unwrap().value_lower {
                heap.push(Node {
                    controls,
                    span: node.span,
                    lo,
                    hi,
                    depth: node.depth + 1,
                    upper,
                });
            }
        }
        subdivisions += 1;
    }
}

fn consider(
    best: &mut Option<CertifiedStressMaximum>,
    spans: &[QuadraticStressSpan],
    span_index: usize,
    fraction: f64,
    point: VectorBounds,
) -> Result<(), ExtremaError> {
    let value = objective(point, span_index)?;
    if best.as_ref().is_none_or(|b| {
        value.lo > b.value_lower
            || (value.lo == b.value_lower
                && (span_index < b.span_index
                    || (span_index == b.span_index && fraction < b.local_fraction)))
    }) {
        let s = spans[span_index];
        let station = if fraction == 0.0 {
            s.start
        } else if fraction == 1.0 {
            s.end
        } else {
            (s.start * (1.0 - fraction) + s.end * fraction).clamp(s.start, s.end)
        };
        if !station.is_finite() {
            return Err(ExtremaError::UnrepresentableBound { span_index });
        }
        *best = Some(CertifiedStressMaximum {
            span_index,
            local_fraction: fraction,
            station,
            value_lower: value.lo,
            value_upper: value.hi,
            upper_bound: value.hi,
            certified_gap: value.hi - value.lo,
            subdivisions: 0,
        });
    }
    Ok(())
}

fn control_upper(controls: Controls, span: usize) -> Result<f64, ExtremaError> {
    let mut upper: f64 = 0.0;
    for point in controls {
        upper = upper.max(objective(point, span)?.hi);
    }
    Ok(upper)
}

fn objective(point: VectorBounds, span_index: usize) -> Result<Interval, ExtremaError> {
    if point
        .iter()
        .any(|x| !x.lo.is_finite() || !x.hi.is_finite() || x.lo > x.hi)
    {
        return Err(ExtremaError::UnrepresentableBound { span_index });
    }
    let [a, b, c] = point.map(Interval::absolute);
    let norm_lo = norm_bound(b.lo, c.lo, false);
    let norm_hi = norm_bound(b.hi, c.hi, true);
    let lo = nonnegative_add(a.lo, norm_lo, false);
    let hi = nonnegative_add(a.hi, norm_hi, true);
    if !hi.is_finite() || !lo.is_finite() || lo > hi {
        return Err(ExtremaError::UnrepresentableBound { span_index });
    }
    Ok(Interval { lo, hi })
}

// Monotone scalar norm on exact nonnegative component bounds. All arithmetic
// rounds outward, including sqrt; hypot has no specified rounding guarantee.
fn norm_bound(x: f64, y: f64, upper: bool) -> f64 {
    let max = x.max(y);
    let min = x.min(y);
    if min == 0.0 {
        return max;
    }
    let ratio = directed(min / max, upper);
    let square = directed(ratio * ratio, upper);
    let sum = directed(1.0 + square, upper);
    let root = directed(sum.sqrt(), upper);
    directed(max * root, upper)
}
fn directed(x: f64, upper: bool) -> f64 {
    if upper {
        x.next_up()
    } else {
        x.next_down().max(0.0)
    }
}
fn nonnegative_add(x: f64, y: f64, upper: bool) -> f64 {
    if x == 0.0 {
        y
    } else if y == 0.0 {
        x
    } else {
        directed(x + y, upper)
    }
}
fn average_down(x: f64, y: f64) -> f64 {
    if x == y {
        x
    } else {
        ((x * 0.5).next_down() + (y * 0.5).next_down()).next_down()
    }
}
fn average_up(x: f64, y: f64) -> f64 {
    if x == y {
        x
    } else {
        ((x * 0.5).next_up() + (y * 0.5).next_up()).next_up()
    }
}
fn average_vector(x: VectorBounds, y: VectorBounds) -> VectorBounds {
    std::array::from_fn(|i| x[i].average(y[i]))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn span(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> QuadraticStressSpan {
        QuadraticStressSpan {
            start: 0.0,
            end: 1.0,
            axial: a,
            bending_y: b,
            bending_z: c,
        }
    }
    fn x1() -> QuadraticStressSpan {
        span([0.0; 3], [0.0, 2e6, 0.0], [0.0, 0.5e6, 1e6])
    }
    fn encloses(result: CertifiedStressMaximum, exact: f64) {
        assert!(
            result.value_lower <= exact && result.upper_bound >= exact,
            "expected {exact:.17e} within [{:.17e},{:.17e}]",
            result.value_lower,
            result.upper_bound
        );
        assert!(result.certified_gap <= 1e-12 + 1e-12 * result.value_lower);
        assert!(result.value_upper >= result.value_lower);
    }
    #[test]
    fn x1_certifies_the_peak_missed_by_eight_sign_candidates() {
        let r = bound_piecewise_elastic_maximum(&[x1()]).unwrap();
        let exact = 1e6 * (71.0 + 8.0 * 2_f64.sqrt()).sqrt() / 8.0;
        encloses(r, exact);
        let t = (6.0 - 2_f64.sqrt()) / 8.0;
        assert!((r.local_fraction - t).abs() < 2e-6);
        assert!(r.value_lower > 1e6 * 325_f64.sqrt() / 16.0);
        assert!(r.subdivisions < 1000);
    }
    #[test]
    fn constant_zero_and_constant_biaxial_fields_are_enclosed() {
        let z = bound_piecewise_elastic_maximum(&[span([0.0; 3], [0.0; 3], [0.0; 3])]).unwrap();
        encloses(z, 0.0);
        assert_eq!(z.subdivisions, 0);
        let r = bound_piecewise_elastic_maximum(&[span([-2e6; 3], [3e6; 3], [4e6; 3])]).unwrap();
        encloses(r, 7e6);
        assert_eq!(r.subdivisions, 0);
    }
    #[test]
    fn axial_zero_crossing_and_simultaneous_bending_zero_need_no_root_special_case() {
        encloses(
            bound_piecewise_elastic_maximum(&[span(
                [-2.0, 0.0, 2.0],
                [-3.0, 0.0, 3.0],
                [-4.0, 0.0, 4.0],
            )])
            .unwrap(),
            7.0,
        );
        encloses(
            bound_piecewise_elastic_maximum(&[span([0.0; 3], [1.0, -1.0, 1.0], [0.0; 3])]).unwrap(),
            1.0,
        );
    }
    #[test]
    fn repeated_stationary_root_with_zero_bending_vector_is_safe() {
        // b=(2u-1)^2,c=0; D=b² has a derivative root of multiplicity three at u=.5.
        let r =
            bound_piecewise_elastic_maximum(&[span([0.0; 3], [1.0, -1.0, 1.0], [0.0; 3])]).unwrap();
        encloses(r, 1.0);
        assert_eq!(r.local_fraction, 0.0);
    }
    #[test]
    fn discontinuity_retains_both_one_sided_values_and_governing_span() {
        let first = QuadraticStressSpan {
            end: 2.0,
            ..span([1.0; 3], [0.0; 3], [0.0; 3])
        };
        let second = QuadraticStressSpan {
            start: 2.0,
            end: 3.0,
            ..span([9.0, 5.0, 1.0], [0.0; 3], [0.0; 3])
        };
        let r = bound_piecewise_elastic_maximum(&[first, second]).unwrap();
        encloses(r, 9.0);
        assert_eq!(r.span_index, 1);
        assert_eq!(r.local_fraction, 0.0);
        assert_eq!(r.station, 2.0);
    }
    #[test]
    fn short_interval_preserves_the_local_witness_when_station_rounds() {
        let s = QuadraticStressSpan {
            start: 1.0,
            end: 1.0_f64.next_up(),
            ..x1()
        };
        let r = bound_piecewise_elastic_maximum(&[s]).unwrap();
        encloses(r, 1e6 * (71.0 + 8.0 * 2_f64.sqrt()).sqrt() / 8.0);
        assert!(r.local_fraction > 0.0 && r.local_fraction < 1.0);
        assert!(r.station >= s.start && r.station <= s.end);
    }
    #[test]
    fn section_axis_quarter_rotation_and_action_reversal_preserve_bound() {
        let s = x1();
        let r = bound_piecewise_elastic_maximum(&[QuadraticStressSpan {
            bending_y: s.bending_z,
            bending_z: s.bending_y.map(|v| -v),
            ..s
        }])
        .unwrap();
        encloses(r, 1e6 * (71.0 + 8.0 * 2_f64.sqrt()).sqrt() / 8.0);
        let r = bound_piecewise_elastic_maximum(&[span([-2.0; 3], [-3.0; 3], [-4.0; 3])]).unwrap();
        encloses(r, 7.0);
    }
    #[test]
    fn no_budget_or_depth_is_an_explicit_unresolved_result() {
        for (splits, depth, reason) in [
            (0, 48, UnresolvedReason::SubdivisionLimit),
            (100, 0, UnresolvedReason::DepthLimit),
        ] {
            let ExtremaError::Unresolved {
                best,
                reason: actual,
            } = bound_with_limits(&[x1()], splits, depth).unwrap_err()
            else {
                panic!("wrong error")
            };
            assert_eq!(actual, reason);
            assert!(best.upper_bound > best.value_lower);
            assert!(best.upper_bound >= 1.1340862821217075e6);
        }
    }
    #[test]
    fn invalid_or_unrepresentable_inputs_never_become_zero_maxima() {
        assert_eq!(
            bound_piecewise_elastic_maximum(&[]),
            Err(ExtremaError::EmptyInput)
        );
        assert!(matches!(
            bound_piecewise_elastic_maximum(&[QuadraticStressSpan { start: 1.0, ..x1() }]),
            Err(ExtremaError::InvalidSpan { .. })
        ));
        assert!(matches!(
            bound_piecewise_elastic_maximum(&[span([f64::NAN; 3], [0.0; 3], [0.0; 3])]),
            Err(ExtremaError::NonFiniteCoefficient { .. })
        ));
        assert!(matches!(
            bound_piecewise_elastic_maximum(&[span([f64::MAX; 3], [f64::MAX; 3], [0.0; 3])]),
            Err(ExtremaError::UnrepresentableBound { .. })
        ));
    }
    #[test]
    fn scaled_norm_does_not_square_large_components_and_bounds_subnormals() {
        let r = bound_piecewise_elastic_maximum(&[span([0.0; 3], [3e200; 3], [4e200; 3])]).unwrap();
        encloses(r, 5e200);
        let tiny = f64::from_bits(1);
        let r = bound_piecewise_elastic_maximum(&[span([0.0; 3], [tiny; 3], [0.0; 3])]).unwrap();
        encloses(r, tiny);
    }
    #[test]
    fn exact_interior_quadratic_peak_is_located_and_bounded() {
        let r =
            bound_piecewise_elastic_maximum(&[span([0.0; 3], [0.0, 2.0, 0.0], [0.0; 3])]).unwrap();
        encloses(r, 1.0);
        assert_eq!(r.local_fraction, 0.5);
    }
    #[test]
    fn directed_averages_enclose_extreme_and_opposite_values() {
        assert!(average_down(f64::MAX, -f64::MAX) <= 0.0);
        assert!(average_up(f64::MAX, -f64::MAX) >= 0.0);
        let tiny = f64::from_bits(1);
        assert!(average_down(0.0, tiny) <= 0.0);
        assert!(average_up(0.0, tiny) >= tiny);
        assert_eq!(average_down(f64::MAX, f64::MAX), f64::MAX);
        assert_eq!(average_up(0.0, 0.0), 0.0);
    }
}
