//! T4-U5b: an explicit bilateral linear elastic tie rod
//! (`explicit_axial_rods_v1`, law `bilateral_linear`) on an objective
//! connector (DEL-04-07 Design `u4b-u5b-implementation.md` §3.2).
//!
//! Each rod is its own two-node element beside its connector. Its end nodes
//! are always the connector's end nodes; its offsets are authored in the
//! connector axes Q. With d = [u_i, θ_i, u_j, θ_j] (global translations and
//! infinitesimal rotation vectors):
//!
//! - o = fl(Q·offset_local), each component one exact dot rounded once, as a
//!   connector attachment is formed;
//! - the represented difference d = (x_j − x_i) + (o_j − o_i), formed
//!   componentwise from the two differences (the absolute attachment points
//!   x + o are never formed, as the connector forms r);
//! - l = |d| correctly rounded (`correct_norm::norm3`), refused when zero;
//!   n = d/l, each component one rounding;
//! - e = n·(v_bj − v_bi) = C d with v_b = u + θ × o, so
//!   C = [−nᵀ, −(o_i × n)ᵀ, nᵀ, (o_j × n)ᵀ]; each rotational entry
//!   (o × n)_k = n·(e_k × o) is one exact dot of n with the binary64 offset
//!   cross product, rounded once;
//! - Ke = Cᵀ k C and the reference load +Cᵀ k e_ref, each entry one
//!   `ExactAccumulator` sum rounded once (a product of three binary64 values
//!   enters exactly through its error-free split), so Ke is exactly symmetric;
//! - recovery T = k(C d − e_ref), tension positive (e_ref < 0 is pretension),
//!   and the rod's nodal action CᵀT (node on element).
//!
//! Rods are never connector member loads: the connector's g = K(q − q_ref),
//! its rows and its q_ref primitive are unchanged. Rods carry no pressure.
use crate::connector::ObjectiveConnector;
use crate::exact_sum::{ExactAccumulator, SumError};
use crate::load_ledger::{gamma, product_upward, round_upward};
use crate::{
    force_scaled_matrix, force_scaled_value, ForceScale, FrameKernelError, FrameNode, Matrix12,
    Matrix3, DOF_PER_NODE, ELEMENT_DOF,
};

/// The K-D5 closure family of a rod's reference load
/// (`LoadFormula::TieRodReference`). The product's summand identity is
/// `{connector}:{rod}:tie_rod_reference`.
pub const TIE_ROD_REFERENCE_FAMILY: &str = "tie_rod_reference";

/// The degenerate-length guard's factor 32u = 2^-48 (manager ruling, K1
/// repair 2): a rod is refused as `ZeroLength` when
/// l ≤ 2^-48·(‖x_j − x_i‖∞ + ‖o_i‖∞ + ‖o_j‖∞).
pub const LENGTH_GUARD: f64 = 1.0 / 281_474_976_710_656.0; // 2^-48

/// Smallest |a·b| whose error-free split `a·b − fl(a·b)` is exact (DN-2).
const SPLIT_EXACT_MIN: f64 = f64::from_bits(54 << 52); // 2^-969

/// One rod's authored mechanical inputs. Offsets are in the connector axes Q
/// (the columns of `ObjectiveConnector::axes`), from the connector's end
/// nodes i and j.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TieRodInput {
    pub offset_i: [f64; 3],
    pub offset_j: [f64; 3],
    /// k_r, N/m: finite and positive.
    pub stiffness: f64,
    /// e_ref, m: finite; negative is pretension.
    pub reference_extension: f64,
}

/// Why a rod is refused. Through the product, `ZeroLength` is
/// `JOINT_TIE_ROD_ZERO_LENGTH`; the others are input findings
/// (`JOINT_TIE_ROD_INPUT_INVALID`) or a formation range refusal.
#[derive(Debug, Clone, PartialEq)]
pub enum TieRodError {
    /// A non-finite input value.
    NonFinite { name: &'static str },
    /// k_r is not positive, finite and normal.
    Stiffness,
    /// The represented difference d = (x_j − x_i) + (o_j − o_i) is zero or
    /// degenerate: l = |d| ≤ 32u·(‖x_j − x_i‖∞ + ‖o_i‖∞ + ‖o_j‖∞), u = 2^-53
    /// (`LENGTH_GUARD`), decided exactly. A representation guard, like the
    /// connector's `ALIGNMENT_GUARD`, not a physical tolerance: below it the
    /// direction n is rounding noise of the operands.
    ZeroLength,
    /// A formed value leaves the binary64 range the element needs.
    Range { name: &'static str },
}

impl std::fmt::Display for TieRodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFinite { name } => write!(f, "tie rod input {name} is not finite"),
            Self::Stiffness => write!(f, "tie rod stiffness must be positive, finite and normal"),
            Self::ZeroLength => write!(
                f,
                "tie rod length is zero: its attachment points coincide in the represented difference (x_j - x_i) + (o_j - o_i)"
            ),
            Self::Range { name } => write!(f, "tie rod {name} outside the binary64 range"),
        }
    }
}

impl std::error::Error for TieRodError {}

/// +Cᵀ k e_ref and a bound on each entry's distance from the exact value of
/// the authored operands (x_i, x_j, Q, offsets, k, e_ref).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TieRodReferenceLoad {
    pub values: [f64; ELEMENT_DOF],
    pub bounds: [f64; ELEMENT_DOF],
}

/// A rod's recovered binary64 state for element displacements d.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TieRodRecovery {
    /// e = C d, one exact sum rounded once.
    pub extension: f64,
    /// e − e_ref, one exact sum rounded once.
    pub elongation: f64,
    /// T = k(e − e_ref), tension positive.
    pub tension: f64,
    /// CᵀT, node on element: [F_i, M_i, F_j, M_j], global, moments about
    /// the node.
    pub nodal_action: [f64; ELEMENT_DOF],
}

/// An explicit bilateral linear elastic tie rod between the end nodes of an
/// objective connector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TieRod {
    node_i: FrameNode,
    node_j: FrameNode,
    axes: Matrix3,
    offset_i_local: [f64; 3],
    offset_j_local: [f64; 3],
    o_i: [f64; 3],
    o_j: [f64; 3],
    stiffness: f64,
    reference_extension: f64,
}

impl TieRod {
    /// The rod on `connector`'s end nodes and axes. Refused: a non-finite
    /// offset or reference extension, a stiffness that is not positive
    /// normal, a zero or degenerate represented length (`LENGTH_GUARD`), and
    /// a direction, length, offset arm (o × n), formed stiffness or
    /// reference load outside the binary64 range (including |k·e_ref| below
    /// the exact-split floor). Every infallible accessor below is
    /// representable once this succeeds (force scaling changes only k).
    pub fn new(connector: &ObjectiveConnector, input: TieRodInput) -> Result<Self, TieRodError> {
        finite3("offset_i", input.offset_i)?;
        finite3("offset_j", input.offset_j)?;
        if !input.reference_extension.is_finite() {
            return Err(TieRodError::NonFinite {
                name: "reference extension",
            });
        }
        if !(input.stiffness.is_normal() && input.stiffness > 0.0) {
            return Err(TieRodError::Stiffness);
        }
        let axes = connector.axes();
        let o_i = rotate(&axes, input.offset_i)?;
        let o_j = rotate(&axes, input.offset_j)?;
        let rod = Self {
            node_i: connector.node_i(),
            node_j: connector.node_j(),
            axes,
            offset_i_local: input.offset_i,
            offset_j_local: input.offset_j,
            o_i,
            o_j,
            stiffness: input.stiffness,
            reference_extension: input.reference_extension,
        };
        rod.direction_and_length()?;
        rod.try_c()?;
        rod.global_stiffness().map_err(|_| TieRodError::Range {
            name: "formed stiffness",
        })?;
        rod.reference_load().map_err(|_| TieRodError::Range {
            name: "reference load",
        })?;
        Ok(rod)
    }

    pub fn node_i(&self) -> FrameNode {
        self.node_i
    }

    pub fn node_j(&self) -> FrameNode {
        self.node_j
    }

    /// Q, row-major, columns are the connector axes (the frame of the
    /// authored offsets).
    pub fn axes(&self) -> Matrix3 {
        self.axes
    }

    /// The authored offsets in the connector axes, unchanged.
    pub fn local_offsets(&self) -> ([f64; 3], [f64; 3]) {
        (self.offset_i_local, self.offset_j_local)
    }

    /// The global offsets o_i, o_j = fl(Q·offset) (`offset_global_m`).
    pub fn offsets(&self) -> ([f64; 3], [f64; 3]) {
        (self.o_i, self.o_j)
    }

    /// k_r, N/m (at 2^b on a force-scaled rod).
    pub fn stiffness(&self) -> f64 {
        self.stiffness
    }

    /// e_ref, m.
    pub fn reference_extension(&self) -> f64 {
        self.reference_extension
    }

    /// The represented difference d = (x_j − x_i) + (o_j − o_i),
    /// componentwise in binary64.
    pub fn chord(&self) -> [f64; 3] {
        difference(
            self.node_i.coordinates,
            self.node_j.coordinates,
            self.o_i,
            self.o_j,
        )
        .expect("checked by the constructor")
    }

    /// l = |d|, correctly rounded (`length_m`).
    pub fn length(&self) -> f64 {
        self.direction_and_length()
            .expect("checked by the constructor")
            .1
    }

    /// n = d/l, each component rounded once (`direction_global`).
    pub fn direction(&self) -> [f64; 3] {
        self.direction_and_length()
            .expect("checked by the constructor")
            .0
    }

    /// n·Q_x, one exact dot rounded once (`axial_share`).
    pub fn axial_share(&self) -> f64 {
        let n = self.direction();
        exact_dot3(n, [self.axes[0][0], self.axes[1][0], self.axes[2][0]])
            .expect("bounded operands")
    }

    fn direction_and_length(&self) -> Result<([f64; 3], f64), TieRodError> {
        let d = difference(
            self.node_i.coordinates,
            self.node_j.coordinates,
            self.o_i,
            self.o_j,
        )?;
        if d.iter().all(|v| *v == 0.0) || degenerate(self, d)? {
            return Err(TieRodError::ZeroLength);
        }
        let l = open_pipe_stress_correct_norm::norm3(d[0], d[1], d[2]);
        if !l.is_normal() {
            return Err(TieRodError::Range { name: "length" });
        }
        let n = [d[0] / l, d[1] / l, d[2] / l];
        if n.iter().any(|v| !v.is_finite()) {
            return Err(TieRodError::Range { name: "direction" });
        }
        Ok((n, l))
    }

    /// C (1 × 12) in binary64: [−n, −(o_i × n), n, (o_j × n)].
    /// Representable: `new` formed it (`try_c`).
    pub fn c(&self) -> [f64; ELEMENT_DOF] {
        self.try_c()
            .expect("representable: formed by the constructor")
    }

    /// C, or `Range` when an offset arm (o × n)_k leaves binary64.
    fn try_c(&self) -> Result<[f64; ELEMENT_DOF], TieRodError> {
        let (n, _) = self.direction_and_length()?;
        let arm_i = offset_cross(self.o_i, n)?;
        let arm_j = offset_cross(self.o_j, n)?;
        let mut c = [0.0; ELEMENT_DOF];
        for k in 0..3 {
            c[k] = -n[k];
            c[3 + k] = -arm_i[k];
            c[6 + k] = n[k];
            c[9 + k] = arm_j[k];
        }
        Ok(c)
    }

    /// Ke = Cᵀ k C (12 × 12): each entry C_a k C_b is one exact sum rounded
    /// once, so Ke is exactly symmetric. A non-finite entry, or a product
    /// too small for its exact split, is refused.
    pub fn global_stiffness(&self) -> Result<Matrix12, FrameKernelError> {
        let c = self.c();
        let mut ke = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
        for a in 0..ELEMENT_DOF {
            for b in a..ELEMENT_DOF {
                let mut sum = ExactAccumulator::new();
                triple(&mut sum, c[a], self.stiffness, c[b])?;
                let value = sum.round().map_err(|_| range("tie rod Ke"))?;
                ke[a][b] = value;
                ke[b][a] = value;
            }
        }
        Ok(ke)
    }

    /// K2b (N-5): the formed Ke times 2^b, exactly, as the assembly adds it.
    pub fn force_scaled_global_stiffness(
        &self,
        scale: ForceScale,
    ) -> Result<Matrix12, FrameKernelError> {
        force_scaled_matrix(
            "tie rod Ke*2^b (force scale)",
            &self.global_stiffness()?,
            scale,
        )
    }

    /// K2b: this rod with k times 2^b, exactly (K-D5's scaled source). Its
    /// `global_stiffness` is then 2^b·Ke bit for bit wherever both are
    /// normal. Geometry and e_ref are unchanged.
    pub fn force_scaled(&self, scale: ForceScale) -> Result<Self, FrameKernelError> {
        let mut rod = *self;
        rod.stiffness = force_scaled_value("tie rod k*2^b (force scale)", rod.stiffness, scale)?;
        Ok(rod)
    }

    /// SA's formation allowance of Ke (symmetry provenance): one rounding of
    /// an exact product, |fl(s) − s| ≤ u·|C_a| k |C_b| + 2^-1074, rounded
    /// upward (0 where the entry is an exact zero), with operation count 1.
    pub fn formation_roundoff(&self) -> Result<(Matrix12, [[usize; 12]; 12]), FrameKernelError> {
        let c = self.c();
        let tiny = f64::from_bits(1);
        let mut bounds = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
        for a in 0..ELEMENT_DOF {
            for b in a..ELEMENT_DOF {
                let total = product_upward(product_upward(c[a].abs(), self.stiffness), c[b].abs());
                if total == 0.0 {
                    continue;
                }
                let bound = (product_upward(gamma(1), total) + tiny).next_up();
                if !bound.is_finite() {
                    return Err(range("tie rod allowance"));
                }
                bounds[a][b] = bound;
                bounds[b][a] = bound;
            }
        }
        Ok((bounds, [[1; ELEMENT_DOF]; ELEMENT_DOF]))
    }

    /// +Cᵀ k e_ref, each entry C_a k e_ref one exact sum rounded once, with
    /// the bound from K-D5's own enclosure of the authored operands. `None`
    /// exactly when e_ref = 0 (the source-known zero: "one ε").
    pub fn reference_load(&self) -> Result<Option<TieRodReferenceLoad>, FrameKernelError> {
        if self.reference_extension == 0.0 {
            return Ok(None);
        }
        let c = self.c();
        let mut values = [0.0; ELEMENT_DOF];
        for (a, value) in values.iter_mut().enumerate() {
            let mut sum = ExactAccumulator::new();
            triple(&mut sum, c[a], self.stiffness, self.reference_extension)?;
            *value = sum.round().map_err(|_| range("tie rod reference load"))?;
        }
        let bounds = crate::structural::tie_rod_reference_load_bounds(self, &values)?;
        Ok(Some(TieRodReferenceLoad { values, bounds }))
    }

    /// This rod's 12 entries of a global displacement vector.
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

    /// e = C d and e − e_ref, each one exact sum rounded once; T = k(e −
    /// e_ref) and CᵀT, each one exact product rounded once.
    pub fn recover(&self, d: &[f64; ELEMENT_DOF]) -> Result<TieRodRecovery, FrameKernelError> {
        if d.iter().any(|v| !v.is_finite()) {
            return Err(FrameKernelError::NonFiniteInput {
                name: "tie rod displacements",
                value: f64::NAN,
            });
        }
        let c = self.c();
        let rounded = |sum: &ExactAccumulator| sum.round().map_err(|_| range("tie rod recovery"));
        let mut sum = ExactAccumulator::new();
        for l in 0..ELEMENT_DOF {
            sum.add_product(c[l], d[l]).map_err(sum_error)?;
        }
        let extension = rounded(&sum)?;
        sum.add(-self.reference_extension).map_err(sum_error)?;
        let elongation = rounded(&sum)?;
        let mut product = ExactAccumulator::new();
        product
            .add_product(self.stiffness, elongation)
            .map_err(sum_error)?;
        let tension = rounded(&product)?;
        let mut nodal_action = [0.0; ELEMENT_DOF];
        for (a, action) in nodal_action.iter_mut().enumerate() {
            let mut sum = ExactAccumulator::new();
            sum.add_product(c[a], tension).map_err(sum_error)?;
            *action = rounded(&sum)?;
        }
        Ok(TieRodRecovery {
            extension,
            elongation,
            tension,
            nodal_action,
        })
    }
}

/// The global DOFs of a rod between `node_i` and `node_j`.
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
    range("tie rod exact sum")
}

fn finite3(name: &'static str, v: [f64; 3]) -> Result<(), TieRodError> {
    if v.iter().all(|x| x.is_finite()) {
        Ok(())
    } else {
        Err(TieRodError::NonFinite { name })
    }
}

/// Adds a·b·c exactly: a·(b·c) with b·c split error-free into p + e.
fn triple(sum: &mut ExactAccumulator, a: f64, b: f64, c: f64) -> Result<(), FrameKernelError> {
    if a == 0.0 || b == 0.0 || c == 0.0 {
        return Ok(());
    }
    let p = b * c;
    if !p.is_finite() || p.abs() < SPLIT_EXACT_MIN {
        return Err(range("tie rod product split"));
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

/// (o × n)_k = n·(e_k × o), one exact dot rounded once. e_k × o is exact
/// (signed components of o). A finite offset can still give an arm beyond
/// binary64 (|o| near the overflow threshold): `Range`, never a panic.
fn offset_cross(o: [f64; 3], n: [f64; 3]) -> Result<[f64; 3], TieRodError> {
    let unit_cross = [[0.0, -o[2], o[1]], [o[2], 0.0, -o[0]], [-o[1], o[0], 0.0]];
    let mut arm = [0.0; 3];
    for (slot, row) in arm.iter_mut().zip(unit_cross) {
        *slot = exact_dot3(n, row)
            .ok()
            .filter(|v| v.is_finite())
            .ok_or(TieRodError::Range {
                name: "offset arm (o x n)",
            })?;
    }
    Ok(arm)
}

/// The degenerate-length guard, decided exactly: with
/// T↑ ≥ 2^-48·(‖x_j − x_i‖∞ + ‖o_i‖∞ + ‖o_j‖∞) (|x_j − x_i| rounded upward,
/// the sum exact, the scaling rounded once upward), the rod is degenerate
/// when Σ d_k² − T↑² ≤ 0 (one exact sum of exact products, so l itself is
/// never rounded). Since T↑ is directed upward the decision never admits a
/// rod at or below the guard.
fn degenerate(rod: &TieRod, d: [f64; 3]) -> Result<bool, TieRodError> {
    let range = |_| TieRodError::Range {
        name: "length guard",
    };
    let (xi, xj) = (rod.node_i.coordinates, rod.node_j.coordinates);
    let mut dx = 0.0_f64;
    for k in 0..3 {
        let mut difference = ExactAccumulator::new();
        difference.add(xj[k]).map_err(range)?;
        difference.add(-xi[k]).map_err(range)?;
        if difference.signum() < 0 {
            difference = ExactAccumulator::new();
            difference.add(xi[k]).map_err(range)?;
            difference.add(-xj[k]).map_err(range)?;
        }
        dx = dx.max(round_upward(&difference).map_err(range)?);
    }
    let norm = |o: [f64; 3]| o[0].abs().max(o[1].abs()).max(o[2].abs());
    let mut scale = ExactAccumulator::new();
    for value in [dx, norm(rod.o_i), norm(rod.o_j)] {
        scale.add(value).map_err(range)?;
    }
    // T↑ = 2^-48·S rounded once upward from the exact S (S itself may exceed
    // binary64 when an offset is near the overflow threshold).
    let nearest = scale.round_scaled(-48).map_err(range)?;
    let mut excess = scale;
    excess
        .add_product(-nearest, 1.0 / LENGTH_GUARD)
        .map_err(range)?;
    let threshold = if excess.signum() > 0 {
        nearest.next_up()
    } else {
        nearest
    };
    let mut margin = ExactAccumulator::new();
    for v in d {
        margin.add_product(v, v).map_err(range)?;
    }
    margin.add_product(-threshold, threshold).map_err(range)?;
    Ok(margin.signum() <= 0)
}

/// o = fl(Q·offset): each component one exact dot, rounded once.
fn rotate(axes: &Matrix3, offset: [f64; 3]) -> Result<[f64; 3], TieRodError> {
    let mut o = [0.0; 3];
    for (row, slot) in o.iter_mut().enumerate() {
        *slot = exact_dot3(axes[row], offset).map_err(|_| TieRodError::Range {
            name: "global offset",
        })?;
        if !slot.is_finite() {
            return Err(TieRodError::Range {
                name: "global offset",
            });
        }
    }
    Ok(o)
}

/// d = (x_j − x_i) + (o_j − o_i) in binary64, componentwise.
fn difference(
    xi: [f64; 3],
    xj: [f64; 3],
    oi: [f64; 3],
    oj: [f64; 3],
) -> Result<[f64; 3], TieRodError> {
    let mut d = [0.0; 3];
    for k in 0..3 {
        d[k] = (xj[k] - xi[k]) + (oj[k] - oi[k]);
        if !d[k].is_finite() {
            return Err(TieRodError::Range {
                name: "represented difference",
            });
        }
    }
    Ok(d)
}
