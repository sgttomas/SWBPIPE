//! K4: formation, assembly and reduction at precision p (T3 D1 §4.1.2 items
//! 1–4 and 6).
//!
//! "Every operation below is rounded to p bits, and every input is a binary64
//! value lifted exactly." Every multi-term sum is one exact expansion rounded
//! once (`ExactWideSum`), including the frame's dot products (ROOT's K4 ruling:
//! a refinement of D1's letter, strictly more accurate, one rounding per step).
//!
//! 1. **Frame** (the product's algorithm, `FK/lib.rs:525-539`): d = x_j − x_i,
//!    L = √(d·d), e_x = d/L; e_y from the Gram–Schmidt of y_reference, e_z from
//!    e_x × e_y, each normalized. "No axis tolerance is used at p."
//! 2. **Basic deformations**: B = B_local·T (6×12, global), rows: extension,
//!    twist, θz_i and θz_j relative to the chord, θy_i and θy_j relative to the
//!    chord, with 1/L.
//! 3. **Constitutive operator** D = diag(EA/L, GJ/L) ⊕ (EI_z/L)[[4,2],[2,4]] ⊕
//!    (EI_y/L)[[4,2],[2,4]]; the 4 and 2 are exact power-of-two scalings.
//!    DB is one exact expansion (≤ 2 products) per entry, and K_e = Bᵀ(DB) one
//!    exact expansion (≤ 6 products) per upper-triangle entry, mirrored.
//! 4. **Assembly (exact)**: "Each pattern entry of K is formed as one exact
//!    expansion of its p-bit element contributions and binary64 spring
//!    stiffnesses … rounded once to p", on K1's `SparsePattern`. A directional
//!    spring (ROOT's K4 ruling Q6) contributes k·n nᵀ/(nᵀn), formed at p from its
//!    binary64 direction as fl(fl(k·n_a n_b)/fl(nᵀn)).
//! 6. **Reduction (exact)**: "Each rhs_i is one exact expansion: the ledger
//!    terms plus the exact products −K_ic·u_c … rounded once to p. Neither K nor
//!    rhs is ever rounded back to binary64."
//!
//! **D1 revision 5a.3 (R7 §4.1.6.2 items 1–3).** Each formation also forms
//! g_m = 2^k, the least k ≥ 0 with 4^k·(y_c·y_c) ≥ y_ref·y_ref, both dot
//! products exact and the comparison exact (y_c is the Gram–Schmidt residual
//! above, before normalization). The bounded operator of a member is g·B̄ᵀ(|D|B̄),
//! with B̄ = 1 at every axis-component position and |1/L| at the bending rows'
//! translation entries, stage-rounded (|D|B̄ one exact sum per entry, rounded
//! once; then B̄ᵀ(|D|B̄) likewise; then the exact scaling by g). Ā is its
//! assembly: one exact sum per pattern entry, rounded once, of the element
//! blocks, |k| of global-axis springs and the directional blocks' formed entries
//! in absolute value, with both triangles formed separately.
use super::adaptive::{AttemptStop, StageGuard};
use super::ledger::RetainedLedger;
use super::source::{PrimitiveSource, SpringKind, StraightMember};
use super::wide::multi::{SupportedWidth, WideContext};
use super::wide::Wide;
use super::wide_sum::ExactWideSum;
use crate::structural::SparsePattern;
use crate::DOF_PER_NODE;

/// Row-major offsets of the 12×12 upper triangle (78 entries).
const UPPER_START: [usize; 12] = {
    let mut starts = [0usize; 12];
    let mut a = 1;
    while a < 12 {
        starts[a] = starts[a - 1] + (12 - (a - 1));
        a += 1;
    }
    starts
};

fn upper_index(a: usize, b: usize) -> usize {
    let (a, b) = if a <= b { (a, b) } else { (b, a) };
    UPPER_START[a] + (b - a)
}

/// A member's operators at one precision.
#[derive(Debug, Clone)]
pub(crate) struct MemberOperators<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    pub(crate) node_i: u32,
    pub(crate) node_j: u32,
    /// Rows: e_x, e_y, e_z.
    pub(crate) axes: [[Wide<L>; 3]; 3],
    pub(crate) inv_length: Wide<L>,
    /// EA/L, GJ/L, EI_z/L, EI_y/L.
    pub(crate) axial: Wide<L>,
    pub(crate) torsion: Wide<L>,
    pub(crate) bend_z: Wide<L>,
    pub(crate) bend_y: Wide<L>,
    /// B = B_local·T.
    #[allow(dead_code)] // read by the rigid-mode tests (E) and kept as evidence
    pub(crate) b: [[Wide<L>; 12]; 6],
    ke: [Wide<L>; 78],
    /// g_m = 2^g_exp (D1 revision 5a.3, R7 §4.1.6.2 item 2).
    pub(crate) g_exp: u32,
}

/// The coefficients a member's bounded operator needs (R7 §4.1.6.2 items 1–2).
#[derive(Debug, Clone)]
pub(crate) struct BoundedCoefficients<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    pub(crate) inv_length: Wide<L>,
    pub(crate) axial: Wide<L>,
    pub(crate) torsion: Wide<L>,
    pub(crate) bend_z: Wide<L>,
    pub(crate) bend_y: Wide<L>,
    pub(crate) g_exp: u32,
}

impl<const L: usize> BoundedCoefficients<L>
where
    Wide<L>: SupportedWidth,
{
    /// The same values at a width M ≥ L (exact).
    pub(crate) fn widen<const M: usize>(&self) -> BoundedCoefficients<M>
    where
        Wide<M>: SupportedWidth,
    {
        BoundedCoefficients {
            inv_length: self.inv_length.widen::<M>(),
            axial: self.axial.widen::<M>(),
            torsion: self.torsion.widen::<M>(),
            bend_z: self.bend_z.widen::<M>(),
            bend_y: self.bend_y.widen::<M>(),
            g_exp: self.g_exp,
        }
    }
}

impl<const L: usize> MemberOperators<L>
where
    Wide<L>: SupportedWidth,
{
    /// K_e[a][b] (symmetric).
    pub(crate) fn ke(&self, a: usize, b: usize) -> &Wide<L> {
        &self.ke[upper_index(a, b)]
    }

    /// The member's bounded-operator coefficients.
    pub(crate) fn bounded(&self) -> BoundedCoefficients<L> {
        BoundedCoefficients {
            inv_length: self.inv_length,
            axial: self.axial,
            torsion: self.torsion,
            bend_z: self.bend_z,
            bend_y: self.bend_y,
            g_exp: self.g_exp,
        }
    }

    /// The element's global DOFs (node i's six, then node j's).
    pub(crate) fn dofs(&self) -> [usize; 12] {
        let mut out = [0usize; 12];
        for k in 0..6 {
            out[k] = self.node_i as usize * DOF_PER_NODE + k;
            out[6 + k] = self.node_j as usize * DOF_PER_NODE + k;
        }
        out
    }
}

/// A directional spring's 3×3 block at one precision.
#[derive(Debug, Clone)]
pub(crate) struct DirectionalBlock<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    pub(crate) node: u32,
    pub(crate) kind: SpringKind,
    pub(crate) k: [[Wide<L>; 3]; 3],
}

impl<const L: usize> DirectionalBlock<L>
where
    Wide<L>: SupportedWidth,
{
    /// The same block at a width M ≥ L (exact).
    pub(crate) fn widen<const M: usize>(&self) -> DirectionalBlock<M>
    where
        Wide<M>: SupportedWidth,
    {
        DirectionalBlock {
            node: self.node,
            kind: self.kind,
            k: self.k.map(|row| row.map(|v| v.widen::<M>())),
        }
    }
}

fn lift<const L: usize>(x: f64) -> Result<Wide<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    Ok(Wide::<L>::from_f64(x)?)
}

/// fl(Σ_k a_k·b_k), one exact expansion rounded once.
fn dot<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    a: &[Wide<L>; 3],
    b: &[Wide<L>; 3],
) -> Result<Wide<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    sum.clear();
    for k in 0..3 {
        sum.add_product(ctx, &a[k], &b[k], false)?;
    }
    Ok(sum.round(ctx)?)
}

/// v/|v| with |v| = √(fl(v·v)).
fn normalize<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    v: &[Wide<L>; 3],
) -> Result<([Wide<L>; 3], Wide<L>), AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let squared = dot(ctx, sum, v, v)?;
    let norm = ctx.sqrt(&squared)?;
    Ok((
        [
            ctx.div(&v[0], &norm)?,
            ctx.div(&v[1], &norm)?,
            ctx.div(&v[2], &norm)?,
        ],
        norm,
    ))
}

/// The member's operators at the context's precision (module documentation,
/// items 1–3).
pub(crate) fn form_member<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    nodes: &[[f64; 3]],
    m: &StraightMember,
) -> Result<MemberOperators<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let zero = Wide::<L>::ZERO;
    let xi = nodes[m.node_i as usize];
    let xj = nodes[m.node_j as usize];
    // 1. The frame.
    let mut d = [zero; 3];
    for k in 0..3 {
        sum.clear();
        sum.add_binary64(xj[k], false)?;
        sum.add_binary64(xi[k], true)?;
        d[k] = sum.round(ctx)?;
    }
    let (ex, length) = normalize(ctx, sum, &d)?;
    // V-K seeded fault VK-F04 (§7.3-4): e_x's first two components swapped.
    #[cfg(any(test, feature = "mutation-controls"))]
    let ex = if super::seeded::active(super::seeded::Fault::F04) {
        [ex[1], ex[0], ex[2]]
    } else {
        ex
    };
    let yr = [
        lift::<L>(m.y_reference[0])?,
        lift::<L>(m.y_reference[1])?,
        lift::<L>(m.y_reference[2])?,
    ];
    let projection = dot(ctx, sum, &yr, &ex)?;
    let mut yc = [zero; 3];
    for k in 0..3 {
        sum.clear();
        sum.add_wide(&yr[k], false)?;
        sum.add_product(ctx, &projection, &ex[k], true)?;
        yc[k] = sum.round(ctx)?;
    }
    let (ey, _) = normalize(ctx, sum, &yc)?;
    let g_exp = gram_exponent(ctx, sum, &yc, &yr)?;
    let mut zc = [zero; 3];
    for (k, (p, q)) in [(1usize, 2usize), (2, 0), (0, 1)].into_iter().enumerate() {
        sum.clear();
        sum.add_product(ctx, &ex[p], &ey[q], false)?;
        sum.add_product(ctx, &ex[q], &ey[p], true)?;
        zc[k] = sum.round(ctx)?;
    }
    let (ez, _) = normalize(ctx, sum, &zc)?;
    let inv_length = ctx.div(&Wide::<L>::ONE, &length)?;
    // 3. The section terms over L.
    let e = lift::<L>(m.elastic_modulus)?;
    let over_length = |ctx: &mut WideContext<L>,
                       sum: &mut ExactWideSum,
                       a: &Wide<L>,
                       b: f64|
     -> Result<Wide<L>, AttemptStop> {
        let b = lift::<L>(b)?;
        sum.clear();
        sum.add_product(ctx, a, &b, false)?;
        let product = sum.round(ctx)?;
        Ok(ctx.div(&product, &length)?)
    };
    let axial = over_length(ctx, sum, &e, m.area)?;
    let g = lift::<L>(m.shear_modulus)?;
    let torsion = over_length(ctx, sum, &g, m.torsion_constant)?;
    let bend_z = over_length(ctx, sum, &e, m.second_moment_z)?;
    let bend_y = over_length(ctx, sum, &e, m.second_moment_y)?;
    // 2. B = B_local·T (global).
    let mut b = [[zero; 12]; 6];
    for k in 0..3 {
        let iy = ctx.mul(&inv_length, &ey[k])?;
        let iz = ctx.mul(&inv_length, &ez[k])?;
        b[0][k] = ex[k].neg();
        b[0][6 + k] = ex[k];
        b[1][3 + k] = ex[k].neg();
        b[1][9 + k] = ex[k];
        for (row, rotation) in [(2usize, 3usize), (3, 9)] {
            b[row][k] = iy;
            b[row][6 + k] = iy.neg();
            b[row][rotation + k] = ez[k];
        }
        for (row, rotation) in [(4usize, 3usize), (5, 9)] {
            b[row][k] = iz.neg();
            b[row][6 + k] = iz;
            b[row][rotation + k] = ey[k];
        }
    }
    // D: the 4c and 2c entries are exact scalings.
    let four_z = bend_z.mul_pow2(2)?;
    let two_z = bend_z.mul_pow2(1)?;
    let four_y = bend_y.mul_pow2(2)?;
    let two_y = bend_y.mul_pow2(1)?;
    let d_rows: [[(usize, Wide<L>); 2]; 6] = [
        [(0, axial), (0, zero)],
        [(1, torsion), (1, zero)],
        [(2, four_z), (3, two_z)],
        [(2, two_z), (3, four_z)],
        [(4, four_y), (5, two_y)],
        [(4, two_y), (5, four_y)],
    ];
    let mut db = [[zero; 12]; 6];
    for r in 0..6 {
        for c in 0..12 {
            sum.clear();
            for (s, coefficient) in &d_rows[r] {
                sum.add_product(ctx, coefficient, &b[*s][c], false)?;
            }
            db[r][c] = sum.round(ctx)?;
        }
    }
    let mut ke = [zero; 78];
    for a in 0..12 {
        for bb in a..12 {
            sum.clear();
            for r in 0..6 {
                sum.add_product(ctx, &b[r][a], &db[r][bb], false)?;
            }
            ke[upper_index(a, bb)] = sum.round(ctx)?;
        }
    }
    Ok(MemberOperators {
        node_i: m.node_i,
        node_j: m.node_j,
        axes: [ex, ey, ez],
        inv_length,
        axial,
        torsion,
        bend_z,
        bend_y,
        b,
        ke,
        g_exp,
    })
}

/// The least k ≥ 0 with 4^k·(y_c·y_c) − y_ref·y_ref ≥ 0, decided exactly (R7
/// §4.1.6.2 item 2). y_c ≠ 0 here (it was normalized). The search starts below
/// the exponent estimate and each test is one exact expansion in `sum`, so the
/// formation's work counts it.
fn gram_exponent<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    yc: &[Wide<L>; 3],
    yr: &[Wide<L>; 3],
) -> Result<u32, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let top = |v: &[Wide<L>; 3]| {
        v.iter()
            .filter(|x| !x.is_zero())
            .map(|x| x.exponent())
            .max()
            .unwrap_or(0)
    };
    // 4^(top_r − top_c − 2) < (y_ref·y_ref)/(y_c·y_c) < 4^(top_r − top_c + 2),
    // so no k below the start passes, and at most five tests run.
    let mut k = (top(yr) - top(yc) - 2).max(0);
    loop {
        sum.clear();
        for c in 0..3 {
            let scaled = yc[c].mul_pow2(k)?;
            sum.add_product(ctx, &scaled, &scaled, false)?;
        }
        for c in 0..3 {
            sum.add_product(ctx, &yr[c], &yr[c], true)?;
        }
        if sum.signum()? >= 0 {
            return u32::try_from(k).map_err(|_| AttemptStop::Exponent);
        }
        k += 1;
    }
}

/// A member's bounded operator g·B̄ᵀ(|D|B̄), all 144 entries, stage-rounded at
/// the context's precision (module documentation; emu7's `element_bounded`).
pub(crate) fn bounded_block<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    m: &BoundedCoefficients<L>,
) -> Result<[[Wide<L>; 12]; 12], AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let zero = Wide::<L>::ZERO;
    let one = Wide::<L>::ONE;
    let inv = m.inv_length.abs();
    let mut bb = [[zero; 12]; 6];
    for k in 0..3 {
        bb[0][k] = one;
        bb[0][6 + k] = one;
        bb[1][3 + k] = one;
        bb[1][9 + k] = one;
        for (row, rotation) in [(2usize, 3usize), (3, 9), (4, 3), (5, 9)] {
            bb[row][k] = inv;
            bb[row][6 + k] = inv;
            bb[row][rotation + k] = one;
        }
    }
    let (z, y) = (m.bend_z.abs(), m.bend_y.abs());
    let d_rows: [[(usize, Wide<L>); 2]; 6] = [
        [(0, m.axial.abs()), (0, zero)],
        [(1, m.torsion.abs()), (1, zero)],
        [(2, z.mul_pow2(2)?), (3, z.mul_pow2(1)?)],
        [(2, z.mul_pow2(1)?), (3, z.mul_pow2(2)?)],
        [(4, y.mul_pow2(2)?), (5, y.mul_pow2(1)?)],
        [(4, y.mul_pow2(1)?), (5, y.mul_pow2(2)?)],
    ];
    let mut db = [[zero; 12]; 6];
    for r in 0..6 {
        for c in 0..12 {
            sum.clear();
            for (s, coefficient) in &d_rows[r] {
                sum.add_product(ctx, coefficient, &bb[*s][c], false)?;
            }
            db[r][c] = sum.round(ctx)?;
        }
    }
    let g = i64::from(m.g_exp);
    let mut out = [[zero; 12]; 12];
    for a in 0..12 {
        for b in 0..12 {
            sum.clear();
            for r in 0..6 {
                sum.add_product(ctx, &bb[r][a], &db[r][b], false)?;
            }
            out[a][b] = sum.round(ctx)?.mul_pow2(g)?;
        }
    }
    Ok(out)
}

/// Every member's operators at the context's precision.
pub(crate) fn form_members<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    source: &PrimitiveSource,
) -> Result<Vec<MemberOperators<L>>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let mut out = Vec::with_capacity(source.members().len());
    for m in source.members() {
        out.push(form_member(ctx, sum, source.nodes(), m)?);
        guard.check(ctx, sum)?;
    }
    Ok(out)
}

/// Every directional spring's block at the context's precision.
pub(crate) fn form_directional<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    source: &PrimitiveSource,
) -> Result<Vec<DirectionalBlock<L>>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let zero = Wide::<L>::ZERO;
    let mut out = Vec::with_capacity(source.directional_springs().len());
    for s in source.directional_springs() {
        let n = [
            lift::<L>(s.direction[0])?,
            lift::<L>(s.direction[1])?,
            lift::<L>(s.direction[2])?,
        ];
        let norm2 = dot(ctx, sum, &n, &n)?;
        let k = lift::<L>(s.stiffness)?;
        let mut block = [[zero; 3]; 3];
        for a in 0..3 {
            for b in a..3 {
                sum.clear();
                sum.add_product(ctx, &n[a], &n[b], false)?;
                let m = sum.round(ctx)?;
                let km = ctx.mul(&k, &m)?;
                let v = ctx.div(&km, &norm2)?;
                // V-K seeded fault VK-S1: k·n·nᵀ without the division by nᵀn.
                #[cfg(any(test, feature = "mutation-controls"))]
                let v = if super::seeded::active(super::seeded::Fault::S1) {
                    km
                } else {
                    v
                };
                block[a][b] = v;
                block[b][a] = v;
            }
        }
        out.push(DirectionalBlock {
            node: s.node,
            kind: s.kind,
            k: block,
        });
    }
    Ok(out)
}

/// One contribution to a pattern entry.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Contribution {
    Member { member: u32, a: u8, b: u8 },
    Spring { spring: u32 },
    Directional { spring: u32, a: u8, b: u8 },
}

/// The source's sparse structure: K1's `SparsePattern` over every DOF, and the
/// contributions of each upper-triangle entry (integer data, per source).
#[derive(Debug, Clone)]
pub(crate) struct Structure {
    pub(crate) pattern: SparsePattern,
    starts: Vec<usize>,
    items: Vec<Contribution>,
}

impl Structure {
    /// `SparsePattern::from_positions` of the member 12×12 blocks, the spring
    /// diagonals and the directional springs' 3×3 node blocks.
    pub(crate) fn new(source: &PrimitiveSource) -> Result<Self, AttemptStop> {
        let n = source.dof_count();
        let mut positions: Vec<(usize, usize)> = Vec::new();
        let member_dofs = |m: &StraightMember| {
            let mut out = [0usize; 12];
            for k in 0..6 {
                out[k] = m.node_i as usize * DOF_PER_NODE + k;
                out[6 + k] = m.node_j as usize * DOF_PER_NODE + k;
            }
            out
        };
        for m in source.members() {
            let dofs = member_dofs(m);
            for &r in &dofs {
                for &c in &dofs {
                    if r <= c {
                        positions.push((r, c));
                    }
                }
            }
        }
        for s in source.springs() {
            let d = s.dof.global();
            positions.push((d, d));
        }
        for s in source.directional_springs() {
            let base = s.node as usize * DOF_PER_NODE + s.kind.offset();
            for a in 0..3 {
                for b in a..3 {
                    positions.push((base + a, base + b));
                }
            }
        }
        let pattern = SparsePattern::from_positions(n, positions.iter().copied())
            .map_err(|_| AttemptStop::Structure)?;
        let entries = pattern.entry_count();
        let mut tagged: Vec<(usize, Contribution)> = Vec::new();
        let find = |r: usize, c: usize| pattern.find(r, c).ok_or(AttemptStop::Structure);
        for (index, m) in source.members().iter().enumerate() {
            let dofs = member_dofs(m);
            for a in 0..12 {
                for b in 0..12 {
                    if dofs[a] <= dofs[b] {
                        tagged.push((
                            find(dofs[a], dofs[b])?,
                            Contribution::Member {
                                member: index as u32,
                                a: a as u8,
                                b: b as u8,
                            },
                        ));
                    }
                }
            }
        }
        for (index, s) in source.springs().iter().enumerate() {
            let d = s.dof.global();
            tagged.push((
                find(d, d)?,
                Contribution::Spring {
                    spring: index as u32,
                },
            ));
        }
        for (index, s) in source.directional_springs().iter().enumerate() {
            let base = s.node as usize * DOF_PER_NODE + s.kind.offset();
            for a in 0..3 {
                for b in a..3 {
                    tagged.push((
                        find(base + a, base + b)?,
                        Contribution::Directional {
                            spring: index as u32,
                            a: a as u8,
                            b: b as u8,
                        },
                    ));
                }
            }
        }
        // Counting sort by entry (stable: member order, then springs).
        let mut counts = vec![0usize; entries + 1];
        for (entry, _) in &tagged {
            counts[entry + 1] += 1;
        }
        for k in 0..entries {
            counts[k + 1] += counts[k];
        }
        let starts = counts.clone();
        let mut next = counts;
        let mut items = vec![Contribution::Spring { spring: 0 }; tagged.len()];
        for (entry, item) in tagged {
            items[next[entry]] = item;
            next[entry] += 1;
        }
        Ok(Self {
            pattern,
            starts,
            items,
        })
    }

    /// Stored pattern entries (both triangles).
    pub(crate) fn entry_count(&self) -> usize {
        self.pattern.entry_count()
    }

    /// The contributions of an upper-triangle (row ≤ col) entry: member
    /// entries (a, b) with row = dofs[a] and col = dofs[b], springs, and
    /// directional-block entries (a, b). A lower entry reads its transpose's.
    pub(crate) fn contributions(&self, upper: usize) -> &[Contribution] {
        &self.items[self.starts[upper]..self.starts[upper + 1]]
    }

    /// (row, col, entry index) of every stored entry, rows ascending.
    pub(crate) fn entries(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        (0..self.pattern.dimension()).flat_map(move |row| {
            self.pattern
                .row_range(row)
                .map(move |index| (row, self.pattern.column(index), index))
        })
    }
}

/// The assembled K at the context's precision: one value per pattern entry.
pub(crate) fn assemble<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    source: &PrimitiveSource,
    structure: &Structure,
    members: &[MemberOperators<L>],
    directional: &[DirectionalBlock<L>],
) -> Result<Vec<Wide<L>>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let mut values = vec![Wide::<L>::ZERO; structure.entry_count()];
    let mut done = vec![false; structure.entry_count()];
    for (row, col, index) in structure.entries() {
        if row > col {
            continue;
        }
        sum.clear();
        for item in &structure.items[structure.starts[index]..structure.starts[index + 1]] {
            match *item {
                Contribution::Member { member, a, b } => {
                    sum.add_wide(members[member as usize].ke(a as usize, b as usize), false)?
                }
                Contribution::Spring { spring } => {
                    sum.add_binary64(source.springs()[spring as usize].stiffness, false)?
                }
                Contribution::Directional { spring, a, b } => sum.add_wide(
                    &directional[spring as usize].k[a as usize][b as usize],
                    false,
                )?,
            }
        }
        values[index] = sum.round(ctx)?;
        // V-K seeded fault VK-F03 (§7.3-4): the smallest contribution to a
        // diagonal entry dropped (R1's NC-LOST-SOFT).
        #[cfg(any(test, feature = "mutation-controls"))]
        if row == col && super::seeded::active(super::seeded::Fault::F03) {
            let items = &structure.items[structure.starts[index]..structure.starts[index + 1]];
            if items.len() > 1 {
                let mut terms: Vec<Wide<L>> = Vec::with_capacity(items.len());
                for item in items {
                    terms.push(match *item {
                        Contribution::Member { member, a, b } => {
                            *members[member as usize].ke(a as usize, b as usize)
                        }
                        Contribution::Spring { spring } => {
                            lift::<L>(source.springs()[spring as usize].stiffness)?
                        }
                        Contribution::Directional { spring, a, b } => {
                            directional[spring as usize].k[a as usize][b as usize]
                        }
                    });
                }
                let smallest = (0..terms.len())
                    .min_by(|&x, &y| terms[x].abs().cmp_value(&terms[y].abs()))
                    .unwrap();
                sum.clear();
                for (t, term) in terms.iter().enumerate() {
                    if t != smallest {
                        sum.add_wide(term, false)?;
                    }
                }
                values[index] = sum.round(ctx)?;
            }
        }
        // V-K seeded fault VK-F01 (§7.3-1): the entry promoted from its
        // binary64 rounding.
        #[cfg(any(test, feature = "mutation-controls"))]
        if super::seeded::active(super::seeded::Fault::F01) {
            let rounded = values[index].to_binary64().value().unwrap_or(0.0);
            values[index] = lift::<L>(rounded)?;
        }
        done[index] = true;
        if row == col {
            guard.check(ctx, sum)?;
        }
    }
    for (row, col, index) in structure.entries() {
        if row > col {
            let mirror = structure.pattern.transpose(index);
            debug_assert!(done[mirror]);
            values[index] = values[mirror];
        }
    }
    Ok(values)
}

/// Ā at the context's precision (module documentation): one value per pattern
/// entry, both triangles formed separately (the lower triangle reads the
/// element blocks' (b, a) entries), each one exact sum rounded once.
pub(crate) fn assemble_bounded<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    source: &PrimitiveSource,
    structure: &Structure,
    members: &[BoundedCoefficients<L>],
    directional: &[DirectionalBlock<L>],
) -> Result<Vec<Wide<L>>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let mut blocks = Vec::with_capacity(members.len());
    for m in members {
        blocks.push(bounded_block(ctx, sum, m)?);
        guard.check(ctx, sum)?;
    }
    let mut values = vec![Wide::<L>::ZERO; structure.entry_count()];
    for (row, col, index) in structure.entries() {
        let (upper, mirrored) = if row <= col {
            (index, false)
        } else {
            (structure.pattern.transpose(index), true)
        };
        sum.clear();
        for item in &structure.items[structure.starts[upper]..structure.starts[upper + 1]] {
            match *item {
                Contribution::Member { member, a, b } => {
                    let (a, b) = if mirrored { (b, a) } else { (a, b) };
                    sum.add_wide(&blocks[member as usize][a as usize][b as usize], false)?
                }
                Contribution::Spring { spring } => {
                    sum.add_binary64(source.springs()[spring as usize].stiffness.abs(), false)?
                }
                Contribution::Directional { spring, a, b } => {
                    let (a, b) = if mirrored { (b, a) } else { (a, b) };
                    sum.add_wide(
                        &directional[spring as usize].k[a as usize][b as usize].abs(),
                        false,
                    )?
                }
            }
        }
        values[index] = sum.round(ctx)?;
        if row == col {
            guard.check(ctx, sum)?;
        }
    }
    Ok(values)
}

/// The reduced right-hand side at p of every free DOF (module documentation,
/// item 6): the ledger's exact net plus the exact products −K_ic·u_c over the
/// prescribed columns, rounded once. `u` holds the prescribed values at p (a
/// case's binary64 values exactly; a combination's exact sum rounded once).
pub(crate) fn reduced_rhs<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    source: &PrimitiveSource,
    structure: &Structure,
    k: &[Wide<L>],
    ledger: &RetainedLedger,
    free: &[usize],
    u: &[Wide<L>],
) -> Result<Vec<Wide<L>>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let mut out = Vec::with_capacity(free.len());
    for &i in free {
        sum.clear();
        ledger.add_to(i, sum, false)?;
        for index in structure.pattern.row_range(i) {
            let c = structure.pattern.column(index);
            if source.constraint(c).is_some() && !u[c].is_zero() {
                sum.add_product(ctx, &k[index], &u[c], true)?;
            }
        }
        out.push(sum.round(ctx)?);
    }
    Ok(out)
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/assemble_tests.rs"]
mod tests;
