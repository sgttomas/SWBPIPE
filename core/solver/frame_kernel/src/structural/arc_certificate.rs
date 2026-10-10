//! T4-U1b (R-1): the certificate for the consistent uniform-load vector of a
//! realized arc (`curved-numerical-integrity.md`, "Certified arc loads";
//! T4-I9 DESIGN_R01 §3, confirmed by T4-RV8).
//!
//! The vector of T4-U1's objective arc (B1) is re-formed in ball arithmetic:
//! a ball (m, r) has a `Wide2` midpoint m, rounded to nearest at p = 128,
//! and a binary64 radius r ≥ 0; it holds every real x with |x − m| ≤ r. Each
//! operation's radius is the real lemma's (L1–L7 below) evaluated in binary64
//! by directed primitives: round to nearest, then one unconditional step in
//! the required direction (Lemma R: a finite round-to-nearest result is
//! within one step of the exact value on the whole grid, subnormals
//! included). Every denominator is a lower bound, every numerator, |m| and
//! constant an upper bound.
//!
//! | Lemma | Midpoint | Radius (real) | Precondition |
//! |---|---|---|---|
//! | L0 lift | exact | 0 | finite |
//! | L1 ± | rnd(m_a ± m_b) | r_a + r_b + u\|m\| | — |
//! | L2 × | rnd(m_a m_b) | \|m_a\|r_b + \|m_b\|r_a + r_a r_b + u\|m\| | — |
//! | L3 ÷ | rnd(m_a/m_b) | (r_a + \|m_a/m_b\|r_b)/(\|m_b\| − r_b) + u\|m\| | \|m_b\| > r_b |
//! | L4 √ | rnd(√m_a) | r_a(1 + u)/m + u\|m\| | m_a − r_a > 0 |
//! | L5 atan | `atan_positive` | r_t/(1 + (m_t − r_t)²) + 21.54u/(1 − 21.54u)·m | m_t − r_t > 0 |
//! | L6 2ᵏ, − | exact | r·2ᵏ, r | — |
//! | L7 inverse | Gauss–Jordan X̃ at p | ‖X̃‖∞ρ/(1 − ρ) entrywise | ρ < 1/2 |
//!
//! with u = 2⁻¹²⁸. The result for each of the 12 components is the exact
//! binary64 split of its midpoint (`Formation::Exact { scale: 1.0,
//! scaled_intended }`) and its radius (the operand bound, plus γ₅ for a
//! generated load). Any failed precondition returns `Err`, and the caller
//! keeps `Formation::CannotBound` (S11-G SF-2). No libm call is used: the
//! only transcendental is K3a's `atan_positive` with its proved bound.
//! The certificate allocates no ledger term and touches no case force.
//!
//! Every radius sum here is a chain of `add_up` calls (no binary64 fold).

use super::formation_check::CurvedFormation;
use super::retained::wide::{Wide2, WideArith, WideError};
use crate::load_ledger::gamma;
use std::cmp::Ordering;

/// Precision of the midpoints.
const PRECISION: u32 = 128;
/// u = 2⁻¹²⁸ (exact).
const U64: f64 = f64::from_bits(895_u64 << 52);
/// next_up(1) = 1 + 2⁻⁵² ≥ 1 + u.
const ONE_UP: f64 = f64::from_bits(1.0_f64.to_bits() + 1);
/// 22·2⁻¹²⁸ (exact) ≥ 21.54u/(1 − 21.54u).
const K_ATAN: f64 = f64::from_bits((899_u64 << 52) | (3_u64 << 49));
/// 2⁻¹⁰⁷⁴, the output rule's allowance for a truncated split.
const TINY: f64 = f64::from_bits(1);

/// One certified component: the split of its midpoint and its radius.
#[derive(Debug, Clone, PartialEq)]
pub struct CertifiedLoadTerm {
    /// The exact binary64 split of the ball midpoint m (at most 3 terms).
    pub intended: Vec<f64>,
    /// r̂, including the output rule's 2⁻¹⁰⁷⁴ when the split truncated.
    pub radius: f64,
    /// abs_up(m) ≥ |m|.
    pub magnitude_up: f64,
    /// The intensity had at most one nonzero component (Theorem 2).
    pub single_component: bool,
}

impl CertifiedLoadTerm {
    /// r̂, or add_up(r̂, mul_up(γ₅, add_up(magnitude_up, r̂))) for a generated
    /// load; +∞ for a generated load whose intensity has more than one
    /// nonzero component (Theorem 2).
    pub fn operand_bound(&self, generated: bool) -> f64 {
        if !generated {
            self.radius
        } else if !self.single_component {
            f64::INFINITY
        } else {
            add_up(
                self.radius,
                mul_up(gamma(5), add_up(self.magnitude_up, self.radius)),
            )
        }
    }
}

/// The 12 certified components, [node i; node j] × [F, M] in global axes,
/// and the residual bound ρ̂ of the verified inverse.
#[derive(Debug, Clone, PartialEq)]
pub struct CertifiedLoadVector {
    pub terms: [CertifiedLoadTerm; 12],
    pub rho: f64,
}

/// A failed precondition (DESIGN_R01 §3.5): the terms stay `CannotBound`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArcCertificateFailure {
    /// Class 2: a divisor ball reaches zero (L3).
    DivisorReachesZero { site: &'static str },
    /// Class 3 (and class 1's exact edge): a square-root argument ball is
    /// not positive, or its root is below the binary64 range (L4).
    SqrtArgumentNotPositive { site: &'static str },
    /// Class 4: the arctangent argument ball is not positive (L5).
    AtanArgumentNotPositive,
    /// Class 5: ρ̂ ≥ 1/2 (L7).
    ResidualNotContracting { rho: f64 },
    /// Class 8: a radius is not finite.
    RadiusNotFinite { site: &'static str },
    /// Classes 6, 7 and 9: a zero Gauss–Jordan pivot (`DivisionByZero`),
    /// any other `WideError`, or a split overflow (`SplitOverflow`).
    Wide(WideError),
}

impl From<WideError> for ArcCertificateFailure {
    fn from(error: WideError) -> Self {
        Self::Wide(error)
    }
}

type Failure = ArcCertificateFailure;

// ---------------------------------------------------------------------------
// Directed binary64 primitives (DESIGN_R01 §3.2), for a, b ≥ 0
// ---------------------------------------------------------------------------

/// ≥ a + b.
fn add_up(a: f64, b: f64) -> f64 {
    if a == 0.0 {
        b
    } else if b == 0.0 {
        a
    } else {
        (a + b).next_up()
    }
}

/// ≤ a + b.
fn add_dn(a: f64, b: f64) -> f64 {
    if a == 0.0 {
        b
    } else if b == 0.0 {
        a
    } else {
        (a + b).next_down()
    }
}

/// ≤ a − b (not clamped; a caller that needs a positive result checks it).
fn sub_dn(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        a
    } else {
        (a - b).next_down()
    }
}

/// ≥ ab.
fn mul_up(a: f64, b: f64) -> f64 {
    if a == 0.0 || b == 0.0 {
        0.0
    } else {
        (a * b).next_up()
    }
}

/// ≤ ab, and ≥ 0.
fn mul_dn(a: f64, b: f64) -> f64 {
    if a == 0.0 || b == 0.0 {
        0.0
    } else {
        (a * b).next_down().max(0.0)
    }
}

/// ≥ a/b for b > 0.
fn div_up(a: f64, b: f64) -> f64 {
    if a == 0.0 {
        0.0
    } else {
        (a / b).next_up()
    }
}

/// |t₀| ≤ |m| (the split truncates toward zero); 0 for an empty split.
fn abs_dn(m: &Wide2) -> Result<f64, Failure> {
    let split = m.split_binary64()?;
    Ok(split.terms().first().map_or(0.0, |t| t.abs()))
}

/// next_up(|t₀|) ≥ |m| for m ≠ 0 (2⁻¹⁰⁷⁴ for an empty split); 0 for m = 0.
fn abs_up(m: &Wide2) -> Result<f64, Failure> {
    if m.is_zero() {
        return Ok(0.0);
    }
    Ok(abs_dn(m)?.next_up())
}

/// u·|m|, rounded up.
fn u_term(m: &Wide2) -> Result<f64, Failure> {
    Ok(mul_up(U64, abs_up(m)?))
}

fn finite(r: f64, site: &'static str) -> Result<f64, Failure> {
    if r.is_finite() {
        Ok(r)
    } else {
        Err(Failure::RadiusNotFinite { site })
    }
}

fn is_positive(m: &Wide2) -> bool {
    m.cmp_value(&Wide2::ZERO) == Ordering::Greater
}

// ---------------------------------------------------------------------------
// Balls and the lemmas
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
struct Ball {
    m: Wide2,
    r: f64,
}

impl Ball {
    const ZERO: Self = Self {
        m: Wide2::ZERO,
        r: 0.0,
    };
    const ONE: Self = Self {
        m: Wide2::ONE,
        r: 0.0,
    };

    fn exact(m: Wide2) -> Self {
        Self { m, r: 0.0 }
    }

    /// L6: negation keeps the radius.
    fn neg(self) -> Self {
        Self {
            m: self.m.neg(),
            r: self.r,
        }
    }
}

struct Balls {
    arith: WideArith,
}

impl Balls {
    fn new() -> Result<Self, Failure> {
        Ok(Self {
            arith: WideArith::new(PRECISION)?,
        })
    }

    /// L0: the exact lift of a finite binary64 value.
    fn lift(&self, x: f64) -> Result<Ball, Failure> {
        Ok(Ball::exact(Wide2::from_f64(x)?))
    }

    /// L1.
    fn add(&mut self, a: Ball, b: Ball) -> Result<Ball, Failure> {
        let m = self.arith.add(&a.m, &b.m)?;
        let r = add_up(add_up(a.r, b.r), u_term(&m)?);
        Ok(Ball {
            m,
            r: finite(r, "L1")?,
        })
    }

    /// L1 with b negated (L6).
    fn sub(&mut self, a: Ball, b: Ball) -> Result<Ball, Failure> {
        self.add(a, b.neg())
    }

    /// L2.
    fn mul(&mut self, a: Ball, b: Ball) -> Result<Ball, Failure> {
        let m = self.arith.mul(&a.m, &b.m)?;
        let r = add_up(mul_up(abs_up(&a.m)?, b.r), mul_up(abs_up(&b.m)?, a.r));
        let r = add_up(r, mul_up(a.r, b.r));
        let r = add_up(r, u_term(&m)?);
        Ok(Ball {
            m,
            r: finite(r, "L2")?,
        })
    }

    /// L3.
    fn div(&mut self, a: Ball, b: Ball, site: &'static str) -> Result<Ball, Failure> {
        let den = if b.m.is_zero() {
            0.0
        } else {
            sub_dn(abs_dn(&b.m)?, b.r)
        };
        if !(den > 0.0) {
            return Err(Failure::DivisorReachesZero { site });
        }
        let m = self.arith.div(&a.m, &b.m)?;
        let quotient_up = mul_up(abs_up(&m)?, ONE_UP);
        let num = add_up(a.r, mul_up(quotient_up, b.r));
        let r = add_up(div_up(num, den), u_term(&m)?);
        Ok(Ball {
            m,
            r: finite(r, "L3")?,
        })
    }

    /// L4.
    fn sqrt(&mut self, a: Ball, site: &'static str) -> Result<Ball, Failure> {
        if !(is_positive(&a.m) && sub_dn(abs_dn(&a.m)?, a.r) > 0.0) {
            return Err(Failure::SqrtArgumentNotPositive { site });
        }
        let m = self.arith.sqrt(&a.m)?;
        let den = abs_dn(&m)?;
        if !(den > 0.0) {
            return Err(Failure::SqrtArgumentNotPositive { site });
        }
        let r = add_up(div_up(mul_up(a.r, ONE_UP), den), u_term(&m)?);
        Ok(Ball {
            m,
            r: finite(r, "L4")?,
        })
    }

    /// L5 with the sharpened derivative bound 1/(1 + (m_t − r_t)²) (A-6).
    fn atan(&mut self, t: Ball) -> Result<Ball, Failure> {
        let lo = if is_positive(&t.m) {
            sub_dn(abs_dn(&t.m)?, t.r)
        } else {
            0.0
        };
        if !(lo > 0.0) {
            return Err(Failure::AtanArgumentNotPositive);
        }
        let m = self.arith.atan_positive(&t.m)?;
        let den = add_dn(1.0, mul_dn(lo, lo));
        let r = add_up(div_up(t.r, den), mul_up(K_ATAN, abs_up(&m)?));
        Ok(Ball {
            m,
            r: finite(r, "L5")?,
        })
    }

    /// L6: exact scaling by 2ᵏ (k = ±1 here).
    fn pow2(&mut self, a: Ball, k: i64) -> Result<Ball, Failure> {
        let m = a.m.mul_pow2(k)?;
        let factor = if k > 0 { 2.0 } else { 0.5 };
        Ok(Ball {
            m,
            r: finite(mul_up(a.r, factor), "L6")?,
        })
    }

    fn half(&mut self, a: Ball) -> Result<Ball, Failure> {
        self.pow2(a, -1)
    }

    fn dot(&mut self, a: &[Ball; 3], b: &[Ball; 3]) -> Result<Ball, Failure> {
        let p0 = self.mul(a[0], b[0])?;
        let p1 = self.mul(a[1], b[1])?;
        let s = self.add(p0, p1)?;
        let p2 = self.mul(a[2], b[2])?;
        self.add(s, p2)
    }

    fn cross(&mut self, a: &[Ball; 3], b: &[Ball; 3]) -> Result<[Ball; 3], Failure> {
        let mut out = [Ball::ZERO; 3];
        for (i, slot) in out.iter_mut().enumerate() {
            let (j, k) = ((i + 1) % 3, (i + 2) % 3);
            let left = self.mul(a[j], b[k])?;
            let right = self.mul(a[k], b[j])?;
            *slot = self.sub(left, right)?;
        }
        Ok(out)
    }

    /// q(u, v) = Σ_r Σ_k (u_r·𝒢_rk)·v_k, accumulated left to right from 0.
    fn quad<const N: usize>(
        &mut self,
        gram: &[[Ball; N]; 3],
        left: &[Ball; 3],
        right: &[Ball; N],
    ) -> Result<Ball, Failure> {
        let mut total = Ball::ZERO;
        for r in 0..3 {
            for k in 0..N {
                let weighted = self.mul(left[r], gram[r][k])?;
                let term = self.mul(weighted, right[k])?;
                total = self.add(total, term)?;
            }
        }
        Ok(total)
    }
}

// ---------------------------------------------------------------------------
// L7: the verified inverse
// ---------------------------------------------------------------------------

/// Gauss–Jordan at p on the midpoints: pivot the first row of largest |m|
/// at or below the diagonal, divide the pivot row by the pivot, eliminate
/// every other row with a nonzero entry. A zero pivot is `DivisionByZero`.
fn gauss_jordan(arith: &mut WideArith, f: &[[Wide2; 6]; 6]) -> Result<[[Wide2; 6]; 6], Failure> {
    let mut m = *f;
    let mut inv = [[Wide2::ZERO; 6]; 6];
    for (i, row) in inv.iter_mut().enumerate() {
        row[i] = Wide2::ONE;
    }
    for col in 0..6 {
        let mut pivot = col;
        for r in col + 1..6 {
            if m[r][col].abs().cmp_value(&m[pivot][col].abs()) == Ordering::Greater {
                pivot = r;
            }
        }
        m.swap(col, pivot);
        inv.swap(col, pivot);
        let d = m[col][col];
        if d.is_zero() {
            return Err(Failure::Wide(WideError::DivisionByZero));
        }
        for c in 0..6 {
            m[col][c] = arith.div(&m[col][c], &d)?;
        }
        for c in 0..6 {
            inv[col][c] = arith.div(&inv[col][c], &d)?;
        }
        for r in 0..6 {
            if r == col || m[r][col].is_zero() {
                continue;
            }
            let factor = m[r][col];
            for c in 0..6 {
                let t = arith.mul(&factor, &m[col][c])?;
                m[r][c] = arith.sub(&m[r][c], &t)?;
            }
            for c in 0..6 {
                let t = arith.mul(&factor, &inv[col][c])?;
                inv[r][c] = arith.sub(&inv[r][c], &t)?;
            }
        }
    }
    Ok(inv)
}

/// L7: X̃ from the midpoints, R = I − F·X̃ in balls, ρ̂ its row bound; every
/// entry of F⁻¹ is within ‖X̃‖∞ρ̂/(1 − ρ̂) of X̃ when ρ̂ < 1/2.
fn invert_verified(
    balls: &mut Balls,
    f: &[[Ball; 6]; 6],
) -> Result<([[Ball; 6]; 6], f64), Failure> {
    let mut mid = [[Wide2::ZERO; 6]; 6];
    for r in 0..6 {
        for k in 0..6 {
            mid[r][k] = f[r][k].m;
        }
    }
    let xt = gauss_jordan(&mut balls.arith, &mid)?;
    let mut rho = 0.0_f64;
    for r in 0..6 {
        let mut row = 0.0;
        for k in 0..6 {
            let mut acc = if r == k { Ball::ONE } else { Ball::ZERO };
            for j in 0..6 {
                let product = balls.mul(f[r][j], Ball::exact(xt[j][k]))?;
                acc = balls.sub(acc, product)?;
            }
            row = add_up(row, add_up(abs_up(&acc.m)?, acc.r));
        }
        rho = rho.max(row);
    }
    if !(rho < 0.5) {
        return Err(Failure::ResidualNotContracting { rho });
    }
    let mut norm = 0.0_f64;
    for row_values in &xt {
        let mut row = 0.0;
        for value in row_values {
            row = add_up(row, abs_up(value)?);
        }
        norm = norm.max(row);
    }
    let radius = finite(div_up(mul_up(norm, rho), sub_dn(1.0, rho)), "L7")?;
    let mut inverse = [[Ball::ZERO; 6]; 6];
    for r in 0..6 {
        for k in 0..6 {
            inverse[r][k] = Ball {
                m: xt[r][k],
                r: radius,
            };
        }
    }
    Ok((inverse, rho))
}

// ---------------------------------------------------------------------------
// The evaluation (DESIGN_R01 §3.3, the normative order)
// ---------------------------------------------------------------------------

/// Internal actions of one unit load: in-plane moment, out-of-plane moment,
/// torsion and axial force over {1, cos θ, sin θ}.
type Actions = [[Ball; 3]; 4];

/// T4-U1b (R-1): the consistent uniform-load vector of the objective arc,
/// certified (DESIGN_R01 §3). Order [node i; node j] × [F, M] in global axes.
pub fn certify_curved_uniform_load(
    bend: &CurvedFormation,
    intensity: [f64; 3],
) -> Result<CertifiedLoadVector, ArcCertificateFailure> {
    let mut b = Balls::new()?;
    let single_component = intensity.iter().filter(|w| **w != 0.0).count() <= 1;

    // (1) d, d·d, L.
    let mut d = [Ball::ZERO; 3];
    for (k, slot) in d.iter_mut().enumerate() {
        let xj = b.lift(bend.coordinates_j[k])?;
        let xi = b.lift(bend.coordinates_i[k])?;
        *slot = b.sub(xj, xi)?;
    }
    let dd = b.dot(&d, &d)?;
    let length = b.sqrt(dd, "chord length")?;
    let radius = b.lift(bend.radius)?;

    // (2) s = (L/2)/R; 2R; q = (2R)(2R) − d·d; c_h = √q/(2R).
    let two_r = b.pow2(radius, 1)?;
    let half_length = b.half(length)?;
    let s_h = b.div(half_length, radius, "radius")?;
    let four_r2 = b.mul(two_r, two_r)?;
    let q = b.sub(four_r2, dd)?;
    let root_q = b.sqrt(q, "4R^2 - d.d")?;
    let c_h = b.div(root_q, two_r, "2R")?;

    // (3) φ, S, 1 − C, C, S₂, C₂ − 1, C₂ (stable forms).
    let t = b.div(s_h, c_h, "cos(phi/2)")?;
    let half_phi_atan = b.atan(t)?;
    let phi = b.pow2(half_phi_atan, 1)?;
    let sc = b.mul(s_h, c_h)?;
    let sin = b.pow2(sc, 1)?;
    let ss = b.mul(s_h, s_h)?;
    let one_c = b.pow2(ss, 1)?;
    let cos = b.sub(Ball::ONE, one_c)?;
    let sin_cos = b.mul(sin, cos)?;
    let sin2 = b.pow2(sin_cos, 1)?;
    let sin_sin = b.mul(sin, sin)?;
    let c2m1 = b.pow2(sin_sin, 1)?.neg();
    let cos2 = b.add(Ball::ONE, c2m1)?;

    // (4) Axes and local quantities.
    let mut dh = [Ball::ZERO; 3];
    for k in 0..3 {
        dh[k] = b.div(d[k], length, "chord length")?;
    }
    let mut y = [Ball::ZERO; 3];
    for k in 0..3 {
        y[k] = b.lift(bend.y_reference[k])?;
    }
    let projection = b.dot(&y, &dh)?;
    let mut nr = [Ball::ZERO; 3];
    for k in 0..3 {
        let along = b.mul(projection, dh[k])?;
        nr[k] = b.sub(y[k], along)?;
    }
    let nn_sq = b.dot(&nr, &nr)?;
    let nn = b.sqrt(nn_sq, "bow normal")?;
    let mut nh = [Ball::ZERO; 3];
    for k in 0..3 {
        nh[k] = b.div(nr[k], nn, "bow normal")?;
    }
    let mut ex = [Ball::ZERO; 3];
    for k in 0..3 {
        let radial_d = b.mul(s_h, dh[k])?.neg();
        let radial_n = b.mul(c_h, nh[k])?;
        ex[k] = b.add(radial_d, radial_n)?;
    }
    let ez = b.cross(&nh, &dh)?;
    let ey = b.cross(&ez, &ex)?;
    let axes = [ex, ey, ez];
    let mut w = [Ball::ZERO; 3];
    for k in 0..3 {
        w[k] = b.lift(intensity[k])?;
    }
    let mut wl = [Ball::ZERO; 3];
    let mut chord = [Ball::ZERO; 3];
    for a in 0..3 {
        wl[a] = b.dot(&axes[a], &w)?;
    }
    for a in 0..3 {
        chord[a] = b.dot(&axes[a], &d)?;
    }

    // (5) Unit-load actions, Gram and extended Gram, load actions.
    let (one, zero) = (Ball::ONE, Ball::ZERO);
    let rs = b.mul(radius, sin)?;
    let rc = b.mul(radius, cos)?;
    let z3 = [zero; 3];
    let cases: [Actions; 6] = [
        [[rs.neg(), zero, radius], z3, z3, [zero, zero, one.neg()]],
        [[rc, radius.neg(), zero], z3, z3, [zero, one, zero]],
        [z3, [zero, rs, rc.neg()], [radius, rc.neg(), rs.neg()], z3],
        [z3, [zero, one, zero], [zero, zero, one.neg()], z3],
        [z3, [zero, zero, one], [zero, one, zero], z3],
        [[one, zero, zero], z3, z3, z3],
    ];
    let half_phi = b.half(phi)?;
    let s2_half = b.half(sin2)?;
    let q_s2 = b.half(s2_half)?;
    let ss_full = b.mul(sin, sin)?;
    let half_ss = b.half(ss_full)?;
    let g11 = b.add(half_phi, q_s2)?;
    let g22 = b.sub(half_phi, q_s2)?;
    let gram = [
        [phi, sin, one_c],
        [sin, g11, half_ss],
        [one_c, half_ss, g22],
    ];
    let phi_s = b.mul(phi, sin)?;
    let phi_c = b.mul(phi, cos)?;
    let i_tc = b.sub(phi_s, one_c)?;
    let i_ts = b.sub(sin, phi_c)?;
    let phi_phi = b.mul(phi, phi)?;
    let phi2_half = b.half(phi_phi)?;
    let phi2_4 = b.half(phi2_half)?;
    let phi_s2 = b.mul(phi, sin2)?;
    let phi_s2_half = b.half(phi_s2)?;
    let phi_s2_4 = b.half(phi_s2_half)?;
    let c2m1_2 = b.half(c2m1)?;
    let c2m1_4 = b.half(c2m1_2)?;
    let c2m1_8 = b.half(c2m1_4)?;
    let tcc = b.add(phi2_4, phi_s2_4)?;
    let i_tcc = b.add(tcc, c2m1_8)?;
    let tss = b.sub(phi2_4, phi_s2_4)?;
    let i_tss = b.sub(tss, c2m1_8)?;
    let s2_2 = b.half(sin2)?;
    let s2_4 = b.half(s2_2)?;
    let s2_8 = b.half(s2_4)?;
    let phi_c2 = b.mul(phi, cos2)?;
    let phi_c2_half = b.half(phi_c2)?;
    let phi_c2_4 = b.half(phi_c2_half)?;
    let i_tsc = b.sub(s2_8, phi_c2_4)?;
    let phi_phi_e = b.mul(phi, phi)?;
    let phi2_half_e = b.half(phi_phi_e)?;
    let egram = [
        [phi, sin, one_c, phi2_half_e, i_tc, i_ts],
        [sin, g11, half_ss, i_tc, i_tcc, i_tsc],
        [one_c, half_ss, g22, i_ts, i_tsc, i_tss],
    ];
    let r2 = b.mul(radius, radius)?;
    let [wx, wy, wz] = wl;
    let a_x = [sin, phi.neg(), one.neg(), zero, one, zero];
    let a_y = [cos.neg(), one, phi.neg(), zero, zero, one];
    let mut ld_ip = [zero; 6];
    for k in 0..6 {
        let left = b.mul(wy, a_x[k])?;
        let right = b.mul(wx, a_y[k])?;
        let diff = b.sub(left, right)?;
        ld_ip[k] = b.mul(r2, diff)?;
    }
    let r2wz = b.mul(r2, wz)?;
    let r2wz_c = b.mul(r2wz, cos)?;
    let r2wz_s = b.mul(r2wz, sin)?;
    let ld_op = [r2wz, r2wz_c.neg(), r2wz_s.neg(), zero, zero, zero];
    let r2wz_phi = b.mul(r2wz, phi)?;
    let r2wz_s_t = b.mul(r2wz, sin)?;
    let r2wz_c_t = b.mul(r2wz, cos)?;
    let ld_t = [r2wz_phi, r2wz_s_t.neg(), r2wz_c_t, r2wz.neg(), zero, zero];
    let rwy = b.mul(radius, wy)?;
    let rwx = b.mul(radius, wx)?;
    let rwy_phi = b.mul(rwy, phi)?;
    let rwx_phi = b.mul(rwx, phi)?;
    let ld_ax = [zero, rwy_phi, rwx_phi.neg(), zero, rwy.neg(), rwx];
    let loads = [ld_ip, ld_op, ld_t, ld_ax];

    // (6) F (upper triangle, mirrored) and δ.
    let em = b.lift(bend.elastic_modulus)?;
    let inertia = b.lift(bend.second_moment)?;
    let gm = b.lift(bend.shear_modulus)?;
    let torsion = b.lift(bend.torsion_constant)?;
    let area = b.lift(bend.area)?;
    let ei = b.mul(em, inertia)?;
    let gj = b.mul(gm, torsion)?;
    let ea = b.mul(em, area)?;
    let k_in = b.lift(bend.in_plane_flexibility_factor)?;
    let k_out = b.lift(bend.out_of_plane_flexibility_factor)?;
    let rigidity = Rigidity {
        radius,
        k_in,
        k_out,
        ei,
        gj,
        ea,
    };
    let mut f = [[Ball::ZERO; 6]; 6];
    for r in 0..6 {
        for k in r..6 {
            let mut q4 = [zero; 4];
            for j in 0..4 {
                q4[j] = b.quad(&gram, &cases[r][j], &cases[k][j])?;
            }
            let v = rigidity.energy(&mut b, q4)?;
            f[r][k] = v;
            f[k][r] = v;
        }
    }
    let mut delta = [zero; 6];
    for r in 0..6 {
        let mut q4 = [zero; 4];
        for j in 0..4 {
            q4[j] = b.quad(&egram, &cases[r][j], &loads[j])?;
        }
        delta[r] = rigidity.energy(&mut b, q4)?;
    }

    // (7) The verified inverse.
    let (inverse, rho) = invert_verified(&mut b, &f)?;

    // (8) X = −F⁻¹δ, W, H, p_i = H X + W, p_j = −X, then global.
    let mut x = [zero; 6];
    for r in 0..6 {
        for k in 0..6 {
            let product = b.mul(inverse[r][k], delta[k])?;
            x[r] = b.sub(x[r], product)?;
        }
    }
    let s_minus_phi = b.sub(sin, phi)?;
    let arm = [b.mul(r2, s_minus_phi)?, b.mul(r2, one_c)?, zero];
    let moment = b.cross(&arm, &wl)?;
    let r_phi = b.mul(radius, phi)?;
    let mut resultant = [zero; 6];
    for a in 0..3 {
        resultant[a] = b.mul(r_phi, wl[a])?;
        resultant[3 + a] = moment[a];
    }
    let mut h = [[zero; 6]; 6];
    for (i, row) in h.iter_mut().enumerate() {
        row[i] = one;
    }
    h[3][1] = chord[2].neg();
    h[3][2] = chord[1];
    h[4][0] = chord[2];
    h[4][2] = chord[0].neg();
    h[5][0] = chord[1].neg();
    h[5][1] = chord[0];
    let mut local = [zero; 12];
    for r in 0..6 {
        let mut transferred = zero;
        for k in 0..6 {
            let product = b.mul(h[r][k], x[k])?;
            transferred = b.add(transferred, product)?;
        }
        local[r] = b.add(transferred, resultant[r])?;
        local[6 + r] = x[r].neg();
    }
    let mut global = [zero; 12];
    for block in 0..4 {
        for comp in 0..3 {
            let mut g = zero;
            for a in 0..3 {
                let product = b.mul(axes[a][comp], local[3 * block + a])?;
                g = b.add(g, product)?;
            }
            global[3 * block + comp] = g;
        }
    }

    // (9) Split each component; a truncated split adds 2⁻¹⁰⁷⁴.
    let mut terms: Vec<CertifiedLoadTerm> = Vec::with_capacity(12);
    for ball in &global {
        let split = ball.m.split_binary64()?;
        let radius = if split.truncated_below_min_subnormal() {
            add_up(ball.r, TINY)
        } else {
            ball.r
        };
        terms.push(CertifiedLoadTerm {
            intended: split.terms().to_vec(),
            radius: finite(radius, "output")?,
            magnitude_up: abs_up(&ball.m)?,
            single_component,
        });
    }
    let terms: [CertifiedLoadTerm; 12] = terms
        .try_into()
        .map_err(|_| Failure::Wide(WideError::CountRange("arc certificate terms")))?;
    Ok(CertifiedLoadVector { terms, rho })
}

/// The strain-energy weights of B1's flexibility.
struct Rigidity {
    radius: Ball,
    k_in: Ball,
    k_out: Ball,
    ei: Ball,
    gj: Ball,
    ea: Ball,
}

impl Rigidity {
    /// R·((((k_in q_in)/EI + (k_out q_out)/EI) + q_t/GJ) + q_a/EA).
    fn energy(&self, b: &mut Balls, q: [Ball; 4]) -> Result<Ball, Failure> {
        let in_plane = b.mul(self.k_in, q[0])?;
        let in_plane = b.div(in_plane, self.ei, "EI")?;
        let out_of_plane = b.mul(self.k_out, q[1])?;
        let out_of_plane = b.div(out_of_plane, self.ei, "EI")?;
        let t = b.add(in_plane, out_of_plane)?;
        let torsion = b.div(q[2], self.gj, "GJ")?;
        let t = b.add(t, torsion)?;
        let axial = b.div(q[3], self.ea, "EA")?;
        let t = b.add(t, axial)?;
        b.mul(self.radius, t)
    }
}

#[cfg(test)]
#[path = "arc_certificate_tests.rs"]
mod tests;
