//! K4: recovery before rounding (T3 D1 §4.1.5) and publication (§5 item 7).
//!
//! For each member, at p: d_local = T u; e = B_local d_local; Q = D e; end
//! actions (node-on-element, local) = B_localᵀ Q; station actions (linear for
//! nodal loads). Also at p: spring actions −k u, reactions (K u − f) on the
//! constrained rows, the node displacement magnitudes and the support groups'
//! force and moment magnitudes (ROOT's K4 ruling O2). "Each component of
//! d_local, e, Q and the end actions is one exact expansion of its (at most
//! five) product terms, rounded once. Each reaction is one exact expansion of
//! K_cj·u_j products and the ledger terms, rounded once."
//!
//! A station at fraction t publishes the "j-end action of the sub-member i→x":
//! (N, V_y, V_z, T, M_y(t), M_z(t)) with M_z(t) = t·Q3 + t·Q2 − Q2 and
//! M_y(t) = t·Q5 + t·Q4 − Q4; at t = 1 it is the j-end action and at t = 0
//! minus the i-end action.
//!
//! Publication: "Each published quantity is rounded to binary64 once", with K3's
//! `Binary64Outcome` (ROOT's K4 ruling Q11); an exact zero is +0.0 (D1 §4.1.2).
use super::adaptive::{AttemptStop, StageGuard};
use super::assemble::{DirectionalBlock, MemberOperators, Structure};
use super::ledger::RetainedLedger;
use super::source::{put_i64, put_u32, put_u64, Component, Dof, PrimitiveSource, SpringKind};
use super::wide::multi::{Binary64Outcome, SupportedWidth, WideContext};
use super::wide::Wide;
use super::wide_sum::ExactWideSum;
use crate::DOF_PER_NODE;

/// The kinds of D1 §4.1.6 that K4's quantities belong to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Translation,
    Rotation,
    Force,
    Moment,
}

impl Kind {
    pub(crate) const ALL: [Kind; 4] =
        [Kind::Translation, Kind::Rotation, Kind::Force, Kind::Moment];
    pub(crate) fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum End {
    I,
    J,
}

/// A published quantity of a case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QuantityId {
    Displacement(Dof),
    DisplacementMagnitude(u32),
    EndAction {
        member: u32,
        end: End,
        component: Component,
    },
    StationAction {
        station: u32,
        component: Component,
    },
    SpringAction {
        spring: u32,
        component: Component,
    },
    DirectionalSpringAction {
        spring: u32,
        component: Component,
    },
    Reaction(Dof),
    SupportForceMagnitude(u32),
    SupportMomentMagnitude(u32),
}

/// A quantity's place in the published set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuantityMeta {
    pub id: QuantityId,
    pub kind: Kind,
    pub body: u32,
    /// Rule 2a: a displacement or rotation at a restrained or prescribed DOF.
    pub input_derived: bool,
}

fn component_kind(component: Component, motion: bool) -> Kind {
    match (component.is_translation(), motion) {
        (true, true) => Kind::Translation,
        (false, true) => Kind::Rotation,
        (true, false) => Kind::Force,
        (false, false) => Kind::Moment,
    }
}

/// The published set in canonical order: displacements (all DOFs), node
/// displacement magnitudes, end actions (by member id, end, component), station
/// actions, spring actions, directional spring actions, reactions (constrained
/// DOFs), support magnitudes.
pub fn layout(source: &PrimitiveSource) -> Vec<QuantityMeta> {
    let mut out = Vec::new();
    let body = |node: u32| source.body_of_node(node);
    for g in 0..source.dof_count() {
        let dof = Dof::from_global(g);
        out.push(QuantityMeta {
            id: QuantityId::Displacement(dof),
            kind: component_kind(dof.component, true),
            body: body(dof.node),
            input_derived: source.constraint(g).is_some(),
        });
    }
    for node in 0..source.node_count() as u32 {
        out.push(QuantityMeta {
            id: QuantityId::DisplacementMagnitude(node),
            kind: Kind::Translation,
            body: body(node),
            input_derived: false,
        });
    }
    for m in source.members() {
        for end in [End::I, End::J] {
            for component in Component::ALL {
                out.push(QuantityMeta {
                    id: QuantityId::EndAction {
                        member: m.id,
                        end,
                        component,
                    },
                    kind: component_kind(component, false),
                    body: body(m.node_i),
                    input_derived: false,
                });
            }
        }
    }
    for s in source.stations() {
        let m = &source.members()[source.member_index(s.member).unwrap()];
        for component in Component::ALL {
            out.push(QuantityMeta {
                id: QuantityId::StationAction {
                    station: s.id,
                    component,
                },
                kind: component_kind(component, false),
                body: body(m.node_i),
                input_derived: false,
            });
        }
    }
    for s in source.springs() {
        out.push(QuantityMeta {
            id: QuantityId::SpringAction {
                spring: s.id,
                component: s.dof.component,
            },
            kind: component_kind(s.dof.component, false),
            body: body(s.dof.node),
            input_derived: false,
        });
    }
    for s in source.directional_springs() {
        for k in 0..3 {
            let component = Component::from_index(s.kind.offset() + k);
            out.push(QuantityMeta {
                id: QuantityId::DirectionalSpringAction {
                    spring: s.id,
                    component,
                },
                kind: component_kind(component, false),
                body: body(s.node),
                input_derived: false,
            });
        }
    }
    for c in source.constraints() {
        out.push(QuantityMeta {
            id: QuantityId::Reaction(c.dof),
            kind: component_kind(c.dof.component, false),
            body: body(c.dof.node),
            input_derived: false,
        });
    }
    for g in source.supports() {
        out.push(QuantityMeta {
            id: QuantityId::SupportForceMagnitude(g.id),
            kind: Kind::Force,
            body: body(g.node),
            input_derived: false,
        });
        out.push(QuantityMeta {
            id: QuantityId::SupportMomentMagnitude(g.id),
            kind: Kind::Moment,
            body: body(g.node),
            input_derived: false,
        });
    }
    out
}

/// The recovered quantities at one precision.
#[derive(Debug, Clone)]
pub(crate) struct Recovered<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    /// Q = (N, T, M_zi, M_zj, M_yi, M_yj) per member (id order).
    pub(crate) q: Vec<[Wide<L>; 6]>,
    /// Aligned with `layout`.
    pub(crate) values: Vec<Wide<L>>,
}

fn lift<const L: usize>(x: f64) -> Result<Wide<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    Ok(Wide::<L>::from_f64(x)?)
}

#[derive(Clone, Copy)]
struct LocalLoad<const L: usize> where Wide<L>: SupportedWidth {
    wl: [Wide<L>; 3], wll: [Wide<L>; 3], h: [Wide<L>; 3],
}
fn local_load<const L: usize>(ctx: &mut WideContext<L>, sum: &mut ExactWideSum,
    ledger: &RetainedLedger, member: u32, op: &MemberOperators<L>) -> Result<Option<LocalLoad<L>>, AttemptStop>
where Wide<L>: SupportedWidth {
    if !ledger.member_is_loaded(member) { return Ok(None); }
    let mut wl = [Wide::<L>::ZERO; 3];
    let mut wll = wl; let mut h = wl;
    for r in 0..3 {
        sum.clear();
        for term in ledger.declared.iter().filter(|d| d.member == member) {
            for c in 0..3 { sum.add_product(ctx, &op.axes[r][c], &lift::<L>(f64::from_bits(term.intensity_bits[c]))?, false)?; }
        }
        let w = sum.round(ctx)?;
        wl[r] = ctx.mul(&w, &op.length)?;
        wll[r] = ctx.mul(&wl[r], &op.length)?;
        h[r] = ctx.div(&wll[r], &lift::<L>(12.0)?)?;
    }
    Ok(Some(LocalLoad { wl, wll, h }))
}
fn canonical<const L: usize>(x: Wide<L>) -> Wide<L> where Wide<L>: SupportedWidth {
    if x.is_zero() { Wide::<L>::ZERO } else { x }
}

/// fl_p(√(fl_p(Σ v_k²))), v the given values (exact sum, rounded once, then √).
fn magnitude<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    v: &[Wide<L>],
) -> Result<Wide<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    sum.clear();
    for x in v {
        sum.add_product(ctx, x, x, false)?;
    }
    let squared = sum.round(ctx)?;
    Ok(ctx.sqrt(&squared)?)
}

/// Every published quantity at the context's precision, from u (all DOFs).
#[allow(clippy::too_many_arguments)]
pub(crate) fn recover<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    source: &PrimitiveSource,
    layout: &[QuantityMeta],
    structure: &Structure,
    k: &[Wide<L>],
    members: &[MemberOperators<L>],
    directional: &[DirectionalBlock<L>],
    ledger: &RetainedLedger,
    u: &[Wide<L>],
) -> Result<Recovered<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let zero = Wide::<L>::ZERO;
    // V-K seeded fault VK-F02 (§7.3-2): u rounded to binary64 before recovery.
    #[cfg(any(test, feature = "mutation-controls"))]
    let seeded_u: Vec<Wide<L>>;
    #[cfg(any(test, feature = "mutation-controls"))]
    let u: &[Wide<L>] = if super::seeded::active(super::seeded::Fault::F02) {
        let mut rounded = Vec::with_capacity(u.len());
        for w in u {
            rounded.push(lift::<L>(w.to_binary64().value().unwrap_or(0.0))?);
        }
        seeded_u = rounded;
        &seeded_u
    } else {
        u
    };
    let mut values: Vec<Wide<L>> = Vec::with_capacity(layout.len());
    // Displacements, then node magnitudes.
    values.extend_from_slice(u);
    for node in 0..source.node_count() {
        let base = node * DOF_PER_NODE;
        values.push(magnitude(ctx, sum, &u[base..base + 3])?);
    }
    // Members.
    let mut q_all = Vec::with_capacity(members.len());
    let mut end_actions: Vec<[Wide<L>; 12]> = Vec::with_capacity(members.len());
    let mut local_loads = if ledger.declared.is_empty() { Vec::new() } else { Vec::with_capacity(members.len()) };
    for (mi, m) in members.iter().enumerate() {
        let load = local_load(ctx, sum, ledger, source.members()[mi].id, m)?;
        let dofs = m.dofs();
        // d_local = T u: block b (tr_i, rot_i, tr_j, rot_j), local axis r.
        let mut d = [zero; 12];
        for block in 0..4 {
            for r in 0..3 {
                sum.clear();
                for c in 0..3 {
                    sum.add_product(ctx, &m.axes[r][c], &u[dofs[3 * block + c]], false)?;
                }
                d[3 * block + r] = sum.round(ctx)?;
            }
        }
        let inv = &m.inv_length;
        let mut e = [zero; 6];
        // e0 = u_j − u_i, e1 = θx_j − θx_i (local).
        for (index, (plus, minus)) in [(6usize, 0usize), (9, 3)].into_iter().enumerate() {
            sum.clear();
            sum.add_wide(&d[plus], false)?;
            sum.add_wide(&d[minus], true)?;
            e[index] = sum.round(ctx)?;
        }
        // e2, e3 = θz − (v_j − v_i)/L; e4, e5 = θy + (w_j − w_i)/L.
        for (index, rotation, transverse, sign) in [
            (2usize, 5usize, 1usize, true),
            (3, 11, 1, true),
            (4, 4, 2, false),
            (5, 10, 2, false),
        ] {
            sum.clear();
            sum.add_wide(&d[rotation], false)?;
            // θz: + (v_i − v_j)/L; θy: − (w_i − w_j)/L.
            sum.add_product(ctx, inv, &d[transverse], !sign)?;
            sum.add_product(ctx, inv, &d[transverse + 6], sign)?;
            e[index] = sum.round(ctx)?;
        }
        // Q = D·e: Euler–Bernoulli's exact (4c, 2c), or a Timoshenko member's
        // signed (d₁, d₂) (O4 §4.2).
        let (four_z, two_z) = m.bending(0)?;
        let (four_y, two_y) = m.bending(1)?;
        let mut q = [zero; 6];
        for (index, terms) in [
            [(m.axial, 0usize), (zero, 0)],
            [(m.torsion, 1), (zero, 1)],
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
            q[index] = sum.round(ctx)?;
        }
        // End actions B_localᵀ Q.
        let shear = |ctx: &mut WideContext<L>,
                     sum: &mut ExactWideSum,
                     a: &Wide<L>,
                     b: &Wide<L>,
                     negate: bool|
         -> Result<Wide<L>, AttemptStop> {
            sum.clear();
            sum.add_product(ctx, inv, a, negate)?;
            sum.add_product(ctx, inv, b, negate)?;
            Ok(sum.round(ctx)?)
        };
        let vy_i = shear(ctx, sum, &q[2], &q[3], false)?;
        let vz_i = shear(ctx, sum, &q[4], &q[5], true)?;
        let mut actions = [
            q[0].neg(),
            vy_i,
            vz_i,
            q[1].neg(),
            q[4],
            q[2],
            q[0],
            vy_i.neg(),
            vz_i.neg(),
            q[1],
            q[5],
            q[3],
        ];
        if let Some(load) = load {
            for a in [0usize, 1, 2, 4, 5, 6, 7, 8, 10, 11] {
                sum.clear();
                match a {
                    0 => sum.add_wide(&q[0], true)?,
                    6 => sum.add_wide(&q[0], false)?,
                    1 | 7 => { sum.add_product(ctx, inv, &q[2], a == 7)?; sum.add_product(ctx, inv, &q[3], a == 7)?; }
                    2 | 8 => { sum.add_product(ctx, inv, &q[4], a == 2)?; sum.add_product(ctx, inv, &q[5], a == 2)?; }
                    4 => sum.add_wide(&q[4], false)?, 10 => sum.add_wide(&q[5], false)?,
                    5 => sum.add_wide(&q[2], false)?, 11 => sum.add_wide(&q[3], false)?, _ => unreachable!(),
                }
                if a % 6 < 3 { sum.add_wide(&load.wl[a % 6].mul_pow2(-1)?, true)?; }
                else { let axis = if a % 6 == 4 { 2 } else { 1 };
                    sum.add_wide(&load.h[axis], matches!(a, 5 | 10))?; }
                actions[a] = canonical(sum.round(ctx)?);
            }
        }
        if !ledger.declared.is_empty() { local_loads.push(load); }
        end_actions.push(actions);
        q_all.push(q);
        guard.check(ctx, sum)?;
    }
    for actions in &end_actions {
        values.extend_from_slice(actions);
    }
    // Stations.
    for s in source.stations() {
        let index = source.member_index(s.member).unwrap();
        let q = &q_all[index];
        let actions = &end_actions[index];
        if let Some(load) = local_loads.get(index).copied().flatten() {
            if s.fraction == 0.0 || s.fraction == 1.0 {
                let start = if s.fraction == 0.0 { 0 } else { 6 };
                for c in 0..6 { values.push(canonical(if start == 0 { actions[c].neg() } else { actions[6 + c] })); }
                continue;
            }
            let t = lift::<L>(s.fraction)?;
            let t2 = ctx.mul(&t, &t)?; // exact: a binary64 square has <=106 bits.
            for c in 0..3 {
                sum.clear();
                if c == 0 { sum.add_wide(&q[0], false)?; }
                else { let pair = if c == 1 { (2,3,true) } else { (4,5,false) };
                    sum.add_product(ctx, &members[index].inv_length, &q[pair.0], pair.2)?;
                    sum.add_product(ctx, &members[index].inv_length, &q[pair.1], pair.2)?; }
                sum.add_wide(&load.wl[c].mul_pow2(-1)?, false)?;
                sum.add_product(ctx, &load.wl[c], &t, true)?;
                values.push(canonical(sum.round(ctx)?));
            }
            values.push(canonical(q[1]));
            for (a,b,c,negative) in [(5usize,4usize,2usize,false),(3,2,1,true)] {
                sum.clear();
                sum.add_product(ctx, &t, &q[a], false)?;
                sum.add_product(ctx, &t, &q[b], false)?;
                sum.add_wide(&q[b], true)?;
                sum.add_wide(&load.h[c], !negative)?;
                let half = load.wll[c].mul_pow2(-1)?;
                sum.add_product(ctx, &half, &t, negative)?;
                sum.add_product(ctx, &half, &t2, !negative)?;
                values.push(canonical(sum.round(ctx)?));
            }
            continue;
        }
        let t = lift::<L>(s.fraction)?;
        let moment = |ctx: &mut WideContext<L>,
                      sum: &mut ExactWideSum,
                      at_j: &Wide<L>,
                      at_i: &Wide<L>|
         -> Result<Wide<L>, AttemptStop> {
            sum.clear();
            sum.add_product(ctx, &t, at_j, false)?;
            sum.add_product(ctx, &t, at_i, false)?;
            sum.add_wide(at_i, true)?;
            Ok(sum.round(ctx)?)
        };
        let my = moment(ctx, sum, &q[5], &q[4])?;
        let mz = moment(ctx, sum, &q[3], &q[2])?;
        values.extend_from_slice(&[actions[6], actions[7], actions[8], actions[9], my, mz]);
    }
    // Spring actions.
    let mut spring_action = Vec::with_capacity(source.springs().len());
    for s in source.springs() {
        sum.clear();
        sum.add_product(ctx, &lift::<L>(s.stiffness)?, &u[s.dof.global()], true)?;
        let action = sum.round(ctx)?;
        spring_action.push(action);
        values.push(action);
    }
    let mut directional_action = Vec::with_capacity(directional.len());
    for block in directional {
        let base = block.node as usize * DOF_PER_NODE + block.kind.offset();
        let mut components = [zero; 3];
        for a in 0..3 {
            sum.clear();
            for b in 0..3 {
                sum.add_product(ctx, &block.k[a][b], &u[base + b], true)?;
            }
            components[a] = sum.round(ctx)?;
        }
        values.extend_from_slice(&components);
        directional_action.push(components);
    }
    // Reactions: Σ_j K_cj·u_j − f_c, exact (the ledger enters exactly).
    let mut reaction = vec![None; source.dof_count()];
    for c in source.constraints() {
        let g = c.dof.global();
        sum.clear();
        for index in structure.pattern.row_range(g) {
            let j = structure.pattern.column(index);
            sum.add_product(ctx, &k[index], &u[j], false)?;
        }
        ledger.add_to(g, sum, true)?;
        let r = sum.round(ctx)?;
        // V-K seeded fault VK-S2: the reaction with the opposite sign.
        #[cfg(any(test, feature = "mutation-controls"))]
        let r = if super::seeded::active(super::seeded::Fault::S2) {
            r.neg()
        } else {
            r
        };
        reaction[g] = Some(r);
        values.push(r);
    }
    guard.check(ctx, sum)?;
    // Support groups: components as exact sums of the group's reactions and
    // spring actions (each already rounded once), then the magnitudes.
    for group in source.supports() {
        let mut components = [zero; 6];
        for (c, component) in components.iter_mut().enumerate() {
            let g = group.node as usize * DOF_PER_NODE + c;
            sum.clear();
            if group.restrained[c] {
                if let Some(r) = &reaction[g] {
                    sum.add_wide(r, false)?;
                }
            }
            for id in &group.springs {
                let k = source.springs().binary_search_by_key(id, |s| s.id).unwrap();
                if source.springs()[k].dof.component.index() == c {
                    sum.add_wide(&spring_action[k], false)?;
                }
            }
            for id in &group.directional_springs {
                let k = source
                    .directional_springs()
                    .binary_search_by_key(id, |s| s.id)
                    .unwrap();
                let s = &source.directional_springs()[k];
                let offset = s.kind.offset();
                if (offset..offset + 3).contains(&c) {
                    sum.add_wide(&directional_action[k][c - offset], false)?;
                }
            }
            *component = sum.round(ctx)?;
        }
        values.push(magnitude(ctx, sum, &components[..3])?);
        values.push(magnitude(ctx, sum, &components[3..])?);
    }
    debug_assert_eq!(values.len(), layout.len());
    let _ = SpringKind::Translation;
    Ok(Recovered { q: q_all, values })
}

/// The binary64 publication of one quantity: an exact zero is +0.0 (D1
/// §4.1.2); otherwise K3's `to_binary64`, rounded once, with its outcome.
pub(crate) fn publish_value<const L: usize>(q: &Wide<L>) -> Binary64Outcome
where
    Wide<L>: SupportedWidth,
{
    if q.is_zero() {
        Binary64Outcome::Normal(0.0)
    } else {
        q.to_binary64()
    }
}

/// The canonical retained-state encoding (D1 §4.1.8: "the canonical limbs of
/// u_p and every member's Q"; ROOT's K4 ruling Q10): p, L, then u for every
/// DOF ascending and every member's Q (id order), each as K3's `parts()` with
/// a zero's sign normalized to +.
pub(crate) fn state_encoding<const L: usize>(p: u32, u: &[Wide<L>], q: &[[Wide<L>; 6]]) -> Vec<u8>
where
    Wide<L>: SupportedWidth,
{
    let mut out = b"K4RST\x01".to_vec();
    put_u32(&mut out, p);
    put_u32(&mut out, L as u32);
    let value = |out: &mut Vec<u8>, w: &Wide<L>| {
        let (negative, exponent, significand) = w.parts();
        out.push(u8::from(negative && !w.is_zero()));
        put_i64(out, exponent);
        for limb in significand {
            put_u64(out, limb);
        }
    };
    put_u32(&mut out, u.len() as u32);
    for w in u {
        value(&mut out, w);
    }
    put_u32(&mut out, q.len() as u32);
    for member in q {
        for w in member {
            value(&mut out, w);
        }
    }
    out
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/recover_tests.rs"]
mod tests;
