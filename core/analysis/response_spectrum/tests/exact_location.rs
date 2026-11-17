//! Agent 0's decision 2 (DEL-04-10 scope): the period T = 1/f of the represented binary64 query
//! frequency is located against the represented binary64 knots and range ends exactly, as a
//! rational, before any interpolation; rounding 1/f never decides the range or the side of a
//! knot. Every case below is stated in represented operands: the binary64 query and knots, never
//! the decimals they were written from or an exact modal frequency.
//!
//! The expected locations are established here without the product's comparison: each case
//! writes its binary64 operands as exact rationals n/2^k (each literal is checked to equal its
//! rational, which is exact because n < 2^53), and the side follows from integer arithmetic on
//! those rationals. A second, independent oracle, the sign of the fused multiply-add residual
//! P·f − 1, is compared with the product over many operands. A frequency table is located by
//! ordinary comparison of binary64 values, which is exact.
// The integer derivations of the expected sides are checked at compile time on purpose.
#![allow(clippy::assertions_on_constants)]

use open_pipe_stress_response_spectrum::{
    period_query_ordering, Abscissa, AxisScale, Interpolation, Location, OrdinateKind, QueryFrequency, RecordUnits,
    RefusalCode, ScaleFactor, SpectrumRecord, SpectrumTable,
};
use std::cmp::Ordering;

const RULES: [(AxisScale, AxisScale); 4] = [
    (AxisScale::Linear, AxisScale::Linear),
    (AxisScale::Log, AxisScale::Log),
    (AxisScale::Linear, AxisScale::Log),
    (AxisScale::Log, AxisScale::Linear),
];

fn table(abscissa: Abscissa, points: &[[f64; 2]], rule: (AxisScale, AxisScale)) -> SpectrumTable {
    let record = SpectrumRecord {
        id: "SP".into(),
        label: None,
        abscissa,
        ordinate: OrdinateKind::Acceleration,
        units: RecordUnits { abscissa: if abscissa == Abscissa::Period { "s" } else { "Hz" }.into(), ordinate: Some("m/s^2".into()) },
        g_value: None,
        damping_ratio: 0.05,
        interpolation: Interpolation { abscissa: rule.0, ordinate: rule.1 },
        points: points.to_vec(),
        tags: None,
        notes: None,
    };
    SpectrumTable::admit(&record).expect("admitted")
}

fn period(points: &[[f64; 2]]) -> SpectrumTable {
    table(Abscissa::Period, points, (AxisScale::Linear, AxisScale::Linear))
}

fn hz(f: f64) -> QueryFrequency {
    QueryFrequency::new(f, "Hz").unwrap()
}

fn locate(table: &SpectrumTable, f: f64) -> Result<Location, RefusalCode> {
    table.locate(hz(f)).map_err(|r| r.code)
}

/// `n / 2^k` as a binary64, exact for n < 2^53.
fn rational(n: u64, k: i32) -> f64 {
    assert!(n < 1 << 53);
    n as f64 / 2f64.powi(k)
}

#[test]
fn frequency_table_range_ends_are_exact() {
    let t1 = table(Abscissa::Frequency, &[[0.3, 0.9], [1.5, 3.6], [6.25, 9.81], [25.0, 7.5], [80.0, 2.2]], RULES[0]);
    assert_eq!(locate(&t1, 0.3), Ok(Location::Knot { index: 0 }));
    assert_eq!(locate(&t1, 0.3f64.next_down()), Err(RefusalCode::FrequencyOutsideRange));
    assert_eq!(locate(&t1, 0.3f64.next_up()), Ok(Location::Segment { lower: 0 }));
    assert_eq!(locate(&t1, 80.0), Ok(Location::Knot { index: 4 }));
    assert_eq!(locate(&t1, 80.0f64.next_up()), Err(RefusalCode::FrequencyOutsideRange));
    assert_eq!(locate(&t1, 80.0f64.next_down()), Ok(Location::Segment { lower: 3 }));
    assert_eq!(locate(&t1, 6.25), Ok(Location::Knot { index: 2 }));
}

#[test]
fn period_table_range_ends_are_exact_one_ulp_either_side() {
    // T2's periods are powers of two, so 1/f is exact at its tabulated frequencies.
    let t2 = period(&[[0.03125, 0.35], [0.125, 1.1], [0.5, 0.8], [2.0, 0.25], [4.0, 0.1]]);
    assert_eq!(locate(&t2, 32.0), Ok(Location::Knot { index: 0 }));
    // A frequency above 32 Hz has a period below 1/32 s: outside, below the first knot.
    assert_eq!(locate(&t2, 32.0f64.next_up()), Err(RefusalCode::FrequencyOutsideRange));
    assert_eq!(locate(&t2, 32.0f64.next_down()), Ok(Location::Segment { lower: 0 }));
    assert_eq!(locate(&t2, 0.25), Ok(Location::Knot { index: 4 }));
    assert_eq!(locate(&t2, 0.25f64.next_down()), Err(RefusalCode::FrequencyOutsideRange));
    assert_eq!(locate(&t2, 0.25f64.next_up()), Ok(Location::Segment { lower: 3 }));
    assert_eq!(locate(&t2, 8.0), Ok(Location::Knot { index: 1 }));
    assert_eq!(locate(&t2, 8.0f64.next_up()), Ok(Location::Segment { lower: 0 }));
    assert_eq!(locate(&t2, 8.0f64.next_down()), Ok(Location::Segment { lower: 1 }));
}

#[test]
fn a_rounded_reciprocal_that_lands_on_a_knot_does_not_decide_the_location() {
    // First knot 0.1 s = 3602879701896397/2^55, query 10 Hz. 1/10 rounds to that knot, but
    // P·f = 36028797018963970/2^55 > 1, so T = 1/10 < P: below the range, refused.
    let p = rational(3602879701896397, 55);
    assert_eq!(p, 0.1);
    const _: () = assert!(3602879701896397u128 * 10 > 1u128 << 55);
    assert_eq!(1.0 / 10.0, p);
    let first = period(&[[0.1, 2.0], [0.5, 3.0], [5.0, 1.0]]);
    assert_eq!(locate(&first, 10.0), Err(RefusalCode::FrequencyOutsideRange));

    // Last knot 5 s, query f = 7205759403792793/2^55 (0.19999999999999998 Hz): 1/f rounds to 5,
    // but 5·7205759403792793 = 36028797018963965 < 2^55, so T > 5: above the range, refused.
    let f = rational(7205759403792793, 55);
    assert_eq!(f, 0.19999999999999998);
    const _: () = assert!(5 * 7205759403792793u128 < 1u128 << 55);
    assert_eq!(1.0 / f, 5.0);
    assert_eq!(locate(&first, f), Err(RefusalCode::FrequencyOutsideRange));

    // Query f = 0.2 = 3602879701896397/2^54: 1/f rounds to 5 again, but 5·3602879701896397 =
    // 18014398509481985 > 2^54, so T < 5: strictly inside the last segment.
    let f = rational(3602879701896397, 54);
    assert_eq!(f, 0.2);
    const _: () = assert!(5 * 3602879701896397u128 > 1u128 << 54);
    assert_eq!(1.0 / f, 5.0);
    assert_eq!(locate(&first, f), Ok(Location::Segment { lower: 1 }));
    for rule in RULES {
        let ordinate = table(Abscissa::Period, &[[0.1, 2.0], [0.5, 3.0], [5.0, 1.0]], rule)
            .evaluate(hz(f), ScaleFactor::new(1.0).unwrap())
            .unwrap();
        // T is within one unit in the last place of the knot, so S is the knot's ordinate to rounding.
        assert!((ordinate.pseudo_acceleration_m_per_s2 - 1.0).abs() <= 1e-15, "{rule:?}: {ordinate:?}");
    }

    // Interior knot 0.7 = 3152519739159347/2^52 and f = 6433713753386423/2^52
    // (1.4285714285714286 Hz): 1/f rounds to 0.7, but the product of the numerators is below
    // 2^104, so T > 0.7: in the segment above the knot, not on it.
    let (p, f) = (rational(3152519739159347, 52), rational(6433713753386423, 52));
    assert_eq!((p, f), (0.7, 1.4285714285714286));
    const _: () = assert!(3152519739159347u128 * 6433713753386423 < 1u128 << 104);
    assert_eq!(1.0 / f, p);
    assert_eq!(locate(&period(&[[0.5, 1.0], [0.7, 2.0], [1.0, 1.5]]), f), Ok(Location::Segment { lower: 1 }));

    // Interior knot 3/2 and f = 6004799503160661/2^53 (0.6666666666666666 Hz): 1/f rounds to
    // 1.5, but 3·6004799503160661 = 18014398509481983 < 2·2^53, so T > 1.5.
    let f = rational(6004799503160661, 53);
    assert_eq!(f, 0.6666666666666666);
    const _: () = assert!(3 * 6004799503160661u128 < 2 * (1u128 << 53));
    assert_eq!(1.0 / f, 1.5);
    assert_eq!(locate(&period(&[[1.0, 1.0], [1.5, 2.0], [2.0, 1.5]]), f), Ok(Location::Segment { lower: 1 }));
}

#[test]
fn subnormal_and_overflowing_operands_are_located_exactly() {
    // 2^-1023 is subnormal; with f = 2^1023 the product is exactly 1: on the knot.
    let tiny = f64::from_bits(1 << 51);
    assert_eq!(tiny, 2f64.powi(-1023));
    let big = 2f64.powi(1023);
    let sub = period(&[[tiny, 1.0], [1.0, 2.0]]);
    assert_eq!(locate(&sub, big), Ok(Location::Knot { index: 0 }));
    assert_eq!(locate(&sub, big.next_up()), Err(RefusalCode::FrequencyOutsideRange));
    assert_eq!(locate(&sub, big.next_down()), Ok(Location::Segment { lower: 0 }));
    // The smallest subnormal knot against 2^1023: the product is 2^-51 < 1, so T > P.
    assert_eq!(period_query_ordering(big, f64::from_bits(1)), Ordering::Greater);

    // f = 5e-324 (2^-1074): 1/f overflows binary64, yet T = 2^1074 is located exactly, above 1e308.
    let wide = period(&[[1.0, 1.0], [1e308, 2.0]]);
    let smallest = f64::from_bits(1);
    assert!((1.0 / smallest).is_infinite());
    assert_eq!(locate(&wide, smallest), Err(RefusalCode::FrequencyOutsideRange));
    // f = f64::MAX against the knot 1 s: T is tiny, below the range.
    assert_eq!(locate(&wide, f64::MAX), Err(RefusalCode::FrequencyOutsideRange));

    // Every rule evaluates to a finite ordinate between its segment's ordinates at the extremes.
    let s = ScaleFactor::new(1.0).unwrap();
    for rule in RULES {
        for (abscissa, points, f) in [
            (Abscissa::Period, [[tiny, 1.0], [1.0, 2.0]], big.next_down()),
            (Abscissa::Period, [[tiny, 1.0], [1.0, 2.0]], 3.0),
            (Abscissa::Period, [[1.0, 1.0], [1e308, 2.0]], 2f64.powi(-1000)),
            (Abscissa::Frequency, [[smallest, 1.0], [f64::MAX, 2.0]], 1.0),
            (Abscissa::Frequency, [[smallest, 1.0], [f64::MAX, 2.0]], 1e-310),
        ] {
            let y = table(abscissa, &points, rule).evaluate(hz(f), s).unwrap().pseudo_acceleration_m_per_s2;
            assert!(y.is_finite() && (1.0..=2.0).contains(&y), "{abscissa:?} {rule:?} f = {f:e}: {y}");
        }
    }
}

/// A small deterministic generator (xorshift64*), so the operands are the same on every run.
struct Operands(u64);

impl Operands {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    /// A positive binary64 in [2^-30, 2^30) with a uniformly random significand.
    fn positive(&mut self) -> f64 {
        let bits = self.next();
        let exponent = 1023 - 30 + (bits >> 58) % 60;
        f64::from_bits((exponent << 52) | (bits & ((1 << 52) - 1)))
    }
}

#[test]
fn the_exact_ordering_agrees_with_the_fused_multiply_add_residual() {
    // In this range the residual P·f − 1 is formed by one fused multiply-add, correctly rounded,
    // and is far above the subnormal range, so its sign is the exact sign and it is zero only when
    // P·f = 1. It is an independent oracle for the integer comparison.
    let oracle = |f: f64, p: f64| 0.0f64.partial_cmp(&p.mul_add(f, -1.0)).unwrap();
    let mut operands = Operands(0x9e37_79b9_7f4a_7c15);
    let mut near_knot_sides = [0usize; 3];
    for i in 0..20_000 {
        // Every fiftieth knot is a power of two, so that some reciprocals are exact and land on it.
        let p = match operands.positive() {
            p if i % 50 == 0 => f64::from_bits(p.to_bits() & !((1 << 52) - 1)),
            p => p,
        };
        let f = operands.positive();
        assert_eq!(period_query_ordering(f, p), oracle(f, p), "f = {f:e}, P = {p:e}");
        // Frequencies within a few units in the last place of 1/P, where rounding the reciprocal
        // is most likely to mislead.
        let mut g = 1.0 / p;
        for _ in 0..(operands.next() % 4) {
            g = g.next_down();
        }
        for _ in 0..7 {
            let exact = period_query_ordering(g, p);
            assert_eq!(exact, oracle(g, p), "f = {g:e}, P = {p:e}");
            near_knot_sides[(exact as i8 + 1) as usize] += 1;
            g = g.next_up();
        }
    }
    assert!(near_knot_sides.iter().all(|&n| n > 0), "every side reached near the knots: {near_knot_sides:?}");
}
