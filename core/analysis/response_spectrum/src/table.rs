//! The validated spectrum table, its exact location of a query and its evaluation.
use crate::exact::period_query_ordering;
use crate::record::{Abscissa, AxisScale, Interpolation, OrdinateKind, SpectrumRecord};
use crate::refusal::{Refusal, RefusalCode};
use crate::scaled::{exp_scaled, Scaled};
use open_pipe_stress_units::{canonical_unit, convert, unit_by_symbol, Dimension, UnitId};
use std::cmp::Ordering;

/// The relative accuracy to which every published value is stated (REQ-008's verification
/// tolerance); a value the error argument cannot support to it is refused.
pub const STATED_ACCURACY: f64 = 1e-9;

/// The premise on the platform's `ln`, `ln_1p` and `exp`: each result is within this relative
/// error, 2⁻⁵⁰ (4 ulp), of the exact value. It is assumed, not checked: Rust's `f64` methods call the
/// platform library (Apple's libm on macOS, glibc on Linux, the Microsoft C runtime on Windows),
/// whose documented accuracy for these functions is within this premise, and the crate's tests
/// observe it on the platform they run on.
pub const LIBM_RELATIVE_ERROR: f64 = f64::from_bits((1023 - 50) << 52);

/// The unit roundoff of binary64, 2⁻⁵³.
const U: f64 = f64::EPSILON / 2.0;
/// A relative margin for rounding in the bound arithmetic itself; each bound is a few operations.
const BOUND_MARGIN: f64 = f64::from_bits((1023 - 40) << 52);
/// The relative error bound of [`ln_ratio`] and [`ln_product`], 2.11·κ + 3u: in their three branches
/// 1.45·γ₂ + κ, 1.45·u + κ and 2.11·κ + u (module error argument).
const LN_BOUND: f64 = (2.11 * LIBM_RELATIVE_ERROR + 3.0 * U) * (1.0 + BOUND_MARGIN);
/// The relative error of dropping a positive addend below 2⁻¹⁰⁰⁰ of the other ([`Scaled::add`]).
const ADD_DROP: f64 = f64::from_bits(24 << 52); // 2⁻⁹⁹⁹
/// The absolute error of the exponential's reduced argument ([`exp_scaled`]): one rounding of
/// |r| ≤ 0.3466 and the product k·LN2_LO's rounding with the ln 2 split's error, for |k| ≤ 2200.
const REDUCTION_ABSOLUTE: f64 = 0.36 * U + 1e-22;

/// γₙ = n·u/(1 − n·u): the relative error of n successive roundings (Higham, Lemma 3.1).
fn gamma(n: u32) -> f64 {
    let nu = f64::from(n) * U;
    nu / (1.0 - nu) * (1.0 + BOUND_MARGIN)
}

/// The relative error bound of a product of two factors with relative errors a and b.
fn compose(a: f64, b: f64) -> f64 {
    (a + b + a * b) * (1.0 + BOUND_MARGIN)
}

/// The largest damping ratio a spectrum solve admits (REQ-001).
pub const MAX_SOLVE_DAMPING_RATIO: f64 = 0.2;

const QUERY: &str = "query";

/// A query frequency, normalized to hertz through the unit catalog.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QueryFrequency(f64);

impl QueryFrequency {
    /// A positive finite frequency in a catalog frequency unit. Anything else is refused with
    /// `SPECTRUM_QUERY_FREQUENCY_INVALID`.
    pub fn new(value: f64, unit: &str) -> Result<Self, Refusal> {
        let invalid = |detail: String| Refusal::new(RefusalCode::QueryFrequencyInvalid, QUERY, Some("frequency"), detail);
        let unit_id = unit_by_symbol(unit, Dimension::Frequency)
            .map_err(|_| invalid(format!("`{unit}` is not a frequency unit of the units catalog")))?;
        if !(value.is_finite() && value > 0.0) {
            return Err(invalid(format!("{value} is not a positive finite frequency")));
        }
        let hertz = convert(value, unit_id, UnitId::Hertz).map_err(|e| invalid(e.to_string()))?;
        if !(hertz.is_finite() && hertz > 0.0) {
            return Err(invalid(format!("{value} {unit} is not a positive finite frequency in Hz")));
        }
        Ok(Self(hertz))
    }

    /// The frequency in hertz.
    pub fn hertz(self) -> f64 {
        self.0
    }
}

/// The scale factor of a direction: positive and finite, no default (REQ-001, Directions).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaleFactor(f64);

impl ScaleFactor {
    pub fn new(value: f64) -> Result<Self, Refusal> {
        if value.is_finite() && value > 0.0 {
            Ok(Self(value))
        } else {
            Err(Refusal::new(RefusalCode::ScaleFactorInvalid, QUERY, Some("scale_factor"),
                format!("{value} is not a positive finite scale factor")))
        }
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

/// Where a query lies in a table, located exactly (period queries by
/// [`crate::period_query_ordering`]). A query outside the tabulated range has no location; it
/// is refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    /// Exactly on the knot with this index; the ordinate is the tabulated one.
    Knot { index: usize },
    /// Strictly between the knots `lower` and `lower + 1`.
    Segment { lower: usize },
}

/// The measure of a table's ordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrdinateMeasure {
    /// An acceleration in m/s².
    MetersPerSecondSquared,
    /// A multiple of the user's g.
    GMultiple,
}

/// An evaluated ordinate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ordinate {
    pub location: Location,
    /// The interpolated ordinate in the table's own measure ([`SpectrumTable::ordinate_measure`]).
    pub interpolated: f64,
    /// The certified relative error bound of `interpolated` (at most [`STATED_ACCURACY`]).
    pub interpolated_bound: f64,
    /// The pseudo-spectral acceleration S(f), m/s².
    pub pseudo_acceleration_m_per_s2: f64,
    /// The certified relative error bound of `pseudo_acceleration_m_per_s2`.
    pub pseudo_acceleration_bound: f64,
    /// The scaled pseudo-spectral acceleration s·S(f), m/s².
    pub scaled_pseudo_acceleration_m_per_s2: f64,
    /// The certified relative error bound of `scaled_pseudo_acceleration_m_per_s2`.
    pub scaled_bound: f64,
}

/// A spectrum record admitted for a spectrum solve.
///
/// Its abscissae are normalized to Hz (a frequency table) or s (a period table), and acceleration
/// ordinates to m/s², through the unit catalog; g-multiple ordinates are kept as authored with g
/// normalized to m/s². The catalog's present frequency, time and acceleration units are Hz, s and
/// m/s², so the normalization is the identity and every knot is the authored binary64 value.
#[derive(Debug, Clone, PartialEq)]
pub struct SpectrumTable {
    id: String,
    abscissa: Abscissa,
    ordinate: OrdinateKind,
    interpolation: Interpolation,
    damping_ratio: f64,
    knots: Vec<f64>,
    values: Vec<f64>,
    g_m_per_s2: Option<f64>,
}

impl SpectrumTable {
    /// Admits a record, or returns every refusal found, in field order.
    ///
    /// The record's own rules (DEL-02-01 §2.4) are refused as `SPECTRUM_RECORD_INVALID` naming the
    /// field; a valid record whose damping ratio exceeds [`MAX_SOLVE_DAMPING_RATIO`] is refused as
    /// `SPECTRUM_DAMPING_OUT_OF_RANGE`.
    pub fn admit(record: &SpectrumRecord) -> Result<Self, Vec<Refusal>> {
        let object = if record.id.trim().is_empty() { "spectrum" } else { record.id.as_str() };
        let mut refusals = Vec::new();
        macro_rules! invalid {
            ($field:expr, $detail:expr) => {
                refusals.push(Refusal::new(RefusalCode::RecordInvalid, object, Some($field), $detail))
            };
        }
        if record.id.trim().is_empty() {
            invalid!("id", "is empty".into());
        }
        let (abscissa_dimension, family) = match record.abscissa {
            Abscissa::Frequency => (Dimension::Frequency, "frequency"),
            Abscissa::Period => (Dimension::Time, "time"),
        };
        let abscissa_unit = unit_by_symbol(&record.units.abscissa, abscissa_dimension).ok();
        if abscissa_unit.is_none() {
            invalid!("units.abscissa", format!("`{}` is not a {family} unit of the units catalog", record.units.abscissa));
        }
        let acceleration = record.ordinate == OrdinateKind::Acceleration;
        let mut ordinate_unit = None;
        match (&record.units.ordinate, acceleration) {
            (None, true) => invalid!("units.ordinate", "is required when ordinate is acceleration".into()),
            (Some(_), false) => invalid!("units.ordinate", "is present only when ordinate is acceleration".into()),
            (Some(unit), true) => match unit_by_symbol(unit, Dimension::Acceleration) {
                Ok(id) => ordinate_unit = Some(id),
                Err(_) => invalid!("units.ordinate", format!("`{unit}` is not an acceleration unit of the units catalog")),
            },
            (None, false) => {}
        }
        let mut g_m_per_s2 = None;
        match (&record.g_value, acceleration) {
            (None, false) => invalid!("g_value", "is required when ordinate is g_multiple".into()),
            (Some(_), true) => invalid!("g_value", "is present only when ordinate is g_multiple".into()),
            (Some(g), false) => match unit_by_symbol(&g.unit, Dimension::Acceleration) {
                Err(_) => invalid!("g_value", format!("unit `{}` is not an acceleration unit of the units catalog", g.unit)),
                Ok(_) if !(g.value.is_finite() && g.value > 0.0) => {
                    invalid!("g_value", format!("value {} is not a positive finite acceleration", g.value));
                }
                Ok(unit) => match to_canonical(g.value, unit) {
                    Some(value) => g_m_per_s2 = Some(value),
                    None => invalid!("g_value", format!("{} {} is not a positive finite acceleration in m/s^2", g.value, g.unit)),
                },
            },
            (None, true) => {}
        }
        let damping = record.damping_ratio;
        let damping_valid = damping.is_finite() && (0.0..1.0).contains(&damping);
        if !damping_valid {
            invalid!("damping_ratio", format!("{damping} is not a finite ratio in [0, 1)"));
        }
        if record.points.len() < 2 {
            invalid!("points", format!("has {} points; a spectrum has at least two", record.points.len()));
        }
        let mut previous: Option<f64> = None;
        for (k, [x, y]) in record.points.iter().copied().enumerate() {
            let field = format!("points[{k}]");
            if !(x.is_finite() && x > 0.0) {
                invalid!(&field, format!("abscissa {x} is not positive and finite"));
            } else if previous.is_some_and(|before| x <= before) {
                invalid!(&field, format!("abscissa {x} does not strictly increase"));
            }
            if !(y.is_finite() && y > 0.0) {
                invalid!(&field, format!("ordinate {y} is not positive and finite"));
            }
            if x.is_finite() {
                previous = Some(x);
            }
        }
        // Normalization. With the present catalog it is the identity; a unit whose conversion
        // rounds could collapse or overflow a value, which is refused rather than repaired.
        let mut knots = Vec::with_capacity(record.points.len());
        let mut values = Vec::with_capacity(record.points.len());
        if refusals.is_empty() {
            let abscissa_unit = abscissa_unit.expect("checked above");
            for (k, [x, y]) in record.points.iter().copied().enumerate() {
                let field = format!("points[{k}]");
                match to_canonical(x, abscissa_unit) {
                    Some(value) if knots.last().is_none_or(|&before| value > before) => knots.push(value),
                    _ => invalid!(&field, format!("abscissa {x} is not positive, finite and strictly increasing once normalized")),
                }
                let value = match ordinate_unit {
                    Some(unit) => to_canonical(y, unit),
                    None => Some(y),
                };
                match value {
                    Some(value) => values.push(value),
                    None => invalid!(&field, format!("ordinate {y} is not positive and finite once normalized")),
                }
            }
        }
        if !refusals.is_empty() {
            return Err(refusals);
        }
        if damping > MAX_SOLVE_DAMPING_RATIO {
            return Err(vec![Refusal::new(RefusalCode::DampingOutOfRange, object, Some("damping_ratio"),
                format!("damping ratio {damping} exceeds {MAX_SOLVE_DAMPING_RATIO}, the largest a spectrum solve admits"))]);
        }
        Ok(Self {
            id: record.id.clone(),
            abscissa: record.abscissa,
            ordinate: record.ordinate,
            interpolation: record.interpolation,
            damping_ratio: damping,
            knots,
            values,
            g_m_per_s2,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn abscissa(&self) -> Abscissa {
        self.abscissa
    }

    pub fn interpolation(&self) -> Interpolation {
        self.interpolation
    }

    /// The recorded damping ratio. The ordinates are already at this damping; it enters no
    /// evaluation (REQ-002: damping is represented only through the spectra).
    pub fn damping_ratio(&self) -> f64 {
        self.damping_ratio
    }

    /// The normalized knots: Hz for a frequency table, s for a period table, ascending.
    pub fn knots(&self) -> &[f64] {
        &self.knots
    }

    /// The ordinates in [`SpectrumTable::ordinate_measure`].
    pub fn ordinates(&self) -> &[f64] {
        &self.values
    }

    pub fn ordinate_measure(&self) -> OrdinateMeasure {
        match self.ordinate {
            OrdinateKind::Acceleration => OrdinateMeasure::MetersPerSecondSquared,
            OrdinateKind::GMultiple => OrdinateMeasure::GMultiple,
        }
    }

    /// The user's g in m/s², for a g-multiple table.
    pub fn g_m_per_s2(&self) -> Option<f64> {
        self.g_m_per_s2
    }

    /// Where the query lies relative to knot `index`, on the table's authored axis, exactly.
    fn query_ordering(&self, query: QueryFrequency, index: usize) -> Ordering {
        let knot = self.knots[index];
        match self.abscissa {
            Abscissa::Frequency => query.0.partial_cmp(&knot).expect("finite operands"),
            Abscissa::Period => period_query_ordering(query.0, knot),
        }
    }

    /// Locates the represented query exactly against the represented knots and range ends
    /// (decision 2): this selects the interval and decides in or out of range, and nothing more.
    /// Outside the tabulated range the query is refused, `SPECTRUM_FREQUENCY_OUTSIDE_RANGE`;
    /// nothing is extrapolated.
    pub fn locate(&self, query: QueryFrequency) -> Result<Location, Refusal> {
        let last = self.knots.len() - 1;
        let outside = |side: &str| {
            let (axis, unit) = match self.abscissa {
                Abscissa::Frequency => ("frequency", "Hz"),
                Abscissa::Period => ("period", "s"),
            };
            Refusal::new(RefusalCode::FrequencyOutsideRange, &self.id, None, format!(
                "query frequency {} Hz lies {side} the tabulated {axis} range [{}, {}] {unit}; no spectrum is extrapolated",
                query.0, self.knots[0], self.knots[last]))
        };
        match self.query_ordering(query, 0) {
            Ordering::Less => return Err(outside("below")),
            Ordering::Equal => return Ok(Location::Knot { index: 0 }),
            Ordering::Greater => {}
        }
        match self.query_ordering(query, last) {
            Ordering::Greater => return Err(outside("above")),
            Ordering::Equal => return Ok(Location::Knot { index: last }),
            Ordering::Less => {}
        }
        // Invariant: knot[lo] < query < knot[hi].
        let (mut lo, mut hi) = (0, last);
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            match self.query_ordering(query, mid) {
                Ordering::Equal => return Ok(Location::Knot { index: mid }),
                Ordering::Less => hi = mid,
                Ordering::Greater => lo = mid,
            }
        }
        Ok(Location::Segment { lower: lo })
    }

    /// Evaluates the interpolated ordinate, S and s·S at the query, each certified to
    /// [`STATED_ACCURACY`] or refused.
    ///
    /// After exact location (decision 2), on the authored axis (decision 1), the arithmetic is
    /// binary64 on significands with exact exponent bookkeeping ([`crate::scaled`]), so no
    /// intermediate overflows or underflows whatever the magnitudes of the represented inputs. On a
    /// knot the tabulated ordinate is used unchanged. In a segment [x₀, x₁] with ordinates y₀, y₁,
    /// the two weights w₀ = 1 − t and w₁ = t are each formed directly as a quotient of positive
    /// quantities, never as 1 − t:
    /// - frequency, linear: w₁ = (f − x₀)/D and w₀ = (x₁ − f)/D, D = x₁ − x₀;
    /// - period, linear: w₁ = (1 − f·x₀)/(f·D) and w₀ = (f·x₁ − 1)/(f·D), each numerator by one fused
    ///   multiply-add (f·x₁ itself when it is at least 2⁵⁴), so T = 1/f is never rounded;
    /// - frequency, log: w₁ = ln(f/x₀)/ln(x₁/x₀) and w₀ = ln(x₁/f)/ln(x₁/x₀);
    /// - period, log: w₁ = −ln(f·x₀)/ln(x₁/x₀) and w₀ = ln(f·x₁)/ln(x₁/x₀).
    ///
    /// The ordinate is w₀·y₀ + w₁·y₁ (linear), a sum of two positive terms, or, from the knot with the
    /// larger weight, y₀·exp(w₁·L) or y₁·exp(−w₀·L) with L = ln(y₁/y₀) (log), so |z| ≤ |L|/2 in the
    /// exponent. S is the ordinate times g for a g-multiple table, and s·S is S times the scale
    /// factor, both before anything is rounded to binary64.
    ///
    /// Each published value carries a relative error bound derived from the operations (see the
    /// error argument in the crate README and the DEL-04-10 scope), under IEEE 754 binary64 round
    /// to nearest and the premise that the platform's `ln`, `ln_1p` and `exp` are each within
    /// relative error [`LIBM_RELATIVE_ERROR`] (4 ulp), plus the error of rounding it to binary64:
    /// none in the normal range, at most 2⁻¹⁰⁷⁵ absolutely in the subnormal range. A value whose bound
    /// exceeds [`STATED_ACCURACY`], or that is zero or infinite in binary64, is refused,
    /// `SPECTRUM_EVALUATION_ACCURACY_UNSUPPORTED`, and the evaluation publishes nothing.
    pub fn evaluate(&self, query: QueryFrequency, scale: ScaleFactor) -> Result<Ordinate, Refusal> {
        let location = self.locate(query)?;
        let (ordinate, ordinate_bound) = match location {
            Location::Knot { index } => (Scaled::new(self.values[index]), 0.0),
            Location::Segment { lower } => self.interpolate(query.0, lower),
        };
        let (pseudo, pseudo_bound) = match self.g_m_per_s2 {
            Some(g) => (ordinate.mul(Scaled::new(g)), compose(ordinate_bound, gamma(1))),
            None => (ordinate, ordinate_bound),
        };
        let scaled = pseudo.mul(Scaled::new(scale.0));
        let scaled_bound = compose(pseudo_bound, gamma(1));
        let (interpolated, interpolated_bound) = self.certify("interpolated ordinate", ordinate, ordinate_bound)?;
        let (pseudo_acceleration, pseudo_acceleration_bound) = self.certify("pseudo-acceleration S", pseudo, pseudo_bound)?;
        let (scaled_pseudo_acceleration, scaled_bound) = self.certify("scaled pseudo-acceleration s·S", scaled, scaled_bound)?;
        Ok(Ordinate {
            location,
            interpolated,
            interpolated_bound,
            pseudo_acceleration_m_per_s2: pseudo_acceleration,
            pseudo_acceleration_bound,
            scaled_pseudo_acceleration_m_per_s2: scaled_pseudo_acceleration,
            scaled_bound,
        })
    }

    /// The published binary64 of `value` and its total relative error bound, or the accuracy
    /// refusal. `bound` is the relative error of `value` against the exact evaluation of the
    /// represented inputs; rounding into the subnormal range adds at most 2⁻¹⁰⁷⁵ absolutely, that is
    /// 2⁻¹⁰⁷⁵·(1 + bound)/value relatively, since the exact value is at least value/(1 + bound).
    fn certify(&self, what: &str, value: Scaled, bound: f64) -> Result<(f64, f64), Refusal> {
        let published = value.to_f64();
        let (m, e) = value.parts();
        let total = if (-1022..=1023).contains(&e) {
            bound
        } else if published > 0.0 && published.is_finite() {
            bound + 2f64.powi(-1075 - e) / m * (1.0 + bound) * (1.0 + BOUND_MARGIN)
        } else {
            f64::INFINITY
        };
        if total <= STATED_ACCURACY {
            return Ok((published, total));
        }
        let why = if !published.is_finite() {
            "exceeds the largest binary64".to_string()
        } else if published == 0.0 {
            "is below the smallest binary64 and would round to zero".to_string()
        } else {
            format!("lies in the binary64 subnormal range, where its relative error bound is {total:e}")
        };
        Err(Refusal::new(RefusalCode::EvaluationAccuracyUnsupported, &self.id, None, format!(
            "the {what}, {m}·2^{e}, {why}; it cannot be stated within {STATED_ACCURACY:e} relative, and the evaluation \
             publishes no value")))
    }

    /// The ordinate on segment `lower` at the interior query frequency `f`, and its relative error
    /// bound against the exact interpolation of the represented knots, ordinates and query.
    fn interpolate(&self, f: f64, lower: usize) -> (Scaled, f64) {
        let (x0, x1) = (self.knots[lower], self.knots[lower + 1]);
        let (y0, y1) = (self.values[lower], self.values[lower + 1]);
        // Exact location makes every numerator and denominator below strictly positive: a
        // difference of binary64 values that is subnormal is exact, and 1 − f·x₀ and f·x₁ − 1, when
        // nonzero, are at least 2⁻¹⁰⁶ (the exact products lie on a grid of spacing 2^(e_f+e_x) with
        // e_f + e_x > −106 near 1), so no rounding reaches zero.
        let (w0, w1, weight_bound) = match (self.abscissa, self.interpolation.abscissa) {
            (Abscissa::Frequency, AxisScale::Linear) => {
                let d = Scaled::new(x1 - x0);
                (Scaled::new(x1 - f).div(d), Scaled::new(f - x0).div(d), gamma(3))
            }
            (Abscissa::Period, AxisScale::Linear) => {
                let n1 = Scaled::new(-f.mul_add(x0, -1.0));
                let p1 = Scaled::new(f).mul(Scaled::new(x1));
                // Below 2⁵⁴ the fused multiply-add forms f·x₁ − 1 with one rounding; at or above it,
                // the rounded product differs from f·x₁ − 1 by at most u + 2⁻⁵⁴(1 + u) relatively.
                let n0 = if p1.cmp_pow2(54) == Ordering::Less { Scaled::new(f.mul_add(x1, -1.0)) } else { p1 };
                let d = Scaled::new(f).mul(Scaled::new(x1 - x0));
                (n0.div(d), n1.div(d), gamma(5))
            }
            (abscissa, AxisScale::Log) => {
                let den = ln_ratio(x1, x0);
                let (a0, a1) = match abscissa {
                    Abscissa::Frequency => (ln_ratio(x1, f), ln_ratio(f, x0)),
                    Abscissa::Period => (ln_product(f, x1), -ln_product(f, x0)),
                };
                (Scaled::new(a0 / den), Scaled::new(a1 / den), compose(compose(LN_BOUND, LN_BOUND), gamma(1)))
            }
        };
        match self.interpolation.ordinate {
            AxisScale::Linear => {
                let y = w0.mul(Scaled::new(y0)).add(w1.mul(Scaled::new(y1)));
                (y, compose(compose(weight_bound, gamma(2)), ADD_DROP))
            }
            AxisScale::Log => {
                let l = ln_ratio(y1, y0);
                let (base, weight, sign) = if w1.le(w0) { (y0, w1, 1.0) } else { (y1, w0, -1.0) };
                if l == 0.0 {
                    // ln_ratio returns zero only for equal ordinates.
                    return (Scaled::new(base), 0.0);
                }
                // z = ±w·L; its relative error composes the weight's, L's and one rounding, and its
                // rounding to binary64 adds at most 2⁻¹⁰⁷⁵ absolutely.
                let z = sign * l.signum() * weight.mul(Scaled::new(l.abs())).to_f64();
                let dz = z.abs() * compose(compose(weight_bound, LN_BOUND), gamma(1)) + 2f64.powi(-1074);
                // e^(z*) = e^z·e^(z*−z), |e^(z*−z) − 1| ≤ 1.01·|z* − z| for |z* − z| ≤ 0.01.
                let exp_bound = compose(compose(1.01 * dz + REDUCTION_ABSOLUTE, LIBM_RELATIVE_ERROR), gamma(1));
                (Scaled::new(base).mul(exp_scaled(z)), exp_bound)
            }
        }
    }
}

/// The damping ratio shared by every table of one request, or `SPECTRUM_DAMPING_MISMATCH`
/// naming the first table that differs (REQ-001, Damping). `None` for no table.
pub fn common_damping_ratio(tables: &[&SpectrumTable]) -> Result<Option<f64>, Refusal> {
    let Some(first) = tables.first() else { return Ok(None) };
    for table in &tables[1..] {
        if table.damping_ratio != first.damping_ratio {
            return Err(Refusal::new(RefusalCode::DampingMismatch, &table.id, Some("damping_ratio"), format!(
                "damping ratio {} differs from {} of spectrum `{}`; all spectra of one request share one damping ratio",
                table.damping_ratio, first.damping_ratio, first.id)));
        }
    }
    Ok(Some(first.damping_ratio))
}

/// A positive finite value in `unit`, in its dimension's canonical unit, if that is positive
/// and finite.
fn to_canonical(value: f64, unit: UnitId) -> Option<f64> {
    let canonical = canonical_unit(unit.definition().dimension)?;
    convert(value, unit, canonical).ok().filter(|v| v.is_finite() && *v > 0.0)
}

/// ln(a/b) for positive finite a and b, within relative error [`LN_BOUND`]. When a/b is normal and in
/// [1/2, 2], ln_1p of (a − b)/b (one rounding at most in a − b, one in the quotient; ln_1p's condition
/// is at most 1.45 there); when normal elsewhere, ln of the ratio (one rounding; condition at most
/// 1/ln 2); otherwise ln a − ln b, whose exact value is at least 708.39 in magnitude while each
/// logarithm is at most 744.45, so the two libm errors amount to at most 2.11·κ relatively.
fn ln_ratio(a: f64, b: f64) -> f64 {
    let r = a / b;
    if r.is_normal() {
        if (0.5..=2.0).contains(&r) {
            ((a - b) / b).ln_1p()
        } else {
            r.ln()
        }
    } else {
        a.ln() - b.ln()
    }
}

/// ln(a·b) for positive finite a and b, within relative error [`LN_BOUND`], by the same three branches
/// as [`ln_ratio`], with a·b − 1 by one fused multiply-add in the first.
fn ln_product(a: f64, b: f64) -> f64 {
    let p = a * b;
    if p.is_normal() {
        if (0.5..=2.0).contains(&p) {
            a.mul_add(b, -1.0).ln_1p()
        } else {
            p.ln()
        }
    } else {
        a.ln() + b.ln()
    }
}
