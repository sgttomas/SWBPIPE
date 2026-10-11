//! T4-U1b: tests of the arc load certificate (T4-I9 DESIGN_R01 §6 with
//! T4-RV8's corrections C-3 and C-4). Frozen data in
//! `validation/references/t4_i9/`: `arc_load_references.txt` (RV129's
//! independent 120-digit reference) and `arc_certificate_pins.txt` (the
//! design's executable specification, bit for bit).
use super::*;
use crate::exact_sum::ExactAccumulator;

const REFERENCES: &str =
    include_str!("../../../../../validation/references/t4_i9/arc_load_references.txt");
const PINS: &str =
    include_str!("../../../../../validation/references/t4_i9/arc_certificate_pins.txt");
const ATAN: &str = include_str!("../../tests/retained_wide/atan.txt");

fn hex(token: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(token, 16).expect(token))
}

fn formation(args: &[f64]) -> (CurvedFormation, [f64; 3]) {
    assert_eq!(args.len(), 20);
    (
        CurvedFormation {
            node_i: 0,
            node_j: 1,
            coordinates_i: [args[0], args[1], args[2]],
            coordinates_j: [args[3], args[4], args[5]],
            radius: args[6],
            y_reference: [args[7], args[8], args[9]],
            elastic_modulus: args[10],
            shear_modulus: args[11],
            area: args[12],
            second_moment: args[13],
            torsion_constant: args[14],
            in_plane_flexibility_factor: args[15],
            out_of_plane_flexibility_factor: args[16],
        },
        [args[17], args[18], args[19]],
    )
}

/// T15's body (`s11g_tests.rs` `curved_body` at origin 0): chord (2, 0, 0),
/// R = √2, OD 0.168 m, wall 0.007 m, E 200 GPa, G 80 GPa.
fn t15_body(k: f64) -> CurvedFormation {
    let (ro, ri) = (0.084_f64, 0.084_f64 - 0.007);
    let area = std::f64::consts::PI * (ro * ro - ri * ri);
    let inertia = std::f64::consts::PI * (ro.powi(4) - ri.powi(4)) / 4.0;
    CurvedFormation {
        node_i: 0,
        node_j: 1,
        coordinates_i: [0.0; 3],
        coordinates_j: [2.0, 0.0, 0.0],
        radius: 2.0_f64.sqrt(),
        y_reference: [0.0, 1.0, 0.0],
        elastic_modulus: 200e9,
        shear_modulus: 80e9,
        area,
        second_moment: inertia,
        torsion_constant: 2.0 * inertia,
        in_plane_flexibility_factor: k,
        out_of_plane_flexibility_factor: k,
    }
}

/// DESIGN_R01 §3.5 class of a failure.
fn class_of(failure: &ArcCertificateFailure) -> u8 {
    match failure {
        ArcCertificateFailure::DivisorReachesZero { .. } => 2,
        ArcCertificateFailure::SqrtArgumentNotPositive { .. } => 3,
        ArcCertificateFailure::AtanArgumentNotPositive => 4,
        ArcCertificateFailure::ResidualNotContracting { .. } => 5,
        ArcCertificateFailure::Wide(WideError::DivisionByZero) => 6,
        ArcCertificateFailure::Wide(WideError::SplitOverflow) => 9,
        ArcCertificateFailure::Wide(_) => 7,
        ArcCertificateFailure::RadiusNotFinite { .. } => 8,
    }
}

fn split_terms(m: &Wide2) -> Vec<f64> {
    let split = m.split_binary64().unwrap();
    assert!(!split.truncated_below_min_subnormal());
    split.terms().to_vec()
}

/// The 128-bit value whose exact binary64 split is `terms`.
fn from_terms(terms: &[f64]) -> Wide2 {
    let mut a = WideArith::new(PRECISION).unwrap();
    let mut value = Wide2::ZERO;
    for &t in terms {
        value = a.add(&value, &Wide2::from_f64(t).unwrap()).unwrap();
    }
    assert_eq!(split_terms(&value), terms);
    value
}

/// Sign of Σ a_i·b_i, exactly.
fn sign_of(products: &[(f64, f64)]) -> i8 {
    let mut acc = ExactAccumulator::new();
    for &(a, b) in products {
        acc.add_product(a, b).unwrap();
    }
    acc.signum()
}

/// Exactly: |Σx − Σy| + e ≤ r.
fn within(x: &[f64], y: &[f64], e: f64, r: f64) -> bool {
    let mut diff: Vec<(f64, f64)> = x.iter().map(|&v| (v, 1.0)).collect();
    diff.extend(y.iter().map(|&v| (v, -1.0)));
    let s = f64::from(sign_of(&diff));
    let mut slack: Vec<(f64, f64)> = vec![(r, 1.0), (e, -1.0)];
    slack.extend(diff.iter().map(|&(u, v)| (u, -s * v)));
    sign_of(&slack) >= 0
}

fn max_magnitude(vector: &CertifiedLoadVector) -> f64 {
    vector
        .terms
        .iter()
        .map(|t| t.intended.first().copied().unwrap_or(0.0).abs())
        .fold(0.0_f64, f64::max)
}

fn max_radius(vector: &CertifiedLoadVector) -> f64 {
    vector
        .terms
        .iter()
        .map(|t| t.radius)
        .fold(0.0_f64, f64::max)
}

// ---------------------------------------------------------------------------
// V2c: the directed primitives, checked exactly
// ---------------------------------------------------------------------------

#[test]
fn constants_are_the_specified_values() {
    assert_eq!(U64, 2.0_f64.powi(-128));
    assert_eq!(ONE_UP, 1.0_f64.next_up());
    assert_eq!(K_ATAN, 22.0 * 2.0_f64.powi(-128));
    assert_eq!(TINY, f64::MIN_POSITIVE * 2.0_f64.powi(-52));
    assert_eq!(TINY / 2.0, 0.0);
    // K_ATAN ≥ 21.54u/(1 − 21.54u) ⇔ 22(1 − 21.54u) ≥ 21.54 ⇔ 0.46 ≥ 473.88u.
    assert!(0.46 >= 473.88 * U64);
}

#[test]
fn directed_primitives_are_on_the_right_side() {
    // a2_checks.py's crafted inputs (each under-bounds without its step),
    // subnormal, binade-edge and near-overflow operands.
    let pairs: [(f64, f64); 17] = [
        (0.8287992267179636, 3.7494236685242854e-05),
        (1.363591885540791, 0.00019411731273072204),
        (0.9522637228048398, 0.0006730070574286667),
        (1.4795854905185333, 0.0004178009747299189),
        (1.4077782219387978e-163, 2.5298885888751426e-160),
        (9.724e-320, 0.8573287831355694),
        (TINY, TINY),
        (TINY, 0.5),
        (3.0 * TINY, 1.0 / 3.0),
        (f64::MIN_POSITIVE, 0.75),
        (f64::MIN_POSITIVE.next_down(), 1.0_f64.next_up()),
        (1.0, 1.0_f64.next_down()),
        (2.0_f64.next_down(), 2.0_f64.next_down()),
        (1e300, 1e8),
        (f64::MAX / 4.0, 3.0),
        (1.0 / 3.0, 2.0 / 3.0),
        (0.1, 0.2),
    ];
    let mut checks = 0;
    for &(a, b) in &pairs {
        for (x, y) in [(a, b), (b, a)] {
            let sum = [(x, -1.0), (y, -1.0)];
            let got = add_up(x, y);
            if got.is_finite() {
                assert!(
                    sign_of(&[(got, 1.0), sum[0], sum[1]]) >= 0,
                    "add_up {x:e} {y:e}"
                );
                checks += 1;
            }
            let got = add_dn(x, y);
            assert!(
                sign_of(&[(got, 1.0), sum[0], sum[1]]) <= 0,
                "add_dn {x:e} {y:e}"
            );
            let got = mul_up(x, y);
            if got.is_finite() {
                assert!(sign_of(&[(got, 1.0), (x, -y)]) >= 0, "mul_up {x:e} {y:e}");
                checks += 1;
            }
            let got = mul_dn(x, y);
            assert!(
                got >= 0.0 && sign_of(&[(got, 1.0), (x, -y)]) <= 0,
                "mul_dn {x:e} {y:e}"
            );
            let got = div_up(x, y);
            if got.is_finite() {
                // got ≥ x/y ⇔ got·y − x ≥ 0 (y > 0); exact unless got·y underflows,
                // which ExactAccumulator still represents exactly.
                assert!(sign_of(&[(got, y), (x, -1.0)]) >= 0, "div_up {x:e} {y:e}");
                checks += 1;
            }
            let (hi, lo) = if x >= y { (x, y) } else { (y, x) };
            let got = sub_dn(hi, lo);
            assert!(
                sign_of(&[(got, 1.0), (hi, -1.0), (lo, 1.0)]) <= 0,
                "sub_dn {hi:e} {lo:e}"
            );
            checks += 3;
        }
    }
    // Round to nearest alone under-bounds these (MU6e): the step is needed.
    let (a, b) = (0.8287992267179636, 3.7494236685242854e-05);
    assert!(sign_of(&[(a + b, 1.0), (a, -1.0), (b, -1.0)]) < 0);
    let (a, b) = (1.363591885540791, 0.00019411731273072204);
    assert!(sign_of(&[(a - b, 1.0), (a, -1.0), (b, 1.0)]) > 0);
    let (a, b) = (0.9522637228048398, 0.0006730070574286667);
    assert!(sign_of(&[(a * b, 1.0), (a, -b)]) < 0);
    let (a, b) = (1.4795854905185333, 0.0004178009747299189);
    assert!(sign_of(&[(a / b, b), (a, -1.0)]) < 0);
    // MU6f: the fma-remainder pattern of `load_ledger::product_upward`
    // (without RV129's subnormal guard) under-bounds this subnormal product;
    // mul_up does not.
    let (a, b) = (1.4077782219387978e-163, 2.5298885888751426e-160);
    assert!(sign_of(&[(crate::load_ledger::product_upward(a, b), 1.0), (a, -b)]) < 0);
    assert!(sign_of(&[(mul_up(a, b), 1.0), (a, -b)]) >= 0);
    // Subnormals are not flushed to zero (Lemma R's premise).
    assert!(TINY > 0.0 && f64::MIN_POSITIVE / 4.0 > 0.0 && (3.0 * TINY) * 0.5 > 0.0);
    // Zeros are exact; overflow is +∞ (class 8 at the caller).
    assert_eq!(add_up(0.0, 3.0), 3.0);
    assert_eq!(mul_up(0.0, f64::MAX), 0.0);
    assert_eq!(div_up(0.0, 3.0), 0.0);
    assert_eq!(sub_dn(3.0, 0.0), 3.0);
    assert_eq!(add_up(f64::MAX, f64::MAX), f64::INFINITY);
    assert_eq!(mul_up(f64::MAX, 2.0), f64::INFINITY);
    assert!(checks > 150, "{checks}");
}

#[test]
fn magnitude_bounds_bracket_the_midpoint() {
    let mut a = WideArith::new(PRECISION).unwrap();
    let third = a.div(&Wide2::ONE, &Wide2::from_f64(3.0).unwrap()).unwrap();
    for m in [
        third,
        third.neg(),
        third.mul_pow2(-1060).unwrap(),
        third.mul_pow2(-1100).unwrap(),
        third.mul_pow2(1000).unwrap(),
        Wide2::ONE,
    ] {
        let (dn, up) = (abs_dn(&m).unwrap(), abs_up(&m).unwrap());
        let m_abs = m.abs();
        assert!(Wide2::from_f64(dn).unwrap().cmp_value(&m_abs) != Ordering::Greater);
        assert!(Wide2::from_f64(up).unwrap().cmp_value(&m_abs) == Ordering::Greater);
    }
    let below = third.mul_pow2(-1200).unwrap();
    assert_eq!(
        (abs_dn(&below).unwrap(), abs_up(&below).unwrap()),
        (0.0, TINY)
    );
    assert_eq!(abs_up(&Wide2::ZERO).unwrap(), 0.0);
    let huge = Wide2::ONE.mul_pow2(1024).unwrap();
    assert_eq!(
        abs_up(&huge),
        Err(ArcCertificateFailure::Wide(WideError::SplitOverflow))
    );
}

// ---------------------------------------------------------------------------
// V2d: the lemmas on adversarial inputs
// ---------------------------------------------------------------------------

fn parse_ball(fields: &[&str]) -> (Ball, usize) {
    let n: usize = fields[0].parse().unwrap();
    let terms: Vec<f64> = fields[1..=n].iter().map(|t| hex(t)).collect();
    (
        Ball {
            m: from_terms(&terms),
            r: hex(fields[n + 1]),
        },
        n + 2,
    )
}

type LemmaCase = (String, String, Ball, Option<Ball>, Ball);

fn lemma_cases() -> Vec<LemmaCase> {
    let mut out = Vec::new();
    for line in PINS.lines().filter(|l| l.starts_with("lemma ")) {
        let f: Vec<&str> = line.split_whitespace().collect();
        let (name, op) = (f[1].to_string(), f[2].to_string());
        let (a, used) = parse_ball(&f[3..]);
        let mut at = 3 + used;
        let b = if f[at] == "-" {
            at += 1;
            None
        } else {
            let (b, used) = parse_ball(&f[at..]);
            at += used;
            Some(b)
        };
        assert_eq!(f[at], "->", "{line}");
        let (result, _) = parse_ball(&f[at + 1..]);
        out.push((name, op, a, b, result));
    }
    out
}

fn lemma(name: &str) -> LemmaCase {
    lemma_cases().into_iter().find(|c| c.0 == name).unwrap()
}

fn apply_lemma(op: &str, a: Ball, b: Option<Ball>) -> Result<Ball, ArcCertificateFailure> {
    let mut balls = Balls::new().unwrap();
    match op {
        "add" => balls.add(a, b.unwrap()),
        "sub" => balls.sub(a, b.unwrap()),
        "mul" => balls.mul(a, b.unwrap()),
        "div" => balls.div(a, b.unwrap(), "test"),
        "sqrt" => balls.sqrt(a, "test"),
        "atan" => balls.atan(a),
        "half" => balls.half(a),
        other => panic!("{other}"),
    }
}

/// V2b at lemma level (MU6a, MU6g: one-ulp changes the pins kill).
#[test]
fn lemmas_match_the_specification_bit_for_bit() {
    let cases = lemma_cases();
    assert_eq!(cases.len(), 10);
    for (name, op, a, b, expected) in cases {
        let got = apply_lemma(&op, a, b).unwrap();
        assert_eq!(split_terms(&got.m), split_terms(&expected.m), "{name}");
        assert_eq!(
            got.r.to_bits(),
            expected.r.to_bits(),
            "{name}: {:e} vs {:e}",
            got.r,
            expected.r
        );
    }
}

/// L1/L2 (MU6a, MU6b) hold at every corner of the operand balls, exactly.
#[test]
fn l1_l2_enclose_the_extreme_points() {
    let mut tested = 0;
    for (name, op, a, b, _) in lemma_cases() {
        let Some(b) = b else { continue };
        if !matches!(op.as_str(), "add" | "sub" | "mul") {
            continue;
        }
        let got = apply_lemma(&op, a, Some(b)).unwrap();
        let (am, bm, m) = (split_terms(&a.m), split_terms(&b.m), split_terms(&got.m));
        for sa in [-1.0, 1.0] {
            for sb in [-1.0, 1.0] {
                let mut x = am.clone();
                x.push(sa * a.r);
                let mut y = bm.clone();
                y.push(sb * b.r);
                let mut diff: Vec<(f64, f64)> = match op.as_str() {
                    "add" => x
                        .iter()
                        .map(|&v| (v, 1.0))
                        .chain(y.iter().map(|&v| (v, 1.0)))
                        .collect(),
                    "sub" => x
                        .iter()
                        .map(|&v| (v, 1.0))
                        .chain(y.iter().map(|&v| (v, -1.0)))
                        .collect(),
                    _ => x
                        .iter()
                        .flat_map(|&u| y.iter().map(move |&v| (u, v)))
                        .collect(),
                };
                diff.extend(m.iter().map(|&t| (t, -1.0)));
                let s = f64::from(sign_of(&diff));
                let mut slack = vec![(got.r, 1.0)];
                slack.extend(diff.iter().map(|&(u, v)| (u, -s * v)));
                assert!(sign_of(&slack) >= 0, "{name} corner {sa} {sb}");
                tested += 1;
            }
        }
    }
    assert_eq!(tested, 12);
}

/// MU6c/MU6d: L3's denominator must be |m_b|↓ − r_b. With r_b =
/// (1 − 2⁻⁴⁰)|m_b| the extreme point m_a/(m_b − r_b) is enclosed by the
/// design's radius, and not by the radius from |m_b|↑ or from |m_b| alone.
#[test]
fn l3_encloses_the_near_singular_extreme_point() {
    let (_, _, a, b, _) = lemma("L3_div_near_singular");
    let b = b.unwrap();
    let got = apply_lemma("div", a, Some(b)).unwrap();
    let (am, bm, m) = (split_terms(&a.m), split_terms(&b.m), split_terms(&got.m));
    // |x* − m| ≤ r ⇔ |m_a − m(m_b − r_b)| ≤ r(m_b − r_b), with m_b − r_b > 0.
    let encloses = |r: f64| {
        let mut den = bm.clone();
        den.push(-b.r);
        let mut diff: Vec<(f64, f64)> = am.iter().map(|&v| (v, 1.0)).collect();
        for &t in &m {
            for &d in &den {
                diff.push((t, -d));
            }
        }
        let s = f64::from(sign_of(&diff));
        let mut slack: Vec<(f64, f64)> = den.iter().map(|&d| (r, d)).collect();
        slack.extend(diff.iter().map(|&(u, v)| (u, -s * v)));
        sign_of(&slack) >= 0
    };
    assert!(encloses(got.r));
    let q_up = mul_up(abs_up(&got.m).unwrap(), ONE_UP);
    let numerator = add_up(a.r, mul_up(q_up, b.r));
    let mu6d = add_up(
        div_up(numerator, sub_dn(abs_up(&b.m).unwrap(), b.r)),
        u_term(&got.m).unwrap(),
    );
    assert!(!encloses(mu6d), "MU6d survives");
    let mu6c = add_up(
        div_up(numerator, abs_dn(&b.m).unwrap()),
        u_term(&got.m).unwrap(),
    );
    assert!(!encloses(mu6c), "MU6c survives");
}

/// MU6h: L5's derivative bound must be taken at m_t − r_t. On the wide ball
/// around 1/3 (r_t = 0.3), atan at the low end is enclosed by the design's
/// radius and not by the bound taken at m_t + r_t. atan(lo) is bracketed by
/// `atan_positive` and K3a's proved relative bound (≤ K_ATAN).
#[test]
fn l5_encloses_the_low_end_of_a_wide_ball() {
    let (_, _, t, _, _) = lemma("L5_atan_third_wide");
    let got = apply_lemma("atan", t, None).unwrap();
    let mut a = WideArith::new(PRECISION).unwrap();
    let lo_point = a.sub(&t.m, &Wide2::from_f64(t.r).unwrap()).unwrap();
    let approx = a.atan_positive(&lo_point).unwrap();
    let dist = a.sub(&approx, &got.m).unwrap().abs();
    let rounding = mul_up(ONE_UP, mul_up(U64, abs_up(&dist).unwrap()));
    let slack = add_up(mul_up(K_ATAN, abs_up(&approx).unwrap()), rounding);
    // The point lo_point itself is exact (125 significant bits).
    assert!(
        add_up(abs_up(&dist).unwrap(), slack) <= got.r,
        "design L5 does not enclose"
    );
    let hi = add_up(abs_up(&t.m).unwrap(), t.r);
    let mu6h = add_up(
        div_up(t.r, add_dn(1.0, mul_dn(hi, hi))),
        mul_up(K_ATAN, abs_up(&got.m).unwrap()),
    );
    assert!(
        sub_dn(abs_dn(&dist).unwrap(), slack) > mu6h,
        "MU6h survives"
    );
}

/// N-2 (RV129 O-4): K3a's proved relative bound for `atan_positive`, checked
/// exactly on every committed `atanpos` vector: 100·|atan_positive(t) −
/// (hi + lo)| ≤ 2154·2⁻¹²⁸·(hi + lo).
#[test]
fn atan_positive_relative_bound_on_committed_vectors() {
    let parse = |token: &str| -> Wide2 {
        if token.starts_with('Z') {
            return Wide2::ZERO;
        }
        let negative = token.starts_with('-');
        let (h, e) = token[1..].split_once('p').unwrap();
        let sig = u128::from_str_radix(h, 16).unwrap();
        Wide2::from_parts(
            negative,
            e.parse().unwrap(),
            [sig as u64, (sig >> 64) as u64],
        )
        .unwrap()
    };
    let bound = 2154.0 * U64;
    let mut checked = 0;
    for line in ATAN.lines().filter(|l| l.starts_with("atanpos ")) {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f[4].starts_with("E:") {
            continue;
        }
        let mut a = WideArith::new(PRECISION).unwrap();
        let got = a.atan_positive(&parse(f[3])).unwrap();
        // The inequality is homogeneous: scale all three by 2^-e(hi) (exact)
        // so that their splits are exact.
        let shift = -parse(f[5]).exponent();
        let scaled = |v: Wide2| split_terms(&v.mul_pow2(shift).unwrap());
        let (g, h, l) = (scaled(got), scaled(parse(f[5])), scaled(parse(f[6])));
        let mut d: Vec<(f64, f64)> = g.iter().map(|&v| (v, 1.0)).collect();
        d.extend(h.iter().chain(l.iter()).map(|&v| (v, -1.0)));
        let s = f64::from(sign_of(&d));
        let mut slack: Vec<(f64, f64)> = h.iter().chain(l.iter()).map(|&v| (v, bound)).collect();
        slack.extend(d.iter().map(|&(u, v)| (u, -100.0 * s * v)));
        assert!(sign_of(&slack) >= 0, "{line}");
        checked += 1;
    }
    assert_eq!(checked, 251);
}

// ---------------------------------------------------------------------------
// V2b: the evaluation, bit for bit against the specification
// ---------------------------------------------------------------------------

#[test]
fn matches_the_specification_bit_for_bit() {
    let (mut cases, mut certified) = (0, 0);
    let mut lines = PINS
        .lines()
        .filter(|l| !l.starts_with('#') && !l.starts_with("lemma "));
    while let Some(head) = lines.next() {
        let label = head.strip_prefix("case ").expect(head);
        let args: Vec<f64> = lines.next().unwrap()["args ".len()..]
            .split_whitespace()
            .map(hex)
            .collect();
        let (bend, intensity) = formation(&args);
        let result = certify_curved_uniform_load(&bend, intensity);
        let next = lines.next().unwrap();
        if let Some(err) = next.strip_prefix("err ") {
            let f: Vec<&str> = err.split_whitespace().collect();
            let failure = result.expect_err(label);
            assert_eq!(
                class_of(&failure),
                f[0].parse::<u8>().unwrap(),
                "{label}: {failure:?}"
            );
            if let ArcCertificateFailure::ResidualNotContracting { rho } = failure {
                assert_eq!(rho.to_bits(), hex(f[1]).to_bits(), "{label}");
            }
        } else {
            let vector = result.unwrap_or_else(|e| panic!("{label}: {e:?}"));
            let mut pin = next;
            for (i, term) in vector.terms.iter().enumerate() {
                let f: Vec<&str> = pin.split_whitespace().collect();
                assert_eq!((f[0], f[1].parse::<usize>().unwrap()), ("pin", i));
                let n: usize = f[2].parse().unwrap();
                let expected: Vec<u64> = f[3..3 + n].iter().map(|t| hex(t).to_bits()).collect();
                let got: Vec<u64> = term.intended.iter().map(|t| t.to_bits()).collect();
                assert_eq!(got, expected, "{label} component {i}");
                assert_eq!(
                    term.radius.to_bits(),
                    hex(f[3 + n]).to_bits(),
                    "{label} radius {i}"
                );
                pin = lines.next().unwrap();
            }
            assert_eq!(
                vector.rho.to_bits(),
                hex(&pin["rho ".len()..]).to_bits(),
                "{label} rho"
            );
            certified += 1;
        }
        assert_eq!(lines.next(), Some("end"));
        cases += 1;
    }
    assert_eq!((cases, certified), (25, 17));
}

// ---------------------------------------------------------------------------
// V2: enclosure of RV129's independent reference
// ---------------------------------------------------------------------------

struct ReferenceCase {
    label: String,
    args: Vec<f64>,
    /// Per component: up to four binary64 terms and the generator's error bound.
    refs: Vec<(Vec<f64>, f64)>,
}

fn reference_cases() -> Vec<ReferenceCase> {
    let mut out = Vec::new();
    let mut lines = REFERENCES.lines().filter(|l| !l.starts_with('#'));
    while let Some(head) = lines.next() {
        let label = head.strip_prefix("case ").expect(head).to_string();
        assert!(lines.next().unwrap().starts_with("phi "));
        let args: Vec<f64> = lines.next().unwrap()["args ".len()..]
            .split_whitespace()
            .map(hex)
            .collect();
        let mut refs = Vec::new();
        for i in 0..12 {
            let f: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
            assert_eq!((f[0], f[1].parse::<usize>().unwrap()), ("ref", i));
            refs.push((f[2..6].iter().map(|t| hex(t)).collect(), hex(f[6])));
        }
        assert_eq!(lines.next(), Some("end"));
        out.push(ReferenceCase { label, args, refs });
    }
    out
}

#[test]
fn enclosure_against_frozen_references() {
    let (mut certified, mut refused) = (0, Vec::new());
    let mut worst = 0.0_f64;
    for case in reference_cases() {
        let (bend, intensity) = formation(&case.args);
        match certify_curved_uniform_load(&bend, intensity) {
            Ok(vector) => {
                for (i, (term, (reference, e))) in vector.terms.iter().zip(&case.refs).enumerate() {
                    assert!(
                        within(reference, &term.intended, *e, term.radius),
                        "{}: component {i} not enclosed",
                        case.label
                    );
                    if term.radius > 0.0 {
                        let mut d = ExactAccumulator::new();
                        for &v in reference {
                            d.add(v).unwrap();
                        }
                        for &v in &term.intended {
                            d.add(-v).unwrap();
                        }
                        worst = worst.max((d.round().unwrap().abs() + e) / term.radius);
                    }
                }
                certified += 1;
            }
            Err(failure) => {
                assert_eq!(class_of(&failure), 5, "{}: {failure:?}", case.label);
                refused.push(case.label);
            }
        }
    }
    // Refused naturally by ρ̂ ≥ 1/2 only (DESIGN_R01 §7.1; RV129's extra set).
    assert_eq!(
        refused,
        [
            "k=1e+36 90 deg",
            "k=1e+37 90 deg",
            "k=1e+38 90 deg",
            "k=1e+40 90 deg"
        ]
    );
    assert_eq!(certified, 75);
    println!("V2: {certified} certified and enclosed; worst (|ref - m| + e)/r about {worst:.3e}");
}

/// A-6: near π the sharpened L5 keeps the radius small (RV129's skew chords
/// to π − 6.5e-10; round 00's Lipschitz-1 bound gave 3.7e-6 of max|m|).
#[test]
fn near_pi_radius_stays_small() {
    let mut seen = 0;
    for case in reference_cases()
        .into_iter()
        .filter(|c| c.label.starts_with("near pi"))
    {
        let (bend, intensity) = formation(&case.args);
        let vector = certify_curved_uniform_load(&bend, intensity).unwrap();
        let (radius, scale) = (max_radius(&vector), max_magnitude(&vector));
        assert!(
            radius <= 1e-20 * scale,
            "{}: {radius:e} of {scale:e}",
            case.label
        );
        seen += 1;
    }
    assert_eq!(seen, 4);
}

// ---------------------------------------------------------------------------
// V2e: the nine failed-precondition classes (DESIGN_R01 §3.5)
// ---------------------------------------------------------------------------

/// Class 1 (admissibility) falls to class 3 at its exact edges.
#[test]
fn refuses_inadmissible_arcs() {
    let w = [0.0, 0.0, 0.3];
    let mut bend = t15_body(1.0);
    bend.coordinates_j = bend.coordinates_i;
    assert_eq!(
        certify_curved_uniform_load(&bend, w).unwrap_err(),
        ArcCertificateFailure::SqrtArgumentNotPositive {
            site: "chord length"
        }
    );
    let mut bend = t15_body(1.0);
    bend.radius = 1.0; // d = (2R, 0, 0): q = 0
    assert_eq!(
        certify_curved_uniform_load(&bend, w).unwrap_err(),
        ArcCertificateFailure::SqrtArgumentNotPositive { site: "4R^2 - d.d" }
    );
    let mut bend = t15_body(1.0);
    bend.y_reference = [3.0, 0.0, 0.0]; // y ∥ d: n·n = 0
    assert_eq!(
        certify_curved_uniform_load(&bend, w).unwrap_err(),
        ArcCertificateFailure::SqrtArgumentNotPositive { site: "bow normal" }
    );
    let mut bend = t15_body(1.0);
    bend.radius = 0.5; // R below half the chord: q < 0
    assert_eq!(
        class_of(&certify_curved_uniform_load(&bend, w).unwrap_err()),
        3
    );
}

#[test]
fn l3_refuses_a_divisor_ball_containing_zero() {
    let mut balls = Balls::new().unwrap();
    for divisor in [
        Ball {
            m: Wide2::ONE,
            r: 1.0,
        },
        Ball {
            m: Wide2::ONE,
            r: 2.0,
        },
        Ball {
            m: Wide2::ONE.neg(),
            r: 1.0,
        },
        Ball::ZERO,
        Ball {
            m: Wide2::ZERO,
            r: 1e-300,
        },
    ] {
        assert_eq!(
            balls.div(Ball::ONE, divisor, "probe").unwrap_err(),
            ArcCertificateFailure::DivisorReachesZero { site: "probe" }
        );
    }
    let inside = Ball {
        m: Wide2::ONE,
        r: 1.0_f64.next_down().next_down(),
    };
    assert!(balls.div(Ball::ONE, inside, "probe").is_ok());
}

#[test]
fn refuses_zero_rigidity() {
    let mut bend = t15_body(2.0);
    bend.elastic_modulus = 0.0;
    assert_eq!(
        certify_curved_uniform_load(&bend, [0.0, 0.0, 0.3]).unwrap_err(),
        ArcCertificateFailure::DivisorReachesZero { site: "EI" }
    );
}

#[test]
fn l4_refuses_a_nonpositive_argument_ball() {
    let mut balls = Balls::new().unwrap();
    for arg in [
        Ball::ZERO,
        Ball {
            m: Wide2::ONE.neg(),
            r: 0.0,
        },
        Ball {
            m: Wide2::ONE,
            r: 1.0,
        },
        Ball {
            m: Wide2::ONE,
            r: 1.5,
        },
    ] {
        assert_eq!(
            balls.sqrt(arg, "probe").unwrap_err(),
            ArcCertificateFailure::SqrtArgumentNotPositive { site: "probe" }
        );
    }
    assert!(balls
        .sqrt(
            Ball {
                m: Wide2::ONE,
                r: 0.5
            },
            "probe"
        )
        .is_ok());
}

#[test]
fn l5_refuses_a_nonpositive_argument_ball() {
    let mut balls = Balls::new().unwrap();
    for arg in [
        Ball::ZERO,
        Ball {
            m: Wide2::ONE.neg(),
            r: 0.0,
        },
        Ball {
            m: Wide2::ONE,
            r: 1.0,
        },
        Ball {
            m: Wide2::ONE,
            r: 3.0,
        },
    ] {
        assert_eq!(
            balls.atan(arg).unwrap_err(),
            ArcCertificateFailure::AtanArgumentNotPositive
        );
    }
}

/// Class 5: ρ̂ ∈ [1/2, 1) is refused (kills MU4's "threshold raised to 1").
#[test]
fn l7_refuses_rho_in_half_to_one() {
    let mut balls = Balls::new().unwrap();
    // F = I with radius 0.1 on every entry: X̃ = I and ρ̂ = 6·0.1↑ ∈ [1/2, 1).
    let mut f = [[Ball::ZERO; 6]; 6];
    for (r, row) in f.iter_mut().enumerate() {
        for (k, entry) in row.iter_mut().enumerate() {
            *entry = Ball {
                m: if r == k { Wide2::ONE } else { Wide2::ZERO },
                r: 0.1,
            };
        }
    }
    match invert_verified(&mut balls, &f) {
        Err(ArcCertificateFailure::ResidualNotContracting { rho }) => {
            assert!((0.5..1.0).contains(&rho), "{rho}")
        }
        other => panic!("{other:?}"),
    }
    // ρ̂ < 1/2 certifies; each entry's radius is at least ‖X̃‖∞ρ̂/(1 − ρ̂).
    for row in f.iter_mut() {
        for entry in row.iter_mut() {
            entry.r = 0.05;
        }
    }
    let (inverse, rho) = invert_verified(&mut balls, &f).unwrap();
    assert!((0.29..0.31).contains(&rho), "{rho}");
    assert!(inverse[3][2].r >= rho / (1.0 - rho));
}

#[test]
fn gauss_jordan_refuses_a_singular_midpoint_matrix() {
    let mut a = WideArith::new(PRECISION).unwrap();
    let mut f = [[Wide2::ZERO; 6]; 6];
    for (r, row) in f.iter_mut().enumerate().take(5) {
        row[r] = Wide2::ONE;
    }
    assert_eq!(
        gauss_jordan(&mut a, &f).unwrap_err(),
        ArcCertificateFailure::Wide(WideError::DivisionByZero)
    );
    f[5] = f[0];
    assert_eq!(
        gauss_jordan(&mut a, &f).unwrap_err(),
        ArcCertificateFailure::Wide(WideError::DivisionByZero)
    );
}

#[test]
fn maps_wide_errors() {
    let mut bend = t15_body(2.0);
    bend.coordinates_i[1] = f64::NAN;
    assert_eq!(
        certify_curved_uniform_load(&bend, [0.0, 0.0, 0.3]).unwrap_err(),
        ArcCertificateFailure::Wide(WideError::NonFinite)
    );
    assert_eq!(
        certify_curved_uniform_load(&t15_body(2.0), [0.0, 0.0, f64::INFINITY]).unwrap_err(),
        ArcCertificateFailure::Wide(WideError::NonFinite)
    );
}

/// Class 8: a radius that overflows.
#[test]
fn refuses_a_non_finite_radius() {
    let mut balls = Balls::new().unwrap();
    let big = Ball {
        m: Wide2::ONE,
        r: f64::MAX,
    };
    assert_eq!(
        balls.add(big, big).unwrap_err(),
        ArcCertificateFailure::RadiusNotFinite { site: "L1" }
    );
    let four = Ball::exact(Wide2::from_f64(4.0).unwrap());
    assert_eq!(
        balls.mul(big, four).unwrap_err(),
        ArcCertificateFailure::RadiusNotFinite { site: "L2" }
    );
}

/// Class 9 (T4-RV8 C-3: an intensity of 1e308, not 1e300).
#[test]
fn split_overflow_refuses() {
    assert_eq!(
        certify_curved_uniform_load(&t15_body(2.0), [0.0, 0.0, 1e308]).unwrap_err(),
        ArcCertificateFailure::Wide(WideError::SplitOverflow)
    );
    assert!(certify_curved_uniform_load(&t15_body(2.0), [0.0, 0.0, 1e300]).is_ok());
    let mut a = WideArith::new(PRECISION).unwrap();
    let third = a.div(&Wide2::ONE, &Wide2::from_f64(3.0).unwrap()).unwrap();
    let huge = Ball::exact(third.mul_pow2(1027).unwrap());
    assert_eq!(
        finish_component(&huge, true).unwrap_err(),
        ArcCertificateFailure::Wide(WideError::SplitOverflow)
    );
}

/// The output rule: a split truncated below 2⁻¹⁰⁷⁴ adds 2⁻¹⁰⁷⁴ to r̂, and
/// the truncated terms stay enclosed.
#[test]
fn truncated_split_adds_the_smallest_subnormal() {
    let mut a = WideArith::new(PRECISION).unwrap();
    let third = a.div(&Wide2::ONE, &Wide2::from_f64(3.0).unwrap()).unwrap();
    let m = third.mul_pow2(-1040).unwrap();
    assert!(m.split_binary64().unwrap().truncated_below_min_subnormal());
    for r in [0.0, 1e-320, 1e-300] {
        let term = finish_component(&Ball { m, r }, true).unwrap();
        assert_eq!(term.radius.to_bits(), add_up(r, TINY).to_bits());
        // |m − Σt| < 2⁻¹⁰⁷⁴ ≤ r̂: compare at 128 bits (exact here).
        let rest = a.sub(&m, &from_terms(&term.intended)).unwrap().abs();
        assert!(rest.cmp_value(&Wide2::from_f64(term.radius).unwrap()) == Ordering::Less);
    }
    let exact = finish_component(&Ball { m: third, r: 1e-30 }, true).unwrap();
    assert_eq!(exact.radius, 1e-30);
    // In the evaluation: T15's body at w = 1e-300 truncates (V2b pins it).
    let vector = certify_curved_uniform_load(&t15_body(2.0), [0.0, 0.0, 1e-300]).unwrap();
    assert!(vector.terms.iter().all(|t| t.radius >= TINY));
}

// ---------------------------------------------------------------------------
// The operand bound (Theorem 2) and C-4
// ---------------------------------------------------------------------------

/// MU9: the generated-load γ₅ term, bit for bit.
#[test]
fn generated_operand_bound_bits() {
    let term = CertifiedLoadTerm {
        intended: vec![1.0],
        radius: 1e-30,
        magnitude_up: 1.0_f64.next_up(),
        single_component: true,
    };
    assert_eq!(term.operand_bound(false), 1e-30);
    let expected = add_up(1e-30, mul_up(gamma(5), add_up(1.0_f64.next_up(), 1e-30)));
    assert_eq!(term.operand_bound(true).to_bits(), expected.to_bits());
    assert!(term.operand_bound(true) > 5.0 * 2.0_f64.powi(-53));
    let multi = CertifiedLoadTerm {
        single_component: false,
        ..term.clone()
    };
    assert_eq!(multi.operand_bound(false), 1e-30);
    assert_eq!(multi.operand_bound(true), f64::INFINITY);
    // C-4: a zero intensity has at most one nonzero component; f = 0, so a
    // generated load's bound is 0, not +∞. Two nonzero components are not.
    let zero = certify_curved_uniform_load(&t15_body(2.0), [0.0; 3]).unwrap();
    for t in &zero.terms {
        assert!(t.single_component && t.intended.is_empty());
        assert_eq!(t.operand_bound(true), 0.0);
    }
    let two = certify_curved_uniform_load(&t15_body(2.0), [0.1, 0.0, 0.3]).unwrap();
    assert!(two.terms.iter().all(|t| !t.single_component));
}

/// A-5 (T15d's premise): at k = 1e10 on T15's body the radius is small (the
/// design's emulation: 5e-16 of max|m|); and T15c's window (§7.1): refused at
/// k = 1e36 with ρ̂ ∈ [1/2, 1).
#[test]
fn large_flexibility_certifies_with_a_small_radius() {
    let vector = certify_curved_uniform_load(&t15_body(1e10), [0.0, 0.0, 0.3]).unwrap();
    let (radius, scale) = (max_radius(&vector), max_magnitude(&vector));
    assert!(radius <= 1e-14 * scale, "{radius:e} {scale:e}");
    match certify_curved_uniform_load(&t15_body(1e36), [0.0, 0.0, 0.3]) {
        Err(ArcCertificateFailure::ResidualNotContracting { rho }) => {
            assert!((0.5..1.0).contains(&rho), "{rho}")
        }
        other => panic!("{other:?}"),
    }
}
