//! T4-U3 (J-A): the objective connector, a symmetric-midpoint, small-rotation,
//! elastic two-node element (JR §2; `ObjectiveConnectorV1`).
//!
//! Kinematics, with d = [u_i, θ_i, u_j, θ_j] (global translations and
//! infinitesimal rotation vectors), node positions x_i, x_j, global attachment
//! offsets a_i, a_j and the connector axes Q (a proper triad whose **columns**
//! are the axes in global coordinates):
//!
//! - r = (x_j − x_i) + (a_j − a_i), formed componentwise from the two
//!   differences. The absolute attachment positions x + a are never formed
//!   (RV130 S-3): at UTM coordinates their rounding would enter r.
//! - q = B d with B_t = Qᵀ[−I, S(a_i) + S(r)/2, I, −S(a_j) + S(r)/2] and
//!   B_r = Qᵀ[0, −I, 0, I], S(v)w = v × w.
//! - g = K(q − q_ref), U = (q − q_ref)ᵀK(q − q_ref)/2, end actions f = Bᵀg
//!   (node on element), Ke = BᵀKB, and the installed-state assembly load
//!   +BᵀK q_ref.
//!
//! Representation (the decode, outside EF): the authored scaled work matrix H
//! (21-entry upper triangle, N·m) with its translation scale Ls gives
//! K = D⁻¹HD⁻¹, D = diag(Ls, Ls, Ls, 1, 1, 1), in binary64; each global offset
//! is a = fl(Q_node · offset_local).
//!
//! Arithmetic:
//! - each entry of B's rotational translation blocks is one exact dot of the
//!   binary64 Q with the rounded sums S(a) ± S(r)/2, rounded once;
//! - each entry of Ke, of BᵀKq_ref, and of the recovered q, q − q_ref, g, Bᵀg
//!   and U is one `ExactAccumulator` sum rounded once (a product of three
//!   binary64 values enters exactly through its error-free split);
//! - the definiteness decision is exact (an LDLᵀ over integers, no square
//!   root and no tolerance), on the authored H and on the binary64 K.
//!
//! The representation guards (orthonormal triads, Q.x along r) are
//! machine-precision-scaled arithmetic guards (CONNECTOR_CONTRACT_V1 §2), not
//! physical installation tolerances.
use crate::exact_sum::{ExactAccumulator, SumError};
use crate::load_ledger::{gamma, product_upward, round_upward, UNIT_ROUNDOFF};
use crate::{
    force_scaled_matrix, force_scaled_value, ForceScale, FrameKernelError, FrameNode, Matrix12,
    Matrix3, DOF_PER_NODE, ELEMENT_DOF,
};

/// A symmetric 6×6 matrix in the connector's generalized coordinates
/// [q_t; q_r].
pub type Matrix6 = [[f64; 6]; 6];
/// The connector's compatibility matrix B (6 × 12).
pub type ConnectorB = [[f64; ELEMENT_DOF]; 6];

/// Orthonormality guard of a triad: |MᵀM − I| ≤ 2^-49 entrywise
/// (dimensionless; a binary64 rounding of an exact rotation is within 3u).
pub const TRIAD_GUARD: f64 = 16.0 * UNIT_ROUNDOFF;
/// Alignment guard of Q.x with r, relative to the position scale
/// max_k(|x_i| + |x_j| + |a_i| + |a_j|)_k: 2^-48.
pub const ALIGNMENT_GUARD: f64 = 32.0 * UNIT_ROUNDOFF;
/// Smallest |a·b| whose error-free split `a·b − fl(a·b)` is exact (DN-2).
const SPLIT_EXACT_MIN: f64 = f64::from_bits(54 << 52); // 2^-969

/// One end's authored attachment: the initial node triad (columns are axes,
/// global) and the offset in that triad. a = fl(node_axes · offset_local).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConnectorAttachment {
    pub node_axes: Matrix3,
    pub offset_local: [f64; 3],
}

impl ConnectorAttachment {
    /// An attachment whose node triad is the global frame: a = offset.
    pub fn global(offset: [f64; 3]) -> Self {
        Self {
            node_axes: IDENTITY,
            offset_local: offset,
        }
    }
}

/// The authored scaled work matrix (`ScaledConnectorWorkMatrixV1`): the
/// upper triangle (0,0)..(0,5), (1,1)..(1,5), …, (5,5) of H in N·m, with
/// the translation scale Ls in m (rotation scale 1 rad).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaledWorkMatrix {
    pub upper_triangle: [f64; 21],
    pub translation_scale: f64,
}

/// The exact inertia class of a symmetric matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorDefiniteness {
    PositiveDefinite,
    /// Singular: at least one zero pivot, every one with a zero remaining
    /// column, and no negative pivot.
    PositiveSemidefinite,
    Indefinite,
}

/// Why the constructor refuses a connector. Through the product this is
/// `OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE` (slot table §4.2).
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectorError {
    /// A non-finite input value.
    NonFinite { name: &'static str },
    /// Both ends name the same node.
    RepeatedNode { node_index: usize },
    /// A triad (Q or a node triad) is not proper orthonormal within
    /// `TRIAD_GUARD`.
    ImproperTriad { name: &'static str },
    /// r ≠ 0 and Q.x is not along +r within `ALIGNMENT_GUARD`.
    AxisMisaligned,
    /// Ls is not positive, finite and normal.
    TranslationScale,
    /// H (or the decoded K) is indefinite: never projected.
    NotPositiveSemidefinite,
    /// A decoded or formed value leaves the binary64 normal range.
    Range { name: &'static str },
}

impl std::fmt::Display for ConnectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFinite { name } => write!(f, "connector input {name} is not finite"),
            Self::RepeatedNode { node_index } => {
                write!(f, "connector connects node index {node_index} to itself")
            }
            Self::ImproperTriad { name } => {
                write!(f, "connector triad {name} is not proper orthonormal")
            }
            Self::AxisMisaligned => write!(f, "connector axis Q.x is not along +r"),
            Self::TranslationScale => {
                write!(
                    f,
                    "connector translation scale must be positive, finite and normal"
                )
            }
            Self::NotPositiveSemidefinite => {
                write!(f, "connector work matrix is not positive semidefinite")
            }
            Self::Range { name } => write!(f, "connector {name} outside the binary64 normal range"),
        }
    }
}

impl std::error::Error for ConnectorError {}

/// +BᵀK q_ref (the installed-state assembly load) and the bound on each
/// entry's formation error from the held operands (S13).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConnectorReferenceLoad {
    pub values: [f64; ELEMENT_DOF],
    pub bounds: [f64; ELEMENT_DOF],
}

/// The connector's recovered state for element displacements d (S14).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConnectorRecovery {
    /// q = B d.
    pub q: [f64; 6],
    /// q − q_ref, each one exact sum rounded once.
    pub deformation: [f64; 6],
    /// g = K(q − q_ref), local.
    pub g: [f64; 6],
    /// U = (q − q_ref)ᵀ g / 2.
    pub energy: f64,
    /// F = Q g_t and M = Q g_r, global.
    pub force_global: [f64; 3],
    pub moment_global: [f64; 3],
    /// f = Bᵀg, node on element: [F_i, M_i, F_j, M_j].
    pub end_actions: [f64; ELEMENT_DOF],
}

const IDENTITY: Matrix3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

/// The objective connector between two distinct nodes (J-A).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectiveConnector {
    node_i: FrameNode,
    node_j: FrameNode,
    a_i: [f64; 3],
    a_j: [f64; 3],
    axes: Matrix3,
    stiffness: ScaledWorkMatrix,
    k: Matrix6,
    q_ref: [f64; 6],
    definiteness: ConnectorDefiniteness,
}

impl ObjectiveConnector {
    /// The decode and the representation checks. Refused: a repeated node,
    /// any non-finite input, an improper triad, Q.x not along +r (r ≠ 0),
    /// Ls not positive normal, an indefinite H or K, and a K or Ke outside
    /// the normal range.
    pub fn new(
        node_i: FrameNode,
        node_j: FrameNode,
        end_i: ConnectorAttachment,
        end_j: ConnectorAttachment,
        axes: Matrix3,
        stiffness: ScaledWorkMatrix,
        q_ref: [f64; 6],
    ) -> Result<Self, ConnectorError> {
        if node_i.index == node_j.index {
            return Err(ConnectorError::RepeatedNode {
                node_index: node_i.index,
            });
        }
        finite3("node_i coordinates", node_i.coordinates)?;
        finite3("node_j coordinates", node_j.coordinates)?;
        for (name, end) in [("end_i", &end_i), ("end_j", &end_j)] {
            for row in end.node_axes {
                finite3(name, row)?;
            }
            finite3(name, end.offset_local)?;
            if !proper_triad(&end.node_axes)? {
                return Err(ConnectorError::ImproperTriad { name });
            }
        }
        for row in axes {
            finite3("connector axes", row)?;
        }
        if !proper_triad(&axes)? {
            return Err(ConnectorError::ImproperTriad {
                name: "connector axes",
            });
        }
        if stiffness.upper_triangle.iter().any(|v| !v.is_finite()) {
            return Err(ConnectorError::NonFinite {
                name: "work matrix",
            });
        }
        let ls = stiffness.translation_scale;
        if !(ls.is_normal() && ls > 0.0) {
            return Err(ConnectorError::TranslationScale);
        }
        if q_ref.iter().any(|v| !v.is_finite()) {
            return Err(ConnectorError::NonFinite { name: "q_ref" });
        }
        let a_i = rotate_offset(&end_i)?;
        let a_j = rotate_offset(&end_j)?;
        let k = decode_work_matrix(&stiffness)?;
        let (_, _, r) = chord(node_i.coordinates, node_j.coordinates, a_i, a_j)?;
        aligned(&axes, [node_i.coordinates, node_j.coordinates, a_i, a_j], r)?;
        let authored = symmetric_from_upper(&stiffness.upper_triangle);
        let definiteness = match (exact_definiteness(&authored), exact_definiteness(&k)) {
            (ConnectorDefiniteness::Indefinite, _) | (_, ConnectorDefiniteness::Indefinite) => {
                return Err(ConnectorError::NotPositiveSemidefinite)
            }
            (ConnectorDefiniteness::PositiveDefinite, ConnectorDefiniteness::PositiveDefinite) => {
                ConnectorDefiniteness::PositiveDefinite
            }
            _ => ConnectorDefiniteness::PositiveSemidefinite,
        };
        let connector = Self {
            node_i,
            node_j,
            a_i,
            a_j,
            axes,
            stiffness,
            k,
            q_ref,
            definiteness,
        };
        connector
            .global_stiffness()
            .map_err(|_| ConnectorError::Range {
                name: "formed stiffness",
            })?;
        Ok(connector)
    }

    pub fn node_i(&self) -> FrameNode {
        self.node_i
    }

    pub fn node_j(&self) -> FrameNode {
        self.node_j
    }

    /// The global offsets a_i, a_j (the decode's binary64 values).
    pub fn offsets(&self) -> ([f64; 3], [f64; 3]) {
        (self.a_i, self.a_j)
    }

    /// Q, row-major, columns are the connector axes.
    pub fn axes(&self) -> Matrix3 {
        self.axes
    }

    /// The authored scaled work matrix, unchanged.
    pub fn work_matrix(&self) -> ScaledWorkMatrix {
        self.stiffness
    }

    /// K = D⁻¹HD⁻¹ in binary64: the stiffness every consumer uses.
    pub fn stiffness(&self) -> Matrix6 {
        self.k
    }

    pub fn q_ref(&self) -> [f64; 6] {
        self.q_ref
    }

    /// The exact decision of the constructor: positive definite only when
    /// both the authored H and the binary64 K are (an indefinite one was
    /// refused). W4 links a PD connector; any other is unqualified (S11).
    pub fn definiteness(&self) -> ConnectorDefiniteness {
        self.definiteness
    }

    /// r = (x_j − x_i) + (a_j − a_i), componentwise in binary64.
    pub fn chord(&self) -> [f64; 3] {
        let (_, _, r) = chord(
            self.node_i.coordinates,
            self.node_j.coordinates,
            self.a_i,
            self.a_j,
        )
        .expect("checked by the constructor");
        r
    }

    /// B (6 × 12) in binary64. Exact blocks: ±Qᵀ and the rotation rows. Each
    /// entry of Qᵀ(S(a_i) + S(r)/2) and Qᵀ(−S(a_j) + S(r)/2) is one exact dot
    /// of Q's column with the once-rounded sum, rounded once.
    pub fn b(&self) -> ConnectorB {
        let r = self.chord();
        let half = [0.5 * r[0], 0.5 * r[1], 0.5 * r[2]];
        let s_r = skew(half);
        let s_ai = skew(self.a_i);
        let s_aj = skew(self.a_j);
        let mut m_i = [[0.0; 3]; 3];
        let mut m_j = [[0.0; 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                m_i[row][col] = s_ai[row][col] + s_r[row][col];
                m_j[row][col] = s_r[row][col] - s_aj[row][col];
            }
        }
        let mut b = [[0.0; ELEMENT_DOF]; 6];
        for axis in 0..3 {
            let column = [self.axes[0][axis], self.axes[1][axis], self.axes[2][axis]];
            for c in 0..3 {
                b[axis][c] = -column[c];
                b[axis][6 + c] = column[c];
                b[3 + axis][3 + c] = -column[c];
                b[3 + axis][9 + c] = column[c];
                b[axis][3 + c] = exact_dot3(column, [m_i[0][c], m_i[1][c], m_i[2][c]])
                    .expect("bounded operands");
                b[axis][9 + c] = exact_dot3(column, [m_j[0][c], m_j[1][c], m_j[2][c]])
                    .expect("bounded operands");
            }
        }
        b
    }

    /// Ke = BᵀKB (12 × 12): each entry Σ_kl B_ki K_kl B_lj is one exact sum
    /// rounded once, so Ke is exactly symmetric. A non-finite entry, or a
    /// product too small for its exact split, is refused.
    pub fn global_stiffness(&self) -> Result<Matrix12, FrameKernelError> {
        let b = self.b();
        let mut ke = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
        for i in 0..ELEMENT_DOF {
            for j in i..ELEMENT_DOF {
                let mut sum = ExactAccumulator::new();
                for k in 0..6 {
                    if b[k][i] == 0.0 {
                        continue;
                    }
                    for l in 0..6 {
                        triple(&mut sum, b[k][i], self.k[k][l], b[l][j])?;
                    }
                }
                let value = sum.round().map_err(|_| range("connector Ke"))?;
                ke[i][j] = value;
                ke[j][i] = value;
            }
        }
        Ok(ke)
    }

    /// K2b (N-5): the formed Ke times 2^b, exactly (`force_scaled_matrix`),
    /// as the assembly adds it.
    pub fn force_scaled_global_stiffness(
        &self,
        scale: ForceScale,
    ) -> Result<Matrix12, FrameKernelError> {
        force_scaled_matrix(
            "connector Ke*2^b (force scale)",
            &self.global_stiffness()?,
            scale,
        )
    }

    /// K2b: this connector with K times 2^b, exactly (K-D5's scaled source).
    /// Its `global_stiffness` is then 2^b·Ke bit for bit wherever both are
    /// normal. H, Ls and q_ref are unchanged.
    pub fn force_scaled(&self, scale: ForceScale) -> Result<Self, FrameKernelError> {
        let mut connector = *self;
        for row in connector.k.iter_mut() {
            for entry in row.iter_mut() {
                *entry = force_scaled_value("connector K*2^b (force scale)", *entry, scale)?;
            }
        }
        Ok(connector)
    }

    /// SA's formation allowance of Ke (symmetry provenance): one rounding of
    /// an exact sum, |fl(s) − s| ≤ u·Σ|B||K||B| + 2^-1074, rounded upward
    /// (0 where every term is zero), with operation count 1.
    pub fn formation_roundoff(&self) -> Result<(Matrix12, [[usize; 12]; 12]), FrameKernelError> {
        let b = self.b();
        let tiny = f64::from_bits(1);
        let mut bounds = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
        for i in 0..ELEMENT_DOF {
            for j in i..ELEMENT_DOF {
                let mut magnitude = ExactAccumulator::new();
                for k in 0..6 {
                    for l in 0..6 {
                        let p = product_upward(b[k][i].abs(), self.k[k][l].abs());
                        magnitude
                            .add(product_upward(p, b[l][j].abs()))
                            .map_err(|_| range("connector allowance"))?;
                    }
                }
                let total = round_upward(&magnitude).map_err(|_| range("connector allowance"))?;
                // Every term zero: the entry is an exact zero, with no rounding.
                if total == 0.0 {
                    continue;
                }
                let bound = (product_upward(gamma(1), total) + tiny).next_up();
                if !bound.is_finite() {
                    return Err(range("connector allowance"));
                }
                bounds[i][j] = bound;
                bounds[j][i] = bound;
            }
        }
        Ok((bounds, [[1; ELEMENT_DOF]; ELEMENT_DOF]))
    }

    /// True when K q_ref = 0 exactly (the stress-free installed state).
    pub fn stress_free(&self) -> bool {
        (0..6).all(|k| {
            let mut sum = ExactAccumulator::new();
            for l in 0..6 {
                sum.add_product(self.k[k][l], self.q_ref[l])
                    .expect("finite operands");
            }
            sum.is_zero()
        })
    }

    /// S13: +BᵀK q_ref, each entry Σ_kl B_ki K_kl q_ref,l in one exact sum
    /// rounded once, with `connector_reference_load_bound`. `None` exactly
    /// when K q_ref = 0 (N-7).
    pub fn reference_load(&self) -> Result<Option<ConnectorReferenceLoad>, FrameKernelError> {
        if self.stress_free() {
            return Ok(None);
        }
        let b = self.b();
        let mut values = [0.0; ELEMENT_DOF];
        for (i, value) in values.iter_mut().enumerate() {
            let mut sum = ExactAccumulator::new();
            for k in 0..6 {
                if b[k][i] == 0.0 {
                    continue;
                }
                for l in 0..6 {
                    triple(&mut sum, b[k][i], self.k[k][l], self.q_ref[l])?;
                }
            }
            *value = sum.round().map_err(|_| range("connector reference load"))?;
        }
        Ok(Some(ConnectorReferenceLoad {
            values,
            bounds: connector_reference_load_bound(self)?,
        }))
    }

    /// This connector's 12 entries of a global displacement vector.
    pub fn element_displacements(&self, u: &[f64]) -> Option<[f64; ELEMENT_DOF]> {
        let mut d = [0.0; ELEMENT_DOF];
        for (slot, dof) in d
            .iter_mut()
            .zip(element_dofs(self.node_i.index, self.node_j.index))
        {
            *slot = *u.get(dof)?;
        }
        d.iter().all(|v| v.is_finite()).then_some(d)
    }

    /// S14: q = B d; each q_k − q_ref,k = Σ_l B_kl d_l − q_ref,k is one exact
    /// sum rounded once; g = K(q − q_ref), Bᵀg, Q g and U are exact sums of
    /// the rounded operands, each rounded once.
    pub fn recover(&self, d: &[f64; ELEMENT_DOF]) -> Result<ConnectorRecovery, FrameKernelError> {
        if d.iter().any(|v| !v.is_finite()) {
            return Err(FrameKernelError::NonFiniteInput {
                name: "connector displacements",
                value: f64::NAN,
            });
        }
        let b = self.b();
        let rounded = |sum: &ExactAccumulator| sum.round().map_err(|_| range("connector recovery"));
        let mut q = [0.0; 6];
        let mut deformation = [0.0; 6];
        for k in 0..6 {
            let mut sum = ExactAccumulator::new();
            for l in 0..ELEMENT_DOF {
                sum.add_product(b[k][l], d[l]).map_err(sum_error)?;
            }
            q[k] = rounded(&sum)?;
            sum.add(-self.q_ref[k]).map_err(sum_error)?;
            deformation[k] = rounded(&sum)?;
        }
        let mut g = [0.0; 6];
        for k in 0..6 {
            let mut sum = ExactAccumulator::new();
            for l in 0..6 {
                sum.add_product(self.k[k][l], deformation[l])
                    .map_err(sum_error)?;
            }
            g[k] = rounded(&sum)?;
        }
        let mut energy_sum = ExactAccumulator::new();
        for k in 0..6 {
            energy_sum
                .add_product(deformation[k], g[k])
                .map_err(sum_error)?;
        }
        let energy = energy_sum
            .round_scaled(-1)
            .map_err(|_| range("connector energy"))?;
        let mut force_global = [0.0; 3];
        let mut moment_global = [0.0; 3];
        for row in 0..3 {
            force_global[row] = exact_dot3(self.axes[row], [g[0], g[1], g[2]])?;
            moment_global[row] = exact_dot3(self.axes[row], [g[3], g[4], g[5]])?;
        }
        let mut end_actions = [0.0; ELEMENT_DOF];
        for (i, action) in end_actions.iter_mut().enumerate() {
            let mut sum = ExactAccumulator::new();
            for k in 0..6 {
                sum.add_product(b[k][i], g[k]).map_err(sum_error)?;
            }
            *action = rounded(&sum)?;
        }
        Ok(ConnectorRecovery {
            q,
            deformation,
            g,
            energy,
            force_global,
            moment_global,
            end_actions,
        })
    }
}

/// S13 (RV130 S-4, N-e): a bound on |fl(t) − t| for each entry of
/// t = BᵀK q_ref, where B is the exact matrix of the held binary64 operands
/// (x_i, x_j, a_i, a_j, Q, K, q_ref) and fl(t) is `reference_load`'s value.
///
/// With G = |K||q_ref| and the operand form Mop = |S(a)| + |S(r_op)|/2,
/// r_op = |x_j − x_i| + |a_j − a_i| per component (never |r̂|):
/// - r̂ rounds three times (two differences and their sum): |r̂ − r| ≤ γ3·r_op;
/// - each sum S(a) ± S(r̂)/2 rounds once, so |M̂ − M| ≤ γ4·Mop;
/// - each Qᵀ·M̂ dot rounds once: |B̂ − B| ≤ γ5·|Q|ᵀMop on those blocks
///   (zero on the exact blocks);
/// - the entry rounds once: u·Σ_k |B̂_ki| G_k.
///
/// Bound_i = γ1·Σ_k |B̂_ki| G_k + Σ_k ΔB_ki G_k + 2^-1074, every product and
/// sum rounded upward.
pub fn connector_reference_load_bound(
    connector: &ObjectiveConnector,
) -> Result<[f64; ELEMENT_DOF], FrameKernelError> {
    let b = connector.b();
    let k = connector.k;
    let mut g_op = [0.0; 6];
    for (row, slot) in g_op.iter_mut().enumerate() {
        let mut sum = ExactAccumulator::new();
        for l in 0..6 {
            sum.add(product_upward(k[row][l].abs(), connector.q_ref[l].abs()))
                .map_err(sum_error)?;
        }
        *slot = round_upward(&sum).map_err(sum_error)?;
    }
    let delta = rotational_operand_bound(connector)?;
    let tiny = f64::from_bits(1);
    let mut bounds = [0.0; ELEMENT_DOF];
    for (i, bound) in bounds.iter_mut().enumerate() {
        let mut rounding = ExactAccumulator::new();
        let mut operand = ExactAccumulator::new();
        for row in 0..6 {
            rounding
                .add(product_upward(b[row][i].abs(), g_op[row]))
                .map_err(sum_error)?;
            operand
                .add(product_upward(delta[row][i], g_op[row]))
                .map_err(sum_error)?;
        }
        let mut total = ExactAccumulator::new();
        total
            .add(product_upward(
                gamma(1),
                round_upward(&rounding).map_err(sum_error)?,
            ))
            .map_err(sum_error)?;
        total
            .add(round_upward(&operand).map_err(sum_error)?)
            .map_err(sum_error)?;
        total.add(tiny).map_err(sum_error)?;
        *bound = round_upward(&total).map_err(sum_error)?;
        if !bound.is_finite() {
            return Err(range("connector reference load bound"));
        }
    }
    Ok(bounds)
}

/// ΔB: an entrywise bound on |B̂ − B| (zero on the exact blocks).
fn rotational_operand_bound(
    connector: &ObjectiveConnector,
) -> Result<[[f64; ELEMENT_DOF]; 6], FrameKernelError> {
    let (xi, xj) = (connector.node_i.coordinates, connector.node_j.coordinates);
    let (ai, aj) = (connector.a_i, connector.a_j);
    let mut r_op = [0.0; 3];
    for c in 0..3 {
        let mut total = ExactAccumulator::new();
        total
            .add(difference_magnitude_upward(xj[c], xi[c]).map_err(sum_error)?)
            .map_err(sum_error)?;
        total
            .add(difference_magnitude_upward(aj[c], ai[c]).map_err(sum_error)?)
            .map_err(sum_error)?;
        r_op[c] = round_upward(&total).map_err(sum_error)?;
    }
    // Mop = |S(a)| + |S(r_op)|/2: the same pattern for both ends.
    let half = r_op.map(|v| (0.5 * v).next_up());
    let operand = |a: [f64; 3]| -> Result<[[f64; 3]; 3], FrameKernelError> {
        let s_a = skew(a.map(f64::abs));
        let s_r = skew(half);
        let mut m = [[0.0; 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                let mut sum = ExactAccumulator::new();
                sum.add(s_a[row][col].abs()).map_err(sum_error)?;
                sum.add(s_r[row][col].abs()).map_err(sum_error)?;
                m[row][col] = round_upward(&sum).map_err(sum_error)?;
            }
        }
        Ok(m)
    };
    let m_i = operand(ai)?;
    let m_j = operand(aj)?;
    let factor = gamma(5);
    let mut delta = [[0.0; ELEMENT_DOF]; 6];
    for axis in 0..3 {
        for c in 0..3 {
            for (block, m) in [(3, &m_i), (9, &m_j)] {
                let mut sum = ExactAccumulator::new();
                for row in 0..3 {
                    sum.add(product_upward(connector.axes[row][axis].abs(), m[row][c]))
                        .map_err(sum_error)?;
                }
                delta[axis][block + c] =
                    product_upward(factor, round_upward(&sum).map_err(sum_error)?);
            }
        }
    }
    Ok(delta)
}

/// |x − y| (exact), rounded upward.
fn difference_magnitude_upward(x: f64, y: f64) -> Result<f64, SumError> {
    let mut sum = ExactAccumulator::new();
    sum.add(x)?;
    sum.add(-y)?;
    if sum.signum() < 0 {
        sum = ExactAccumulator::new();
        sum.add(y)?;
        sum.add(-x)?;
    }
    round_upward(&sum)
}

/// The global DOFs of a connector between `node_i` and `node_j`.
fn element_dofs(node_i: usize, node_j: usize) -> [usize; ELEMENT_DOF] {
    let mut map = [0; ELEMENT_DOF];
    for local in 0..DOF_PER_NODE {
        map[local] = node_i * DOF_PER_NODE + local;
        map[DOF_PER_NODE + local] = node_j * DOF_PER_NODE + local;
    }
    map
}

fn range(name: &'static str) -> FrameKernelError {
    FrameKernelError::NumericalRange { name }
}

fn sum_error(_: SumError) -> FrameKernelError {
    range("connector exact sum")
}

fn finite3(name: &'static str, v: [f64; 3]) -> Result<(), ConnectorError> {
    if v.iter().all(|x| x.is_finite()) {
        Ok(())
    } else {
        Err(ConnectorError::NonFinite { name })
    }
}

/// Adds a·b·c exactly: a·(b·c) with b·c split error-free into p + e.
fn triple(sum: &mut ExactAccumulator, a: f64, b: f64, c: f64) -> Result<(), FrameKernelError> {
    if a == 0.0 || b == 0.0 || c == 0.0 {
        return Ok(());
    }
    let p = b * c;
    if !p.is_finite() || p.abs() < SPLIT_EXACT_MIN {
        return Err(range("connector product split"));
    }
    let e = b.mul_add(c, -p);
    sum.add_product(a, p).map_err(sum_error)?;
    sum.add_product(a, e).map_err(sum_error)
}

fn exact_dot3(a: [f64; 3], b: [f64; 3]) -> Result<f64, FrameKernelError> {
    let mut sum = ExactAccumulator::new();
    for k in 0..3 {
        sum.add_product(a[k], b[k]).map_err(sum_error)?;
    }
    sum.round().map_err(sum_error)
}

/// S(v): S(v)w = v × w.
fn skew(v: [f64; 3]) -> [[f64; 3]; 3] {
    [[0.0, -v[2], v[1]], [v[2], 0.0, -v[0]], [-v[1], v[0], 0.0]]
}

/// Whether the columns of `m` form a proper orthonormal triad: each entry of
/// MᵀM (one exact dot, rounded once) within `TRIAD_GUARD` of I, and
/// det M > 0.
fn proper_triad(m: &Matrix3) -> Result<bool, ConnectorError> {
    let column = |c: usize| [m[0][c], m[1][c], m[2][c]];
    for a in 0..3 {
        for b in a..3 {
            let g = exact_dot3(column(a), column(b))
                .map_err(|_| ConnectorError::Range { name: "triad" })?;
            let target = if a == b { 1.0 } else { 0.0 };
            if (g - target).abs() > TRIAD_GUARD {
                return Ok(false);
            }
        }
    }
    let (x, y, z) = (column(0), column(1), column(2));
    let normal = [
        y[1] * z[2] - y[2] * z[1],
        y[2] * z[0] - y[0] * z[2],
        y[0] * z[1] - y[1] * z[0],
    ];
    Ok(x[0] * normal[0] + x[1] * normal[1] + x[2] * normal[2] > 0.0)
}

/// a = fl(Q_node · offset): each component one exact dot, rounded once.
fn rotate_offset(end: &ConnectorAttachment) -> Result<[f64; 3], ConnectorError> {
    let mut a = [0.0; 3];
    for (row, slot) in a.iter_mut().enumerate() {
        *slot = exact_dot3(end.node_axes[row], end.offset_local).map_err(|_| {
            ConnectorError::Range {
                name: "attachment offset",
            }
        })?;
    }
    Ok(a)
}

/// (x_j − x_i, a_j − a_i, r) in binary64, componentwise. r's nonzero
/// components must be at least 2^-1021, so that r/2 is exact.
fn chord(
    xi: [f64; 3],
    xj: [f64; 3],
    ai: [f64; 3],
    aj: [f64; 3],
) -> Result<([f64; 3], [f64; 3], [f64; 3]), ConnectorError> {
    let difference = [xj[0] - xi[0], xj[1] - xi[1], xj[2] - xi[2]];
    let offsets = [aj[0] - ai[0], aj[1] - ai[1], aj[2] - ai[2]];
    let r = [
        difference[0] + offsets[0],
        difference[1] + offsets[1],
        difference[2] + offsets[2],
    ];
    let smallest_halvable = 2.0 * f64::MIN_POSITIVE;
    for value in difference.iter().chain(&offsets).chain(&r) {
        if !value.is_finite() || (*value != 0.0 && value.abs() < smallest_halvable) {
            return Err(ConnectorError::Range { name: "chord r" });
        }
    }
    Ok((difference, offsets, r))
}

/// For r ≠ 0 beyond the representation guard, Q.x must lie along +r:
/// |Q.x × r| ≤ g and Q.x·r > 0. The guard is `ALIGNMENT_GUARD` times the
/// position scale P = max_k(|x_i| + |x_j| + |a_i| + |a_j|)_k, the size of
/// the authored positions' own representation error (CONNECTOR_CONTRACT_V1
/// §2: a position/length scale, separate from the triad scale). A chord
/// within g of zero is a coincident attachment: the authored Q is used as
/// given, never replaced.
fn aligned(axes: &Matrix3, positions: [[f64; 3]; 4], r: [f64; 3]) -> Result<(), ConnectorError> {
    let scale = |k: usize| {
        positions[0][k].abs()
            + positions[1][k].abs()
            + positions[2][k].abs()
            + positions[3][k].abs()
    };
    let guard = ALIGNMENT_GUARD * scale(0).max(scale(1)).max(scale(2));
    if r.iter().all(|v| v.abs() <= guard) {
        return Ok(());
    }
    let x = [axes[0][0], axes[1][0], axes[2][0]];
    let cross = [
        x[1] * r[2] - x[2] * r[1],
        x[2] * r[0] - x[0] * r[2],
        x[0] * r[1] - x[1] * r[0],
    ];
    let along = x[0] * r[0] + x[1] * r[1] + x[2] * r[2];
    if cross.iter().any(|v| v.abs() > guard) || !(along > 0.0) {
        return Err(ConnectorError::AxisMisaligned);
    }
    Ok(())
}

/// The full symmetric matrix of a 21-entry upper triangle.
pub fn symmetric_from_upper(upper: &[f64; 21]) -> Matrix6 {
    let mut m = [[0.0; 6]; 6];
    let mut index = 0;
    for i in 0..6 {
        for j in i..6 {
            m[i][j] = upper[index];
            m[j][i] = upper[index];
            index = index.wrapping_add(1);
        }
    }
    m
}

/// K = D⁻¹HD⁻¹: a translation index divides once by Ls, so K_tt = (H/Ls)/Ls,
/// K_tr = H/Ls and K_rr = H. A nonzero entry that leaves the normal range
/// is refused.
fn decode_work_matrix(stiffness: &ScaledWorkMatrix) -> Result<Matrix6, ConnectorError> {
    let h = symmetric_from_upper(&stiffness.upper_triangle);
    let ls = stiffness.translation_scale;
    let mut k = [[0.0; 6]; 6];
    for i in 0..6 {
        for j in i..6 {
            let mut value = h[i][j];
            if i < 3 {
                value /= ls;
            }
            if j < 3 {
                value /= ls;
            }
            if h[i][j] != 0.0 && !value.is_normal() {
                return Err(ConnectorError::Range {
                    name: "decoded stiffness",
                });
            }
            k[i][j] = value;
            k[j][i] = value;
        }
    }
    Ok(k)
}

// ------------------------------------------------------- exact inertia (N-8)

/// A signed integer of arbitrary size: sign and little-endian 64-bit limbs,
/// with no leading zero limb (zero is the empty magnitude).
#[derive(Debug, Clone, PartialEq, Eq)]
struct Integer {
    negative: bool,
    magnitude: Vec<u64>,
}

impl Integer {
    fn zero() -> Self {
        Self {
            negative: false,
            magnitude: Vec::new(),
        }
    }

    /// value·2^shift for a 64-bit magnitude.
    fn shifted(negative: bool, value: u64, shift: u32) -> Self {
        if value == 0 {
            return Self::zero();
        }
        let words = (shift / 64) as usize;
        let bits = shift % 64;
        let mut magnitude = vec![0; words];
        let wide = u128::from(value) << bits;
        magnitude.push(wide as u64);
        magnitude.push((wide >> 64) as u64);
        let mut result = Self {
            negative,
            magnitude,
        };
        result.trim();
        result
    }

    fn trim(&mut self) {
        while self.magnitude.last() == Some(&0) {
            self.magnitude.pop();
        }
        if self.magnitude.is_empty() {
            self.negative = false;
        }
    }

    fn is_zero(&self) -> bool {
        self.magnitude.is_empty()
    }

    fn signum(&self) -> i8 {
        if self.is_zero() {
            0
        } else if self.negative {
            -1
        } else {
            1
        }
    }

    fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        let mut magnitude = vec![0u64; self.magnitude.len() + other.magnitude.len()];
        for (i, &a) in self.magnitude.iter().enumerate() {
            let mut carry = 0u128;
            for (j, &b) in other.magnitude.iter().enumerate() {
                let t = u128::from(a) * u128::from(b) + u128::from(magnitude[i + j]) + carry;
                magnitude[i + j] = t as u64;
                carry = t >> 64;
            }
            let mut position = i + other.magnitude.len();
            while carry != 0 {
                let t = u128::from(magnitude[position]) + carry;
                magnitude[position] = t as u64;
                carry = t >> 64;
                position = position.wrapping_add(1);
            }
        }
        let mut result = Self {
            negative: self.negative != other.negative,
            magnitude,
        };
        result.trim();
        result
    }

    /// self − other.
    fn sub(&self, other: &Self) -> Self {
        let negated = Self {
            negative: !other.negative && !other.is_zero(),
            magnitude: other.magnitude.clone(),
        };
        if self.negative == negated.negative {
            let mut result = Self {
                negative: self.negative,
                magnitude: add_magnitudes(&self.magnitude, &negated.magnitude),
            };
            result.trim();
            return result;
        }
        match compare_magnitudes(&self.magnitude, &negated.magnitude) {
            std::cmp::Ordering::Equal => Self::zero(),
            std::cmp::Ordering::Greater => {
                let mut result = Self {
                    negative: self.negative,
                    magnitude: subtract_magnitudes(&self.magnitude, &negated.magnitude),
                };
                result.trim();
                result
            }
            std::cmp::Ordering::Less => {
                let mut result = Self {
                    negative: negated.negative,
                    magnitude: subtract_magnitudes(&negated.magnitude, &self.magnitude),
                };
                result.trim();
                result
            }
        }
    }
}

fn add_magnitudes(a: &[u64], b: &[u64]) -> Vec<u64> {
    let (long, short) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    let mut out = Vec::with_capacity(long.len() + 1);
    let mut carry = 0u128;
    for (i, &word) in long.iter().enumerate() {
        let t = u128::from(word) + u128::from(short.get(i).copied().unwrap_or(0)) + carry;
        out.push(t as u64);
        carry = t >> 64;
    }
    out.push(carry as u64);
    out
}

/// a − b for a ≥ b.
fn subtract_magnitudes(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = Vec::with_capacity(a.len());
    let mut borrow = 0u64;
    for (i, &word) in a.iter().enumerate() {
        let (d1, o1) = word.overflowing_sub(b.get(i).copied().unwrap_or(0));
        let (d2, o2) = d1.overflowing_sub(borrow);
        out.push(d2);
        borrow = u64::from(o1 || o2);
    }
    out
}

fn compare_magnitudes(a: &[u64], b: &[u64]) -> std::cmp::Ordering {
    a.len()
        .cmp(&b.len())
        .then_with(|| a.iter().rev().cmp(b.iter().rev()))
}

/// (negative, integer significand, binary exponent of its unit bit).
fn decompose(value: f64) -> (bool, u64, i32) {
    let bits = value.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & ((1u64 << 52) - 1);
    let (significand, unit) = if exponent == 0 {
        (fraction, -1074)
    } else {
        (fraction | (1u64 << 52), exponent - 1075)
    };
    (value.is_sign_negative(), significand, unit)
}

/// The exact inertia class of a symmetric binary64 matrix (N-8; T4-I12
/// round 02 N-2). Every entry is an integer times 2^e_min, so the decision
/// is taken on that integer matrix: a symmetric elimination in the given
/// order, each step multiplying the remaining block by its positive pivot
/// (division free, so the remaining block is a positive multiple of the
/// Schur complement). A negative pivot is indefinite; a zero pivot whose
/// remaining column is nonzero is indefinite (a 2×2 principal minor
/// [[0, b], [b, c]] with b ≠ 0 is negative), never skipped; a zero pivot
/// with a zero remaining column is a semidefinite direction.
pub fn exact_definiteness(matrix: &Matrix6) -> ConnectorDefiniteness {
    let parts = matrix.map(|row| row.map(decompose));
    let lowest = parts
        .iter()
        .flatten()
        .filter(|(_, significand, _)| *significand != 0)
        .map(|(_, _, unit)| *unit)
        .min();
    let Some(lowest) = lowest else {
        return ConnectorDefiniteness::PositiveSemidefinite;
    };
    let mut a: Vec<Vec<Integer>> = parts
        .iter()
        .map(|row| {
            row.iter()
                .map(|&(negative, significand, unit)| {
                    Integer::shifted(negative, significand, (unit - lowest) as u32)
                })
                .collect()
        })
        .collect();
    let n = a.len();
    let mut singular = false;
    for k in 0..n {
        let pivot = a[k][k].clone();
        match pivot.signum() {
            -1 => return ConnectorDefiniteness::Indefinite,
            0 => {
                if (k + 1..n).any(|j| !a[k][j].is_zero()) {
                    return ConnectorDefiniteness::Indefinite;
                }
                singular = true;
            }
            _ => {
                for i in k + 1..n {
                    for j in i..n {
                        let updated = pivot.mul(&a[i][j]).sub(&a[i][k].mul(&a[k][j]));
                        a[i][j] = updated.clone();
                        a[j][i] = updated;
                    }
                }
            }
        }
    }
    if singular {
        ConnectorDefiniteness::PositiveSemidefinite
    } else {
        ConnectorDefiniteness::PositiveDefinite
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn integer(value: i64) -> Integer {
        Integer::shifted(value < 0, value.unsigned_abs(), 0)
    }

    #[test]
    fn integer_arithmetic_is_exact() {
        let a = Integer::shifted(false, u64::MAX, 70);
        let b = Integer::shifted(true, 3, 5);
        let p = a.mul(&b);
        assert!(p.negative);
        // (2^64 − 1)·2^70 · 3·2^5 = 3(2^64 − 1)·2^75.
        let expected = Integer::shifted(false, u64::MAX, 75)
            .mul(&integer(3))
            .mul(&integer(-1));
        assert_eq!(p, expected);
        assert_eq!(integer(5).sub(&integer(7)), integer(-2));
        assert_eq!(integer(-5).sub(&integer(-7)), integer(2));
        assert_eq!(integer(-5).sub(&integer(7)), integer(-12));
        assert!(integer(9).sub(&integer(9)).is_zero());
        let big = Integer::shifted(false, 1, 200);
        assert_eq!(big.sub(&integer(1)).sub(&big), integer(-1));
    }

    #[test]
    fn inertia_of_small_matrices() {
        let mut m = [[0.0; 6]; 6];
        for (i, row) in m.iter_mut().enumerate() {
            row[i] = 1.0 + i as f64;
        }
        assert_eq!(
            exact_definiteness(&m),
            ConnectorDefiniteness::PositiveDefinite
        );
        m[2][2] = 0.0;
        assert_eq!(
            exact_definiteness(&m),
            ConnectorDefiniteness::PositiveSemidefinite
        );
        m[2][3] = 0.5;
        m[3][2] = 0.5;
        assert_eq!(exact_definiteness(&m), ConnectorDefiniteness::Indefinite);
        m[2][3] = 0.0;
        m[3][2] = 0.0;
        m[4][4] = -1e-300;
        assert_eq!(exact_definiteness(&m), ConnectorDefiniteness::Indefinite);
        // A wide exponent range: 2^-1000 against 2^1000 stays exact.
        let mut wide = [[0.0; 6]; 6];
        for (i, row) in wide.iter_mut().enumerate() {
            row[i] = if i % 2 == 0 {
                2f64.powi(1000)
            } else {
                2f64.powi(-1000)
            };
        }
        wide[0][1] = 2f64.powi(0);
        wide[1][0] = 2f64.powi(0);
        // det of the leading 2×2 = 2^1000·2^-1000 − 1 = 0: singular there,
        // with a zero remaining column after the elimination.
        assert_eq!(
            exact_definiteness(&wide),
            ConnectorDefiniteness::PositiveSemidefinite
        );
        wide[0][1] = 1.5;
        wide[1][0] = 1.5;
        assert_eq!(exact_definiteness(&wide), ConnectorDefiniteness::Indefinite);
    }
}
