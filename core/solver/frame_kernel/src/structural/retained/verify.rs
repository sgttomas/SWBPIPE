//! K4: the verification of D1 revision 5a.3 (R7 §4.1.6.1 item 6a, §4.1.6.2
//! and §4.1.6.3; ROOT's A3-0 rulings): the formation scale E and its binary64
//! companions, and the verification pass (`verify_state`): the estimate's
//! residual over the q_W contributions, δ̂ and Ŵ, the correction's residual,
//! the norms, the certified bounds, θ, the g check, the charge and W⁺.
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
use super::adaptive::{
    lme, next_up, AttemptStop, CasePrep, GroupPrep, Shared, Solved, Stage, StageGuard, StageWork,
};
use super::assemble::{
    assemble_bounded, form_directional, form_members, BoundedCoefficients, Contribution,
    DirectionalBlock, MemberOperators, Structure,
};
use super::bound::{
    block_of_rows, block_refusals, certificates, data_blocks, gamma_m, shift_run, shift_start,
    uc_bounds, uc_refusals, BlockBound, BlockCertificate, BlockRefusal, BoundRefusal,
};
use super::directed::{add_toward, binary64_up, mul_toward, round_toward, Toward};
use super::factor::Ordering;
use super::ledger::RetainedLedger;
use super::recover::{recover, Kind, QuantityId, QuantityMeta};
use super::source::PrimitiveSource;
use super::wide::multi::{AttemptWork, SupportedWidth, WideContext};
use super::wide::Wide;
use super::wide_sum::{ExactWideSum, SumWork};
use crate::DOF_PER_NODE;
use std::cmp::Ordering as CmpOrdering;

/// The bits of 2^-438 (item 6a).
pub const PHI_SCALE_BITS: u64 = 0x2490_0000_0000_0000;

/// How each stage of `formation_scale` is rounded.
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
pub fn e_hat(e: [f64; 2], extent: f64) -> [f64; 2] {
    // V-K seeded fault VK-R02 (R7-M2): ê uncoupled.
    #[cfg(any(test, feature = "mutation-controls"))]
    if super::seeded::active(super::seeded::Fault::R02) {
        return e;
    }
    if extent == 0.0 {
        return e;
    }
    let [fo, mo] = e;
    [fo.max(mo / extent), mo.max(extent * fo)]
}

/// ê per body from finite E (item 6a). An ê that overflows binary64 while E
/// encodes (fl(L_b·E_fo) or fl(E_mo/L_b) beyond the range) is the same
/// terminal case as E's own overflow (ROOT's A3b ruling): V, the estimate's
/// threshold and the charge's allowance would be infinite, and no precision
/// can pass, so the verification stops with `ResolutionScale` (F2a:
/// `receipt_encoding`).
pub fn resolution_hats(
    resolution: &[[f64; 2]],
    extents: &[f64],
) -> Result<Vec<[f64; 2]>, AttemptStop> {
    resolution
        .iter()
        .zip(extents)
        .enumerate()
        .map(|(b, (e, &extent))| {
            let hat = e_hat(*e, extent);
            for (kind, v) in [(Kind::Force, hat[0]), (Kind::Moment, hat[1])] {
                if !v.is_finite() {
                    return Err(AttemptStop::ResolutionScale {
                        body: b as u32,
                        kind,
                    });
                }
            }
            Ok(hat)
        })
        .collect()
}

/// Φ = fl↑(2^-438·ê): the nearest, then the next up when it is below the exact
/// product (decided exactly, as `absolute_bound` decides b).
pub fn phi_512(e_hat: f64) -> f64 {
    let scale = f64::from_bits(PHI_SCALE_BITS);
    let back = f64::from_bits(0x5B50_0000_0000_0000); // 2^438
    let nearest = e_hat * scale;
    if nearest * back < e_hat {
        next_up(nearest)
    } else {
        nearest
    }
}

// ------------------------------------------------------------ the verification pass (A3b)

/// q_W = min(3p + 64, 1024) for the verification precision P = 2p (R7
/// §4.1.6.3 item 1): 448 at 256, 832 at 512, 1024 at 1024 (Q4: the state's own
/// entries).
pub(crate) fn q_w_of(verification_precision: u32) -> u32 {
    (3 * (verification_precision / 2) + 64).min(1024)
}

/// Precision-shared verification data: Ā at P, K_e and the directional blocks
/// at q_W, γ_m and every block's comparison-matrix bound (R7 7b).
#[derive(Debug)]
pub(crate) struct VerifyShared<const L: usize, const W: usize>
where
    Wide<L>: SupportedWidth,
    Wide<W>: SupportedWidth,
{
    pub(crate) q_w: u32,
    pub(crate) abar: Vec<Wide<L>>,
    /// Per member, K_e at q_W (all 144 entries; symmetric).
    pub(crate) ke_w: Vec<[[Wide<W>; 12]; 12]>,
    pub(crate) directional_w: Vec<[[Wide<W>; 3]; 3]>,
    pub(crate) gamma: Wide<L>,
    pub(crate) uc: Vec<BlockBound<L>>,
    pub(crate) work: AttemptWork,
    pub(crate) sum_work: SumWork,
    pub(crate) stages: StageWork,
    pub(crate) total: u64,
}

/// A verification build or pass with the work it spent.
pub(crate) struct VerifySpent<T> {
    pub(crate) result: Result<T, AttemptStop>,
    pub(crate) work: AttemptWork,
    pub(crate) sum_work: SumWork,
    pub(crate) stages: StageWork,
    pub(crate) total: u64,
    /// T3 KF3 (amendment A2): the pass's S_c refusals, per block, on every
    /// path. For the shared build, its Uc_c refusals when it stops (RV23-1);
    /// a completed shared build's are in `uc`, and this is empty.
    pub(crate) refusals: Vec<BlockRefusal>,
}

/// Builds `VerifyShared` from the verification precision's shared stages.
pub(crate) fn build_verify_shared<const L: usize, const R: usize, const W: usize>(
    shared: &Shared<L, R>,
    source: &PrimitiveSource,
    group: &GroupPrep,
    guard: StageGuard,
) -> VerifySpent<VerifyShared<L, W>>
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
    Wide<W>: SupportedWidth,
{
    let p = shared.p;
    let q_w = q_w_of(p);
    let mut ctx = WideContext::<L>::new(p).expect("supported precision");
    let mut ctx_w = WideContext::<W>::new(q_w).expect("supported precision");
    let mut sum = ExactWideSum::new();
    let mut stages = StageWork::default();
    // T3 KF3: the stage in progress, for a stopped build's unstaged work.
    let mut current = Stage::BoundedFormation;
    // RV23-1: the Uc_c refusal slots, one per block, held outside the build so
    // that a stop after a refusal keeps it in the evidence.
    let mut uc_refused: Vec<Option<BoundRefusal>> = vec![None; group.blocks.len()];
    let mut run = || -> Result<VerifyShared<L, W>, AttemptStop> {
        let spent = |ctx: &WideContext<L>, ctx_w: &WideContext<W>, sum: &ExactWideSum| {
            lme(ctx) + lme(ctx_w) + sum.work().limb_multiply_equivalents()
        };
        let t0 = spent(&ctx, &ctx_w, &sum);
        let g = guard;
        let bounded: Vec<BoundedCoefficients<L>> =
            shared.members.iter().map(|m| m.bounded()).collect();
        let abar = assemble_bounded(
            &mut ctx,
            &mut sum,
            &g,
            source,
            &group.structure,
            &bounded,
            &shared.directional,
        )?;
        let t1 = spent(&ctx, &ctx_w, &sum);
        stages.bounded_formation = t1 - t0;
        current = Stage::WideFormation;
        let (ke_w, directional_w) = if q_w == p {
            (
                shared
                    .members
                    .iter()
                    .map(|m| full_ke(m).map(|row| row.map(|v| v.widen::<W>())))
                    .collect(),
                shared
                    .directional
                    .iter()
                    .map(|d| d.k.map(|row| row.map(|v| v.widen::<W>())))
                    .collect(),
            )
        } else {
            let gw = guard.with_base(lme(&ctx));
            let members_w = form_members(&mut ctx_w, &mut sum, &gw, source)?;
            let directional_w = form_directional(&mut ctx_w, &mut sum, source)?;
            (
                members_w.iter().map(full_ke).collect(),
                directional_w.iter().map(|d| d.k).collect(),
            )
        };
        let t2 = spent(&ctx, &ctx_w, &sum);
        stages.wide_formation = t2 - t1;
        current = Stage::Uc;
        let gu = guard.with_base(lme(&ctx_w));
        let nf = group.ordering.free.len();
        let gamma = gamma_m(&mut ctx, &mut sum, nf)?;
        let rows = block_of_rows(&group.ordering, &group.blocks);
        // Amendment A2: a refusal makes only its block's Uc_c unavailable; it
        // no longer stops the build.
        let uc = uc_bounds(
            &mut ctx,
            &mut sum,
            &gu,
            &shared.factor,
            &rows,
            &mut uc_refused,
            &gamma,
        )?;
        let t3 = spent(&ctx, &ctx_w, &sum);
        stages.uc = t3 - t2;
        Ok(VerifyShared {
            q_w,
            abar,
            ke_w,
            directional_w,
            gamma,
            uc,
            work: AttemptWork::default(),
            sum_work: SumWork::default(),
            stages: StageWork::default(),
            total: 0,
        })
    };
    let result = run();
    let mut work = AttemptWork::default();
    work.record(&ctx);
    work.record(&ctx_w);
    let sum_work = sum.work();
    let total = work.limb_multiply_equivalents() + sum_work.limb_multiply_equivalents();
    stages.close_stopped(&result, current, total);
    // RV23-1: a stopped build's Uc_c refusals (a completed build's are in `uc`).
    let refusals = if result.is_err() {
        uc_refusals(&uc_refused)
    } else {
        Vec::new()
    };
    let result = result.map(|mut v| {
        v.work = work;
        v.sum_work = sum_work;
        v.stages = stages.clone();
        v.total = total;
        v
    });
    VerifySpent {
        result,
        work,
        sum_work,
        stages,
        total,
        refusals,
    }
}

fn full_ke<const L: usize>(m: &MemberOperators<L>) -> [[Wide<L>; 12]; 12]
where
    Wide<L>: SupportedWidth,
{
    let mut out = [[Wide::<L>::ZERO; 12]; 12];
    for (a, row) in out.iter_mut().enumerate() {
        for (b, v) in row.iter_mut().enumerate() {
            *v = *m.ke(a, b);
        }
    }
    out
}

/// Per block, the norms of item 6 (each rounded upward).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BlockNorms<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    /// ‖SĀS‖ as the larger of the 1-norm and the ∞-norm, and both.
    pub(crate) sas: Wide<L>,
    pub(crate) sas_one: Wide<L>,
    pub(crate) sas_inf: Wide<L>,
    /// ‖SĀ|u⁰|‖_∞.
    pub(crate) sau: Wide<L>,
    /// ‖S·r‖_∞ and ‖S·r₂‖_∞.
    pub(crate) sr: Wide<L>,
    pub(crate) sr2: Wide<L>,
    /// ‖S⁻¹δ̂‖_∞.
    pub(crate) sid: Wide<L>,
}

/// Per body: B_b, θ_b and the charge's terms (item 11).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BodyReport<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    pub(crate) b: Option<Wide<L>>,
    pub(crate) theta: Wide<L>,
    pub(crate) norms: BlockNorms<L>,
    pub(crate) n_u: Wide<L>,
    pub(crate) t1: Wide<L>,
    pub(crate) t2: Wide<L>,
    pub(crate) t3: Wide<L>,
}

/// Everything R7 §4.1.6.2 and §4.1.6.3 items 1–14 compute at one
/// verification state.
#[derive(Debug, Clone)]
pub(crate) struct VerificationReport<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    pub(crate) precision: u32,
    #[allow(dead_code)] // evidence (E-CHARGE)
    pub(crate) q_w: u32,
    /// [E_fo, E_mo] per body (uncoupled, rounded upward to binary64).
    pub(crate) resolution: Vec<[f64; 2]>,
    #[allow(dead_code)] // evidence (E-CHARGE, E-HEADROOM)
    pub(crate) e_rows: Vec<Option<Wide<L>>>,
    /// Ŵ_q on force and moment rows.
    pub(crate) w: Vec<Option<Wide<L>>>,
    /// ‖ā_q S‖₁ on force and moment rows.
    #[allow(dead_code)] // evidence (E-CHARGE)
    pub(crate) a_s: Vec<Option<Wide<L>>>,
    /// C_q on force and moment rows (None when a block lacks B).
    pub(crate) charge: Vec<Option<Wide<L>>>,
    /// W⁺ on free translation and rotation rows and node magnitudes (None
    /// when a block lacks B: rule (a) then uses 0, R7-order Q7).
    pub(crate) w_plus: Vec<Option<Wide<L>>>,
    /// r̂ and δ̂ per free position.
    #[allow(dead_code)] // evidence (E-CHARGE)
    pub(crate) r_hat: Vec<Wide<L>>,
    #[allow(dead_code)] // evidence (E-CHARGE)
    pub(crate) delta: Vec<Wide<L>>,
    pub(crate) blocks: Vec<BlockCertificate<L>>,
    #[allow(dead_code)] // evidence (E-CHARGE)
    pub(crate) norms: Vec<BlockNorms<L>>,
    /// θ_c for each block with data.
    pub(crate) theta: Vec<Option<Wide<L>>>,
    pub(crate) bodies: Vec<BodyReport<L>>,
    /// The largest g_exp in the g check's scope, and the first member (id)
    /// above P − 16.
    pub(crate) g_max: u32,
    pub(crate) g_violation: Option<u32>,
    pub(crate) uc_missing: Option<usize>,
    pub(crate) shift_factorizations: u8,
}

/// K^c's contributions to row g (free), times u⁰ (exact at prescribed DOFs),
/// subtracted from `r` (item 1): r_g = f_g − Σ k·u⁰. With `delta`, instead
/// subtracts Σ_j K^c_gj·δ̂_j over the free columns (item 5's second part).
#[allow(clippy::too_many_arguments)]
fn contribution_products<const L: usize, const W: usize>(
    ctx_w: &mut WideContext<W>,
    r: &mut ExactWideSum,
    structure: &Structure,
    source: &PrimitiveSource,
    vs_ke: &[[[Wide<W>; 12]; 12]],
    vs_dir: &[[[Wide<W>; 3]; 3]],
    ordering: &Ordering,
    g: usize,
    free_values: &[Wide<L>],
    prescribed: Option<&[Vec<Wide<W>>]>,
) -> Result<(), AttemptStop>
where
    Wide<L>: SupportedWidth,
    Wide<W>: SupportedWidth,
{
    for index in structure.pattern.row_range(g) {
        let j = structure.pattern.column(index);
        let position = ordering.position[j];
        let operands: Vec<Wide<W>> = if position != usize::MAX {
            let v = free_values[position];
            if v.is_zero() {
                continue;
            }
            vec![v.widen::<W>()]
        } else {
            match prescribed {
                Some(terms) => terms[j].clone(),
                None => continue,
            }
        };
        if operands.is_empty() {
            continue;
        }
        let upper = if g <= j {
            index
        } else {
            structure.pattern.transpose(index)
        };
        for item in structure.contributions(upper) {
            let k = match *item {
                Contribution::Member { member, a, b } => {
                    vs_ke[member as usize][a as usize][b as usize]
                }
                Contribution::Spring { spring } => {
                    Wide::<W>::from_f64(source.springs()[spring as usize].stiffness)?
                }
                Contribution::Directional { spring, a, b } => {
                    vs_dir[spring as usize][a as usize][b as usize]
                }
            };
            if k.is_zero() {
                continue;
            }
            for u in &operands {
                r.add_product(ctx_w, &k, u, true)?;
            }
        }
    }
    Ok(())
}

/// max(a, b).
fn wmax<const L: usize>(a: &Wide<L>, b: &Wide<L>) -> Wide<L>
where
    Wide<L>: SupportedWidth,
{
    if b.cmp_value(a) == CmpOrdering::Greater {
        *b
    } else {
        *a
    }
}

/// The verification pass at one state (R7 §4.1.6.2 item 4, §4.1.6.3 items
/// 1–12): E, r, δ̂, Ŵ, r₂, the norms, ‖ā_q S‖₁, B_c (Uc_c and, where 7c needs
/// it, the shift), θ_c, the g check, t₁–t₃, C_q and W⁺.
#[allow(clippy::too_many_lines)]
pub(crate) fn verify_state<const L: usize, const R: usize, const W: usize>(
    shared: &Shared<L, R>,
    vs: &VerifyShared<L, W>,
    prep: &CasePrep,
    group: &GroupPrep,
    state: &Solved<L>,
    guard: StageGuard,
) -> VerifySpent<VerificationReport<L>>
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
    Wide<W>: SupportedWidth,
{
    let p = shared.p;
    let q_w = vs.q_w;
    let mut ctx = WideContext::<L>::new(p).expect("supported precision");
    let mut ctx_w = WideContext::<W>::new(q_w).expect("supported precision");
    let mut sum = ExactWideSum::new();
    let mut stages = StageWork::default();
    // T3 KF3: the stage in progress, for a stopped pass's unstaged work, and
    // the pass's S_c refusals (amendment A2), both kept on every path.
    let mut current = Stage::Scale;
    let mut refusals: Vec<BlockRefusal> = Vec::new();
    let mut run = || -> Result<VerificationReport<L>, AttemptStop> {
        let spent = |ctx: &WideContext<L>, ctx_w: &WideContext<W>, sum: &ExactWideSum| {
            lme(ctx) + lme(ctx_w) + sum.work().limb_multiply_equivalents()
        };
        let check = |ctx: &WideContext<L>, ctx_w: &WideContext<W>, sum: &ExactWideSum| {
            guard.test(spent(ctx, ctx_w, sum))
        };
        let source = &prep.source;
        let layout = &prep.layout;
        let structure = &group.structure;
        let ordering = &group.ordering;
        let blocks = &group.blocks;
        let free = &ordering.free;
        let nf = free.len();
        let scale = shared.factor.scale();
        let u = &state.u;
        let zero = Wide::<L>::ZERO;
        let t0 = spent(&ctx, &ctx_w, &sum);
        // ---- E (§4.1.6.2 item 4): |u| with the prescribed values as at P.
        let w_abs: Vec<Wide<L>> = u.iter().map(|v| v.abs()).collect();
        let e_rows = formation_scale(
            &mut ctx,
            &mut sum,
            &guard,
            source,
            layout,
            structure,
            &vs.abar,
            &shared.members,
            &shared.directional,
            Some(&prep.ledger),
            &w_abs,
            StageRounding::Nearest,
        )?;
        let bodies = source.body_count() as usize;
        let resolution = resolution_scale(layout, &e_rows, bodies)?;
        for (b, e) in resolution.iter().enumerate() {
            for (kind, v) in [(Kind::Force, e[0]), (Kind::Moment, e[1])] {
                if !v.is_finite() {
                    return Err(AttemptStop::ResolutionScale {
                        body: b as u32,
                        kind,
                    });
                }
            }
        }
        // E encodes for every body; then ê (ROOT's A3b ruling).
        resolution_hats(&resolution, &prep.extents)?;
        let t1 = spent(&ctx, &ctx_w, &sum);
        stages.scale = t1 - t0;
        current = Stage::Estimate;
        // ---- The exact prescribed values (item 1): each term c·v at q_W and
        // at P (both exact: at most 106 bits), and whether the sum is nonzero.
        let mut terms_w: Vec<Vec<Wide<W>>> = vec![Vec::new(); source.dof_count()];
        let mut terms_p: Vec<Vec<Wide<L>>> = vec![Vec::new(); source.dof_count()];
        let mut prescribed_nonzero = vec![false; source.dof_count()];
        let mut prescribed_negative = vec![false; source.dof_count()];
        for (g, list) in &prep.prescribed {
            sum.clear();
            for &(c, v) in list {
                if c == 0.0 || v == 0.0 {
                    continue;
                }
                let (cw, vw) = (Wide::<W>::from_f64(c)?, Wide::<W>::from_f64(v)?);
                let mut t = ExactWideSum::new();
                t.add_product(&mut ctx_w, &cw, &vw, false)?;
                let tw = t.round(&mut ctx_w)?;
                let (cl, vl) = (Wide::<L>::from_f64(c)?, Wide::<L>::from_f64(v)?);
                t.clear();
                t.add_product(&mut ctx, &cl, &vl, false)?;
                let tl = t.round(&mut ctx)?;
                sum.add_wide(&tl, false)?;
                terms_w[*g].push(tw);
                terms_p[*g].push(tl);
            }
            let sign = sum.signum();
            prescribed_nonzero[*g] = sign != 0;
            prescribed_negative[*g] = sign < 0;
            sum.clear();
        }
        // ---- r (item 1), r̂, ‖S·r‖ per row, over the q_W contributions.
        let u_free: Vec<Wide<L>> = free.iter().map(|&g| u[g]).collect();
        let mut r_hat = vec![zero; nf];
        let mut sr_row = vec![zero; nf];
        for (a, &g) in free.iter().enumerate() {
            let mut r = ExactWideSum::new();
            prep.ledger.add_to(g, &mut r, false)?;
            contribution_products(
                &mut ctx_w,
                &mut r,
                structure,
                source,
                &vs.ke_w,
                &vs.directional_w,
                ordering,
                g,
                &u_free,
                Some(&terms_w),
            )?;
            let mut rr = r.clone();
            r_hat[a] = r.round(&mut ctx)?;
            rr.make_absolute();
            sr_row[a] = round_toward(&mut ctx, &mut rr, Toward::Up)?.mul_pow2(scale[a])?;
            if a % 64 == 63 {
                check(&ctx, &ctx_w, &sum)?;
            }
        }
        // ---- δ̂ (item 2) and Ŵ (item 3): the recovery at P, ledger omitted.
        let delta = if nf > 0 {
            shared.factor.solve(&mut ctx, &r_hat)?
        } else {
            Vec::new()
        };
        let mut delta_full = vec![zero; source.dof_count()];
        for (a, &g) in free.iter().enumerate() {
            delta_full[g] = delta[a];
        }
        let empty = RetainedLedger::empty();
        let recovered = recover(
            &mut ctx,
            &mut sum,
            &guard,
            source,
            layout,
            structure,
            &shared.k,
            &shared.members,
            &shared.directional,
            &empty,
            &delta_full,
        )?;
        let w: Vec<Option<Wide<L>>> = layout
            .iter()
            .zip(&recovered.values)
            .map(|(meta, v)| matches!(meta.kind, Kind::Force | Kind::Moment).then(|| v.abs()))
            .collect();
        let t2 = spent(&ctx, &ctx_w, &sum);
        stages.estimate = t2 - t1;
        current = Stage::Charge;
        // ---- r₂ (item 5): r − K^c·δ̂, one exact expansion per row.
        let mut sr2_row = vec![zero; nf];
        for (a, &g) in free.iter().enumerate() {
            let mut r = ExactWideSum::new();
            prep.ledger.add_to(g, &mut r, false)?;
            contribution_products(
                &mut ctx_w,
                &mut r,
                structure,
                source,
                &vs.ke_w,
                &vs.directional_w,
                ordering,
                g,
                &u_free,
                Some(&terms_w),
            )?;
            contribution_products(
                &mut ctx_w,
                &mut r,
                structure,
                source,
                &vs.ke_w,
                &vs.directional_w,
                ordering,
                g,
                &delta,
                None,
            )?;
            r.make_absolute();
            sr2_row[a] = round_toward(&mut ctx, &mut r, Toward::Up)?.mul_pow2(scale[a])?;
            if a % 64 == 63 {
                check(&ctx, &ctx_w, &sum)?;
            }
        }
        // ---- The norms of item 6 over Ā's free rows.
        let mut sas_inf_row = vec![zero; nf];
        let mut sas_one_col = vec![zero; nf];
        let mut sau_row = vec![zero; nf];
        for (a, &g) in free.iter().enumerate() {
            let mut inf = ExactWideSum::new();
            let mut one = ExactWideSum::new();
            let mut au = ExactWideSum::new();
            for index in structure.pattern.row_range(g) {
                let j = structure.pattern.column(index);
                let b = ordering.position[j];
                let abar = &vs.abar[index];
                if b != usize::MAX {
                    // Row a's ∞-norm term s_a·Ā_aj·s_j; column a's 1-norm term
                    // s_j·Ā_ja·s_a (Ā_ja at the transposed entry).
                    inf.add_wide_scaled(abar, false, 1, scale[a] + scale[b])?;
                    let transposed = &vs.abar[structure.pattern.transpose(index)];
                    one.add_wide_scaled(transposed, false, 1, scale[a] + scale[b])?;
                    au.add_product(&mut ctx, abar, &u[j].abs(), false)?;
                } else {
                    for t in &terms_p[j] {
                        let negate = t.is_sign_negative() != prescribed_negative[j];
                        au.add_product(&mut ctx, abar, &t.abs(), negate)?;
                    }
                }
            }
            sas_inf_row[a] = round_toward(&mut ctx, &mut inf, Toward::Up)?;
            sas_one_col[a] = round_toward(&mut ctx, &mut one, Toward::Up)?;
            sau_row[a] = round_toward(&mut ctx, &mut au, Toward::Up)?.mul_pow2(scale[a])?;
            if a % 64 == 63 {
                check(&ctx, &ctx_w, &sum)?;
            }
        }
        // ---- ‖ā_q S‖₁: E's expansion with s at free DOFs, upward (Q2).
        let mut w_s = vec![zero; source.dof_count()];
        for (a, &g) in free.iter().enumerate() {
            w_s[g] = Wide::<L>::ONE.mul_pow2(scale[a])?;
        }
        let a_s = formation_scale(
            &mut ctx,
            &mut sum,
            &guard,
            source,
            layout,
            structure,
            &vs.abar,
            &shared.members,
            &shared.directional,
            None,
            &w_s,
            StageRounding::Up,
        )?;
        let t3 = spent(&ctx, &ctx_w, &sum);
        stages.charge = t3 - t2;
        current = Stage::Bound;
        // ---- B_c (7a–7d): data flags, the shift where 7c needs it.
        let data = data_blocks(
            blocks,
            ordering,
            structure,
            &prep.ledger,
            &prescribed_nonzero,
            u,
        );
        // Amendment A2: a refusal in 7c makes S_c unavailable for its block.
        let mut s_refused = vec![None; blocks.len()];
        #[cfg(test)]
        let hooked = hooks::estimates(&shared.est_blocks);
        #[cfg(test)]
        let est_blocks: &[Wide<L>] = &hooked;
        #[cfg(not(test))]
        let est_blocks: &[Wide<L>] = &shared.est_blocks;
        let start = shift_start(
            &mut ctx,
            &mut sum,
            blocks,
            &vs.uc,
            est_blocks,
            &data,
            &mut s_refused,
        )?;
        let t4 = spent(&ctx, &ctx_w, &sum);
        stages.bound = t4 - t3;
        current = Stage::Shift;
        let shifted = shift_run(
            &mut ctx,
            &mut sum,
            &guard,
            structure,
            &shared.k,
            ordering,
            shared.factor.scale(),
            blocks,
            &vs.gamma,
            &start,
            &mut s_refused,
        );
        refusals = block_refusals::<L>(&[], &s_refused);
        let (shifts, shift_factorizations) = shifted?;
        let t5 = spent(&ctx, &ctx_w, &sum);
        stages.shift = t5 - t4;
        current = Stage::Bound;
        // 7d with A2: a block with data left with no bound after a refusal
        // stops the attempt with that refusal, before any `uc` rejection
        // (ROOT's rulings 1 and 2 on I19's plan).
        let (certificates, uc_missing, stop) =
            certificates(blocks, &vs.uc, est_blocks, &data, &shifts, &s_refused);
        if let Some((_, refusal)) = stop {
            return Err(refusal.stop());
        }
        // Per-block norms.
        let mut norms = Vec::with_capacity(blocks.len());
        for positions in &blocks.positions {
            let mut n = BlockNorms {
                sas: zero,
                sas_one: zero,
                sas_inf: zero,
                sau: zero,
                sr: zero,
                sr2: zero,
                sid: zero,
            };
            for &a in positions {
                n.sas_one = wmax(&n.sas_one, &sas_one_col[a]);
                n.sas_inf = wmax(&n.sas_inf, &sas_inf_row[a]);
                n.sau = wmax(&n.sau, &sau_row[a]);
                n.sr = wmax(&n.sr, &sr_row[a]);
                n.sr2 = wmax(&n.sr2, &sr2_row[a]);
                n.sid = wmax(&n.sid, &delta[a].abs().mul_pow2(-scale[a])?);
            }
            n.sas = wmax(&n.sas_one, &n.sas_inf);
            norms.push(n);
        }
        // θ_c (item 9) on the blocks with data.
        let mut theta = vec![None; blocks.len()];
        for b in 0..blocks.len() {
            if let Some(bc) = &certificates[b].b {
                let t = mul_toward(&mut ctx, &mut sum, bc, &norms[b].sas, Toward::Up)?;
                theta[b] = Some(t.mul_pow2(7 - i64::from(p))?);
            }
        }
        // The g check (item 10).
        let mut g_max = 0u32;
        let mut g_violation = None;
        let limit = p - 16;
        for (m, op) in source.members().iter().zip(&shared.members) {
            let dofs = op.dofs();
            let in_scope = dofs.iter().any(|&d| {
                let b = ordering.position[d];
                if b == usize::MAX {
                    prescribed_nonzero[d]
                } else {
                    data[blocks.of[b] as usize]
                }
            });
            if in_scope {
                g_max = g_max.max(op.g_exp);
                if op.g_exp > limit && g_violation.is_none() {
                    g_violation = Some(m.id);
                }
            }
        }
        // Per body (item 11): B_b and the norms over its blocks with data.
        let mut body_reports = Vec::with_capacity(bodies);
        for body in 0..bodies {
            let mut bb: Option<Wide<L>> = None;
            let mut theta_b = zero;
            let mut n = BlockNorms {
                sas: zero,
                sas_one: zero,
                sas_inf: zero,
                sau: zero,
                sr: zero,
                sr2: zero,
                sid: zero,
            };
            for b in 0..blocks.len() {
                if blocks.body[b] as usize != body || !data[b] {
                    continue;
                }
                if let Some(bc) = &certificates[b].b {
                    bb = Some(bb.map_or(*bc, |x| wmax(&x, bc)));
                }
                if let Some(t) = &theta[b] {
                    theta_b = wmax(&theta_b, t);
                }
                let nb = &norms[b];
                n.sas = wmax(&n.sas, &nb.sas);
                n.sas_one = wmax(&n.sas_one, &nb.sas_one);
                n.sas_inf = wmax(&n.sas_inf, &nb.sas_inf);
                n.sau = wmax(&n.sau, &nb.sau);
                n.sr = wmax(&n.sr, &nb.sr);
                n.sr2 = wmax(&n.sr2, &nb.sr2);
                n.sid = wmax(&n.sid, &nb.sid);
            }
            let b_val = bb.unwrap_or(zero);
            // N_u = ‖SĀ|u⁰|‖ + 2·B_b·‖SĀS‖·‖S·r‖; t₁ = 2^(7−q_W)·B_b·N_u.
            let x = mul_toward(&mut ctx, &mut sum, &b_val.mul_pow2(1)?, &n.sas, Toward::Up)?;
            let y = mul_toward(&mut ctx, &mut sum, &x, &n.sr, Toward::Up)?;
            let n_u = add_toward(&mut ctx, &mut sum, &n.sau, &y, Toward::Up)?;
            let t1 = mul_toward(&mut ctx, &mut sum, &b_val, &n_u, Toward::Up)?
                .mul_pow2(7 - i64::from(q_w))?;
            // t₂ = 70·2^-P·‖S⁻¹δ̂‖; t₃ = 3·B_b·‖S·r₂‖.
            let seventy = Wide::<L>::from_f64(70.0)?;
            let t2 = mul_toward(&mut ctx, &mut sum, &seventy, &n.sid, Toward::Up)?
                .mul_pow2(-i64::from(p))?;
            let three = Wide::<L>::from_f64(3.0)?;
            let three_b = mul_toward(&mut ctx, &mut sum, &three, &b_val, Toward::Up)?;
            let t3 = mul_toward(&mut ctx, &mut sum, &three_b, &n.sr2, Toward::Up)?;
            body_reports.push(BodyReport {
                b: bb,
                theta: theta_b,
                norms: n,
                n_u,
                t1,
                t2,
                t3,
            });
        }
        let t6 = spent(&ctx, &ctx_w, &sum);
        stages.bound = (t4 - t3) + (t6 - t5);
        current = Stage::Charge;
        // C_q (item 11) and W⁺ (item 12), unless a block with data lacks B.
        let mut charge = vec![None; layout.len()];
        let mut w_plus = vec![None; layout.len()];
        if uc_missing.is_none() {
            for (index, meta) in layout.iter().enumerate() {
                let body = &body_reports[meta.body as usize];
                match (meta.kind, meta.id) {
                    (Kind::Force | Kind::Moment, _) => {
                        let Some(sa) = &a_s[index] else { continue };
                        sum.clear();
                        sum.add_product(&mut ctx, sa, &body.t1, false)?;
                        sum.add_product(&mut ctx, sa, &body.t2, false)?;
                        sum.add_product(&mut ctx, sa, &body.t3, false)?;
                        charge[index] = Some(round_toward(&mut ctx, &mut sum, Toward::Up)?);
                    }
                    (_, QuantityId::Displacement(dof)) if !meta.input_derived => {
                        let a = ordering.position[dof.global()];
                        sum.clear();
                        sum.add_wide(&delta[a].abs(), false)?;
                        sum.add_wide_scaled(&body.t1, false, 1, scale[a])?;
                        sum.add_wide_scaled(&body.t3, false, 1, scale[a])?;
                        w_plus[index] = Some(round_toward(&mut ctx, &mut sum, Toward::Up)?);
                    }
                    (_, QuantityId::DisplacementMagnitude(node)) => {
                        sum.clear();
                        for c in 0..3 {
                            let g = node as usize * DOF_PER_NODE + c;
                            let a = ordering.position[g];
                            if a != usize::MAX {
                                sum.add_wide(&delta[a].abs(), false)?;
                                sum.add_wide_scaled(&body.t1, false, 1, scale[a])?;
                                sum.add_wide_scaled(&body.t3, false, 1, scale[a])?;
                            } else {
                                // |u_P,c − Σ c·v|, exact.
                                let mut d = ExactWideSum::new();
                                d.add_wide(&u[g], false)?;
                                for t in &terms_p[g] {
                                    d.add_wide(t, true)?;
                                }
                                d.make_absolute();
                                sum.add_scaled(&d, false, 1, 0)?;
                            }
                        }
                        w_plus[index] = Some(round_toward(&mut ctx, &mut sum, Toward::Up)?);
                    }
                    _ => {}
                }
            }
        }
        let t7 = spent(&ctx, &ctx_w, &sum);
        stages.charge = (t3 - t2) + (t7 - t6);
        check(&ctx, &ctx_w, &sum)?;
        Ok(VerificationReport {
            precision: p,
            q_w,
            resolution,
            e_rows,
            w,
            a_s,
            charge,
            w_plus,
            r_hat,
            delta,
            blocks: certificates,
            norms,
            theta,
            bodies: body_reports,
            g_max,
            g_violation,
            uc_missing,
            shift_factorizations,
        })
    };
    let result = run();
    let mut work = AttemptWork::default();
    work.record(&ctx);
    work.record(&ctx_w);
    let sum_work = sum.work();
    let total = work.limb_multiply_equivalents() + sum_work.limb_multiply_equivalents();
    stages.close_stopped(&result, current, total);
    VerifySpent {
        result,
        work,
        sum_work,
        stages,
        total,
        refusals,
    }
}

/// A test-only hook (T3 KF3; ROOT's ruling 5 on I19's plan, V-K's `seeded`
/// module not being on main): with it set, every block's est_c reads as 0, so
/// no shift runs and S_c does not exist (the "neither bound" control, W2).
#[cfg(test)]
pub(crate) mod hooks {
    use super::super::wide::multi::SupportedWidth;
    use super::super::wide::Wide;
    use std::cell::Cell;

    thread_local! {
        static NO_SHIFT: Cell<bool> = const { Cell::new(false) };
    }

    /// Sets or clears this thread's hook.
    pub(crate) fn set_no_shift(on: bool) {
        NO_SHIFT.with(|h| h.set(on));
    }

    /// est_c as the pass reads it: zero on every block while the hook is set.
    pub(crate) fn estimates<const L: usize>(est: &[Wide<L>]) -> Vec<Wide<L>>
    where
        Wide<L>: SupportedWidth,
    {
        if NO_SHIFT.with(Cell::get) {
            vec![Wide::<L>::ZERO; est.len()]
        } else {
            est.to_vec()
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/verify_tests.rs"]
mod tests;
