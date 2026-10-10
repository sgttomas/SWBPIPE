//! K-D5: the D-5 formation check (T3 D1 revision 5a.2 §4.3.1; `R5_4_CURVED.md` §2).
//!
//! After an ordinary attempt passes its residual gate and would publish
//! `Passed`, the check estimates the forward error of the published
//! displacements as EF = K̃⁻¹ρ with the attempt's own factor, where
//! ρ = f − K_int·u is the residual of the *intended* system:
//! - K_int is re-formed in `Wide<2>` at p = 128 from the binary64 primitives:
//!   straight frames (frame, length, local coefficients and TᵀKT, nothing
//!   shared with binary64 `local_stiffness`), realized curved bends as an
//!   objective element (radial vectors, square roots, the K3a included angle,
//!   the closed-form end flexibility, its inverse, and the equilibrium
//!   transfer from the actual chord), user-stiffness elements with zero
//!   lateral stiffness, and objective connectors (T4-U3: BᵀKB from the
//!   binary64 decode, r from the node and offset differences); ground springs
//!   are their exact binary64 values.
//! - Each free row's ρ_i is one `ExactAccumulator` sum (the same exact-sum
//!   machinery as the kernel's intended-action audit): the load terms (the
//!   ledger's terms where the caller supplied them, otherwise the folded
//!   force), and every re-formed coefficient through its exact split into at
//!   most three binary64 terms times the binary64 u_j (prescribed columns
//!   included, so K_fc·u_c is in ρ). A split truncated below 2^-1074 adds
//!   2^-1074·|u_j| to that row's allowance, which widens ρ_i away from zero.
//!
//! The rule (factor 2): demote when 2|w_i| > 1e-9·max(|q_i|, S*_kind) for a
//! free nodal row, or when max(|q_i|, S*_kind) = 0 and w_i ≠ 0. S* is the
//! coupled body scale of D1 §4.1.6.1 items 4–6 formed from the binary64 u.
//! Anything the check cannot re-form or evaluate (a listed family, a
//! `WideError`, an exact-sum range error, a failed correction solve) demotes
//! with `FormationCheckUnavailable`: the check fails closed, never passes a
//! case silently and never turns a solve into an `Err`.
//!
//! EF does not see load formation before S11-F, member-action and reaction
//! recovery, or input representation (D1 §4.3.1, "What EF does not see").
use super::retained::wide::{Wide2, WideArith, WideError};
use super::{binary_exponent, StructuralError, StructuralSystem};
use crate::connector::ObjectiveConnector;
use crate::exact_sum::{ExactAccumulator, SumError};
use crate::load_ledger::ForceTerm;
use crate::{element_dof_map, FrameElement, DOF_PER_NODE, ELEMENT_DOF};

/// Precision of the re-formation (D1 §4.3.1 step 1).
pub const FORMATION_PRECISION: u32 = 128;
/// The protected relative criterion.
pub const FORMATION_CRITERION: f64 = 1e-9;
/// The trigger factor on |w| (D5C-4: 2, calibrated on the product path).
pub const FORMATION_FACTOR: f64 = 2.0;

/// Binary64 inputs of one realized curved bend (DEC-070 macro-element), as
/// the product's `CurvedBendMacroElement` holds them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurvedFormation {
    pub node_i: usize,
    pub node_j: usize,
    pub coordinates_i: [f64; 3],
    pub coordinates_j: [f64; 3],
    pub center: [f64; 3],
    pub elastic_modulus: f64,
    pub shear_modulus: f64,
    pub area: f64,
    pub second_moment: f64,
    pub torsion_constant: f64,
    pub in_plane_flexibility_factor: f64,
    pub out_of_plane_flexibility_factor: f64,
}

/// The primitives of every stiffness contribution of a structural system,
/// for the formation check (the "typed optional formation source", carried
/// by `FormationCheckedSystem`). Node `k` owns global DOFs 6k..6k+5.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationSource {
    pub node_count: usize,
    pub frames: Vec<FrameElement>,
    /// T4-U3 (S10): objective connectors, re-formed by `connector_matrix`
    /// (at 2^b, K-D5's scaled source holds `ObjectiveConnector::force_scaled`).
    pub connectors: Vec<ObjectiveConnector>,
    pub curved: Vec<CurvedFormation>,
    /// Ground springs (global DOF, stiffness), exact binary64 values.
    pub springs: Vec<(usize, f64)>,
    /// Contributions the check cannot re-form, named (D5C-2). Any entry
    /// demotes the case with `FormationCheckUnavailable`.
    pub unavailable: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormationCheckReason {
    /// The estimate exceeded the criterion (`estimate`).
    Estimate,
    /// The check could not re-form or evaluate the case
    /// (`formation_check_unavailable`); `detail` names the family or failure.
    FormationCheckUnavailable { detail: String },
}

/// Present on `StructuralSolution` only when the check demotes a case; never
/// part of `StructuralReport` (D5C-3), so a report that is not demoted is
/// byte-unchanged and a demoted one differs only in `quality: Sensitive`.
#[derive(Debug, Clone, PartialEq)]
pub struct FormationCheck {
    pub reason: FormationCheckReason,
    /// The demoting free row (the largest trigger value); `None` when unavailable.
    pub global_dof: Option<usize>,
    /// 2·|w_i| at that row, in the row's physical unit (m or rad).
    pub doubled_correction: f64,
    /// max(|q_i|, S*_kind) at that row.
    pub scale: f64,
    /// Trigger value 2|w_i| / (1e-9·scale); +∞ for the zero-scale clause.
    /// The quoted EF reading is half of it.
    pub ratio: f64,
}

impl FormationCheck {
    fn unavailable(detail: impl Into<String>) -> Self {
        Self {
            reason: FormationCheckReason::FormationCheckUnavailable {
                detail: detail.into(),
            },
            global_dof: None,
            doubled_correction: f64::NAN,
            scale: f64::NAN,
            ratio: f64::NAN,
        }
    }
}

/// Why an evaluation could not complete; every variant fails closed.
#[derive(Debug)]
enum Failure {
    Wide(WideError),
    Sum(SumError),
    Structural(StructuralError),
    Shape(&'static str),
}
impl From<WideError> for Failure {
    fn from(e: WideError) -> Self {
        Self::Wide(e)
    }
}
impl From<SumError> for Failure {
    fn from(e: SumError) -> Self {
        Self::Sum(e)
    }
}
impl From<StructuralError> for Failure {
    fn from(e: StructuralError) -> Self {
        Self::Structural(e)
    }
}
impl Failure {
    fn detail(&self) -> String {
        match self {
            Self::Wide(e) => format!("retained arithmetic: {e}"),
            Self::Sum(e) => format!("exact residual sum: {e}"),
            Self::Structural(e) => format!("correction solve: {e}"),
            Self::Shape(s) => format!("formation source: {s}"),
        }
    }
}

/// Runs the check. `None`: the case stays `Passed`. `Some`: the case is
/// demoted to `Sensitive` with this record. Never an error.
pub(super) fn check<F>(
    system: &StructuralSystem<'_>,
    source: &FormationSource,
    force_terms: Option<&[ForceTerm]>,
    scale_exponents: &[i32],
    u: &[f64],
    solve: &F,
) -> Option<FormationCheck>
where
    F: Fn(&[f64]) -> Result<Vec<f64>, StructuralError>,
{
    if let Some(family) = source.unavailable.first() {
        return Some(FormationCheck::unavailable(family.clone()));
    }
    match evaluate(system, source, force_terms, scale_exponents, u, solve) {
        Ok(result) => result,
        Err(failure) => Some(FormationCheck::unavailable(failure.detail())),
    }
}

fn evaluate<F>(
    system: &StructuralSystem<'_>,
    source: &FormationSource,
    force_terms: Option<&[ForceTerm]>,
    scale_exponents: &[i32],
    u: &[f64],
    solve: &F,
) -> Result<Option<FormationCheck>, Failure>
where
    F: Fn(&[f64]) -> Result<Vec<f64>, StructuralError>,
{
    let n = system.force.len();
    let free = system.free_dofs;
    if source.node_count.checked_mul(DOF_PER_NODE) != Some(n)
        || u.len() != n
        || scale_exponents.len() != free.len()
    {
        return Err(Failure::Shape("DOF map"));
    }
    let mut free_index = vec![None; n];
    for (r, &i) in free.iter().enumerate() {
        free_index[i] = Some(r);
    }
    let mut rows = vec![ExactAccumulator::new(); free.len()];
    // |u_j| of every split truncated below 2^-1074 (the per-row allowance).
    let mut truncated: Vec<Vec<f64>> = vec![Vec::new(); free.len()];

    // f: the ledger terms where supplied, otherwise the folded force.
    match force_terms {
        Some(terms) => {
            for term in terms {
                if term.dof >= n {
                    return Err(Failure::Shape("force term DOF"));
                }
                if let Some(r) = free_index[term.dof] {
                    term.accumulate(&mut rows[r], false)?;
                }
            }
        }
        None => {
            for (r, &i) in free.iter().enumerate() {
                rows[r].add(system.force[i])?;
            }
        }
    }

    // − K_int·u, element by element, each coefficient through its exact split.
    let mut arith = WideArith::new(FORMATION_PRECISION)?;
    let mut apply = |ni: usize, nj: usize, k: &Element| -> Result<(), Failure> {
        if ni >= source.node_count || nj >= source.node_count || ni == nj {
            return Err(Failure::Shape("element node"));
        }
        let map = element_dof_map(ni, nj);
        for a in 0..ELEMENT_DOF {
            let Some(r) = free_index[map[a]] else {
                continue;
            };
            for b in 0..ELEMENT_DOF {
                let x = u[map[b]];
                if x == 0.0 || k[a][b].is_zero() {
                    continue;
                }
                if k[a][b].neg().add_product_to(&mut rows[r], x)? {
                    truncated[r].push(x.abs());
                }
            }
        }
        Ok(())
    };
    for e in &source.frames {
        let k = frame_matrix(&mut arith, e)?;
        apply(e.node_i.index, e.node_j.index, &k)?;
    }
    for c in &source.connectors {
        let k = connector_matrix(&mut arith, c)?;
        apply(c.node_i().index, c.node_j().index, &k)?;
    }
    for e in &source.curved {
        let k = curved_matrix(&mut arith, e)?;
        apply(e.node_i, e.node_j, &k)?;
    }
    for &(dof, k) in &source.springs {
        if dof >= n {
            return Err(Failure::Shape("spring DOF"));
        }
        if let Some(r) = free_index[dof] {
            rows[r].add_product(-k, u[dof])?;
        }
    }
    // Widen a truncated row away from zero by its allowance.
    let tiny = f64::from_bits(1); // 2^-1074
    for (row, list) in rows.iter_mut().zip(&truncated) {
        if list.is_empty() {
            continue;
        }
        let sign = if row.signum() < 0 { -tiny } else { tiny };
        for &x in list {
            row.add_product(sign, x)?;
        }
    }

    // w = K̃⁻¹ρ in the attempt's scaled variables (as refinement does), with
    // one extra power of two 2^s so that the largest entry lies in [1, 2).
    let mut exponents = Vec::with_capacity(free.len());
    for (row, &e) in rows.iter().zip(scale_exponents) {
        exponents.push(scaled_exponent(row, e)?);
    }
    let Some(top) = exponents.iter().flatten().copied().max() else {
        return Ok(None); // ρ = 0 exactly: w = 0.
    };
    let shift = -top;
    let mut rhs = Vec::with_capacity(free.len());
    for (row, &e) in rows.iter().zip(scale_exponents) {
        let v = row.round_scaled(e + shift)?;
        // An entry more than 2^1022 below the largest one is flushed to zero.
        rhs.push(if v.is_subnormal() { 0.0 } else { v });
    }
    let delta = solve(&rhs)?;
    if delta.len() != free.len() || delta.iter().any(|v| !v.is_finite()) {
        return Err(Failure::Shape("correction solve output"));
    }

    // The rule, per free nodal row, against S*_kind of the row's body.
    let scales = body_scales(source, free, u);
    let mut worst: Option<FormationCheck> = None;
    for (r, &i) in free.iter().enumerate() {
        let node = i / DOF_PER_NODE;
        let rotation = i % DOF_PER_NODE >= 3;
        let (tr, ro) = scales[node];
        let scale = u[i].abs().max(if rotation { ro } else { tr });
        let w = pow2(delta[r], scale_exponents[r] - shift);
        let doubled = FORMATION_FACTOR * w.abs();
        let ratio = if scale == 0.0 {
            if delta[r] != 0.0 {
                f64::INFINITY
            } else {
                0.0
            }
        } else {
            let criterion = FORMATION_CRITERION * scale;
            if doubled > criterion {
                doubled / criterion
            } else {
                0.0
            }
        };
        if ratio > 0.0 && worst.as_ref().is_none_or(|w| ratio > w.ratio) {
            worst = Some(FormationCheck {
                reason: FormationCheckReason::Estimate,
                global_dof: Some(i),
                doubled_correction: doubled,
                scale,
                ratio,
            });
        }
    }
    Ok(worst)
}

/// Binary exponent of ρ_i·2^e, found without rounding it out of range.
fn scaled_exponent(row: &ExactAccumulator, e: i32) -> Result<Option<i32>, Failure> {
    if row.is_zero() {
        return Ok(None);
    }
    for shift in [
        0, 600, -600, 1200, -1200, 1800, -1800, 2400, -2400, 3000, -3000,
    ] {
        if let Ok(v) = row.round_scaled(e + shift) {
            if v.is_normal() {
                return Ok(Some(binary_exponent(v) - shift));
            }
        }
    }
    Err(Failure::Shape("residual exponent"))
}

/// value·2^k in binary64 steps; overflow gives ±∞ and underflow ±0 (the
/// rule treats ∞ as a demotion and uses the unscaled value for zero-scale).
fn pow2(mut value: f64, mut k: i32) -> f64 {
    while k != 0 {
        let step = k.clamp(-512, 512);
        value *= 2.0_f64.powi(step);
        k -= step;
    }
    value
}

/// (translation, rotation) S* of each node's body (D1 §4.1.6.1 items 1, 4–6):
/// bodies are the connected components of the element graph; S(kind) is the
/// largest free |u| of that kind in the body; L_b from the body's extents.
fn body_scales(source: &FormationSource, free: &[usize], u: &[f64]) -> Vec<(f64, f64)> {
    let count = source.node_count;
    let mut parent: Vec<usize> = (0..count).collect();
    fn root(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
    let mut coordinates: Vec<Option<[f64; 3]>> = vec![None; count];
    let mut edges: Vec<(usize, [f64; 3], usize, [f64; 3])> = Vec::new();
    for e in &source.frames {
        edges.push((
            e.node_i.index,
            e.node_i.coordinates,
            e.node_j.index,
            e.node_j.coordinates,
        ));
    }
    for c in &source.connectors {
        let (i, j) = (c.node_i(), c.node_j());
        edges.push((i.index, i.coordinates, j.index, j.coordinates));
    }
    for e in &source.curved {
        edges.push((e.node_i, e.coordinates_i, e.node_j, e.coordinates_j));
    }
    for &(a, pa, b, pb) in &edges {
        coordinates[a] = Some(pa);
        coordinates[b] = Some(pb);
        let (ra, rb) = (root(&mut parent, a), root(&mut parent, b));
        if ra != rb {
            parent[ra] = rb;
        }
    }
    let mut s_tr = vec![0.0_f64; count];
    let mut s_ro = vec![0.0_f64; count];
    for &i in free {
        let b = root(&mut parent, i / DOF_PER_NODE);
        if i % DOF_PER_NODE < 3 {
            s_tr[b] = s_tr[b].max(u[i].abs());
        } else {
            s_ro[b] = s_ro[b].max(u[i].abs());
        }
    }
    let mut low = vec![[f64::INFINITY; 3]; count];
    let mut high = vec![[f64::NEG_INFINITY; 3]; count];
    for node in 0..count {
        if let Some(p) = coordinates[node] {
            let b = root(&mut parent, node);
            for k in 0..3 {
                low[b][k] = low[b][k].min(p[k]);
                high[b][k] = high[b][k].max(p[k]);
            }
        }
    }
    let mut body = vec![(0.0, 0.0); count];
    for b in 0..count {
        if root(&mut parent, b) != b {
            continue;
        }
        let d: Vec<f64> = (0..3)
            .map(|k| {
                if high[b][k] >= low[b][k] {
                    high[b][k] - low[b][k]
                } else {
                    0.0
                }
            })
            .collect();
        let l_b = ((d[0] * d[0] + d[1] * d[1]) + d[2] * d[2]).sqrt();
        body[b] = if l_b > 0.0 {
            (s_tr[b].max(l_b * s_ro[b]), s_ro[b].max(s_tr[b] / l_b))
        } else {
            (s_tr[b], s_ro[b])
        };
    }
    (0..count)
        .map(|node| body[root(&mut parent, node)])
        .collect()
}

// --------------------------------------------------------------- re-formation

type Element = [[Wide2; ELEMENT_DOF]; ELEMENT_DOF];
type Vec3 = [Wide2; 3];
type M6 = [[Wide2; 6]; 6];

fn lift(x: f64) -> Result<Wide2, WideError> {
    Wide2::from_f64(x)
}
fn lift3(v: [f64; 3]) -> Result<Vec3, WideError> {
    Ok([lift(v[0])?, lift(v[1])?, lift(v[2])?])
}
fn sub3(a: &mut WideArith, x: &Vec3, y: &Vec3) -> Result<Vec3, WideError> {
    Ok([
        a.sub(&x[0], &y[0])?,
        a.sub(&x[1], &y[1])?,
        a.sub(&x[2], &y[2])?,
    ])
}
fn dot3(a: &mut WideArith, x: &Vec3, y: &Vec3) -> Result<Wide2, WideError> {
    let p0 = a.mul(&x[0], &y[0])?;
    let p1 = a.mul(&x[1], &y[1])?;
    let p2 = a.mul(&x[2], &y[2])?;
    let s = a.add(&p0, &p1)?;
    a.add(&s, &p2)
}
fn cross3(a: &mut WideArith, x: &Vec3, y: &Vec3) -> Result<Vec3, WideError> {
    let c = |a: &mut WideArith, p: usize, q: usize| -> Result<Wide2, WideError> {
        let l = a.mul(&x[p], &y[q])?;
        let r = a.mul(&x[q], &y[p])?;
        a.sub(&l, &r)
    };
    Ok([c(a, 1, 2)?, c(a, 2, 0)?, c(a, 0, 1)?])
}
fn scale3(a: &mut WideArith, x: &Vec3, d: &Wide2) -> Result<Vec3, WideError> {
    Ok([a.div(&x[0], d)?, a.div(&x[1], d)?, a.div(&x[2], d)?])
}
fn norm3(a: &mut WideArith, x: &Vec3) -> Result<Wide2, WideError> {
    let s = dot3(a, x, x)?;
    a.sqrt(&s)
}

/// Local axes (rows: x, y, z) from the actual chord and a y reference, as
/// `FrameOrientation::from_x_axis_and_y_reference` defines them, at p.
fn chord_axes(
    a: &mut WideArith,
    xi: [f64; 3],
    xj: [f64; 3],
    y_reference: [f64; 3],
) -> Result<([Vec3; 3], Wide2), WideError> {
    let d = sub3(a, &lift3(xj)?, &lift3(xi)?)?;
    let length = norm3(a, &d)?;
    let ex = scale3(a, &d, &length)?;
    let yr = lift3(y_reference)?;
    let projection = dot3(a, &yr, &ex)?;
    let mut yc = [Wide2::ZERO; 3];
    for k in 0..3 {
        let t = a.mul(&projection, &ex[k])?;
        yc[k] = a.sub(&yr[k], &t)?;
    }
    let ym = norm3(a, &yc)?;
    let ey = scale3(a, &yc, &ym)?;
    let ez = cross3(a, &ex, &ey)?;
    Ok(([ex, ey, ez], length))
}

/// Tᵀ K T with T = diag(R, R, R, R), R's rows the local axes, at p.
fn rotate(a: &mut WideArith, k: &Element, axes: &[Vec3; 3]) -> Result<Element, WideError> {
    let mut out = [[Wide2::ZERO; ELEMENT_DOF]; ELEMENT_DOF];
    for bi in 0..4 {
        for bj in 0..4 {
            let mut empty = true;
            for p in 0..3 {
                for q in 0..3 {
                    empty &= k[3 * bi + p][3 * bj + q].is_zero();
                }
            }
            if empty {
                continue;
            }
            // temp = K_block · R
            let mut temp = [[Wide2::ZERO; 3]; 3];
            for p in 0..3 {
                for c in 0..3 {
                    let mut s = Wide2::ZERO;
                    for q in 0..3 {
                        let kv = &k[3 * bi + p][3 * bj + q];
                        if kv.is_zero() || axes[q][c].is_zero() {
                            continue;
                        }
                        let t = a.mul(kv, &axes[q][c])?;
                        s = a.add(&s, &t)?;
                    }
                    temp[p][c] = s;
                }
            }
            // out_block = Rᵀ · temp
            for r in 0..3 {
                for c in 0..3 {
                    let mut s = Wide2::ZERO;
                    for p in 0..3 {
                        if axes[p][r].is_zero() || temp[p][c].is_zero() {
                            continue;
                        }
                        let t = a.mul(&axes[p][r], &temp[p][c])?;
                        s = a.add(&s, &t)?;
                    }
                    out[3 * bi + r][3 * bj + c] = s;
                }
            }
        }
    }
    Ok(out)
}

fn small(v: f64) -> Wide2 {
    // Exact lifts of the small integers and powers of two used below.
    Wide2::from_f64(v).expect("finite constant")
}

/// A straight frame re-formed from its primitives (D1 §4.3.1 step 1): the
/// frame, L, EA/L, GJ/L, 12EI/L³, 6EI/L², 4EI/L, 2EI/L and TᵀKT, all at p.
fn frame_matrix(a: &mut WideArith, e: &FrameElement) -> Result<Element, WideError> {
    let (axes, length) = chord_axes(a, e.node_i.coordinates, e.node_j.coordinates, e.y_reference)?;
    let s = &e.section;
    let (em, gm) = (lift(s.elastic_modulus)?, lift(s.shear_modulus)?);
    let l2 = a.mul(&length, &length)?;
    let l3 = a.mul(&l2, &length)?;
    let ea = a.mul(&em, &lift(s.area)?)?;
    let axial = a.div(&ea, &length)?;
    let gj = a.mul(&gm, &lift(s.torsion_constant)?)?;
    let torsion = a.div(&gj, &length)?;
    let mut coefficients = |i: f64| -> Result<[Wide2; 4], WideError> {
        let ei = a.mul(&em, &lift(i)?)?;
        let t12 = a.mul(&small(12.0), &ei)?;
        let t6 = a.mul(&small(6.0), &ei)?;
        let t4 = a.mul(&small(4.0), &ei)?;
        let t2 = a.mul(&small(2.0), &ei)?;
        Ok([
            a.div(&t12, &l3)?,
            a.div(&t6, &l2)?,
            a.div(&t4, &length)?,
            a.div(&t2, &length)?,
        ])
    };
    let [z12, z6, z4, z2] = coefficients(s.second_moment_z)?;
    let [y12, y6, y4, y2] = coefficients(s.second_moment_y)?;
    let mut k = [[Wide2::ZERO; ELEMENT_DOF]; ELEMENT_DOF];
    let pair = |k: &mut Element, i: usize, j: usize, v: Wide2| {
        k[i][i] = v;
        k[j][j] = v;
        k[i][j] = v.neg();
        k[j][i] = v.neg();
    };
    pair(&mut k, 0, 6, axial);
    pair(&mut k, 3, 9, torsion);
    // FK add_bending_z (uy, rz) and add_bending_y (uz, ry) patterns.
    let bz = [
        [z12, z6, z12.neg(), z6],
        [z6, z4, z6.neg(), z2],
        [z12.neg(), z6.neg(), z12, z6.neg()],
        [z6, z2, z6.neg(), z4],
    ];
    let by = [
        [y12, y6.neg(), y12.neg(), y6.neg()],
        [y6.neg(), y4, y6, y2],
        [y12.neg(), y6, y12, y6],
        [y6.neg(), y2, y6, y4],
    ];
    for (idx, t) in [([1usize, 5, 7, 11], bz), ([2usize, 4, 8, 10], by)] {
        for r in 0..4 {
            for c in 0..4 {
                k[idx[r]][idx[c]] = t[r][c];
            }
        }
    }
    rotate(a, &k, &axes)
}

/// T4-U3 (S10): an objective connector's BᵀKB re-formed at p from the
/// binary64 decode (node coordinates, global offsets, Q and K), sharing no
/// arithmetic with `ObjectiveConnector::b` or `global_stiffness`:
/// r = (x_j − x_i) + (a_j − a_i) (never x + a), M_i = S(a_i) + S(r)/2,
/// M_j = S(r)/2 − S(a_j), B_t = Qᵀ[−I, M_i, I, M_j], B_r = Qᵀ[0, −I, 0, I],
/// then K·B and Bᵀ(K·B).
fn connector_matrix(a: &mut WideArith, c: &ObjectiveConnector) -> Result<Element, WideError> {
    let xi = lift3(c.node_i().coordinates)?;
    let xj = lift3(c.node_j().coordinates)?;
    let (ai, aj) = c.offsets();
    let (ai, aj) = (lift3(ai)?, lift3(aj)?);
    let d = sub3(a, &xj, &xi)?;
    let o = sub3(a, &aj, &ai)?;
    let r = [
        a.add(&d[0], &o[0])?,
        a.add(&d[1], &o[1])?,
        a.add(&d[2], &o[2])?,
    ];
    let half = [r[0].mul_pow2(-1)?, r[1].mul_pow2(-1)?, r[2].mul_pow2(-1)?];
    let skew = |v: &Vec3| -> [[Wide2; 3]; 3] {
        [
            [Wide2::ZERO, v[2].neg(), v[1]],
            [v[2], Wide2::ZERO, v[0].neg()],
            [v[1].neg(), v[0], Wide2::ZERO],
        ]
    };
    let (s_r, s_ai, s_aj) = (skew(&half), skew(&ai), skew(&aj));
    let mut m_i = [[Wide2::ZERO; 3]; 3];
    let mut m_j = [[Wide2::ZERO; 3]; 3];
    for row in 0..3 {
        for col in 0..3 {
            m_i[row][col] = a.add(&s_ai[row][col], &s_r[row][col])?;
            m_j[row][col] = a.sub(&s_r[row][col], &s_aj[row][col])?;
        }
    }
    let axes = c.axes();
    let mut q = [[Wide2::ZERO; 3]; 3];
    for row in 0..3 {
        q[row] = lift3(axes[row])?;
    }
    let mut b = [[Wide2::ZERO; ELEMENT_DOF]; 6];
    for axis in 0..3 {
        for col in 0..3 {
            let qc = q[col][axis];
            b[axis][col] = qc.neg();
            b[axis][6 + col] = qc;
            b[3 + axis][3 + col] = qc.neg();
            b[3 + axis][9 + col] = qc;
            let mut ti = Wide2::ZERO;
            let mut tj = Wide2::ZERO;
            for k in 0..3 {
                let pi = a.mul(&q[k][axis], &m_i[k][col])?;
                ti = a.add(&ti, &pi)?;
                let pj = a.mul(&q[k][axis], &m_j[k][col])?;
                tj = a.add(&tj, &pj)?;
            }
            b[axis][3 + col] = ti;
            b[axis][9 + col] = tj;
        }
    }
    let stiffness = c.stiffness();
    let mut k = [[Wide2::ZERO; 6]; 6];
    for row in 0..6 {
        for col in 0..6 {
            k[row][col] = lift(stiffness[row][col])?;
        }
    }
    let mut kb = [[Wide2::ZERO; ELEMENT_DOF]; 6];
    for row in 0..6 {
        for col in 0..ELEMENT_DOF {
            let mut s = Wide2::ZERO;
            for l in 0..6 {
                if k[row][l].is_zero() || b[l][col].is_zero() {
                    continue;
                }
                let t = a.mul(&k[row][l], &b[l][col])?;
                s = a.add(&s, &t)?;
            }
            kb[row][col] = s;
        }
    }
    let mut out = [[Wide2::ZERO; ELEMENT_DOF]; ELEMENT_DOF];
    for i in 0..ELEMENT_DOF {
        for j in 0..ELEMENT_DOF {
            let mut s = Wide2::ZERO;
            for row in 0..6 {
                if b[row][i].is_zero() || kb[row][j].is_zero() {
                    continue;
                }
                let t = a.mul(&b[row][i], &kb[row][j])?;
                s = a.add(&s, &t)?;
            }
            out[i][j] = s;
        }
    }
    Ok(out)
}

/// A realized curved bend re-formed as an objective element
/// (`R5_4_CURVED.md` §2 steps 1–6).
fn curved_matrix(a: &mut WideArith, e: &CurvedFormation) -> Result<Element, WideError> {
    let xi = lift3(e.coordinates_i)?;
    let xj = lift3(e.coordinates_j)?;
    let c = lift3(e.center)?;
    // 1. Radial vectors, their lengths and R.
    let ri = sub3(a, &xi, &c)?;
    let rj = sub3(a, &xj, &c)?;
    let ni = norm3(a, &ri)?;
    let nj = norm3(a, &rj)?;
    let sum = a.add(&ni, &nj)?;
    let radius = sum.mul_pow2(-1)?;
    // 2. cos φ and sin φ with square roots only.
    let normal = cross3(a, &ri, &rj)?;
    let nn = norm3(a, &normal)?;
    let nij = a.mul(&ni, &nj)?;
    let rr = dot3(a, &ri, &rj)?;
    let cos = a.div(&rr, &nij)?;
    let sin = a.div(&nn, &nij)?;
    // 3. φ by the K3a included-angle arctangent.
    let phi = a.included_angle(&sin, &cos)?;
    // 4. The product's closed-form end flexibility at p, and its inverse.
    let one = Wide2::ONE;
    let sc = a.mul(&sin, &cos)?;
    let sin2 = sc.mul_pow2(1)?; // sin 2φ = 2 sin φ cos φ
    let one_minus_cos = a.sub(&one, &cos)?;
    let ss = a.mul(&sin, &sin)?;
    let half_ss = ss.mul_pow2(-1)?;
    let half_phi = phi.mul_pow2(-1)?;
    let quarter_sin2 = sin2.mul_pow2(-2)?;
    let gram = [
        [phi, sin, one_minus_cos],
        [sin, a.add(&half_phi, &quarter_sin2)?, half_ss],
        [one_minus_cos, half_ss, a.sub(&half_phi, &quarter_sin2)?],
    ];
    let cases = unit_load_actions(a, &radius, &sin, &cos)?;
    let em = lift(e.elastic_modulus)?;
    let bending = a.mul(&em, &lift(e.second_moment)?)?;
    let torsion = a.mul(&lift(e.shear_modulus)?, &lift(e.torsion_constant)?)?;
    let axial = a.mul(&em, &lift(e.area)?)?;
    let fin = lift(e.in_plane_flexibility_factor)?;
    let fout = lift(e.out_of_plane_flexibility_factor)?;
    let mut flexibility = [[Wide2::ZERO; 6]; 6];
    for row in 0..6 {
        for col in row..6 {
            let q_in = quad(a, &gram, &cases[row][0], &cases[col][0])?;
            let q_out = quad(a, &gram, &cases[row][1], &cases[col][1])?;
            let q_t = quad(a, &gram, &cases[row][2], &cases[col][2])?;
            let q_a = quad(a, &gram, &cases[row][3], &cases[col][3])?;
            let t_in = a.mul(&fin, &q_in)?;
            let t_in = a.div(&t_in, &bending)?;
            let t_out = a.mul(&fout, &q_out)?;
            let t_out = a.div(&t_out, &bending)?;
            let t_t = a.div(&q_t, &torsion)?;
            let t_a = a.div(&q_a, &axial)?;
            let s = a.add(&t_in, &t_out)?;
            let s = a.add(&s, &t_t)?;
            let s = a.add(&s, &t_a)?;
            let v = a.mul(&radius, &s)?;
            flexibility[row][col] = v;
            flexibility[col][row] = v;
        }
    }
    let tip = invert6(a, &flexibility)?;
    // 5. Local axes: x radial at node i, z the bend-plane normal, y = z × x.
    let ex = scale3(a, &ri, &ni)?;
    let ez = scale3(a, &normal, &nn)?;
    let ey = cross3(a, &ez, &ex)?;
    let axes = [ex, ey, ez];
    // 6. H from the actual chord x_j − x_i in the local axes.
    let d = sub3(a, &xj, &xi)?;
    let chord = [
        dot3(a, &axes[0], &d)?,
        dot3(a, &axes[1], &d)?,
        dot3(a, &axes[2], &d)?,
    ];
    let mut h = [[Wide2::ZERO; 6]; 6];
    for (i, row) in h.iter_mut().enumerate() {
        row[i] = one;
    }
    h[3][1] = chord[2].neg();
    h[3][2] = chord[1];
    h[4][0] = chord[2];
    h[4][2] = chord[0].neg();
    h[5][0] = chord[1].neg();
    h[5][1] = chord[0];
    let coupled = mul6(a, &h, &tip, false)?;
    let anchored = mul6(a, &coupled, &h, true)?;
    let mut k = [[Wide2::ZERO; ELEMENT_DOF]; ELEMENT_DOF];
    for r in 0..6 {
        for c in 0..6 {
            k[r][c] = anchored[r][c];
            k[r][c + 6] = coupled[r][c].neg();
            k[r + 6][c] = coupled[c][r].neg();
            k[r + 6][c + 6] = tip[r][c];
        }
    }
    rotate(a, &k, &axes)
}

/// The product's `unit_load_actions` series (in-plane, out-of-plane,
/// torsion, axial) for the six unit loads at node j, from R, sin φ and cos φ.
fn unit_load_actions(
    a: &mut WideArith,
    radius: &Wide2,
    sin: &Wide2,
    cos: &Wide2,
) -> Result<[[Vec3; 4]; 6], WideError> {
    let z = Wide2::ZERO;
    let one = Wide2::ONE;
    let r = *radius;
    let rs = a.mul(&r, sin)?;
    let rc = a.mul(&r, cos)?;
    let zero3 = [z, z, z];
    Ok([
        [[rs.neg(), z, r], zero3, zero3, [z, z, one.neg()]],
        [[rc, r.neg(), z], zero3, zero3, [z, one, z]],
        [zero3, [z, rs, rc.neg()], [r, rc.neg(), rs.neg()], zero3],
        [zero3, [z, one, z], [z, z, one.neg()], zero3],
        [zero3, [z, z, one], [z, one, z], zero3],
        [[one, z, z], zero3, zero3, zero3],
    ])
}

fn quad(a: &mut WideArith, g: &[Vec3; 3], left: &Vec3, right: &Vec3) -> Result<Wide2, WideError> {
    let mut s = Wide2::ZERO;
    for r in 0..3 {
        if left[r].is_zero() {
            continue;
        }
        for c in 0..3 {
            if right[c].is_zero() || g[r][c].is_zero() {
                continue;
            }
            let t = a.mul(&left[r], &g[r][c])?;
            let t = a.mul(&t, &right[c])?;
            s = a.add(&s, &t)?;
        }
    }
    Ok(s)
}

fn mul6(a: &mut WideArith, x: &M6, y: &M6, transpose_right: bool) -> Result<M6, WideError> {
    let mut out = [[Wide2::ZERO; 6]; 6];
    for r in 0..6 {
        for c in 0..6 {
            let mut s = Wide2::ZERO;
            for k in 0..6 {
                let yv = if transpose_right { &y[c][k] } else { &y[k][c] };
                if x[r][k].is_zero() || yv.is_zero() {
                    continue;
                }
                let t = a.mul(&x[r][k], yv)?;
                s = a.add(&s, &t)?;
            }
            out[r][c] = s;
        }
    }
    Ok(out)
}

/// Gauss–Jordan inverse at p with partial pivoting, then the symmetric part.
fn invert6(a: &mut WideArith, f: &M6) -> Result<M6, WideError> {
    let mut m = *f;
    let mut inv = [[Wide2::ZERO; 6]; 6];
    for (i, row) in inv.iter_mut().enumerate() {
        row[i] = Wide2::ONE;
    }
    for col in 0..6 {
        let mut pivot = col;
        for r in col + 1..6 {
            if m[r][col].abs().cmp_value(&m[pivot][col].abs()).is_gt() {
                pivot = r;
            }
        }
        m.swap(col, pivot);
        inv.swap(col, pivot);
        let d = m[col][col];
        for c in 0..6 {
            m[col][c] = a.div(&m[col][c], &d)?;
            inv[col][c] = a.div(&inv[col][c], &d)?;
        }
        for r in 0..6 {
            if r == col || m[r][col].is_zero() {
                continue;
            }
            let factor = m[r][col];
            for c in 0..6 {
                let t = a.mul(&factor, &m[col][c])?;
                m[r][c] = a.sub(&m[r][c], &t)?;
                let t = a.mul(&factor, &inv[col][c])?;
                inv[r][c] = a.sub(&inv[r][c], &t)?;
            }
        }
    }
    let mut out = inv;
    for r in 0..6 {
        for c in r + 1..6 {
            let s = a.add(&inv[r][c], &inv[c][r])?;
            let v = s.mul_pow2(-1)?;
            out[r][c] = v;
            out[c][r] = v;
        }
    }
    Ok(out)
}

#[cfg(test)]
#[path = "formation_check_tests.rs"]
mod tests;
