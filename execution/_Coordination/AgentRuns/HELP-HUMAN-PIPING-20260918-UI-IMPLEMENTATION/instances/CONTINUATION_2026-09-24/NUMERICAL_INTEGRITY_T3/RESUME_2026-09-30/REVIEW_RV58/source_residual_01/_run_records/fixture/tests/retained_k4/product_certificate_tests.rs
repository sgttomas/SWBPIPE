//! Frozen independent Fraction targets and actual producing work custody.
use super::*;
#[path = "product_certificate_vectors.rs"]
mod vectors;

fn parse(token: &str) -> Endpoint {
    if token == "Z+" {
        return Endpoint::ZERO;
    }
    let (hex, e) = token[1..].split_once('p').unwrap();
    let digits = format!("{hex:0<256}");
    let mut limbs = [0; 16];
    for (i, v) in limbs.iter_mut().rev().enumerate() {
        *v = u64::from_str_radix(&digits[i * 16..i * 16 + 16], 16).unwrap();
    }
    Endpoint::from_parts(token.starts_with('-'), e.parse().unwrap(), limbs).unwrap()
}
fn input(mode: u8, geometry: [u64; 2], m: [u64; 9], k: [u64; 7]) -> MemberOperands {
    let m = m.map(f64::from_bits);
    let k = k.map(f64::from_bits);
    let material = match mode {
        0 => MaterialOperands::ExactENu { e: m[0], nu: m[1] },
        1 => MaterialOperands::Ordinary { e: m[0], g: m[1] },
        2 => MaterialOperands::Interpolated {
            t_lo: m[0],
            t: m[1],
            t_hi: m[2],
            e_lo: m[3],
            e_hi: m[4],
            g_lo: m[5],
            g_hi: m[6],
            e_hat: m[7],
            g_hat: m[8],
        },
        _ => panic!("fixture mode"),
    };
    MemberOperands {
        diameter: f64::from_bits(geometry[0]),
        effective_wall: f64::from_bits(geometry[1]),
        material,
        admitted: AdmittedOperands {
            e: k[0],
            g: k[1],
            a: k[2],
            j: k[3],
            iz: k[4],
            iy: k[5],
            z_hat: k[6],
        },
    }
}
fn ordinary() -> MemberOperands {
    let (_, mode, geometry, m, k, _, _) = vectors::CASES[1];
    input(mode, geometry, m, k)
}
fn covers(actual: Enclosure, reference: (&str, &str), label: &str) {
    assert!(
        actual.lo.cmp_value(&parse(reference.0)) != Ordering::Greater,
        "lower {label}"
    );
    assert!(
        actual.hi.cmp_value(&parse(reference.1)) != Ordering::Less,
        "upper {label}"
    );
}
#[test]
fn product_certificate_fraction_geometry_material_and_coefficients() {
    assert_eq!(vectors::CASES.len(), 8);
    for &(name, mode, geometry, m, k, reference, products) in vectors::CASES {
        let input = input(mode, geometry, m, k);
        let spent = member_coefficients(&input);
        assert!(spent.work().status().is_exact(), "{name}: {spent:?}");
        let v = spent.result().unwrap();
        assert_eq!(v.effective_wall_bits, geometry[1]);
        let intervals = [
            Enclosure::point(v.c),
            v.ri,
            v.p,
            v.q,
            v.geometry_g,
            v.a,
            v.i,
            v.j,
            v.z,
            v.e,
            v.g,
            v.coefficients[0],
            v.coefficients[1],
            v.coefficients[2],
            v.coefficients[3],
            v.represented_z.unwrap_or(Enclosure::point(Endpoint::ZERO)),
        ];
        for (a, r) in intervals.into_iter().zip(reference.iter().copied()) {
            covers(a, r, name);
        }
        for j in 0..4 {
            assert_eq!(
                v.admitted_products[j],
                parse(products[j]),
                "exact K {name} {j}"
            );
            assert!(
                v.coefficient_differences[j].cmp_value(&parse(products[j + 4])) != Ordering::Less,
                "delta {name} {j}"
            );
        }
        let entries = spent.work.entries.map(|n| n.exact().unwrap());
        assert_eq!(entries[Entry::R4 as usize], if mode == 2 { 4 } else { 0 });
        assert_eq!(entries[Entry::B64U as usize], 0);
        assert_eq!(spent.work.f64_arithmetic.exact(), Ok(0));
        // An entered DM with an exact zero operand has no TwoProduct core
        // call. Count the actual prefix, not a one-core-call-per-entry envelope.
        let m = m.map(f64::from_bits);
        let zero_products = if mode == 2 {
            [
                (m[2], m[3]),
                (m[1], m[3]),
                (m[1], m[4]),
                (m[0], m[4]),
                (m[2], m[5]),
                (m[1], m[5]),
                (m[1], m[6]),
                (m[0], m[6]),
            ]
            .iter()
            .filter(|(a, b)| *a == 0.0 || *b == 0.0)
            .count() as u64
        } else {
            0
        };
        assert_eq!(
            spent.work.wide.width::<16>().two_product,
            entries[Entry::Mul as usize] + entries[Entry::Div as usize] - zero_products,
            "actual TwoProduct calls {name}"
        );
        assert!(spent.work.wide.checked_lme().exact().unwrap() > 0);
        assert!(spent.work.sums.checked_lme().exact().unwrap() > 0);
    }
}
#[test]
fn product_certificate_exact_k_retains_missing_binary64_tail() {
    let x = f64::from_bits(1.0f64.to_bits() + 1);
    let mut work = NumericWork::new();
    let exact = work.exact_product(x, x).unwrap();
    let rounded = lift(x * x).unwrap();
    let delta = work.absdiff(&exact, &rounded).unwrap();
    assert_eq!(delta, shift(&Endpoint::ONE, -104).unwrap());
    assert_ne!(exact, rounded);
}
#[test]
fn product_certificate_exact_zero_and_positive_subnormal_source() {
    let mut work = NumericWork::new();
    assert_eq!(
        work.scalar(Entry::Sub, &Endpoint::ONE, &Endpoint::ONE, Toward::Up)
            .unwrap(),
        Endpoint::ZERO
    );
    assert_eq!(lift(-0.0).unwrap().parts().0, false);
    let (_, mode, g, m, k, _, _) = vectors::CASES[4];
    let spent = member_coefficients(&input(mode, g, m, k));
    let v = spent.result().unwrap();
    assert!(positive(&v.i.lo));
    assert_eq!(v.i.lo.to_binary64().value().unwrap_or(0.0), 0.0);
    // Positive wide source geometry survives even where a binary64 property
    // would be zero. This abstract input does not prove product reachability.
    assert!(v.i.lo.exponent() < -1074);
}
#[test]
fn product_certificate_source_sign_precedes_positive_rounded_material() {
    let mut x = ordinary();
    x.material = MaterialOperands::Interpolated {
        t_lo: 0.,
        t: 7.,
        t_hi: 25.,
        e_lo: -7.,
        e_hi: 18.,
        g_lo: 1.,
        g_hi: 1.,
        e_hat: 2f64.powi(-50),
        g_hat: 1.,
    };
    x.admitted.e = 2f64.powi(-50);
    x.admitted.g = 1.;
    let spent = member_coefficients(&x);
    assert!(matches!(
        spent.result(),
        Err(NumericError::NonpositiveSource)
    ));
    assert!(spent.work.status().is_exact());
    assert_eq!(spent.work.entries[Entry::R4 as usize].exact(), Ok(1));
    assert_eq!(spent.work.entries[Entry::Mul as usize].exact(), Ok(4));
    assert!(spent.work.sums.checked_lme().exact().unwrap() > 0);
}
#[test]
fn product_certificate_refusals_and_ordinary_independent_g() {
    let mut x = ordinary();
    x.material = MaterialOperands::Ordinary { e: 2., g: 7. };
    x.admitted.e = 2.;
    x.admitted.g = 7.;
    let spent = member_coefficients(&x);
    assert_eq!(spent.result().unwrap().g.lo, lift(7.).unwrap());
    x.admitted.g = 6.;
    assert!(matches!(
        member_coefficients(&x).result(),
        Err(NumericError::MaterialBits)
    ));
    let mut x = ordinary();
    x.effective_wall = x.diameter / 2.;
    assert!(matches!(
        member_coefficients(&x).result(),
        Err(NumericError::InvalidGeometry)
    ));
    let mut x = ordinary();
    x.admitted.iy = 2.;
    assert!(matches!(
        member_coefficients(&x).result(),
        Err(NumericError::AxisBits)
    ));
    let mut x = ordinary();
    x.diameter = f64::NAN;
    assert!(matches!(
        member_coefficients(&x).result(),
        Err(NumericError::NonFinite)
    ));
    let mut x = ordinary();
    x.material = MaterialOperands::ExactENu {
        e: x.admitted.e,
        nu: -1.,
    };
    assert!(matches!(
        member_coefficients(&x).result(),
        Err(NumericError::InvalidMaterial)
    ));
    let mut x = ordinary();
    x.material = MaterialOperands::Interpolated {
        t_lo: 0.,
        t: 0.,
        t_hi: 1.,
        e_lo: 1.,
        e_hi: 1.,
        g_lo: 1.,
        g_hi: 1.,
        e_hat: 1.,
        g_hat: 1.,
    };
    assert!(matches!(
        member_coefficients(&x).result(),
        Err(NumericError::TemperatureOrder)
    ));
    let mut work = NumericWork::new();
    assert_eq!(
        work.scalar(Entry::Div, &Endpoint::ONE, &Endpoint::ZERO, Toward::Up),
        Err(NumericError::NonpositiveDenominator)
    );
    assert_eq!(work.entries[Entry::Div as usize].exact(), Ok(1));
    assert_eq!(work.wide.checked_lme().exact(), Ok(0));
}
#[test]
fn product_certificate_scalar_and_r4_failure_prefixes() {
    let mut work = NumericWork::new();
    let huge = shift(&Endpoint::ONE, 9000).unwrap();
    assert!(work
        .scalar(Entry::Add, &huge, &Endpoint::ONE, Toward::Up)
        .is_err());
    assert_eq!(work.entries[Entry::Add as usize].exact(), Ok(1));
    assert!(work.sums.checked_lme().exact().unwrap() > 0);
    let mut sum = ExactWideSum::new();
    sum.add_binary64(1., false).unwrap();
    sum.signum().unwrap();
    sum.test_seed_term_work(u64::MAX);
    let mut work = NumericWork::new();
    let result = scalar_owned(
        &mut work,
        Entry::Div,
        &Endpoint::ONE,
        &Endpoint::ZERO,
        Toward::Up,
        WideContext::<16>::new(1024).unwrap(),
        sum,
    );
    assert_eq!(result, Err(NumericError::NonpositiveDenominator));
    // Numeric refusal and failed aggregate work extraction coexist.
    assert!(!work.status().is_exact());
    assert_eq!(
        work.checked(result),
        Err(NumericError::NonpositiveDenominator)
    );
    let mut sum = ExactWideSum::new();
    sum.test_seed_term_work(u64::MAX - 2);
    let mut work = NumericWork::new();
    let p = [
        lift(3.).unwrap(),
        Endpoint::ONE,
        lift(4.).unwrap(),
        Endpoint::ONE,
    ];
    assert!(r4_owned(
        &mut work,
        &p,
        Toward::Down,
        WideContext::<16>::new(1024).unwrap(),
        sum
    )
    .is_err());
    assert!(!work.status().is_exact());
    assert!(work.sums.term_limbs >= u64::MAX - 2);
}
#[test]
fn product_certificate_checked_merge_blocks_success_and_keeps_numeric_cause() {
    let mut work = NumericWork::new();
    let mut seeded = ExactWideSum::new();
    seeded.test_seed_term_work(u64::MAX);
    work.sums = seeded.work();
    let result = scalar_owned(
        &mut work,
        Entry::Add,
        &Endpoint::ONE,
        &Endpoint::ONE,
        Toward::Up,
        WideContext::<16>::new(1024).unwrap(),
        ExactWideSum::new(),
    );
    assert!(matches!(
        result,
        Err(NumericError::Arithmetic(AttemptStop::WorkAccounting(_)))
    ));
    assert!(!work.status().is_exact());
    let cause = Err::<(), _>(NumericError::InvalidGeometry);
    assert_eq!(work.checked(cause.clone()), cause);
}

#[test]
fn product_certificate_b64u_zero_normal_successor_underflow_and_overflow() {
    let one = Endpoint::ONE;
    let mut work = NumericWork::new();
    let half_ulp = shift(&one, -54).unwrap();
    let successor_input = work
        .scalar(Entry::Add, &one, &half_ulp, Toward::Up)
        .unwrap();
    let max = lift(f64::MAX).unwrap();
    let small = shift(&one, 969).unwrap();
    let max_successor = work.scalar(Entry::Add, &max, &small, Toward::Up).unwrap();
    for (x, expected, early) in [
        (Endpoint::ZERO, 0., false),
        (one, 1., false),
        (successor_input, f64::from_bits(1.0f64.to_bits() + 1), false),
        (shift(&one, -1075).unwrap(), f64::from_bits(1), false),
        (shift(&one, -1074).unwrap(), f64::from_bits(1), false),
        (max_successor, f64::INFINITY, false),
        (shift(&one, 1024).unwrap(), f64::INFINITY, true),
    ] {
        let (result, sums) = directed::binary64_up_spent(&x).into_parts();
        assert_eq!(result.as_ref().unwrap().to_bits(), expected.to_bits());
        assert_eq!(
            directed::binary64_up(&x).unwrap().to_bits(),
            expected.to_bits()
        );
        assert!(sums.checked_lme().status().is_exact());
        if early || x.is_zero() {
            assert_eq!(sums.checked_lme().exact(), Ok(0));
        } else {
            assert!(sums.checked_lme().exact().unwrap() > 0);
        }
        let mut collector = NumericWork::new();
        let collected = collector.binary64_up(&x);
        assert_eq!(collector.entries[Entry::B64U as usize].exact(), Ok(1));
        assert_eq!(collector.wide.checked_lme().exact(), Ok(0));
        assert_eq!(collector.sums, sums);
        if expected.is_infinite() {
            assert_eq!(collected, Err(NumericError::Binary64Range));
        } else {
            assert_eq!(collected.unwrap().to_bits(), expected.to_bits());
        }
    }
}
#[test]
fn product_certificate_b64u_each_refusal_prefix_and_collection_status() {
    let x = lift(1.).unwrap();
    let mut first = ExactWideSum::new();
    first.add_wide(&x, false).unwrap();
    let first_work = first.work();
    first.add_binary64(1., true).unwrap();
    let two_work = first.work();
    let prefixes = [
        (0, 0),
        (first_work.checked_lme().exact().unwrap(), 1),
        (two_work.checked_lme().exact().unwrap(), 2),
    ];
    for (budget, stage) in prefixes {
        let mut sum = ExactWideSum::new();
        sum.test_seed_term_work(u64::MAX - budget);
        let (result, sums) = directed::test_binary64_up_owned(&x, 1., sum).into_parts();
        assert!(result.is_err(), "stage {stage}");
        assert!(!sums.checked_lme().status().is_exact());
        assert!(sums.term_limbs >= u64::MAX - budget);
        if stage == 0 {
            assert_eq!(sums.term_limbs, u64::MAX);
        }
        if stage == 1 {
            assert_eq!(sums.term_limbs, u64::MAX - budget + first_work.term_limbs);
        }
        if stage == 2 {
            assert_eq!(sums.term_limbs, u64::MAX - budget + two_work.term_limbs);
        }
        let mut collector = NumericWork::new();
        let saved = result.clone();
        let collected = collector.collect_binary64(result, sums);
        assert_eq!(collected, saved.map_err(NumericError::Arithmetic));
        assert!(!collector.status().is_exact());
    }
    let mut collector = NumericWork::new();
    let mut sum = ExactWideSum::new();
    sum.test_seed_term_work(u64::MAX);
    collector.sums = sum.work();
    let (result, sums) = directed::binary64_up_spent(&x).into_parts();
    assert!(matches!(
        collector.collect_binary64(result, sums),
        Err(NumericError::Arithmetic(AttemptStop::WorkAccounting(_)))
    ));
    assert!(!collector.status().is_exact());
    assert_eq!(
        collector.collect_binary64(Err(AttemptStop::Exponent), SumWork::default()),
        Err(NumericError::Arithmetic(AttemptStop::Exponent))
    );
}
