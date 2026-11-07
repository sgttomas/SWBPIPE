//! Re-formation of K and M from their leaves, the formation difference, and
//! the residual check on the re-formed pencil (design §3.4 checks 1 to 3).
//!
//! **Operands** ([`ElementOperand`]) are neutral: authored geometry,
//! sections, moduli, stiffness scales, flexibility factors, springs,
//! component weights and the frame kernel's own connector and tie-rod
//! types. No binary64 matrix used by the solve is an input, and no
//! product type appears; the product maps its model into these in I2.
//!
//! **Stiffness.** Straights, realized bends, connectors and tie rods are
//! re-formed by the frame kernel's p128 seam
//! (`structural::element_reformation`, seam (i)), with A, I and J
//! re-derived here in double-double from OD and wall ([`section`]) and the
//! rigid-family moduli s·E and s·G formed as exact double-double products.
//! Springs are re-formed here: an axis spring adds k to its diagonal; a
//! rank-one spring adds k·n̂n̂ᵀ, n̂ = n/|n| in double-double, on a node's
//! translations or rotations. Each p128 entry is taken as the
//! double-double sum of its exact binary64 split.
//!
//! **Mass** ([`mass`]): consistent straights with the bending-plane sign map
//! and Rayleigh rotary inertia, half-arc lumps with section rotary inertia,
//! and component bodies m = W/a after the binary64 range premise. A member
//! whose mass a component body replaces carries stiffness only
//! (`carries_mass: false`). Connectors, tie rods and springs carry no mass.
//!
//! **Checks.** [`formation_difference`] accumulates every re-formed element
//! contribution in double-double into one accumulator per stored entry of
//! the formed K_ff and M_ff (16 B per stored entry), so assembly rounding is
//! included, and reports ‖DΔKD‖_∞ and ‖DΔMD‖_∞ with D = diag(|k_ii|^-½) of
//! the formed K_ff, the re-formed norms ‖K̃̂‖_∞ and ‖M̃̂‖_∞, and the largest
//! section-derivation difference. A re-formed contribution at a position the
//! formed pattern does not store is compared with zero and counted.
//! [`residual_estimate`] evaluates r̂ = K̂φ − λM̂φ element by element and
//! gives η, κ_λ and e_λ = η·κ_λ (check 2); [`cluster_error_estimates`] the
//! cluster-level e_λ; `orthogonality::m_orthogonality_reformed` check 3 on
//! M̂. These are numerical-integrity checks with stated premises, not
//! bounds (§3.4); the thresholds they are judged against belong to the
//! caller (DEC-026).

pub mod mass;
pub mod section;

use std::error::Error;
use std::fmt;

use open_pipe_stress_frame_kernel::connector::ObjectiveConnector;
use open_pipe_stress_frame_kernel::structural::element_reformation::{
    reform_bend, reform_connector, reform_straight, reform_tie_rod, BendOperands, ReformedElement,
    StraightOperands, WideOperand,
};
use open_pipe_stress_frame_kernel::structural::retained_api::WideError;
use open_pipe_stress_frame_kernel::tie_rod::TieRod;
use open_pipe_stress_sparse_direct::SymmetricMatrixEntry;

use crate::admission::{check_entry, PencilMatrix, PencilRangeUnsupported};
use crate::dd::{max_propagating_nan, Dd, WideDd};
use crate::exact::{ExactSum, LimbAllotmentExceeded, Term};
use crate::support::stable_counting_order;
use crate::workspace::{reset_rows, ProductWorkspace, Retained};

// Optional test-only observation of the actual T7 call/read multiplicities:
// assemblies, constructor range reads, matching reads, re-formed norm reads,
// and formed-difference reads. No instrumentation enters a product build.
#[cfg(test)]
std::thread_local! {
    static T7_PASSES: std::cell::Cell<Option<[usize; 5]>> = const { std::cell::Cell::new(None) };
}
#[cfg(test)]
fn t7_pass(kind: usize) {
    T7_PASSES.with(|cell| {
        if let Some(mut counts) = cell.get() {
            counts[kind] += 1;
            cell.set(Some(counts));
        }
    });
}
use mass::{
    arc_lumps, component_mass, straight_mass, ComponentBody, ComponentRangeOperand, GeometryError,
};
use section::{
    derive_section, section_derivation_difference, FormedSection, MassBasis, PipeSection,
    SectionDerivationDifference, SectionError,
};

/// Moduli and the rigid-family stiffness scale s (E and G become s·E and
/// s·G, formed exactly in double-double).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Moduli {
    pub elastic_modulus: f64,
    pub shear_modulus: f64,
    pub stiffness_scale: Option<f64>,
}

/// A straight Euler–Bernoulli member between global nodes `node_i` and
/// `node_j`. `formed_section` is the A, I and J the formation used, for the
/// section-derivation difference.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StraightOperand {
    pub node_i: usize,
    pub node_j: usize,
    pub coordinates_i: [f64; 3],
    pub coordinates_j: [f64; 3],
    pub y_reference: [f64; 3],
    pub section: PipeSection,
    pub moduli: Moduli,
    pub carries_mass: bool,
    pub formed_section: Option<FormedSection>,
}

/// A realized bend (DEC-070): nodes, radius R and the bow-side y reference.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArcOperand {
    pub node_i: usize,
    pub node_j: usize,
    pub coordinates_i: [f64; 3],
    pub coordinates_j: [f64; 3],
    pub radius: f64,
    pub y_reference: [f64; 3],
    pub section: PipeSection,
    pub moduli: Moduli,
    pub in_plane_flexibility_factor: f64,
    pub out_of_plane_flexibility_factor: f64,
    pub carries_mass: bool,
    pub formed_section: Option<FormedSection>,
}

/// A linear spring to ground.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpringOperand {
    /// k on one global DOF.
    Axis { dof: usize, stiffness: f64 },
    /// k·n̂n̂ᵀ on a node's translations (or rotations), n̂ = n/|n|. n is squared in
    /// ordinary-exponent double-double, so a caller passes a canonically scaled n
    /// (R-6/R-7: `SkewDirection::scaled`, max|n_i| ∈ [1, 2)); the block is invariant
    /// under that exact power-of-two scaling.
    RankOne {
        node: usize,
        rotational: bool,
        direction: [f64; 3],
        stiffness: f64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementOperand {
    Straight(StraightOperand),
    Arc(ArcOperand),
    ComponentBody(ComponentBody),
    Spring(SpringOperand),
    Connector(ObjectiveConnector),
    TieRod(TieRod),
}

/// Why an element could not be re-formed: the set is refused
/// (`MODAL_REFORMATION_UNAVAILABLE`, naming the element).
#[derive(Debug, Clone, PartialEq)]
pub enum ReformationFailure {
    Section(SectionError),
    Geometry(GeometryError),
    Stiffness(WideError),
    /// p128 entries with set bits below 2⁻¹⁰⁷⁴.
    StiffnessUnderflow,
    ComponentMassOutOfRange(ComponentRangeOperand),
    /// A spring or modulus input is not finite, or a scale or stiffness is
    /// not positive, or a direction is zero.
    InvalidOperand(&'static str),
    /// A DOF beyond the DOF map.
    DofOutOfRange {
        dof: usize,
    },
    /// A DOF map entry beyond the dimension.
    FreeIndexOutOfRange {
        dof: usize,
        index: usize,
    },
    /// An assembled re-formed entry, nonzero, outside the admission range
    /// (`MODAL_PENCIL_RANGE_UNSUPPORTED`); `element` is the first element
    /// contributing to it. Row and column are free indices.
    PencilRangeUnsupported(PencilRangeUnsupported),
    /// An assembled entry's exact sum would grow past the caller's limb
    /// allotment ([`ReformedPencil::new_within_limb_allotment`]); refused
    /// before that growth. `element` is the contributing element.
    LimbAllotmentExceeded(LimbAllotmentExceeded),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReformationError {
    /// The element's position in the operand list.
    pub element: usize,
    pub failure: ReformationFailure,
}

impl fmt::Display for ReformationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "element {} cannot be re-formed: {:?}",
            self.element, self.failure
        )
    }
}

impl Error for ReformationError {}

/// A re-formed element matrix on global DOFs, symmetric, its lower triangle
/// (by local index) packed row by row.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementMatrix {
    pub dofs: Vec<usize>,
    lower: Vec<Dd>,
}

impl ElementMatrix {
    fn from_square<const N: usize>(dofs: Vec<usize>, m: &[[Dd; N]; N]) -> Self {
        let mut lower = Vec::with_capacity(N * (N + 1) / 2);
        for (i, row) in m.iter().enumerate() {
            lower.extend_from_slice(&row[..=i]);
        }
        Self { dofs, lower }
    }

    pub fn size(&self) -> usize {
        self.dofs.len()
    }

    /// Entry (i, j), read from the lower triangle.
    pub fn get(&self, i: usize, j: usize) -> Dd {
        let (r, c) = if i >= j { (i, j) } else { (j, i) };
        self.lower[r * (r + 1) / 2 + c]
    }
}

fn node_dofs(node_i: usize, node_j: usize) -> Vec<usize> {
    (0..6)
        .map(|k| 6 * node_i + k)
        .chain((0..6).map(|k| 6 * node_j + k))
        .collect()
}

fn wide(x: Dd) -> WideOperand {
    WideOperand { hi: x.hi, lo: x.lo }
}

fn from_reformed(
    dofs: Vec<usize>,
    k: &ReformedElement,
) -> Result<ElementMatrix, ReformationFailure> {
    if k.truncated_below_min_subnormal() {
        return Err(ReformationFailure::StiffnessUnderflow);
    }
    let mut m = [[Dd::ZERO; 12]; 12];
    for (i, row) in m.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            *v = Dd::from_terms(&k.terms(i, j));
        }
    }
    Ok(ElementMatrix::from_square(dofs, &m))
}

/// (s·E, s·G) in double-double.
fn moduli(m: &Moduli) -> Result<(Dd, Dd), ReformationFailure> {
    let positive = |v: f64| v.is_finite() && v > 0.0;
    if !positive(m.elastic_modulus) || !positive(m.shear_modulus) {
        return Err(ReformationFailure::InvalidOperand("modulus"));
    }
    match m.stiffness_scale {
        None => Ok((
            Dd::from_f64(m.elastic_modulus),
            Dd::from_f64(m.shear_modulus),
        )),
        Some(s) if positive(s) => Ok((
            Dd::product(s, m.elastic_modulus),
            Dd::product(s, m.shear_modulus),
        )),
        Some(_) => Err(ReformationFailure::InvalidOperand("stiffness_scale")),
    }
}

/// The element's re-formed stiffness, if it has one.
pub fn reform_element_stiffness(
    element: &ElementOperand,
    basis: MassBasis,
) -> Result<Option<ElementMatrix>, ReformationFailure> {
    match element {
        ElementOperand::Straight(s) => {
            let q = derive_section(&s.section, basis).map_err(ReformationFailure::Section)?;
            let (e, g) = moduli(&s.moduli)?;
            let k = reform_straight(&StraightOperands {
                coordinates_i: s.coordinates_i,
                coordinates_j: s.coordinates_j,
                y_reference: s.y_reference,
                elastic_modulus: wide(e),
                shear_modulus: wide(g),
                area: wide(q.area),
                second_moment_y: wide(q.second_moment),
                second_moment_z: wide(q.second_moment),
                torsion_constant: wide(q.torsion_constant),
                shear_area: None,
            })
            .map_err(ReformationFailure::Stiffness)?;
            from_reformed(node_dofs(s.node_i, s.node_j), &k).map(Some)
        }
        ElementOperand::Arc(a) => {
            let q = derive_section(&a.section, basis).map_err(ReformationFailure::Section)?;
            let (e, g) = moduli(&a.moduli)?;
            let k = reform_bend(&BendOperands {
                coordinates_i: a.coordinates_i,
                coordinates_j: a.coordinates_j,
                radius: a.radius,
                y_reference: a.y_reference,
                elastic_modulus: wide(e),
                shear_modulus: wide(g),
                area: wide(q.area),
                second_moment: wide(q.second_moment),
                torsion_constant: wide(q.torsion_constant),
                in_plane_flexibility_factor: WideOperand::from_f64(a.in_plane_flexibility_factor),
                out_of_plane_flexibility_factor: WideOperand::from_f64(
                    a.out_of_plane_flexibility_factor,
                ),
            })
            .map_err(ReformationFailure::Stiffness)?;
            from_reformed(node_dofs(a.node_i, a.node_j), &k).map(Some)
        }
        ElementOperand::Connector(c) => {
            let k = reform_connector(c).map_err(ReformationFailure::Stiffness)?;
            from_reformed(node_dofs(c.node_i().index, c.node_j().index), &k).map(Some)
        }
        ElementOperand::TieRod(r) => {
            let k = reform_tie_rod(r).map_err(ReformationFailure::Stiffness)?;
            from_reformed(node_dofs(r.node_i().index, r.node_j().index), &k).map(Some)
        }
        ElementOperand::Spring(SpringOperand::Axis { dof, stiffness }) => {
            if !(stiffness.is_finite() && *stiffness > 0.0) {
                return Err(ReformationFailure::InvalidOperand("spring stiffness"));
            }
            Ok(Some(ElementMatrix::from_square(
                vec![*dof],
                &[[Dd::from_f64(*stiffness)]],
            )))
        }
        ElementOperand::Spring(SpringOperand::RankOne {
            node,
            rotational,
            direction,
            stiffness,
        }) => {
            if !(stiffness.is_finite() && *stiffness > 0.0) {
                return Err(ReformationFailure::InvalidOperand("spring stiffness"));
            }
            if direction.iter().any(|x| !x.is_finite()) || direction.iter().all(|&x| x == 0.0) {
                return Err(ReformationFailure::InvalidOperand("spring direction"));
            }
            let n = direction.map(Dd::from_f64);
            let nn = n[0].mul(n[0]).add(n[1].mul(n[1])).add(n[2].mul(n[2]));
            let k = Dd::from_f64(*stiffness).div(nn);
            let mut m = [[Dd::ZERO; 3]; 3];
            for (a, row) in m.iter_mut().enumerate() {
                for (b, v) in row.iter_mut().enumerate() {
                    *v = k.mul(n[a]).mul(n[b]);
                }
            }
            let base = 6 * node + if *rotational { 3 } else { 0 };
            Ok(Some(ElementMatrix::from_square(
                (base..base + 3).collect(),
                &m,
            )))
        }
        ElementOperand::ComponentBody(_) => Ok(None),
    }
}

/// The element's re-formed mass contributions (none, one, or an arc's two
/// lumps).
pub fn reform_element_mass(
    element: &ElementOperand,
    basis: MassBasis,
) -> Result<Vec<ElementMatrix>, ReformationFailure> {
    match element {
        ElementOperand::Straight(s) if s.carries_mass => {
            let q = derive_section(&s.section, basis).map_err(ReformationFailure::Section)?;
            let m = straight_mass(
                s.coordinates_i,
                s.coordinates_j,
                s.y_reference,
                q.mass_per_length,
                q.bending_rotary_inertia_per_length,
                q.polar_rotary_inertia_per_length,
            )
            .map_err(ReformationFailure::Geometry)?;
            Ok(vec![ElementMatrix::from_square(
                node_dofs(s.node_i, s.node_j),
                &m,
            )])
        }
        ElementOperand::Arc(a) if a.carries_mass => {
            let q = derive_section(&a.section, basis).map_err(ReformationFailure::Section)?;
            let [li, lj] = arc_lumps(
                a.coordinates_i,
                a.coordinates_j,
                a.radius,
                a.y_reference,
                q.mass_per_length,
                q.bending_rotary_inertia_per_length,
                q.polar_rotary_inertia_per_length,
            )
            .map_err(ReformationFailure::Geometry)?;
            Ok(vec![
                ElementMatrix::from_square((0..6).map(|k| 6 * a.node_i + k).collect(), &li),
                ElementMatrix::from_square((0..6).map(|k| 6 * a.node_j + k).collect(), &lj),
            ])
        }
        ElementOperand::ComponentBody(b) => {
            let m = component_mass(b).map_err(ReformationFailure::ComponentMassOutOfRange)?;
            Ok(vec![ElementMatrix::from_square(
                (0..6).map(|k| 6 * b.node + k).collect(),
                &m,
            )])
        }
        _ => Ok(Vec::new()),
    }
}

/// R-6/R-7 (DEL-04-03 skew-restraints.md §11.3 D-1 (a)): one global DOF's
/// reduced coordinates under a restraint basis: the free coordinate indices c
/// with q_g,c ≠ 0 and their weights (at most 3; restrained coordinates are
/// omitted). Q is node-local and division-free; the same `congruence_terms`
/// enumeration forms the product's K′ and M′.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Coordinates {
    entries: [(usize, f64); 3],
    len: usize,
}

impl Coordinates {
    /// `None` for more than three entries or a non-finite or zero weight.
    pub fn new(entries: &[(usize, f64)]) -> Option<Self> {
        if entries.len() > 3 || entries.iter().any(|&(_, q)| !q.is_finite() || q == 0.0) {
            return None;
        }
        let mut out = Self::default();
        out.entries[..entries.len()].copy_from_slice(entries);
        out.len = entries.len();
        Some(out)
    }
    pub fn as_slice(&self) -> &[(usize, f64)] {
        &self.entries[..self.len]
    }
}

/// An element matrix with its DOFs mapped to the free numbering (`None`
/// for a restrained DOF). `mixed`, only for an element touching a DOF whose
/// coordinates are mixed by a restraint basis, maps every element DOF to its
/// weighted coordinates instead (`rows` then unused); identity elements carry
/// none, so the identity path allocates nothing new.
#[derive(Debug, Clone, PartialEq)]
struct Mapped {
    element: usize,
    rows: Vec<Option<usize>>,
    matrix: ElementMatrix,
    mixed: Option<Vec<Coordinates>>,
}

/// The exact product a·b as two scaled parts ((value, exponent) each, the
/// second possibly zero): significands in [1, 2), so the TwoProduct is exact
/// and never underflows.
fn two_prod_scaled(a: f64, b: f64) -> ((f64, i32), (f64, i32)) {
    let (ea, eb) = (crate::dd::binary_exponent(a), crate::dd::binary_exponent(b));
    let (hi, lo) = crate::dd::two_prod(crate::dd::scale_by_pow2(a, -ea), crate::dd::scale_by_pow2(b, -eb));
    ((hi, ea + eb), (lo, ea + eb))
}

/// One assembly position's contribution (see [`for_each_position`]).
#[derive(Clone, Copy)]
enum Contribution {
    /// e_ab: its two words, each added as a scaled value.
    Plain(Dd),
    /// q_a·e_ab·q_b: one exact product per word of e_ab.
    Mixed { qa: f64, entry: Dd, qb: f64 },
}

/// Visits every assembly position of `list` (lower triangle, r ≥ c), in
/// the fixed order of the former map's insertions: element by element; for
/// a mixed element every ordered element pair (a, b) and every coordinate
/// pair with r ≥ c (D-1 (a)); otherwise each pair b ≤ a of mapped rows,
/// keyed (max, min). `visit` gets the element, the key and the
/// contribution; its first error stops the traversal.
///
/// Count t, in support steps (each loop body at most 24 operations
/// besides `visit`): one per element and per row a; plain, one per pair
/// b ≤ a (a restrained row's skip included), so 1 + d + d(d + 1)/2 ≤ 3d²;
/// mixed, one per pair (a, b), per coordinate of a and per coordinate pair
/// (at most three coordinates per DOF), so 1 + d + d² + 3d² + 9d² ≤ 15d².
/// So t ≤ θ·D_list, D_list = Σ d², θ = 3 with no mixed element and 15
/// otherwise. Visits v: at most d(d + 1)/2 ≤ d² per plain element and 9d²
/// per mixed one.
fn for_each_position<E>(
    list: &[Mapped],
    mut visit: impl FnMut(usize, (usize, usize), Contribution) -> Result<(), E>,
) -> Result<(), E> {
    for e in list {
        if let Some(mixed) = &e.mixed {
            for (a, ca) in mixed.iter().enumerate() {
                for (b, cb) in mixed.iter().enumerate() {
                    let entry = e.matrix.get(a, b);
                    for &(ra, qa) in ca.as_slice() {
                        for &(rb, qb) in cb.as_slice() {
                            if ra < rb {
                                continue;
                            }
                            visit(e.element, (ra, rb), Contribution::Mixed { qa, entry, qb })?;
                        }
                    }
                }
            }
            continue;
        }
        for (a, ra) in e.rows.iter().enumerate() {
            let Some(ra) = *ra else { continue };
            for (b, rb) in e.rows.iter().enumerate().take(a + 1) {
                let Some(rb) = *rb else { continue };
                let key = if ra >= rb { (ra, rb) } else { (rb, ra) };
                visit(e.element, key, Contribution::Plain(e.matrix.get(a, b)))?;
            }
        }
    }
    Ok(())
}

impl Mapped {
    fn touches(&self, row: usize, col: usize) -> bool {
        match &self.mixed {
            None => self.rows.contains(&Some(row)) && self.rows.contains(&Some(col)),
            Some(mixed) => {
                let has = |r: usize| mixed.iter().any(|c| c.as_slice().iter().any(|&(i, _)| i == r));
                has(row) && has(col)
            }
        }
    }
}

/// The re-formed pencil K̂, M̂ on the free DOFs, kept element by element.
///
/// It owns the workspace of the residual and orthogonality stages
/// (`workspace`): the accumulators and scratch vectors reused for every pair
/// and every product, retained until the pencil is dropped. The workspace
/// is not part of the pencil's value (equality, `Debug` and a clone ignore
/// it).
#[derive(Clone, PartialEq)]
pub struct ReformedPencil {
    dimension: usize,
    stiffness: Vec<Mapped>,
    mass: Vec<Mapped>,
    section_difference: Option<SectionDerivationDifference>,
    workspace: Retained<ProductWorkspace>,
}

impl fmt::Debug for ReformedPencil {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReformedPencil")
            .field("dimension", &self.dimension)
            .field("stiffness", &self.stiffness)
            .field("mass", &self.mass)
            .field("section_difference", &self.section_difference)
            .finish()
    }
}

impl ReformedPencil {
    /// Re-forms every element. `dof_map[g]` is the free index of global DOF
    /// g, or `None` when it is restrained.
    pub fn new(
        dimension: usize,
        dof_map: &[Option<usize>],
        basis: MassBasis,
        elements: &[ElementOperand],
    ) -> Result<Self, ReformationError> {
        Self::new_within_limb_allotment(dimension, dof_map, basis, elements, None)
    }

    /// [`ReformedPencil::new`] with an optional limb allotment on the
    /// assembled entries' exact sums (the formation maps, design §5.3.8):
    /// an entry whose sum would need more limbs refuses
    /// (`ReformationFailure::LimbAllotmentExceeded`) before that growth is
    /// allocated. `None` is `new`.
    pub fn new_within_limb_allotment(
        dimension: usize,
        dof_map: &[Option<usize>],
        basis: MassBasis,
        elements: &[ElementOperand],
        limb_allotment: Option<usize>,
    ) -> Result<Self, ReformationError> {
        Self::new_on_coordinates(dimension, dof_map, None, basis, elements, limb_allotment)
    }

    /// R-6/R-7 (D-1 (a)): the re-formed reduced pencil K̂′ = Q̃ᵀK̂Q̃, M̂′ = Q̃ᵀM̂Q̃.
    /// `coordinates[g]` is `Some` exactly for a global DOF whose coordinates a
    /// restraint basis mixes (its free coordinates and weights); every other
    /// DOF maps through `dof_map` as in [`ReformedPencil::new`]. Each term
    /// q_ir·k·q_jc (·x) enters the exact sums as one exact product.
    pub fn new_with_coordinates(
        dimension: usize,
        dof_map: &[Option<usize>],
        coordinates: &[Option<Coordinates>],
        basis: MassBasis,
        elements: &[ElementOperand],
    ) -> Result<Self, ReformationError> {
        Self::new_on_coordinates(dimension, dof_map, Some(coordinates), basis, elements, None)
    }

    /// [`ReformedPencil::new_with_coordinates`] under an optional limb allotment
    /// (design §5.3.8), as [`ReformedPencil::new_within_limb_allotment`] (D-1 N1).
    pub fn new_with_coordinates_within_limb_allotment(
        dimension: usize,
        dof_map: &[Option<usize>],
        coordinates: &[Option<Coordinates>],
        basis: MassBasis,
        elements: &[ElementOperand],
        limb_allotment: Option<usize>,
    ) -> Result<Self, ReformationError> {
        Self::new_on_coordinates(dimension, dof_map, Some(coordinates), basis, elements, limb_allotment)
    }

    fn new_on_coordinates(
        dimension: usize,
        dof_map: &[Option<usize>],
        coordinates: Option<&[Option<Coordinates>]>,
        basis: MassBasis,
        elements: &[ElementOperand],
        limb_allotment: Option<usize>,
    ) -> Result<Self, ReformationError> {
        let map = |element: usize, matrix: ElementMatrix| -> Result<Mapped, ReformationError> {
            let mut rows = Vec::with_capacity(matrix.size());
            for &dof in &matrix.dofs {
                let fail = |failure| ReformationError { element, failure };
                let entry = *dof_map
                    .get(dof)
                    .ok_or(fail(ReformationFailure::DofOutOfRange { dof }))?;
                if let Some(index) = entry {
                    if index >= dimension {
                        return Err(fail(ReformationFailure::FreeIndexOutOfRange { dof, index }));
                    }
                }
                rows.push(entry);
            }
            let mixed = match coordinates {
                Some(coordinates) if matrix.dofs.iter().any(|&dof| coordinates.get(dof).is_some_and(Option::is_some)) => {
                    let mut mixed = Vec::with_capacity(matrix.size());
                    for (&dof, entry) in matrix.dofs.iter().zip(&rows) {
                        let fail = |failure| ReformationError { element, failure };
                        let weighted = match coordinates.get(dof).copied().flatten() {
                            Some(weighted) => weighted,
                            None => entry.map_or(Coordinates::default(), |index| Coordinates {
                                entries: [(index, 1.0), (0, 0.0), (0, 0.0)],
                                len: 1,
                            }),
                        };
                        if let Some(&(index, _)) = weighted.as_slice().iter().find(|&&(index, _)| index >= dimension) {
                            return Err(fail(ReformationFailure::FreeIndexOutOfRange { dof, index }));
                        }
                        mixed.push(weighted);
                    }
                    Some(mixed)
                }
                _ => None,
            };
            Ok(Mapped {
                element,
                rows,
                matrix,
                mixed,
            })
        };
        let mut pencil = Self {
            dimension,
            stiffness: Vec::new(),
            mass: Vec::new(),
            section_difference: None,
            workspace: Retained::default(),
        };
        for (index, element) in elements.iter().enumerate() {
            let fail = |failure| ReformationError {
                element: index,
                failure,
            };
            if let Some(k) = reform_element_stiffness(element, basis).map_err(fail)? {
                pencil.stiffness.push(map(index, k)?);
            }
            for m in reform_element_mass(element, basis).map_err(fail)? {
                pencil.mass.push(map(index, m)?);
            }
            let (section, formed) = match element {
                ElementOperand::Straight(s) => (s.section, s.formed_section),
                ElementOperand::Arc(a) => (a.section, a.formed_section),
                _ => continue,
            };
            if let Some(formed) = formed {
                let q = derive_section(&section, basis)
                    .map_err(|e| fail(ReformationFailure::Section(e)))?;
                let d = section_derivation_difference(&q, &formed);
                pencil.section_difference =
                    Some(pencil.section_difference.map_or(d, |p| p.combine(d)));
            }
        }
        // One scratch vector for every entry read; each map is read in key
        // order and dropped before the next is built.
        let mut scratch = Vec::new();
        for (matrix, list) in [
            (PencilMatrix::Stiffness, &pencil.stiffness),
            (PencilMatrix::Mass, &pencil.mass),
        ] {
            let assembled = Self::assembled_exact(list, dimension, limb_allotment).map_err(
                |(element, exceeded)| ReformationError {
                    element,
                    failure: ReformationFailure::LimbAllotmentExceeded(exceeded),
                },
            )?;
            // 5 support steps a position: `to_dd` (at most five scaling
            // iterations each way at the level-1 span) and `check_entry`.
            for &((row, col), ref sum) in &assembled {
                #[cfg(test)]
                t7_pass(1);
                let value = sum.to_wide_with(&mut scratch).to_dd();
                if let Err(range) = check_entry(matrix, row, col, value.to_f64()) {
                    let element = list
                        .iter()
                        .find(|e| e.touches(row, col))
                        .map_or(0, |e| e.element);
                    return Err(ReformationError {
                        element,
                        failure: ReformationFailure::PencilRangeUnsupported(range),
                    });
                }
            }
        }
        Ok(pencil)
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// The largest section-derivation difference over the elements that
    /// supplied their formed section, componentwise.
    pub fn section_derivation_difference(&self) -> Option<SectionDerivationDifference> {
        self.section_difference
    }

    /// M̂x into `rows` (each row's exact sum, element by element), in the
    /// order of [`ReformedPencil::mass_product_exact`].
    pub(crate) fn mass_product_into(&self, x: &[f64], rows: &mut [ExactSum]) {
        assert_eq!(
            x.len(),
            self.dimension,
            "vector length differs from the pencil"
        );
        Self::product_into(&self.mass, x, None, rows);
    }

    /// The pencil's stage workspace (`crate::workspace`).
    pub(crate) fn workspace(&self) -> &Retained<ProductWorkspace> {
        &self.workspace
    }

    /// Σ_b factor·e_ab·x_b into each row's exact sum, element by element.
    fn product_into(list: &[Mapped], x: &[f64], factor: Option<f64>, rows: &mut [ExactSum]) {
        for e in list {
            if let Some(mixed) = &e.mixed {
                // (Q̃ᵀ E Q̃ x)_r: every ordered element pair (a, b) and every
                // coordinate pair, q_a,r·e_ab·q_b,c·x_c (·factor) exactly.
                for (a, ca) in mixed.iter().enumerate() {
                    for (b, cb) in mixed.iter().enumerate() {
                        let entry = e.matrix.get(a, b);
                        for &(ra, qa) in ca.as_slice() {
                            for &(rb, qb) in cb.as_slice() {
                                if x[rb] == 0.0 {
                                    continue;
                                }
                                for word in [entry.hi, entry.lo] {
                                    if word == 0.0 {
                                        continue;
                                    }
                                    // With a factor (five terms), the two weights are first
                                    // folded into one exact scaled pair.
                                    let _ = match factor {
                                        None => rows[ra].add_product4_within(&[qa, word, qb, x[rb]], 0, None),
                                        Some(f) => {
                                            let (hi, lo) = two_prod_scaled(qa, qb);
                                            let _ = rows[ra].add_product4_within(&[hi.0, word, f, x[rb]], hi.1, None);
                                            if lo.0 != 0.0 {
                                                let _ = rows[ra].add_product4_within(&[lo.0, word, f, x[rb]], lo.1, None);
                                            }
                                            Ok(())
                                        }
                                    };
                                }
                            }
                        }
                    }
                }
                continue;
            }
            for (a, ra) in e.rows.iter().enumerate() {
                let Some(ra) = *ra else { continue };
                for (b, rb) in e.rows.iter().enumerate() {
                    let Some(rb) = *rb else { continue };
                    if x[rb] == 0.0 {
                        continue;
                    }
                    match factor {
                        None => rows[ra].add_dd_product(e.matrix.get(a, b), &[x[rb]]),
                        Some(f) => rows[ra].add_dd_product(e.matrix.get(a, b), &[f, x[rb]]),
                    }
                }
            }
        }
    }

    fn exact_product(&self, list: &[Mapped], x: &[f64]) -> Vec<ExactSum> {
        assert_eq!(
            x.len(),
            self.dimension,
            "vector length differs from the pencil"
        );
        let mut rows = vec![ExactSum::new(); self.dimension];
        Self::product_into(list, x, None, &mut rows);
        rows
    }

    /// K̂x, each row summed exactly over the elements' double-double
    /// entries times x, then rounded once.
    pub fn stiffness_product(&self, x: &[f64]) -> Vec<WideDd> {
        self.exact_product(&self.stiffness, x)
            .iter()
            .map(ExactSum::to_wide)
            .collect()
    }

    /// M̂x, each row summed exactly, then rounded once.
    pub fn mass_product(&self, x: &[f64]) -> Vec<WideDd> {
        self.exact_product(&self.mass, x)
            .iter()
            .map(ExactSum::to_wide)
            .collect()
    }

    /// M̂x, each row as its exact sum.
    pub fn mass_product_exact(&self, x: &[f64]) -> Vec<ExactSum> {
        self.exact_product(&self.mass, x)
    }

    /// The assembled entries, each the exact sum of the element
    /// contributions, ascending by (row, col) with each position once. Under
    /// a limb allotment, an entry whose sum would grow past it refuses
    /// before that growth, naming the contributing element (the
    /// formation-map guard, design §5.3.8).
    ///
    /// **Former map.** This was a `BTreeMap` filled by `entry(key)
    /// .or_default()` at each position visit, in [`for_each_position`]'s
    /// order. Here the visits' keys are put in order by a stable counting
    /// sort (column, then row: `support::stable_counting_order`), each visit
    /// is given its key's slot, and the visits are replayed in the same
    /// order, adding the same parts to the same slot. Each sum therefore
    /// receives the same adds in the same order, the entries come out in the
    /// map's key order, and the first refusing add, in visit order, is the
    /// one the map refused, with the same sum state and so the same
    /// `LimbAllotmentExceeded`. The keys are below `dimension`: `rows` and
    /// `mixed` hold only free indices checked against it.
    ///
    /// **Count** with v position visits and t traversal steps
    /// ([`for_each_position`]: t ≤ θ·D_list, θ = 3 plain or 15 mixed), in
    /// support steps (`support`'s unit; an `ExactSum` call is one
    /// operation, its limb work `T7.limbs`): three traversals 3t and their
    /// visit bodies 3v (each at most 30 operations); two counting sorts
    /// 2(3v + 2n + 1); `slot_of`'s fill, the slots and the output's pushes
    /// 3v; and 1 for the call. So at most `3t + 12v + 4n + 3` with n =
    /// `dimension`.
    ///
    /// **Capacity.** The keys (16v B), the two orders (8v B each), the
    /// counting starts (8n B, one per sort), `slot_of` (8v B) and the
    /// result, exactly u ≤ v entries of a key and an `ExactSum` (each
    /// allocated at its exact length; the keys and orders are freed before
    /// the adds, `slot_of` on return, also on a refusal). Peak 40v + 8n B
    /// plus the result and its sums' limbs, where the map held u entries
    /// in its nodes. Nothing grows: every push is within a capacity counted
    /// before it, and allocation is std's infallible kind, as before.
    fn assembled_exact(
        list: &[Mapped],
        dimension: usize,
        limb_allotment: Option<usize>,
    ) -> Result<Vec<((usize, usize), ExactSum)>, (usize, LimbAllotmentExceeded)> {
        #[cfg(test)]
        t7_pass(0);
        let unreachable = |never: std::convert::Infallible| match never {};
        let mut visits = 0usize;
        for_each_position(list, |_, _, _| {
            visits += 1;
            Ok(())
        })
        .unwrap_or_else(unreachable);
        let mut keys: Vec<(usize, usize)> = Vec::with_capacity(visits);
        for_each_position(list, |_, key, _| {
            keys.push(key);
            Ok(())
        })
        .unwrap_or_else(unreachable);
        let by_col = stable_counting_order(0..visits, visits, dimension, |i| keys[i].1);
        let by_key = stable_counting_order(by_col.iter().copied(), visits, dimension, |i| keys[i].0);
        drop(by_col);
        // Slots in key order: a new slot at each change of key.
        let mut slot_of = vec![0usize; visits];
        let mut distinct = 0usize;
        let mut previous: Option<(usize, usize)> = None;
        for &visit in &by_key {
            if previous != Some(keys[visit]) {
                distinct += 1;
                previous = Some(keys[visit]);
            }
            slot_of[visit] = distinct - 1;
        }
        let mut out: Vec<((usize, usize), ExactSum)> = Vec::with_capacity(distinct);
        previous = None;
        for &visit in &by_key {
            if previous != Some(keys[visit]) {
                out.push((keys[visit], ExactSum::new()));
                previous = Some(keys[visit]);
            }
        }
        drop(by_key);
        drop(keys);
        let mut visit = 0usize;
        for_each_position(list, |element, _, contribution| -> Result<(), (usize, LimbAllotmentExceeded)> {
            let slot = &mut out[slot_of[visit]].1;
            visit += 1;
            let refused = |exceeded| (element, exceeded);
            match contribution {
                Contribution::Plain(entry) => {
                    slot.add_scaled_within(entry.hi, 0, limb_allotment)
                        .map_err(refused)?;
                    slot.add_scaled_within(entry.lo, 0, limb_allotment)
                        .map_err(refused)?;
                }
                Contribution::Mixed { qa, entry, qb } => {
                    for word in [entry.hi, entry.lo] {
                        slot.add_product4_within(&[qa, word, qb], 0, limb_allotment)
                            .map_err(refused)?;
                    }
                }
            }
            Ok(())
        })?;
        Ok(out)
    }

    /// The assembled entries rounded once to double-double, ascending.
    fn assembled(&self, list: &[Mapped]) -> Vec<((usize, usize), Dd)> {
        let mut scratch = Vec::new();
        Self::assembled_exact(list, self.dimension, None)
            .expect("no allotment")
            .into_iter()
            .map(|(key, sum)| (key, sum.to_wide_with(&mut scratch).to_dd()))
            .collect()
    }

    /// The assembled K̂ on the free DOFs, lower triangle, ascending.
    pub fn assembled_stiffness(&self) -> Vec<((usize, usize), Dd)> {
        self.assembled(&self.stiffness)
    }

    /// The assembled M̂ on the free DOFs, lower triangle, ascending.
    pub fn assembled_mass(&self) -> Vec<((usize, usize), Dd)> {
        self.assembled(&self.mass)
    }

    /// The position visits of the assembly of K̂ and of M̂ (v in
    /// [`ReformedPencil::assembled_exact`]'s count), for resource evidence.
    pub fn assembly_visits(&self) -> [usize; 2] {
        [&self.stiffness, &self.mass].map(|list| {
            let mut visits = 0usize;
            for_each_position(list, |_, _, _| {
                visits += 1;
                Ok::<(), std::convert::Infallible>(())
            })
            .unwrap_or_else(|never| match never {});
            visits
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormationError {
    EntryOutOfBounds {
        row: usize,
        col: usize,
    },
    NonFiniteEntry {
        row: usize,
        col: usize,
    },
    DuplicateEntry {
        row: usize,
        col: usize,
    },
    /// A formed stiffness diagonal that is zero or absent.
    ZeroStiffnessDiagonal {
        index: usize,
    },
    /// A nonzero formed entry outside the admission range
    /// (`MODAL_PENCIL_RANGE_UNSUPPORTED`).
    PencilRangeUnsupported(PencilRangeUnsupported),
    /// A computed quantity (`quantity`) that is not finite, or not normal
    /// or exactly zero.
    ResultOutOfRange {
        quantity: &'static str,
    },
    /// An exact sum of the formation difference (an assembled re-formed
    /// entry or a formed entry's accumulator) would grow past the caller's
    /// limb allotment ([`formation_difference_within_limb_allotment`]);
    /// refused before that growth.
    LimbAllotmentExceeded(LimbAllotmentExceeded),
}

impl fmt::Display for FormationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "formed matrix input refused: {self:?}")
    }
}

impl Error for FormationError {}

/// D = diag(|k_ii|^-½) of the formed K_ff and the re-formed scaled norms,
/// the inputs of check 2.
#[derive(Debug, Clone, PartialEq)]
pub struct PencilScaling {
    pub scaling: Vec<Dd>,
    /// ‖DK̂D‖_∞.
    pub reformed_stiffness_norm: f64,
    /// ‖DM̂D‖_∞.
    pub reformed_mass_norm: f64,
}

/// One matrix's formation difference.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatrixDifference {
    /// ‖D(formed − re-formed)D‖_∞.
    pub norm: f64,
    /// The row attaining it (the first on a tie).
    pub row: usize,
    /// Re-formed nonzero positions the formed pattern does not store.
    pub off_pattern: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FormationDifference {
    pub stiffness: MatrixDifference,
    pub mass: MatrixDifference,
    pub section: Option<SectionDerivationDifference>,
    pub scaling: PencilScaling,
}

/// Lower-triangle formed entries, ascending, checked.
///
/// The order is a stable counting sort (column, then row) of the checked
/// entries, where this was `sort_unstable_by_key`: with no equal keys both
/// give the one ascending order, and with equal keys both refuse the least
/// duplicated key, so the order among equal keys is never observed.
///
/// Count with F entries, in support steps (each body at most 40
/// operations): the checks F, two counting sorts 2(3F + 2n + 1), the
/// gather F, the duplicate scan F and 1, so `9F + 4n + 3`. Capacity: the
/// checked list and the sorted list, F entries (24 B) each, and the two
/// orders and the starts (8F, 8F and 8n B, freed on return).
fn formed_lower(
    n: usize,
    matrix: PencilMatrix,
    entries: &[SymmetricMatrixEntry],
) -> Result<Vec<((usize, usize), f64)>, FormationError> {
    let mut out = Vec::with_capacity(entries.len());
    for e in entries {
        if e.row >= n || e.col >= n {
            return Err(FormationError::EntryOutOfBounds {
                row: e.row,
                col: e.col,
            });
        }
        if !e.value.is_finite() {
            return Err(FormationError::NonFiniteEntry {
                row: e.row,
                col: e.col,
            });
        }
        let key = if e.row >= e.col {
            (e.row, e.col)
        } else {
            (e.col, e.row)
        };
        check_entry(matrix, key.0, key.1, e.value).map_err(FormationError::PencilRangeUnsupported)?;
        out.push((key, e.value));
    }
    let by_col = stable_counting_order(0..out.len(), out.len(), n, |i| out[i].0 .1);
    let by_key = stable_counting_order(by_col.iter().copied(), out.len(), n, |i| out[i].0 .0);
    drop(by_col);
    let mut sorted = Vec::with_capacity(out.len());
    for &i in &by_key {
        sorted.push(out[i]);
    }
    drop(by_key);
    drop(out);
    let out = sorted;
    for w in out.windows(2) {
        if w[0].0 == w[1].0 {
            return Err(FormationError::DuplicateEntry {
                row: w[0].0 .0,
                col: w[0].0 .1,
            });
        }
    }
    Ok(out)
}

/// The scaled row sums Σ_j |d_i x_ij d_j| over the full symmetric matrix of
/// a lower-triangle list, in `WideDd`, and the row of the largest. A row sum
/// that is not finite, or not normal or zero once rounded, refuses
/// (`quantity` names it); no NaN is passed over by the maximum.
fn scaled_norm(
    n: usize,
    d: &[Dd],
    entries: impl Iterator<Item = ((usize, usize), WideDd)>,
    quantity: &'static str,
) -> Result<(f64, usize), FormationError> {
    let mut rows = vec![WideDd::ZERO; n];
    for ((r, c), v) in entries {
        let s = WideDd::from_dd(d[r])
            .mul(v)
            .mul(WideDd::from_dd(d[c]))
            .abs();
        rows[r] = rows[r].add(s);
        if r != c {
            rows[c] = rows[c].add(s);
        }
    }
    let mut best = (0.0, 0);
    for (i, s) in rows.iter().enumerate() {
        let s = s
            .to_normal_or_zero()
            .ok_or(FormationError::ResultOutOfRange { quantity })?;
        if s > best.0 {
            best = (s, i);
        }
    }
    Ok(best)
}

/// One matrix's formation difference: `formed` and `reformed` are each
/// ascending by (row, col), each position once.
///
/// Each re-formed position is matched to its formed entry by one merge
/// walk (a cursor that only advances), where this binary-searched the
/// formed list; both find the one formed entry with the same key, or none.
/// The off-pattern values were a `BTreeMap` filled in the re-formed (so
/// ascending) key order with distinct keys; here they are pushed in that
/// same order, so they are read in the same order and counted the same.
///
/// Count with F formed entries and u re-formed ones, in support steps (an
/// `ExactSum` call one operation): the accumulators F, the absent count's
/// walk F + u, the matching walk F + u, the term adds within it (one step
/// a term, at most L̄ terms a read, so u·L̄), and `scaled_norm` at 24 steps
/// an entry (about 1,253 operations of WideDd arithmetic: two `from_dd`,
/// two `mul`, `abs` and two `add`) over the F + (≤ u) deltas and the u
/// re-formed entries, and per row its fill and 6 steps
/// (`to_normal_or_zero`, at most six scaling iterations: about 265
/// operations), twice; and 3. So at most `27F + 50u + u·L̄ + 14n + 3`.
/// Capacity: F accumulators, the off-pattern list at exactly the number of
/// re-formed positions absent from `formed` (≤ u; a key and a `WideDd`
/// each), the reads' scratch and term list, and two `scaled_norm` row
/// vectors of n.
fn difference(
    n: usize,
    d: &[Dd],
    formed: &[((usize, usize), f64)],
    reformed: &[((usize, usize), ExactSum)],
    names: [&'static str; 2],
    limb_allotment: Option<usize>,
) -> Result<(MatrixDifference, f64), FormationError> {
    let exceeded = FormationError::LimbAllotmentExceeded;
    // One exact accumulator per stored formed entry, in the formed order:
    // the formed value minus every element contribution, rounded once.
    let mut accumulators: Vec<ExactSum> = Vec::with_capacity(formed.len());
    for &(_, f) in formed {
        let mut sum = ExactSum::new();
        sum.add_scaled_within(f, 0, limb_allotment)
            .map_err(exceeded)?;
        accumulators.push(sum);
    }
    // One scratch vector and one term list, reused for every entry read.
    let mut scratch = Vec::new();
    let mut terms: Vec<Term> = Vec::new();
    // The re-formed positions absent from the formed list bound the
    // off-pattern list's capacity.
    let mut absent = 0usize;
    let mut cursor = 0usize;
    for (key, _) in reformed {
        while cursor < formed.len() && formed[cursor].0 < *key {
            cursor += 1;
        }
        if !(cursor < formed.len() && formed[cursor].0 == *key) {
            absent += 1;
        }
    }
    let mut off_pattern: Vec<((usize, usize), WideDd)> = Vec::with_capacity(absent);
    cursor = 0;
    for (key, sum) in reformed {
        #[cfg(test)]
        t7_pass(2);
        while cursor < formed.len() && formed[cursor].0 < *key {
            cursor += 1;
        }
        let found = if cursor < formed.len() && formed[cursor].0 == *key { Ok(cursor) } else { Err(()) };
        match found {
            Ok(at) => {
                sum.terms_into(&mut scratch, &mut terms);
                for &(value, exponent) in &terms {
                    accumulators[at]
                        .add_scaled_within(-value, exponent, limb_allotment)
                        .map_err(exceeded)?;
                }
            }
            Err(_) => {
                let value = sum.to_wide_with(&mut scratch);
                if !value.is_zero() {
                    off_pattern.push((*key, value.neg()));
                }
            }
        }
    }
    let deltas = formed
        .iter()
        .zip(&accumulators)
        .map(|(&(key, _), sum)| {
            #[cfg(test)]
            t7_pass(4);
            (key, sum.to_wide_with(&mut scratch))
        })
        .chain(off_pattern.iter().copied());
    let (norm, row) = scaled_norm(n, d, deltas, names[0])?;
    let (reformed_norm, _) = scaled_norm(
        n,
        d,
        reformed
            .iter()
            .map(|(k, sum)| {
                #[cfg(test)]
                t7_pass(3);
                (*k, sum.to_wide_with(&mut scratch))
            }),
        names[1],
    )?;
    Ok((
        MatrixDifference {
            norm,
            row,
            off_pattern: off_pattern.len(),
        },
        reformed_norm,
    ))
}

/// Check 1's formation difference on the assembled matrices (module docs).
/// `formed_stiffness` and `formed_mass` are the stored entries of the formed
/// K_ff and M_ff, one triangle each, on the pencil's free numbering.
pub fn formation_difference(
    pencil: &ReformedPencil,
    formed_stiffness: &[SymmetricMatrixEntry],
    formed_mass: &[SymmetricMatrixEntry],
) -> Result<FormationDifference, FormationError> {
    formation_difference_within_limb_allotment(pencil, formed_stiffness, formed_mass, None)
}

/// [`formation_difference`] with an optional limb allotment on its exact
/// sums (the formation maps and the formed entries' accumulators, design
/// §5.3.8): a sum that would need more limbs refuses
/// (`FormationError::LimbAllotmentExceeded`) before that growth is
/// allocated. `None` is `formation_difference`.
pub fn formation_difference_within_limb_allotment(
    pencil: &ReformedPencil,
    formed_stiffness: &[SymmetricMatrixEntry],
    formed_mass: &[SymmetricMatrixEntry],
    limb_allotment: Option<usize>,
) -> Result<FormationDifference, FormationError> {
    let n = pencil.dimension;
    let assembled = |list: &[Mapped]| {
        ReformedPencil::assembled_exact(list, n, limb_allotment)
            .map_err(|(_, exceeded)| FormationError::LimbAllotmentExceeded(exceeded))
    };
    let k = formed_lower(n, PencilMatrix::Stiffness, formed_stiffness)?;
    let m = formed_lower(n, PencilMatrix::Mass, formed_mass)?;
    // Each diagonal by a merge walk over the ascending K list (n + F steps),
    // where this binary-searched it: the one entry keyed (i, i), or none.
    // Its scaling is 16 support steps an index (`Dd::div`, `Dd::sqrt`).
    let mut scaling = vec![Dd::ZERO; n];
    let mut cursor = 0usize;
    for (index, d) in scaling.iter_mut().enumerate() {
        while cursor < k.len() && k[cursor].0 < (index, index) {
            cursor += 1;
        }
        let diagonal = (cursor < k.len() && k[cursor].0 == (index, index))
            .then(|| k[cursor].1)
            .filter(|&v| v != 0.0)
            .ok_or(FormationError::ZeroStiffnessDiagonal { index })?;
        *d = Dd::from_f64(1.0).div(Dd::from_f64(diagonal.abs()).sqrt());
    }
    let (stiffness, reformed_stiffness_norm) = difference(
        n,
        &scaling,
        &k,
        &assembled(&pencil.stiffness)?,
        ["stiffness formation difference", "re-formed stiffness norm"],
        limb_allotment,
    )?;
    let (mass, reformed_mass_norm) = difference(
        n,
        &scaling,
        &m,
        &assembled(&pencil.mass)?,
        ["mass formation difference", "re-formed mass norm"],
        limb_allotment,
    )?;
    Ok(FormationDifference {
        stiffness,
        mass,
        section: pencil.section_difference,
        scaling: PencilScaling {
            scaling,
            reformed_stiffness_norm,
            reformed_mass_norm,
        },
    })
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResidualError {
    DimensionMismatch {
        expected: usize,
        actual: usize,
    },
    NonFinite {
        index: usize,
    },
    /// λ is not finite and positive.
    InvalidEigenvalue,
    /// φᵀM̂φ is not positive.
    NonPositiveGeneralizedMass,
    /// A returned quantity (`quantity`) that is not finite, or not normal or
    /// exactly zero, or a scaling input that is not.
    ResultOutOfRange {
        quantity: &'static str,
    },
    /// An error estimate given to `cluster_error_estimates` that is not
    /// finite and non-negative.
    InvalidEstimate {
        index: usize,
    },
}

impl fmt::Display for ResidualError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "residual input refused: {self:?}")
    }
}

impl Error for ResidualError {}

/// Check 2 for one pair (λ, φ).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResidualEstimate {
    /// ‖D r̂‖₂.
    pub scaled_residual: f64,
    /// η = ‖D r̂‖₂ / ((‖K̃̂‖ + λ‖M̃̂‖)·‖φ̃‖₂).
    pub backward_error: f64,
    /// κ_λ = (‖K̃̂‖ + λ‖M̃̂‖)·‖φ̃‖₂² / (λ·φᵀM̂φ).
    pub condition: f64,
    /// e_λ = η·κ_λ, an estimate under first-order perturbation theory with
    /// λ isolated, not a bound.
    pub error_estimate: f64,
    /// φᵀM̂φ.
    pub generalized_mass: f64,
    /// ‖D r̂‖₂ and φᵀM̂φ are published (they scale with φ): each nonzero but
    /// below the binary64 normal range is published as 0 with its flag
    /// (design §3.2, admission range). η, κ_λ and e_λ are compared (with
    /// τ_r and in the Sturm separation) and must be normal or exactly zero.
    pub scaled_residual_below_normal_range: bool,
    pub generalized_mass_below_normal_range: bool,
}

/// r̂ = K̂φ − λM̂φ element by element (each re-formed entry times φ_j and
/// the sums in double-double), then η, κ_λ and e_λ (design
/// §3.4 check 2), with the scaling of [`formation_difference`].
pub fn residual_estimate(
    pencil: &ReformedPencil,
    scaling: &PencilScaling,
    eigenvalue: f64,
    vector: &[f64],
) -> Result<ResidualEstimate, ResidualError> {
    let n = pencil.dimension;
    if vector.len() != n || scaling.scaling.len() != n {
        return Err(ResidualError::DimensionMismatch {
            expected: n,
            actual: vector.len(),
        });
    }
    if let Some(index) = vector.iter().position(|x| !x.is_finite()) {
        return Err(ResidualError::NonFinite { index });
    }
    if !(eigenvalue.is_finite() && eigenvalue > 0.0) {
        return Err(ResidualError::InvalidEigenvalue);
    }
    // r̂_i = Σ K̂_ib φ_b − λ Σ M̂_ib φ_b summed exactly per row, so the
    // cancellation of an accurate pair exposes no rounding, and φᵀM̂φ summed
    // exactly from M̂φ's exact terms. The squares and norms (sums of
    // nonnegative terms) are in WideDd: φ may be scaled arbitrarily, and η,
    // κ_λ and e_λ do not depend on its scale.
    //
    // The row sums, the generalized-mass sum and the scratch vectors are the
    // pencil's workspace, reset here and reused for every pair (module
    // `workspace`); the operations and their order are those of fresh ones.
    let (residual, tilde, generalized) = pencil.workspace.with(|w| {
        let ProductWorkspace {
            rows,
            mass_rows,
            sum: generalized_sum,
            limbs,
            terms,
        } = w;
        let rows = reset_rows(rows, n);
        ReformedPencil::product_into(&pencil.stiffness, vector, None, rows);
        ReformedPencil::product_into(&pencil.mass, vector, Some(-eigenvalue), rows);
        let mx = reset_rows(mass_rows, n);
        pencil.mass_product_into(vector, mx);
        let mut residual = WideDd::ZERO;
        let mut tilde = WideDd::ZERO;
        generalized_sum.reset();
        for i in 0..n {
            let d = WideDd::from_dd(scaling.scaling[i]);
            let r = rows[i].to_wide_with(limbs).mul(d);
            residual = residual.add(r.mul(r));
            let t = WideDd::from_f64(vector[i]).div(d);
            tilde = tilde.add(t.mul(t));
            mx[i].terms_into(limbs, terms);
            generalized_sum.add_terms_product(terms, &[vector[i]]);
        }
        (residual, tilde, generalized_sum.to_wide_with(limbs))
    });
    let lambda = WideDd::from_f64(eigenvalue);
    let out_of_range = |quantity| ResidualError::ResultOutOfRange { quantity };
    if !generalized.is_finite() {
        return Err(out_of_range("generalized_mass"));
    }
    if !generalized.is_positive() {
        return Err(ResidualError::NonPositiveGeneralizedMass);
    }
    let (k_norm, m_norm) = (
        scaling.reformed_stiffness_norm,
        scaling.reformed_mass_norm,
    );
    if !(k_norm.is_finite() && k_norm > 0.0 && m_norm.is_finite() && m_norm >= 0.0) {
        return Err(out_of_range("scaling norms"));
    }
    let scale = WideDd::from_f64(k_norm).add(lambda.mul_f64(m_norm));
    let residual = residual.sqrt();
    let backward = residual.div(scale.mul(tilde.sqrt()));
    let condition = scale.mul(tilde).div(generalized.mul(lambda));
    let compared = |quantity, value: WideDd| value.to_normal_or_zero().ok_or(out_of_range(quantity));
    let published = |quantity, value: WideDd| value.to_published().ok_or(out_of_range(quantity));
    let (scaled_residual, scaled_residual_below_normal_range) =
        published("scaled_residual", residual)?;
    let (generalized_mass, generalized_mass_below_normal_range) =
        published("generalized_mass", generalized)?;
    Ok(ResidualEstimate {
        scaled_residual,
        backward_error: compared("backward_error", backward)?,
        condition: compared("condition", condition)?,
        error_estimate: compared("error_estimate", backward.mul(condition))?,
        generalized_mass,
        scaled_residual_below_normal_range,
        generalized_mass_below_normal_range,
    })
}

/// The cluster-level e_λ: for each cluster id, the largest e_λ over its
/// members (`estimates[i]` belongs to `cluster_ids[i]`, ids from
/// `clusters::cluster_ids`). An estimate that is not finite and
/// non-negative refuses (`InvalidEstimate`), so the maximum never passes
/// over a NaN.
pub fn cluster_error_estimates(
    estimates: &[f64],
    cluster_ids: &[usize],
) -> Result<Vec<f64>, ResidualError> {
    assert_eq!(
        estimates.len(),
        cluster_ids.len(),
        "one cluster id per estimate"
    );
    if let Some(index) = estimates
        .iter()
        .position(|e| !(e.is_finite() && *e >= 0.0))
    {
        return Err(ResidualError::InvalidEstimate { index });
    }
    let count = cluster_ids.iter().max().map_or(0, |m| m + 1);
    let mut out = vec![0.0f64; count];
    for (&e, &id) in estimates.iter().zip(cluster_ids) {
        out[id] = max_propagating_nan(out[id], e);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Review round 3, item B: a NaN row sum is refused, not passed over by
    /// the maximum.
    #[test]
    fn scaled_norm_refuses_a_nan_row() {
        let d = [Dd::from_f64(1.0), Dd::from_f64(1.0)];
        let entries = [
            ((0, 0), WideDd::from_f64(2.0)),
            ((1, 1), WideDd::from_f64(f64::NAN)),
        ];
        assert_eq!(
            scaled_norm(2, &d, entries.into_iter(), "test"),
            Err(FormationError::ResultOutOfRange { quantity: "test" })
        );
        let finite = [
            ((0, 0), WideDd::from_f64(2.0)),
            ((1, 0), WideDd::from_f64(-3.0)),
        ];
        assert_eq!(scaled_norm(2, &d, finite.into_iter(), "test"), Ok((5.0, 0)));
    }

    /// One node, six free DOFs: axis springs and a rank-one spring for K, a
    /// component body with an offset centre of gravity for M (entries with
    /// nonzero low words, so the exact sums grow below).
    fn small_pencil(limb_allotment: Option<usize>) -> Result<ReformedPencil, ReformationError> {
        let mut elements: Vec<ElementOperand> = (0..6)
            .map(|dof| {
                ElementOperand::Spring(SpringOperand::Axis {
                    dof,
                    stiffness: 1e3 * (dof + 1) as f64 / 7.0,
                })
            })
            .collect();
        elements.push(ElementOperand::Spring(SpringOperand::RankOne {
            node: 0,
            rotational: false,
            direction: [1.0, 2.0, 3.0],
            stiffness: 5e2,
        }));
        elements.push(ElementOperand::ComponentBody(ComponentBody {
            node: 0,
            weight: 1000.0,
            acceleration: 9.80665,
            center_of_gravity: [0.3, 0.7, 1.1],
        }));
        let map: Vec<Option<usize>> = (0..6).map(Some).collect();
        ReformedPencil::new_within_limb_allotment(
            6,
            &map,
            MassBasis::EffectiveWall,
            &elements,
            limb_allotment,
        )
    }

    #[test]
    fn t7_complete_path_replays_both_matrices_twice_and_keeps_every_read() {
        T7_PASSES.with(|cell| assert_eq!(cell.replace(Some([0; 5])), None));
        let pencil = small_pencil(None).unwrap();
        let first = T7_PASSES.with(|cell| cell.replace(None).unwrap());
        // Obtain the independent input lists outside the observed pipeline;
        // doing this is additional work, not hidden in the T7 observations.
        let k = pencil.assembled_stiffness();
        let m = pencil.assembled_mass();
        let positions = k.len() + m.len();
        assert_eq!(first, [2, positions, 0, 0, 0]);
        let formed = |entries: Vec<((usize, usize), Dd)>| entries.into_iter()
            .map(|((row, col), value)| SymmetricMatrixEntry { row, col, value: value.to_f64() })
            .collect::<Vec<_>>();
        let (k, m) = (formed(k), formed(m));
        T7_PASSES.with(|cell| cell.set(Some([0; 5])));
        formation_difference(&pencil, &k, &m).unwrap();
        let second = T7_PASSES.with(|cell| cell.replace(None).unwrap());
        assert_eq!(second, [2, 0, positions, positions, k.len() + m.len()]);
        // A first failed insertion skips the rest. The same full-path bounds
        // cover this refusal; no success-only multiplicity is assumed.
        T7_PASSES.with(|cell| cell.set(Some([0; 5])));
        assert!(small_pencil(Some(0)).is_err());
        let refused = T7_PASSES.with(|cell| cell.replace(None).unwrap());
        assert_eq!(refused, [1, 0, 0, 0, 0]);
    }

    /// R-6/R-7 D-1 (a): the coordinate re-formation. No mixed DOF is today's pencil
    /// bit for bit; an explicit identity weight takes the mixed path and gives the
    /// same assembled entries; a weight of 2 on DOF 0 scales its row and column
    /// exactly (K′₀₀ = 4K₀₀, K′₀c = 2K₀c), in K′ and in M′ alike (the modal review:
    /// the M′ half fails if the mass congruence drops its weight).
    #[test]
    fn coordinate_reformation_is_the_congruence() {
        let identity = small_pencil(None).unwrap();
        let elements = small_elements();
        let map: Vec<Option<usize>> = (0..6).map(Some).collect();
        let none = vec![None; 6];
        let unmixed = ReformedPencil::new_with_coordinates(6, &map, &none, MassBasis::EffectiveWall, &elements).unwrap();
        assert_eq!(unmixed, identity);
        let unit: Vec<Option<Coordinates>> = (0..6).map(|i| Coordinates::new(&[(i, 1.0)])).collect();
        let mixed = ReformedPencil::new_with_coordinates(6, &map, &unit, MassBasis::EffectiveWall, &elements).unwrap();
        assert_eq!(mixed.assembled_stiffness(), identity.assembled_stiffness());
        assert_eq!(mixed.assembled_mass(), identity.assembled_mass());
        let mut doubled = unit.clone();
        doubled[0] = Coordinates::new(&[(0, 2.0)]);
        let scaled = ReformedPencil::new_with_coordinates(6, &map, &doubled, MassBasis::EffectiveWall, &elements).unwrap();
        // The exact sums scale by 2 per weighted index; each is rounded once to
        // double-double, whose last bit is not scale-equivariant, so the
        // comparison is within that one rounding (2^-104 relative).
        let within = |a: Dd, b: Dd, what: &str| {
            assert!(a.sub(b).to_f64().abs() <= b.hi.abs() * 2f64.powi(-104), "{what}: {a:?} against {b:?}");
        };
        for ((key, value), (_, base)) in scaled.assembled_stiffness().into_iter().zip(identity.assembled_stiffness()) {
            let factor = [key.0, key.1].iter().filter(|&&i| i == 0).count() as i32;
            within(value, base.mul_pow2(factor), &format!("{key:?}"));
        }
        // The products agree with the assembled congruence: (Q̃ᵀK̂Q̃)x.
        let x = [1.0, -0.5, 0.25, 2.0, -1.0, 0.125];
        let product = scaled.stiffness_product(&x);
        let mut base_x = x;
        base_x[0] *= 2.0;
        let base = identity.stiffness_product(&base_x);
        within(product[1].to_dd(), base[1].to_dd(), "row 1");
        within(product[0].to_dd(), base[0].to_dd().mul_pow2(1), "row 0");
        // M′ under the same weight: the body's mass touches DOF 0's row and column (its
        // diagonal and its offset-centre-of-gravity coupling), so a dropped weight shows.
        let weighted: Vec<_> = identity.assembled_mass().into_iter().filter(|((r, c), v)| (*r == 0 || *c == 0) && v.hi != 0.0).collect();
        assert!(weighted.iter().any(|((r, c), _)| *r == 0 && *c == 0), "M has a DOF 0 diagonal");
        assert!(weighted.iter().any(|((r, c), _)| *r != *c), "M has a DOF 0 coupling");
        assert_eq!(scaled.assembled_mass().len(), identity.assembled_mass().len());
        for ((key, value), (base_key, base)) in scaled.assembled_mass().into_iter().zip(identity.assembled_mass()) {
            assert_eq!(key, base_key);
            let factor = [key.0, key.1].iter().filter(|&&i| i == 0).count() as i32;
            within(value, base.mul_pow2(factor), &format!("M {key:?}"));
            if factor > 0 && base.hi != 0.0 {
                assert!(value.sub(base).to_f64().abs() > base.hi.abs() * 0.5, "M {key:?}: the weight is applied");
            }
        }
        let mass_product = scaled.mass_product(&x);
        let mass_base = identity.mass_product(&base_x);
        within(mass_product[1].to_dd(), mass_base[1].to_dd(), "M row 1");
        within(mass_product[0].to_dd(), mass_base[0].to_dd().mul_pow2(1), "M row 0");
    }

    fn small_elements() -> Vec<ElementOperand> {
        let mut elements: Vec<ElementOperand> = (0..6)
            .map(|dof| ElementOperand::Spring(SpringOperand::Axis { dof, stiffness: 1e3 * (dof + 1) as f64 / 7.0 }))
            .collect();
        elements.push(ElementOperand::Spring(SpringOperand::RankOne { node: 0, rotational: false, direction: [1.0, 2.0, 3.0], stiffness: 5e2 }));
        elements.push(ElementOperand::ComponentBody(ComponentBody { node: 0, weight: 1000.0, acceleration: 9.80665, center_of_gravity: [0.3, 0.7, 1.1] }));
        elements
    }

    fn formed(list: Vec<((usize, usize), Dd)>) -> Vec<SymmetricMatrixEntry> {
        list.into_iter()
            .map(|((row, col), v)| SymmetricMatrixEntry {
                row,
                col,
                value: v.hi,
            })
            .collect()
    }

    /// The formation-map guard (design §5.3.8): under a limb allotment the
    /// re-formation and the formation difference refuse before the growth
    /// that would exceed it, naming the element; at or above the largest
    /// span they used, their results are those without one.
    #[test]
    fn formation_refuses_past_a_limb_allotment_before_growing() {
        let pencil = small_pencil(None).unwrap();
        let (k, m) = (
            formed(pencil.assembled_stiffness()),
            formed(pencil.assembled_mass()),
        );
        let unguarded = formation_difference(&pencil, &k, &m).unwrap();
        let mut smallest = None;
        for allotment in 0..64 {
            match small_pencil(Some(allotment)) {
                Ok(p) => {
                    assert_eq!(p, pencil);
                    smallest.get_or_insert(allotment);
                }
                Err(ReformationError {
                    element,
                    failure: ReformationFailure::LimbAllotmentExceeded(e),
                }) => {
                    assert!(smallest.is_none(), "{allotment}");
                    assert!(element < 8 && e.allotment == allotment && e.required > allotment);
                }
                Err(other) => panic!("{other:?}"),
            }
        }
        let smallest = smallest.expect("an allotment admits the pencil");
        assert!(smallest > 4, "{smallest}");
        let mut admitted = false;
        for allotment in 0..64 {
            match formation_difference_within_limb_allotment(&pencil, &k, &m, Some(allotment)) {
                Ok(d) => {
                    assert_eq!(d, unguarded);
                    admitted = true;
                }
                Err(FormationError::LimbAllotmentExceeded(e)) => {
                    assert!(!admitted && e.allotment == allotment && e.required > allotment);
                }
                Err(other) => panic!("{other:?}"),
            }
        }
        assert!(admitted);
    }

    /// Workspace reuse in the residual and orthogonality stages: after a
    /// wide-span pair (many limbs), an early refusal and the other stage,
    /// each call gives exactly what a fresh pencil (a clone, whose
    /// workspace is empty) gives, and the retained capacity is reused, not
    /// reallocated.
    #[test]
    fn a_reused_pencil_workspace_gives_the_fresh_result() {
        let pencil = small_pencil(None).unwrap();
        let (k, m) = (
            formed(pencil.assembled_stiffness()),
            formed(pencil.assembled_mass()),
        );
        let scaling = formation_difference(&pencil, &k, &m).unwrap().scaling;
        let narrow = [1.0, -0.5, 0.25, 0.125, -2.0, 0.75];
        let wide = [p(-400), 1.0, p(300), -p(-200), p(100), -3.0];
        let zero = [0.0; 6];
        let fresh =
            |lambda: f64, v: &[f64]| residual_estimate(&pencil.clone(), &scaling, lambda, v);
        let reused = |lambda: f64, v: &[f64]| residual_estimate(&pencil, &scaling, lambda, v);
        assert_eq!(reused(3.0, &wide), fresh(3.0, &wide));
        let storage = |p: &ReformedPencil| {
            p.workspace().with(|w| {
                w.rows
                    .iter()
                    .chain(&w.mass_rows)
                    .map(|r| r.limb_storage().1)
                    .collect::<Vec<_>>()
            })
        };
        let after_wide = storage(&pencil);
        assert!(after_wide.iter().any(|&c| c > 20), "{after_wide:?}");
        assert_eq!(
            reused(3.0, &zero),
            Err(ResidualError::NonPositiveGeneralizedMass)
        );
        for lambda in [3.0, 0.5, 1e6] {
            assert_eq!(reused(lambda, &narrow), fresh(lambda, &narrow));
        }
        let shapes: [&[f64]; 3] = [&narrow, &wide, &[0.0, 1.0, 0.0, 0.0, 0.0, 1.0]];
        let orthogonality = crate::orthogonality::m_orthogonality_reformed(&pencil, &shapes);
        assert_eq!(
            orthogonality,
            crate::orthogonality::m_orthogonality_reformed(&pencil.clone(), &shapes)
        );
        assert_eq!(reused(3.0, &wide), fresh(3.0, &wide));
        assert_eq!(reused(3.0, &narrow), fresh(3.0, &narrow));
        // The narrow pairs and the orthogonality stage needed no growth.
        assert_eq!(storage(&pencil), after_wide);
        // A clone, compared equal, starts with an empty workspace.
        let clone = pencil.clone();
        assert_eq!(clone, pencil);
        assert!(storage(&clone).is_empty());
    }

    fn p(e: i32) -> f64 {
        crate::dd::scale_by_pow2(1.0, e)
    }

    /// Review round 3, item B: a NaN estimate was dropped by f64::max.
    #[test]
    fn cluster_estimates_refuse_a_nan() {
        assert_eq!(
            cluster_error_estimates(&[1e-9, f64::NAN, 2e-9], &[0, 0, 1]),
            Err(ResidualError::InvalidEstimate { index: 1 })
        );
        assert_eq!(
            cluster_error_estimates(&[1e-9, 3e-9, 2e-9], &[0, 0, 1]),
            Ok(vec![3e-9, 2e-9])
        );
    }
}

/// Old against new for the assembly and the formation difference (design
/// §5.3.8, `T7.assembly`): the former `BTreeMap`, `sort_unstable_by_key` and
/// `binary_search_by_key` code, kept here as the test-only oracle, against
/// the repository-owned replacement, on random and adversarial lists. Every
/// compared sum is compared by its whole state (`ExactSum`'s equality is
/// over its limbs, `low`, pending adds and non-finite part) and its limb
/// storage; every float by its bits.
#[cfg(test)]
mod former_oracle_tests {
    use super::*;
    use std::collections::BTreeMap;

    struct Mix(u64);
    impl Mix {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }
        fn below(&mut self, n: usize) -> usize {
            (self.next() % n as u64) as usize
        }
        /// A nonzero binary64 of either sign over a wide exponent range.
        fn value(&mut self) -> f64 {
            let significand = 1.0 + (self.next() >> 11) as f64 / (1u64 << 53) as f64;
            let exponent = self.below(120) as i32 - 60;
            let sign = if self.below(2) == 0 { -1.0 } else { 1.0 };
            sign * crate::dd::scale_by_pow2(significand, exponent)
        }
        fn dd(&mut self) -> Dd {
            match self.below(6) {
                0 => Dd::ZERO,
                1 => Dd::from_f64(self.value()),
                _ => {
                    let hi = self.value();
                    let lo = crate::dd::scale_by_pow2(self.value().abs().min(1.0) * hi.abs(), -(54 + self.below(10) as i32));
                    Dd { hi, lo: if self.below(2) == 0 { lo } else { -lo } }
                }
            }
        }
    }

    /// The former `assembled_exact`, verbatim but for its signature.
    fn former_assembled_exact(
        list: &[Mapped],
        limb_allotment: Option<usize>,
    ) -> Result<BTreeMap<(usize, usize), ExactSum>, (usize, LimbAllotmentExceeded)> {
        let mut out: BTreeMap<(usize, usize), ExactSum> = BTreeMap::new();
        for e in list {
            if let Some(mixed) = &e.mixed {
                for (a, ca) in mixed.iter().enumerate() {
                    for (b, cb) in mixed.iter().enumerate() {
                        let entry = e.matrix.get(a, b);
                        for &(ra, qa) in ca.as_slice() {
                            for &(rb, qb) in cb.as_slice() {
                                if ra < rb {
                                    continue;
                                }
                                let slot = out.entry((ra, rb)).or_default();
                                for word in [entry.hi, entry.lo] {
                                    slot.add_product4_within(&[qa, word, qb], 0, limb_allotment)
                                        .map_err(|exceeded| (e.element, exceeded))?;
                                }
                            }
                        }
                    }
                }
                continue;
            }
            for (a, ra) in e.rows.iter().enumerate() {
                let Some(ra) = *ra else { continue };
                for (b, rb) in e.rows.iter().enumerate().take(a + 1) {
                    let Some(rb) = *rb else { continue };
                    let key = if ra >= rb { (ra, rb) } else { (rb, ra) };
                    let entry = e.matrix.get(a, b);
                    let slot = out.entry(key).or_default();
                    let refused = |exceeded| (e.element, exceeded);
                    slot.add_scaled_within(entry.hi, 0, limb_allotment).map_err(refused)?;
                    slot.add_scaled_within(entry.lo, 0, limb_allotment).map_err(refused)?;
                }
            }
        }
        Ok(out)
    }

    /// The former `formed_lower`, verbatim.
    fn former_formed_lower(
        n: usize,
        matrix: PencilMatrix,
        entries: &[SymmetricMatrixEntry],
    ) -> Result<Vec<((usize, usize), f64)>, FormationError> {
        let mut out = Vec::with_capacity(entries.len());
        for e in entries {
            if e.row >= n || e.col >= n {
                return Err(FormationError::EntryOutOfBounds { row: e.row, col: e.col });
            }
            if !e.value.is_finite() {
                return Err(FormationError::NonFiniteEntry { row: e.row, col: e.col });
            }
            let key = if e.row >= e.col { (e.row, e.col) } else { (e.col, e.row) };
            check_entry(matrix, key.0, key.1, e.value).map_err(FormationError::PencilRangeUnsupported)?;
            out.push((key, e.value));
        }
        out.sort_unstable_by_key(|x| x.0);
        for w in out.windows(2) {
            if w[0].0 == w[1].0 {
                return Err(FormationError::DuplicateEntry { row: w[0].0 .0, col: w[0].0 .1 });
            }
        }
        Ok(out)
    }

    /// The former `difference`, verbatim but for its signature.
    fn former_difference(
        n: usize,
        d: &[Dd],
        formed: &[((usize, usize), f64)],
        reformed: &BTreeMap<(usize, usize), ExactSum>,
        names: [&'static str; 2],
        limb_allotment: Option<usize>,
    ) -> Result<(MatrixDifference, f64), FormationError> {
        let exceeded = FormationError::LimbAllotmentExceeded;
        let mut accumulators: Vec<ExactSum> = Vec::with_capacity(formed.len());
        for &(_, f) in formed {
            let mut sum = ExactSum::new();
            sum.add_scaled_within(f, 0, limb_allotment).map_err(exceeded)?;
            accumulators.push(sum);
        }
        let mut scratch = Vec::new();
        let mut terms: Vec<Term> = Vec::new();
        let mut off_pattern = BTreeMap::new();
        for (key, sum) in reformed {
            match formed.binary_search_by_key(key, |x| x.0) {
                Ok(at) => {
                    sum.terms_into(&mut scratch, &mut terms);
                    for &(value, exponent) in &terms {
                        accumulators[at].add_scaled_within(-value, exponent, limb_allotment).map_err(exceeded)?;
                    }
                }
                Err(_) => {
                    let value = sum.to_wide_with(&mut scratch);
                    if !value.is_zero() {
                        off_pattern.insert(*key, value.neg());
                    }
                }
            }
        }
        let deltas = formed
            .iter()
            .zip(&accumulators)
            .map(|(&(key, _), sum)| (key, sum.to_wide_with(&mut scratch)))
            .chain(off_pattern.iter().map(|(&key, &v)| (key, v)));
        let (norm, row) = scaled_norm(n, d, deltas, names[0])?;
        let (reformed_norm, _) =
            scaled_norm(n, d, reformed.iter().map(|(&k, sum)| (k, sum.to_wide_with(&mut scratch))), names[1])?;
        Ok((MatrixDifference { norm, row, off_pattern: off_pattern.len() }, reformed_norm))
    }

    fn random_list(mix: &mut Mix, n: usize) -> Vec<Mapped> {
        let elements = 1 + mix.below(6);
        (0..elements)
            .map(|element| {
                let d = [1, 2, 3, 6, 12][mix.below(5)];
                let lower = (0..d * (d + 1) / 2).map(|_| mix.dd()).collect();
                let rows = (0..d).map(|_| (mix.below(5) != 0).then(|| mix.below(n))).collect();
                let mixed = (mix.below(5) < 2).then(|| {
                    (0..d)
                        .map(|_| {
                            let len = mix.below(4);
                            let entries: Vec<(usize, f64)> = (0..len).map(|_| (mix.below(n), mix.value())).collect();
                            Coordinates::new(&entries).unwrap()
                        })
                        .collect()
                });
                Mapped { element: element * 3 + 1, rows, matrix: ElementMatrix { dofs: (0..d).collect(), lower }, mixed }
            })
            .collect()
    }

    fn same_assembly(
        new: &Result<Vec<((usize, usize), ExactSum)>, (usize, LimbAllotmentExceeded)>,
        old: &Result<BTreeMap<(usize, usize), ExactSum>, (usize, LimbAllotmentExceeded)>,
        what: &str,
    ) {
        match (new, old) {
            (Ok(new), Ok(old)) => {
                assert_eq!(new.len(), old.len(), "{what}: entry count");
                assert_eq!(new.capacity(), Vec::<((usize, usize), ExactSum)>::with_capacity(new.len()).capacity(), "{what}: the counted capacity");
                for ((key, sum), (old_key, old_sum)) in new.iter().zip(old) {
                    assert_eq!(key, old_key, "{what}");
                    assert_eq!(sum, old_sum, "{what}: {key:?}");
                    assert_eq!(sum.limb_storage(), old_sum.limb_storage(), "{what}: {key:?}");
                    assert_eq!(sum.to_wide().mantissa.hi.to_bits(), old_sum.to_wide().mantissa.hi.to_bits(), "{what}");
                    assert_eq!(sum.to_wide().mantissa.lo.to_bits(), old_sum.to_wide().mantissa.lo.to_bits(), "{what}");
                    assert_eq!(sum.to_wide().exponent, old_sum.to_wide().exponent, "{what}");
                }
            }
            (Err(new), Err(old)) => assert_eq!(new, old, "{what}: the same element and the same refusal"),
            (new, old) => panic!("{what}: new {:?} against old {:?}", new.as_ref().map(Vec::len), old.as_ref().map(BTreeMap::len)),
        }
    }

    #[test]
    fn the_assembly_is_the_former_map_with_its_refusals() {
        let mut mix = Mix(57);
        let mut refusals = 0;
        // The empty list and a list whose rows are all restrained.
        for list in [vec![], vec![Mapped { element: 0, rows: vec![None, None], matrix: ElementMatrix { dofs: vec![0, 1], lower: vec![Dd::from_f64(1.0); 3] }, mixed: None }]] {
            same_assembly(&ReformedPencil::assembled_exact(&list, 3, None), &former_assembled_exact(&list, None), "empty");
        }
        for case in 0..300 {
            let n = 1 + mix.below(12);
            let list = random_list(&mut mix, n);
            let unlimited = ReformedPencil::assembled_exact(&list, n, None);
            same_assembly(&unlimited, &former_assembled_exact(&list, None), &format!("case {case}"));
            let longest = unlimited.unwrap().iter().map(|(_, s)| s.limb_storage().0).max().unwrap_or(0);
            // Every allotment from 0 to one above the longest sum: below it
            // the same element refuses with the same requirement, at and above
            // it nothing refuses.
            for allotment in 0..=longest + 1 {
                let new = ReformedPencil::assembled_exact(&list, n, Some(allotment));
                refusals += usize::from(new.is_err());
                same_assembly(&new, &former_assembled_exact(&list, Some(allotment)), &format!("case {case}, allotment {allotment}"));
            }
            // The visit count is the traversal's.
            let mut pencil = ReformedPencil { dimension: n, stiffness: list.clone(), mass: list, section_difference: None, workspace: Retained::default() };
            let [k, m] = pencil.assembly_visits();
            assert_eq!(k, m);
            pencil.mass.clear();
            assert_eq!(pencil.assembly_visits()[1], 0);
        }
        assert!(refusals > 0, "the allotments exercise the refusal");
    }

    fn entry(row: usize, col: usize, value: f64) -> SymmetricMatrixEntry {
        SymmetricMatrixEntry { row, col, value }
    }

    fn difference_bits(found: &Result<(MatrixDifference, f64), FormationError>) -> Result<(u64, usize, usize, u64), FormationError> {
        found.clone().map(|(MatrixDifference { norm, row, off_pattern }, reformed)| (norm.to_bits(), row, off_pattern, reformed.to_bits()))
    }

    #[test]
    fn the_formed_order_and_the_difference_are_the_former_ones() {
        let mut mix = Mix(91);
        let mut off = 0;
        for case in 0..300 {
            let n = 1 + mix.below(10);
            let list = random_list(&mut mix, n);
            let new_reformed = ReformedPencil::assembled_exact(&list, n, None).unwrap();
            let old_reformed = former_assembled_exact(&list, None).unwrap();
            // The formed entries: a random part of the re-formed pattern with
            // perturbed values, some absent positions (off-pattern), some extra
            // positions, either orientation, shuffled; sometimes a duplicate.
            let mut formed: Vec<SymmetricMatrixEntry> = Vec::new();
            for ((r, c), sum) in &new_reformed {
                if mix.below(5) == 0 {
                    continue;
                }
                let value = sum.to_f64();
                let value = if value == 0.0 || !value.is_normal() { 1.0 } else { value * (1.0 + (mix.below(3) as f64 - 1.0) * 1e-13) };
                formed.push(if mix.below(2) == 0 { entry(*r, *c, value) } else { entry(*c, *r, value) });
            }
            for _ in 0..mix.below(4) {
                let (r, c) = (mix.below(n), mix.below(n));
                formed.push(entry(r, c, mix.value()));
            }
            for i in (1..formed.len()).rev() {
                formed.swap(i, mix.below(i + 1));
            }
            if case % 7 == 0 && !formed.is_empty() {
                let again = formed[mix.below(formed.len())];
                formed.push(again);
            }
            let matrix = PencilMatrix::Stiffness;
            let new_formed = formed_lower(n, matrix, &formed);
            let old_formed = former_formed_lower(n, matrix, &formed);
            match (&new_formed, &old_formed) {
                (Ok(a), Ok(b)) => {
                    assert_eq!(a.len(), b.len());
                    for (x, y) in a.iter().zip(b) {
                        assert_eq!((x.0, x.1.to_bits()), (y.0, y.1.to_bits()), "case {case}");
                    }
                }
                (a, b) => assert_eq!(a.as_ref().err(), b.as_ref().err(), "case {case}: the same refusal"),
            }
            let Ok(formed) = old_formed else { continue };
            let d: Vec<Dd> = (0..n).map(|_| Dd::from_f64(mix.value().abs())).collect();
            for allotment in [None, Some(3), Some(8), Some(64)] {
                let names = ["difference", "norm"];
                let new = difference(n, &d, &formed, &new_reformed, names, allotment);
                let old = former_difference(n, &d, &formed, &old_reformed, names, allotment);
                assert_eq!(difference_bits(&new), difference_bits(&old), "case {case}, allotment {allotment:?}");
                if let Ok((m, _)) = new {
                    off += m.off_pattern;
                }
            }
        }
        assert!(off > 0, "off-pattern positions are exercised");
    }

    #[test]
    fn the_formed_order_refuses_the_least_duplicate_whatever_the_input_order() {
        let entries = [entry(2, 1, 1.0), entry(1, 1, 2.0), entry(1, 2, 3.0), entry(0, 0, 4.0), entry(0, 0, 5.0)];
        for rotation in 0..entries.len() {
            let mut list = entries.to_vec();
            list.rotate_left(rotation);
            assert_eq!(formed_lower(3, PencilMatrix::Mass, &list), Err(FormationError::DuplicateEntry { row: 0, col: 0 }));
            assert_eq!(former_formed_lower(3, PencilMatrix::Mass, &list), Err(FormationError::DuplicateEntry { row: 0, col: 0 }));
        }
        // An out-of-range entry refuses before any order is formed.
        assert_eq!(formed_lower(3, PencilMatrix::Mass, &[entry(3, 0, 1.0)]), Err(FormationError::EntryOutOfBounds { row: 3, col: 0 }));
        assert_eq!(formed_lower(1, PencilMatrix::Mass, &[]), Ok(vec![]));
        // Reverse order and a singleton.
        let reverse: Vec<_> = (0..4).rev().flat_map(|r| (0..=r).rev().map(move |c| entry(r, c, (r * 4 + c + 1) as f64))).collect();
        assert_eq!(formed_lower(4, PencilMatrix::Mass, &reverse), former_formed_lower(4, PencilMatrix::Mass, &reverse));
        assert_eq!(formed_lower(1, PencilMatrix::Mass, &[entry(0, 0, 1.0)]), Ok(vec![((0, 0), 1.0)]));
    }

    /// The diagonal walk: the first index whose diagonal is absent or zero
    /// refuses, as the binary search did.
    #[test]
    fn the_diagonal_walk_refuses_the_first_missing_or_zero_diagonal() {
        let k = [entry(0, 0, 4.0), entry(1, 0, 1.0), entry(2, 1, 1.0), entry(2, 2, 9.0)];
        let pencil = ReformedPencil { dimension: 3, stiffness: vec![], mass: vec![], section_difference: None, workspace: Retained::default() };
        let found = formation_difference(&pencil, &k, &[]);
        assert_eq!(found, Err(FormationError::ZeroStiffnessDiagonal { index: 1 }));
        let zero = [entry(0, 0, 0.0), entry(1, 1, 1.0)];
        let pencil = ReformedPencil { dimension: 2, ..pencil };
        assert_eq!(formation_difference(&pencil, &zero, &[]), Err(FormationError::ZeroStiffnessDiagonal { index: 0 }));
        let full = [entry(1, 1, 4.0), entry(0, 0, 16.0)];
        let found = formation_difference(&pencil, &full, &[]).unwrap();
        assert_eq!(found.scaling.scaling, vec![Dd::from_f64(0.25), Dd::from_f64(0.5)]);
    }
}
