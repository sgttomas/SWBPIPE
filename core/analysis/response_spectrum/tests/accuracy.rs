//! Final-result evidence for the conditional error bounds (IEEE 754 round to nearest and the
//! unchecked libm premise in `LIBM_RELATIVE_ERROR`), not a direct test or proof of the
//! primitive-function premise. Each published value is checked against
//! independent expectations computed from the exact rationals of the represented inputs at 150
//! digits (`tests/oracle/generate.py`, which reads no product code).
//!
//! The cases are the ten probes of the first independent review at 03c88215f, rebuilt from their
//! stated inputs (their 150-digit expectations agree with the review's to 1e-41), published values
//! either side of the subnormal threshold and of 2^1024, the period near-knot numerator
//! 1 − f·x₀ = 2⁻¹⁰⁴, and 240 seeded adversarial tables over all eight axis and rule combinations, with
//! exponents across the whole binary64 range and queries within a few units in the last place of a
//! knot. They test the error argument on these inputs; they do not prove it.
use open_pipe_stress_response_spectrum::{
    Abscissa, AxisScale, Interpolation, Location, OrdinateKind, QuantityInput, QueryFrequency, RecordUnits,
    RefusalCode, ScaleFactor, SpectrumRecord, SpectrumTable, STATED_ACCURACY,
};

pub struct Case {
    pub name: &'static str,
    pub period: bool,
    pub log_x: bool,
    pub log_y: bool,
    pub points: [[f64; 2]; 2],
    pub f: f64,
    pub g: Option<f64>,
    pub s: f64,
    pub expected: Option<[&'static str; 3]>,
}

#[path = "oracle/cases.rs"]
mod cases;
use cases::CASES;

const U: f64 = f64::EPSILON / 2.0;

fn table(case: &Case) -> SpectrumTable {
    let scale = |log| if log { AxisScale::Log } else { AxisScale::Linear };
    let record = SpectrumRecord {
        id: case.name.into(),
        label: None,
        abscissa: if case.period { Abscissa::Period } else { Abscissa::Frequency },
        ordinate: if case.g.is_some() { OrdinateKind::GMultiple } else { OrdinateKind::Acceleration },
        units: RecordUnits {
            abscissa: if case.period { "s" } else { "Hz" }.into(),
            ordinate: if case.g.is_some() { None } else { Some("m/s^2".into()) },
        },
        g_value: case.g.map(|value| QuantityInput { value, unit: "m/s^2".into() }),
        damping_ratio: 0.05,
        interpolation: Interpolation { abscissa: scale(case.log_x), ordinate: scale(case.log_y) },
        points: case.points.to_vec(),
        tags: None,
        notes: None,
    };
    SpectrumTable::admit(&record).unwrap_or_else(|r| panic!("{}: {r:?}", case.name))
}

/// The relative difference of `published` from the exact `expected`, allowing for the rounding of
/// the expected decimal to binary64 (at most u relatively, or 2⁻¹⁰⁷⁵ absolutely when subnormal) and
/// of this check's own two operations.
fn check(name: &str, what: &str, published: f64, expected: &str, bound: f64) -> f64 {
    let e: f64 = expected.parse().unwrap();
    assert!(e > 0.0 && e.is_finite(), "{name} {what}: expected {expected} parses to {e}");
    let parse = if e >= f64::MIN_POSITIVE { U } else { f64::from_bits(1) / 2.0 / e };
    let r = ((published - e) / e).abs();
    assert!(bound <= STATED_ACCURACY, "{name} {what}: certified bound {bound:e} exceeds the stated accuracy");
    assert!(r <= bound + parse + 2.0 * U, "{name} {what}: {published:e} against {expected}, relative {r:e} > bound {bound:e}");
    r
}

#[test]
fn every_published_value_meets_its_certified_bound_or_the_evaluation_is_refused() {
    let (mut accepted, mut refused, mut largest_ratio) = (0, 0, 0.0f64);
    for case in CASES {
        let outcome = table(case).evaluate(QueryFrequency::new(case.f, "Hz").unwrap(), ScaleFactor::new(case.s).unwrap());
        match (case.expected, outcome) {
            (Some([y, s, ss]), Ok(o)) => {
                for (what, published, expected, bound) in [
                    ("ordinate", o.interpolated, y, o.interpolated_bound),
                    ("S", o.pseudo_acceleration_m_per_s2, s, o.pseudo_acceleration_bound),
                    ("s·S", o.scaled_pseudo_acceleration_m_per_s2, ss, o.scaled_bound),
                ] {
                    let r = check(case.name, what, published, expected, bound);
                    if bound > 0.0 {
                        largest_ratio = largest_ratio.max(r / bound);
                    }
                }
                accepted += 1;
            }
            (None, Err(refusal)) => {
                assert_eq!(refusal.code, RefusalCode::EvaluationAccuracyUnsupported, "{}: {refusal}", case.name);
                refused += 1;
            }
            (expected, outcome) => panic!("{}: expected {expected:?}, got {outcome:?}", case.name),
        }
    }
    assert_eq!((accepted, refused), (240, 21));
    eprintln!("accuracy: {accepted} accepted, {refused} refused; largest observed error / certified bound {largest_ratio:.3}");
}

#[test]
fn the_first_review_witnesses_have_their_stated_outcomes() {
    let outcome = |name: &str| {
        let case = CASES.iter().find(|c| c.name == name).unwrap();
        table(case).evaluate(QueryFrequency::new(case.f, "Hz").unwrap(), ScaleFactor::new(case.s).unwrap())
    };
    for name in ["linear_cancellation", "log_exp_overflow", "log_exp_subnormal", "log_exp_zero", "log_exp_precision",
                 "linear_parameter_underflow", "period_subnormal_spacing", "period_subnormal_spacing_logx"] {
        let o = outcome(name).unwrap_or_else(|r| panic!("{name}: {r}"));
        assert!(matches!(o.location, Location::Segment { .. }), "{name}");
    }
    // S = 1e-400 is below the smallest binary64; S = 1e-320 is subnormal with a rounding of up to
    // 2.5e-4 of itself. Each is refused although s·S alone would be representable.
    for name in ["g_underflow_then_scale", "g_subnormal_then_scale"] {
        let refusal = outcome(name).unwrap_err();
        assert_eq!(refusal.code, RefusalCode::EvaluationAccuracyUnsupported, "{name}");
        assert!(refusal.detail.contains("pseudo-acceleration S"), "{name}: {}", refusal.detail);
    }
}

// These characterize today's conservative policy, not permanent required refusals. A later
// argument may publish these exact answers if it establishes their accuracy. No tolerance or
// arithmetic repair is introduced here.
#[test]
fn current_argument_can_refuse_exact_subnormal_and_upper_edge_answers() {
    for (name, points, f, exact) in [
        ("exact_subnormal_knot", [[1.0, 1e-320], [2.0, 1e-320]], 1.0, 1e-320),
        ("constant_upper_edge", [[f64::from_bits(1.0f64.to_bits() - 1), f64::MAX], [2.0, f64::MAX]], 1.0, f64::MAX),
    ] {
        // A constant table has this exact represented answer at every query, including the knot.
        let case = Case { name, period: false, log_x: false, log_y: false, points,
            f, g: None, s: 1.0, expected: None };
        assert!(exact.is_finite() && exact > 0.0);
        assert_eq!(case.points[0][1], exact);
        assert_eq!(case.points[1][1], exact);
        let refusal = table(&case).evaluate(QueryFrequency::new(f, "Hz").unwrap(), ScaleFactor::new(1.0).unwrap()).unwrap_err();
        assert_eq!(refusal.code, RefusalCode::EvaluationAccuracyUnsupported);
        assert!(refusal.detail.contains("this implemented error argument does not establish"));
        assert!(!refusal.detail.contains("cannot be stated"));
    }
}
