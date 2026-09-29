//! K4: the verification of D1 revision 5a.3 (R7 §4.1.6.1 item 6a, §4.1.6.2
//! and §4.1.6.3; ROOT's A3-0 rulings). Checkpoint A3a holds the formation
//! scale E and its binary64 companions; the verification estimate, the charge
//! and W⁺ follow at A3b.
//!
//! **The formation scale E (§4.1.6.2 item 4).** Every force or moment row's
//! recovery expansion evaluated with the bounded operator B̄, Ā and absolute
//! operands, stage by stage as `recover` forms the row: the node's 1-norm
//! d̄ = |w_x| + |w_y| + |w_z| (the same for the three local axes, since B̄ puts
//! 1 at every axis component), ē (ē0 = d̄_j + d̄_i for extension and twist;
//! ē2..5 = d̄_rot + (1/L)(d̄_i + d̄_j)), Q̄ = |D|·ē, V̄ = (1/L)(Q̄_a + Q̄_b), the
//! end actions multiplied by g_m; station moments |t|·Q̄_j + |t|·Q̄_i + Q̄_i
//! (×g_m), forces as at end j; spring actions |k|·w; directional actions
//! Σ_b |k_ab|·w_b; reactions |f_c| + Σ_j Ā_cj·w_j, with f_c the exact ledger net;
//! and a support group's magnitude row as one exact sum over every contributor
//! of its three components (reactions, spring actions and directional
//! components), rounded once (ROOT's ruling Q12). Each stage is one exact sum
//! of exact two-factor products, rounded once: to nearest for E (R7 §3.1), or
//! upward for ‖ā_q S‖₁, the same expansion with the operand s (R7 item 6;
//! ROOT's ruling Q2).
//!
//! **E(body, kind)** is the maximum over the body's force (moment) rows,
//! unpublishable rows included, rounded upward once to binary64. **ê** (item
//! 6a) couples it in binary64: ê_fo = max(E_fo, fl(E_mo/L_b)),
//! ê_mo = max(E_mo, fl(L_b·E_fo)); L_b = 0 gives ê = E. **Φ** = fl↑(2^-438·ê),
//! decided exactly as b is (the constant's bits `0x2490000000000000`).
use super::adaptive::{next_up, AttemptStop, StageGuard};
use super::assemble::{DirectionalBlock, MemberOperators, Structure};
use super::directed::{binary64_up, round_toward, Toward};
use super::ledger::RetainedLedger;
use super::recover::{Kind, QuantityMeta};
use super::source::PrimitiveSource;
use super::wide::multi::{SupportedWidth, WideContext};
use super::wide::Wide;
use super::wide_sum::ExactWideSum;
use crate::DOF_PER_NODE;
use std::cmp::Ordering as CmpOrdering;

/// The bits of 2^-438 (item 6a).
pub(crate) const PHI_SCALE_BITS: u64 = 0x2490_0000_0000_0000;

/// How each stage of `formation_scale` is rounded.
#[allow(dead_code)] // A3b: the verification pass (E-UNIT at A3a)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StageRounding {
    /// E (R7 §3.1): to nearest.
    Nearest,
    /// ‖ā_q S‖₁ (R7 item 6): upward.
    Up,
}

fn stage<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    rounding: StageRounding,
) -> Result<Wide<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    match rounding {
        StageRounding::Nearest => {
            let v = sum.round(ctx)?;
            sum.clear();
            Ok(v)
        }
        StageRounding::Up => round_toward(ctx, sum, Toward::Up),
    }
}

/// E_q for every force or moment row of `layout` (None for the other rows),
/// with the nonnegative operand w at every DOF (module documentation). The
/// ledger enters the reactions when given.
#[allow(dead_code)] // A3b: the verification pass (E-UNIT at A3a)
#[allow(clippy::too_many_arguments)]
pub(crate) fn formation_scale<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    source: &PrimitiveSource,
    layout: &[QuantityMeta],
    structure: &Structure,
    abar: &[Wide<L>],
    members: &[MemberOperators<L>],
    directional: &[DirectionalBlock<L>],
    ledger: Option<&RetainedLedger>,
    w: &[Wide<L>],
    rounding: StageRounding,
) -> Result<Vec<Option<Wide<L>>>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let zero = Wide::<L>::ZERO;
    let mut out: Vec<Option<Wide<L>>> = Vec::with_capacity(layout.len());
    // Displacements and node magnitudes carry no E.
    out.resize(source.dof_count() + source.node_count(), None);
    // Members: the end actions.
    let mut q_all: Vec<[Wide<L>; 6]> = Vec::with_capacity(members.len());
    let mut actions_all: Vec<[Wide<L>; 12]> = Vec::with_capacity(members.len());
    for m in members {
        let dofs = m.dofs();
        // d̄ per block (tr_i, rot_i, tr_j, rot_j).
        let mut block = [zero; 4];
        for (b, value) in block.iter_mut().enumerate() {
            sum.clear();
            for c in 0..3 {
                sum.add_wide(&w[dofs[3 * b + c]], false)?;
            }
            *value = stage(ctx, sum, rounding)?;
        }
        let d = |index: usize| block[index / 3];
        let inv = m.inv_length.abs();
        let mut e = [zero; 6];
        for (index, (plus, minus)) in [(6usize, 0usize), (9, 3)].into_iter().enumerate() {
            sum.clear();
            sum.add_wide(&d(plus), false)?;
            sum.add_wide(&d(minus), false)?;
            e[index] = stage(ctx, sum, rounding)?;
        }
        for (index, rotation, transverse) in
            [(2usize, 5usize, 1usize), (3, 11, 1), (4, 4, 2), (5, 10, 2)]
        {
            sum.clear();
            sum.add_wide(&d(rotation), false)?;
            sum.add_product(ctx, &inv, &d(transverse), false)?;
            sum.add_product(ctx, &inv, &d(transverse + 6), false)?;
            e[index] = stage(ctx, sum, rounding)?;
        }
        let (bz, by) = (m.bend_z.abs(), m.bend_y.abs());
        let (four_z, two_z) = (bz.mul_pow2(2)?, bz.mul_pow2(1)?);
        let (four_y, two_y) = (by.mul_pow2(2)?, by.mul_pow2(1)?);
        let mut q = [zero; 6];
        for (index, terms) in [
            [(m.axial.abs(), 0usize), (zero, 0)],
            [(m.torsion.abs(), 1), (zero, 1)],
            [(four_z, 2), (two_z, 3)],
            [(two_z, 2), (four_z, 3)],
            [(four_y, 4), (two_y, 5)],
            [(two_y, 4), (four_y, 5)],
        ]
        .into_iter()
        .enumerate()
        {
            sum.clear();
            for (coefficient, at) in terms {
                sum.add_product(ctx, &coefficient, &e[at], false)?;
            }
            q[index] = stage(ctx, sum, rounding)?;
        }
        let mut shear = |a: &Wide<L>, b: &Wide<L>| -> Result<Wide<L>, AttemptStop> {
            sum.clear();
            sum.add_product(ctx, &inv, a, false)?;
            sum.add_product(ctx, &inv, b, false)?;
            stage(ctx, sum, rounding)
        };
        let vy = shear(&q[2], &q[3])?;
        let vz = shear(&q[4], &q[5])?;
        let g = i64::from(m.g_exp);
        let raw = [
            q[0], vy, vz, q[1], q[4], q[2], q[0], vy, vz, q[1], q[5], q[3],
        ];
        let mut actions = [zero; 12];
        for (a, r) in actions.iter_mut().zip(raw) {
            *a = r.mul_pow2(g)?;
        }
        q_all.push(q);
        actions_all.push(actions);
        guard.check(ctx, sum)?;
    }
    for actions in &actions_all {
        out.extend(actions.iter().map(|a| Some(*a)));
    }
    // Stations.
    for s in source.stations() {
        let index = source.member_index(s.member).unwrap();
        let q = &q_all[index];
        let actions = &actions_all[index];
        let g = i64::from(members[index].g_exp);
        let t = Wide::<L>::from_f64(s.fraction)?.abs();
        let mut moment = |at_j: &Wide<L>, at_i: &Wide<L>| -> Result<Wide<L>, AttemptStop> {
            sum.clear();
            sum.add_product(ctx, &t, at_j, false)?;
            sum.add_product(ctx, &t, at_i, false)?;
            sum.add_wide(at_i, false)?;
            Ok(stage(ctx, sum, rounding)?.mul_pow2(g)?)
        };
        let my = moment(&q[5], &q[4])?;
        let mz = moment(&q[3], &q[2])?;
        for v in [actions[6], actions[7], actions[8], actions[9], my, mz] {
            out.push(Some(v));
        }
    }
    // Spring actions.
    let mut spring_e = Vec::with_capacity(source.springs().len());
    for s in source.springs() {
        sum.clear();
        let k = Wide::<L>::from_f64(s.stiffness)?.abs();
        sum.add_product(ctx, &k, &w[s.dof.global()], false)?;
        let v = stage(ctx, sum, rounding)?;
        spring_e.push(v);
        out.push(Some(v));
    }
    // Directional spring actions.
    let mut directional_e = Vec::with_capacity(directional.len());
    for block in directional {
        let base = block.node as usize * DOF_PER_NODE + block.kind.offset();
        let mut components = [zero; 3];
        for (a, component) in components.iter_mut().enumerate() {
            sum.clear();
            for b in 0..3 {
                sum.add_product(ctx, &block.k[a][b].abs(), &w[base + b], false)?;
            }
            *component = stage(ctx, sum, rounding)?;
        }
        out.extend(components.iter().map(|c| Some(*c)));
        directional_e.push(components);
    }
    // Reactions: |f_c| + Σ_j Ā_cj·w_j.
    let mut reaction_e = vec![None; source.dof_count()];
    for c in source.constraints() {
        let g = c.dof.global();
        sum.clear();
        if let Some(net) = ledger.and_then(|l| l.net(g)) {
            if !net.is_zero() {
                sum.add_integer(false, &net.magnitude, net.exponent)?;
            }
        }
        for index in structure.pattern.row_range(g) {
            let j = structure.pattern.column(index);
            sum.add_product(ctx, &abar[index], &w[j], false)?;
        }
        let v = stage(ctx, sum, rounding)?;
        reaction_e[g] = Some(v);
        out.push(Some(v));
    }
    guard.check(ctx, sum)?;
    // Support groups: one exact sum over every contributor of the components.
    for group in source.supports() {
        for range in [0usize..3, 3..6] {
            sum.clear();
            for c in range {
                let g = group.node as usize * DOF_PER_NODE + c;
                if group.restrained[c] {
                    if let Some(r) = &reaction_e[g] {
                        sum.add_wide(r, false)?;
                    }
                }
                for id in &group.springs {
                    let k = source.springs().binary_search_by_key(id, |s| s.id).unwrap();
                    if source.springs()[k].dof.component.index() == c {
                        sum.add_wide(&spring_e[k], false)?;
                    }
                }
                for id in &group.directional_springs {
                    let k = source
                        .directional_springs()
                        .binary_search_by_key(id, |s| s.id)
                        .unwrap();
                    let offset = source.directional_springs()[k].kind.offset();
                    if (offset..offset + 3).contains(&c) {
                        sum.add_wide(&directional_e[k][c - offset], false)?;
                    }
                }
            }
            out.push(Some(stage(ctx, sum, rounding)?));
        }
    }
    debug_assert_eq!(out.len(), layout.len());
    // Only force and moment rows carry E.
    for (meta, e) in layout.iter().zip(out.iter_mut()) {
        if !matches!(meta.kind, Kind::Force | Kind::Moment) {
            *e = None;
        }
    }
    Ok(out)
}

/// E(body, kind) for force and moment: the largest E_q over the body's rows of
/// the kind, rounded upward once to binary64 (+∞ when it overflows). `[E_fo,
/// E_mo]` per body.
#[allow(dead_code)] // A3b: the verification pass (E-UNIT at A3a)
pub(crate) fn resolution_scale<const L: usize>(
    layout: &[QuantityMeta],
    e_rows: &[Option<Wide<L>>],
    bodies: usize,
) -> Result<Vec<[f64; 2]>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let mut top = vec![[Wide::<L>::ZERO; 2]; bodies];
    for (meta, e) in layout.iter().zip(e_rows) {
        let (Some(e), Some(slot)) = (
            e,
            match meta.kind {
                Kind::Force => Some(0usize),
                Kind::Moment => Some(1),
                _ => None,
            },
        ) else {
            continue;
        };
        let current = &mut top[meta.body as usize][slot];
        if e.cmp_value(current) == CmpOrdering::Greater {
            *current = *e;
        }
    }
    top.iter()
        .map(|[fo, mo]| Ok([binary64_up(fo)?, binary64_up(mo)?]))
        .collect()
}

/// Item 6a's coupling in binary64: ê_fo = max(E_fo, fl(E_mo/L_b)),
/// ê_mo = max(E_mo, fl(L_b·E_fo)); a single-node body (L_b = 0) keeps E.
#[allow(dead_code)] // A3b: V, the estimate's threshold, the allowance and Φ
pub(crate) fn e_hat(e: [f64; 2], extent: f64) -> [f64; 2] {
    if extent == 0.0 {
        return e;
    }
    let [fo, mo] = e;
    [fo.max(mo / extent), mo.max(extent * fo)]
}

/// Φ = fl↑(2^-438·ê): the nearest, then the next up when it is below the exact
/// product (decided exactly, as `absolute_bound` decides b).
#[allow(dead_code)] // A3b: the ceiling floor
pub(crate) fn phi_512(e_hat: f64) -> f64 {
    let scale = f64::from_bits(PHI_SCALE_BITS);
    let back = f64::from_bits(0x5B50_0000_0000_0000); // 2^438
    let nearest = e_hat * scale;
    if nearest * back < e_hat {
        next_up(nearest)
    } else {
        nearest
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/verify_tests.rs"]
mod tests;
