//! Bounded private normalized member arithmetic. These scalar records carry no
//! source identity, eligibility, row certificate, admission policy, or tariff.
#![allow(dead_code)] // No product caller in this isolated implementation slice.

use super::adaptive::AttemptStop;
use super::directed::{self, Toward};
use super::ledger::RetainedLedger;
use super::wide::multi::{AttemptWork, WideContext};
use super::wide::Wide;
use super::wide_sum::{ExactWideSum, SumWork};
use super::work::{WorkFault, WorkStatus, WorkTotal};
use std::cmp::Ordering;

type Endpoint = Wide<16>;

#[derive(Debug, Clone, Copy)]
pub(super) struct Enclosure {
    lo: Endpoint,
    hi: Endpoint,
}
impl Enclosure {
    pub(crate) fn endpoints(&self) -> (&Endpoint, &Endpoint) {
        (&self.lo, &self.hi)
    }
    fn point(x: Endpoint) -> Self {
        Self { lo: x, hi: x }
    }
    fn positive(self) -> Result<Self, NumericError> {
        if positive(&self.lo) && self.lo.cmp_value(&self.hi) != Ordering::Greater {
            Ok(self)
        } else {
            Err(NumericError::NonpositiveSource)
        }
    }
    fn hull_point(self, x: Endpoint) -> Self {
        Self {
            lo: min(self.lo, x),
            hi: max(self.hi, x),
        }
    }
}

/// Numeric routes only. A future checked source adapter must prove selection,
/// adjacency, actual owner/ordinal association and all existing admission rules.
#[derive(Debug, Clone, Copy)]
pub(super) enum MaterialOperands {
    ExactENu {
        e: f64,
        nu: f64,
    },
    Ordinary {
        e: f64,
        g: f64,
    }, // base or exact point, independently selected E/G
    Interpolated {
        t_lo: f64,
        t: f64,
        t_hi: f64,
        e_lo: f64,
        e_hi: f64,
        g_lo: f64,
        g_hi: f64,
        e_hat: f64,
        g_hat: f64,
    },
}
#[derive(Debug, Clone, Copy)]
pub(super) struct AdmittedOperands {
    e: f64,
    g: f64,
    a: f64,
    j: f64,
    iz: f64,
    iy: f64,
    z_hat: f64,
}
#[derive(Debug, Clone, Copy)]
pub(super) struct MemberOperands {
    diameter: f64,
    effective_wall: f64,
    material: MaterialOperands,
    admitted: AdmittedOperands,
}
#[derive(Debug)]
pub(super) struct MemberEnclosures {
    effective_wall_bits: u64,
    c: Endpoint,
    ri: Enclosure,
    p: Enclosure,
    q: Enclosure,
    geometry_g: Enclosure,
    a: Enclosure,
    i: Enclosure,
    j: Enclosure,
    z: Enclosure,
    e: Enclosure,
    g: Enclosure,
    coefficients: [Enclosure; 4],
    admitted_products: [Endpoint; 4],
    coefficient_differences: [Endpoint; 4],
    represented_z: Option<Enclosure>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum NumericError {
    Arithmetic(AttemptStop),
    NonFinite,
    NonpositiveSource,
    InvalidGeometry,
    InvalidMaterial,
    MaterialBits,
    TemperatureOrder,
    NonpositiveDenominator,
    AxisBits,
    Binary64Range,
}
impl From<AttemptStop> for NumericError {
    fn from(e: AttemptStop) -> Self {
        Self::Arithmetic(e)
    }
}

/// Typed counts only; no endpoints or public radius authority.
#[derive(Debug, Clone, Copy)]
pub struct NumericTrace {
    pub wide_lme: WorkTotal, pub exact_sum_lme: WorkTotal,
    pub entries: [WorkTotal;7], pub f64_arithmetic: WorkTotal, pub sticky_status: WorkStatus,
}
/// Additional local trace writes/return-copy work, never native LME. Checked and
/// sticky; the public transaction must separately account its later projections.
#[derive(Debug, Default)]
pub struct TraceCopyWork { pub events: WorkTotal, pub bytes: WorkTotal }
impl TraceCopyWork {
    pub(crate) fn record<T>(&mut self) {
        self.events=self.events.add(WorkTotal::exact_count(1));
        let bytes=match u64::try_from(std::mem::size_of::<T>()) {Ok(n)=>WorkTotal::exact_count(n),
            Err(_)=>WorkTotal::zero().join_status(WorkStatus::from_fault(super::work::WorkFault::Overflow))};
        self.bytes=self.bytes.add(bytes);
    }
    pub fn status(&self)->WorkStatus {self.events.status().join(self.bytes.status())}
}
#[derive(Debug, Clone, Copy)]
enum Entry {
    Add = 0,
    Sub = 1,
    Mul = 2,
    Div = 3,
    R4 = 4,
    B64U = 5,
    Sqrt = 6,
}
#[derive(Debug)]
pub(super) struct NumericWork {
    wide: AttemptWork,
    sums: SumWork,
    entries: [WorkTotal; 7],
    // No rounded binary64 arithmetic is performed by this component. Conversion
    // entries are counted separately; abs/successor are bit operations.
    f64_arithmetic: WorkTotal,
    status: WorkStatus,
}
impl NumericWork {
    fn trace(&self, copies:&mut TraceCopyWork)->NumericTrace {
        copies.record::<NumericTrace>();
        NumericTrace {wide_lme:self.wide.checked_lme(),exact_sum_lme:self.sums.checked_lme(),
            entries:self.entries,f64_arithmetic:self.f64_arithmetic,sticky_status:self.status}
    }
    fn new() -> Self {
        Self {
            wide: AttemptWork::default(),
            sums: SumWork::default(),
            entries: [WorkTotal::zero(); 7],
            f64_arithmetic: WorkTotal::zero(),
            status: WorkStatus::default(),
        }
    }
    fn status(&self) -> WorkStatus {
        self.entries.iter().fold(
            self.wide
                .checked_lme()
                .add(self.sums.checked_lme())
                .add(self.f64_arithmetic)
                .status()
                .join(self.status),
            |s, v| s.join(v.status()),
        )
    }
    fn begin(&mut self, entry: Entry) -> Result<(), NumericError> {
        if let Some(f) = self.status().fault() {
            return Err(AttemptStop::WorkAccounting(f).into());
        }
        let i = entry as usize;
        let next = self.entries[i].add(WorkTotal::exact_count(1));
        self.status = self.status.join(next.status());
        if let Some(f) = next.status().fault() {
            return Err(AttemptStop::WorkAccounting(f).into());
        }
        self.entries[i] = next;
        Ok(())
    }
    fn checked<T>(&mut self, result: Result<T, NumericError>) -> Result<T, NumericError> {
        if let Err(NumericError::Arithmetic(AttemptStop::WorkAccounting(f))) = &result {
            self.status = self.status.join(WorkStatus::from_fault(*f));
        }
        match result {
            Err(e) => Err(e), // retain original numeric cause, even with unavailable work
            Ok(v) => match self.status().fault() {
                Some(f) => Err(AttemptStop::WorkAccounting(f).into()),
                None => Ok(v),
            },
        }
    }
}
#[derive(Debug)]
pub(super) struct MemberSpent {
    result: Result<MemberEnclosures, NumericError>,
    work: NumericWork,
}
impl MemberSpent {
    pub(super) fn result(&self) -> Result<&MemberEnclosures, NumericError> {
        match &self.result {
            Err(e) => Err(e.clone()),
            Ok(v) => match self.work.status().fault() {
                Some(f) => Err(AttemptStop::WorkAccounting(f).into()),
                None => Ok(v),
            },
        }
    }
    pub(super) fn work(&self) -> &NumericWork {
        &self.work
    }
}

fn positive(x: &Endpoint) -> bool {
    !x.is_zero() && !x.is_sign_negative()
}
fn min(a: Endpoint, b: Endpoint) -> Endpoint {
    if a.cmp_value(&b) == Ordering::Greater {
        b
    } else {
        a
    }
}
fn max(a: Endpoint, b: Endpoint) -> Endpoint {
    if a.cmp_value(&b) == Ordering::Less {
        b
    } else {
        a
    }
}
fn lift(v: f64) -> Result<Endpoint, NumericError> {
    if !v.is_finite() {
        return Err(NumericError::NonFinite);
    }
    if v == 0.0 {
        return Ok(Endpoint::ZERO);
    }
    Endpoint::from_f64(v).map_err(|e| NumericError::Arithmetic(e.into()))
}
fn pos_lift(v: f64) -> Result<Endpoint, NumericError> {
    let x = lift(v)?;
    if positive(&x) {
        Ok(x)
    } else {
        Err(NumericError::NonpositiveSource)
    }
}
fn shift(x: &Endpoint, k: i64) -> Result<Endpoint, NumericError> {
    x.mul_pow2(k)
        .map_err(|e| NumericError::Arithmetic(e.into()))
}
fn shift_interval(x: Enclosure, k: i64) -> Result<Enclosure, NumericError> {
    Ok(Enclosure {
        lo: shift(&x.lo, k)?,
        hi: shift(&x.hi, k)?,
    })
}

// An operation owns its fresh context and sum. No fallible exit can bypass
// collection, including denominator rejection and lower-pass exact sign checks.
fn scalar_owned(
    work: &mut NumericWork,
    entry: Entry,
    a: &Endpoint,
    b: &Endpoint,
    toward: Toward,
    mut ctx: WideContext<16>,
    mut sum: ExactWideSum,
) -> Result<Endpoint, NumericError> {
    let result = (|| {
        if matches!(entry, Entry::Div) && !positive(b) {
            return Err(NumericError::NonpositiveDenominator);
        }
        let v = match entry {
            Entry::Add => directed::add_toward(&mut ctx, &mut sum, a, b, toward),
            Entry::Sub => directed::sub_toward(&mut ctx, &mut sum, a, b, toward),
            Entry::Mul => directed::mul_toward(&mut ctx, &mut sum, a, b, toward),
            Entry::Div => directed::div_toward(&mut ctx, &mut sum, a, b, toward),
            _ => return Err(AttemptStop::WorkAccounting(WorkFault::Inconsistent).into()),
        }?;
        Ok(if v.is_zero() { Endpoint::ZERO } else { v })
    })();
    work.wide.record(&ctx);
    work.sums.merge(&sum.work());
    work.checked(result)
}
impl NumericWork {
    fn scalar(
        &mut self,
        entry: Entry,
        a: &Endpoint,
        b: &Endpoint,
        t: Toward,
    ) -> Result<Endpoint, NumericError> {
        self.begin(entry)?;
        let ctx = WideContext::<16>::new(1024).map_err(|e| NumericError::Arithmetic(e.into()))?;
        scalar_owned(self, entry, a, b, t, ctx, ExactWideSum::new())
    }
    fn mul(&mut self, a: Enclosure, b: Enclosure) -> Result<Enclosure, NumericError> {
        a.positive()?;
        b.positive()?;
        Ok(Enclosure {
            lo: self.scalar(Entry::Mul, &a.lo, &b.lo, Toward::Down)?,
            hi: self.scalar(Entry::Mul, &a.hi, &b.hi, Toward::Up)?,
        })
        .and_then(Enclosure::positive)
    }
    fn div(&mut self, a: Enclosure, b: Enclosure) -> Result<Enclosure, NumericError> {
        a.positive()?;
        if !positive(&b.lo) || b.lo.cmp_value(&b.hi) == Ordering::Greater {
            return Err(NumericError::NonpositiveDenominator);
        }
        Ok(Enclosure {
            lo: self.scalar(Entry::Div, &a.lo, &b.hi, Toward::Down)?,
            hi: self.scalar(Entry::Div, &a.hi, &b.lo, Toward::Up)?,
        })
        .and_then(Enclosure::positive)
    }
    fn sub_pair(&mut self, a: &Endpoint, b: &Endpoint) -> Result<Enclosure, NumericError> {
        Ok(Enclosure {
            lo: self.scalar(Entry::Sub, a, b, Toward::Down)?,
            hi: self.scalar(Entry::Sub, a, b, Toward::Up)?,
        })
    }
    fn add(&mut self, a: Enclosure, b: Enclosure) -> Result<Enclosure, NumericError> {
        Ok(Enclosure {
            lo: self.scalar(Entry::Add, &a.lo, &b.lo, Toward::Down)?,
            hi: self.scalar(Entry::Add, &a.hi, &b.hi, Toward::Up)?,
        })
    }
    fn exact_product(&mut self, a: f64, b: f64) -> Result<Endpoint, NumericError> {
        // Separate lifts preserve up to 106 significant bits, including 2^-104.
        let (a, b) = (lift(a)?, lift(b)?);
        self.scalar(Entry::Mul, &a, &b, Toward::Up)
    }
    fn r4(&mut self, p: &[Endpoint; 4], t: Toward) -> Result<Endpoint, NumericError> {
        self.begin(Entry::R4)?;
        let ctx = WideContext::<16>::new(1024).map_err(|e| NumericError::Arithmetic(e.into()))?;
        r4_owned(self, p, t, ctx, ExactWideSum::new())
    }
    fn absdiff(&mut self, a: &Endpoint, b: &Endpoint) -> Result<Endpoint, NumericError> {
        let (large, small) = if a.cmp_value(b) == Ordering::Less {
            (b, a)
        } else {
            (a, b)
        };
        self.scalar(Entry::Sub, large, small, Toward::Up)
    }
    fn binary64_up(&mut self, x: &Endpoint) -> Result<f64, NumericError> {
        if x.is_sign_negative() && !x.is_zero() {
            return Err(NumericError::InvalidMaterial);
        }
        self.begin(Entry::B64U)?;
        let (result, sums) = directed::binary64_up_spent(x).into_parts();
        self.collect_binary64(result, sums)
    }
    fn collect_binary64(
        &mut self,
        result: Result<f64, AttemptStop>,
        sums: SumWork,
    ) -> Result<f64, NumericError> {
        self.sums.merge(&sums);
        let value = self.checked(result.map_err(NumericError::Arithmetic))?;
        if value.is_finite() {
            Ok(value)
        } else {
            Err(NumericError::Binary64Range)
        }
    }
}
/// B2-K (KD §3.3): an owner ledger's exact net at a DOF, enclosed outward at
/// 1024 bits as [RD1024(N_g), RU1024(N_g)]. Each endpoint is one `Add` entry
/// ("an exact sum rounded once toward a direction"), with a fresh context and
/// exact sum whose work is collected on every exit. A net of at most 1024
/// significant bits gives lo == hi exactly (E5).
fn net_owned(
    work: &mut NumericWork,
    ledger: &RetainedLedger,
    dof: usize,
    toward: Toward,
    mut ctx: WideContext<16>,
    mut sum: ExactWideSum,
) -> Result<Endpoint, NumericError> {
    let result = (|| -> Result<Endpoint, AttemptStop> {
        ledger.add_to(dof, &mut sum, false)?;
        let v = if hooks::nearest_net() {
            sum.round(&mut ctx)?
        } else {
            directed::round_toward(&mut ctx, &mut sum, toward)?
        };
        Ok(if v.is_zero() { Endpoint::ZERO } else { v })
    })();
    work.wide.record(&ctx);
    work.sums.merge(&sum.work());
    work.checked(result.map_err(NumericError::Arithmetic))
}
impl NumericWork {
    pub(super) fn ledger_net(
        &mut self,
        ledger: &RetainedLedger,
        dof: usize,
    ) -> Result<Enclosure, NumericError> {
        let lo = self.net_toward(ledger, dof, Toward::Down)?;
        let hi = self.net_toward(ledger, dof, Toward::Up)?;
        Ok(Enclosure { lo, hi })
    }
    fn net_toward(
        &mut self,
        ledger: &RetainedLedger,
        dof: usize,
        toward: Toward,
    ) -> Result<Endpoint, NumericError> {
        self.begin(Entry::Add)?;
        let ctx = WideContext::<16>::new(1024).map_err(|e| NumericError::Arithmetic(e.into()))?;
        net_owned(self, ledger, dof, toward, ctx, ExactWideSum::new())
    }
}
fn r4_owned(
    work: &mut NumericWork,
    p: &[Endpoint; 4],
    t: Toward,
    mut ctx: WideContext<16>,
    mut sum: ExactWideSum,
) -> Result<Endpoint, NumericError> {
    let result = (|| {
        for (v, negate) in p.iter().zip([false, true, false, true]) {
            sum.add_wide(v, negate).map_err(AttemptStop::from)?;
        }
        if t == Toward::Down && sum.signum().map_err(AttemptStop::from)? <= 0 {
            return Err(NumericError::NonpositiveSource);
        }
        directed::round_toward(&mut ctx, &mut sum, t).map_err(NumericError::Arithmetic)
    })();
    work.wide.record(&ctx);
    work.sums.merge(&sum.work());
    work.checked(result)
}

fn material(
    work: &mut NumericWork,
    m: MaterialOperands,
    k: &AdmittedOperands,
) -> Result<(Enclosure, Enclosure), NumericError> {
    match m {
        MaterialOperands::ExactENu { e, nu } => {
            let e = pos_lift(e)?;
            let nu = lift(nu)?;
            if nu.cmp_value(&Endpoint::ONE.neg()) != Ordering::Greater
                || nu.cmp_value(&shift(&Endpoint::ONE, -1)?) != Ordering::Less
            {
                return Err(NumericError::InvalidMaterial);
            }
            if e.cmp_value(&lift(k.e)?) != Ordering::Equal {
                return Err(NumericError::MaterialBits);
            }
            let denominator = work.add(Enclosure::point(Endpoint::ONE), Enclosure::point(nu))?;
            let denominator = shift_interval(denominator, 1)?;
            let e = Enclosure::point(e);
            let g = work.div(e, denominator)?;
            Ok((e, g))
        }
        MaterialOperands::Ordinary { e, g } => {
            let pair = (pos_lift(e)?, pos_lift(g)?);
            if e.to_bits() != k.e.to_bits() || g.to_bits() != k.g.to_bits() {
                return Err(NumericError::MaterialBits);
            }
            Ok((Enclosure::point(pair.0), Enclosure::point(pair.1)))
        }
        MaterialOperands::Interpolated {
            t_lo,
            t,
            t_hi,
            e_lo,
            e_hi,
            g_lo,
            g_hi,
            e_hat,
            g_hat,
        } => {
            let (lo, at, hi) = (lift(t_lo)?, lift(t)?, lift(t_hi)?);
            if lo.cmp_value(&at) != Ordering::Less || at.cmp_value(&hi) != Ordering::Less {
                return Err(NumericError::TemperatureOrder);
            }
            let (ea, ga) = (pos_lift(e_hat)?, pos_lift(g_hat)?);
            pos_lift(g_lo)?;
            pos_lift(g_hi)?;
            if e_hat.to_bits() != k.e.to_bits() || g_hat.to_bits() != k.g.to_bits() {
                return Err(NumericError::MaterialBits);
            }
            let h = work.sub_pair(&hi, &lo)?.positive()?;
            let mut property = |xlo: f64, xhi: f64, actual: Endpoint| {
                let p = [
                    work.exact_product(t_hi, xlo)?,
                    work.exact_product(t, xlo)?,
                    work.exact_product(t, xhi)?,
                    work.exact_product(t_lo, xhi)?,
                ];
                let n = Enclosure {
                    lo: work.r4(&p, Toward::Down)?,
                    hi: work.r4(&p, Toward::Up)?,
                };
                Ok::<_, NumericError>(work.div(n, h)?.hull_point(actual))
            };
            let e = property(e_lo, e_hi, ea)?;
            let g = property(g_lo, g_hi, ga)?;
            Ok((e, g))
        }
    }
}

fn pi() -> Result<Enclosure, NumericError> {
    // Reviewed 514-bit numerators / 2^512, normalized at exponent 1. Fixed
    // constants only; no binary64 pi and no runtime transcendental calculation.
    const LOWER: [u64; 16] = [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0xc000000000000000,
        0x4fe1356d6d51c245,
        0x302b0a6df25f1437,
        0xef9519b3cd3a431b,
        0x514a08798e3404dd,
        0x020bbea63b139b22,
        0x29024e088a67cc74,
        0xc4c6628b80dc1cd1,
        0xc90fdaa22168c234,
    ];
    const UPPER: [u64; 16] = [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x4fe1356d6d51c246,
        0x302b0a6df25f1437,
        0xef9519b3cd3a431b,
        0x514a08798e3404dd,
        0x020bbea63b139b22,
        0x29024e088a67cc74,
        0xc4c6628b80dc1cd1,
        0xc90fdaa22168c234,
    ];
    let make = |limbs| {
        Endpoint::from_parts(false, 1, limbs).map_err(|e| NumericError::Arithmetic(e.into()))
    };
    Ok(Enclosure {
        lo: make(LOWER)?,
        hi: make(UPPER)?,
    })
}

pub(super) fn member_coefficients(input: &MemberOperands) -> MemberSpent {
    let mut work = NumericWork::new();
    let result = build_member(input, &mut work);
    let result = work.checked(result);
    MemberSpent { result, work }
}
fn build_member(
    input: &MemberOperands,
    work: &mut NumericWork,
) -> Result<MemberEnclosures, NumericError> {
    let k = &input.admitted;
    for v in [k.e, k.g, k.a, k.j, k.iz, k.iy] {
        pos_lift(v)?;
    }
    let (e, g) = material(work, input.material, k)?;
    let d = pos_lift(input.diameter)?;
    let t = pos_lift(input.effective_wall)?;
    let c = shift(&d, -1)?;
    if t.cmp_value(&c) != Ordering::Less {
        return Err(NumericError::InvalidGeometry);
    }
    let ri = work.sub_pair(&c, &t)?.positive()?;
    let d_minus_t = work.sub_pair(&d, &t)?.positive()?;
    let p = work.mul(Enclosure::point(t), d_minus_t)?;
    // c is the exact binary64 D lift shifted by one: c*c has <=106 bits.
    let c_square = Enclosure::point(work.scalar(Entry::Mul, &c, &c, Toward::Up)?);
    let ri_square = work.mul(ri, ri)?;
    let q = work.add(c_square, ri_square)?.positive()?;
    let geometry_g = work.mul(p, q)?;
    let pi = pi()?;
    let a = work.mul(pi, p)?;
    let i = shift_interval(work.mul(pi, geometry_g)?, -2)?;
    let j = shift_interval(i, 1)?;
    let z = work.div(i, Enclosure::point(c))?;
    let coefficients = [
        work.mul(e, a)?,
        work.mul(g, j)?,
        work.mul(e, i)?,
        work.mul(e, i)?,
    ];
    let admitted_products = [
        work.exact_product(k.e, k.a)?,
        work.exact_product(k.g, k.j)?,
        work.exact_product(k.e, k.iz)?,
        work.exact_product(k.e, k.iy)?,
    ];
    let mut coefficient_differences = [Endpoint::ZERO; 4];
    for index in 0..4 {
        let lo = work.absdiff(&coefficients[index].lo, &admitted_products[index])?;
        let hi = work.absdiff(&coefficients[index].hi, &admitted_products[index])?;
        coefficient_differences[index] = max(lo, hi);
    }
    // B3-K K3-2: represented Z is a section and recipe quantity, not a material
    // one, so the Iy = Iz axis check and hull(I_K/c, Z-hat) hold for every
    // material, the exact E/nu route included (DEF-O `rows.component_stress`).
    let represented_z = if hooks::material_gated(&input.material) {
        None
    } else {
        if k.iy.to_bits() != k.iz.to_bits() {
            return Err(NumericError::AxisBits);
        }
        let actual = pos_lift(k.z_hat)?;
        Some(
            work.div(Enclosure::point(pos_lift(k.iz)?), Enclosure::point(c))?
                .hull_point(actual),
        )
    };
    Ok(MemberEnclosures {
        effective_wall_bits: input.effective_wall.to_bits(),
        c,
        ri,
        p,
        q,
        geometry_g,
        a,
        i,
        j,
        z,
        e,
        g,
        coefficients,
        admitted_products,
        coefficient_differences,
        represented_z,
    })
}

/// B3-K SA-2's discriminator. Outside `cfg(test)` the gate never applies. In
/// tests, a thread may restore the pre-K3-2 material gate (represented Z only for
/// the ordinary and interpolated materials) to show that an exact E/nu stress or
/// maximum row fails without K3-2 and certifies with it.
pub(crate) mod hooks {
    use super::MaterialOperands;
    #[cfg(not(test))]
    #[inline(always)]
    pub(crate) fn material_gated(_material: &MaterialOperands) -> bool {
        false
    }
    /// B2-K SF-4's control (test-only): the combined net rounded to nearest at
    /// 1024 bits instead of outward. Never applies outside `cfg(test)`.
    #[cfg(not(test))]
    #[inline(always)]
    pub(crate) fn nearest_net() -> bool {
        false
    }
    #[cfg(test)]
    thread_local! {
        static NEAREST_NET: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
    #[cfg(test)]
    pub(crate) fn set_nearest_net(on: bool) {
        NEAREST_NET.with(|g| g.set(on));
    }
    #[cfg(test)]
    pub(crate) fn nearest_net() -> bool {
        NEAREST_NET.with(std::cell::Cell::get)
    }
    #[cfg(test)]
    thread_local! {
        static MATERIAL_GATE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
    #[cfg(test)]
    pub(crate) fn set_material_gate(on: bool) {
        MATERIAL_GATE.with(|g| g.set(on));
    }
    #[cfg(test)]
    pub(crate) fn material_gated(material: &MaterialOperands) -> bool {
        MATERIAL_GATE.with(std::cell::Cell::get)
            && matches!(material, MaterialOperands::ExactENu { .. })
    }
}

// I51's fixed scalar preparation. This result proves geometry rounding only;
// PP must independently authenticate the normalized input and actual new source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnulusPreparationVersion { NormalizedAnnulusV1 }
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PreparedSectionBits { bits: [u64; 5] }
impl PreparedSectionBits {
    pub fn bits(self) -> [u64; 5] { self.bits }
    pub fn values(self) -> [f64; 5] { self.bits.map(f64::from_bits) }
}
#[derive(Debug)]
pub struct PreparedAnnulus {
    input: [u64; 2], section: PreparedSectionBits,
}
impl PreparedAnnulus {
    pub fn input_bits(&self) -> [u64; 2] { self.input }
    pub fn section_bits(&self) -> PreparedSectionBits { self.section }
    pub fn algorithm(&self) -> AnnulusPreparationVersion { AnnulusPreparationVersion::NormalizedAnnulusV1 }
}
#[derive(Debug, Clone, PartialEq)]
pub enum SectionPreparationError {
    InvalidGeometry, AmbiguousRounding(usize), PrimitiveRange(usize),
    Arithmetic(PreparationArithmeticCause), Accounting,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PreparationArithmeticCause(NumericError);
impl PreparationArithmeticCause { pub fn cause(&self)->&NumericError {&self.0} }
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum PreparationEndpoint { Lo, Hi, Exact }
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct PreparationConversion {pub property:usize,pub endpoint:PreparationEndpoint,pub outcome:super::wide::multi::Binary64Outcome}

#[derive(Debug)]
pub struct SectionPreparationWork {
    numeric: NumericWork,
    pub initialized_endpoints: WorkTotal,
    pub conversions: WorkTotal,
    pub checks: WorkTotal,
    pub endpoint_assignments: WorkTotal,
    pub layout_bytes: [usize; 8],
    conversion_outcomes:[Option<PreparationConversion>;9], conversion_entries:usize,
    pub trace_copy_work:TraceCopyWork,
}
impl SectionPreparationWork {
    fn new() -> Self {
        Self { numeric: NumericWork::new(), initialized_endpoints: WorkTotal::zero(),
            conversions: WorkTotal::zero(), checks: WorkTotal::zero(),
            endpoint_assignments: WorkTotal::zero(), layout_bytes: [
                std::mem::size_of::<SectionPrepFrame>(), std::mem::size_of::<Endpoint>(),
                std::mem::size_of::<NumericWork>(), std::mem::size_of::<WideContext<16>>(),
                std::mem::size_of::<ExactWideSum>(), std::mem::size_of::<PreparedAnnulus>(),
                std::mem::size_of::<super::wide::multi::Binary64Outcome>(),
                std::mem::align_of::<SectionPrepFrame>() ],
            conversion_outcomes:[None;9],conversion_entries:0,trace_copy_work:TraceCopyWork {events:WorkTotal::exact_count(9),
                bytes:WorkTotal::exact_count(std::mem::size_of::<[Option<PreparationConversion>;9]>() as u64)} }
    }
    pub fn numeric_trace(&self,copies:&mut TraceCopyWork)->NumericTrace {self.numeric.trace(copies)}
    pub fn conversion_outcomes(&self)->&[Option<PreparationConversion>] {&self.conversion_outcomes[..self.conversion_entries]}
    pub fn status(&self) -> WorkStatus {
        self.numeric.status().join(self.initialized_endpoints.status())
            .join(self.conversions.status()).join(self.checks.status()).join(self.endpoint_assignments.status())
    }
    fn check(&mut self, n: u64) -> Result<(), SectionPreparationError> {
        self.checks = self.checks.add(WorkTotal::exact_count(n));
        if self.status().is_exact() { Ok(()) } else { Err(SectionPreparationError::Accounting) }
    }
    fn assigned(&mut self, n: u64) -> Result<(), SectionPreparationError> {
        self.endpoint_assignments = self.endpoint_assignments.add(WorkTotal::exact_count(n));
        self.check(0)
    }
    fn round(&mut self, value: &Endpoint, property: usize, endpoint:PreparationEndpoint) -> Result<f64, SectionPreparationError> {
        self.check(1)?;
        // Capacity admission precedes the actual entered-call counter.
        self.trace_copy_work.record::<usize>();
        if self.conversion_entries>=self.conversion_outcomes.len(){return Err(SectionPreparationError::Accounting);}
        self.conversions = self.conversions.add(WorkTotal::exact_count(1));
        self.check(0)?;
        // Existing conversion has no arithmetic tariff; this records its entered
        // call. Its internal integer visits remain explicit auxiliary work.
        let outcome=value.to_binary64();
        self.conversion_outcomes[self.conversion_entries]=Some(PreparationConversion{property,endpoint,outcome});
        self.conversion_entries+=1;
        self.trace_copy_work.record::<PreparationConversion>();
        match outcome {
            super::wide::multi::Binary64Outcome::Normal(v) if v.is_normal() && v > 0.0 => Ok(v),
            _ => Err(SectionPreparationError::PrimitiveRange(property)),
        }
    }
}
// Exactly 27 Endpoint fields. Existing helper/caller/return temporaries are
// separate from this named frame and are reported by the C0 layout evidence.
struct SectionPrepFrame {
    d: Endpoint, t: Endpoint, c: Endpoint, ri: Enclosure, dm: Enclosure,
    p: Enclosure, c2: Endpoint, ri2: Enclosure, q: Enclosure, g: Enclosure,
    pi: Enclosure, a: Enclosure, i: Enclosure, j: Enclosure, z: Enclosure,
    returned: Endpoint,
}
impl SectionPrepFrame {
    fn zero() -> Self {
        let z = Enclosure::point(Endpoint::ZERO);
        Self { d: Endpoint::ZERO, t: Endpoint::ZERO, c: Endpoint::ZERO,
            ri:z,dm:z,p:z,c2:Endpoint::ZERO,ri2:z,q:z,g:z,pi:z,a:z,i:z,j:z,z:z,
            returned:Endpoint::ZERO }
    }
}
#[derive(Debug)]
pub struct AnnulusPreparationSpent {
    result: Result<PreparedAnnulus, SectionPreparationError>, work: SectionPreparationWork,
}
impl AnnulusPreparationSpent {
    pub fn result(&self) -> Result<&PreparedAnnulus, &SectionPreparationError> { self.result.as_ref() }
    pub fn work(&self) -> &SectionPreparationWork { &self.work }
    pub fn into_parts(self) -> (Result<PreparedAnnulus, SectionPreparationError>, SectionPreparationWork) {
        (self.result, self.work)
    }
}
pub fn prepare_product_annulus(diameter:f64,effective_wall:f64) -> AnnulusPreparationSpent {
    let mut work=SectionPreparationWork::new();
    let result=(|| {
        work.check(3)?;
        if !diameter.is_finite() || !effective_wall.is_finite() || diameter<=0.0 || effective_wall<=0.0 {
            return Err(SectionPreparationError::InvalidGeometry);
        }
        work.initialized_endpoints=WorkTotal::exact_count(27);
        let mut f=SectionPrepFrame::zero();
        let numeric=(|| -> Result<(),NumericError> {
            macro_rules! assign { ($field:ident,$n:expr,$value:expr) => {{
                work.endpoint_assignments=work.endpoint_assignments.add(WorkTotal::exact_count($n));
                if let Some(e)=work.status().fault() {return Err(NumericError::Arithmetic(AttemptStop::WorkAccounting(e)));}
                f.$field=$value;
            }}; }
            assign!(d,1,pos_lift(diameter)?); assign!(t,1,pos_lift(effective_wall)?); assign!(c,1,shift(&f.d,-1)?);
            if f.t.cmp_value(&f.c)!=Ordering::Less { return Err(NumericError::InvalidGeometry); }
            assign!(ri,2,work.numeric.sub_pair(&f.c,&f.t)?.positive()?);
            assign!(dm,2,work.numeric.sub_pair(&f.d,&f.t)?.positive()?);
            assign!(p,2,work.numeric.mul(Enclosure::point(f.t),f.dm)?);
            assign!(c2,1,work.numeric.scalar(Entry::Mul,&f.c,&f.c,Toward::Up)?);
            assign!(ri2,2,work.numeric.mul(f.ri,f.ri)?);
            assign!(q,2,work.numeric.add(Enclosure::point(f.c2),f.ri2)?.positive()?);
            assign!(g,2,work.numeric.mul(f.p,f.q)?);
            assign!(pi,2,pi()?);
            assign!(a,2,work.numeric.mul(f.pi,f.p)?);
            assign!(i,2,shift_interval(work.numeric.mul(f.pi,f.g)?,-2)?);
            assign!(j,2,shift_interval(f.i,1)?);
            assign!(z,2,work.numeric.div(f.i,Enclosure::point(f.c))?);
            Ok(())
        })();
        numeric.map_err(|e| SectionPreparationError::Arithmetic(PreparationArithmeticCause(e)))?;
        let mut bits=[0;5];
        for (i,iv) in [&f.a,&f.i,&f.j,&f.z].into_iter().enumerate() {
            let lo=work.round(&iv.lo,i,PreparationEndpoint::Lo)?; let hi=work.round(&iv.hi,i,PreparationEndpoint::Hi)?;
            work.check(1)?;
            if lo.to_bits()!=hi.to_bits() { return Err(SectionPreparationError::AmbiguousRounding(i)); }
            bits[i]=lo.to_bits();
        }
        bits[4]=work.round(&f.c,4,PreparationEndpoint::Exact)?.to_bits();
        work.check(0)?;
        Ok(PreparedAnnulus {input:[diameter.to_bits(),effective_wall.to_bits()],section:PreparedSectionBits{bits}})
    })();
    AnnulusPreparationSpent{result,work}
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/product_certificate_tests.rs"]
mod tests;

pub(crate) mod bridge;

pub(crate) mod source_residual;

pub(crate) mod final_case;
