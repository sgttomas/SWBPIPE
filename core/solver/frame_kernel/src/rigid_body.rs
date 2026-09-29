//! Geometric rigid-motion screen, separate from matrix invertibility.
use crate::structural::{gamma, Expansion, StructuralError};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectiveFamily {
    /// Welded unreleased straight frames with a positive objective elastic energy.
    WeldedUnreleasedElasticFrames,
    /// Geometry is diagnostic only; no physical-mechanism assertion is licensed.
    Unqualified,
}
#[derive(Debug, Clone, PartialEq)]
pub enum RigidBodyStatus {
    Restrained,
    MechanismWitnessed,
    NumericallyUnresolved,
    UnqualifiedFamily,
}
#[derive(Debug, Clone, PartialEq)]
pub struct RigidBodyAssessment {
    pub status: RigidBodyStatus,
    pub singular_values: [f64; 6],
    pub rank_screen: f64,
    pub characteristic_length: f64,
    pub origin: [f64; 3],
    /// Dimensionless [tx,ty,tz,rx,ry,rz]; translations at nodes are length*rigid translation.
    pub rigid_parameters: Option<[f64; 6]>,
    pub node_motion: Option<Vec<[f64; 6]>>,
    pub maximum_constraint_action: Option<f64>,
    pub iterations: usize,
}
/// Coordinates and ground indices are LOCAL to one actual connected body supplied by caller.
/// Positive ground springs and actually selected active contacts may occur in ground_dofs.
/// Inter-node connectors are not grounds. This utility does not validate connectivity or internal modes.
pub fn assess_rigid_body(
    coordinates: &[[f64; 3]],
    ground_dofs: &[usize],
    family: ObjectiveFamily,
) -> Result<RigidBodyAssessment, StructuralError> {
    if coordinates.is_empty()
        || coordinates.iter().flatten().any(|v| !v.is_finite())
        || ground_dofs.iter().any(|&i| i >= 6 * coordinates.len())
    {
        return Err(StructuralError::InvalidInput("rigid geometry"));
    }
    let origin = coordinates[0];
    let mut relative = Vec::new();
    let mut length: f64 = 0.0;
    for p in coordinates {
        let r = [p[0] - origin[0], p[1] - origin[1], p[2] - origin[2]];
        if r.iter().any(|v| !v.is_finite()) {
            return Err(StructuralError::Range("relative coordinates"));
        }
        length = length.max(r[0].hypot(r[1]).hypot(r[2]));
        relative.push(r);
    }
    if !length.is_finite() {
        return Err(StructuralError::Range("nonfinite characteristic length"));
    }
    if length == 0.0 {
        length = 1.0;
    }
    for r in &mut relative {
        for x in r {
            *x /= length;
        }
    }
    let motion_rows = |node: usize| {
        let [x, y, z] = relative[node];
        [
            [1.0, 0.0, 0.0, 0.0, z, -y],
            [0.0, 1.0, 0.0, -z, 0.0, x],
            [0.0, 0.0, 1.0, y, -x, 0.0],
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        ]
    };
    let mut original = Vec::new();

    for &d in ground_dofs {
        let mut row = motion_rows(d / 6)[d % 6];

        let norm = row.iter().map(|x| x * x).sum::<f64>().sqrt();
        for x in &mut row {
            *x /= norm;
        }
        original.push(row);
    }
    // One-sided Jacobi SVD avoids B^T B and its squared condition number.
    let mut b = original.clone();
    let mut v = [[0.0; 6]; 6];
    for i in 0..6 {
        v[i][i] = 1.0;
    }
    let mut converged = false;
    let mut iterations = 0;
    for sweep in 0..64 {
        iterations = sweep + 1;
        let mut changed = false;
        for p in 0..6 {
            for q in p + 1..6 {
                let alpha: f64 = b.iter().map(|r| r[p] * r[p]).sum();
                let beta: f64 = b.iter().map(|r| r[q] * r[q]).sum();
                let cross: f64 = b.iter().map(|r| r[p] * r[q]).sum();
                if cross.abs() <= 8.0 * f64::EPSILON * (alpha.sqrt() * beta.sqrt()) {
                    continue;
                }
                let zeta = (beta - alpha) / (2.0 * cross);
                let t = zeta.signum() / (zeta.abs() + zeta.hypot(1.0));
                let t = if zeta == 0.0 { 1.0 } else { t };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = c * t;
                for row in &mut b {
                    let a = row[p];
                    let d = row[q];
                    row[p] = c * a - s * d;
                    row[q] = s * a + c * d;
                }
                for row in &mut v {
                    let a = row[p];
                    let d = row[q];
                    row[p] = c * a - s * d;
                    row[q] = s * a + c * d;
                }
                changed = true;
            }
        }
        if !changed {
            converged = true;
            break;
        }
    }
    let singular = std::array::from_fn(|j| b.iter().map(|r| r[j] * r[j]).sum::<f64>().sqrt());
    let max = singular.iter().copied().fold(0.0, f64::max);
    let screen = 64.0 * gamma(ground_dofs.len().max(6)) * max;
    let index = (0..6)
        .min_by(|&i, &j| singular[i].total_cmp(&singular[j]))
        .unwrap();
    let mut result = RigidBodyAssessment {
        status: RigidBodyStatus::NumericallyUnresolved,
        singular_values: singular,
        rank_screen: screen,
        characteristic_length: length,
        origin,
        rigid_parameters: None,
        node_motion: None,
        maximum_constraint_action: None,
        iterations,
    };
    if family == ObjectiveFamily::Unqualified {
        result.status = RigidBodyStatus::UnqualifiedFamily;
        return Ok(result);
    }
    if converged && singular[index] > screen {
        result.status = RigidBodyStatus::Restrained;
        return Ok(result);
    }
    // Rounded/centered rows only generate candidates. The final witness below uses
    // exact expansions of the ORIGINAL represented coordinates, not these rows.
    let mut candidates = vec![std::array::from_fn(|i| v[i][index])];
    for axis in 0..6 {
        let mut c = [0.0; 6];
        c[axis] = 1.0;
        candidates.push(c);
    }
    for r in &relative {
        if r.iter().any(|x| *x != 0.0) {
            candidates.push([0.0, 0.0, 0.0, r[0], r[1], r[2]]);
        }
    }
    // Unnormalized authored differences can preserve exact axes (e.g. (3,4,0))
    // that division by a non-radix characteristic length would perturb.
    for p in coordinates {
        let r = [p[0] - origin[0], p[1] - origin[1], p[2] - origin[2]];
        if r.iter().any(|x| *x != 0.0) {
            candidates.push([0.0, 0.0, 0.0, r[0], r[1], r[2]]);
        }
    }
    for candidate in candidates {
        // Unsupported expansion range or an inexact recovered motion is numerical
        // uncertainty. Neither turns a nonzero original constraint action into zero.
        if let Ok(Some(motions)) =
            original_rigid_witness(coordinates, ground_dofs, length, &candidate)
        {
            result.status = RigidBodyStatus::MechanismWitnessed;
            result.rigid_parameters = Some(candidate);
            result.node_motion = Some(motions);
            result.maximum_constraint_action = Some(0.0);
            break;
        }
    }
    Ok(result)
}
fn original_rigid_witness(
    coordinates: &[[f64; 3]],
    ground: &[usize],
    length: f64,
    candidate: &[f64; 6],
) -> Result<Option<Vec<[f64; 6]>>, StructuralError> {
    if candidate.iter().any(|v| !v.is_finite()) || candidate.iter().all(|v| *v == 0.0) {
        return Ok(None);
    }
    let origin = coordinates[0];
    let mut exact_motions = Vec::new();
    for p in coordinates {
        let mut delta: [Expansion; 3] = std::array::from_fn(|_| Expansion::default());
        for axis in 0..3 {
            delta[axis].add(p[axis])?;
            delta[axis].add(-origin[axis])?;
        }
        let mut motion: [Expansion; 6] = std::array::from_fn(|_| Expansion::default());
        for axis in 0..3 {
            motion[axis].add_product(length, candidate[axis], 0)?;
            motion[axis + 3].add(candidate[axis + 3])?;
        }
        // omega cross (x-original_origin), with exact subtraction and products.
        for (component, positive_axis, positive_omega, negative_axis, negative_omega) in
            [(0, 2, 4, 1, 5), (1, 0, 5, 2, 3), (2, 1, 3, 0, 4)]
        {
            for &part in &delta[positive_axis].terms {
                motion[component].add_product(part, candidate[positive_omega], 0)?;
            }
            for &part in &delta[negative_axis].terms {
                motion[component].add_product(-part, candidate[negative_omega], 0)?;
            }
        }
        exact_motions.push(motion);
    }
    if ground
        .iter()
        .any(|&d| !exact_motions[d / 6][d % 6].is_zero())
    {
        return Ok(None);
    }
    // Publish node_motion only if it exactly represents this common original-coordinate
    // rigid motion. A rounded recovery is not silently presented as an exact witness.
    let mut motions = Vec::new();
    for exact in exact_motions {
        let mut motion = [0.0; 6];
        for j in 0..6 {
            let Some(value) = exact[j].exact_scalar()? else {
                return Ok(None);
            };
            motion[j] = value;
        }
        motions.push(motion);
    }
    Ok(Some(motions))
}

// ------------------------------------------------------------------ K5 (W4)
//
// T3 D1 revision 5a.2 §4.9 (W4): the constrained-body witness. A body made of
// objective sub-bodies (nodes joined by straight frames, or by curved elements
// qualified by their matched macro source), joined by user-element ties and
// held by ground rows, is assessed with six unknowns (the tie reduction below).
// ROOT's K5 ruling Q4(b): the new screen calls no function of unspecified
// precision. It uses IEEE +, -, *, /, sqrt, the exact `Expansion` (TwoProduct
// through `mul_add`) and integer operations on the bits. `assess_rigid_body`
// and `original_rigid_witness` above are unchanged.

use crate::UserStiffnessElement;

/// K5 (Q6): the kind of a directional ground row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GroundKind {
    /// `n·u = 0` at the node.
    Translation,
    /// `n·θ = 0` at the node.
    Rotation,
}

/// K5: one ground row of a constrained body. Node indices are local.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstrainedGround {
    /// A restrained or prescribed DOF, or a positive spring: `6·node + dof`, as
    /// `assess_rigid_body`'s `ground_dofs`.
    Dof(usize),
    /// Q6: the row `nᵀ[I, −skew(r)]` (translation) or `[0, nᵀ]` (rotation) at
    /// `node`. `direction` is finite and nonzero; the exact check uses it as
    /// given (never normalized).
    Directional {
        node: usize,
        kind: GroundKind,
        direction: [f64; 3],
    },
}

/// K5: `ConstrainedAssessment::unresolved` when the rank lies in the τ_B band,
/// or no candidate is an exact, representable null motion.
pub const CONSTRAINED_RANK_UNRESOLVED: &str = "constrained-body rank unresolved";
/// K5 (RV14-4): `ConstrainedAssessment::unresolved` when an exact witness was
/// found, but its `[t/L, θ]` is not exactly representable (t/L overflows for a
/// tiny L, or rounds for a huge one). Nothing is published.
pub const CONSTRAINED_WITNESS_PARAMETERS_UNREPRESENTABLE: &str =
    "constrained-body witness parameters not representable";

/// K5: W4's assessment of one constrained body. `status` is never
/// `UnqualifiedFamily`: qualifying the elements is the caller's (the tie rule
/// `user_element_tie`; the adapter's curved rule).
#[derive(Debug, Clone, PartialEq)]
pub struct ConstrainedAssessment {
    pub status: RigidBodyStatus,
    pub singular_values: [f64; 6],
    /// τ_B = 64·γ(max(m, 6))·σ_max.
    pub rank_screen: f64,
    /// m: the nonzero reduced rows (the grounds, and up to three per cycle).
    pub rows: usize,
    /// The non-tree ties (cycles), including ties inside one sub-body.
    pub cycles: usize,
    /// L: a power of two from the exponent bits of the largest virtual
    /// coordinate (1 when every virtual coordinate is zero).
    pub characteristic_length: f64,
    /// The coordinates of local node 0 (the origin o).
    pub origin: [f64; 3],
    /// `[t/L, θ]` of the published witness (dimensionless translation), exact:
    /// `rigid_parameters[i]·L` is node 0's `u_i` bit for bit.
    pub rigid_parameters: Option<[f64; 6]>,
    /// Exact `[u, θ]` per local node; `Some` only with `MechanismWitnessed`.
    pub node_motion: Option<Vec<[f64; 6]>>,
    pub iterations: usize,
    /// Why the status is `NumericallyUnresolved` (`None` for the other
    /// statuses): `CONSTRAINED_RANK_UNRESOLVED` or
    /// `CONSTRAINED_WITNESS_PARAMETERS_UNREPRESENTABLE`.
    pub unresolved: Option<&'static str>,
}

/// K5 (Q5(a)): why today's user element is not a tie.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TieRefusal {
    /// "axial", "lateral", "angular" or "torsional": zero, negative or not finite.
    Stiffness(&'static str),
    /// The element repeats a node or has no valid local orientation.
    Orientation,
}

/// K5 (Q5(a)): today's user-stiffness element has relative springs along its
/// local axes, the lateral ones without the rigid-body moment coupling. Its
/// energy `Σ k_d (Δ_d)²` over the six relative local DOFs is zero exactly for
/// `u_a = u_b` and `θ_a = θ_b` when all four stiffnesses are finite and positive
/// and its orientation is valid: that is the tie W4 imposes. With a zero lateral
/// stiffness the relative lateral translation is free, so the element is no tie.
/// Returns the tie's node indices. T4's M07 repair changes this rule (the FK
/// test `k5_t4_tripwire_user_tie_space_is_the_represented_null_space`).
pub fn user_element_tie(element: &UserStiffnessElement) -> Result<[usize; 2], TieRefusal> {
    for (name, value) in [
        ("axial", element.axial_stiffness),
        ("lateral", element.lateral_stiffness),
        ("angular", element.angular_stiffness),
        ("torsional", element.torsional_stiffness),
    ] {
        if !(value.is_finite() && value > 0.0) {
            return Err(TieRefusal::Stiffness(name));
        }
    }
    if element.node_i.index == element.node_j.index || element.orientation().is_err() {
        return Err(TieRefusal::Orientation);
    }
    Ok([element.node_i.index, element.node_j.index])
}

/// K5: the connected components of `links` (objective elements: straight
/// frames and qualified curved elements) over `node_count` local nodes. Each
/// component is ascending, and the components are ordered by their smallest
/// node. A node with no link is its own sub-body.
pub fn objective_sub_bodies(
    node_count: usize,
    links: &[[usize; 2]],
) -> Result<Vec<Vec<usize>>, StructuralError> {
    if links.iter().flatten().any(|&node| node >= node_count) {
        return Err(INVALID);
    }
    let mut adjacent = vec![Vec::new(); node_count];
    for &[a, b] in links {
        if a != b {
            adjacent[a].push(b);
            adjacent[b].push(a);
        }
    }
    let mut seen = vec![false; node_count];
    let mut bodies = Vec::new();
    for seed in 0..node_count {
        if seen[seed] {
            continue;
        }
        seen[seed] = true;
        let mut body = vec![seed];
        let mut next = 0;
        while next < body.len() {
            let node = body[next];
            next += 1;
            for &other in &adjacent[node] {
                if !seen[other] {
                    seen[other] = true;
                    body.push(other);
                }
            }
        }
        body.sort_unstable();
        bodies.push(body);
    }
    Ok(bodies)
}

const INVALID: StructuralError = StructuralError::InvalidInput("constrained geometry");
const RANGE: StructuralError = StructuralError::Range("constrained relative coordinates");
/// 2^20: the grid of the rationalized candidates.
const SNAP_GRID: f64 = 1_048_576.0;
/// 2^-24: the largest move a rationalized candidate's snap may make.
const SNAP_RESIDUAL: f64 = 5.960_464_477_539_063e-8;
/// 2^-20: the prefilter bound on |B·c| relative to ‖c‖∞ (§5.3 of I14's plan).
const PREFILTER: f64 = 9.536_743_164_062_5e-7;
/// 2^-26: a reduced basis vector below this is dependent.
const DEPENDENT: f64 = 1.490_116_119_384_765_6e-8;

/// A ground row in canonical form; DOF rows carry their axis.
#[derive(Debug, Clone, Copy)]
struct GroundRow {
    node: usize,
    kind: GroundKind,
    axis: Option<usize>,
    direction: [f64; 3],
}

impl GroundRow {
    /// Canonical order: node, kind, DOF rows before directional rows, axis,
    /// then the direction's bits.
    fn key(&self) -> (usize, GroundKind, bool, usize, [u64; 3]) {
        (
            self.node,
            self.kind,
            self.axis.is_none(),
            self.axis.unwrap_or(0),
            self.direction.map(f64::to_bits),
        )
    }
}

/// The body after validation, canonical ordering and the exact tie reduction.
struct ReducedBody {
    /// Canonical ties, (min, max), sorted.
    ties: Vec<[usize; 2]>,
    /// Whether each canonical tie is a spanning-tree tie.
    tree: Vec<bool>,
    /// Canonical ground rows.
    grounds: Vec<GroundRow>,
    /// v(x) = x − o + s_B, exact.
    virtual_positions: Vec<[Expansion; 3]>,
}

/// K5 (D1 §4.9, W4): one connected body of objective sub-bodies joined by
/// user-element ties and held by ground rows. Node indices are local.
///
/// - **Reduction.** A tie (a ∈ A, b ∈ B) forces θ_A = θ_B and
///   t_B = t_A + θ × (x_a − x_b). Along a breadth-first spanning tree of the tie
///   graph over the sub-bodies (from the sub-body of node 0, ties in canonical
///   order), every node moves as u(x) = t + θ × v(x), θ(x) = θ, with the exact
///   virtual position v(x) = x − o + s_B; s_B sums the tree ties' offsets
///   x_a − x_b from the root, and o is local node 0. Every other tie adds
///   θ × (v(a) − v(b)) = 0. The map from (t, θ) to the sub-bodies' motions is
///   injective, so the null space of the unreduced stacked map (six unknowns per
///   sub-body) is the image of this six-unknown system's.
/// - **Screen.** The grounds and cycles as unit rows in `[t/L, θ]`, with L a
///   power of two; the one-sided Jacobi SVD of `assess_rigid_body` with `sqrt`
///   forms in place of `hypot`; and its rank screen τ_B = 64·γ(max(m, 6))·σ_max.
/// - **Witness.** A candidate is verified exactly (`Expansion`) against every
///   tie and ground, with the original coordinates. It is published as its
///   canonical representative r = k·p/p_j: p = `[u(node 0), θ]`, p_j its first
///   nonzero component, k the smallest of 1..=64 for which r is exact and every
///   node motion is exactly representable. If no candidate gives one, the
///   status is `NumericallyUnresolved`, never a rounded direction.
///
/// Sub-bodies, the nodes in them, ties, tie ends and grounds may come in any
/// order. They are put in a canonical order first, so the status and every bit
/// depend on the content only. Invalid input is
/// `InvalidInput("constrained geometry")`; an exact coordinate difference or
/// virtual position outside the binary64 range is
/// `Range("constrained relative coordinates")`.
pub fn assess_constrained_bodies(
    coordinates: &[[f64; 3]],
    sub_bodies: &[Vec<usize>],
    ties: &[[usize; 2]],
    grounds: &[ConstrainedGround],
) -> Result<ConstrainedAssessment, StructuralError> {
    let body = reduce_constrained_body(coordinates, sub_bodies, ties, grounds)?;
    let origin = coordinates[0];
    let mut virtual_rounded = Vec::with_capacity(coordinates.len());
    for position in &body.virtual_positions {
        virtual_rounded.push(rounded3(position)?);
    }
    let mut cycle_rounded = Vec::new();
    for (k, &[a, b]) in body.ties.iter().enumerate() {
        if !body.tree[k] {
            let offset = difference3(&body.virtual_positions[a], &body.virtual_positions[b])?;
            cycle_rounded.push(rounded3(&offset)?);
        }
    }
    let magnitude = virtual_rounded
        .iter()
        .flatten()
        .fold(0.0_f64, |m, x| m.max(x.abs()));
    let length = if magnitude == 0.0 {
        1.0
    } else {
        pow2(exponent_of(magnitude))
    };

    // The reduced rows in [t/L, θ]: grounds in canonical order, then cycles in
    // canonical tie order. A row that is exactly zero imposes nothing and is
    // dropped.
    let mut rows: Vec<[f64; 6]> = Vec::new();
    for ground in &body.grounds {
        let w = virtual_rounded[ground.node].map(|x| x / length);
        let row = match (ground.kind, ground.axis) {
            (GroundKind::Translation, Some(axis)) => translation_row(w, axis),
            (GroundKind::Rotation, Some(axis)) => {
                let mut row = [0.0; 6];
                row[3 + axis] = 1.0;
                row
            }
            (GroundKind::Translation, None) => {
                let n = unit_scaled(ground.direction);
                let m = cross(w, n);
                [n[0], n[1], n[2], m[0], m[1], m[2]]
            }
            (GroundKind::Rotation, None) => {
                let n = unit_scaled(ground.direction);
                [0.0, 0.0, 0.0, n[0], n[1], n[2]]
            }
        };
        push_normalized(&mut rows, row);
    }
    for &offset in &cycle_rounded {
        // (θ × c)_k = θ·(c × e_k).
        let c = unit_scaled(offset);
        for m in [[0.0, c[2], -c[1]], [-c[2], 0.0, c[0]], [c[1], -c[0], 0.0]] {
            push_normalized(&mut rows, [0.0, 0.0, 0.0, m[0], m[1], m[2]]);
        }
    }

    // One-sided Jacobi SVD, as `assess_rigid_body` (:88-131), with
    // `root_one_plus_square` in place of `hypot(ζ, 1)`.
    let mut b = rows.clone();
    let mut v = [[0.0; 6]; 6];
    for (i, row) in v.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    let mut converged = false;
    let mut iterations = 0;
    for sweep in 0..64 {
        iterations = sweep + 1;
        let mut changed = false;
        for p in 0..6 {
            for q in p + 1..6 {
                let (mut alpha, mut beta, mut gamma_pq) = (0.0_f64, 0.0_f64, 0.0_f64);
                for r in &b {
                    alpha += r[p] * r[p];
                    beta += r[q] * r[q];
                    gamma_pq += r[p] * r[q];
                }
                if gamma_pq.abs() <= 8.0 * f64::EPSILON * (alpha.sqrt() * beta.sqrt()) {
                    continue;
                }
                let zeta = (beta - alpha) / (2.0 * gamma_pq);
                let t = if zeta == 0.0 {
                    1.0
                } else {
                    zeta.signum() / (zeta.abs() + root_one_plus_square(zeta))
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = c * t;
                for row in b.iter_mut().chain(v.iter_mut()) {
                    let a = row[p];
                    let d = row[q];
                    row[p] = c * a - s * d;
                    row[q] = s * a + c * d;
                }
                changed = true;
            }
        }
        if !changed {
            converged = true;
            break;
        }
    }
    let singular: [f64; 6] = std::array::from_fn(|j| {
        let mut sum = 0.0_f64;
        for r in &b {
            sum += r[j] * r[j];
        }
        sum.sqrt()
    });
    let max = singular.iter().copied().fold(0.0, f64::max);
    let screen = 64.0 * gamma(rows.len().max(6)) * max;
    let index = (0..6)
        .min_by(|&i, &j| singular[i].total_cmp(&singular[j]))
        .unwrap_or(0);
    let mut result = ConstrainedAssessment {
        status: RigidBodyStatus::NumericallyUnresolved,
        singular_values: singular,
        rank_screen: screen,
        rows: rows.len(),
        cycles: cycle_rounded.len(),
        characteristic_length: length,
        origin,
        rigid_parameters: None,
        node_motion: None,
        iterations,
        unresolved: Some(CONSTRAINED_RANK_UNRESOLVED),
    };
    if converged && singular[index] > screen {
        result.status = RigidBodyStatus::Restrained;
        result.unresolved = None;
        return Ok(result);
    }

    // Candidates, in canonical order, in physical units [t, θ]. They only
    // propose; each is verified exactly against the original coordinates.
    let context = WitnessContext {
        body: &body,
        rows: &rows,
        length,
    };
    let column = |j: usize| -> [f64; 6] { std::array::from_fn(|i| v[i][j]) };
    let physical = |w: [f64; 6]| -> [f64; 6] {
        [
            w[0] * length,
            w[1] * length,
            w[2] * length,
            w[3],
            w[4],
            w[5],
        ]
    };
    // 1. The SVD vector.
    if let Some(found) = context.try_candidate(physical(column(index))) {
        return Ok(context.publish(result, found));
    }
    // 2. Rationalized null-basis candidates: the columns at or below τ_B (and
    //    the minimal one) in echelon form, each normalized by its largest
    //    component, times k = 1..=64, snapped to the 2^-20 grid. They recover
    //    small-integer and short dyadic null motions exactly. They are formed
    //    in the scaled units [t/L, θ], which coordinates times 2^k leave
    //    unchanged (L scales with them), then mapped to [t, θ].
    let mut basis = (0..6)
        .filter(|&j| singular[j] <= screen || j == index)
        .map(column)
        .collect::<Vec<_>>();
    reduce_to_echelon(&mut basis);
    for w in basis {
        let scale = w.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
        if !(scale.is_finite() && scale > 0.0) {
            continue;
        }
        let q = w.map(|x| x / scale);
        for k in 1..=64_u32 {
            let x = q.map(|x| f64::from(k) * x);
            let y = x.map(snap_to_grid);
            if (0..6).all(|i| (x[i] - y[i]).abs() <= SNAP_RESIDUAL) {
                if let Some(found) = context.try_candidate(physical(y)) {
                    return Ok(context.publish(result, found));
                }
            }
        }
    }
    // 3. The six unit axes.
    for axis in 0..6 {
        let mut e = [0.0; 6];
        e[axis] = 1.0;
        if let Some(found) = context.try_candidate(e) {
            return Ok(context.publish(result, found));
        }
    }
    // 4. Rotation about the axis through o along each virtual position.
    for w in &virtual_rounded {
        if w.iter().any(|&x| x != 0.0) {
            if let Some(found) = context.try_candidate([0.0, 0.0, 0.0, w[0], w[1], w[2]]) {
                return Ok(context.publish(result, found));
            }
        }
    }
    // 5. Rotation about each cycle offset.
    for c in &cycle_rounded {
        if c.iter().any(|&x| x != 0.0) {
            if let Some(found) = context.try_candidate([0.0, 0.0, 0.0, c[0], c[1], c[2]]) {
                return Ok(context.publish(result, found));
            }
        }
    }
    Ok(result)
}

/// Validation, canonical order and the exact tie reduction.
fn reduce_constrained_body(
    coordinates: &[[f64; 3]],
    sub_bodies: &[Vec<usize>],
    ties: &[[usize; 2]],
    grounds: &[ConstrainedGround],
) -> Result<ReducedBody, StructuralError> {
    let n = coordinates.len();
    if n == 0 || coordinates.iter().flatten().any(|x| !x.is_finite()) {
        return Err(INVALID);
    }
    let mut bodies = Vec::with_capacity(sub_bodies.len());
    for body in sub_bodies {
        if body.is_empty() {
            return Err(INVALID);
        }
        let mut body = body.clone();
        body.sort_unstable();
        bodies.push(body);
    }
    bodies.sort_unstable_by_key(|body| body[0]);
    let mut owner = vec![usize::MAX; n];
    for (index, body) in bodies.iter().enumerate() {
        for &node in body {
            if node >= n || owner[node] != usize::MAX {
                return Err(INVALID);
            }
            owner[node] = index;
        }
    }
    if owner.contains(&usize::MAX) {
        return Err(INVALID);
    }
    let mut ties = ties
        .iter()
        .map(|&[a, b]| [a.min(b), a.max(b)])
        .collect::<Vec<_>>();
    if ties.iter().any(|&[a, b]| b >= n || a == b) {
        return Err(INVALID);
    }
    ties.sort_unstable();
    let mut rows = Vec::with_capacity(grounds.len());
    for ground in grounds {
        rows.push(match *ground {
            ConstrainedGround::Dof(dof) => {
                if dof >= 6 * n {
                    return Err(INVALID);
                }
                let mut direction = [0.0; 3];
                direction[dof % 3] = 1.0;
                GroundRow {
                    node: dof / 6,
                    kind: if dof % 6 < 3 {
                        GroundKind::Translation
                    } else {
                        GroundKind::Rotation
                    },
                    axis: Some(dof % 3),
                    direction,
                }
            }
            ConstrainedGround::Directional {
                node,
                kind,
                direction,
            } => {
                if node >= n
                    || direction.iter().any(|x| !x.is_finite())
                    || direction.iter().all(|&x| x == 0.0)
                {
                    return Err(INVALID);
                }
                GroundRow {
                    node,
                    kind,
                    axis: None,
                    direction,
                }
            }
        });
    }
    rows.sort_unstable_by_key(GroundRow::key);

    // The spanning tree over the sub-bodies: breadth first from the sub-body of
    // node 0 (index 0 after sorting), ties in canonical order. s_root = 0 and
    // s_B = s_A + (x_a − x_b) for the tree tie (a ∈ A, b ∈ B), exactly.
    let mut adjacency = vec![Vec::new(); bodies.len()];
    for (k, &[a, b]) in ties.iter().enumerate() {
        if owner[a] != owner[b] {
            adjacency[owner[a]].push(k);
            adjacency[owner[b]].push(k);
        }
    }
    let mut tree = vec![false; ties.len()];
    let mut offsets: Vec<Option<[Expansion; 3]>> = vec![None; bodies.len()];
    offsets[0] = Some(Default::default());
    let mut queue = vec![0];
    let mut head = 0;
    while head < queue.len() {
        let current = queue[head];
        head += 1;
        for &k in &adjacency[current] {
            let [a, b] = ties[k];
            let (near, far) = if owner[a] == current { (a, b) } else { (b, a) };
            let other = owner[far];
            if offsets[other].is_some() {
                continue;
            }
            let mut offset = offsets[current].clone().ok_or(INVALID)?;
            for axis in 0..3 {
                offset[axis]
                    .add(coordinates[near][axis])
                    .map_err(|_| RANGE)?;
                offset[axis]
                    .add(-coordinates[far][axis])
                    .map_err(|_| RANGE)?;
            }
            offsets[other] = Some(offset);
            tree[k] = true;
            queue.push(other);
        }
    }
    if offsets.iter().any(Option::is_none) {
        return Err(INVALID);
    }
    let origin = coordinates[0];
    let mut virtual_positions = Vec::with_capacity(n);
    for (node, point) in coordinates.iter().enumerate() {
        let mut position = offsets[owner[node]].clone().ok_or(INVALID)?;
        for axis in 0..3 {
            position[axis].add(point[axis]).map_err(|_| RANGE)?;
            position[axis].add(-origin[axis]).map_err(|_| RANGE)?;
        }
        virtual_positions.push(position);
    }
    Ok(ReducedBody {
        ties,
        tree,
        grounds: rows,
        virtual_positions,
    })
}

/// A verified witness: the canonical `[t, θ]` and the exact node motions.
struct Witness {
    parameters: [f64; 6],
    motions: Vec<[f64; 6]>,
}

struct WitnessContext<'b> {
    body: &'b ReducedBody,
    rows: &'b [[f64; 6]],
    length: f64,
}

impl WitnessContext<'_> {
    /// Prefilter, exact verification of `p`, then its canonical representative.
    fn try_candidate(&self, p: [f64; 6]) -> Option<Witness> {
        if p.iter().any(|x| !x.is_finite()) || p.iter().all(|&x| x == 0.0) {
            return None;
        }
        // Performance only: an exact null vector passes (its computed |B·c| is
        // a few hundred ε·‖c‖∞ at most; I14's plan §5.3).
        let scaled = [
            p[0] / self.length,
            p[1] / self.length,
            p[2] / self.length,
            p[3],
            p[4],
            p[5],
        ];
        let size = scaled.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
        if !(size.is_finite() && size > 0.0) {
            return None;
        }
        for row in self.rows {
            let mut dot = 0.0_f64;
            for i in 0..6 {
                dot += row[i] * scaled[i];
            }
            if !(dot.abs() <= PREFILTER * size) {
                return None;
            }
        }
        match self.null_translations(p) {
            Ok(Some(_)) => self.canonical(p),
            _ => None,
        }
    }

    /// r = k·p/p_j for the smallest k in 1..=64 with r exact and every node
    /// motion exactly representable, verified again.
    fn canonical(&self, p: [f64; 6]) -> Option<Witness> {
        let j = p.iter().position(|&x| x != 0.0)?;
        for k in 1..=64_u32 {
            let k = f64::from(k);
            let mut r = [0.0; 6];
            let mut exact = true;
            for i in 0..6 {
                if i == j {
                    r[i] = k;
                } else if p[i] != 0.0 {
                    let forms = [(k * p[i]) / p[j], k * (p[i] / p[j]), p[i] * (k / p[j])];
                    match forms.into_iter().find(|&y| exact_ratio(y, p[j], k, p[i])) {
                        Some(y) => r[i] = y,
                        None => {
                            exact = false;
                            break;
                        }
                    }
                }
            }
            if !exact {
                continue;
            }
            if let Ok(Some(motions)) = self.exact_motions(r) {
                return Some(Witness {
                    parameters: r,
                    motions,
                });
            }
        }
        None
    }

    /// The exact translations of `p` at every node, if `p` satisfies every tie
    /// and ground exactly.
    fn null_translations(
        &self,
        p: [f64; 6],
    ) -> Result<Option<Vec<[Expansion; 3]>>, StructuralError> {
        let theta = [p[3], p[4], p[5]];
        let mut translations = Vec::with_capacity(self.body.virtual_positions.len());
        for v in &self.body.virtual_positions {
            let mut u: [Expansion; 3] = Default::default();
            for axis in 0..3 {
                u[axis].add(p[axis])?;
            }
            // θ × v, with exact products of each term of v.
            for (component, positive_axis, positive_theta, negative_axis, negative_theta) in
                [(0, 2, 1, 1, 2), (1, 0, 2, 2, 0), (2, 1, 0, 0, 1)]
            {
                for &term in &v[positive_axis].terms {
                    u[component].add_product(term, theta[positive_theta], 0)?;
                }
                for &term in &v[negative_axis].terms {
                    u[component].add_product(-term, theta[negative_theta], 0)?;
                }
            }
            translations.push(u);
        }
        for &[a, b] in &self.body.ties {
            let d = difference3(&translations[a], &translations[b])?;
            if !d.iter().all(Expansion::is_zero) {
                return Ok(None);
            }
        }
        for ground in &self.body.grounds {
            let u = &translations[ground.node];
            let zero = match (ground.kind, ground.axis) {
                (GroundKind::Translation, Some(axis)) => u[axis].is_zero(),
                (GroundKind::Rotation, Some(axis)) => theta[axis] == 0.0,
                (GroundKind::Translation, None) => {
                    let mut action = Expansion::default();
                    for axis in 0..3 {
                        for &term in &u[axis].terms {
                            action.add_product(ground.direction[axis], term, 0)?;
                        }
                    }
                    action.is_zero()
                }
                (GroundKind::Rotation, None) => {
                    let mut action = Expansion::default();
                    for axis in 0..3 {
                        action.add_product(ground.direction[axis], theta[axis], 0)?;
                    }
                    action.is_zero()
                }
            };
            if !zero {
                return Ok(None);
            }
        }
        Ok(Some(translations))
    }

    /// The node motions `[u, θ]` of `r`, if `r` is an exact null motion and
    /// every component is exactly representable.
    fn exact_motions(&self, r: [f64; 6]) -> Result<Option<Vec<[f64; 6]>>, StructuralError> {
        let Some(translations) = self.null_translations(r)? else {
            return Ok(None);
        };
        let mut motions = Vec::with_capacity(translations.len());
        for u in translations {
            let mut motion = [0.0, 0.0, 0.0, r[3], r[4], r[5]];
            for axis in 0..3 {
                let Some(value) = u[axis].exact_scalar()? else {
                    return Ok(None);
                };
                motion[axis] = value;
            }
            motions.push(motion);
        }
        Ok(Some(motions))
    }

    /// Publishes the verified witness, unless its `[t/L, θ]` is not exact
    /// (RV14-4). With L = 2^e, t_i/L is exact exactly when it is finite and
    /// scales back to t_i: an overflow (tiny L) gives ∞, and a rounding (huge
    /// L, a subnormal quotient) scales back to another value. Then the witness
    /// is refused: the status stays `NumericallyUnresolved`, with
    /// `CONSTRAINED_WITNESS_PARAMETERS_UNREPRESENTABLE`, and nothing is published.
    fn publish(&self, mut result: ConstrainedAssessment, found: Witness) -> ConstrainedAssessment {
        let r = found.parameters;
        let t = [r[0] / self.length, r[1] / self.length, r[2] / self.length];
        if (0..3).any(|i| !(t[i].is_finite() && t[i] * self.length == r[i])) {
            result.unresolved = Some(CONSTRAINED_WITNESS_PARAMETERS_UNREPRESENTABLE);
            return result;
        }
        result.status = RigidBodyStatus::MechanismWitnessed;
        result.unresolved = None;
        result.rigid_parameters = Some([t[0], t[1], t[2], r[3], r[4], r[5]]);
        result.node_motion = Some(found.motions);
        result
    }
}

/// y·p_j = k·p_i exactly.
fn exact_ratio(y: f64, pj: f64, k: f64, pi: f64) -> bool {
    if !y.is_finite() {
        return false;
    }
    let mut e = Expansion::default();
    e.add_product(y, pj, 0).is_ok() && e.add_product(-k, pi, 0).is_ok() && e.is_zero()
}

fn rounded3(v: &[Expansion; 3]) -> Result<[f64; 3], StructuralError> {
    let mut out = [0.0; 3];
    for axis in 0..3 {
        out[axis] = v[axis].round().map_err(|_| RANGE)?;
    }
    Ok(out)
}

fn difference3(a: &[Expansion; 3], b: &[Expansion; 3]) -> Result<[Expansion; 3], StructuralError> {
    let mut out = a.clone();
    for axis in 0..3 {
        for &term in &b[axis].terms {
            out[axis].add(-term).map_err(|_| RANGE)?;
        }
    }
    Ok(out)
}

/// ⌊log₂ |x|⌋ from the bits, for finite nonzero x (subnormals included).
fn exponent_of(x: f64) -> i32 {
    let bits = x.abs().to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    if biased != 0 {
        biased - 1023
    } else {
        -1074 + (63 - (bits & ((1_u64 << 52) - 1)).leading_zeros() as i32)
    }
}

/// 2^e from the bits, for e in −1074..=1023.
fn pow2(e: i32) -> f64 {
    if e >= -1022 {
        f64::from_bits(((e + 1023) as u64) << 52)
    } else {
        f64::from_bits(1_u64 << (e + 1074))
    }
}

/// x·2^e in steps of representable powers of two.
fn scale_pow2(mut x: f64, mut e: i32) -> f64 {
    while e != 0 && x != 0.0 {
        let step = e.clamp(-1022, 1023);
        x *= pow2(step);
        e -= step;
    }
    x
}

/// `v` scaled by a power of two so that its largest |component| is in [1, 2).
fn unit_scaled(v: [f64; 3]) -> [f64; 3] {
    let m = v.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
    if m == 0.0 {
        return v;
    }
    let e = exponent_of(m);
    v.map(|x| scale_pow2(x, -e))
}

/// Pushes `row` scaled to unit length (first by a power of two, then by its
/// `sqrt` norm), unless it is exactly zero.
fn push_normalized(rows: &mut Vec<[f64; 6]>, row: [f64; 6]) {
    let m = row.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
    if !(m > 0.0) {
        return;
    }
    let e = exponent_of(m);
    let row = row.map(|x| scale_pow2(x, -e));
    let mut sum = 0.0_f64;
    for x in row {
        sum += x * x;
    }
    let norm = sum.sqrt();
    rows.push(row.map(|x| x / norm));
}

/// The translation row of axis k at the scaled virtual position w:
/// e_k·(t̃ + θ × w).
fn translation_row(w: [f64; 3], axis: usize) -> [f64; 6] {
    let [x, y, z] = w;
    match axis {
        0 => [1.0, 0.0, 0.0, 0.0, z, -y],
        1 => [0.0, 1.0, 0.0, -z, 0.0, x],
        _ => [0.0, 0.0, 1.0, y, -x, 0.0],
    }
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// √(ζ² + 1) with IEEE operations only (in place of `ζ.hypot(1.0)`). No
/// overflow: for |ζ| ≤ 1, ζ² + 1 ≤ 2; otherwise |ζ|·√((1/ζ)² + 1), whose root
/// argument is at most 2. An infinite ζ gives ∞ (t = 0 in the Jacobi step).
fn root_one_plus_square(zeta: f64) -> f64 {
    let a = zeta.abs();
    if a <= 1.0 {
        (a * a + 1.0).sqrt()
    } else {
        let w = 1.0 / a;
        a * (w * w + 1.0).sqrt()
    }
}

/// x rounded to the nearest multiple of 2^-20 (half away from zero), with
/// integer conversion only; |x| ≤ 64 here.
fn snap_to_grid(x: f64) -> f64 {
    let scaled = x * SNAP_GRID;
    if !(scaled.abs() < 4_503_599_627_370_496.0) {
        return x;
    }
    let n = if scaled >= 0.0 {
        (scaled + 0.5) as i64
    } else {
        (scaled - 0.5) as i64
    };
    n as f64 / SNAP_GRID
}

/// Gauss–Jordan echelon form of `vectors` in binary64: each kept vector has 1
/// at its pivot (its largest remaining component, the lowest index on ties)
/// and 0 at every other kept vector's pivot. Dependent vectors are dropped.
fn reduce_to_echelon(vectors: &mut Vec<[f64; 6]>) {
    let mut reduced: Vec<([f64; 6], usize)> = Vec::new();
    for mut w in std::mem::take(vectors) {
        for (r, pivot) in &reduced {
            let f = w[*pivot];
            if f != 0.0 {
                for i in 0..6 {
                    w[i] -= f * r[i];
                }
            }
            w[*pivot] = 0.0;
        }
        let mut pivot = None;
        let mut best = 0.0_f64;
        for (i, x) in w.iter().enumerate() {
            if reduced.iter().any(|(_, p)| *p == i) {
                continue;
            }
            if x.abs() > best {
                best = x.abs();
                pivot = Some(i);
            }
        }
        let Some(pivot) = pivot else {
            continue;
        };
        if best <= DEPENDENT {
            continue;
        }
        let divisor = w[pivot];
        for x in &mut w {
            *x /= divisor;
        }
        w[pivot] = 1.0;
        for (r, _) in &mut reduced {
            let f = r[pivot];
            if f != 0.0 {
                for i in 0..6 {
                    r[i] -= f * w[i];
                }
            }
            r[pivot] = 0.0;
        }
        reduced.push((w, pivot));
    }
    vectors.extend(reduced.into_iter().map(|(w, _)| w));
}

#[cfg(test)]
mod k5_unit_tests {
    use super::*;

    #[test]
    fn k5_root_one_plus_square_is_exact_where_the_root_is_exact_and_never_overflows() {
        // Exact cases: sqrt of a representable exact square.
        assert_eq!(root_one_plus_square(0.0), 1.0);
        assert_eq!(root_one_plus_square(-0.0), 1.0);
        // 0.75² + 1 = 1.5625 = 1.25².
        assert_eq!(root_one_plus_square(0.75), 1.25);
        assert_eq!(root_one_plus_square(-0.75), 1.25);
        // Large powers of two: (2^-k)² + 1 rounds to 1 for k ≥ 27.
        for k in [27, 100, 511, 1000] {
            let z = pow2(k);
            assert_eq!(root_one_plus_square(z), z, "2^{k}");
            assert_eq!(root_one_plus_square(-z), z, "-2^{k}");
        }
        assert_eq!(root_one_plus_square(f64::MAX), f64::MAX);
        assert_eq!(root_one_plus_square(f64::INFINITY), f64::INFINITY);
        // ζ = ±1: √2, one correctly rounded sqrt on both branches' boundary.
        assert_eq!(root_one_plus_square(1.0), 2.0_f64.sqrt());
        assert_eq!(root_one_plus_square(-1.0), 2.0_f64.sqrt());
        // 1 + ε: the second branch, a finite value between √2 and √2·(1 + ε).
        let above = 1.0 + f64::EPSILON;
        let r = root_one_plus_square(above);
        assert!(r >= 2.0_f64.sqrt() && r <= 2.0_f64.sqrt() * (1.0 + 4.0 * f64::EPSILON));
    }

    #[test]
    fn k5_powers_of_two_come_from_the_bits() {
        assert_eq!(exponent_of(1.0), 0);
        assert_eq!(exponent_of(1.5), 0);
        assert_eq!(exponent_of(2.0), 1);
        assert_eq!(exponent_of(-0.75), -1);
        assert_eq!(exponent_of(f64::MAX), 1023);
        assert_eq!(exponent_of(f64::MIN_POSITIVE), -1022);
        assert_eq!(exponent_of(f64::from_bits(1)), -1074);
        assert_eq!(exponent_of(f64::from_bits(3)), -1073);
        for e in [-1074, -1073, -1023, -1022, -1, 0, 1, 52, 1023] {
            let x = pow2(e);
            assert_eq!(exponent_of(x), e, "{e}");
            assert_eq!(scale_pow2(x, -e), 1.0, "{e}");
        }
        assert_eq!(scale_pow2(f64::from_bits(1), 1074), 1.0);
        assert_eq!(scale_pow2(1.0, -1074), f64::from_bits(1));
        assert_eq!(unit_scaled([3.0, -0.5, 0.0]), [1.5, -0.25, 0.0]);
        assert_eq!(unit_scaled([0.0; 3]), [0.0; 3]);
        assert_eq!(snap_to_grid(3.000_000_01), 3.0);
        assert_eq!(snap_to_grid(-0.25), -0.25);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oblique_geometry_and_actual_rotation_restraint() {
        let coords = [[0.0, 0.0, 0.0], [1.2, 1.6, 0.0]];
        let translations = [0, 1, 2, 6, 7, 8];
        let open = assess_rigid_body(
            &coords,
            &translations,
            ObjectiveFamily::WeldedUnreleasedElasticFrames,
        )
        .unwrap();
        assert_ne!(open.status, RigidBodyStatus::Restrained);
        let mut wrong = translations.to_vec();
        wrong.push(5);
        assert_ne!(
            assess_rigid_body(
                &coords,
                &wrong,
                ObjectiveFamily::WeldedUnreleasedElasticFrames
            )
            .unwrap()
            .status,
            RigidBodyStatus::Restrained
        );
        let mut right = translations.to_vec();
        right.push(3);
        assert_eq!(
            assess_rigid_body(
                &coords,
                &right,
                ObjectiveFamily::WeldedUnreleasedElasticFrames
            )
            .unwrap()
            .status,
            RigidBodyStatus::Restrained
        );
    }
    #[test]
    fn full_ground_rank_does_not_assert_internal_stiffness_rank() {
        let coords = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        let ground = [0, 1, 2, 3, 4, 5];
        assert_eq!(
            assess_rigid_body(
                &coords,
                &ground,
                ObjectiveFamily::WeldedUnreleasedElasticFrames
            )
            .unwrap()
            .status,
            RigidBodyStatus::Restrained
        );
        let k = vec![vec![1.0, 0.0], vec![0.0, 0.0]];
        let f = [0.0, 0.0];
        let free = [0, 1];
        let s = crate::structural::StructuralSystem {
            stiffness: &k,
            force: &f,
            free_dofs: &free,
            prescribed: &[],
            contributions: None,
            symmetry: None,
        };
        assert!(crate::structural::solve_structural_dense(&s).is_err());
        assert_eq!(
            assess_rigid_body(&coords, &ground, ObjectiveFamily::Unqualified)
                .unwrap()
                .status,
            RigidBodyStatus::UnqualifiedFamily
        );
    }
    #[test]
    fn empty_ground_has_explicit_null_and_origin_shift_does_not_change_rank() {
        let coords = [[1e8, 1e8, 1e8], [1e8 + 2.0, 1e8, 1e8]];
        let result =
            assess_rigid_body(&coords, &[], ObjectiveFamily::WeldedUnreleasedElasticFrames)
                .unwrap();
        assert_eq!(result.status, RigidBodyStatus::MechanismWitnessed);
        assert!(result.node_motion.is_some());
        let ground = [0, 1, 2, 3, 4, 5];
        assert_eq!(
            assess_rigid_body(
                &coords,
                &ground,
                ObjectiveFamily::WeldedUnreleasedElasticFrames
            )
            .unwrap()
            .status,
            RigidBodyStatus::Restrained
        );
    }
    #[test]
    fn overflowing_geometry_length_is_range_uncertainty() {
        let coords = [[0.0, 0.0, 0.0], [1.5e308, 1.5e308, 1.5e308]];
        assert!(matches!(
            assess_rigid_body(&coords, &[], ObjectiveFamily::WeldedUnreleasedElasticFrames),
            Err(StructuralError::Range(_))
        ));
    }
    #[test]
    fn krev01_original_geometry_refutes_coarsened_null_witnesses() {
        let examples = [
            [[0.0, 0.0, 0.0], [1e200, 0.0, 0.0], [0.0, 1e-200, 0.0]],
            [
                [1e16, 1e16, 0.0],
                [-1e16, -1e16, 0.0],
                [-1e16, -9999999999999998.0, 0.0],
            ],
        ];
        let ground = [0, 1, 2, 6, 7, 8, 12, 13, 14];
        for coordinates in examples {
            for units in [0.5, 1.0, 2.0] {
                let coordinates = coordinates.map(|p| p.map(|x| x * units));
                let result = assess_rigid_body(
                    &coordinates,
                    &ground,
                    ObjectiveFamily::WeldedUnreleasedElasticFrames,
                )
                .unwrap();
                assert_ne!(result.status, RigidBodyStatus::MechanismWitnessed);
                assert!(result.node_motion.is_none());
            }
        }
        let coords = [[0.0, 0.0, 0.0], [1e200, 0.0, 0.0], [0.0, 1e-200, 0.0]];
        assert!(
            original_rigid_witness(&coords, &ground, 1e200, &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0])
                .unwrap()
                .is_none()
        );
    }
    #[test]
    fn krev01_valid_oblique_witness_survives_origin_and_radix_units() {
        let ground = [0, 1, 2, 6, 7, 8];
        for units in [0.5, 1.0, 2.0] {
            for shift in [0.0, 8.0] {
                let coords = [
                    [shift, shift, 0.0],
                    [shift + 3.0 * units, shift + 4.0 * units, 0.0],
                ];
                let result = assess_rigid_body(
                    &coords,
                    &ground,
                    ObjectiveFamily::WeldedUnreleasedElasticFrames,
                )
                .unwrap();
                assert_eq!(result.status, RigidBodyStatus::MechanismWitnessed);
                for motion in result.node_motion.unwrap() {
                    assert_eq!(&motion[..3], &[0.0, 0.0, 0.0]);
                }
            }
        }
    }
}
