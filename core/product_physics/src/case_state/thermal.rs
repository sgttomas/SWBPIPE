//! Private normalized thermal/free-length and axial-fit mathematics.
//!
//! Inputs already carry Kelvin, inverse Kelvin, metres and engineering strain.
//! IDs, units, material applicability and model coverage belong to the caller.
//! The returned strains are the consumed values: never reconstruct them by
//! subtracting one from the rounded descriptive stretches.
//!
//! Arithmetic assumes IEEE binary64 with correctly rounded basic operations and
//! fused `mul_add`, without reassociation. The existing exact-sum accumulator is
//! reused; product residuals are retained where their exact dyadic range fits.
//! Interpolation fractions, division and host exp/exp_m1 remain rounded operations.
//! This module supplies stable formulas and explicit range refusals, not an
//! arbitrary-range relative-error certificate or finite-deformation qualification.
use crate::pressure_sum::exact_sum;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ThermalPoint {
    pub temperature_kelvin: f64,
    /// Inverse Kelvin for coefficient tables, dimensionless for dilation tables.
    pub value: f64,
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CoefficientData {
    Constant(f64),
    Table(Vec<ThermalPoint>),
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NormalizedExpansionLaw {
    EngineeringSecant {
        datum_kelvin: f64,
        data: CoefficientData,
    },
    EngineeringDilation {
        datum_kelvin: f64,
        points: Vec<ThermalPoint>,
    },
    DifferentialPerDatumLength {
        datum_kelvin: f64,
        points: Vec<ThermalPoint>,
    },
    LogarithmicPerCurrentLength {
        datum_kelvin: f64,
        points: Vec<ThermalPoint>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ThermalInput {
    UnchangedReference,
    ExplicitIntervalStrain {
        strain: f64,
    },
    ConstantAlphaInterval {
        coefficient_per_kelvin: f64,
        temperature_change_kelvin: f64,
    },
    FreeLengthState {
        installation_kelvin: f64,
        operating_kelvin: f64,
        law: NormalizedExpansionLaw,
    },
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum FitInput {
    None,
    EngineeringStrain {
        strain: f64,
    },
    NaturalLengthChange {
        change_m: f64,
        reference_length_m: f64,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConsumedSegment {
    pub lower_index: usize,
    pub upper_index: usize,
    /// Actual covered subinterval in increasing temperature order. Direction of
    /// a consumed integral is retained by the caller's installation/operating T.
    pub start_kelvin: f64,
    pub end_kelvin: f64,
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ResolvedStrain {
    pub thermal_strain: f64,
    pub thermal_stretch: f64,
    pub fit_strain: f64,
    pub fit_stretch: f64,
    pub total_eigenstrain: f64,
    pub definition: &'static str,
    pub datum_kelvin: Option<f64>,
    pub installation_kelvin: Option<f64>,
    pub operating_kelvin: Option<f64>,
    pub installation_datum_stretch: Option<f64>,
    pub operating_datum_stretch: Option<f64>,
    pub consumed_point_indices: Vec<usize>,
    pub consumed_segments: Vec<ConsumedSegment>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StateMathError {
    NonFinite(&'static str),
    InvalidTemperature,
    InvalidReferenceLength,
    TableTooShort,
    TableTooLong,
    TableNotStrictlyIncreasing { index: usize },
    OutsideTableCoverage,
    NonzeroDilationAtDatum,
    NonPositiveStretch(&'static str),
    ArithmeticRange(&'static str),
}

fn finite(value: f64, name: &'static str) -> Result<f64, StateMathError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(StateMathError::NonFinite(name))
    }
}
fn temperature(value: f64) -> Result<(), StateMathError> {
    finite(value, "temperature")?;
    if value < 0.0 {
        Err(StateMathError::InvalidTemperature)
    } else {
        Ok(())
    }
}
fn published(value: f64, name: &'static str) -> Result<f64, StateMathError> {
    if !value.is_finite() || (value != 0.0 && !value.is_normal()) {
        Err(StateMathError::ArithmeticRange(name))
    } else {
        Ok(if value == 0.0 { 0.0 } else { value })
    }
}
fn sum(values: impl IntoIterator<Item = f64>, name: &'static str) -> Result<f64, StateMathError> {
    let result = exact_sum(values).map_err(|_| StateMathError::ArithmeticRange(name))?;
    published(result, name)
}
fn lowest_bit_exponent(value: f64) -> i32 {
    let bits = value.abs().to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & ((1u64 << 52) - 1);
    if exponent == 0 {
        -1074 + fraction.trailing_zeros() as i32
    } else {
        exponent - 1023 - 52 + ((1u64 << 52) | fraction).trailing_zeros() as i32
    }
}
/// Error-free product pair for the admitted dyadic range. Refuse a product
/// whose lowest source bit falls below binary64, rather than silently erase its
/// residual during later cancellation. FMA residual may itself be subnormal.
fn product(a: f64, b: f64) -> Result<[f64; 2], StateMathError> {
    finite(a, "product operand")?;
    finite(b, "product operand")?;
    if a == 0.0 || b == 0.0 {
        return Ok([0.0; 2]);
    }
    if lowest_bit_exponent(a) + lowest_bit_exponent(b) < -1074 {
        return Err(StateMathError::ArithmeticRange(
            "product residual underflow",
        ));
    }
    let high = published(a * b, "product")?;
    let low = a.mul_add(b, -high);
    if !low.is_finite() {
        return Err(StateMathError::ArithmeticRange("product residual"));
    }
    Ok([high, low])
}
fn divide(a: f64, b: f64) -> Result<f64, StateMathError> {
    if !b.is_finite() || b == 0.0 {
        return Err(StateMathError::ArithmeticRange("division denominator"));
    }
    let value = published(a / b, "division")?;
    if a != 0.0 && value == 0.0 {
        return Err(StateMathError::ArithmeticRange("division underflow"));
    }
    Ok(value)
}
fn stretch(
    terms: impl IntoIterator<Item = f64>,
    name: &'static str,
) -> Result<f64, StateMathError> {
    let value = sum(std::iter::once(1.0).chain(terms), name)?;
    if value <= 0.0 {
        Err(StateMathError::NonPositiveStretch(name))
    } else {
        Ok(value)
    }
}
#[derive(Default)]
struct Consumption {
    points: BTreeSet<usize>,
    segments: Vec<ConsumedSegment>,
}
impl Consumption {
    fn segment(&mut self, index: usize, start: f64, end: f64) {
        self.points.extend([index, index + 1]);
        let segment = ConsumedSegment {
            lower_index: index,
            upper_index: index + 1,
            start_kelvin: start,
            end_kelvin: end,
        };
        if !self.segments.contains(&segment) {
            self.segments.push(segment);
        }
    }
}
fn validate_table(points: &[ThermalPoint]) -> Result<(), StateMathError> {
    if points.len() > 4096 {
        return Err(StateMathError::TableTooLong);
    }
    if points.len() < 2 {
        return Err(StateMathError::TableTooShort);
    }
    for (i, p) in points.iter().enumerate() {
        temperature(p.temperature_kelvin)?;
        finite(p.value, "table value")?;
        if i > 0 && points[i - 1].temperature_kelvin >= p.temperature_kelvin {
            return Err(StateMathError::TableNotStrictlyIncreasing { index: i });
        }
    }
    Ok(())
}
fn covered(points: &[ThermalPoint], t: f64) -> Result<(), StateMathError> {
    temperature(t)?;
    if t < points[0].temperature_kelvin || t > points[points.len() - 1].temperature_kelvin {
        Err(StateMathError::OutsideTableCoverage)
    } else {
        Ok(())
    }
}
fn fraction(a: f64, b: f64, t: f64) -> Result<f64, StateMathError> {
    divide(
        sum([t, -a], "interpolation offset")?,
        sum([b, -a], "interpolation span")?,
    )
}
fn sample(points: &[ThermalPoint], t: f64, used: &mut Consumption) -> Result<f64, StateMathError> {
    covered(points, t)?;
    if let Some(i) = points.iter().position(|p| p.temperature_kelvin == t) {
        used.points.insert(i);
        return Ok(points[i].value);
    }
    let i = points
        .windows(2)
        .position(|w| w[0].temperature_kelvin < t && t < w[1].temperature_kelvin)
        .ok_or(StateMathError::OutsideTableCoverage)?;
    let (a, b) = (points[i], points[i + 1]);
    used.segment(i, t, t);
    if a.value == b.value {
        return Ok(a.value);
    }
    let f = fraction(a.temperature_kelvin, b.temperature_kelvin, t)?;
    let difference = sum([b.value, -a.value], "interpolated value difference")?;
    let p = product(difference, f)?;
    sum([a.value, p[0], p[1]], "interpolated value")
}
/// Integrate the declared piecewise-linear coefficient, retaining the fixed
/// trapezoid ordering per segment and exact sum of its rounded product pairs.
fn integral(
    points: &[ThermalPoint],
    from: f64,
    to: f64,
    used: &mut Consumption,
) -> Result<f64, StateMathError> {
    covered(points, from)?;
    covered(points, to)?;
    if from == to {
        sample(points, from, used)?;
        return Ok(0.0);
    }
    let (lo, hi, sign) = if from < to {
        (from, to, 1.0)
    } else {
        (to, from, -1.0)
    };
    let mut terms = Vec::new();
    let first = points
        .partition_point(|p| p.temperature_kelvin <= lo)
        .saturating_sub(1);
    for (i, w) in points.windows(2).enumerate().skip(first) {
        if w[0].temperature_kelvin >= hi {
            break;
        }
        let a = lo.max(w[0].temperature_kelvin);
        let b = hi.min(w[1].temperature_kelvin);
        if a >= b {
            continue;
        }
        let va = sample(points, a, used)?;
        let vb = sample(points, b, used)?;
        used.segment(i, a, b);
        // Halve before summing to avoid an unnecessary sum overflow.
        let average = sum(
            product(va, 0.5)?.into_iter().chain(product(vb, 0.5)?),
            "coefficient mean",
        )?;
        let width = sum([b, -a], "integration width")?;
        terms.extend(product(width, average)?.map(|x| sign * x));
    }
    sum(terms, "coefficient integral")
}
/// Difference of a piecewise-linear dilation is integrated from its segment
/// slopes. This preserves a small interval change when two large rounded
/// interpolated dilation values would otherwise become equal.
fn dilation_difference(
    points: &[ThermalPoint],
    from: f64,
    to: f64,
    used: &mut Consumption,
) -> Result<f64, StateMathError> {
    covered(points, from)?;
    covered(points, to)?;
    if from == to {
        return Ok(0.0);
    }
    let (lo, hi, sign) = if from < to {
        (from, to, 1.0)
    } else {
        (to, from, -1.0)
    };
    let mut terms = Vec::new();
    let first = points
        .partition_point(|p| p.temperature_kelvin <= lo)
        .saturating_sub(1);
    for (i, w) in points.windows(2).enumerate().skip(first) {
        if w[0].temperature_kelvin >= hi {
            break;
        }
        let a = lo.max(w[0].temperature_kelvin);
        let b = hi.min(w[1].temperature_kelvin);
        if a >= b {
            continue;
        }
        used.segment(i, a, b);
        let ratio = divide(
            sum([b, -a], "dilation interval")?,
            sum(
                [w[1].temperature_kelvin, -w[0].temperature_kelvin],
                "dilation segment",
            )?,
        )?;
        let delta = sum([w[1].value, -w[0].value], "dilation change")?;
        terms.extend(product(delta, ratio)?.map(|v| sign * v));
    }
    sum(terms, "dilation difference")
}
#[derive(Clone, Copy)]
enum PathKind {
    Secant,
    Dilation,
    DatumIntegral,
    LogIntegral,
}
/// A free-length law must remain defined over the complete consumed interval.
/// Segment endpoints suffice for linear dilation. Quadratic secant/datum
/// dilation and logarithmic integral add their interior stationary candidates.
fn check_path(
    points: &[ThermalPoint],
    datum: f64,
    install: f64,
    operate: f64,
    kind: PathKind,
    used: &mut Consumption,
) -> Result<(), StateMathError> {
    let lo = datum.min(install).min(operate);
    let hi = datum.max(install).max(operate);
    covered(points, lo)?;
    covered(points, hi)?;
    let mut candidates = vec![lo, hi, datum, install, operate];
    for (i, w) in points.windows(2).enumerate() {
        let a = lo.max(w[0].temperature_kelvin);
        let b = hi.min(w[1].temperature_kelvin);
        if a > b {
            continue;
        }
        candidates.extend([a, b]);
        if a == b || matches!(kind, PathKind::Dilation) {
            continue;
        }
        let va = sample(points, a, used)?;
        let vb = sample(points, b, used)?;
        let (da, db) = if matches!(kind, PathKind::Secant) {
            let slope = divide(
                sum([w[1].value, -w[0].value], "secant coefficient change")?,
                sum(
                    [w[1].temperature_kelvin, -w[0].temperature_kelvin],
                    "secant segment length",
                )?,
            )?;
            let pa = product(slope, sum([a, -datum], "secant datum offset")?)?;
            let pb = product(slope, sum([b, -datum], "secant datum offset")?)?;
            (
                sum([va, pa[0], pa[1]], "secant derivative")?,
                sum([vb, pb[0], pb[1]], "secant derivative")?,
            )
        } else {
            (va, vb)
        };
        if (da < 0.0 && db > 0.0) || (da > 0.0 && db < 0.0) {
            let fraction = divide(-da, sum([db, -da], "stationary derivative interval")?)?;
            let offset = product(fraction, sum([b, -a], "stationary temperature interval")?)?;
            let t = sum([a, offset[0], offset[1]], "stationary temperature")?;
            if a < t && t < b {
                candidates.push(t);
                used.segment(i, a, b);
            }
        }
    }
    candidates.sort_by(f64::total_cmp);
    candidates.dedup();
    // Walk disjoint sorted pieces once for integral laws. These are suitability
    // checks, not separately consumed eigenstrains or additional load sources.
    let mut previous = lo;
    let mut accumulated = if matches!(kind, PathKind::DatumIntegral | PathKind::LogIntegral) {
        integral(points, datum, lo, used)?
    } else {
        0.0
    };
    for t in candidates {
        match kind {
            PathKind::Dilation => {
                stretch([sample(points, t, used)?], "dilation path stretch")?;
            }
            PathKind::Secant => {
                stretch(
                    product(
                        sample(points, t, used)?,
                        sum([t, -datum], "secant path interval")?,
                    )?,
                    "secant path stretch",
                )?;
            }
            PathKind::DatumIntegral | PathKind::LogIntegral => {
                accumulated = sum(
                    [accumulated, integral(points, previous, t, used)?],
                    "path integral",
                )?;
                if matches!(kind, PathKind::LogIntegral) {
                    let value = published(accumulated.exp(), "logarithmic path stretch")?;
                    if value <= 0.0 {
                        return Err(StateMathError::NonPositiveStretch(
                            "logarithmic path stretch",
                        ));
                    }
                } else {
                    stretch([accumulated], "datum integral path stretch")?;
                }
                previous = t;
            }
        }
    }
    Ok(())
}

struct ThermalResolution {
    strain: f64,
    definition: &'static str,
    datum: Option<f64>,
    installation: Option<f64>,
    operating: Option<f64>,
    datum_stretches: Option<[f64; 2]>,
    used: Consumption,
}
fn direct(strain: f64, definition: &'static str) -> Result<ThermalResolution, StateMathError> {
    let strain = published(strain, "thermal strain")?;
    stretch([strain], "thermal stretch")?;
    Ok(ThermalResolution {
        strain,
        definition,
        datum: None,
        installation: None,
        operating: None,
        datum_stretches: None,
        used: Consumption::default(),
    })
}
fn thermal(input: &ThermalInput) -> Result<ThermalResolution, StateMathError> {
    match input {
        ThermalInput::UnchangedReference => direct(0.0, "unchanged_reference"),
        ThermalInput::ExplicitIntervalStrain { strain } => {
            finite(*strain, "entered thermal strain")?;
            direct(*strain, "explicit_interval_strain")
        }
        ThermalInput::ConstantAlphaInterval {
            coefficient_per_kelvin,
            temperature_change_kelvin,
        } => direct(
            sum(
                product(*coefficient_per_kelvin, *temperature_change_kelvin)?,
                "interval strain",
            )?,
            "constant_alpha_interval",
        ),
        ThermalInput::FreeLengthState {
            installation_kelvin,
            operating_kelvin,
            law,
        } => free_length(*installation_kelvin, *operating_kelvin, law),
    }
}
fn free_length(
    install: f64,
    operate: f64,
    law: &NormalizedExpansionLaw,
) -> Result<ThermalResolution, StateMathError> {
    temperature(install)?;
    temperature(operate)?;
    let mut used = Consumption::default();
    let (datum, definition, li, lo, strain) = match law {
        NormalizedExpansionLaw::EngineeringSecant { datum_kelvin, data } => {
            let datum = *datum_kelvin;
            temperature(datum)?;
            let [ai, ao] = match data {
                CoefficientData::Constant(alpha) => {
                    finite(*alpha, "secant coefficient")?;
                    [*alpha, *alpha]
                }
                CoefficientData::Table(points) => {
                    validate_table(points)?;
                    covered(points, datum)?;
                    check_path(points, datum, install, operate, PathKind::Secant, &mut used)?;
                    [
                        sample(points, install, &mut used)?,
                        sample(points, operate, &mut used)?,
                    ]
                }
            };
            let pi = product(ai, sum([install, -datum], "installation datum interval")?)?;
            let po = product(ao, sum([operate, -datum], "operating datum interval")?)?;
            let li = stretch(pi, "installation datum stretch")?;
            let lo = stretch(po, "operating datum stretch")?;
            let delta = if ai == ao {
                sum(
                    product(ai, sum([operate, -install], "thermal interval")?)?,
                    "secant dilation difference",
                )?
            } else {
                sum([po[0], po[1], -pi[0], -pi[1]], "secant dilation difference")?
            };
            (datum, "engineering_secant", li, lo, divide(delta, li)?)
        }
        NormalizedExpansionLaw::EngineeringDilation {
            datum_kelvin,
            points,
        } => {
            let datum = *datum_kelvin;
            temperature(datum)?;
            validate_table(points)?;
            if sample(points, datum, &mut used)? != 0.0 {
                return Err(StateMathError::NonzeroDilationAtDatum);
            }
            check_path(
                points,
                datum,
                install,
                operate,
                PathKind::Dilation,
                &mut used,
            )?;
            let di = sample(points, install, &mut used)?;
            let d_o = sample(points, operate, &mut used)?;
            let li = stretch([di], "installation datum stretch")?;
            let lo = stretch([d_o], "operating datum stretch")?;
            let delta = dilation_difference(points, install, operate, &mut used)?;
            (datum, "engineering_dilation", li, lo, divide(delta, li)?)
        }
        NormalizedExpansionLaw::DifferentialPerDatumLength {
            datum_kelvin,
            points,
        } => {
            let datum = *datum_kelvin;
            temperature(datum)?;
            validate_table(points)?;
            check_path(
                points,
                datum,
                install,
                operate,
                PathKind::DatumIntegral,
                &mut used,
            )?;
            let di = integral(points, datum, install, &mut used)?;
            let d_o = integral(points, datum, operate, &mut used)?;
            let li = stretch([di], "installation datum stretch")?;
            let lo = stretch([d_o], "operating datum stretch")?;
            let delta = integral(points, install, operate, &mut used)?;
            (
                datum,
                "differential_per_datum_length",
                li,
                lo,
                divide(delta, li)?,
            )
        }
        NormalizedExpansionLaw::LogarithmicPerCurrentLength {
            datum_kelvin,
            points,
        } => {
            let datum = *datum_kelvin;
            temperature(datum)?;
            validate_table(points)?;
            check_path(
                points,
                datum,
                install,
                operate,
                PathKind::LogIntegral,
                &mut used,
            )?;
            let di = integral(points, datum, install, &mut used)?;
            let d_o = integral(points, datum, operate, &mut used)?;
            let li = published(di.exp(), "installation logarithmic stretch")?;
            let lo = published(d_o.exp(), "operating logarithmic stretch")?;
            if li <= 0.0 || lo <= 0.0 {
                return Err(StateMathError::NonPositiveStretch(
                    "logarithmic datum stretch",
                ));
            }
            let delta = integral(points, install, operate, &mut used)?;
            let strain = published(delta.exp_m1(), "logarithmic thermal strain")?;
            if delta != 0.0 && strain == 0.0 {
                return Err(StateMathError::ArithmeticRange(
                    "logarithmic thermal strain underflow",
                ));
            }
            (datum, "logarithmic_per_current_length", li, lo, strain)
        }
    };
    published(strain, "thermal strain")?;
    stretch([strain], "thermal stretch")?;
    Ok(ThermalResolution {
        strain,
        definition,
        datum: Some(datum),
        installation: Some(install),
        operating: Some(operate),
        datum_stretches: Some([li, lo]),
        used,
    })
}
pub(crate) fn resolve_strain(
    thermal_input: &ThermalInput,
    fit_input: &FitInput,
) -> Result<ResolvedStrain, StateMathError> {
    let thermal = thermal(thermal_input)?;
    let fit = match fit_input {
        FitInput::None => 0.0,
        FitInput::EngineeringStrain { strain } => published(*strain, "fit strain")?,
        FitInput::NaturalLengthChange {
            change_m,
            reference_length_m,
        } => {
            finite(*change_m, "natural length change")?;
            if !reference_length_m.is_finite() || *reference_length_m <= 0.0 {
                return Err(StateMathError::InvalidReferenceLength);
            }
            if sum([*reference_length_m, *change_m], "natural length")? <= 0.0 {
                return Err(StateMathError::NonPositiveStretch("fit natural length"));
            }
            divide(*change_m, *reference_length_m)?
        }
    };
    let thermal_stretch = stretch([thermal.strain], "thermal stretch")?;
    let fit_stretch = stretch([fit], "fit stretch")?;
    let cross = product(fit, thermal.strain)?;
    let total = sum(
        [fit, thermal.strain, cross[0], cross[1]],
        "total eigenstrain",
    )?;
    stretch([total], "combined stretch")?;
    Ok(ResolvedStrain {
        thermal_strain: thermal.strain,
        thermal_stretch,
        fit_strain: fit,
        fit_stretch,
        total_eigenstrain: total,
        definition: thermal.definition,
        datum_kelvin: thermal.datum,
        installation_kelvin: thermal.installation,
        operating_kelvin: thermal.operating,
        installation_datum_stretch: thermal.datum_stretches.map(|v| v[0]),
        operating_datum_stretch: thermal.datum_stretches.map(|v| v[1]),
        consumed_point_indices: thermal.used.points.into_iter().collect(),
        consumed_segments: thermal.used.segments,
    })
}
