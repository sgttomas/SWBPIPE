//! K4: geometry first, the ordering, the profile LDLᵀ at p and its screens
//! (T3 D1 §4.1.3), and the triangular solves.
//!
//! - **Geometry first.** FK's `assess_rigid_body` runs per connected body before
//!   any factor; "a witnessed mechanism is refused and never escalated". Per
//!   node and kind, directional springs together with that node's global-axis
//!   springs and rigid DOFs of the kind ground the kind fully when their
//!   directions span R³, decided exactly; a body with a remaining directional
//!   ground that does not span is **not assessed** (no witness is sought and no
//!   outcome of `assess_rigid_body` is used), and proceeds as a
//!   `NumericallyUnresolved` body does (ROOT's amended Q6).
//! - **Ordering** (ROOT's K4 ruling Q7): a port of `sparse_direct`'s
//!   deterministic reverse Cuthill–McKee (`sparse_direct/src/lib.rs:505`), on the
//!   free–free sub-pattern of the structural `SparsePattern`.
//! - **Radix equilibration** as M03 (`structural.rs` `prepare_bound`):
//!   K̃ = S K S with s_i = −⌊e(K_ii)/2⌋, applied exactly.
//! - **Profile LDLᵀ at p**, the loop of `ProfileFactor::factor_structural_profile`
//!   (`structural.rs:1962`), every operation rounded to p (the factorization is
//!   outside the exact-sum rule). **Pivot screen** "d_i > 64·γ_p(m_i)·c_i, with
//!   γ_p(m) = m·2^-p/(1 − m·2^-p)", decided exactly as
//!   d_i·(2^p − m_i) − 64·m_i·c_i > 0. "A failed pivot at p escalates to the next
//!   p … It is never read as a mechanism."
//! - **Negative energy** "for pattern pairs only, at p, against the intended K":
//!   E = K_ii + K_jj − 2|K_ij|, a witness when E < −64·γ_p(4)·(K_ii + K_jj +
//!   2|K_ij|), decided exactly.
//! - **Condition estimate**: Hager–Higham with the alternating-vector safeguard
//!   (`structural.rs:1537`) on K̃ with the p-factor. "rcond ≤ 2^-(p-1) counts as
//!   unresolved at p and escalates." It is model information: "sensitivity to
//!   matrix-entry perturbation, not to authored parameters".
//!
//!   **The estimate's role (D1 revision 5a.3, R7 §5.9).** The estimate est
//!   serves availability only. It screens ill-conditioned candidates early
//!   (rcond ≤ 2^-(p−1), unchanged), and its solves choose the shift σ_c of
//!   §4.1.6.3 item 7c (`bound.rs`: an optional read-only observer collects the
//!   per-block ratios est_c; ROOT's A3-0 ruling Q4). It carries no part of the
//!   honesty argument.
//!
//! **Lemmas D and E (R7 §4.1.6.3)** take `factor()`'s loop as read at
//! `cef218a10`. `factor()`, `pivot_passes`, `negative_pair`, `solve_scaled` and
//! `solve` are unchanged since then; the certified bounds read L and D through
//! the read-only accessors below, and the shifted factorization is a separate
//! operation-for-operation copy of the loop in `bound.rs`.
use super::adaptive::{AttemptStop, StageGuard};
use super::assemble::Structure;
use super::bound::BlockRatios;
use super::source::{PrimitiveSource, SpringKind};
use super::wide::multi::{SupportedWidth, WideContext};
use super::wide::Wide;
use super::wide_sum::ExactWideSum;
use crate::rigid_body::{assess_rigid_body, ObjectiveFamily, RigidBodyStatus};
use crate::structural::StructuralError;
use crate::DOF_PER_NODE;
use std::cmp::Ordering as CmpOrdering;
use std::collections::VecDeque;

// ------------------------------------------------------------ geometry first

/// How one body entered the factor.
#[derive(Debug, Clone, PartialEq)]
pub enum BodyGeometry {
    Restrained,
    NumericallyUnresolved,
    /// Not assessed geometrically: directional grounds that do not span R³ at
    /// these (node, kind) pairs.
    NotAssessed(Vec<(u32, SpringKind)>),
}

/// A refusal by the geometric screen (never escalated).
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum GeometryRefusal {
    MechanismWitnessed {
        body: u32,
        rigid_parameters: [f64; 6],
    },
    Unavailable {
        body: u32,
        error: StructuralError,
    },
}

fn lift4(x: f64) -> Option<Wide<4>> {
    Wide::<4>::from_f64(x).ok()
}

/// det[a, b, c] ≠ 0, decided exactly (triple products through two exact
/// products at p = 128 and 192).
fn determinant_nonzero(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Option<bool> {
    let mut c128 = WideContext::<4>::new(128).ok()?;
    let mut c192 = WideContext::<4>::new(192).ok()?;
    let mut sum = ExactWideSum::new();
    for (x, y, z, negate) in [
        (0usize, 1usize, 2usize, false),
        (1, 2, 0, false),
        (2, 0, 1, false),
        (0, 2, 1, true),
        (1, 0, 2, true),
        (2, 1, 0, true),
    ] {
        let (s, e) = c128.two_product(&lift4(a[x])?, &lift4(b[y])?).ok()?;
        let cz = lift4(c[z])?;
        sum.add_product(&mut c192, &s, &cz, negate).ok()?;
        sum.add_product(&mut c192, &e, &cz, negate).ok()?;
    }
    Some(!sum.is_zero().ok()?)
}

/// Whether the directions span R³, decided exactly (None if undecidable).
pub(crate) fn spans_space(directions: &[[f64; 3]]) -> bool {
    let n = directions.len();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if determinant_nonzero(directions[i], directions[j], directions[k]) == Some(true) {
                    return true;
                }
            }
        }
    }
    false
}

fn axis(component: usize) -> [f64; 3] {
    let mut out = [0.0; 3];
    out[component % 3] = 1.0;
    out
}

/// Geometry first, per body (module documentation). A refusal names the first
/// witnessed body.
pub(crate) fn geometry_first(
    source: &PrimitiveSource,
) -> Result<Vec<BodyGeometry>, GeometryRefusal> {
    // V-K seeded fault VK-F07 (§7.3-7): the geometric mechanism check disabled.
    #[cfg(any(test, feature = "mutation-controls"))]
    if super::seeded::active(super::seeded::Fault::F07) {
        return Ok(vec![
            BodyGeometry::NumericallyUnresolved;
            source.body_count() as usize
        ]);
    }
    let mut out = Vec::with_capacity(source.body_count() as usize);
    for body in 0..source.body_count() {
        let nodes = source.body_nodes(body);
        let mut local_of = std::collections::BTreeMap::new();
        for (local, &node) in nodes.iter().enumerate() {
            local_of.insert(node, local);
        }
        let coordinates: Vec<[f64; 3]> =
            nodes.iter().map(|&n| source.nodes()[n as usize]).collect();
        let mut grounds: Vec<usize> = Vec::new();
        let mut not_spanning: Vec<(u32, SpringKind)> = Vec::new();
        for &node in &nodes {
            let local = local_of[&node];
            let base = node as usize * DOF_PER_NODE;
            for component in 0..DOF_PER_NODE {
                if source.constraint(base + component).is_some() {
                    grounds.push(local * DOF_PER_NODE + component);
                }
            }
            for s in source.springs() {
                if s.dof.node == node {
                    grounds.push(local * DOF_PER_NODE + s.dof.component.index());
                }
            }
            for kind in [SpringKind::Translation, SpringKind::Rotation] {
                let directional: Vec<[f64; 3]> = source
                    .directional_springs()
                    .iter()
                    .filter(|s| s.node == node && s.kind == kind)
                    .map(|s| s.direction)
                    .collect();
                if directional.is_empty() {
                    continue;
                }
                let mut directions = directional;
                for component in kind.offset()..kind.offset() + 3 {
                    let rigid = source.constraint(base + component).is_some();
                    let sprung = source
                        .springs()
                        .iter()
                        .any(|s| s.dof.node == node && s.dof.component.index() == component);
                    if rigid || sprung {
                        directions.push(axis(component));
                    }
                }
                if spans_space(&directions) {
                    for component in kind.offset()..kind.offset() + 3 {
                        grounds.push(local * DOF_PER_NODE + component);
                    }
                } else {
                    not_spanning.push((node, kind));
                }
            }
        }
        if !not_spanning.is_empty() {
            out.push(BodyGeometry::NotAssessed(not_spanning));
            continue;
        }
        grounds.sort_unstable();
        grounds.dedup();
        match assess_rigid_body(
            &coordinates,
            &grounds,
            ObjectiveFamily::WeldedUnreleasedElasticFrames,
        ) {
            Ok(assessment) => match assessment.status {
                RigidBodyStatus::MechanismWitnessed => {
                    return Err(GeometryRefusal::MechanismWitnessed {
                        body,
                        rigid_parameters: assessment.rigid_parameters.unwrap_or([0.0; 6]),
                    })
                }
                RigidBodyStatus::Restrained => out.push(BodyGeometry::Restrained),
                RigidBodyStatus::NumericallyUnresolved | RigidBodyStatus::UnqualifiedFamily => {
                    out.push(BodyGeometry::NumericallyUnresolved)
                }
            },
            Err(error) => return Err(GeometryRefusal::Unavailable { body, error }),
        }
    }
    Ok(out)
}

// ------------------------------------------------------------ ordering (Q7)

fn bfs_reachable(seed: usize, neighbors: &[Vec<usize>]) -> Vec<usize> {
    let mut marked = vec![false; neighbors.len()];
    marked[seed] = true;
    let mut reachable = vec![seed];
    let mut queue = VecDeque::new();
    queue.push_back(seed);
    while let Some(node) = queue.pop_front() {
        for &next in &neighbors[node] {
            if !marked[next] {
                marked[next] = true;
                reachable.push(next);
                queue.push_back(next);
            }
        }
    }
    reachable
}

fn bfs_eccentricity(start: usize, neighbors: &[Vec<usize>]) -> (usize, Vec<usize>) {
    let mut marked = vec![false; neighbors.len()];
    marked[start] = true;
    let mut level = vec![start];
    let mut depth = 0;
    loop {
        let mut next_level = Vec::new();
        for &node in &level {
            for &next in &neighbors[node] {
                if !marked[next] {
                    marked[next] = true;
                    next_level.push(next);
                }
            }
        }
        if next_level.is_empty() {
            return (depth, level);
        }
        depth += 1;
        level = next_level;
    }
}

fn min_by_degree_then_index(nodes: &[usize], degrees: &[usize]) -> Option<usize> {
    nodes
        .iter()
        .copied()
        .min_by_key(|&node| (degrees[node], node))
}

fn pseudo_peripheral_start(seed: usize, neighbors: &[Vec<usize>], degrees: &[usize]) -> usize {
    let component = bfs_reachable(seed, neighbors);
    let mut candidate = min_by_degree_then_index(&component, degrees).unwrap_or(seed);
    let (mut candidate_eccentricity, mut last_level) = bfs_eccentricity(candidate, neighbors);
    loop {
        let next = match min_by_degree_then_index(&last_level, degrees) {
            Some(node) => node,
            None => break,
        };
        let (next_eccentricity, next_last_level) = bfs_eccentricity(next, neighbors);
        if next_eccentricity > candidate_eccentricity {
            candidate = next;
            candidate_eccentricity = next_eccentricity;
            last_level = next_last_level;
        } else {
            break;
        }
    }
    candidate
}

/// Deterministic reverse Cuthill–McKee, ported from `sparse_direct` (its
/// tie-break rules): `order[k]` is the original index of the k-th ordered node.
/// Adjacency indices must be in range (internal data).
pub fn reverse_cuthill_mckee(adjacency: &[Vec<usize>]) -> Vec<usize> {
    let node_count = adjacency.len();
    let mut neighbors: Vec<Vec<usize>> = vec![Vec::new(); node_count];
    for (node, raw) in adjacency.iter().enumerate() {
        for &other in raw {
            debug_assert!(other < node_count);
            if other != node {
                neighbors[node].push(other);
                neighbors[other].push(node);
            }
        }
    }
    for list in &mut neighbors {
        list.sort_unstable();
        list.dedup();
    }
    let degrees: Vec<usize> = neighbors.iter().map(Vec::len).collect();
    for list in &mut neighbors {
        list.sort_by_key(|&node| (degrees[node], node));
    }
    let mut visited = vec![false; node_count];
    let mut order = Vec::with_capacity(node_count);
    for seed in 0..node_count {
        if visited[seed] {
            continue;
        }
        let start = pseudo_peripheral_start(seed, &neighbors, &degrees);
        visited[start] = true;
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &next in &neighbors[node] {
                if !visited[next] {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }
    }
    order.reverse();
    order
}

/// The free DOFs, their RCM order and skyline (integer data, per source).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Ordering {
    /// Free global DOFs, ascending (a free position indexes this list).
    pub(crate) free: Vec<usize>,
    /// Global DOF → free position (usize::MAX when constrained).
    pub(crate) position: Vec<usize>,
    /// `order[k]`: the free position of the k-th ordered row.
    pub(crate) order: Vec<usize>,
    /// `rank[a]`: the ordered row of free position a.
    pub(crate) rank: Vec<usize>,
    /// First stored column of each ordered row.
    pub(crate) first: Vec<usize>,
    pub(crate) profile_entries: usize,
}

/// RCM on the free–free sub-pattern (structural entries), and its skyline.
pub(crate) fn order_free(source: &PrimitiveSource, structure: &Structure) -> Ordering {
    let free = source.free_dofs();
    let mut position = vec![usize::MAX; source.dof_count()];
    for (a, &g) in free.iter().enumerate() {
        position[g] = a;
    }
    let adjacency: Vec<Vec<usize>> = free
        .iter()
        .map(|&g| {
            structure
                .pattern
                .row(g)
                .iter()
                .filter_map(|&c| (position[c] != usize::MAX && c != g).then_some(position[c]))
                .collect()
        })
        .collect();
    let order = reverse_cuthill_mckee(&adjacency);
    let mut rank = vec![0usize; free.len()];
    for (k, &a) in order.iter().enumerate() {
        rank[a] = k;
    }
    let first: Vec<usize> = order
        .iter()
        .enumerate()
        .map(|(i, &a)| {
            adjacency[a]
                .iter()
                .map(|&b| rank[b])
                .chain(std::iter::once(i))
                .min()
                .unwrap_or(i)
        })
        .collect();
    let profile_entries = first.iter().enumerate().map(|(i, &f)| i - f + 1).sum();
    Ordering {
        free,
        position,
        order,
        rank,
        first,
        profile_entries,
    }
}

// ------------------------------------------------------------ the factor

/// The pivot screen of one ordered row, kept for the margin evidence.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PivotScreen<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    pub(crate) pivot: Wide<L>,
    pub(crate) scale: Wide<L>,
    pub(crate) operations: u64,
}

/// The p-factor of K̃ (radix-equilibrated, RCM-ordered skyline LDLᵀ).
#[derive(Debug, Clone)]
pub(crate) struct RetainedFactor<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    order: Vec<usize>,
    first: Vec<usize>,
    rows: Vec<Vec<Wide<L>>>,
    /// Scale exponent of each free position.
    scale: Vec<i64>,
    pub(crate) screens: Vec<PivotScreen<L>>,
}

fn stored<const L: usize>(
    structure: &Structure,
    k: &[Wide<L>],
    row: usize,
    col: usize,
) -> Option<Wide<L>>
where
    Wide<L>: SupportedWidth,
{
    structure.pattern.find(row, col).map(|index| k[index])
}

/// d·(2^p − m) − 64·m·c > 0, exactly.
fn pivot_passes<const L: usize>(
    sum: &mut ExactWideSum,
    pivot: &Wide<L>,
    scale: &Wide<L>,
    m: u64,
    p: u32,
) -> Result<bool, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    sum.clear();
    sum.add_wide_scaled(pivot, false, 1, i64::from(p))?;
    sum.add_wide_scaled(pivot, true, m, 0)?;
    sum.add_wide_scaled(
        scale,
        true,
        m.checked_mul(64)
            .ok_or(AttemptStop::CountRange("pivot multiplier"))?,
        0,
    )?;
    Ok(sum.signum()? > 0)
}

/// The pattern-pair negative-energy witness on K at p (module documentation).
pub(crate) fn negative_pair<const L: usize>(
    sum: &mut ExactWideSum,
    structure: &Structure,
    k: &[Wide<L>],
    ordering: &Ordering,
    p: u32,
) -> Result<Option<(usize, usize)>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    for &i in &ordering.free {
        let Some(kii) = stored(structure, k, i, i) else {
            continue;
        };
        if kii.cmp_value(&Wide::<L>::ZERO) == CmpOrdering::Less {
            return Ok(Some((i, i)));
        }
        for index in structure.pattern.row_range(i) {
            let j = structure.pattern.column(index);
            if j <= i || ordering.position[j] == usize::MAX {
                continue;
            }
            let kjj = stored(structure, k, j, j).unwrap_or(Wide::<L>::ZERO);
            let kij = k[index].abs();
            // (K_ii + K_jj − 2|K_ij|)(2^p − 4) + 256 (K_ii + K_jj + 2|K_ij|) < 0
            sum.clear();
            let pp = i64::from(p);
            sum.add_wide_scaled(&kii, false, 1, pp)?;
            sum.add_wide_scaled(&kjj, false, 1, pp)?;
            sum.add_wide_scaled(&kij, true, 2, pp)?;
            sum.add_wide_scaled(&kii, false, 252, 0)?;
            sum.add_wide_scaled(&kjj, false, 252, 0)?;
            sum.add_wide_scaled(&kij, false, 520, 0)?;
            if sum.signum()? < 0 {
                return Ok(Some((i, j)));
            }
        }
    }
    Ok(None)
}

/// The p-factor (module documentation). A failed pivot is `Pivot` (or
/// `NegativeEnergy` when a pattern pair witnesses negative energy).
pub(crate) fn factor<const L: usize>(
    ctx: &mut WideContext<L>,
    sum: &mut ExactWideSum,
    guard: &StageGuard,
    structure: &Structure,
    k: &[Wide<L>],
    ordering: &Ordering,
) -> Result<RetainedFactor<L>, AttemptStop>
where
    Wide<L>: SupportedWidth,
{
    let p = ctx.precision();
    let n = ordering.free.len();
    let zero = Wide::<L>::ZERO;
    // Radix equilibration.
    let mut scale = Vec::with_capacity(n);
    for &g in &ordering.free {
        let d = stored(structure, k, g, g).unwrap_or(zero);
        if d.is_zero() {
            return Err(AttemptStop::ZeroDiagonal { global_dof: g });
        }
        if d.is_sign_negative() {
            return Err(AttemptStop::NegativeEnergy { i: g, j: g });
        }
        scale.push(-d.exponent().div_euclid(2));
    }
    // Scaled ordered rows.
    let mut rows: Vec<Vec<Wide<L>>> = Vec::with_capacity(n);
    for i in 0..n {
        let a = ordering.order[i];
        let g = ordering.free[a];
        let f = ordering.first[i];
        let mut row = vec![zero; i - f + 1];
        for index in structure.pattern.row_range(g) {
            let c = structure.pattern.column(index);
            let b = ordering.position[c];
            if b == usize::MAX {
                continue;
            }
            let j = ordering.rank[b];
            if j <= i {
                debug_assert!(j >= f);
                row[j - f] = k[index].mul_pow2(scale[a] + scale[b])?;
            }
        }
        rows.push(row);
    }
    let first = ordering.first.clone();
    let get = |rows: &Vec<Vec<Wide<L>>>, i: usize, j: usize| -> Wide<L> {
        if j < first[i] {
            zero
        } else {
            rows[i][j - first[i]]
        }
    };
    let mut work = vec![zero; n];
    let mut screens = Vec::with_capacity(n);
    for i in 0..n {
        for j in first[i]..i {
            let mut s = get(&rows, i, j);
            for kk in first[i].max(first[j])..j {
                let t = ctx.mul(&work[kk], &get(&rows, j, kk))?;
                s = ctx.sub(&s, &t)?;
            }
            work[j] = s;
            let d = get(&rows, j, j);
            rows[i][j - first[i]] = ctx.div(&s, &d)?;
        }
        let mut pivot = get(&rows, i, i);
        let mut cancellation = pivot.abs();
        for kk in first[i]..i {
            let term = ctx.mul(&work[kk], &get(&rows, i, kk))?;
            pivot = ctx.sub(&pivot, &term)?;
            cancellation = ctx.add(&cancellation, &term.abs())?;
        }
        let operations = u64::try_from(i - first[i])
            .ok()
            .and_then(|n| n.checked_mul(2))
            .and_then(|n| n.checked_add(2))
            .ok_or(AttemptStop::CountRange("factor operations"))?;
        if !pivot_passes(sum, &pivot, &cancellation, operations, p)? {
            let global = ordering.free[ordering.order[i]];
            return Err(match negative_pair(sum, structure, k, ordering, p)? {
                Some((a, b)) => AttemptStop::NegativeEnergy { i: a, j: b },
                None => AttemptStop::Pivot { global_dof: global },
            });
        }
        screens.push(PivotScreen {
            pivot,
            scale: cancellation,
            operations,
        });
        rows[i][i - first[i]] = pivot;
        guard.check(ctx, sum)?;
    }
    Ok(RetainedFactor {
        order: ordering.order.clone(),
        first,
        rows,
        scale,
        screens,
    })
}

impl<const L: usize> RetainedFactor<L>
where
    Wide<L>: SupportedWidth,
{
    /// L's (i, j) entry for j < i, D's for j = i, in elimination order (read
    /// by the certified bounds, R7 §4.1.6.3 item 7b).
    pub(crate) fn get(&self, i: usize, j: usize) -> Wide<L> {
        if j < self.first[i] {
            Wide::<L>::ZERO
        } else {
            self.rows[i][j - self.first[i]]
        }
    }

    /// First stored column of each ordered row.
    pub(crate) fn first(&self) -> &[usize] {
        &self.first
    }

    /// The radix scale exponent of each free position (s_a = 2^scale[a]).
    pub(crate) fn scale(&self) -> &[i64] {
        &self.scale
    }

    /// K̃ x = b, both in free-position order (ProfileFactor::solve's loops).
    pub(crate) fn solve_scaled(
        &self,
        ctx: &mut WideContext<L>,
        rhs: &[Wide<L>],
    ) -> Result<Vec<Wide<L>>, AttemptStop> {
        let n = self.order.len();
        let mut x: Vec<Wide<L>> = self.order.iter().map(|&a| rhs[a]).collect();
        for i in 0..n {
            for j in self.first[i]..i {
                let t = ctx.mul(&self.get(i, j), &x[j])?;
                x[i] = ctx.sub(&x[i], &t)?;
            }
        }
        for (i, value) in x.iter_mut().enumerate() {
            *value = ctx.div(value, &self.get(i, i))?;
        }
        for i in (0..n).rev() {
            let v = x[i];
            for j in self.first[i]..i {
                let t = ctx.mul(&self.get(i, j), &v)?;
                x[j] = ctx.sub(&x[j], &t)?;
            }
        }
        let mut out = vec![Wide::<L>::ZERO; n];
        for (i, &a) in self.order.iter().enumerate() {
            out[a] = x[i];
        }
        Ok(out)
    }

    /// K x = b in free-position order: b scaled by S, solved, x = S y (exact
    /// scalings).
    pub(crate) fn solve(
        &self,
        ctx: &mut WideContext<L>,
        rhs: &[Wide<L>],
    ) -> Result<Vec<Wide<L>>, AttemptStop> {
        let scaled: Vec<Wide<L>> = rhs
            .iter()
            .zip(&self.scale)
            .map(|(b, &s)| b.mul_pow2(s))
            .collect::<Result<_, _>>()?;
        let y = self.solve_scaled(ctx, &scaled)?;
        Ok(y.iter()
            .zip(&self.scale)
            .map(|(v, &s)| v.mul_pow2(s))
            .collect::<Result<_, _>>()?)
    }

    /// Hager–Higham on K̃ with this factor (module documentation). Returns
    /// rcond at p, or `Condition` when rcond ≤ 2^-(p−1).
    pub(crate) fn condition(
        &self,
        ctx: &mut WideContext<L>,
        sum: &mut ExactWideSum,
        structure: &Structure,
        k: &[Wide<L>],
        ordering: &Ordering,
    ) -> Result<Wide<L>, AttemptStop> {
        self.condition_observed(ctx, sum, structure, k, ordering, None)
    }

    /// `condition`, with an optional read-only observer that sees each of the
    /// screen's solves y = K̃⁻¹x (R7 §4.1.6.3 item 7c's est_c). The observer
    /// changes nothing the screen computes.
    pub(crate) fn condition_observed(
        &self,
        ctx: &mut WideContext<L>,
        sum: &mut ExactWideSum,
        structure: &Structure,
        k: &[Wide<L>],
        ordering: &Ordering,
        mut observer: Option<&mut BlockRatios<L>>,
    ) -> Result<Wide<L>, AttemptStop> {
        let p = ctx.precision();
        let n = ordering.free.len();
        let one = Wide::<L>::ONE;
        if n == 0 {
            return Ok(one);
        }
        let lift_count =
            |x: usize| -> Result<Wide<L>, AttemptStop> { Ok(Wide::<L>::from_f64(x as f64)?) };
        // ‖K̃‖₁: the largest column sum (exact, rounded once).
        let mut norm = Wide::<L>::ZERO;
        for (a, &g) in ordering.free.iter().enumerate() {
            sum.clear();
            for index in structure.pattern.row_range(g) {
                let b = ordering.position[structure.pattern.column(index)];
                if b != usize::MAX {
                    sum.add_wide_scaled(&k[index].abs(), false, 1, self.scale[a] + self.scale[b])?;
                }
            }
            let column = sum.round(ctx)?;
            if column.cmp_value(&norm) == CmpOrdering::Greater {
                norm = column;
            }
        }
        let abs_sum = |ctx: &mut WideContext<L>,
                       sum: &mut ExactWideSum,
                       v: &[Wide<L>]|
         -> Result<Wide<L>, AttemptStop> {
            sum.clear();
            for x in v {
                sum.add_wide(&x.abs(), false)?;
            }
            Ok(sum.round(ctx)?)
        };
        let inverse_n = ctx.div(&one, &lift_count(n)?)?;
        let mut x = vec![inverse_n; n];
        let mut estimate = Wide::<L>::ZERO;
        let mut previous = n;
        for _ in 0..5 {
            let y = self.solve_scaled(ctx, &x)?;
            if let Some(o) = observer.as_deref_mut() {
                o.offer(ctx, sum, &x, &y)?;
            }
            let current = abs_sum(ctx, sum, &y)?;
            if current.cmp_value(&estimate) != CmpOrdering::Greater && !estimate.is_zero() {
                break;
            }
            estimate = current;
            let signs: Vec<Wide<L>> = y
                .iter()
                .map(|v| {
                    if v.is_zero() || !v.is_sign_negative() {
                        one
                    } else {
                        one.neg()
                    }
                })
                .collect();
            let z = self.solve_scaled(ctx, &signs)?;
            if let Some(o) = observer.as_deref_mut() {
                o.offer(ctx, sum, &signs, &z)?;
            }
            let mut j = 0;
            for i in 1..n {
                if z[i].abs().cmp_value(&z[j].abs()) != CmpOrdering::Less {
                    j = i;
                }
            }
            sum.clear();
            for i in 0..n {
                sum.add_product(ctx, &z[i], &x[i], false)?;
            }
            let dot = sum.round(ctx)?;
            if z[j].abs().cmp_value(&dot) != CmpOrdering::Greater || j == previous {
                break;
            }
            previous = j;
            x.iter_mut().for_each(|v| *v = Wide::<L>::ZERO);
            x[j] = one;
        }
        let mut alternating = Vec::with_capacity(n);
        for i in 0..n {
            let magnitude = if n == 1 {
                one
            } else {
                ctx.div(&lift_count(n - 1 + i)?, &lift_count(n - 1)?)?
            };
            alternating.push(if i % 2 == 0 {
                magnitude
            } else {
                magnitude.neg()
            });
        }
        let y = self.solve_scaled(ctx, &alternating)?;
        if let Some(o) = observer.as_deref_mut() {
            o.offer(ctx, sum, &alternating, &y)?;
        }
        let total = abs_sum(ctx, sum, &y)?.mul_pow2(1)?;
        let alternative = ctx.div(
            &total,
            &lift_count(
                n.checked_mul(3)
                    .ok_or(AttemptStop::CountRange("condition multiplier"))?,
            )?,
        )?;
        if alternative.cmp_value(&estimate) == CmpOrdering::Greater {
            estimate = alternative;
        }
        let product = ctx.mul(&norm, &estimate)?;
        let boundary = one.mul_pow2(i64::from(p) - 1)?;
        if product.is_zero() || product.cmp_value(&boundary) != CmpOrdering::Less {
            return Err(AttemptStop::Condition);
        }
        let rcond = ctx.div(&one, &product)?;
        Ok(if rcond.cmp_value(&one) == CmpOrdering::Greater {
            one
        } else {
            rcond
        })
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/factor_tests.rs"]
mod tests;
