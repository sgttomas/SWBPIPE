//! K4: directed wide rounding for D1 revision 5a.3's certified bounds (R7
//! §4.1.6.3 items 6, 7, 9, 11 and 12; ROOT's A3-0 rulings Q2 and Q4).
//!
//! R7 7b: "Every quantity below is nonnegative and formed at P bits, rounded
//! upward (to nearest, then one ulp up when below the exact value, as fl↑ in
//! item 6a)", with the denominators 1 − t_c and σ_c, and the difference σ′_c,
//! rounded downward. "Either rounding direction may be implemented as round to
//! nearest followed, unconditionally, by one ulp in the required direction."
//!
//! K4 rounds to nearest and moves one P-bit step only when the nearest value
//! lies on the wrong side of the exact value, decided exactly (emu7's `ru` and
//! `rd`). That result is never on the wrong side, and it is the tightest one.
//!
//! Every operand has at most P bits (the context's precision), as
//! `ExactWideSum::add_product` requires; the results are P-bit values.
use super::adaptive::{next_up, AttemptStop};
use super::wide::multi::{Binary64Outcome, SupportedWidth, WideContext};
use super::wide::Wide;
use super::wide_sum::ExactWideSum;

/// The direction of a bound's rounding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Toward {
    Up,
    Down,
}

/// The adjacent P-bit value of a nonzero x in the given direction: its
/// magnitude grows by 2^(e−P+1), or shrinks by that, or by 2^(e−P) when |x|
/// is a power of two (e = x.exponent()). One exact sum of representable
/// value, so its rounding is exact.
fn step<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    x: &Wide<L>,
    toward: Toward,
) -> Result<Wide<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    debug_assert!(!x.is_zero(), "a directed step from zero is not formed");
    let p = i64::from(ctx.precision());
    let away = (toward == Toward::Up) != x.is_sign_negative();
    let e = x.exponent();
    let unit = if away || !x.fits_precision(1) {
        e - p + 1
    } else {
        e - p
    };
    sum.clear();
    sum.add_wide(x, false)?;
    // Away from zero adds a unit to the magnitude, toward zero removes one.
    let negate = x.is_sign_negative() == away;
    sum.add_wide_scaled(&Wide::<L>::ONE, negate, 1, unit)?;
    let out = sum.round(ctx)?;
    sum.clear();
    Ok(out)
}

/// `sum`'s exact value rounded at the context's precision toward `toward`:
/// to nearest, then one step when the nearest lies on the wrong side, decided
/// exactly. Clears `sum`.
pub(crate) fn round_toward<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    toward: Toward,
) -> Result<Wide<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let nearest = sum.round(ctx)?;
    sum.add_wide(&nearest, true)?;
    // The sign of (exact − nearest).
    let side = sum.signum();
    sum.clear();
    let wrong = match toward {
        Toward::Up => side > 0,
        Toward::Down => side < 0,
    };
    if wrong {
        step(ctx, sum, &nearest, toward)
    } else {
        Ok(nearest)
    }
}

/// a + b, rounded toward `toward`.
pub(crate) fn add_toward<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    a: &Wide<L>,
    b: &Wide<L>,
    toward: Toward,
) -> Result<Wide<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    sum.clear();
    sum.add_wide(a, false)?;
    sum.add_wide(b, false)?;
    round_toward(ctx, sum, toward)
}

/// a − b, rounded toward `toward`.
pub(crate) fn sub_toward<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    a: &Wide<L>,
    b: &Wide<L>,
    toward: Toward,
) -> Result<Wide<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    sum.clear();
    sum.add_wide(a, false)?;
    sum.add_wide(b, true)?;
    round_toward(ctx, sum, toward)
}

/// a · b, rounded toward `toward`.
pub(crate) fn mul_toward<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    a: &Wide<L>,
    b: &Wide<L>,
    toward: Toward,
) -> Result<Wide<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    sum.clear();
    sum.add_product(ctx, a, b, false)?;
    round_toward(ctx, sum, toward)
}

/// a / b for b > 0, rounded toward `toward`: the nearest quotient q, then the
/// exact sign of q·b − a decides whether q lies on the wrong side.
pub(crate) fn div_toward<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    a: &Wide<L>,
    b: &Wide<L>,
    toward: Toward,
) -> Result<Wide<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    debug_assert!(!b.is_zero() && !b.is_sign_negative());
    let q = ctx.div(a, b)?;
    if q.is_zero() {
        // Only an exact zero numerator gives a zero quotient at K4's widths.
        return Ok(q);
    }
    sum.clear();
    sum.add_product(ctx, &q, b, false)?;
    sum.add_wide(a, true)?;
    // q·b − a has the sign of q − a/b (b > 0).
    let side = sum.signum();
    sum.clear();
    let wrong = match toward {
        Toward::Up => side < 0,
        Toward::Down => side > 0,
    };
    if wrong {
        step(ctx, sum, &q, toward)
    } else {
        Ok(q)
    }
}

/// The least binary64 not below x ≥ 0 (+∞ beyond the range): K3's nearest
/// conversion, then `next_up` when the exact comparison shows it below x.
pub(crate) fn binary64_up<const L: usize>(x: &Wide<L>) -> Result<f64, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    debug_assert!(x.is_zero() || !x.is_sign_negative());
    let nearest = match x.to_binary64() {
        Binary64Outcome::Normal(v) => v.abs(),
        Binary64Outcome::Subnormal { value, .. } => value.abs(),
        Binary64Outcome::Underflow { .. } => 0.0,
        Binary64Outcome::Overflow { .. } => return Ok(f64::INFINITY),
    };
    let mut sum = ExactWideSum::new();
    sum.add_wide(x, false)?;
    sum.add_binary64(nearest, true)?;
    if sum.signum() > 0 {
        Ok(if nearest == f64::MAX {
            f64::INFINITY
        } else {
            next_up(nearest)
        })
    } else {
        Ok(nearest)
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/directed_tests.rs"]
mod tests;
