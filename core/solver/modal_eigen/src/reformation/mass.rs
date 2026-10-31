//! Element mass re-formation in double-double (design §2.2 to §2.4; §3.4
//! check 1 (ii)).
//!
//! **Straight, Euler–Bernoulli** (§2.2). In member axes, axial m̄L·[1/3, 1/6]
//! on (u_x,i, u_x,j), torsional j̄L·[1/3, 1/6] on (θ_x,i, θ_x,j), and in each
//! bending plane the consistent m̄L/420·[156, 22L, 54, −13L; …] plus the
//! Rayleigh ī/(30L)·[36, 3L, −36, 3L; …]. The plane matrices act on
//! [u_y, θ_z] in the x–y plane and on [u_z, −θ_y] in the x–z plane, so the
//! x–z placement multiplies entry (a, b) by s_a·s_b with s = (1, −1, 1, −1).
//! The local 12×12 is turned to global axes with TᵀMT, T = diag(R, R, R, R),
//! R's rows the member axes formed from the chord and the y reference as the
//! frame kernel forms them.
//!
//! **Realized arc** (§2.4). With d = x_j − x_i, L = |d|, n̂ the unit part of
//! the y reference normal to d̂ (the bow side), s = sin α = L/(2R),
//! c = cos α = √(R² − L²/4)/R and α = 2·atan(s/(1 + c)) the half-angle. Each
//! half is a rigid lump about its end node in its own frame: e1 radial
//! (centre to node), e2 the tangent into the half, e3 = e1 × e2; at node i
//! e1 = −s·d̂ + c·n̂, e2 = c·d̂ + s·n̂, and at node j e1 = s·d̂ + c·n̂,
//! e2 = −c·d̂ + s·n̂. In that frame, per scale factor (m̄R for the mass, m̄R²
//! for S, m̄R³ for J_c, R for T_t):
//!
//! | Entry | Value | Evaluated as |
//! |---|---|---|
//! | mass | α | α |
//! | S_1 | sin α − α | −f₁(α) |
//! | S_2 | 1 − cos α | s²/(1 + c) |
//! | J_c,11 = T_t,11 | α/2 − sin 2α/4 | f₂(α) |
//! | J_c,22 | 3α/2 + sin 2α/4 − 2 sin α | f₃(α) |
//! | J_c,33 | 2(α − sin α) | 2f₁(α) |
//! | J_c,12 | 1 − cos α − sin²α/2 | s⁴/(2(1 + c)²) |
//! | T_t,22 | α/2 + sin 2α/4 | α/2 + sc/2 |
//! | T_t,12 | −sin²α/2 | −s²/2 |
//!
//! with the series f₁(α) = α − sin α = Σ_{k≥1} (−1)^{k+1} α^{2k+1}/(2k+1)!,
//! f₂(α) = f₁(2α)/4 and f₃(α) = Σ_{k≥2} (−1)^k (2^{2k−1} − 2) α^{2k+1}/(2k+1)!
//! (leading terms α³/6, α³/3 and α⁵/20), each summed until a term falls
//! below 2⁻¹¹⁰ of the sum. **The switch point is the whole admitted range**,
//! 0 < α < π/2 (the chord is shorter than the diameter): there the series
//! lose at most about a factor 2 to cancellation, while the closed forms of
//! f₁, f₂ and f₃ lose a factor of order α⁻², α⁻² and α⁻⁴ (a 10⁻²⁸ relative
//! error at α = 1/2 for f₃), so the series are used for every α. The other
//! entries have no cancellation in the forms shown. The section term is
//! J_s = R·[j̄·T_t + ī·(α·I − T_t)], and the lump's second moment is
//! m̄R³·J_c + J_s; first moment S and second moment J are turned to global
//! axes as Σ S_k·e_k and Σ J_ab·e_a·e_bᵀ. The 6×6 is
//! [[m·I, −[S]×], [[S]×, J]].
//!
//! **Component body** (§2.3). m = W/a, at offset r from the node: the 6×6
//! [[m·I, −m[r]×], [m[r]×, m(|r|²I − rrᵀ)]], after the four-part binary64
//! range premise of §2.3 item 6.

use std::error::Error;
use std::fmt;

use crate::dd::{atan_unit, Dd};

pub type Vec3 = [Dd; 3];
pub type Block6 = [[Dd; 6]; 6];
pub type Block12 = [[Dd; 12]; 12];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryError {
    NonFinite,
    /// The chord has zero length.
    ZeroLength,
    /// The y reference is parallel to the chord.
    YReferenceParallel,
    /// The chord is not shorter than the bend's diameter.
    ArcAngleDomain,
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => write!(f, "a geometry input is not finite"),
            Self::ZeroLength => write!(f, "the member chord has zero length"),
            Self::YReferenceParallel => write!(f, "the y reference is parallel to the chord"),
            Self::ArcAngleDomain => write!(f, "the bend chord is not shorter than its diameter"),
        }
    }
}

impl Error for GeometryError {}

/// The operand outside the §2.3 item 6 premise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentRangeOperand {
    /// W is not a positive normal binary64 number.
    Weight,
    /// a is not a positive normal binary64 number.
    Acceleration,
    /// fl(W/a) is not normal.
    MassQuotient,
    /// fl(a·fl(W/a)) is not finite and normal.
    WeightProduct,
    /// An entry m·r_i or m·r_i·r_j of the spatial block is not finite and
    /// zero or normal.
    SpatialBlock,
    /// The centre of gravity is not finite.
    CenterOfGravity,
}

impl fmt::Display for ComponentRangeOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Weight => "W",
            Self::Acceleration => "a",
            Self::MassQuotient => "W/a",
            Self::WeightProduct => "a*(W/a)",
            Self::SpatialBlock => "spatial block m*r_i*r_j",
            Self::CenterOfGravity => "center of gravity",
        };
        write!(f, "{name}")
    }
}

/// A rigid-family component body: weight W (N), the request's acceleration
/// a (m/s²) and the centre of gravity r from the node in global axes (m).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComponentBody {
    pub node: usize,
    pub weight: f64,
    pub acceleration: f64,
    pub center_of_gravity: [f64; 3],
}

fn lift3(v: [f64; 3]) -> Result<Vec3, GeometryError> {
    if v.iter().any(|x| !x.is_finite()) {
        return Err(GeometryError::NonFinite);
    }
    Ok([Dd::from_f64(v[0]), Dd::from_f64(v[1]), Dd::from_f64(v[2])])
}
fn sub3(x: &Vec3, y: &Vec3) -> Vec3 {
    [x[0].sub(y[0]), x[1].sub(y[1]), x[2].sub(y[2])]
}
fn dot3(x: &Vec3, y: &Vec3) -> Dd {
    x[0].mul(y[0]).add(x[1].mul(y[1])).add(x[2].mul(y[2]))
}
fn cross3(x: &Vec3, y: &Vec3) -> Vec3 {
    [
        x[1].mul(y[2]).sub(x[2].mul(y[1])),
        x[2].mul(y[0]).sub(x[0].mul(y[2])),
        x[0].mul(y[1]).sub(x[1].mul(y[0])),
    ]
}
fn scale3(x: &Vec3, k: Dd) -> Vec3 {
    [x[0].mul(k), x[1].mul(k), x[2].mul(k)]
}
fn div3(x: &Vec3, k: Dd) -> Vec3 {
    [x[0].div(k), x[1].div(k), x[2].div(k)]
}
fn add3(x: &Vec3, y: &Vec3) -> Vec3 {
    [x[0].add(y[0]), x[1].add(y[1]), x[2].add(y[2])]
}

/// d = x_j − x_i, L and the unit chord d̂, and n̂, the unit part of the y
/// reference normal to d̂.
struct Chord {
    length: Dd,
    unit: Vec3,
    normal: Vec3,
}

fn chord(xi: [f64; 3], xj: [f64; 3], y_reference: [f64; 3]) -> Result<Chord, GeometryError> {
    let d = sub3(&lift3(xj)?, &lift3(xi)?);
    let length = dot3(&d, &d).sqrt();
    if length.is_zero() {
        return Err(GeometryError::ZeroLength);
    }
    let unit = div3(&d, length);
    let yr = lift3(y_reference)?;
    let projection = dot3(&yr, &unit);
    let yc = sub3(&yr, &scale3(&unit, projection));
    let ym = dot3(&yc, &yc).sqrt();
    if ym.is_zero() || !(ym.hi > 0.0) {
        return Err(GeometryError::YReferenceParallel);
    }
    Ok(Chord {
        length,
        unit,
        normal: div3(&yc, ym),
    })
}

/// Rᵀ·B·R for every 3×3 block B of a 12×12, R's rows the axes.
fn rotate12(k: &Block12, axes: &[Vec3; 3]) -> Block12 {
    let mut out = [[Dd::ZERO; 12]; 12];
    for bi in 0..4 {
        for bj in 0..4 {
            for r in 0..3 {
                for c in 0..3 {
                    let mut s = Dd::ZERO;
                    for p in 0..3 {
                        for q in 0..3 {
                            let v = k[3 * bi + p][3 * bj + q];
                            if v.is_zero() {
                                continue;
                            }
                            s = s.add(axes[p][r].mul(v).mul(axes[q][c]));
                        }
                    }
                    out[3 * bi + r][3 * bj + c] = s;
                }
            }
        }
    }
    out
}

/// The x–y plane of the consistent mass, (coefficient, power of L).
const MASS_PLANE: [[(f64, u32); 4]; 4] = [
    [(156.0, 0), (22.0, 1), (54.0, 0), (-13.0, 1)],
    [(22.0, 1), (4.0, 2), (13.0, 1), (-3.0, 2)],
    [(54.0, 0), (13.0, 1), (156.0, 0), (-22.0, 1)],
    [(-13.0, 1), (-3.0, 2), (-22.0, 1), (4.0, 2)],
];
/// The x–y plane of the Rayleigh rotary inertia.
const ROTARY_PLANE: [[(f64, u32); 4]; 4] = [
    [(36.0, 0), (3.0, 1), (-36.0, 0), (3.0, 1)],
    [(3.0, 1), (4.0, 2), (-3.0, 1), (-1.0, 2)],
    [(-36.0, 0), (-3.0, 1), (36.0, 0), (-3.0, 1)],
    [(3.0, 1), (-1.0, 2), (-3.0, 1), (4.0, 2)],
];

/// A straight member's consistent mass in global axes (module docs), with
/// m̄, ī and j̄ per length.
pub fn straight_mass(
    xi: [f64; 3],
    xj: [f64; 3],
    y_reference: [f64; 3],
    mbar: Dd,
    ibar: Dd,
    jbar: Dd,
) -> Result<Block12, GeometryError> {
    let ch = chord(xi, xj, y_reference)?;
    let ex = ch.unit;
    let ey = ch.normal;
    let ez = cross3(&ex, &ey);
    let l = ch.length;
    let l2 = l.mul(l);
    let powers = [Dd::from_f64(1.0), l, l2];
    let translation = mbar.mul(l).div(Dd::from_f64(420.0));
    let rotary = ibar.div(l.mul_f64(30.0));
    let mut plane = [[Dd::ZERO; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            let (cm, pm) = MASS_PLANE[r][c];
            let (cr, pr) = ROTARY_PLANE[r][c];
            let a = translation.mul_f64(cm).mul(powers[pm as usize]);
            let b = rotary.mul_f64(cr).mul(powers[pr as usize]);
            plane[r][c] = a.add(b);
        }
    }
    let mut k = [[Dd::ZERO; 12]; 12];
    let third = |x: Dd| x.mul(l).div(Dd::from_f64(3.0));
    let sixth = |x: Dd| x.mul(l).div(Dd::from_f64(6.0));
    for (a, b, x) in [(0usize, 6usize, mbar), (3, 9, jbar)] {
        k[a][a] = third(x);
        k[b][b] = third(x);
        k[a][b] = sixth(x);
        k[b][a] = sixth(x);
    }
    // [u_y, θ_z] in the x–y plane; [u_z, −θ_y] in the x–z plane.
    let sign = [1.0, -1.0, 1.0, -1.0];
    for r in 0..4 {
        for c in 0..4 {
            let xy = [1usize, 5, 7, 11];
            let xz = [2usize, 4, 8, 10];
            k[xy[r]][xy[c]] = plane[r][c];
            k[xz[r]][xz[c]] = plane[r][c].mul_f64(sign[r] * sign[c]);
        }
    }
    Ok(rotate12(&k, &[ex, ey, ez]))
}

/// Terms are summed until one falls below 2⁻¹¹⁰ of the running sum.
const SERIES_STOP: f64 = 7.703719777548943e-34; // 2^-110
const SERIES_MAX_TERMS: u32 = 60;

/// f₁(α) = α − sin α by its series.
fn alpha_minus_sin(alpha: Dd) -> Dd {
    let a2 = alpha.mul(alpha);
    let mut power = alpha; // α^{2k+1}/(2k+1)!
    let mut sum = Dd::ZERO;
    for k in 1..=SERIES_MAX_TERMS {
        let k = f64::from(k);
        power = power.mul(a2).div(Dd::from_f64((2.0 * k) * (2.0 * k + 1.0)));
        let term = if k as u32 % 2 == 1 { power } else { power.neg() };
        sum = sum.add(term);
        if power.hi.abs() < SERIES_STOP * sum.hi.abs() {
            return sum;
        }
    }
    unreachable!("the α − sin α series converges within its term limit for α < π")
}

/// f₃(α) = 3α/2 + sin 2α/4 − 2 sin α by its series.
fn jc22_series(alpha: Dd) -> Dd {
    let a2 = alpha.mul(alpha);
    let mut power = alpha;
    let mut sum = Dd::ZERO;
    for k in 1..=SERIES_MAX_TERMS {
        let kf = f64::from(k);
        power = power.mul(a2).div(Dd::from_f64((2.0 * kf) * (2.0 * kf + 1.0)));
        if k == 1 {
            continue; // coefficient 2¹ − 2 = 0
        }
        let coefficient = Dd::from_f64(2f64.powi(2 * k as i32 - 1)).add_f64(-2.0);
        let term = power.mul(coefficient);
        sum = sum.add(if k % 2 == 0 { term } else { term.neg() });
        if term.hi.abs() < SERIES_STOP * sum.hi.abs() {
            return sum;
        }
    }
    unreachable!("the J_c,22 series converges within its term limit for α < π/2")
}

/// The local entries of one half-arc, unscaled (module docs).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HalfArcLocal {
    pub alpha: Dd,
    pub s: [Dd; 2],
    pub jc11: Dd,
    pub jc22: Dd,
    pub jc33: Dd,
    pub jc12: Dd,
    pub tt11: Dd,
    pub tt22: Dd,
    pub tt12: Dd,
}

/// The half-arc's local entries from s = sin α and c = cos α, and α.
pub fn half_arc_local(sin: Dd, cos: Dd, alpha: Dd) -> HalfArcLocal {
    let one_plus_cos = Dd::from_f64(1.0).add(cos);
    let s2 = sin.mul(sin);
    let f1 = alpha_minus_sin(alpha);
    let f2 = alpha_minus_sin(alpha.mul_pow2(1)).mul_pow2(-2);
    HalfArcLocal {
        alpha,
        s: [f1.neg(), s2.div(one_plus_cos)],
        jc11: f2,
        jc22: jc22_series(alpha),
        jc33: f1.mul_pow2(1),
        jc12: s2.mul(s2).div(one_plus_cos.mul(one_plus_cos)).mul_pow2(-1),
        tt11: f2,
        tt22: alpha.add(sin.mul(cos)).mul_pow2(-1),
        tt12: s2.mul_pow2(-1).neg(),
    }
}

/// The 6×6 [[m·I, −[S]×], [[S]×, J]].
fn rigid_lump(mass: Dd, first: &Vec3, second: &[[Dd; 3]; 3]) -> Block6 {
    let skew = [
        [Dd::ZERO, first[2].neg(), first[1]],
        [first[2], Dd::ZERO, first[0].neg()],
        [first[1].neg(), first[0], Dd::ZERO],
    ];
    let mut out = [[Dd::ZERO; 6]; 6];
    for r in 0..3 {
        out[r][r] = mass;
        for c in 0..3 {
            out[r][3 + c] = skew[r][c].neg();
            out[3 + r][c] = skew[r][c];
            out[3 + r][3 + c] = second[r][c];
        }
    }
    out
}

/// The two half-arc lumps (node i's, then node j's), each a global 6×6
/// about its node (module docs).
#[allow(clippy::too_many_arguments)]
pub fn arc_lumps(
    xi: [f64; 3],
    xj: [f64; 3],
    radius: f64,
    y_reference: [f64; 3],
    mbar: Dd,
    ibar: Dd,
    jbar: Dd,
) -> Result<[Block6; 2], GeometryError> {
    if !(radius.is_finite() && radius > 0.0) {
        return Err(GeometryError::ArcAngleDomain);
    }
    let ch = chord(xi, xj, y_reference)?;
    let r = Dd::from_f64(radius);
    let half_chord = ch.length.mul_pow2(-1);
    let span = r.mul(r).sub(half_chord.mul(half_chord));
    if !(span.hi > 0.0) {
        return Err(GeometryError::ArcAngleDomain);
    }
    let sin = half_chord.div(r);
    let cos = span.sqrt().div(r);
    let alpha = atan_unit(sin.div(Dd::from_f64(1.0).add(cos))).mul_pow2(1);
    let local = half_arc_local(sin, cos, alpha);
    let (d, n) = (ch.unit, ch.normal);
    let frames = [
        {
            let e1 = add3(&scale3(&d, sin.neg()), &scale3(&n, cos));
            let e2 = add3(&scale3(&d, cos), &scale3(&n, sin));
            let e3 = cross3(&e1, &e2);
            [e1, e2, e3]
        },
        {
            let e1 = add3(&scale3(&d, sin), &scale3(&n, cos));
            let e2 = add3(&scale3(&d, cos.neg()), &scale3(&n, sin));
            let e3 = cross3(&e1, &e2);
            [e1, e2, e3]
        },
    ];
    let mass = mbar.mul(r).mul(alpha);
    let mr2 = mbar.mul(r).mul(r);
    let mr3 = mr2.mul(r);
    let first_local = [local.s[0].mul(mr2), local.s[1].mul(mr2), Dd::ZERO];
    let jc = [
        [local.jc11, local.jc12, Dd::ZERO],
        [local.jc12, local.jc22, Dd::ZERO],
        [Dd::ZERO, Dd::ZERO, local.jc33],
    ];
    let tt = [
        [local.tt11, local.tt12, Dd::ZERO],
        [local.tt12, local.tt22, Dd::ZERO],
        [Dd::ZERO, Dd::ZERO, Dd::ZERO],
    ];
    let mut second_local = [[Dd::ZERO; 3]; 3];
    for a in 0..3 {
        for b in 0..3 {
            let identity = if a == b { alpha } else { Dd::ZERO };
            let section = jbar
                .mul(tt[a][b])
                .add(ibar.mul(identity.sub(tt[a][b])))
                .mul(r);
            second_local[a][b] = mr3.mul(jc[a][b]).add(section);
        }
    }
    let mut lumps = [[[Dd::ZERO; 6]; 6]; 2];
    for (lump, e) in lumps.iter_mut().zip(&frames) {
        let mut first = [Dd::ZERO; 3];
        let mut second = [[Dd::ZERO; 3]; 3];
        for p in 0..3 {
            for k in 0..3 {
                first[p] = first[p].add(first_local[k].mul(e[k][p]));
            }
            for q in 0..3 {
                let mut s = Dd::ZERO;
                for a in 0..3 {
                    for b in 0..3 {
                        if second_local[a][b].is_zero() {
                            continue;
                        }
                        s = s.add(e[a][p].mul(second_local[a][b]).mul(e[b][q]));
                    }
                }
                second[p][q] = s;
            }
        }
        *lump = rigid_lump(mass, &first, &second);
    }
    Ok(lumps)
}

fn zero_or_normal(value: f64) -> bool {
    value == 0.0 || value.is_normal()
}

/// The four-part premise of design §2.3 item 6, in binary64, in order.
pub fn component_range(body: &ComponentBody) -> Result<(), ComponentRangeOperand> {
    let (w, a) = (body.weight, body.acceleration);
    if !(w.is_normal() && w > 0.0) {
        return Err(ComponentRangeOperand::Weight);
    }
    if !(a.is_normal() && a > 0.0) {
        return Err(ComponentRangeOperand::Acceleration);
    }
    let m = w / a;
    if !m.is_normal() {
        return Err(ComponentRangeOperand::MassQuotient);
    }
    if !(a * m).is_normal() {
        return Err(ComponentRangeOperand::WeightProduct);
    }
    let r = body.center_of_gravity;
    if r.iter().any(|x| !x.is_finite()) {
        return Err(ComponentRangeOperand::CenterOfGravity);
    }
    for i in 0..3 {
        let first = m * r[i];
        // A nonzero exact value that rounds to zero is out of range.
        if !zero_or_normal(first) || (first == 0.0 && r[i] != 0.0) {
            return Err(ComponentRangeOperand::SpatialBlock);
        }
        for j in 0..3 {
            let second = first * r[j];
            if !zero_or_normal(second) || (second == 0.0 && first != 0.0 && r[j] != 0.0) {
                return Err(ComponentRangeOperand::SpatialBlock);
            }
        }
    }
    for i in 0..3 {
        let (j, k) = ((i + 1) % 3, (i + 2) % 3);
        if !(m * r[j] * r[j] + m * r[k] * r[k]).is_finite() {
            return Err(ComponentRangeOperand::SpatialBlock);
        }
    }
    Ok(())
}

/// The component body's 6×6 about its node, with m = W/a in double-double,
/// after the range premise.
pub fn component_mass(body: &ComponentBody) -> Result<Block6, ComponentRangeOperand> {
    component_range(body)?;
    let m = Dd::from_f64(body.weight).div(Dd::from_f64(body.acceleration));
    let r = body.center_of_gravity.map(Dd::from_f64);
    let rr = dot3(&r, &r);
    let first = scale3(&r, m);
    let mut second = [[Dd::ZERO; 3]; 3];
    for a in 0..3 {
        for b in 0..3 {
            let identity = if a == b { rr } else { Dd::ZERO };
            second[a][b] = m.mul(identity.sub(r[a].mul(r[b])));
        }
    }
    Ok(rigid_lump(m, &first, &second))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(x: Dd, y: f64, tol: f64) {
        assert!(
            (x.to_f64() - y).abs() <= tol * y.abs(),
            "{} vs {y}",
            x.to_f64()
        );
    }

    /// The series against the closed forms where neither cancels much.
    #[test]
    fn series_match_closed_forms_at_moderate_angles() {
        for alpha in [0.75_f64, 1.0, 1.5] {
            let a = Dd::from_f64(alpha);
            close(alpha_minus_sin(a), alpha - alpha.sin(), 1e-14);
            close(
                jc22_series(a),
                1.5 * alpha + (2.0 * alpha).sin() / 4.0 - 2.0 * alpha.sin(),
                1e-13,
            );
        }
        // Leading terms at a small angle, exactly in their order.
        let a = Dd::from_f64(1e-4);
        close(alpha_minus_sin(a), 1e-12 / 6.0, 1e-8);
        close(jc22_series(a), 1e-20 / 20.0, 1e-8);
    }

    #[test]
    fn component_range_parts() {
        let body = |w: f64, a: f64, r: [f64; 3]| ComponentBody {
            node: 0,
            weight: w,
            acceleration: a,
            center_of_gravity: r,
        };
        assert_eq!(component_range(&body(2560.0, 9.8125, [0.25, 0.0, 0.0])), Ok(()));
        assert_eq!(
            component_range(&body(0.0, 9.8125, [0.0; 3])),
            Err(ComponentRangeOperand::Weight)
        );
        assert_eq!(
            component_range(&body(1.0, -1.0, [0.0; 3])),
            Err(ComponentRangeOperand::Acceleration)
        );
        // m·r rounds a nonzero product to zero.
        assert_eq!(
            component_range(&body(1e-300, 1e-8, [1e-300, 0.0, 0.0])),
            Err(ComponentRangeOperand::SpatialBlock)
        );
    }
}
