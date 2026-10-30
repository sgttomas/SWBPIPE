//! Independent exact fixtures and spent-work custody for the two private helpers.
use super::super::super::wide::WideError;
use super::*;

#[path = "certificate_arithmetic_vectors.rs"]
mod vectors;

fn parse(token: &str) -> Wide<16> {
    if token == "Z+" {
        return Wide::<16>::ZERO;
    }
    if token == "Z-" {
        return Wide::<16>::ZERO.neg();
    }
    let (hex, exponent) = token[1..].split_once('p').unwrap();
    let mut digits = hex.to_owned();
    while digits.len() < 256 {
        digits.push('0');
    }
    let mut limbs = [0u64; 16];
    for (i, limb) in limbs.iter_mut().rev().enumerate() {
        *limb = u64::from_str_radix(&digits[16 * i..16 * i + 16], 16).unwrap();
    }
    Wide::<16>::from_parts(token.starts_with('-'), exponent.parse().unwrap(), limbs).unwrap()
}

fn token(value: &Wide<16>) -> String {
    let (negative, exponent, limbs) = value.parts();
    if value.is_zero() {
        return if negative { "Z-" } else { "Z+" }.to_owned();
    }
    let hex: String = limbs.iter().rev().map(|n| format!("{n:016x}")).collect();
    format!(
        "{}{}p{exponent}",
        if negative { '-' } else { '+' },
        hex.trim_end_matches('0')
    )
}

#[test]
fn certificate_sqrt_exact_vectors_both_directions_and_adjacent_neighbors() {
    assert_eq!(vectors::SQRT.len(), 14);
    for &(name, input, down, up, nearest) in vectors::SQRT {
        let a = parse(input);
        let mut ctx = WideContext::<16>::new(1024).unwrap();
        assert_eq!(token(&ctx.sqrt(&a).unwrap()), nearest, "nearest {name}");
        for (direction, expected) in [(Toward::Down, down), (Toward::Up, up)] {
            let spent = sqrt_endpoint(&a, direction);
            assert!(spent.status().is_exact(), "{name}: {spent:?}");
            assert_eq!(
                token(spent.result().unwrap()),
                expected,
                "{name} {direction:?}"
            );
            let work = spent.work().width::<16>();
            assert_eq!(work.sqrt, 1);
            assert_eq!(work.two_product, 1);
            assert_eq!(work.round, u64::from(expected != nearest));
            assert_eq!(
                (work.add, work.sub, work.mul, work.div, work.two_sum),
                (0, 0, 0, 0, 0)
            );
            assert!(spent.work().checked_lme().exact().unwrap() <= 18_820);
            assert!(spent.sum_work().checked_lme().exact().unwrap() < 4096);
            assert_eq!((spent.comparisons(), spent.f64_operations()), (0, 0));
        }
        // These independently generated endpoints are equal or adjacent in
        // the fixed-width format, including a power-of-two binade crossing.
        if down != up {
            let low = parse(down);
            let mut sum = ExactWideSum::new();
            assert_eq!(
                token(&step(&mut ctx, &mut sum, &low, Toward::Up).unwrap()),
                up
            );
        }
    }
}

#[test]
fn certificate_sqrt_zero_negative_and_intermediate_range_prefixes() {
    for zero in ["Z+", "Z-"] {
        for direction in [Toward::Down, Toward::Up] {
            let spent = sqrt_endpoint(&parse(zero), direction);
            assert_eq!(token(spent.result().unwrap()), "Z+");
            assert_eq!(spent.work().width::<16>().sqrt, 1);
            assert_eq!(spent.work().width::<16>().two_product, 0);
            assert_eq!(spent.sum_work().checked_lme().exact(), Ok(0));
        }
    }
    let negative = sqrt_endpoint(&parse("-8p0"), Toward::Up);
    assert_eq!(
        negative.result(),
        Err(HelperError::Arithmetic(AttemptStop::Arithmetic(
            WideError::NegativeSqrt
        )))
    );
    assert!(negative.status().is_exact());
    assert_eq!(negative.work().width::<16>().sqrt, 1);
    assert_eq!(negative.sum_work().checked_lme().exact(), Ok(0));

    let mut limbs = [0u64; 16];
    limbs[15] = 1 << 63;
    // Irrational sqrt at the minimum odd exponent: its exact q*q residual
    // has an exponent below -LIMIT, even though q itself is representable.
    let a = Wide::<16>::from_parts(false, -(1i64 << 62) + 1, limbs).unwrap();
    for direction in [Toward::Down, Toward::Up] {
        let spent = sqrt_endpoint(&a, direction);
        assert_eq!(
            spent.result(),
            Err(HelperError::Arithmetic(AttemptStop::Exponent))
        );
        assert!(spent.status().is_exact());
        assert_eq!(spent.work().width::<16>().sqrt, 1);
        assert_eq!(spent.work().width::<16>().two_product, 1);
    }
}

#[test]
fn certificate_small_bound_exact_quantum_vectors_and_existing_bits() {
    assert_eq!(vectors::SMALL.len(), 18);
    for &(name, value, scale, b0, r, expected) in vectors::SMALL {
        let value = f64::from_bits(value);
        let scale = f64::from_bits(scale);
        let spent = small_row_bound(value, scale);
        assert!(spent.status().is_exact(), "{name}: {spent:?}");
        assert_eq!(spent.result().unwrap().to_bits(), expected, "{name}");
        assert_eq!(
            super::super::super::adaptive::row_bound(value, scale).to_bits(),
            expected,
            "compatibility {name}"
        );
        assert_eq!(spent.f64_operations(), 4);
        assert!((2..=64).contains(&spent.comparisons()));
        assert_eq!(spent.work().checked_lme().exact(), Ok(0));
        assert!(
            spent.sum_work().checked_lme().exact().unwrap() < u64::from(spent.comparisons()) * 4096
        );
        // Sum work from every candidate is retained, not only the final sum.
        assert!(spent.sum_work().term_limbs >= u64::from(spent.comparisons()) * 3);
        let mut exact = ExactWideSum::new();
        assert!(
            bound_comparison(
                &mut exact,
                f64::from_bits(b0),
                f64::from_bits(r),
                f64::from_bits(expected)
            )
            .unwrap()
                <= 0
        );
        let mut below = ExactWideSum::new();
        assert!(
            bound_comparison(
                &mut below,
                f64::from_bits(b0),
                f64::from_bits(r),
                f64::from_bits(expected - 1)
            )
            .unwrap()
                > 0
        );
    }
}

#[test]
fn certificate_small_bound_double_rounding_discriminator() {
    let scale = f64::from_bits(1);
    let spent = small_row_bound(16.0, scale);
    let mut ctx = WideContext::<16>::new(1024).unwrap();
    let mut sum = ExactWideSum::new();
    sum.add_binary64(f64::from_bits(1), false).unwrap();
    sum.add_binary64(16.0 / 9_007_199_254_740_992.0, false)
        .unwrap();
    sum.add_binary64(f64::from_bits(1), false).unwrap();
    let nearest1024 = sum.round(&mut ctx).unwrap();
    let lost_tail = super::super::binary64_up(&nearest1024).unwrap();
    assert_eq!(spent.result().unwrap().to_bits(), lost_tail.to_bits() + 1);
}

#[test]
fn certificate_invalid_inputs_have_no_arithmetic_or_search_work() {
    for (value, scale) in [
        (f64::NAN, 1.0),
        (f64::INFINITY, f64::from_bits(1)),
        (f64::NEG_INFINITY, f64::from_bits(1)),
        (0.0, 0.0),
        (0.0, -0.0),
        (0.0, -f64::from_bits(1)),
        (0.0, f64::NAN),
        (0.0, f64::INFINITY),
        (0.0, f64::from_bits(0x0230_0000_0000_0000)),
        (0.0, 1.0),
    ] {
        let spent = small_row_bound(value, scale);
        assert_eq!(spent.result(), Err(HelperError::InvalidSmallBoundInput));
        assert!(spent.status().is_exact());
        assert_eq!((spent.comparisons(), spent.f64_operations()), (0, 0));
        assert_eq!(spent.work().checked_lme().exact(), Ok(0));
        assert_eq!(spent.sum_work().checked_lme().exact(), Ok(0));
    }
}

#[test]
fn certificate_failed_comparison_retains_partial_sum_and_counts() {
    let mut sum = ExactWideSum::new();
    sum.test_seed_term_work(u64::MAX - 2);
    let mut spent = EntrySpent::new(Ok(1.0));
    spent.result = spent
        .compare_owned(1.0, 2.0, 3.0f64.to_bits(), sum)
        .map(|_| 1.0);
    assert_eq!(spent.comparisons(), 1);
    assert!(!spent.status().is_exact());
    assert!(matches!(
        spent.result(),
        Err(HelperError::Arithmetic(AttemptStop::WorkAccounting(_)))
    ));
    assert!(spent.sum_work().term_limbs > u64::MAX - 2);
    assert!(spent.sum_work().checked_lme().exact().is_err());
}

#[test]
fn certificate_sqrt_failure_custody_and_original_error_coexistence() {
    let a = parse("+8p1");
    for remaining in [0, 16, 64, 128, 256] {
        let mut sum = ExactWideSum::new();
        sum.test_seed_term_work(u64::MAX - remaining);
        let spent = sqrt_owned(&a, Toward::Up, WideContext::<16>::new(1024).unwrap(), sum);
        assert!(!spent.status().is_exact());
        assert!(spent.result().is_err());
        assert_eq!(spent.work().width::<16>().sqrt, 1);
        assert_eq!(spent.work().width::<16>().two_product, 1);
        assert!(spent.sum_work().term_limbs >= u64::MAX - remaining);
    }
    let mut sum = ExactWideSum::new();
    sum.test_seed_term_work(u64::MAX);
    let spent = sqrt_owned(
        &parse("-8p0"),
        Toward::Up,
        WideContext::<16>::new(1024).unwrap(),
        sum,
    );
    assert_eq!(
        spent.result(),
        Err(HelperError::Arithmetic(AttemptStop::Arithmetic(
            WideError::NegativeSqrt
        )))
    );
    assert!(!spent.status().is_exact());
    // Even an otherwise successful numerical value cannot be extracted when
    // the joined work has overflowed; no saturated getter certifies this.
    let mut blocked = EntrySpent::new(Ok(3.0));
    blocked.sum_work = spent.sum_work();
    blocked.work = spent.work();
    assert!(matches!(
        blocked.result(),
        Err(HelperError::Arithmetic(AttemptStop::WorkAccounting(_)))
    ));
}

#[test]
fn certificate_adjacent_step_failure_keeps_completed_side_work() {
    let &(_, input, down, _, nearest) = vectors::SQRT.iter().find(|v| v.0 == "sqrt_two").unwrap();
    let (no_step, stepping) = if down == nearest {
        (Toward::Down, Toward::Up)
    } else {
        (Toward::Up, Toward::Down)
    };
    let a = parse(input);
    let baseline = sqrt_endpoint(&a, no_step);
    let side = baseline.sum_work();
    let side_total = side.checked_lme().exact().unwrap();
    let mut sum = ExactWideSum::new();
    sum.test_seed_term_work(u64::MAX - side_total);
    let failed = sqrt_owned(&a, stepping, WideContext::<16>::new(1024).unwrap(), sum);
    assert!(matches!(
        &failed.result,
        Err(HelperError::Arithmetic(AttemptStop::WorkAccounting(_)))
    ));
    assert!(!failed.status().is_exact());
    // The complete q*q-a side test was collected; the step's first insertion
    // was refused before it performed work, so no context round was charged.
    assert_eq!(
        failed.sum_work().term_limbs,
        u64::MAX - side_total + side.term_limbs
    );
    assert_eq!(failed.sum_work().shift_limbs, side.shift_limbs);
    assert_eq!(failed.sum_work().net_limbs, side.net_limbs);
    assert_eq!(failed.work().width::<16>().round, 0);
    assert_eq!(failed.work().width::<16>().sqrt, 1);
    assert_eq!(failed.work().width::<16>().two_product, 1);
}
