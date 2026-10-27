//! K-D5 repaired recovery against the same p=128 intended stiffness and loads.
//! Matrices and row operators are re-formed on the stack. Correction products
//! enter exact sums separately: no binary64 `u+w+w2` is formed. Every published
//! scalar is rounded once from its ball midpoint; radii and third-order
//! remainders are accumulated upward. See DEL-04-01 Design §3.
use super::arc_certificate::{
    abs_up, add_up, arc_geometry, div_up, finite, mul_up, ArcCertificateFailure, Ball, Balls,
};
use super::formation_check::{
    ball_chord_axes, chord_axes, connector_matrix, curved_matrix, frame_matrix, frames_bit_equal,
    load_reformation, primitive_frame_bound, straight_primitive_frame, tie_rod_balls,
    tie_rod_matrix, CurvedFormation, FormationSource, LoadFormula, LoadPrimitive,
    MemberBoundInputs,
};
use super::retained::wide::multi::Binary64Outcome;
use super::retained::wide::{Wide2, WideArith, WideError};
use crate::connector::ObjectiveConnector;
use crate::exact_sum::{ExactAccumulator, SumError};
use crate::load_ledger::AssembledForce;
use crate::{element_dof_map, FrameElement};

pub type RecoveryFailure = ArcCertificateFailure;
const TINY: f64 = f64::from_bits(1);
const U: f64 = f64::from_bits(895_u64 << 52);

fn sum_error(_: SumError) -> RecoveryFailure {
    RecoveryFailure::Wide(WideError::CountRange("certified recovery exact sum"))
}
fn shape(site: &'static str) -> RecoveryFailure {
    RecoveryFailure::Wide(WideError::CountRange(site))
}
/// Exact power scaling, with upward rounding for underflow as well as normal values.
fn scale_up(mut x: f64, mut exponent: i32) -> f64 {
    while x != 0.0 && exponent != 0 {
        let step = exponent.clamp(-512, 512);
        x = mul_up(x, 2.0_f64.powi(step));
        exponent -= step;
    }
    x
}

/// Projects the exact fixed-grid sum once to p=128, using integer guard/sticky
/// bits. Construction by four exact 32-bit chunks introduces no new rounding.
fn sum_ball(sum: &ExactAccumulator, radius: f64) -> Result<Ball, RecoveryFailure> {
    let (negative, mag, quantum) = sum.net_parts();
    let Some(top) = mag.iter().rposition(|&v| v != 0) else {
        return Ok(Ball {
            m: Wide2::ZERO,
            r: finite(radius, "exact sum")?,
        });
    };
    let high = top * 64 + 63 - mag[top].leading_zeros() as usize;
    let drop = high.saturating_sub(127);
    let bit = |n: usize| -> bool {
        mag.get(n / 64)
            .is_some_and(|v| v & (1_u64 << (n % 64)) != 0)
    };
    let mut sig = 0_u128;
    for i in (drop..=high).rev() {
        sig = (sig << 1) | u128::from(bit(i));
    }
    let inexact = (0..drop).any(bit);
    let round = drop > 0 && bit(drop - 1) && ((0..drop - 1).any(bit) || sig & 1 != 0);
    let mut power = quantum + drop as i64;
    if round {
        match sig.checked_add(1) {
            Some(v) => sig = v,
            None => {
                sig = 1_u128 << 127;
                power += 1;
            }
        }
    }
    let mut a = WideArith::new(128)?;
    let mut m = Wide2::ZERO;
    for i in 0..4 {
        let v = ((sig >> (32 * i)) & 0xffff_ffff) as u32;
        if v != 0 {
            let chunk = Wide2::from_f64(f64::from(v))?.mul_pow2(power + 32 * i as i64)?;
            m = a.add(&m, &chunk)?;
        }
    }
    if negative {
        m = m.neg();
    }
    let r = if inexact {
        add_up(radius, mul_up(U, abs_up(&m)?))
    } else {
        radius
    };
    Ok(Ball {
        m,
        r: finite(r, "exact sum")?,
    })
}
fn add_ball(
    sum: &mut ExactAccumulator,
    radius: &mut f64,
    ball: Ball,
    negate: bool,
) -> Result<(), RecoveryFailure> {
    let split = ball.m.split_binary64()?;
    for &part in split.terms() {
        sum.add(if negate { -part } else { part })
            .map_err(sum_error)?;
    }
    *radius = add_up(*radius, ball.r);
    if split.truncated_below_min_subnormal() {
        *radius = add_up(*radius, TINY);
    }
    Ok(())
}
/// |published - m|, formed upward (zero when they are equal).
fn publication_gap(published: f64, m: Wide2) -> Result<f64, RecoveryFailure> {
    let mut a = WideArith::new(128)?;
    let gap = a.sub(&Wide2::from_f64(published)?, &m)?;
    Ok(if gap.is_zero() {
        0.0
    } else {
        add_up(abs_up(&gap)?, mul_up(U, abs_up(&gap)?))
    })
}
fn midpoint_publication(m: Wide2) -> Result<f64, RecoveryFailure> {
    // Wide's converter applies the power before rounding, including subnormals.
    // v_g = fl(m_g) (kd5-member-relative-bound.md §3.1): a nonzero midpoint
    // below half the least subnormal rounds to zero. That is a correctly
    // rounded publication, not an unavailable one; its gap |0 - m_g| enters
    // the row's rounding term of B_g. Only an overflowing rounding is
    // unrepresentable.
    let v = match m.to_binary64() {
        Binary64Outcome::Underflow { .. } => 0.0,
        outcome => outcome
            .value()
            .ok_or_else(|| shape("recovery publication"))?,
    };
    if v.is_finite() {
        Ok(if v == 0.0 { 0.0 } else { v })
    } else {
        Err(shape("recovery publication"))
    }
}

/// A p=128 value with an evaluation radius. Construction helpers expose no
/// retained-arithmetic internals to product callers.
#[derive(Debug, Clone, Copy)]
pub struct CertifiedValue {
    ball: Ball,
}
impl CertifiedValue {
    pub const ZERO: Self = Self { ball: Ball::ZERO };
    pub fn exact(value: f64) -> Result<Self, RecoveryFailure> {
        Ok(Self {
            ball: Balls::new()?.lift(value)?,
        })
    }
    pub fn published(&self) -> Result<f64, RecoveryFailure> {
        midpoint_publication(self.ball.m)
    }
    pub fn radius(&self) -> f64 {
        self.ball.r
    }
    pub fn add(self, other: Self) -> Result<Self, RecoveryFailure> {
        Ok(Self {
            ball: Balls::new()?.add(self.ball, other.ball)?,
        })
    }
    pub fn product(self, other: Self) -> Result<Self, RecoveryFailure> {
        Ok(Self {
            ball: Balls::new()?.mul(self.ball, other.ball)?,
        })
    }
    pub fn divided(self, other: Self) -> Result<Self, RecoveryFailure> {
        Ok(Self {
            ball: Balls::new()?.div(self.ball, other.ball, "certified value divisor")?,
        })
    }
}

/// One repaired scalar and the three terms of its error bound.
#[derive(Debug, Clone, Copy)]
pub struct CertifiedRow {
    value: Ball,
    pub published: f64,
    pub radius: f64,
    pub remainder: f64,
    /// Upward evaluation scale; +infinity means its magnitude exceeds the
    /// binary64 metadata range. Such a scale cannot establish notice Z.
    pub absolute_scale: f64,
    pub rounding_gap: f64,
}
impl CertifiedRow {
    pub const ZERO: Self = Self {
        value: Ball::ZERO,
        published: 0.0,
        radius: 0.0,
        remainder: 0.0,
        absolute_scale: 0.0,
        rounding_gap: 0.0,
    };
    fn finish(value: Ball, remainder: f64, absolute_scale: f64) -> Result<Self, RecoveryFailure> {
        if absolute_scale.is_nan() || absolute_scale < 0.0 {
            return Err(shape("recovery absolute scale"));
        }
        let published = midpoint_publication(value.m)?;
        Ok(Self {
            value,
            published,
            radius: value.r,
            remainder: finite(remainder, "recovery remainder")?,
            absolute_scale,
            rounding_gap: publication_gap(published, value.m)?,
        })
    }
    /// The same certificate published as `published`: the rounding term is
    /// re-formed against the ball midpoint, so |published - x| <= bound()
    /// still holds for every x the ball and remainder enclose.
    fn republished(self, published: f64) -> Result<Self, RecoveryFailure> {
        Ok(Self {
            published,
            rounding_gap: publication_gap(published, self.value.m)?,
            ..self
        })
    }
    pub fn bound(&self) -> f64 {
        add_up(add_up(self.rounding_gap, self.radius), self.remainder)
    }
    pub fn certified_magnitude_up(&self) -> Result<f64, RecoveryFailure> {
        Ok(add_up(
            add_up(abs_up(&self.value.m)?, self.radius),
            self.remainder,
        ))
    }
    pub fn value(&self) -> CertifiedValue {
        CertifiedValue { ball: self.value }
    }
    /// Derived stress is formed before the scalar publication, preserving the
    /// recovered resultant's radius and remainder through the same factor.
    pub fn scaled(self, factor: f64) -> Result<Self, RecoveryFailure> {
        let mut b = Balls::new()?;
        let value = b.mul(self.value, b.lift(factor)?)?;
        Self::finish(
            value,
            mul_up(self.remainder, factor.abs()),
            mul_up(self.absolute_scale, factor.abs()),
        )
    }
    pub fn divided(self, denominator: f64) -> Result<Self, RecoveryFailure> {
        let mut b = Balls::new()?;
        let value = b.div(self.value, b.lift(denominator)?, "stress denominator")?;
        Self::finish(
            value,
            div_up(self.remainder, denominator.abs()),
            div_up(self.absolute_scale, denominator.abs()),
        )
    }
    /// Forms factor*(self+other) at p and publishes only the final value.
    /// Scaling before the ball sum avoids an unrepresentable intermediate
    /// (for example Pa above MAX whose published MPa is finite).
    pub fn sum_scaled(self, other: Self, factor: f64) -> Result<Self, RecoveryFailure> {
        let mut b = Balls::new()?;
        let scale = b.lift(factor)?;
        let left = b.mul(self.value, scale)?;
        let right = b.mul(other.value, scale)?;
        let value = b.add(left, right)?;
        let remainder = add_up(
            mul_up(self.remainder, factor.abs()),
            mul_up(other.remainder, factor.abs()),
        );
        let absolute_scale = add_up(
            mul_up(self.absolute_scale, factor.abs()),
            mul_up(other.absolute_scale, factor.abs()),
        );
        Self::finish(value, remainder, absolute_scale)
    }
    /// Integer unit conversions divide before the final publication, including
    /// a sum whose unconverted midpoint is outside binary64.
    pub fn sum_divided(self, other: Self, denominator: f64) -> Result<Self, RecoveryFailure> {
        let mut b = Balls::new()?;
        let divisor = b.lift(denominator)?;
        let left = b.div(self.value, divisor, "scaled sum divisor")?;
        let right = b.div(other.value, divisor, "scaled sum divisor")?;
        let value = b.add(left, right)?;
        let remainder = add_up(
            div_up(self.remainder, denominator.abs()),
            div_up(other.remainder, denominator.abs()),
        );
        let absolute_scale = add_up(
            div_up(self.absolute_scale, denominator.abs()),
            div_up(other.absolute_scale, denominator.abs()),
        );
        Self::finish(value, remainder, absolute_scale)
    }
    /// Wall axial action N_eff+pAi: the pressure product is formed at p.
    pub fn add_value(self, value: CertifiedValue) -> Result<Self, RecoveryFailure> {
        let out = Balls::new()?.add(self.value, value.ball)?;
        Self::finish(
            out,
            self.remainder,
            add_up(
                self.absolute_scale,
                add_up(abs_up(&value.ball.m)?, value.ball.r),
            ),
        )
    }
    /// T4-U5b (S_joint = g_x + Σ T_r·(n_r·Q_x) − Pe on the certified route):
    /// the sum of two certified rows of the same corrected displacement. The
    /// balls add at p; the remainders and scales add upward (triangle).
    ///
    /// Precondition (the caller's): both rows come from the same certified
    /// attempt, that is, the same u and the same `MemberBoundInputs`
    /// (w, w₂, exponents, Λσ). Each remainder bounds its own row's distance
    /// from that attempt's intended system; summing rows of different
    /// attempts would certify against no one system. A row carries no
    /// attempt identity, so this is not checked here.
    pub fn add_row(self, other: Self) -> Result<Self, RecoveryFailure> {
        let out = Balls::new()?.add(self.value, other.value)?;
        Self::finish(
            out,
            add_up(self.remainder, other.remainder),
            add_up(self.absolute_scale, other.absolute_scale),
        )
    }
    /// T4-U5b: this row times a ball formed at p (a rod's n_r·Q_x): the ball
    /// product, with the remainder and scale times the factor's upper
    /// magnitude |m| + r, upward. The factor must not depend on the
    /// displacement (it carries no remainder); a row later combined with
    /// `add_row` keeps that method's same-attempt precondition.
    pub fn product_value(self, factor: CertifiedValue) -> Result<Self, RecoveryFailure> {
        let out = Balls::new()?.mul(self.value, factor.ball)?;
        let magnitude = add_up(abs_up(&factor.ball.m)?, factor.ball.r);
        Self::finish(
            out,
            mul_up(self.remainder, magnitude),
            mul_up(self.absolute_scale, magnitude),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CertifiedGroup {
    /// A nonfinite group norm is unavailable comparison metadata, even when
    /// every component is finite. The caller must fail the judgment closed.
    pub q: f64,
    pub q_hat: f64,
    pub bound: f64,
    pub absolute_scale: f64,
}
impl CertifiedGroup {
    pub fn comparison_available(&self) -> bool {
        self.q.is_finite()
            && self.q >= 0.0
            && self.q_hat.is_finite()
            && self.q_hat >= self.q
            && self.bound.is_finite()
            && self.bound >= 0.0
    }
    /// The separately published magnitude has its own scalar rounding gap
    /// and square-root evaluation radius, in addition to the vector remainder.
    pub fn magnitude(rows: &[CertifiedRow]) -> Result<Self, RecoveryFailure> {
        let magnitude = certified_magnitude(rows)?;
        certified_group(&[magnitude])
    }
}

fn norm_up(values: impl IntoIterator<Item = f64>) -> f64 {
    let mut norm = 0.0;
    for v in values {
        if v != 0.0 {
            norm = if norm == 0.0 {
                v.abs()
            } else {
                open_pipe_stress_correct_norm::norm2(norm, v).next_up()
            };
        }
    }
    norm
}
/// Norm of certified components at p, followed by one scalar publication.
pub fn certified_magnitude(rows: &[CertifiedRow]) -> Result<CertifiedRow, RecoveryFailure> {
    // The intended midpoint may have an exponent far above binary64 when
    // squared, even though the resulting norm is representable. Scale before
    // every ball product so its directed binary64 radius stays in range.
    let maximum = rows
        .iter()
        .fold(0.0_f64, |m, row| m.max(row.published.abs()));
    let exponent = if maximum == 0.0 {
        0
    } else if maximum.is_normal() {
        ((maximum.to_bits() >> 52) & 0x7ff) as i32 - 1023
    } else {
        63 - maximum.to_bits().leading_zeros() as i32 - 1074
    };
    let mut b = Balls::new()?;
    let mut sq = Ball::ZERO;
    for row in rows {
        let scaled = b.pow2(row.value, -i64::from(exponent))?;
        let part = b.mul(scaled, scaled)?;
        sq = b.add(sq, part)?;
    }
    let root = if sq.m.is_zero() && sq.r == 0.0 {
        Ball::ZERO
    } else if let Ok(root) = b.sqrt(sq, "resultant magnitude") {
        root
    } else {
        // At zero or near cancellation the absolute square-root continuity
        // bound avoids L4's positive-ball precondition.
        let mut a = WideArith::new(128)?;
        let m = a.sqrt(&sq.m)?;
        Ball {
            m,
            r: add_up(sq.r.sqrt().next_up(), mul_up(U, abs_up(&m)?)),
        }
    };
    let value = b.pow2(root, i64::from(exponent))?;
    CertifiedRow::finish(
        value,
        norm_up(rows.iter().map(|r| r.remainder)),
        norm_up(rows.iter().map(|r| r.absolute_scale)),
    )
}

pub fn certified_group(rows: &[CertifiedRow]) -> Result<CertifiedGroup, RecoveryFailure> {
    let q = match rows {
        [a] => a.published.abs(),
        [a, b] => open_pipe_stress_correct_norm::norm2(a.published, b.published),
        [a, b, c] => open_pipe_stress_correct_norm::norm3(a.published, b.published, c.published),
        [] => 0.0,
        _ => return Err(shape("recovery group size")),
    };
    let mut midpoint_norm = 0.0_f64;
    for row in rows {
        let v = abs_up(&row.value.m)?;
        if v != 0.0 {
            midpoint_norm = if midpoint_norm == 0.0 {
                v
            } else {
                open_pipe_stress_correct_norm::norm2(midpoint_norm, v).next_up()
            };
        }
    }
    let radii = norm_up(rows.iter().map(|r| r.radius));
    let remainders = norm_up(rows.iter().map(|r| r.remainder));
    let rounding = norm_up(rows.iter().map(|r| r.rounding_gap));
    let upper = add_up(add_up(midpoint_norm, radii), remainders);
    Ok(CertifiedGroup {
        q,
        q_hat: q.max(upper),
        bound: add_up(add_up(rounding, radii), remainders),
        absolute_scale: norm_up(rows.iter().map(|r| r.absolute_scale)),
    })
}

/// Global intensity re-formed from the same authored or generated primitive.
#[derive(Debug, Clone, Copy)]
pub struct CertifiedUniformLoad {
    intensity: [Ball; 3],
    pub start_fraction: f64,
    pub end_fraction: f64,
}
#[derive(Debug, Clone)]
pub struct CertifiedMemberLoads<'a> {
    equivalent: [Ball; 12],
    equivalent_scale: [f64; 12],
    primitives: &'a [LoadPrimitive],
    node_i: usize,
    node_j: usize,
    source_prefix: &'a str,
    /// T4-U8 (Design §4 item 2): the FormationSource frame of this member
    /// (`with_member_source`), which a straight recovery frame must equal.
    member_frame: MemberFrame<'a>,
}
/// Whether member loads are bound to a formation source, and the source frame
/// of their member there (`None`: the member has no frame in that source).
#[derive(Debug, Clone, Copy)]
enum MemberFrame<'a> {
    Unbound,
    Source(Option<&'a FrameElement>),
}
impl Default for CertifiedMemberLoads<'_> {
    fn default() -> Self {
        Self {
            equivalent: [Ball::ZERO; 12],
            equivalent_scale: [0.0; 12],
            primitives: &[],
            node_i: usize::MAX,
            node_j: usize::MAX,
            source_prefix: "",
            member_frame: MemberFrame::Unbound,
        }
    }
}
fn same_pair(ni: usize, nj: usize, i: usize, j: usize) -> bool {
    ni == i && nj == j
}
fn member_pair(formula: &LoadFormula) -> Option<(usize, usize)> {
    match formula {
        LoadFormula::UniformStraight { frame, .. }
        | LoadFormula::FreeStrainStraight { frame, .. } => {
            Some((frame.node_i.index, frame.node_j.index))
        }
        LoadFormula::UniformCurved { arc, .. }
        | LoadFormula::FreeStrainCurved { arc, .. }
        | LoadFormula::PressureFreeStrainCurved { arc, .. } => Some((arc.node_i, arc.node_j)),
        LoadFormula::ConnectorReference { connector } => {
            Some((connector.node_i().index, connector.node_j().index))
        }
        // Effective-force pressure nodal operands do not subtract from member
        // recovery; its mechanical wall action has its own pAi group.
        // Nonlinear sliding friction forces are nodal (kd5-nonlinear-active-set.md §3.1).
        // T4-U5b: a tie rod's reference load is its own element's, never a
        // connector member load (the connector's certified ends stay elastic).
        LoadFormula::PressureOperand { .. }
        | LoadFormula::JointEffectiveArea { .. }
        | LoadFormula::TieRodReference { .. }
        | LoadFormula::NodalProduct { .. }
        | LoadFormula::SlidingFrictionExplicit { .. }
        | LoadFormula::SlidingFrictionCoupled { .. } => None,
    }
}
impl<'a> CertifiedMemberLoads<'a> {
    pub fn from_primitives(
        node_i: usize,
        node_j: usize,
        primitives: &'a [LoadPrimitive],
    ) -> Result<Self, RecoveryFailure> {
        Self::from_member_primitives(node_i, node_j, primitives, "")
    }
    /// Member-qualified source identities prevent parallel members with the
    /// same node pair from consuming one another's primitive summands.
    pub fn from_member_primitives(
        node_i: usize,
        node_j: usize,
        primitives: &'a [LoadPrimitive],
        source_prefix: &'a str,
    ) -> Result<Self, RecoveryFailure> {
        let mut out = Self {
            primitives,
            node_i,
            node_j,
            source_prefix,
            ..Self::default()
        };
        let member = |p: &&LoadPrimitive| {
            p.identity.source.starts_with(source_prefix)
                && member_pair(&p.formula).is_some_and(|(i, j)| same_pair(node_i, node_j, i, j))
        };
        if primitives.iter().filter(member).any(|p| p.local_row >= 12) {
            return Err(shape("member primitive local row"));
        }
        // Each primitive is re-formed once and enters its element row's exact
        // sum as its own addend (Design §3.1: one exact sum of the intended
        // load terms), so cancelling primitives net exactly; the net is then
        // projected once to p, its radii and the projection added upward.
        // One accumulator is live at a time (stack only).
        for row in 0..12 {
            let mut sum = ExactAccumulator::new();
            let mut radius = 0.0;
            for primitive in primitives.iter().filter(member).filter(|p| p.local_row == row) {
                let (m, r) = load_reformation(primitive)?;
                add_ball(&mut sum, &mut radius, Ball { m: m[row], r: r[row] }, false)?;
                out.equivalent_scale[row] =
                    add_up(out.equivalent_scale[row], add_up(abs_up(&m[row])?, r[row]));
            }
            out.equivalent[row] = sum_ball(&sum, radius)?;
        }
        Ok(out)
    }
    /// T4-U8 (Design §4 item 2): binds these member loads to `source`, the
    /// formation source the case's K_int is formed from, and to `member`,
    /// their member's identity there (`FormationSource::frame_members`; the
    /// product's `member:{len}:{id}:`, the same prefix that selects its
    /// loads). A certified straight recovery requires loads bound here and a
    /// recovery frame bit-equal to that member's source frame, beam-theory
    /// leaf included; a member with no frame there, an empty identity, or a
    /// source without complete member identities fails closed.
    pub fn with_member_source(mut self, source: &'a FormationSource<'a>, member: &str) -> Self {
        self.member_frame = MemberFrame::Source(
            super::formation_check::member_frame_index(source, member).map(|k| &source.frames[k]),
        );
        self
    }
    /// Re-forms each source once on the stack; the complete scalar closure
    /// carries row 0 even when its load is zero. No recovery load cache exists.
    fn uniform_loads(
        &self,
    ) -> impl Iterator<Item = Result<CertifiedUniformLoad, RecoveryFailure>> + '_ {
        self.primitives
            .iter()
            .filter(|p| {
                p.identity.source.starts_with(self.source_prefix)
                    && p.local_row == 0
                    && member_pair(&p.formula)
                        .is_some_and(|(i, j)| same_pair(self.node_i, self.node_j, i, j))
            })
            .filter_map(|p| {
                let data = match &p.formula {
                    LoadFormula::UniformStraight {
                        intensity,
                        start_fraction,
                        end_fraction,
                        ..
                    } => Some((intensity, *start_fraction, *end_fraction)),
                    LoadFormula::UniformCurved { intensity, .. } => Some((intensity, 0.0, 1.0)),
                    _ => None,
                }?;
                Some((|| {
                    let mut intensity = super::formation_check::intensity_reformation(data.0)?;
                    for value in &mut intensity {
                        value.m = value.m.mul_pow2(i64::from(p.force_scale_exponent))?;
                        value.r = scale_up(value.r, p.force_scale_exponent);
                    }
                    Ok(CertifiedUniformLoad {
                        intensity,
                        start_fraction: data.1,
                        end_fraction: data.2,
                    })
                })())
            })
    }
}

/// The five section locations are endpoints plus the three station rows.
#[derive(Debug, Clone)]
pub struct CertifiedMemberRecovery {
    pub ends: [CertifiedRow; 12],
    pub sections: [[CertifiedRow; 6]; 5],
}
pub const CERTIFIED_SECTION_FRACTIONS: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];
/// End axes formed at p from exact binary64 authored operands. These are the
/// same half-angle operands used by PressureDirection::Tangent; no rounded
/// binary64 tangent is lifted as an exact coefficient.
fn arc_end_axes(b: &mut Balls, arc: &CurvedFormation) -> Result<[[[Ball; 3]; 3]; 2], RecoveryFailure> {
    let (axes, l) = ball_chord_axes(b, arc.coordinates_i, arc.coordinates_j, arc.y_reference)?;
    let r = b.lift(arc.radius)?;
    let diameter = b.pow2(r, 1)?;
    let s = b.div(l, diameter, "arc end tangent")?;
    let ss = b.mul(s, s)?;
    let c2 = b.sub(Ball::ONE, ss)?;
    let c = b.sqrt(c2, "arc end tangent")?;
    let mut out = [[[Ball::ZERO; 3]; 3]; 2];
    for end in 0..2 {
        for k in 0..3 {
            let cx = b.mul(c, axes[0][k])?;
            let sy = b.mul(s, axes[1][k])?;
            let sx = b.mul(s, axes[0][k])?;
            let cy = b.mul(c, axes[1][k])?;
            out[end][0][k] = if end == 0 { b.add(cx, sy)? } else { b.sub(cx, sy)? };
            out[end][1][k] = if end == 0 { b.sub(sx, cy)? } else { b.sub(sx.neg(), cy)? };
            out[end][2][k] = axes[2][k].neg();
        }
    }
    Ok(out)
}

fn tangent_end_operator_and_loads(
    matrix: &[[Wide2; 12]; 12], loads: &CertifiedMemberLoads,
    axes: &[[[Ball; 3]; 3]; 2],
) -> Result<(Operator, [Ball; 12], [f64; 12]), RecoveryFailure> {
    let mut b = Balls::new()?;
    let mut operator = [[Ball::ZERO; 12]; 12];
    let mut equivalent = [Ball::ZERO; 12];
    let mut scale = [0.0; 12];
    for block in 0..4 {
        for axis in 0..3 {
            let row = 3 * block + axis;
            for k in 0..3 {
                let coefficient = axes[block / 2][axis][k];
                for col in 0..12 {
                    let term = b.mul(coefficient, Ball::exact(matrix[3 * block + k][col]))?;
                    operator[row][col] = b.add(operator[row][col], term)?;
                }
                let term = b.mul(coefficient, loads.equivalent[3 * block + k])?;
                equivalent[row] = b.add(equivalent[row], term)?;
                scale[row] = add_up(scale[row], mul_up(ball_magnitude(coefficient)?, loads.equivalent_scale[3 * block + k]));
            }
        }
    }
    Ok((operator, equivalent, scale))
}
type Operator = [[Ball; 12]; 12];
fn local_operator(
    matrix: &[[Wide2; 12]; 12],
    axes: &[[Wide2; 3]; 3],
) -> Result<Operator, RecoveryFailure> {
    let mut b = Balls::new()?;
    let mut out = [[Ball::ZERO; 12]; 12];
    for block in 0..4 {
        for axis in 0..3 {
            for col in 0..12 {
                for c in 0..3 {
                    let product = b.mul(
                        Ball::exact(axes[axis][c]),
                        Ball::exact(matrix[3 * block + c][col]),
                    )?;
                    out[3 * block + axis][col] = b.add(out[3 * block + axis][col], product)?;
                }
            }
        }
    }
    Ok(out)
}
fn local_loads(
    loads: &CertifiedMemberLoads,
    axes: &[[Wide2; 3]; 3],
) -> Result<([Ball; 12], [f64; 12]), RecoveryFailure> {
    let mut b = Balls::new()?;
    let mut out = [Ball::ZERO; 12];
    let mut scale = [0.0; 12];
    for block in 0..4 {
        for axis in 0..3 {
            for c in 0..3 {
                let product = b.mul(Ball::exact(axes[axis][c]), loads.equivalent[3 * block + c])?;
                out[3 * block + axis] = b.add(out[3 * block + axis], product)?;
                scale[3 * block + axis] = add_up(
                    scale[3 * block + axis],
                    mul_up(
                        abs_up(&axes[axis][c])?,
                        loads.equivalent_scale[3 * block + c],
                    ),
                );
            }
        }
    }
    Ok((out, scale))
}
fn corrected_ball(u: f64, w: f64, w2: f64) -> Result<Ball, RecoveryFailure> {
    let mut sum = ExactAccumulator::new();
    for value in [u, w, w2] {
        sum.add(value).map_err(sum_error)?;
    }
    sum_ball(&sum, 0.0)
}
/// The intended load part of one recovered row, as the row's exact sum before
/// its stiffness products (Design §3.1). Every load contribution (a rotated
/// end term, a station transfer, each distributed-load term) is its own exact
/// addend, so contributions that cancel net exactly before the row's single
/// rounding; their radii add upward. `scale` is the absolute evaluation
/// scale of the load terms (a_g, §3.3).
#[derive(Debug, Clone, Default)]
struct RowLoad {
    sum: ExactAccumulator,
    radius: f64,
    scale: f64,
}
impl RowLoad {
    /// Adds `ball` (negated when `negate`) as its own exact addend.
    fn add(&mut self, ball: Ball, negate: bool) -> Result<(), RecoveryFailure> {
        add_ball(&mut self.sum, &mut self.radius, ball, negate)
    }
    /// One load ball subtracted from the row (row = A x - load).
    fn single(load: Ball, scale: f64) -> Result<Self, RecoveryFailure> {
        let mut out = Self {
            scale,
            ..Self::default()
        };
        out.add(load, true)?;
        Ok(out)
    }
}
fn evaluate_row<const N: usize>(
    operator: &[Ball; N],
    map: &[usize; N],
    load: Ball,
    load_scale: f64,
    u: &[f64],
    bound: &MemberBoundInputs,
) -> Result<CertifiedRow, RecoveryFailure> {
    evaluate_row_inner(operator, map, RowLoad::single(load, load_scale)?, u, bound, true)
}
fn evaluate_row_inner<const N: usize>(
    operator: &[Ball; N],
    map: &[usize; N],
    load: RowLoad,
    u: &[f64],
    bound: &MemberBoundInputs,
    include_remainder: bool,
) -> Result<CertifiedRow, RecoveryFailure> {
    // The row's exact sum starts from its load addends (moved, not copied).
    let RowLoad {
        mut sum,
        radius: load_radius,
        scale: load_scale,
    } = load;
    let mut radius = 0.0;
    let mut scale = 0.0;
    let mut norm = 0.0;
    for c in 0..N {
        let i = map[c];
        if i >= u.len() || i >= bound.w.len() || i >= bound.w2.len() || i >= bound.exponents.len() {
            return Err(shape("recovery DOF"));
        }
        let x = corrected_ball(u[i], bound.w[i], bound.w2[i])?;
        let xu = add_up(abs_up(&x.m)?, x.r);
        let coefficient = operator[c];
        // Exact split products of each correction separately; below-grid bits
        // and coefficient ball uncertainty are radii, never midpoint shifts.
        for value in [u[i], bound.w[i], bound.w2[i]] {
            if coefficient.m.add_product_to(&mut sum, value)? {
                radius = add_up(radius, mul_up(TINY, value.abs()));
            }
        }
        radius = add_up(radius, mul_up(coefficient.r, xu));
        let ku = add_up(abs_up(&coefficient.m)?, coefficient.r);
        scale = add_up(scale, mul_up(ku, xu));
        if include_remainder && bound.exponents[i] != i32::MIN {
            norm = add_up(norm, scale_up(ku, bound.exponents[i]));
        }
    }
    radius = add_up(radius, load_radius);
    scale = add_up(scale, load_scale);
    CertifiedRow::finish(
        sum_ball(&sum, radius)?,
        if include_remainder {
            mul_up(norm, bound.lambda_sigma)
        } else {
            0.0
        },
        scale,
    )
}
fn evaluate_ends(
    operator: &Operator,
    map: &[usize; 12],
    loads: &[Ball; 12],
    load_scales: &[f64; 12],
    u: &[f64],
    bound: &MemberBoundInputs,
) -> Result<[CertifiedRow; 12], RecoveryFailure> {
    let mut out = [CertifiedRow::ZERO; 12];
    for r in 0..12 {
        out[r] = evaluate_row(&operator[r], map, loads[r], load_scales[r], u, bound)?;
    }
    Ok(out)
}

struct StraightRecoveryContext {
    axes: [[Wide2; 3]; 3],
    length: Wide2,
    operator: Operator,
    equivalent: [Ball; 12],
    equivalent_scale: [f64; 12],
    map: [usize; 12],
}
/// T4-U8 (Design §4 item 2): the recovery frame of a straight member is
/// bit-equal, its beam-theory leaf included, to the FormationSource frame of
/// the same member (`CertifiedMemberLoads::with_member_source`), whether or
/// not the member has a load primitive, and to the frame of every straight
/// primitive of that member (the member's source prefix and node pair, as
/// `CertifiedMemberLoads` selects them; no force scaling, since recovery
/// takes the physical frame and authored leaves). Otherwise the recovery
/// fails closed: K_int·u with one element and an operator or loads re-formed
/// for another would certify a mixed system. Loads not bound to a source, and
/// a member with no source frame, fail closed.
fn straight_member_frames_bound(
    frame: &FrameElement,
    loads: &CertifiedMemberLoads,
) -> Result<(), RecoveryFailure> {
    match loads.member_frame {
        MemberFrame::Source(Some(source_frame)) if frames_bit_equal(source_frame, frame) => {}
        _ => return Err(shape("straight recovery frame is not its member's formation-source frame")),
    }
    let unbound = loads.primitives.iter().any(|p| {
        p.identity.source.starts_with(loads.source_prefix)
            && member_pair(&p.formula)
                .is_some_and(|(i, j)| same_pair(loads.node_i, loads.node_j, i, j))
            && straight_primitive_frame(&p.formula).is_some_and(|f| !frames_bit_equal(f, frame))
    });
    if unbound {
        return Err(shape("straight recovery frame is not its member primitives' frame"));
    }
    Ok(())
}
fn straight_context(
    frame: &FrameElement,
    loads: &CertifiedMemberLoads,
) -> Result<StraightRecoveryContext, RecoveryFailure> {
    straight_member_frames_bound(frame, loads)?;
    let mut a = WideArith::new(128)?;
    let matrix = frame_matrix(&mut a, frame)?;
    let (axes, length) = chord_axes(
        &mut a,
        frame.node_i.coordinates,
        frame.node_j.coordinates,
        frame.y_reference,
    )?;
    let operator = local_operator(&matrix, &axes)?;
    let (equivalent, equivalent_scale) = local_loads(loads, &axes)?;
    let map = element_dof_map(frame.node_i.index, frame.node_j.index);
    Ok(StraightRecoveryContext {
        axes,
        length,
        operator,
        equivalent,
        equivalent_scale,
        map,
    })
}
fn straight_section_rows(
    ctx: &StraightRecoveryContext,
    loads: &CertifiedMemberLoads,
    u: &[f64],
    bound: &MemberBoundInputs,
    fraction: Ball,
    include_remainder: bool,
) -> Result<[CertifiedRow; 6], RecoveryFailure> {
    if fraction.m.cmp_value(&Wide2::ZERO).is_lt() || fraction.m.cmp_value(&Wide2::ONE).is_gt() {
        return Err(shape("straight section fraction"));
    }
    let mut b = Balls::new()?;
    let operator = &ctx.operator;
    let equivalent = &ctx.equivalent;
    let equivalent_scale = &ctx.equivalent_scale;
    let axes = &ctx.axes;
    let length = ctx.length;
    let map = ctx.map;
    let distance = b.mul(Ball::exact(length), fraction)?;
    let mut op = [[Ball::ZERO; 12]; 6];
    for r in 0..6 {
        for c in 0..12 {
            op[r][c] = operator[r][c].neg();
        }
    }
    for (moment, shear, sign) in [(4, 2, -1.0), (5, 1, 1.0)] {
        for c in 0..12 {
            let term = b.mul(distance, operator[shear][c])?;
            op[moment][c] = if sign < 0.0 {
                b.sub(op[moment][c], term)?
            } else {
                b.add(op[moment][c], term)?
            };
        }
    }
    // Section row r = op_r x - eq_r. Its load part -eq_r is the row's exact
    // sum of separate addends (`RowLoad`): the i-end term, the station
    // transfer of the end shear (moments) and each uniform primitive's term.
    // Rows are formed one at a time, so one load sum is live (the stack
    // census of `certified_stress_control_sizes` is unchanged); each row
    // re-forms the member's uniform intensities on the stack.
    let mut out = [CertifiedRow::ZERO; 6];
    for r in 0..6 {
        let mut load = RowLoad::default();
        load.add(equivalent[r], false)?;
        load.scale = equivalent_scale[r];
        let transfer = match r {
            4 => Some((2, -1.0)),
            5 => Some((1, 1.0)),
            _ => None,
        };
        if let Some((shear, sign)) = transfer {
            load.scale = add_up(
                load.scale,
                mul_up(
                    add_up(abs_up(&distance.m)?, distance.r),
                    equivalent_scale[shear],
                ),
            );
            // eq_moment = -end_moment + sign * d * end_shear.
            let term = b.mul(distance, equivalent[shear])?;
            load.add(term, sign > 0.0)?;
        }
        if r != 3 {
            for uniform in loads.uniform_loads() {
                let uniform = uniform?;
                let end_value = b.lift(uniform.end_fraction)?;
                let end = if fraction.m.cmp_value(&end_value.m).is_gt() {
                    end_value
                } else {
                    fraction
                };
                if !end.m.cmp_value(&b.lift(uniform.start_fraction)?.m).is_gt() {
                    continue;
                }
                let start = b.mul(Ball::exact(length), b.lift(uniform.start_fraction)?)?;
                let active_end = b.mul(Ball::exact(length), end)?;
                let active = b.sub(active_end, start)?;
                // eq_r += w_r * active (r < 3), eq_My += w_z * lever,
                // eq_Mz -= w_y * lever (w in the chord frame).
                let (axis, factor, negate) = if r < 3 {
                    (r, active, true)
                } else {
                    let ea = b.mul(active_end, active_end)?;
                    let sa = b.mul(start, start)?;
                    let diff = b.sub(ea, sa)?;
                    let half = b.half(diff)?;
                    let da = b.mul(distance, active)?;
                    let lever = b.sub(da, half)?;
                    if r == 4 {
                        (2, lever, true)
                    } else {
                        (1, lever, false)
                    }
                };
                let mut intensity = Ball::ZERO;
                for c in 0..3 {
                    let term = b.mul(Ball::exact(axes[axis][c]), uniform.intensity[c])?;
                    intensity = b.add(intensity, term)?;
                }
                let term = b.mul(intensity, factor)?;
                load.add(term, negate)?;
                load.scale = add_up(load.scale, add_up(abs_up(&term.m)?, term.r));
            }
        }
        out[r] = evaluate_row_inner(&op[r], &map, load, u, bound, include_remainder)?;
    }
    Ok(out)
}
pub fn certify_straight_recovery(
    frame: &FrameElement,
    loads: &CertifiedMemberLoads,
    u: &[f64],
    bound: &MemberBoundInputs,
) -> Result<CertifiedMemberRecovery, RecoveryFailure> {
    let ctx = straight_context(frame, loads)?;
    let ends = evaluate_ends(
        &ctx.operator,
        &ctx.map,
        &ctx.equivalent,
        &ctx.equivalent_scale,
        u,
        bound,
    )?;
    let mut sections = [[CertifiedRow::ZERO; 6]; 5];
    let b = Balls::new()?;
    for (s, &fraction) in CERTIFIED_SECTION_FRACTIONS.iter().enumerate() {
        sections[s] = straight_section_rows(&ctx, loads, u, bound, b.lift(fraction)?, true)?;
    }
    Ok(CertifiedMemberRecovery { ends, sections })
}
/// An arbitrary section uses the identical intended row operator/load recipe
/// as the fixed five locations. Uniform-span boundaries have continuous action
/// values; each adjacent interval retains its own derivative polynomial.
pub fn certify_straight_section(
    frame: &FrameElement,
    loads: &CertifiedMemberLoads,
    u: &[f64],
    bound: &MemberBoundInputs,
    fraction: f64,
) -> Result<[CertifiedRow; 6], RecoveryFailure> {
    let ctx = straight_context(frame, loads)?;
    straight_section_rows(&ctx, loads, u, bound, Balls::new()?.lift(fraction)?, true)
}

/// Pressure pAi from the authored annulus, at the same p as the load closure.
pub fn certified_pressure_area_product(
    pressure: f64,
    outer_diameter: f64,
    wall_thickness: f64,
) -> Result<CertifiedValue, RecoveryFailure> {
    let mut b = Balls::new()?;
    let pi4 = b.atan(Ball::ONE)?;
    let pi2 = b.pow2(pi4, 1)?;
    let pi = b.pow2(pi2, 1)?;
    let od = b.lift(outer_diameter)?;
    let ro = b.half(od)?;
    let t = b.lift(wall_thickness)?;
    let ri = b.sub(ro, t)?;
    let ri2 = b.mul(ri, ri)?;
    let ai = b.mul(pi, ri2)?;
    let p = b.lift(pressure)?;
    Ok(CertifiedValue {
        ball: b.mul(p, ai)?,
    })
}
impl CertifiedValue {
    pub fn scaled(self, factor: f64) -> Result<Self, RecoveryFailure> {
        self.product(Self::exact(factor)?)
    }
    pub fn difference(self, other: Self) -> Result<Self, RecoveryFailure> {
        Ok(Self {
            ball: Balls::new()?.sub(self.ball, other.ball)?,
        })
    }
}

fn fraction_trig(
    b: &mut Balls,
    g: &super::arc_certificate::ArcGeometry,
    fraction: f64,
) -> Result<(Ball, Ball), RecoveryFailure> {
    if fraction == 0.0 {
        return Ok((Ball::ZERO, Ball::ONE));
    }
    if fraction == 1.0 {
        return Ok((g.sin, g.cos));
    }
    let one_plus = b.add(Ball::ONE, g.cos)?;
    let half_plus = b.half(one_plus)?;
    let ch = b.sqrt(half_plus, "arc half cosine")?;
    let half_minus = b.half(g.one_c)?;
    let sh = b.sqrt(half_minus, "arc half sine")?;
    if fraction == 0.5 {
        return Ok((sh, ch));
    }
    let one_plus = b.add(Ball::ONE, ch)?;
    let plus = b.half(one_plus)?;
    let cq = b.sqrt(plus, "arc quarter cosine")?;
    // sin(phi/4)=sin(phi/2)/(2cos(phi/4)) avoids small-angle subtraction.
    let twice = b.pow2(cq, 1)?;
    let sq = b.div(sh, twice, "arc quarter cosine")?;
    if fraction == 0.25 {
        return Ok((sq, cq));
    }
    if fraction == 0.75 {
        let a = b.mul(sh, cq)?;
        let c = b.mul(ch, sq)?;
        let sin = b.add(a, c)?;
        let a = b.mul(ch, cq)?;
        let c = b.mul(sh, sq)?;
        return Ok((sin, b.sub(a, c)?));
    }
    Err(shape("arc certificate station"))
}
fn ball_magnitude(value: Ball) -> Result<f64, RecoveryFailure> {
    Ok(add_up(abs_up(&value.m)?, value.r))
}
fn cross_operators(
    b: &mut Balls,
    arm: &[Ball; 3],
    force: &[[Ball; 12]; 3],
) -> Result<[[Ball; 12]; 3], RecoveryFailure> {
    let mut out = [[Ball::ZERO; 12]; 3];
    for r in 0..3 {
        let j = (r + 1) % 3;
        let k = (r + 2) % 3;
        for c in 0..12 {
            let left = b.mul(arm[j], force[k][c])?;
            let right = b.mul(arm[k], force[j][c])?;
            out[r][c] = b.sub(left, right)?;
        }
    }
    Ok(out)
}

/// Arc end rows use K_int,arc from the formation residual, rather than the
/// independently inverted real-arc stiffness of the former O-13 certificate.
/// Section transfers retain O-13's ball geometry and exact equilibrium recipe.
pub fn certify_arc_recovery(
    arc: &CurvedFormation,
    loads: &CertifiedMemberLoads,
    u: &[f64],
    bound: &MemberBoundInputs,
) -> Result<CertifiedMemberRecovery, RecoveryFailure> {
    let mut a = WideArith::new(128)?;
    let matrix = curved_matrix(&mut a, arc)?;
    let mut b = Balls::new()?;
    let axes = arc_end_axes(&mut b, arc)?;
    let (operator, equivalent, es) = tangent_end_operator_and_loads(&matrix, loads, &axes)?;
    let map = element_dof_map(arc.node_i, arc.node_j);
    let ends = evaluate_ends(&operator, &map, &equivalent, &es, u, bound)?;
    let mut b = Balls::new()?;
    let geometry = arc_geometry(&mut b, arc)?;
    let mut tip = [[Ball::ZERO; 12]; 6];
    let mut teq = [Ball::ZERO; 6];
    let mut tes = [0.0; 6];
    for block in 0..2 {
        for r in 0..3 {
            for k in 0..3 {
                let axis = geometry.axes[r][k];
                for c in 0..12 {
                    let term = b.mul(axis, Ball::exact(matrix[6 + 3 * block + k][c]))?;
                    tip[3 * block + r][c] = b.add(tip[3 * block + r][c], term)?;
                }
                let term = b.mul(axis, loads.equivalent[6 + 3 * block + k])?;
                teq[3 * block + r] = b.add(teq[3 * block + r], term)?;
                tes[3 * block + r] = add_up(
                    tes[3 * block + r],
                    mul_up(
                        ball_magnitude(axis)?,
                        loads.equivalent_scale[6 + 3 * block + k],
                    ),
                );
            }
        }
    }
    let mut sections = [[CertifiedRow::ZERO; 6]; 5];
    for (s, &fraction) in CERTIFIED_SECTION_FRACTIONS.iter().enumerate() {
        let (sin, cos) = fraction_trig(&mut b, &geometry, fraction)?;
        let theta = b.mul(geometry.phi, b.lift(fraction)?)?;
        let remaining = b.sub(geometry.phi, theta)?;
        let dc = b.sub(geometry.cos, cos)?;
        let ds = b.sub(geometry.sin, sin)?;
        let arm = [
            b.mul(geometry.radius, dc)?,
            b.mul(geometry.radius, ds)?,
            Ball::ZERO,
        ];
        let force = [tip[0], tip[1], tip[2]];
        let transfer = cross_operators(&mut b, &arm, &force)?;
        let eqforce = [teq[0], teq[1], teq[2]];
        let eqtransfer = b.cross(&arm, &eqforce)?;
        let mut op = tip;
        let mut eqscale = tes;
        for r in 0..3 {
            for c in 0..12 {
                op[3 + r][c] = b.add(op[3 + r][c], transfer[r][c])?;
            }
            let j = (r + 1) % 3;
            let k = (r + 2) % 3;
            eqscale[3 + r] = add_up(
                eqscale[3 + r],
                add_up(
                    mul_up(ball_magnitude(arm[j])?, tes[k]),
                    mul_up(ball_magnitude(arm[k])?, tes[j]),
                ),
            );
        }
        let axes = [
            [sin.neg(), cos, Ball::ZERO],
            [cos.neg(), sin.neg(), Ball::ZERO],
            [Ball::ZERO, Ball::ZERO, Ball::ONE],
        ];
        // Section row (block, r) = A x - sum_k axes[r][k] * eq[3 block + k],
        // with eq = tip end loads + (moments) their station transfer - each
        // uniform primitive's term. Its load part is the row's exact sum of
        // separate addends (`RowLoad`): the rotated tip term, the rotated
        // transfer and each primitive's rotated term.
        let rotated = |b: &mut Balls, r: usize, v: [Ball; 3]| -> Result<Ball, RecoveryFailure> {
            let mut out = Ball::ZERO;
            for k in 0..3 {
                let term = b.mul(axes[r][k], v[k])?;
                out = b.add(out, term)?;
            }
            Ok(out)
        };
        let mut row_loads: [RowLoad; 6] = Default::default();
        for block in 0..2 {
            for r in 0..3 {
                let tip_term = rotated(&mut b, r, [teq[3 * block], teq[3 * block + 1], teq[3 * block + 2]])?;
                row_loads[3 * block + r].add(tip_term, true)?;
                if block == 1 {
                    let transfer_term = rotated(&mut b, r, eqtransfer)?;
                    row_loads[3 + r].add(transfer_term, true)?;
                }
            }
        }
        let r2 = b.mul(geometry.radius, geometry.radius)?;
        let rc = b.mul(remaining, cos)?;
        let rs = b.mul(remaining, sin)?;
        let distx = b.sub(ds, rc)?;
        let disty = b.sub(dc.neg(), rs)?;
        let distarm = [b.mul(r2, distx)?, b.mul(r2, disty)?, Ball::ZERO];
        let length = b.mul(geometry.radius, remaining)?;
        for load in loads.uniform_loads() {
            let load = load?;
            let mut wl = [Ball::ZERO; 3];
            for r in 0..3 {
                wl[r] = b.dot(&geometry.axes[r], &load.intensity)?;
            }
            let moment = b.cross(&distarm, &wl)?;
            let mut force = [Ball::ZERO; 3];
            for r in 0..3 {
                force[r] = b.mul(length, wl[r])?;
                eqscale[r] = add_up(eqscale[r], ball_magnitude(force[r])?);
                eqscale[3 + r] = add_up(eqscale[3 + r], ball_magnitude(moment[r])?);
            }
            // eq -= term, so the row's load part gains + axes * term.
            for r in 0..3 {
                let force_term = rotated(&mut b, r, force)?;
                row_loads[r].add(force_term, false)?;
                let moment_term = rotated(&mut b, r, moment)?;
                row_loads[3 + r].add(moment_term, false)?;
            }
        }
        for (index, mut load) in row_loads.into_iter().enumerate() {
            let (block, r) = (index / 3, index % 3);
            let mut row = [Ball::ZERO; 12];
            for k in 0..3 {
                for c in 0..12 {
                    let term = b.mul(axes[r][k], op[3 * block + k][c])?;
                    row[c] = b.add(row[c], term)?;
                }
                load.scale = add_up(
                    load.scale,
                    mul_up(ball_magnitude(axes[r][k])?, eqscale[3 * block + k]),
                );
            }
            sections[s][index] = evaluate_row_inner(&row, &map, load, u, bound, true)?;
        }
    }
    Ok(CertifiedMemberRecovery { ends, sections })
}

#[derive(Debug, Clone)]
pub struct CertifiedConnectorRecovery {
    pub ends: [CertifiedRow; 12],
    pub q: [CertifiedRow; 6],
    pub deformation: [CertifiedRow; 6],
    pub g: [CertifiedRow; 6],
}
fn connector_b(connector: &ObjectiveConnector) -> Result<[[Ball; 12]; 6], RecoveryFailure> {
    let mut b = Balls::new()?;
    let ni = connector.node_i();
    let nj = connector.node_j();
    let (ai, aj) = connector.offsets();
    let mut arm = [Ball::ZERO; 3];
    for k in 0..3 {
        let d = b.sub(b.lift(nj.coordinates[k])?, b.lift(ni.coordinates[k])?)?;
        let o = b.sub(b.lift(aj[k])?, b.lift(ai[k])?)?;
        let combined = b.add(d, o)?;
        arm[k] = b.half(combined)?;
    }
    let skew = |v: [Ball; 3]| {
        [
            [Ball::ZERO, v[2].neg(), v[1]],
            [v[2], Ball::ZERO, v[0].neg()],
            [v[1].neg(), v[0], Ball::ZERO],
        ]
    };
    let sai = skew([b.lift(ai[0])?, b.lift(ai[1])?, b.lift(ai[2])?]);
    let saj = skew([b.lift(aj[0])?, b.lift(aj[1])?, b.lift(aj[2])?]);
    let sr = skew(arm);
    let axes = connector.axes();
    let mut out = [[Ball::ZERO; 12]; 6];
    for axis in 0..3 {
        for c in 0..3 {
            let q = b.lift(axes[c][axis])?;
            out[axis][c] = q.neg();
            out[axis][6 + c] = q;
            out[3 + axis][3 + c] = q.neg();
            out[3 + axis][9 + c] = q;
            for k in 0..3 {
                let mi = b.add(sai[k][c], sr[k][c])?;
                let mj = b.sub(sr[k][c], saj[k][c])?;
                let q = b.lift(axes[k][axis])?;
                let term = b.mul(q, mi)?;
                out[axis][3 + c] = b.add(out[axis][3 + c], term)?;
                let term = b.mul(q, mj)?;
                out[axis][9 + c] = b.add(out[axis][9 + c], term)?;
            }
        }
    }
    Ok(out)
}
pub fn certify_connector_recovery(
    connector: &ObjectiveConnector,
    loads: &CertifiedMemberLoads,
    u: &[f64],
    bound: &MemberBoundInputs,
) -> Result<CertifiedConnectorRecovery, RecoveryFailure> {
    let mut a = WideArith::new(128)?;
    let matrix = connector_matrix(&mut a, connector)?;
    let operator = matrix.map(|row| row.map(Ball::exact));
    let map = element_dof_map(connector.node_i().index, connector.node_j().index);
    let ends = evaluate_ends(
        &operator,
        &map,
        &loads.equivalent,
        &loads.equivalent_scale,
        u,
        bound,
    )?;
    let bop = connector_b(connector)?;
    let reference = connector.q_ref();
    let stiffness = connector.stiffness();
    let mut b = Balls::new()?;
    let mut q = [CertifiedRow::ZERO; 6];
    let mut g = [CertifiedRow::ZERO; 6];
    let mut deformation = [CertifiedRow::ZERO; 6];
    for r in 0..6 {
        q[r] = evaluate_row(&bop[r], &map, Ball::ZERO, 0.0, u, bound)?;
        deformation[r] = evaluate_row(
            &bop[r],
            &map,
            b.lift(reference[r])?,
            reference[r].abs(),
            u,
            bound,
        )?;
        let mut op = [Ball::ZERO; 12];
        let mut load = Ball::ZERO;
        let mut ls = 0.0;
        for k in 0..6 {
            let kr = b.lift(stiffness[r][k])?;
            for c in 0..12 {
                let term = b.mul(kr, bop[k][c])?;
                op[c] = b.add(op[c], term)?;
            }
            let term = b.mul(kr, b.lift(reference[k])?)?;
            load = b.add(load, term)?;
            ls = add_up(ls, ball_magnitude(term)?);
        }
        g[r] = evaluate_row(&op, &map, load, ls, u, bound)?;
    }
    Ok(CertifiedConnectorRecovery {
        ends,
        q,
        deformation,
        g,
    })
}

/// T4-U5b: one tie rod's certified recovery at the corrected displacement
/// u + w + w₂ (DEL-04-07 Design `u4b-u5b-implementation.md` §3.2).
#[derive(Debug, Clone)]
pub struct CertifiedTieRodRecovery {
    /// e = C·(u + w + w₂).
    pub extension: CertifiedRow,
    /// T_r = k_r(C·(u + w + w₂) − e_ref), tension positive.
    pub tension: CertifiedRow,
    /// n = d/l, formed at p from the authored operands (balls).
    pub direction: [CertifiedValue; 3],
    /// n·Q_x at p (the rod's share of S_joint's projected term).
    pub axial_share: CertifiedValue,
}
/// T4-U5b: tie rod `rod` of `connector` as certified rows of the corrected
/// displacement. C, n and l are formed in ball arithmetic from the rod's
/// authored operands (node coordinates, Q, offsets in Q), k_r and e_ref are
/// exact lifts, and each row is one exact sum of the three corrections'
/// products (`evaluate_row`): its radius comes from C's and k_r·C's balls
/// through the displacement ball, and its remainder is ‖A·D‖₁·Λσ.
///
/// The connector is the physical one (on W2 as well: the attempt's D′ and
/// Λ′σ′ are carried by `bound`, kd5-member-relative-bound.md §2.2), so the
/// rows need no unscaling. A rod index outside the connector's rods fails.
pub fn certify_tie_rod_recovery(
    connector: &ObjectiveConnector,
    rod: usize,
    u: &[f64],
    bound: &MemberBoundInputs,
) -> Result<CertifiedTieRodRecovery, RecoveryFailure> {
    let rod = connector
        .tie_rods()
        .get(rod)
        .ok_or_else(|| shape("tie-rod recovery: rod outside the connector's rods"))?;
    let mut b = Balls::new()?;
    let geometry = tie_rod_balls(&mut b, rod)?;
    let map = element_dof_map(rod.node_i().index, rod.node_j().index);
    let extension = evaluate_row(&geometry.c, &map, Ball::ZERO, 0.0, u, bound)?;
    let k = b.lift(rod.stiffness())?;
    let mut operator = [Ball::ZERO; 12];
    for (slot, c) in operator.iter_mut().zip(geometry.c) {
        *slot = b.mul(k, c)?;
    }
    let load = b.mul(k, b.lift(rod.reference_extension())?)?;
    let load_scale = add_up(abs_up(&load.m)?, load.r);
    let tension = evaluate_row(&operator, &map, load, load_scale, u, bound)?;
    let axes = rod.axes();
    let axis = [b.lift(axes[0][0])?, b.lift(axes[1][0])?, b.lift(axes[2][0])?];
    let axial_share = b.dot(&geometry.direction, &axis)?;
    Ok(CertifiedTieRodRecovery {
        extension,
        tension,
        direction: geometry.direction.map(|ball| CertifiedValue { ball }),
        axial_share: CertifiedValue { ball: axial_share },
    })
}

/// Restrained reaction K_int,r*corrected_u - f_int,r. The caller supplies the
/// same unscaled physical ledger/source used for member recovery. Element
/// blocks are re-formed, only for elements adjacent to the reaction DOF, and
/// discarded one at a time, never heap-cached.
/// The remainder uses the upward triangle upper bound obtained by summing
/// each contribution's free-column ||A_e D||1; prescribed columns are absent.
pub fn certify_support_reaction(
    source: &FormationSource<'_>,
    force: &AssembledForce,
    dof: usize,
    u: &[f64],
    bound: &MemberBoundInputs,
) -> Result<CertifiedRow, RecoveryFailure> {
    if dof >= u.len() {
        return Err(shape("support reaction DOF"));
    }
    let mut sum = ExactAccumulator::new();
    let mut radius = 0.0;
    let mut scale = 0.0;
    let mut remainder = 0.0;
    let mut a = WideArith::new(128)?;
    let mut apply =
        |ni: usize, nj: usize, matrix: &[[Wide2; 12]; 12]| -> Result<(), RecoveryFailure> {
            let map = element_dof_map(ni, nj);
            let Some(row) = map.iter().position(|&i| i == dof) else {
                return Ok(());
            };
            let op = matrix[row].map(Ball::exact);
            let evaluated = evaluate_row(&op, &map, Ball::ZERO, 0.0, u, bound)?;
            add_ball(&mut sum, &mut radius, evaluated.value, false)?;
            scale = add_up(scale, evaluated.absolute_scale);
            remainder = add_up(remainder, evaluated.remainder);
            Ok(())
        };
    // Adjacency first: only an element whose DOF map contains the reaction
    // DOF contributes, so a block is formed only for such an element. The
    // contributing elements, their order and their arithmetic are unchanged.
    let adjacent = |ni: usize, nj: usize| element_dof_map(ni, nj).contains(&dof);
    for frame in &source.frames {
        let (ni, nj) = (frame.node_i.index, frame.node_j.index);
        if !adjacent(ni, nj) {
            continue;
        }
        let k = frame_matrix(&mut a, frame)?;
        apply(ni, nj, &k)?;
    }
    for arc in &source.curved {
        if !adjacent(arc.node_i, arc.node_j) {
            continue;
        }
        let k = curved_matrix(&mut a, arc)?;
        apply(arc.node_i, arc.node_j, &k)?;
    }
    for connector in &source.connectors {
        let (ni, nj) = (connector.node_i().index, connector.node_j().index);
        if !adjacent(ni, nj) {
            continue;
        }
        let k = connector_matrix(&mut a, connector)?;
        apply(ni, nj, &k)?;
    }
    // T4-U5b: each connector's tie rods, after every connector.
    for rod in source.connectors.iter().flat_map(|c| c.tie_rods()) {
        let (ni, nj) = (rod.node_i().index, rod.node_j().index);
        if !adjacent(ni, nj) {
            continue;
        }
        let k = tie_rod_matrix(&mut a, rod)?;
        apply(ni, nj, &k)?;
    }
    for &(i, k) in &source.springs {
        if i == dof {
            let op = [Ball::exact(Wide2::from_f64(k)?)];
            let row = evaluate_row(&op, &[i], Ball::ZERO, 0.0, u, bound)?;
            add_ball(&mut sum, &mut radius, row.value, false)?;
            scale = add_up(scale, row.absolute_scale);
            remainder = add_up(remainder, row.remainder);
        }
    }
    let mut frame_cursor = super::formation_check::FrameCursor::default();
    for (index, term) in force
        .terms()
        .iter()
        .enumerate()
        .filter(|(_, term)| term.dof == dof)
    {
        let record = force
            .closure_records()
            .get(index)
            .ok_or_else(|| shape("reaction closure"))?;
        if record.formed {
            for primitive in source.loads.iter().filter(|p| p.term_index == index) {
                // T4-U8 (Design §4 item 2): the reaction's straight load
                // primitives are bound to their formation-source frames.
                if !primitive_frame_bound(source, primitive, &mut frame_cursor) {
                    return Err(shape(
                        "support reaction: straight primitive frame is not its formation-source frame",
                    ));
                }
                let (m, r) = load_reformation(primitive)?;
                let row = primitive.local_row;
                let ball = Ball {
                    m: m[row],
                    r: r[row],
                };
                add_ball(&mut sum, &mut radius, ball, true)?;
                scale = add_up(scale, ball_magnitude(ball)?);
            }
        } else {
            let mut held = ExactAccumulator::new();
            term.accumulate(&mut held, false).map_err(sum_error)?;
            let value = sum_ball(&held, 0.0)?;
            add_ball(&mut sum, &mut radius, value, true)?;
            scale = add_up(scale, ball_magnitude(value)?);
        }
    }
    CertifiedRow::finish(sum_ball(&sum, radius)?, remainder, scale)
}
/// Nonlinear K-D5 (DEL-04-01 kd5-nonlinear-active-set.md §2.1): the ball of
/// the displacement or rotation at `dof`, m = u + w + w2 as one exact sum and
/// remainder 2^{exp}*LambdaSigma (A_r = e_dof). At a prescribed DOF there is
/// no correction and no remainder.
///
/// T4-U5b item 6: the published value is the exact sum u + w + w2 rounded
/// once to binary64 (`ExactAccumulator::round`), not the p=128 midpoint
/// rounded again, which double-rounds at a binary64 midpoint (u = 1,
/// w = 2^-53, w2 = 2^-130 is 1 + 2^-52 once, 1 twice). The ball, the
/// remainder and every decision are unchanged; the rounding term is formed
/// for this published value.
pub fn certify_displacement(
    dof: usize,
    u: &[f64],
    bound: &MemberBoundInputs,
) -> Result<CertifiedRow, RecoveryFailure> {
    let row = evaluate_row(&[Ball::ONE], &[dof], Ball::ZERO, 0.0, u, bound)?;
    let mut exact = ExactAccumulator::new();
    for value in [u[dof], bound.w[dof], bound.w2[dof]] {
        exact.add(value).map_err(sum_error)?;
    }
    let published = exact.round().map_err(sum_error)?;
    if !published.is_finite() {
        return Err(shape("recovery publication"));
    }
    row.republished(if published == 0.0 { 0.0 } else { published })
}
/// Nonlinear K-D5 (§1.6, §3.1): one load primitive's re-formation at its
/// row as a published scalar, fl(m) with B = |fl(m) - m| + r. The explicit
/// sliding force has r = 0; the coupled force is its Stage-A ball.
pub fn certify_load_primitive(primitive: &LoadPrimitive) -> Result<CertifiedRow, RecoveryFailure> {
    let (m, r) = load_reformation(primitive)?;
    let row = primitive.local_row;
    if row >= 12 {
        return Err(shape("load primitive row"));
    }
    let ball = Ball {
        m: m[row],
        r: finite(r[row], "load primitive radius")?,
    };
    CertifiedRow::finish(ball, 0.0, ball_magnitude(ball)?)
}
/// Exact sign of sum_i c_i*m_i + sum_k a_k*b_k + sum_j x_j (binary64 c, a,
/// b, x; p=128 m). `None` when a midpoint's split truncates below the
/// subnormal range, an operand is not finite or an exact sum leaves its
/// range: no decision is then made, and every caller fails closed.
fn exact_sign(terms: &[(f64, Wide2)], products: &[(f64, f64)], addends: &[f64]) -> Option<i8> {
    let mut sum = ExactAccumulator::new();
    for &(c, m) in terms {
        let split = m.split_binary64().ok()?;
        if split.truncated_below_min_subnormal() {
            return None;
        }
        for &part in split.terms() {
            sum.add_product(c, part).ok()?;
        }
    }
    for &(a, b) in products {
        if !(a.is_finite() && b.is_finite()) {
            return None;
        }
        sum.add_product(a, b).ok()?;
    }
    for &x in addends {
        if !x.is_finite() {
            return None;
        }
        sum.add(x).ok()?;
    }
    Some(sum.signum())
}
impl CertifiedRow {
    /// The decision radius of the true value about the midpoint: the
    /// evaluation radius plus the remainder, upward (§2.1). The publication
    /// rounding gap is not part of it: decisions use the midpoint m.
    pub fn decision_radius(&self) -> f64 {
        add_up(self.radius, self.remainder)
    }
    /// m - rho > x, decided exactly (§2.2).
    pub fn lower_exceeds(&self, x: f64) -> bool {
        exact_sign(&[(1.0, self.value.m)], &[], &[-self.decision_radius(), -x]) == Some(1)
    }
    /// m + rho < x, decided exactly (§2.2).
    pub fn upper_below(&self, x: f64) -> bool {
        exact_sign(&[(-1.0, self.value.m)], &[], &[-self.decision_radius(), x]) == Some(1)
    }
    /// +1 or -1 when the whole ball lies strictly on one side of zero.
    pub fn established_sign(&self) -> Option<f64> {
        if self.lower_exceeds(0.0) {
            Some(1.0)
        } else if self.upper_below(0.0) {
            Some(-1.0)
        } else {
            None
        }
    }
    fn midpoint_sign(&self) -> f64 {
        if self.value.m.is_sign_negative() {
            -1.0
        } else {
            1.0
        }
    }
    /// Derived-normal stick capacity (§2.2): mu*(|m_n| - rho_n) > |m_t| + rho_t,
    /// decided exactly in one exact sum; 0 must lie outside the normal ball.
    pub fn stick_capacity_exceeds(normal: &Self, coefficient: f64, tangential: &Self) -> bool {
        if !(coefficient.is_finite() && coefficient > 0.0) || normal.established_sign().is_none() {
            return false;
        }
        exact_sign(
            &[
                (coefficient * normal.midpoint_sign(), normal.value.m),
                (-tangential.midpoint_sign(), tangential.value.m),
            ],
            &[(-coefficient, normal.decision_radius())],
            &[-tangential.decision_radius()],
        ) == Some(1)
    }
    /// Explicit-normal stick capacity (§2.2): mu*N > |m_t| + rho_t with
    /// N > 0, decided exactly.
    pub fn stick_capacity_exceeds_explicit(coefficient: f64, normal: f64, tangential: &Self) -> bool {
        if !(coefficient.is_finite() && coefficient > 0.0 && normal.is_finite() && normal > 0.0) {
            return false;
        }
        exact_sign(
            &[(-tangential.midpoint_sign(), tangential.value.m)],
            &[(coefficient, normal)],
            &[-tangential.decision_radius()],
        ) == Some(1)
    }
    /// The enclosure of the true value as a ball: midpoint m, radius
    /// evaluation radius + remainder (upward), for further ball arithmetic.
    pub fn enclosure(&self) -> CertifiedValue {
        CertifiedValue {
            ball: Ball {
                m: self.value.m,
                r: self.decision_radius(),
            },
        }
    }
}
impl CertifiedValue {
    /// A binary64 midpoint with a binary64 radius.
    pub fn with_radius(value: f64, radius: f64) -> Result<Self, RecoveryFailure> {
        Ok(Self {
            ball: Ball {
                m: Wide2::from_f64(value)?,
                r: finite(radius, "certified value radius")?,
            },
        })
    }
    /// The published scalar of this ball: fl(m) with B = |fl(m) - m| + r.
    pub fn into_row(self) -> Result<CertifiedRow, RecoveryFailure> {
        CertifiedRow::finish(self.ball, 0.0, ball_magnitude(self.ball)?)
    }
}

/// Spring support-on-pipe sign is negative on ordinary and W2 routes.
///
/// W-16 R-c (DEL-04-03 Design `pf-nonlinear-imposed-motion.md` §11.1): a
/// spring whose base moves by `ground` publishes -k·(u + w + w₂ - g). When
/// `ground` compares equal to zero, of either sign, the fixed-base row is
/// evaluated unchanged and the authored zero is never subtracted. Otherwise
/// the exact product +k·g joins the accumulator that holds -k·u, -k·w and
/// -k·w₂, and the evaluation scale includes |k·g|; neither k·g nor u - g is
/// folded to binary64.
pub fn certify_spring_reaction(
    dof: usize,
    stiffness: f64,
    ground: f64,
    u: &[f64],
    bound: &MemberBoundInputs,
) -> Result<CertifiedRow, RecoveryFailure> {
    let op = [Ball::exact(Wide2::from_f64(-stiffness)?)];
    if ground == 0.0 {
        return evaluate_row(&op, &[dof], Ball::ZERO, 0.0, u, bound);
    }
    let mut load = RowLoad::default();
    load.sum.add_product(stiffness, ground).map_err(sum_error)?;
    load.scale = mul_up(stiffness.abs(), ground.abs());
    evaluate_row_inner(&op, &[dof], load, u, bound, true)
}

#[cfg(test)]
#[path = "certified_recovery_tests.rs"]
mod tests;

/// Directed bounds of a Bernstein stress coefficient (Pa). They transport the
/// source/evaluation ball, not the displacement remainder against u*.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CertifiedCoefficientBounds {
    pub lower: f64,
    pub upper: f64,
}
/// One independently supplied closed statics interval. Adjacent intervals keep
/// their own degree-2 polynomial; uniform-load intensity jumps do not imply an
/// action jump. The allowance below covers repaired public samples separately.
#[derive(Debug, Clone, Copy)]
pub struct CertifiedStraightStressSpan {
    pub start: f64,
    pub end: f64,
    pub axial: [CertifiedCoefficientBounds; 3],
    pub bending_y: [CertifiedCoefficientBounds; 3],
    pub bending_z: [CertifiedCoefficientBounds; 3],
    pub publication_radius_pa: f64,
}
fn coefficient_bounds(value: Ball) -> Result<CertifiedCoefficientBounds, RecoveryFailure> {
    if value.m.is_zero() && value.r == 0.0 {
        return Ok(CertifiedCoefficientBounds {
            lower: 0.0,
            upper: 0.0,
        });
    }
    let split = value.m.split_binary64()?;
    let radius = if split.truncated_below_min_subnormal() {
        add_up(value.r, TINY)
    } else {
        value.r
    };
    let mut lower = ExactAccumulator::new();
    let mut upper = ExactAccumulator::new();
    for &part in split.terms() {
        lower.add(part).map_err(sum_error)?;
        upper.add(part).map_err(sum_error)?;
    }
    lower.add(-radius).map_err(sum_error)?;
    upper.add(radius).map_err(sum_error)?;
    let lower = lower.round().map_err(sum_error)?.next_down();
    let upper = upper.round().map_err(sum_error)?.next_up();
    if !lower.is_finite() || !upper.is_finite() {
        return Err(shape("stress coefficient transport range"));
    }
    Ok(CertifiedCoefficientBounds { lower, upper })
}
fn positive(value: Ball) -> Ball {
    Ball {
        m: value.m.abs(),
        r: value.r,
    }
}
/// Positive arithmetic majorant of the actual section recipe. Every operand's
/// magnitude/radius is replaced by a uniform upper operand and every +/- by +.
/// L1/L2 radii are monotone in these nonnegative operands. The returned radius
/// therefore bounds the p128 evaluation error at every section, without using R
/// and without sampling an observed row delta. Exact-sum split losses are added
/// separately because the positive-product path does not itself split K.
fn straight_section_evaluation_envelopes(
    ctx: &StraightRecoveryContext,
    loads: &CertifiedMemberLoads,
    u: &[f64],
    bound: &MemberBoundInputs,
) -> Result<[Ball; 6], RecoveryFailure> {
    let mut b = Balls::new()?;
    let lu = abs_up(&ctx.length)?;
    let distance = Ball {
        m: ctx.length,
        r: mul_up(2.0, mul_up(U, lu)),
    };
    let active = Ball {
        m: ctx.length,
        r: add_up(add_up(distance.r, distance.r), mul_up(U, lu)),
    };
    let ea = b.mul(distance, distance)?;
    let ends = b.add(ea, ea)?;
    let half = b.half(ends)?;
    let da = b.mul(distance, active)?;
    let lever = b.add(da, half)?;
    let mut eq: [Ball; 6] = ctx.equivalent[..6]
        .try_into()
        .map_err(|_| shape("section envelope load"))?;
    for value in &mut eq {
        *value = positive(*value);
    }
    // The actual recipe enters each load contribution as its own exact addend
    // (`straight_section_rows`); each addend's split may lose one TINY below
    // the subnormal grid, so the majorant counts them per row.
    let mut addends = [1.0_f64; 6];
    for (moment, shear) in [(4, 2), (5, 1)] {
        let term = b.mul(distance, positive(ctx.equivalent[shear]))?;
        eq[moment] = b.add(eq[moment], term)?;
        addends[moment] += 1.0;
    }
    for load in loads.uniform_loads() {
        let load = load?;
        for r in [0, 1, 2, 4, 5] {
            addends[r] += 1.0;
        }
        let mut intensity = [Ball::ZERO; 3];
        for r in 0..3 {
            for c in 0..3 {
                let term = b.mul(
                    Ball::exact(ctx.axes[r][c].abs()),
                    positive(load.intensity[c]),
                )?;
                intensity[r] = b.add(intensity[r], term)?;
            }
        }
        for r in 0..3 {
            let term = b.mul(intensity[r], active)?;
            eq[r] = b.add(eq[r], term)?;
        }
        let my = b.mul(intensity[2], lever)?;
        eq[4] = b.add(eq[4], my)?;
        let mz = b.mul(intensity[1], lever)?;
        eq[5] = b.add(eq[5], mz)?;
    }
    let mut out = [Ball::ZERO; 6];
    for r in 0..6 {
        let mut sum = Ball::ZERO;
        let mut split_allowance = 0.0;
        for c in 0..12 {
            let i = ctx.map[c];
            if i >= u.len() || i >= bound.w.len() || i >= bound.w2.len() {
                return Err(shape("section envelope DOF"));
            }
            let mut coefficient = positive(ctx.operator[r][c]);
            let shear = if r == 4 {
                Some(2)
            } else if r == 5 {
                Some(1)
            } else {
                None
            };
            if let Some(shear) = shear {
                let term = b.mul(distance, positive(ctx.operator[shear][c]))?;
                coefficient = b.add(coefficient, term)?;
            }
            let x = positive(corrected_ball(u[i], bound.w[i], bound.w2[i])?);
            let term = b.mul(coefficient, x)?;
            sum = b.add(sum, term)?;
            for value in [u[i], bound.w[i], bound.w2[i]] {
                split_allowance = add_up(split_allowance, mul_up(TINY, value.abs()));
            }
        }
        sum = b.add(sum, eq[r])?;
        sum.r = add_up(sum.r, split_allowance);
        if !eq[r].m.is_zero() {
            sum.r = add_up(sum.r, mul_up(TINY, addends[r]));
        }
        sum.r = finite(sum.r, "section evaluation envelope")?;
        out[r] = sum;
    }
    Ok(out)
}
/// Upper error of exact-catalog MPa publication and a later Pa conversion.
/// For |z|<=T, RN(z/1e6) contributes u64*T+1e6*Tiny. RN(y*1e6)
/// followed by one unconditional next_up contributes at most
/// 4*u64*(T+first)+4*Tiny (including subnormal grids). Zero stays exact.
fn mpa_publication_transport(t: f64) -> Result<f64, RecoveryFailure> {
    if t == 0.0 {
        return Ok(0.0);
    }
    let u64 = f64::from_bits(970_u64 << 52);
    let first = add_up(mul_up(u64, t), mul_up(1_000_000.0, TINY));
    let second = add_up(mul_up(4.0 * u64, add_up(t, first)), 4.0 * TINY);
    finite(add_up(first, second), "MPa publication transport")
}
/// Source Bernstein controls from the same intended K, corrected displacement
/// and raw uniform primitives as the repaired row recipe. No binary64 end
/// action or binary64 statics coefficient is lifted into this construction.
/// `axial_increment` is the constant positive j-side wall-force increment (the
/// pressure consumer supplies 2nu*pAi formed by the same at-p pressure law).
///
/// Transport concerns the repaired at-p polynomial/publication only: R is
/// deliberately absent. The ordinary L1 and circular norm maximum consumers
/// share these coefficients but retain their distinct objective functions.
pub fn certify_straight_stress_spans(
    frame: &FrameElement,
    loads: &CertifiedMemberLoads,
    u: &[f64],
    bound: &MemberBoundInputs,
    area: f64,
    section_modulus: f64,
    axial_increment: Option<CertifiedValue>,
) -> Result<Vec<CertifiedStraightStressSpan>, RecoveryFailure> {
    if !(area > 0.0 && area.is_finite() && section_modulus > 0.0 && section_modulus.is_finite()) {
        return Err(shape("stress coefficient section"));
    }
    let ctx = straight_context(frame, loads)?;
    let source_count = loads
        .primitives
        .iter()
        .filter(|p| {
            p.local_row == 0
                && p.identity.source.starts_with(loads.source_prefix)
                && member_pair(&p.formula)
                    .is_some_and(|(i, j)| same_pair(loads.node_i, loads.node_j, i, j))
                && matches!(p.formula, LoadFormula::UniformStraight { .. })
        })
        .count();
    let mut boundaries = Vec::with_capacity(2 * source_count + 2);
    boundaries.extend([0.0, 1.0]);
    for p in loads.primitives.iter().filter(|p| {
        p.local_row == 0
            && p.identity.source.starts_with(loads.source_prefix)
            && member_pair(&p.formula)
                .is_some_and(|(i, j)| same_pair(loads.node_i, loads.node_j, i, j))
    }) {
        if let LoadFormula::UniformStraight {
            start_fraction,
            end_fraction,
            ..
        } = &p.formula
        {
            boundaries.extend([*start_fraction, *end_fraction]);
        }
    }
    boundaries.sort_by(f64::total_cmp);
    boundaries.dedup();
    if boundaries
        .iter()
        .any(|v| !v.is_finite() || *v < 0.0 || *v > 1.0)
    {
        return Err(shape("stress coefficient span"));
    }
    let mut b = Balls::new()?;
    let mut envelope = straight_section_evaluation_envelopes(&ctx, loads, u, bound)?;
    if let Some(increment) = axial_increment {
        envelope[0] = b.add(envelope[0], positive(increment.ball))?;
    }
    let stress_envelope = [
        b.div(envelope[0], b.lift(area)?, "coefficient area")?,
        b.div(
            envelope[4],
            b.lift(section_modulus)?,
            "coefficient section modulus",
        )?,
        b.div(
            envelope[5],
            b.lift(section_modulus)?,
            "coefficient section modulus",
        )?,
    ];
    let evaluation_radius = stress_envelope
        .iter()
        .fold(0.0, |r, value| add_up(r, value.r));
    let mut out = Vec::with_capacity(boundaries.len() - 1);
    for interval in boundaries.windows(2) {
        let start = interval[0];
        let end = interval[1];
        if start >= end {
            continue;
        }
        let fs = b.lift(start)?;
        let fe = b.lift(end)?;
        let sum = b.add(fs, fe)?;
        let fm = b.half(sum)?;
        let points = [
            straight_section_rows(&ctx, loads, u, bound, fs, false)?,
            straight_section_rows(&ctx, loads, u, bound, fm, false)?,
            straight_section_rows(&ctx, loads, u, bound, fe, false)?,
        ];
        let mut controls = [[Ball::ZERO; 3]; 3];
        for (component, row) in [0, 4, 5].into_iter().enumerate() {
            let divisor = if row == 0 { area } else { section_modulus };
            let mut values = points.map(|point| point[row].value);
            if row == 0 {
                if let Some(increment) = axial_increment {
                    for value in &mut values {
                        *value = b.add(*value, increment.ball)?;
                    }
                }
            }
            for value in &mut values {
                *value = b.div(*value, b.lift(divisor)?, "stress coefficient divisor")?;
            }
            let ends = b.add(values[0], values[2])?;
            let half = b.half(ends)?;
            let twice = b.pow2(values[1], 1)?;
            controls[component] = [values[0], b.sub(twice, half)?, values[2]];
        }
        let mut enclosures = [[CertifiedCoefficientBounds {
            lower: 0.0,
            upper: 0.0,
        }; 3]; 3];
        let mut l1_scale = 0.0;
        for component in 0..3 {
            let mut maximum = 0.0_f64;
            for coefficient in 0..3 {
                enclosures[component][coefficient] =
                    coefficient_bounds(controls[component][coefficient])?;
                let v = enclosures[component][coefficient];
                maximum = maximum.max(v.lower.abs()).max(v.upper.abs());
            }
            l1_scale = add_up(l1_scale, maximum);
        }
        l1_scale = finite(l1_scale, "stress polynomial range")?;
        // Following the row recipe, a stress/normal-summary/MPa evaluation has
        // at most16 rounded p128 operations (three section-factor divisions,
        // up to six norm operations, up to three sums, three unit divisions).
        // gamma16 <=32u because16u<1/2. Absolute sums/L1 and Euclidean norms
        // are 1-Lipschitz in the sum of component errors.
        let at_p_scale = add_up(l1_scale, evaluation_radius);
        let extra = add_up(evaluation_radius, mul_up(32.0 * U, at_p_scale));
        let publication_radius_pa = finite(
            add_up(extra, mpa_publication_transport(add_up(l1_scale, extra))?),
            "stress publication radius",
        )?;
        out.push(CertifiedStraightStressSpan {
            start,
            end,
            axial: enclosures[0],
            bending_y: enclosures[1],
            bending_z: enclosures[2],
            publication_radius_pa,
        });
    }
    Ok(out)
}

/// Build-specific resource atoms for retained-pricing's stack/transport model.
/// These are type sizes, not a measurement of a compiler-selected frame. The
/// pricer combines live constructor and evaluation stages from their source.
#[derive(Debug, Clone, Copy)]
pub struct CertifiedStressControlSizes {
    pub span: usize,
    pub context: usize,
    pub wide_matrix: usize,
    pub local_operator: usize,
    pub section_operator: usize,
    pub point_rows: usize,
    pub exact_accumulator: usize,
    pub ball: usize,
    pub wide: usize,
    pub arithmetic: usize,
    pub constructor_workspace_upper: usize,
    pub evaluation_workspace_upper: usize,
    pub workspace_upper: usize,
}
pub const fn certified_stress_control_sizes() -> CertifiedStressControlSizes {
    // Recoverable current-D1 workspace census (no member uniform sources).
    // Kernel-only: PP's retained two42-row certificates/support6-row array,
    // source/mapped Vec backing and all PP caller frames are separate charges.
    // No recursive source evaluation occurs on this admitted call tree.
    //
    // Constructor call tree / separately charged arrays:
    // straight_context -> frame_matrix -> chord_axes/lift3/sub3/norm3/dot3/
    // scale3/cross3, then rotate; and local_operator/local_loads -> Balls.
    // Two Context slots cover caller+return/move. Four Wide matrices cover
    // constructor result, frame K, rotate output and returned-matrix staging.
    // Two Ball operators cover local_operator output+return staging.
    // Wide scratch names include frame axes,length,em,gm,l2,l3,ea,axial,gj,
    // torsion; coefficient closure ei,t12,t6,t4,t2 and returned arrays;
    // T4-U8's Timoshenko leaf adds gas and the captured (G*As)*L^2 option,
    // and in the closure phi,1+phi,4+phi,2-phi,d3,d2,d1 (eleven slots, inside
    // the same 256 bound; the Euler–Bernoulli frame forms none of them);
    // z12/z6/z4/z2/y12/y6/y4/y2; bz/by and their array-iterator/pattern copies;
    // rotate temp,s,t; chord d,length,ex,yr,projection,yc,t,ym,ey,ez;
    // vector-helper return arrays/products and arithmetic-result move slots.
    // Their deliberately retained union is bounded by256 Wide slots, including
    // geometry helpers whose actual lifetimes precede bending/rotate staging.
    //
    // Evaluation call tree / separately charged arrays:
    // certify_straight_stress_spans -> straight_section_evaluation_envelopes
    // or straight_section_rows -> evaluate_row_inner -> corrected_ball ->
    // sum_ball; then CertifiedRow::finish/coefficient_bounds and transport.
    // One Context remains live; two3x6 Row arrays cover control points+move
    // staging; two6x12 Ball operators cover section op+array initialization.
    // Eight ExactAccumulator slots exceed the complete named sum inventory:
    // row sum (a section row's load sum, moved into its evaluation as the
    // row's sum; rows are formed one at a time), corrected sum, coefficient
    // lower/upper sums, and their move slots. Eight68-limb magnitudes cover
    // nested net/project/subtract scratch.
    //
    // Remaining Ball-equivalent slot census (matrices/point arrays excluded):
    // main builder<=64 (envelope6,stress3,controls9,values3,fs/fe/fm/sum,
    // increment/options and argument/return moves); context/local loads<=64
    // (equivalent12,out12,product/axis lift and moves); section builder<=96
    // (eq6,intensity3,load3,fraction/end/start/active/squares/lever/terms,
    // section output/moves); row/corrected/sum/finish<=64; positive envelope
    // <=96 (eq6,out6,intensity3,load3,scalar envelopes/coefficient/x/term);
    // coefficient transport<=16; shared Ball lemmas<=64 (lift/exact/neg,
    // add/sub/mul/div/half/pow2/sqrt arguments/results); empty D1 iterator and
    // Option machinery<=16. Total<=480, charged1024 to include leaf-arithmetic
    // fixed buffers and composite scalar Row moves (ceil(Row/Ball) slots).
    //
    // Scalars/indices/iterator headers: retain the union of the constructor,
    // control/section/envelope loops, row/split/projection loops, and fixed
    // arithmetic leaf frames;256 pointer/f64-sized slots bound it. Arithmetic
    // contexts coexist only in constructor or builder/section/row/projection
    // chains (<=6), charged16. Wide leaves use fixed<=4-limb scratch; exact-sum
    // leaves use the separately charged68-limb buffers, not dynamic arrays.
    // Stable-sort/allocator/system-library stack is not a kernel arithmetic
    // frame; current D1 sorts only[0,1]. Extended-domain scratch is audited by
    // its owning pricing scope before promotion. This is a source upper bound,
    // not an inference from type-size equality or compiler stack measurement.
    let scalars = 1024 * std::mem::size_of::<Ball>()
        + 256 * std::mem::size_of::<usize>()
        + 256 * std::mem::size_of::<Wide2>()
        + 16 * std::mem::size_of::<WideArith>();
    let exact = 8 * std::mem::size_of::<ExactAccumulator>() + 8 * std::mem::size_of::<[u64; 68]>();
    let constructor = 2 * std::mem::size_of::<StraightRecoveryContext>()
        + 4 * std::mem::size_of::<[[Wide2; 12]; 12]>()
        + 2 * std::mem::size_of::<Operator>()
        + scalars
        + exact;
    let evaluation = std::mem::size_of::<StraightRecoveryContext>()
        + 2 * std::mem::size_of::<[[CertifiedRow; 6]; 3]>()
        + 2 * std::mem::size_of::<[[Ball; 12]; 6]>()
        + scalars
        + exact;
    CertifiedStressControlSizes {
        span: std::mem::size_of::<CertifiedStraightStressSpan>(),
        context: std::mem::size_of::<StraightRecoveryContext>(),
        wide_matrix: std::mem::size_of::<[[Wide2; 12]; 12]>(),
        local_operator: std::mem::size_of::<Operator>(),
        section_operator: std::mem::size_of::<[[Ball; 12]; 6]>(),
        point_rows: std::mem::size_of::<[[CertifiedRow; 6]; 3]>(),
        exact_accumulator: std::mem::size_of::<ExactAccumulator>(),
        ball: std::mem::size_of::<Ball>(),
        wide: std::mem::size_of::<Wide2>(),
        arithmetic: std::mem::size_of::<WideArith>(),
        constructor_workspace_upper: constructor,
        evaluation_workspace_upper: evaluation,
        workspace_upper: if constructor > evaluation {
            constructor
        } else {
            evaluation
        },
    }
}
