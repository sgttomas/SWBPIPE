//! Integration candidate: private retained-source unloaded circular-span maximum.
//!
//! Caller proves the admitted source family, constant N, affine My/Mz, and the
//! same positive numeric As/Z for the whole span. Inputs must come directly from
//! retained SectionLocal endpoint functionals [0,4,5], never K*u or public JSON.
//! Numeric As/Z are exact represented operands here, not a geometry/material
//! accuracy certificate. No public/importable proof or new method ID is created.
//!
//! Requires IEEE binary64 round-to-nearest, gradual underflow, correctly rounded
//! sqrt, and no fast-math/reassociation. Outward arithmetic follows the existing
//! elastic_extrema convention; no platform hypot or guessed rounding margin.

const CRITERION: f64 = 1e-9;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct ProjectionBounds {
    functional_index: usize,
    value: f64,
    interval: [f64; 2],
}

impl ProjectionBounds {
    /// Internal numeric transport ONLY. Caller copies these three fields from
    /// one owned retained projection after descriptor/source binding. Passing
    /// arbitrary numbers cannot establish source qualification. No Deserialize.
    pub(super) fn from_retained_parts(
        functional_index: usize,
        value: f64,
        interval: [f64; 2],
    ) -> Result<Self, MaximumError> {
        if !value.is_finite()
            || interval.iter().any(|x| !x.is_finite())
            || interval[0] > value
            || value > interval[1]
        {
            return Err(MaximumError::InvalidProjection);
        }
        Ok(Self {
            functional_index,
            value,
            interval,
        })
    }

    fn absolute(self) -> [f64; 2] {
        let [lo, hi] = self.interval;
        if lo >= 0.0 {
            [lo, hi]
        } else if hi <= 0.0 {
            [-hi, -lo]
        } else {
            [0.0, (-lo).max(hi)]
        }
    }

    fn singleton(self) -> Option<f64> {
        (self.interval[0] == self.interval[1]).then_some(self.interval[0])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Endpoint {
    I,
    J,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Locations {
    /// Strict endpoint interval separation, plus convexity, excludes every
    /// other station. This does not assert a unique circumferential fiber.
    StrictEndpoint(Endpoint),
    /// Both retained endpoint bending components are singleton exact dyadics
    /// and equal with sign. An affine vector with equal ends is constant.
    WholeSpanConstant,
    /// Both are candidates. exact_tie_proven requires singleton bending components
    /// equal in absolute value componentwise, not equal rounded stress results.
    /// An unresolved overlap can still permit a constant field/interior ties.
    EndpointCandidates {
        exact_tie_proven: bool,
        interior_equal_possible: bool,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct EndpointEstimate {
    pub value_pa: f64,
    pub interval_pa: [f64; 2],
    /// [N, My, Mz] indices in the same retained functional set.
    pub functional_indices: [usize; 3],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct EndpointMaximum {
    /// Deterministic max of the two computed representative values.
    pub value_pa: f64,
    /// Encloses the true global maximum for the admitted source span.
    pub interval_pa: [f64; 2],
    pub absolute_error_bound_pa: f64,
    /// Upper bound relative to the actual positive maximum, using its lower
    /// bound as denominator. Exactly-zero stress uses 0 by convention.
    pub relative_error_bound: f64,
    /// A representative endpoint, not an unconditional exact argmax claim.
    pub witness: Endpoint,
    pub locations: Locations,
    pub endpoints: [EndpointEstimate; 2],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MaximumError {
    InvalidProjection,
    InvalidSection,
    /// Constant-N source precondition contradicted by the supplied enclosures.
    InconsistentAxialBounds,
    ArithmeticRange,
    /// Nonzero maximum or its nonzero global bounds cannot be published normal.
    PublicationRange,
    ProtectedCriterionUnestablished,
}

/// Pure bounded computation for one admitted unloaded straight span. Arguments
/// are endpoints i,j, each ordered [N, My, Mz]. Ownership, descriptor identity,
/// model/case/member/section agreement and invocation budget are caller-owned.
pub(super) fn endpoint_maximum(
    area: f64,
    section_modulus: f64,
    ends: [[ProjectionBounds; 3]; 2],
) -> Result<EndpointMaximum, MaximumError> {
    if !area.is_finite() || !section_modulus.is_finite() || area <= 0.0 || section_modulus <= 0.0 {
        return Err(MaximumError::InvalidSection);
    }
    if !overlap(ends[0][0].interval, ends[1][0].interval) {
        return Err(MaximumError::InconsistentAxialBounds);
    }
    let endpoints = [
        endpoint(area, section_modulus, ends[0])?,
        endpoint(area, section_modulus, ends[1])?,
    ];
    let witness = if endpoints[1].value_pa > endpoints[0].value_pa {
        Endpoint::J
    } else {
        Endpoint::I
    };
    let value = endpoints[0].value_pa.max(endpoints[1].value_pa);
    let lo = endpoints[0].interval_pa[0].max(endpoints[1].interval_pa[0]);
    let hi = endpoints[0].interval_pa[1].max(endpoints[1].interval_pa[1]);
    let absolute = if lo == value && value == hi {
        0.0
    } else {
        up((value - lo).max(hi - value))?
    };
    let relative = if hi == 0.0 {
        // Nonnegative stress + exact zero upper bound proves whole-span zero.
        0.0
    } else {
        if !value.is_normal() || !hi.is_normal() || (lo != 0.0 && !lo.is_normal()) {
            return Err(MaximumError::PublicationRange);
        }
        if lo <= 0.0 {
            return Err(MaximumError::ProtectedCriterionUnestablished);
        }
        let relative = if absolute == 0.0 {
            0.0
        } else {
            up(absolute / lo)?
        };
        if relative > CRITERION {
            return Err(MaximumError::ProtectedCriterionUnestablished);
        }
        relative
    };
    let same_exact_bending = (1..3).all(|c| exact_equal(ends[0][c], ends[1][c], false));
    let locations = if same_exact_bending {
        Locations::WholeSpanConstant
    } else if endpoints[0].interval_pa[0] > endpoints[1].interval_pa[1] {
        Locations::StrictEndpoint(Endpoint::I)
    } else if endpoints[1].interval_pa[0] > endpoints[0].interval_pa[1] {
        Locations::StrictEndpoint(Endpoint::J)
    } else {
        Locations::EndpointCandidates {
            exact_tie_proven: (1..3).all(|c| exact_equal(ends[0][c], ends[1][c], true)),
            interior_equal_possible: (1..3)
                .all(|c| overlap(ends[0][c].interval, ends[1][c].interval)),
        }
    };
    Ok(EndpointMaximum {
        value_pa: value,
        interval_pa: [lo, hi],
        absolute_error_bound_pa: absolute,
        relative_error_bound: relative,
        witness,
        locations,
        endpoints,
    })
}

fn overlap(a: [f64; 2], b: [f64; 2]) -> bool {
    a[0] <= b[1] && b[0] <= a[1]
}

fn exact_equal(a: ProjectionBounds, b: ProjectionBounds, magnitude: bool) -> bool {
    match (a.singleton(), b.singleton()) {
        (Some(x), Some(y)) => {
            if magnitude {
                x.abs() == y.abs()
            } else {
                x == y
            }
        }
        _ => false,
    }
}

fn endpoint(
    area: f64,
    z: f64,
    actions: [ProjectionBounds; 3],
) -> Result<EndpointEstimate, MaximumError> {
    // Scaling each moment by Z first prevents avoidable moment-norm overflow.
    // Explicit evaluation order is part of this private recipe.
    let [n, my, mz] = actions.map(|p| p.value.abs());
    let axial = finite(n / area)?;
    let by = finite(my / z)?;
    let bz = finite(mz / z)?;
    let value = finite(axial + scaled_norm(by, bz)?)?;
    let [n, my, mz] = actions.map(ProjectionBounds::absolute);
    let mut bounds = [0.0; 2];
    for (side, upper) in [false, true].into_iter().enumerate() {
        let axial = divide(n[side], area, upper)?;
        let by = divide(my[side], z, upper)?;
        let bz = divide(mz[side], z, upper)?;
        bounds[side] = add(axial, norm_bound(by, bz, upper)?, upper)?;
    }
    if bounds[0] > value || value > bounds[1] {
        return Err(MaximumError::ArithmeticRange);
    }
    Ok(EndpointEstimate {
        value_pa: value,
        interval_pa: bounds,
        functional_indices: actions.map(|p| p.functional_index),
    })
}

fn finite(x: f64) -> Result<f64, MaximumError> {
    if x.is_finite() {
        Ok(x)
    } else {
        Err(MaximumError::ArithmeticRange)
    }
}
fn up(x: f64) -> Result<f64, MaximumError> {
    finite(x.next_up())
}
fn directed(x: f64, upper: bool) -> Result<f64, MaximumError> {
    finite(x)?;
    if upper {
        up(x)
    } else {
        Ok(x.next_down().max(0.0))
    }
}
fn divide(x: f64, d: f64, upper: bool) -> Result<f64, MaximumError> {
    // Exact identities preserve true zero and singleton dyadic cases.
    if x == 0.0 || d == 1.0 {
        return Ok(x);
    }
    if x == d {
        return Ok(1.0);
    }
    directed(x / d, upper)
}
fn multiply(x: f64, y: f64, upper: bool) -> Result<f64, MaximumError> {
    if x == 0.0 || y == 0.0 {
        return Ok(0.0);
    }
    if x == 1.0 {
        return Ok(y);
    }
    if y == 1.0 {
        return Ok(x);
    }
    directed(x * y, upper)
}
fn add(x: f64, y: f64, upper: bool) -> Result<f64, MaximumError> {
    if x == 0.0 {
        return Ok(y);
    }
    if y == 0.0 {
        return Ok(x);
    }
    directed(x + y, upper)
}

fn scaled_norm(y: f64, z: f64) -> Result<f64, MaximumError> {
    let m = y.max(z);
    if m == 0.0 {
        return Ok(0.0);
    }
    let a = y / m;
    let b = z / m;
    finite(m * ((a * a) + (b * b)).sqrt())
}

// Norm is monotone in nonnegative components. Bound the exact corner norm with
// the same fixed max-scaled arithmetic order, rounding every operation outward.
// Subnormal ratios/squares are enclosed with next_up, never dropped as exact 0.
fn norm_bound(y: f64, z: f64, upper: bool) -> Result<f64, MaximumError> {
    if y == 0.0 {
        return Ok(z);
    }
    if z == 0.0 {
        return Ok(y);
    }
    let m = y.max(z);
    let a = divide(y, m, upper)?;
    let b = divide(z, m, upper)?;
    let square_a = multiply(a, a, upper)?;
    let square_b = multiply(b, b, upper)?;
    let sum = add(square_a, square_b, upper)?;
    let root = directed(sum.sqrt(), upper)?;
    multiply(m, root, upper)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn p(i: usize, v: f64) -> ProjectionBounds {
        ProjectionBounds::from_retained_parts(i, v, [v, v]).unwrap()
    }
    fn e(i: usize, n: f64, y: f64, z: f64) -> [ProjectionBounds; 3] {
        [p(i, n), p(i + 1, y), p(i + 2, z)]
    }
    fn within(r: EndpointMaximum, expected: f64) {
        assert!(r.interval_pa[0] <= expected && expected <= r.interval_pa[1]);
        assert!(r.relative_error_bound <= CRITERION);
    }
    #[test]
    fn axial_and_zero_constant_fields_are_not_unique_endpoint_maxima() {
        let r = endpoint_maximum(2.0, 1.0, [e(0, -10.0, 0.0, 0.0), e(3, -10.0, 0.0, 0.0)]).unwrap();
        within(r, 5.0);
        assert_eq!(r.locations, Locations::WholeSpanConstant);
        let z = endpoint_maximum(2.0, 1.0, [e(0, 0.0, 0.0, 0.0), e(3, 0.0, 0.0, 0.0)]).unwrap();
        assert_eq!(z.interval_pa, [0.0, 0.0]);
        assert_eq!(z.absolute_error_bound_pa, 0.0);
    }
    #[test]
    fn pure_biaxial_opposite_and_strict_controls() {
        let r = endpoint_maximum(1.0, 2.0, [e(0, 0.0, 3.0, 0.0), e(3, 0.0, 3.0, 0.0)]).unwrap();
        within(r, 1.5);
        let r = endpoint_maximum(2.0, 2.0, [e(0, 10.0, 3.0, 4.0), e(3, 10.0, 3.0, 4.0)]).unwrap();
        within(r, 7.5);
        assert_eq!(r.locations, Locations::WholeSpanConstant);
        let r = endpoint_maximum(1.0, 1.0, [e(0, 0.0, 3.0, 4.0), e(3, 0.0, -3.0, -4.0)]).unwrap();
        within(r, 5.0);
        assert_eq!(
            r.locations,
            Locations::EndpointCandidates {
                exact_tie_proven: true,
                interior_equal_possible: false
            }
        );
        let r = endpoint_maximum(1.0, 1.0, [e(0, 0.0, 1.0, 0.0), e(3, 0.0, 2.0, 0.0)]).unwrap();
        within(r, 2.0);
        assert_eq!(r.locations, Locations::StrictEndpoint(Endpoint::J));
        assert_eq!(r.witness, Endpoint::J);
        assert_eq!(r.endpoints[0].functional_indices, [0, 1, 2]);
        assert_eq!(r.endpoints[1].functional_indices, [3, 4, 5]);
    }
    #[test]
    fn projection_uncertainty_is_composed_and_not_reported_as_exact_equality() {
        let mut ends = [e(0, 1.0, 3.0, 4.0), e(3, 1.0, 3.0, 4.0)];
        for end in &mut ends {
            end[1] = ProjectionBounds::from_retained_parts(
                end[1].functional_index,
                3.0,
                [3.0 - 1e-10, 3.0 + 1e-10],
            )
            .unwrap();
        }
        let r = endpoint_maximum(1.0, 1.0, ends).unwrap();
        within(r, 6.0);
        assert!(r.absolute_error_bound_pa > 1e-11);
        assert_eq!(
            r.locations,
            Locations::EndpointCandidates {
                exact_tie_proven: false,
                interior_equal_possible: true
            }
        );
        ends[0][0] = ProjectionBounds::from_retained_parts(0, 1.0, [0.9, 1.1]).unwrap();
        ends[1][0] = ProjectionBounds::from_retained_parts(3, 1.0, [0.9, 1.1]).unwrap();
        assert_eq!(
            endpoint_maximum(1.0, 1.0, ends),
            Err(MaximumError::ProtectedCriterionUnestablished)
        );
    }
    #[test]
    fn cancellation_sensitive_action_is_bounded_not_recovered_from_displacements() {
        // Exact retained affine source: (2^53 + 1) - 2^53 = 1. The input here
        // models its projection enclosure, not the catastrophic naive f64 sum.
        let mut ends = [e(0, 1.0, 0.0, 0.0), e(3, 1.0, 0.0, 0.0)];
        for end in &mut ends {
            end[0] = ProjectionBounds::from_retained_parts(
                end[0].functional_index,
                1.0,
                [1.0 - 1e-12, 1.0 + 1e-12],
            )
            .unwrap();
        }
        let r = endpoint_maximum(2.0, 1.0, ends).unwrap();
        within(r, 0.5);
        assert!(r.absolute_error_bound_pa > 4e-13);
    }
    #[test]
    fn primitive_criterion_does_not_automatically_qualify_the_derived_recipe() {
        // Each exact represented enclosure meets 1e-9; the independent Fraction
        // check records those comparisons. The composed arithmetic cannot.
        let y = ProjectionBounds::from_retained_parts(1, 3.0, [2.9999999970000006, 3.000000003])
            .unwrap();
        let z =
            ProjectionBounds::from_retained_parts(2, 4.0, [3.9999999960000006, 4.000000003999999])
                .unwrap();
        assert_eq!(
            endpoint_maximum(1.0, 1.0, [[p(0, 0.0), y, z]; 2]),
            Err(MaximumError::ProtectedCriterionUnestablished)
        );
    }
    #[test]
    fn extreme_normal_and_subnormal_projection_controls() {
        for v in [1e-300, 1e300] {
            let r = endpoint_maximum(1.0, 1.0, [e(0, v, v, v), e(3, v, v, v)]).unwrap();
            assert!(r.value_pa.is_normal());
            assert!(r.relative_error_bound <= CRITERION);
        }
        let tiny = f64::from_bits(1);
        let r = endpoint_maximum(tiny, 1.0, [e(0, tiny, 0.0, 0.0), e(3, tiny, 0.0, 0.0)]).unwrap();
        within(r, 1.0);
        assert_eq!(
            endpoint_maximum(1.0, 1.0, [e(0, tiny, 0.0, 0.0), e(3, tiny, 0.0, 0.0)]),
            Err(MaximumError::PublicationRange)
        );
        assert_eq!(
            endpoint_maximum(2.0, 1.0, [e(0, tiny, 0.0, 0.0), e(3, tiny, 0.0, 0.0)]),
            Err(MaximumError::PublicationRange)
        );
        // A tiny norm component may underflow when squared, but its nonzero
        // contribution remains inside the outward bound of a normal maximum.
        assert!(endpoint_maximum(1.0, 1.0, [e(0, 0.0, 1.0, tiny), e(3, 0.0, 1.0, tiny)]).is_ok());
        assert!(endpoint_maximum(
            1.0,
            1.0,
            [e(0, 1e308, 1e308, 1e308), e(3, 1e308, 1e308, 1e308)]
        )
        .is_err());
    }
    #[test]
    fn invalid_numeric_inputs_and_contradicted_constant_n_are_refused() {
        assert!(ProjectionBounds::from_retained_parts(0, 1.0, [2.0, 3.0]).is_err());
        assert!(ProjectionBounds::from_retained_parts(0, f64::NAN, [0.0, 1.0]).is_err());
        let ends = [e(0, 1.0, 0.0, 0.0), e(3, 2.0, 0.0, 0.0)];
        assert_eq!(
            endpoint_maximum(1.0, 1.0, ends),
            Err(MaximumError::InconsistentAxialBounds)
        );
        assert_eq!(
            endpoint_maximum(0.0, 1.0, ends),
            Err(MaximumError::InvalidSection)
        );
    }
}
