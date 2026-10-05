#![allow(dead_code)]
#[path = "/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a/projects/chirality-piping/core/solver/frame_kernel/src/exact_sum.rs"]
mod exact_sum;
#[path = "/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/work.rs"]
mod work;
#[path = "/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/wide.rs"]
mod wide;
#[path = "/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a/projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/wide_sum.rs"]
mod wide_sum;

#[cfg(test)]
mod rv51 {
    use super::work::{WorkFault, WorkTotal};
    use super::wide::{Wide, multi::WideContext};
    use super::wide_sum::{ExactWideSum, SumRefusal};

    #[test]
    fn arithmetic_matches_u128_oracle_at_independent_boundaries() {
        let points = [0, 1, 2, u64::MAX / 2, u64::MAX / 2 + 1, u64::MAX - 1, u64::MAX];
        for a in points { for b in points {
            let x = WorkTotal::exact_count(a);
            let y = WorkTotal::exact_count(b);
            let sum = u128::from(a) + u128::from(b);
            let product = u128::from(a) * u128::from(b);
            assert_eq!(x.add(y).exact(), if sum <= u128::from(u64::MAX) { Ok(sum as u64) } else { Err(WorkFault::Overflow) });
            assert_eq!(x.mul(b).exact(), if product <= u128::from(u64::MAX) { Ok(product as u64) } else { Err(WorkFault::Overflow) });
            assert_eq!(x.remainder(y).exact(), if a >= b { Ok(a-b) } else { Err(WorkFault::Inconsistent) });
        }}
    }

    #[test]
    fn shift_reservation_includes_pending_term_before_shift() {
        let mut sum = ExactWideSum::new();
        sum.add_integer(false, &[1], 64).unwrap();
        let before = format!("{sum:?}");
        sum.test_seed_term_work(u64::MAX - 9);
        assert_eq!(sum.add_integer(false, &[1], 0), Err(SumRefusal::WorkAccounting(WorkFault::Overflow)));
        assert_eq!(format!("{sum:?}"), before, "the failed prospective shift must not alter value metadata");
        assert_eq!(sum.work().shift_limbs, 0);
        assert_eq!(sum.work().term_limbs, u64::MAX - 9);
        assert_eq!(sum.work().checked_lme().exact(), Err(WorkFault::Overflow));
        let mut exact = ExactWideSum::new();
        exact.add_integer(false, &[1], 64).unwrap();
        exact.test_seed_term_work(u64::MAX - 10);
        exact.add_integer(false, &[1], 0).unwrap();
        assert_eq!(exact.work().shift_limbs, 8);
        assert_eq!(exact.work().checked_lme().exact(), Ok(u64::MAX));
    }

    #[test]
    fn second_carry_failure_retains_only_first_actual_tail_charge() {
        let mut sum = ExactWideSum::new();
        sum.add_integer(false, &[u64::MAX, u64::MAX, u64::MAX], 0).unwrap();
        sum.test_seed_term_work(u64::MAX - 3);
        assert_eq!(sum.add_integer(false, &[1], 0), Err(SumRefusal::WorkAccounting(WorkFault::Overflow)));
        assert_eq!(sum.work().term_limbs, u64::MAX - 2);
        assert!(format!("{sum:?}").contains("value_poisoned: true"));
        sum.clear();
        assert!(format!("{sum:?}").contains("empty: true"));
        assert_eq!(sum.signum(), Err(SumRefusal::WorkAccounting(WorkFault::Overflow)));
        let mut exact = ExactWideSum::new();
        exact.add_integer(false, &[u64::MAX, u64::MAX, u64::MAX], 0).unwrap();
        exact.test_seed_term_work(u64::MAX - 4);
        exact.add_integer(false, &[1], 0).unwrap();
        assert_eq!(exact.work().checked_lme().exact(), Ok(u64::MAX));
    }

    #[test]
    fn second_expansion_term_span_requires_reset_before_reuse() {
        let mut ctx = WideContext::<4>::new(128).unwrap();
        let a = ctx.from_integer(false, &[1, 1u64 << 63], -127).unwrap();
        let mut sum = ExactWideSum::new();
        sum.add_integer(false, &[1], 8001).unwrap();
        assert_eq!(sum.add_product(&mut ctx, &a, &a, false), Err(SumRefusal::Span));
        assert!(format!("{sum:?}").contains("value_poisoned: true"));
        let mut misuse = sum.clone();
        assert_eq!(misuse.signum(), Err(SumRefusal::WorkAccounting(WorkFault::Inconsistent)));
        misuse.reset();
        assert_eq!(misuse.add_binary64(0.0, false), Err(SumRefusal::WorkAccounting(WorkFault::Inconsistent)));
        sum.clear();
        assert_eq!(sum.signum(), Ok(0));
        sum.add_binary64(2.0, false).unwrap();
        assert_eq!(sum.round(&mut ctx).unwrap(), Wide::<4>::from_f64(2.0).unwrap());
    }

    #[test]
    fn later_product_of_limb_span_cannot_expose_partial_value() {
        let mut a = ExactWideSum::new();
        a.add_binary64(1.0, false).unwrap();
        let mut b = ExactWideSum::new();
        b.add_integer(false, &[1], 0).unwrap();
        b.add_integer(false, &[1], 8127).unwrap();
        let mut sum = ExactWideSum::new();
        sum.add_integer(false, &[1], -1).unwrap();
        assert_eq!(sum.add_product_of(&a, &mut b, false), Err(SumRefusal::Span));
        assert!(format!("{sum:?}").contains("value_poisoned: true"));
        sum.clear();
        assert_eq!(sum.signum(), Ok(0));
    }
}
