//! The validated spectrum table, its exact location of a query and its evaluation.
use crate::exact::period_query_ordering;
use crate::record::{Abscissa, AxisScale, Interpolation, OrdinateKind, SpectrumRecord};
use crate::refusal::{Refusal, RefusalCode};
use open_pipe_stress_units::{canonical_unit, convert, unit_by_symbol, Dimension, UnitId};
use std::cmp::Ordering;

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
    /// The pseudo-spectral acceleration S(f), m/s².
    pub pseudo_acceleration_m_per_s2: f64,
    /// The scaled pseudo-spectral acceleration s·S(f), m/s².
    pub scaled_pseudo_acceleration_m_per_s2: f64,
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

    /// Locates the query exactly against the knots and the range ends (decision 2). Outside the
    /// tabulated range it is refused, `SPECTRUM_FREQUENCY_OUTSIDE_RANGE`; nothing is extrapolated.
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

    /// Evaluates the scaled pseudo-spectral acceleration at the query.
    ///
    /// After exact location, the arithmetic is ordinary binary64 (decision 2), on the authored
    /// axis (decision 1). On a knot the tabulated ordinate is returned unchanged. In a segment
    /// [x₀, x₁] with ordinates y₀, y₁, the parameter t is
    /// - frequency, linear: (f − x₀)/(x₁ − x₀);
    /// - frequency, log: ln(f/x₀)/ln(x₁/x₀);
    /// - period, linear: ((1 − f·x₀)/f)/(x₁ − x₀), the numerator 1 − f·x₀ by one fused
    ///   multiply-add (one rounding), so T − x₀ is formed without rounding 1/f first;
    /// - period, log: −ln(f·x₀)/ln(x₁/x₀), since ln(T/x₀) = −ln(f·x₀);
    ///
    /// where each ratio or product inside a logarithm is formed with one rounding when it is a
    /// normal number (through ln_1p of the exactly formed difference when it lies in [1/2, 2]),
    /// and as a difference of logarithms otherwise. The exact t lies strictly in (0, 1); the
    /// computed t is clamped to [0, 1], which only removes rounding. The ordinate is
    /// y₀ + (y₁ − y₀)t (linear) or y₀·exp(t·ln(y₁/y₀)) (log). A g multiple is then multiplied by
    /// g, and the result by the scale factor. The logarithm and exponential are the platform's,
    /// so the log rules' last bits can differ between platforms.
    pub fn evaluate(&self, query: QueryFrequency, scale: ScaleFactor) -> Result<Ordinate, Refusal> {
        let location = self.locate(query)?;
        let interpolated = match location {
            Location::Knot { index } => self.values[index],
            Location::Segment { lower } => self.interpolate(query.0, lower)?,
        };
        let pseudo_acceleration = match self.g_m_per_s2 {
            Some(g) => interpolated * g,
            None => interpolated,
        };
        let scaled = scale.0 * pseudo_acceleration;
        if !(pseudo_acceleration.is_finite() && scaled.is_finite()) {
            return Err(Refusal::new(RefusalCode::EvaluationNonfinite, &self.id, None, format!(
                "the ordinate {interpolated} gives S = {pseudo_acceleration} m/s^2 and s·S = {scaled} m/s^2 at scale {}; \
                 a value beyond the binary64 range is not published", scale.0)));
        }
        Ok(Ordinate {
            location,
            interpolated,
            pseudo_acceleration_m_per_s2: pseudo_acceleration,
            scaled_pseudo_acceleration_m_per_s2: scaled,
        })
    }

    fn interpolate(&self, f: f64, lower: usize) -> Result<f64, Refusal> {
        let (x0, x1) = (self.knots[lower], self.knots[lower + 1]);
        let (y0, y1) = (self.values[lower], self.values[lower + 1]);
        let t = match (self.abscissa, self.interpolation.abscissa) {
            (Abscissa::Frequency, AxisScale::Linear) => (f - x0) / (x1 - x0),
            (Abscissa::Frequency, AxisScale::Log) => ln_ratio(f, x0) / ln_ratio(x1, x0),
            (Abscissa::Period, AxisScale::Linear) => (-f.mul_add(x0, -1.0) / f) / (x1 - x0),
            (Abscissa::Period, AxisScale::Log) => -ln_product(f, x0) / ln_ratio(x1, x0),
        };
        let nonfinite = |what: &str, value: f64| Refusal::new(RefusalCode::EvaluationNonfinite, &self.id, None,
            format!("the interpolation {what} is {value} in the segment [{x0}, {x1}]; no value is published"));
        if !t.is_finite() {
            return Err(nonfinite("parameter", t));
        }
        let t = t.clamp(0.0, 1.0);
        let y = match self.interpolation.ordinate {
            AxisScale::Linear => y0 + (y1 - y0) * t,
            AxisScale::Log => y0 * (t * ln_ratio(y1, y0)).exp(),
        };
        if y.is_finite() {
            Ok(y)
        } else {
            Err(nonfinite("ordinate", y))
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

/// ln(a/b) for positive finite a and b.
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

/// ln(a·b) for positive finite a and b.
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
