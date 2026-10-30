//! The fixed binary1024 D1–D5 declaration. Inputs are exact binary64 leaves;
//! neither the scheduled precision nor a rounded chord enters this formation.
use super::adaptive::AttemptStop;
use super::source::{PrimitiveSource, UniformMemberLoad};
use super::wide::multi::WideContext;
use super::wide::Wide;
use super::wide_sum::ExactWideSum;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DeclaredMemberLoad {
    pub(crate) member: u32,
    pub(crate) node_i: u32,
    pub(crate) node_j: u32,
    pub(crate) intensity_bits: [u64; 3],
    /// F followed by M; node j receives the same F and the opposite M.
    pub(crate) entries: [Wide<16>; 6],
}
impl DeclaredMemberLoad {
    pub(crate) fn active(&self) -> bool { self.intensity_bits.iter().any(|b| f64::from_bits(*b) != 0.0) }
    pub(crate) fn node_entry(&self, at: usize) -> (usize, Wide<16>) {
        let node = if at < 6 { self.node_i } else { self.node_j };
        let component = at % 6;
        let value = self.entries[component];
        let value = if at >= 9 && !value.is_zero() { value.neg() } else { value };
        (node as usize * 6 + component, value)
    }
}
fn lift(x: f64) -> Result<Wide<16>, AttemptStop> { Ok(Wide::<16>::from_f64(x)?) }

/// One member checkpoint after all of that member's authored contributions.
/// The caller supplies its cumulative stage guard, so failed work is retained.
pub(crate) fn form(
    source: &PrimitiveSource,
    loads: &[UniformMemberLoad],
    ctx: &mut WideContext<16>,
    sum: &mut ExactWideSum,
    mut checkpoint: impl FnMut(&WideContext<16>, &ExactWideSum) -> Result<(), AttemptStop>,
) -> Result<Vec<DeclaredMemberLoad>, AttemptStop> {
    debug_assert_eq!(ctx.precision(), 1024);
    let mut out = Vec::with_capacity(loads.len());
    let mut at = 0;
    while at < loads.len() {
        let member = loads[at].member;
        let m = &source.members()[source.member_index(member).ok_or(AttemptStop::Structure)?];
        let xi = source.nodes()[m.node_i as usize];
        let xj = source.nodes()[m.node_j as usize];
        // D1: nine exact binary64 products; d is deliberately never formed.
        sum.clear();
        for a in 0..3 {
            let (i, j) = (lift(xi[a])?, lift(xj[a])?);
            sum.add_product(ctx, &j, &j, false)?;
            sum.add_product(ctx, &i.mul_pow2(1)?, &j, true)?;
            sum.add_product(ctx, &i, &i, false)?;
        }
        let s = sum.round(ctx)?;
        let length = ctx.sqrt(&s)?;
        let g = ctx.div(&length, &lift(12.0)?)?;
        while at < loads.len() && loads[at].member == member {
            let load = &loads[at];
            let w = load.intensity;
            let mut entries = [Wide::<16>::ZERO; 6];
            for k in 0..3 {
                if w[k] != 0.0 { entries[k] = ctx.mul(&length, &lift(w[k])?)?.mul_pow2(-1)?; }
            }
            for (a, (b, c)) in [(1, 2), (2, 0), (0, 1)].into_iter().enumerate() {
                sum.clear();
                // Each x*w is exact (<=106 bits); each g*x*w joins one exact
                // four-term sum. Rounding the cross product changes D5.
                for (x, v, negate) in [(xj[b], w[c], false), (xi[b], w[c], true),
                    (xj[c], w[b], true), (xi[c], w[b], false)] {
                    let product = ctx.mul(&lift(x)?, &lift(v)?)?;
                    sum.add_product(ctx, &g, &product, negate)?;
                }
                entries[3 + a] = sum.round(ctx)?;
            }
            for value in &mut entries { if value.is_zero() { *value = Wide::<16>::ZERO; } }
            out.push(DeclaredMemberLoad { member, node_i: m.node_i, node_j: m.node_j,
                intensity_bits: w.map(f64::to_bits), entries });
            at += 1;
        }
        checkpoint(ctx, sum)?;
    }
    Ok(out)
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/member_load_tests.rs"]
mod tests;
