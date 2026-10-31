//! RT-A4 slice K: the absolute sum and the SRSS (DEL-05-02 Work
//! `absolute-sum-srss-combinations.md` §3, §7 V-1 to V-4). V-1's expected bits
//! are the frozen table of `validation/references/rt_a4_combinations/`, derived
//! by its own generator in exact rationals; nothing here produces them.
use super::*;
use crate::unsigned::{
    adjust, exceeds_term_limit, exponent, lower_gap, midpoint_sign, rounded_root, srss_value,
    upper_gap, Midpoint,
};
use open_pipe_stress_correct_norm::{norm2, norm3};
use open_pipe_stress_frame_kernel::exact_sum::{ExactAccumulator, SumError};

const TABLE: &str =
    include_str!("../../../../validation/references/rt_a4_combinations/references.json");
const REFUSED: &str = "non_representable";

/// The few JSON forms the table uses: objects, arrays and plain strings.
#[derive(Debug)]
enum Json {
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    fn get(&self, key: &str) -> &Json {
        match self {
            Json::Obj(fields) => &fields.iter().find(|(k, _)| k == key).expect(key).1,
            _ => panic!("not an object: {key}"),
        }
    }
    fn str(&self) -> &str {
        match self {
            Json::Str(s) => s,
            _ => panic!("not a string"),
        }
    }
    fn arr(&self) -> &[Json] {
        match self {
            Json::Arr(items) => items,
            _ => panic!("not an array"),
        }
    }
}

fn parse(text: &str) -> Json {
    fn skip(b: &[u8], i: &mut usize) {
        while b[*i].is_ascii_whitespace() {
            *i += 1;
        }
    }
    fn string(b: &[u8], i: &mut usize) -> String {
        assert_eq!(b[*i], b'"');
        let start = *i + 1;
        let end = start + b[start..].iter().position(|&c| c == b'"').unwrap();
        assert!(!b[start..end].contains(&b'\\'), "escapes are not used");
        *i = end + 1;
        String::from_utf8(b[start..end].to_vec()).unwrap()
    }
    fn value(b: &[u8], i: &mut usize) -> Json {
        skip(b, i);
        match b[*i] {
            b'"' => Json::Str(string(b, i)),
            open @ (b'[' | b'{') => {
                let close = if open == b'[' { b']' } else { b'}' };
                *i += 1;
                let (mut items, mut fields) = (Vec::new(), Vec::new());
                loop {
                    skip(b, i);
                    if b[*i] == close {
                        *i += 1;
                        break;
                    }
                    if open == b'[' {
                        items.push(value(b, i));
                    } else {
                        let key = string(b, i);
                        skip(b, i);
                        assert_eq!(b[*i], b':');
                        *i += 1;
                        fields.push((key, value(b, i)));
                    }
                    skip(b, i);
                    if b[*i] == b',' {
                        *i += 1;
                    }
                }
                if open == b'[' {
                    Json::Arr(items)
                } else {
                    Json::Obj(fields)
                }
            }
            other => panic!("unexpected byte {other}"),
        }
    }
    let mut i = 0;
    value(text.as_bytes(), &mut i)
}

fn bits(hex: &str) -> u64 {
    u64::from_str_radix(hex.strip_prefix("0x").unwrap(), 16).unwrap()
}

struct Case {
    id: String,
    terms: Vec<(f64, f64)>,
    absolute_sum: String,
    srss: String,
}

fn table() -> Vec<Case> {
    parse(TABLE)
        .get("cases")
        .arr()
        .iter()
        .map(|case| Case {
            id: case.get("id").str().to_string(),
            terms: case
                .get("terms")
                .arr()
                .iter()
                .map(|t| {
                    let t = t.arr();
                    (
                        f64::from_bits(bits(t[0].str())),
                        f64::from_bits(bits(t[1].str())),
                    )
                })
                .collect(),
            absolute_sum: case.get("absolute_sum").str().to_string(),
            srss: case.get("srss").str().to_string(),
        })
        .collect()
}

fn case(id: &str) -> Case {
    table().into_iter().find(|c| c.id == id).expect(id)
}

fn operands(values: &[f64]) -> Vec<AlgebraOperand> {
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            AlgebraOperand::new(
                format!("t{i}"),
                format!("s{i}"),
                AlgebraQuantity {
                    value: v,
                    dimension: LoadDimension::Force,
                },
                vec![AnalysisStatus::MechanicsSolved],
            )
        })
        .collect()
}

fn terms(factors: &[f64]) -> Vec<CombinationTerm> {
    factors
        .iter()
        .enumerate()
        .map(|(i, &f)| CombinationTerm {
            operand_id: format!("t{i}"),
            factor: f,
        })
        .collect()
}

/// (A, R) through `evaluate_expression`, as bits or the refusal.
fn evaluate(pairs: &[(f64, f64)]) -> (String, String) {
    let values: Vec<f64> = pairs.iter().map(|p| p.1).collect();
    let factors: Vec<f64> = pairs.iter().map(|p| p.0).collect();
    let ops = operands(&values);
    let run = |expression: AlgebraExpression, operation: AlgebraOperation| {
        let result = evaluate_expression(&ops, &expression);
        assert_eq!(result.operation, operation);
        match &result.quantity {
            Some(q) => {
                assert!(result.findings.is_empty());
                assert!(q.value.is_finite());
                assert_eq!(q.dimension, LoadDimension::Force);
                format!("0x{:016x}", q.value.to_bits())
            }
            None => {
                assert_eq!(result.findings.len(), 1, "{:?}", result.findings);
                assert_eq!(result.findings[0].code, FindingCode::NonRepresentable);
                REFUSED.to_string()
            }
        }
    };
    (
        run(
            AlgebraExpression::AbsoluteSum {
                terms: terms(&factors),
            },
            AlgebraOperation::AbsoluteSum,
        ),
        run(
            AlgebraExpression::Srss {
                terms: terms(&factors),
            },
            AlgebraOperation::Srss,
        ),
    )
}

fn four_s(zs: &[f64]) -> ExactAccumulator {
    let mut four = ExactAccumulator::new();
    for &z in zs {
        for _ in 0..4 {
            four.add_product(z, z).unwrap();
        }
    }
    four
}

#[test]
fn v1_every_frozen_row_matches_bit_for_bit() {
    let cases = table();
    assert_eq!(cases.len(), 59);
    let mut refused = [0, 0];
    for case in &cases {
        let (a, r) = evaluate(&case.terms);
        assert_eq!(a, case.absolute_sum, "{} absolute sum", case.id);
        assert_eq!(r, case.srss, "{} srss", case.id);
        refused[0] += usize::from(a == REFUSED);
        refused[1] += usize::from(r == REFUSED);
    }
    assert_eq!(refused, [7, 5]);
}

/// r1 review: at 2⁻¹⁰²², the smallest normal, both gaps are 2⁻¹⁰⁷⁴. Halving the
/// lower gap there puts the lower midpoint at 2⁻¹⁰²² − 2⁻¹⁰⁷⁶, above √S, and
/// the adjustment would step down and back up forever.
#[test]
fn smallest_normal_has_equal_gaps_and_adjustment_stops() {
    let smallest = f64::MIN_POSITIVE;
    let tiny = f64::from_bits(1);
    assert_eq!(upper_gap(smallest), tiny);
    assert_eq!(lower_gap(smallest), tiny);
    assert_eq!(lower_gap(2.0 * smallest), tiny);
    assert_eq!(upper_gap(2.0 * smallest), 2.0 * tiny);
    assert_eq!(lower_gap(1.0), f64::EPSILON / 2.0);
    assert_eq!(upper_gap(f64::MAX), 2f64.powi(971));
    assert_eq!(upper_gap(0.0), tiny);

    let zs = [smallest - tiny, f64::from_bits(1 << 26)]; // 2⁻¹⁰⁴⁸
    let row = case("smallest_normal_gaps");
    assert_eq!(row.terms, vec![(1.0, zs[0]), (1.0, zs[1])]);
    assert_eq!(row.srss, format!("0x{:016x}", smallest.to_bits()));
    let four = four_s(&zs);
    // S is above the midpoint below 2⁻¹⁰²² by 2⁻²¹⁴⁸ − 2⁻²¹⁵⁰, and below the
    // midpoint above it.
    assert_eq!(
        midpoint_sign(&four, zs[0], upper_gap(zs[0]), Midpoint::Above),
        Ok(1)
    );
    assert_eq!(
        midpoint_sign(&four, smallest, lower_gap(smallest), Midpoint::Below),
        Ok(1)
    );
    assert_eq!(
        midpoint_sign(&four, smallest, tiny / 2.0, Midpoint::Below),
        Ok(-1)
    );
    for candidate in [zs[0], smallest, smallest + tiny, smallest + 2.0 * tiny] {
        assert_eq!(adjust(&four, candidate), Ok(smallest), "{candidate:e}");
    }
    assert_eq!(srss_value(&[(1.0, zs[0]), (1.0, zs[1])]), Ok(smallest));
}

#[test]
fn zero_infinite_and_over_max_candidates() {
    // A zero candidate has no lower midpoint; its upper gap is 2⁻¹⁰⁷⁴.
    let three = f64::from_bits(3);
    assert_eq!(adjust(&four_s(&[three]), 0.0), Ok(three));
    assert_eq!(
        adjust(&four_s(&[f64::from_bits(1)]), 0.0),
        Ok(f64::from_bits(1))
    );
    // Infinite and over-MAX candidates are clamped to MAX before adjustment.
    let max = f64::MAX;
    for candidate in [f64::INFINITY, max] {
        assert_eq!(adjust(&four_s(&[max, 1.0]), candidate), Ok(max));
        assert_eq!(
            adjust(&four_s(&[max, max]), candidate),
            Err(SumError::NonRepresentable)
        );
    }
    let below = f64::from_bits(max.to_bits() - 2);
    assert_eq!(adjust(&four_s(&[below]), f64::INFINITY), Ok(below));
}

#[test]
fn overflow_threshold_rows_and_named_cases() {
    // S = (MAX + 2⁹⁷⁰)² − 2⁻²¹⁴⁸, at it and + 2⁻²¹⁴⁸ (V-1); the tie is refused.
    let max = format!("0x{:016x}", f64::MAX.to_bits());
    assert_eq!(case("threshold_minus_quantum").srss, max);
    assert_eq!(case("threshold").srss, REFUSED);
    assert_eq!(case("threshold_plus_quantum").srss, REFUSED);
    assert_eq!(srss_value(&[(1.0, f64::MAX), (1.0, 1.0)]), Ok(f64::MAX));
    assert_eq!(
        srss_value(&[(1.0, f64::MAX), (1.0, f64::MAX)]),
        Err(SumError::NonRepresentable)
    );
    // The threshold row is exactly the midpoint above MAX: the identity at r = MAX.
    let row = case("threshold");
    let zs: Vec<f64> = row.terms.iter().map(|t| t.1).collect();
    let four = four_s(&zs);
    assert_eq!(
        midpoint_sign(&four, f64::MAX, upper_gap(f64::MAX), Midpoint::Above),
        Ok(0)
    );
}

#[test]
fn v2_agrees_with_norm3_on_a_random_sample() {
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut refused = 0;
    let mut subnormal = 0;
    for n in [2usize, 3] {
        for _ in 0..20_000 {
            // One sample in 16 sits at the top binade, where R can overflow.
            let base = if next() % 16 == 0 {
                2046
            } else {
                (next() % 2047) as i64
            };
            let mut zs = [0.0; 3];
            for (i, z) in zs.iter_mut().enumerate().take(n) {
                let spread = match (i, base) {
                    (0, _) => 0,
                    (_, 2046) => (next() % 2) as i64,
                    _ => (next() % 64) as i64,
                };
                let biased = (base - spread).max(0) as u64;
                let sign = next() & (1 << 63);
                *z = f64::from_bits(sign | (biased << 52) | (next() & ((1 << 52) - 1)));
            }
            let pairs: Vec<(f64, f64)> = zs[..n].iter().map(|&z| (1.0, z)).collect();
            let expected = norm3(zs[0], zs[1], zs[2]);
            let got = srss_value(&pairs);
            if expected.is_infinite() {
                assert_eq!(got, Err(SumError::NonRepresentable), "{zs:?}");
                refused += 1;
            } else {
                assert_eq!(got.map(f64::to_bits), Ok(expected.to_bits()), "{zs:?}");
                subnormal += usize::from(expected < f64::MIN_POSITIVE);
            }
        }
    }
    assert!(refused > 0 && subnormal > 0, "{refused} {subnormal}");
}

#[test]
fn v3_every_permutation_of_five_terms_gives_the_same_bits() {
    let row = case("five_terms");
    let expected = (row.absolute_sum.clone(), row.srss.clone());
    let mut order: Vec<usize> = (0..5).collect();
    let mut seen = 0;
    // Heap's algorithm over all 120 orders.
    fn heap(k: usize, order: &mut Vec<usize>, visit: &mut dyn FnMut(&[usize])) {
        if k == 1 {
            visit(order);
            return;
        }
        for i in 0..k {
            heap(k - 1, order, visit);
            let j = if k % 2 == 0 { i } else { 0 };
            order.swap(j, k - 1);
        }
    }
    heap(5, &mut order, &mut |order| {
        let pairs: Vec<(f64, f64)> = order.iter().map(|&i| row.terms[i]).collect();
        assert_eq!(evaluate(&pairs), expected, "{order:?}");
        seen += 1;
    });
    assert_eq!(seen, 120);
}

fn expected_value(hex: &str) -> f64 {
    f64::from_bits(bits(hex))
}

/// V-4 mutant: a binary64 fold in place of the exact sum.
#[test]
fn v4_kills_a_binary64_fold() {
    let row = case("absolute_sum_fold_loses_tail");
    let fold = row.terms.iter().fold(0.0, |s, &(f, x)| s + (f * x).abs());
    assert_ne!(fold.to_bits(), bits(&row.absolute_sum));
    let row = case("fold_loses_tail");
    let fold = row
        .terms
        .iter()
        .fold(0.0, |s, &(f, x)| s + (f * x) * (f * x))
        .sqrt();
    assert_ne!(fold.to_bits(), bits(&row.srss));
    assert_eq!(evaluate(&row.terms).1, row.srss);
}

/// V-4 mutant: a chain of two-term norms. Even a correctly rounded `hypot`
/// (`norm2`) rounds at each link; a libm `hypot` adds its own error on top.
#[test]
fn v4_kills_hypot_chains() {
    for id in [
        "fold_loses_tail",
        "midpoint_even_plus_quantum",
        "pythagorean_tie_down_plus_quantum",
    ] {
        let row = case(id);
        let chain = row.terms.iter().fold(0.0, |r, &(f, x)| norm2(r, f * x));
        assert_ne!(chain.to_bits(), bits(&row.srss), "{id}");
    }
}

/// V-4 mutant: each square rounded to binary64 before the exact sum, the root
/// otherwise exact. Killed by rows whose rounded squares stay normal (the lost
/// low halves move S across a midpoint) and by a subnormal row (they vanish).
#[test]
fn v4_kills_rounding_each_square() {
    for id in [
        "midpoint_even_plus_quantum",
        "midpoint_odd_tie",
        "pythagorean_tie_up",
        "five_terms",
        "subnormal_exact_root",
    ] {
        let row = case(id);
        let zs: Vec<f64> = row.terms.iter().map(|&(f, x)| f * x).collect();
        let largest = zs.iter().fold(0.0_f64, |m, z| m.max(z.abs()));
        let rounded: Vec<(f64, f64)> = zs.iter().map(|z| (z * z, 1.0)).collect();
        let mutant = if rounded.iter().any(|p| !p.0.is_finite()) {
            Err(SumError::NonRepresentable)
        } else {
            rounded_root(&rounded, exponent(largest))
        };
        let expected = if row.srss == REFUSED {
            Err(SumError::NonRepresentable)
        } else {
            Ok(expected_value(&row.srss))
        };
        assert_ne!(mutant, expected, "{id}");
    }
}

/// V-4 mutant: ties away from zero. Each row is an exact tie whose even
/// neighbour is the lower one.
#[test]
fn v4_kills_ties_away_from_zero() {
    let row = case("absolute_sum_tie_even_down");
    let mut tie = ExactAccumulator::new();
    for &(f, x) in &row.terms {
        tie.add_product(f.abs(), x.abs()).unwrap();
    }
    let low = expected_value(&row.absolute_sum);
    tie.add(-low).unwrap();
    tie.add_product(-0.5, upper_gap(low)).unwrap();
    assert!(tie.is_zero() && low.to_bits() & 1 == 0);
    for id in ["midpoint_even_tie", "pythagorean_tie_down"] {
        let row = case(id);
        let zs: Vec<f64> = row.terms.iter().map(|&(f, x)| f * x).collect();
        let low = expected_value(&row.srss);
        let four = four_s(&zs);
        assert_eq!(
            midpoint_sign(&four, low, upper_gap(low), Midpoint::Above),
            Ok(0),
            "{id}"
        );
        assert_eq!(low.to_bits() & 1, 0, "{id}");
    }
}

/// V-4 mutant: −0.0 for a zero result (for example, the sign of a −0.0 term
/// carried through). Every zero row expects +0.0, including rows of −0.0 terms.
#[test]
fn v4_kills_negative_zero_results() {
    let rows = [
        "zero_negative",
        "zero_negative_factor",
        "zero_mixed_signs",
        "z_underflows_to_zero",
        "products_below_half_quantum",
    ];
    for id in rows {
        let row = case(id);
        assert_eq!(
            (row.absolute_sum.as_str(), row.srss.as_str()),
            ("0x0000000000000000", "0x0000000000000000"),
            "{id}"
        );
        assert_eq!(
            evaluate(&row.terms),
            (row.absolute_sum.clone(), row.srss.clone())
        );
    }
    assert!(case("zero_negative").terms[0].1.to_bits() == (-0.0f64).to_bits());
}

#[test]
fn findings_match_the_linear_combination() {
    let ops = operands(&[3.0, 4.0]);
    let run = |terms: Vec<CombinationTerm>| {
        [
            evaluate_expression(
                &ops,
                &AlgebraExpression::AbsoluteSum {
                    terms: terms.clone(),
                },
            ),
            evaluate_expression(&ops, &AlgebraExpression::Srss { terms }),
        ]
    };
    let codes = |r: &AlgebraResult| r.findings.iter().map(|f| f.code).collect::<Vec<_>>();
    for result in run(Vec::new()) {
        assert_eq!(codes(&result), [FindingCode::EmptyExpression]);
        assert_eq!(result.quantity, None);
    }
    for result in run(terms(&[1.0, f64::NAN])) {
        assert_eq!(codes(&result), [FindingCode::NonFiniteFactor]);
    }
    let dup = vec![
        CombinationTerm::new("t0", 1.0).unwrap(),
        CombinationTerm::new("t0", 2.0).unwrap(),
        CombinationTerm::new("missing", 1.0).unwrap(),
    ];
    for result in run(dup) {
        assert_eq!(
            codes(&result),
            [FindingCode::DuplicateOperand, FindingCode::MissingOperand]
        );
        assert_eq!(result.source_operand_ids, vec!["t0"]);
    }
    // Dimension mismatch, missing status, and a non-finite operand value.
    let mut mixed = operands(&[3.0, 4.0, f64::NAN]);
    mixed[1].quantity.dimension = LoadDimension::Moment;
    mixed[0].statuses.clear();
    for expression in [
        AlgebraExpression::AbsoluteSum {
            terms: terms(&[1.0, 1.0, 1.0]),
        },
        AlgebraExpression::Srss {
            terms: terms(&[1.0, 1.0, 1.0]),
        },
    ] {
        let result = evaluate_expression(&mixed, &expression);
        assert_eq!(
            codes(&result),
            [
                FindingCode::StatusBoundaryViolation,
                FindingCode::DimensionMismatch,
                FindingCode::NonRepresentable
            ]
        );
        assert_eq!(result.quantity, None);
    }
    // Success carries the dimension, statuses and source ids.
    for (result, value) in run(terms(&[1.0, -1.0])).into_iter().zip([7.0, 5.0]) {
        assert!(!result.is_blocked());
        assert_eq!(
            result.quantity,
            Some(AlgebraQuantity {
                value,
                dimension: LoadDimension::Force
            })
        );
        assert_eq!(
            result.statuses,
            [
                AnalysisStatus::MechanicsSolved,
                AnalysisStatus::HumanReviewRequired
            ]
        );
        assert_eq!(result.source_operand_ids, vec!["t0", "t1"]);
    }
    // Overflow is a finding, never an infinity.
    let big = operands(&[f64::MAX]);
    for expression in [
        AlgebraExpression::AbsoluteSum {
            terms: terms(&[2.0]),
        },
        AlgebraExpression::Srss {
            terms: terms(&[2.0]),
        },
    ] {
        let result = evaluate_expression(&big, &expression);
        assert_eq!(codes(&result), [FindingCode::NonRepresentable]);
        assert_eq!(result.quantity, None);
    }
}

#[test]
fn more_than_two_to_the_32_terms_are_refused_by_name() {
    assert_eq!(UNSIGNED_COMBINATION_MAX_TERMS, 1 << 32);
    assert!(!exceeds_term_limit(0));
    assert!(!exceeds_term_limit(1 << 32));
    assert!(exceeds_term_limit((1 << 32) + 1));
    assert!(exceeds_term_limit(usize::MAX));
}
